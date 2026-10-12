//! 核心 fake endpoint 的可复现串行往返基线；输出机器可读的延迟与吞吐数据。

use bytes::Bytes;
use std::{error::Error, io, time::Instant};
use tokio::sync::mpsc;
use ydp_core::{Command, Connection, MessageReader, MessageWriter, Transport, TransportError};

type BenchResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

#[derive(serde::Serialize)]
struct Params {
    value: u64,
}

#[derive(serde::Deserialize)]
struct Reply {
    value: u64,
}

struct Echo;

impl Command for Echo {
    type Params = Params;
    type Response = Reply;
    const METHOD: &'static str = "Echo.echo";
}

struct Reader(mpsc::Receiver<Bytes>);
struct Writer(Option<mpsc::Sender<Bytes>>);
struct Endpoint {
    input: mpsc::Receiver<Bytes>,
    output: mpsc::Sender<Bytes>,
}

impl Transport for Endpoint {
    type Reader = Reader;
    type Writer = Writer;

    fn split(self) -> (Self::Reader, Self::Writer) {
        (Reader(self.input), Writer(Some(self.output)))
    }
}

impl MessageReader for Reader {
    async fn recv(&mut self) -> Result<Option<Bytes>, TransportError> {
        Ok(self.0.recv().await)
    }
}

impl MessageWriter for Writer {
    async fn send(&mut self, bytes: Bytes) -> Result<(), TransportError> {
        self.0
            .as_ref()
            .ok_or_else(|| TransportError::new(io::Error::other("写端已经关闭")))?
            .send(bytes)
            .await
            .map_err(|error| TransportError::new(io::Error::other(error)))
    }

    async fn flush(&mut self) -> Result<(), TransportError> {
        Ok(())
    }

    async fn close(&mut self) -> Result<(), TransportError> {
        self.0.take();
        Ok(())
    }
}

fn percentile(sorted: &[u64], numerator: usize, denominator: usize) -> u64 {
    let index = sorted
        .len()
        .saturating_mul(numerator)
        .div_ceil(denominator)
        .saturating_sub(1)
        .min(sorted.len().saturating_sub(1));
    sorted[index]
}

#[cfg(target_os = "linux")]
fn current_rss_bytes() -> Option<u64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    let kib = status.lines().find_map(|line| {
        let value = line.strip_prefix("VmRSS:")?.split_whitespace().next()?;
        value.parse::<u64>().ok()
    })?;
    kib.checked_mul(1024)
}

#[cfg(not(target_os = "linux"))]
fn current_rss_bytes() -> Option<u64> {
    None
}

async fn run() -> BenchResult<()> {
    let iterations = std::env::var("YDP_BENCH_ITERATIONS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(50_000);
    if iterations == 0 {
        return Err(io::Error::other("YDP_BENCH_ITERATIONS 必须大于零").into());
    }
    let (out_tx, mut out_rx) = mpsc::channel::<Bytes>(64);
    let (in_tx, in_rx) = mpsc::channel::<Bytes>(64);
    let endpoint = tokio::spawn(async move {
        let mut count = 0usize;
        while let Some(bytes) = out_rx.recv().await {
            let command: serde_json::Value = serde_json::from_slice(&bytes)?;
            let id = command
                .get("id")
                .and_then(serde_json::Value::as_i64)
                .ok_or_else(|| io::Error::other("命令缺少整数 id"))?;
            let value = command
                .pointer("/params/value")
                .and_then(serde_json::Value::as_u64)
                .ok_or_else(|| io::Error::other("命令缺少 value"))?;
            in_tx
                .send(Bytes::from(
                    serde_json::json!({"id": id, "result": {"value": value}}).to_string(),
                ))
                .await
                .map_err(io::Error::other)?;
            count += 1;
        }
        Ok::<usize, Box<dyn Error + Send + Sync>>(count)
    });
    let (connection, events) = Connection::from_transport(
        Endpoint {
            input: in_rx,
            output: out_tx,
        },
        Default::default(),
    )?;
    drop(events);
    let session = connection.root_session();
    for value in 0..1_000u64 {
        let reply = session.send::<Echo>(Params { value }).await?;
        if reply.value != value {
            return Err(io::Error::other("预热响应不匹配").into());
        }
    }
    let initial_rss_bytes = current_rss_bytes();
    let started = Instant::now();
    let mut latency_ns = Vec::with_capacity(iterations);
    for index in 0..iterations {
        let value = u64::try_from(index)?;
        let request_started = Instant::now();
        let reply = session.send::<Echo>(Params { value }).await?;
        latency_ns.push(u64::try_from(request_started.elapsed().as_nanos())?);
        if reply.value != value {
            return Err(io::Error::other("测量响应不匹配").into());
        }
    }
    let elapsed = started.elapsed();
    latency_ns.sort_unstable();
    let final_rss_bytes = current_rss_bytes();
    drop(session);
    let report = connection.close().await;
    if !report.driver_resources_released {
        return Err(io::Error::other("Driver 资源没有在关闭报告前释放").into());
    }
    let endpoint_count = endpoint.await??;
    if endpoint_count != iterations + 1_000 {
        return Err(io::Error::other("fake endpoint 命令计数不匹配").into());
    }
    println!(
        "YDP_BENCH_REPORT={}",
        serde_json::json!({
            "kind": "serial-fake-endpoint-round-trip",
            "iterations": iterations,
            "warmup_iterations": 1_000,
            "elapsed_ns": elapsed.as_nanos(),
            "throughput_per_second": iterations as f64 / elapsed.as_secs_f64(),
            "latency_ns": {
                "p50": percentile(&latency_ns, 50, 100),
                "p95": percentile(&latency_ns, 95, 100),
                "p99": percentile(&latency_ns, 99, 100),
                "max": latency_ns[latency_ns.len() - 1],
            },
            "initial_rss_bytes": initial_rss_bytes,
            "final_rss_bytes": final_rss_bytes,
            "driver_resources_released": true,
        })
    );
    Ok(())
}

fn main() -> BenchResult<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(run())
}
