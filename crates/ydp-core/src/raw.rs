//! 原始 JSON 共享视图与独立的入站字节账本。
use crate::{Error, ErrorKind, Result};
use bytes::Bytes;
use serde::{Serialize, Serializer, de::DeserializeOwned};
use serde_json::value::RawValue;
use std::{
    fmt,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

/// 已验证 JSON 的拥有型视图；Clone 共享原始帧，并延长 SDK 字节许可寿命。
/// Debug 仅显示长度，避免默认输出协议 payload。
#[derive(Clone)]
pub struct RawJson {
    bytes: Bytes,
    owner: Option<Arc<FrameOwner>>,
}
impl RawJson {
    /// 验证最多 128 MiB、容器深度最多 128 的完整 JSON 值。
    /// 输入无效返回 InvalidArgument，超限返回 ResourceLimit；不访问网络。
    /// ```
    /// let json = ydp_core::RawJson::from_bytes(bytes::Bytes::from_static(b"{\"a\":1}"))?;
    /// assert_eq!(json.as_bytes(), b"{\"a\":1}");
    /// # Ok::<(), ydp_core::Error>(())
    /// ```
    pub fn from_bytes(bytes: Bytes) -> Result<Self> {
        if bytes.len() > 128 * 1024 * 1024 {
            return Err(Error::new(ErrorKind::ResourceLimit {
                resource: "原始 JSON 字节",
            }));
        }
        crate::wire::check_depth(&bytes, 128)
            .map_err(|error| Error::with_source(ErrorKind::InvalidArgument, error))?;
        let _: &RawValue = serde_json::from_slice(&bytes)
            .map_err(|error| Error::with_source(ErrorKind::InvalidArgument, error))?;
        Ok(Self { bytes, owner: None })
    }
    /// 借用原始字节；不复制、不统一空白，返回值不能超过本视图的生命周期。
    /// ```
    /// let json = ydp_core::RawJson::from_bytes(bytes::Bytes::from_static(b"null"))?;
    /// assert_eq!(json.as_bytes(), b"null");
    /// # Ok::<(), ydp_core::Error>(())
    /// ```
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
    /// 在调用者上下文同步解码为拥有型值，失败返回 Decode。
    /// 自定义 Deserialize 的 CPU 耗时不可抢占；其 panic 会传播给调用者。
    /// ```
    /// let json = ydp_core::RawJson::from_bytes(bytes::Bytes::from_static(b"42"))?;
    /// assert_eq!(json.decode::<i64>()?, 42);
    /// # Ok::<(), ydp_core::Error>(())
    /// ```
    pub fn decode<T: DeserializeOwned>(&self) -> Result<T> {
        serde_json::from_slice(&self.bytes)
            .map_err(|error| Error::with_source(ErrorKind::Decode, error))
    }
    /// 返回完整 backing 的保守字节数；例如共享小片段仍计整个入站帧。
    /// 模块内预算测试验证此数值和最后引用释放行为。
    pub(crate) fn backing_len(&self) -> usize {
        self.owner
            .as_ref()
            .map_or(self.bytes.len(), |owner| owner.bytes.len())
    }
}
impl fmt::Debug for RawJson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawJson")
            .field("len", &self.bytes.len())
            .field("backing_len", &self.backing_len())
            .finish()
    }
}
impl Serialize for RawJson {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let raw: &RawValue =
            serde_json::from_slice(&self.bytes).map_err(serde::ser::Error::custom)?;
        raw.serialize(serializer)
    }
}

/// 独立入站物理字节预算，不反向持有运行时或 Control。
pub(crate) struct ByteBudget {
    used: AtomicUsize,
    limit: usize,
}
impl ByteBudget {
    /// 建立计数为零的预算；例如 `ByteBudget::new(1024)` 限制 1024 字节。
    pub(crate) fn new(limit: usize) -> Self {
        Self {
            used: AtomicUsize::new(0),
            limit,
        }
    }
    /// 读取当前被 backing 持有的字节快照；仅用于诊断和资源测试。
    #[cfg(test)]
    pub(crate) fn used(&self) -> usize {
        self.used.load(Ordering::Acquire)
    }
}
struct FrameOwner {
    bytes: Bytes,
    budget: Arc<ByteBudget>,
}
impl Drop for FrameOwner {
    fn drop(&mut self) {
        self.budget
            .used
            .fetch_sub(self.bytes.len(), Ordering::AcqRel);
    }
}
/// 已验证的 UTF-8 帧文本视图；共享原始 Bytes 与整帧预算许可。
#[derive(Clone)]
pub(crate) struct FrameText {
    text: bytestring::ByteString,
    _owner: Arc<FrameOwner>,
}
impl FrameText {
    /// 借用已验证文本，不重新解析 UTF-8；模块测试验证转义前的原帧子串。
    pub(crate) fn as_str(&self) -> &str {
        &self.text
    }
}

/// 计费一次的完整消息；Clone 与切片共享所有者，最后一个引用归还许可。
#[derive(Clone)]
pub(crate) struct Frame {
    owner: Arc<FrameOwner>,
}
impl Frame {
    /// 非阻塞申请消息的完整长度；超限返回 ResourceLimit，不等待消费者。
    /// 使用示例为 `Frame::new(bytes, Arc::new(ByteBudget::new(1024)))`。
    pub(crate) fn new(bytes: Bytes, budget: Arc<ByteBudget>) -> Result<Self> {
        let len = bytes.len();
        budget
            .used
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |used| {
                used.checked_add(len).filter(|&total| total <= budget.limit)
            })
            .map_err(|_| {
                Error::new(ErrorKind::ResourceLimit {
                    resource: "入站 backing 字节",
                })
            })?;
        Ok(Self {
            owner: Arc::new(FrameOwner { bytes, budget }),
        })
    }
    /// 借用完整消息以执行 wire visitor；返回值生命周期受此 Frame 限制。
    #[cfg(test)]
    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.owner.bytes
    }
    /// 完整消息的字节长度；订阅按此值计逻辑队列占用。
    pub(crate) fn len(&self) -> usize {
        self.owner.bytes.len()
    }
    /// 保留已验证片段；示例由对 `frame.as_bytes()` 的 parse 取得 raw。
    /// 若 raw 不属于本帧，返回 InvalidArgument，避免公开库内部切片 panic。
    pub(crate) fn json(&self, raw: &RawValue) -> Result<RawJson> {
        let start = raw.get().as_ptr() as usize;
        let base = self.owner.bytes.as_ptr() as usize;
        let Some(offset) = start.checked_sub(base) else {
            return Err(Error::new(ErrorKind::InvalidArgument));
        };
        let Some(end) = offset
            .checked_add(raw.get().len())
            .filter(|&end| end <= self.len())
        else {
            return Err(Error::new(ErrorKind::InvalidArgument));
        };
        Ok(RawJson {
            bytes: self.owner.bytes.slice(offset..end),
            owner: Some(self.owner.clone()),
        })
    }
    /// 从本帧借用的合法文本构造共享视图；只复制 Bytes 句柄并校验该小片段。
    /// 例如 `frame.text(envelope.method.as_deref().unwrap())` 适用于借用分支；
    /// 解码转义后独立拥有的字符串不属于原帧，应由运行时保存拥有型文本。
    /// 外部片段返回 InvalidArgument；测试覆盖共享地址与最后 owner 释放。
    pub(crate) fn text(&self, raw: &str) -> Result<FrameText> {
        let start = raw.as_ptr() as usize;
        let base = self.owner.bytes.as_ptr() as usize;
        let Some(offset) = start.checked_sub(base) else {
            return Err(Error::new(ErrorKind::InvalidArgument));
        };
        let Some(end) = offset
            .checked_add(raw.len())
            .filter(|&end| end <= self.len())
        else {
            return Err(Error::new(ErrorKind::InvalidArgument));
        };
        let text = bytestring::ByteString::try_from(self.owner.bytes.slice(offset..end))
            .map_err(|error| Error::with_source(ErrorKind::InvalidArgument, error))?;
        Ok(FrameText {
            text,
            _owner: self.owner.clone(),
        })
    }

    /// 为省略 params 的事件提供 `{}`，同时保留原帧的完整预算 owner。
    /// 单元测试验证即使空参数视图也延长整帧许可寿命。
    pub(crate) fn empty_json(&self) -> RawJson {
        RawJson {
            bytes: Bytes::from_static(b"{}"),
            owner: Some(self.owner.clone()),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn last_reference_owns_permit() {
        let bytes = Bytes::from_static(br#"{"id":1,"result":{"x":1}}"#);
        let len = bytes.len();
        let budget = Arc::new(ByteBudget::new(len));
        let frame = Frame::new(bytes, budget.clone()).unwrap();
        let envelope = crate::wire::parse(frame.as_bytes(), 1024, 128).unwrap();
        let raw = frame.json(envelope.result.unwrap()).unwrap();
        assert_eq!(raw.backing_len(), len);
        assert_eq!(raw.decode::<serde_json::Value>().unwrap()["x"], 1);
        let clone = raw.clone();
        drop(raw);
        drop(frame);
        assert_eq!(budget.used(), len);
        assert!(Frame::new(Bytes::from_static(b"{}"), budget.clone()).is_err());
        drop(clone);
        assert_eq!(budget.used(), 0);
        let frame = Frame::new(Bytes::from_static(b"{}"), budget.clone()).unwrap();
        let empty = frame.empty_json();
        drop(frame);
        assert_eq!(budget.used(), 2);
        drop(empty);
        assert_eq!(budget.used(), 0);
    }
    #[test]
    fn budget_competing_owners_and_exact_depth() {
        let budget = Arc::new(ByteBudget::new(2));
        let barrier = Arc::new(std::sync::Barrier::new(3));
        let workers: Vec<_> = (0..2)
            .map(|_| {
                let budget = budget.clone();
                let barrier = barrier.clone();
                std::thread::spawn(move || {
                    barrier.wait();
                    let held = Frame::new(Bytes::from_static(b"{}"), budget);
                    barrier.wait();
                    held
                })
            })
            .collect();
        barrier.wait();
        barrier.wait();
        let frames: Vec<_> = workers
            .into_iter()
            .map(|worker| worker.join().unwrap())
            .collect();
        assert_eq!(frames.iter().filter(|value| value.is_ok()).count(), 1);
        assert_eq!(budget.used(), 2);
        drop(frames);
        assert_eq!(budget.used(), 0);
        let exact = format!("{}0{}", "[".repeat(128), "]".repeat(128));
        assert!(RawJson::from_bytes(Bytes::from(exact)).is_ok());
    }

    #[test]
    fn shared_method_text_keeps_owner() {
        let bytes = Bytes::from_static(r#"{"method":"Echo.消息"}"#.as_bytes());
        let budget = Arc::new(ByteBudget::new(1024));
        let frame = Frame::new(bytes, budget.clone()).unwrap();
        let envelope = crate::wire::parse(frame.as_bytes(), 1024, 128).unwrap();
        let method = envelope.method.as_deref().unwrap();
        let text = frame.text(method).unwrap();
        assert_eq!(text.as_str(), "Echo.消息");
        assert_eq!(text.as_str().as_ptr(), method.as_ptr());
        assert!(frame.text("external").is_err());
        let clone = text.clone();
        let len = frame.len();
        drop(frame);
        drop(text);
        assert_eq!(budget.used(), len);
        drop(clone);
        assert_eq!(budget.used(), 0);
    }

    #[test]
    fn validation_and_external_slice() {
        for invalid in [b"".as_slice(), b"null null", b"{", &[255]] {
            assert!(RawJson::from_bytes(Bytes::copy_from_slice(invalid)).is_err());
        }
        let text = format!("{}0{}", "[".repeat(129), "]".repeat(129));
        assert!(RawJson::from_bytes(Bytes::from(text)).is_err());
        let frame = Frame::new(Bytes::from_static(b"{}"), Arc::new(ByteBudget::new(8))).unwrap();
        let other: Box<RawValue> = serde_json::from_str("null").unwrap();
        assert!(frame.json(&other).is_err());
        assert!(
            !format!(
                "{:?}",
                RawJson::from_bytes(Bytes::from_static(br#""secret""#)).unwrap()
            )
            .contains("secret")
        );
    }
}
