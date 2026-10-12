use bytes::Bytes;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tokio::sync::{Semaphore, mpsc, watch};
use ydp_core::{MessageReader, MessageWriter, Transport, TransportError};

// 内存适配器只转交完整消息；CDP 行为由测试中的端点脚本决定。
/// 独立传递文本消息的测试传输，拥有两个有界通道半部。
pub(super) struct MemoryTransport {
    reader: Reader,
    writer: Writer,
}

/// 测试端点拥有注入入口和接收出口；不进入核心实现。
pub(super) struct Endpoint {
    /// 注入完整入站消息，容量为 32 条；发送等待仅发生在测试端点。
    pub(super) incoming: mpsc::Sender<Bytes>,
    /// 接收核心发送的完整消息，保留原始 wire 字节。
    pub(super) outgoing: mpsc::Receiver<Bytes>,
    /// 写侧实际执行 flush 的次数，用于观察批次行为。
    pub(super) flushes: Arc<AtomicUsize>,
    /// 两半 Drop 总数；关闭报告之后必须为二。
    pub(super) dropped_halves: Arc<AtomicUsize>,
}

/// 读半部；仅在连接终结时丢弃待接收 Future。
pub(super) struct Reader {
    incoming: mpsc::Receiver<Bytes>,
    closed: watch::Receiver<bool>,
    dropped: Arc<AtomicUsize>,
}

/// 写半部；可通过信号量把正在执行的 send 精确停在端点可见之后。
pub(super) struct Writer {
    outgoing: mpsc::Sender<Bytes>,
    closed: watch::Sender<bool>,
    gate: Option<Arc<Semaphore>>,
    flushes: Arc<AtomicUsize>,
    dropped: Arc<AtomicUsize>,
}

/// 构造两个容量 32 的消息通道。gate 为零时 send 在字节可见后等待许可。
/// 使用示例由上层测试调用 `memory(None)` 或 `memory(Some(gate))` 后执行实际通信。
pub(super) fn memory(gate: Option<Arc<Semaphore>>) -> (MemoryTransport, Endpoint) {
    let (incoming, rx) = mpsc::channel(32);
    let (tx, outgoing) = mpsc::channel(32);
    let (close_tx, close_rx) = watch::channel(false);
    let flushes = Arc::new(AtomicUsize::new(0));
    let dropped = Arc::new(AtomicUsize::new(0));
    (
        MemoryTransport {
            reader: Reader {
                incoming: rx,
                closed: close_rx,
                dropped: dropped.clone(),
            },
            writer: Writer {
                outgoing: tx,
                closed: close_tx,
                gate,
                flushes: flushes.clone(),
                dropped: dropped.clone(),
            },
        },
        Endpoint {
            incoming,
            outgoing,
            flushes,
            dropped_halves: dropped,
        },
    )
}

impl Transport for MemoryTransport {
    type Reader = Reader;
    type Writer = Writer;

    fn split(self) -> (Self::Reader, Self::Writer) {
        (self.reader, self.writer)
    }
}

impl MessageReader for Reader {
    async fn recv(&mut self) -> Result<Option<Bytes>, TransportError> {
        if *self.closed.borrow() {
            return Ok(None);
        }
        tokio::select! {
            value = self.incoming.recv() => Ok(value),
            _ = self.closed.changed() => Ok(None),
        }
    }
}

impl MessageWriter for Writer {
    async fn send(&mut self, bytes: Bytes) -> Result<(), TransportError> {
        self.outgoing.send(bytes).await.map_err(|_| {
            TransportError::new(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "测试端点已关闭",
            ))
        })?;
        if let Some(gate) = &self.gate {
            let permit = gate.acquire().await.map_err(TransportError::new)?;
            permit.forget();
        }
        Ok(())
    }

    async fn flush(&mut self) -> Result<(), TransportError> {
        self.flushes.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    async fn close(&mut self) -> Result<(), TransportError> {
        // 内存端点没有额外握手；写半部关闭时让读半部观察 EOF。
        let _ = self.closed.send(true);
        Ok(())
    }
}

impl Drop for Reader {
    fn drop(&mut self) {
        self.dropped.fetch_add(1, Ordering::SeqCst);
    }
}

impl Drop for Writer {
    fn drop(&mut self) {
        self.dropped.fetch_add(1, Ordering::SeqCst);
    }
}
