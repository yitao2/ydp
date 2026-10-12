use crate::{RawJson, ReceiveSeq, RequestId, SessionKey, Submission};
use std::{fmt, sync::Arc};
/// 通信操作的分类结果；失败不隐含自动重试。
pub type Result<T> = std::result::Result<T, Error>;
/// 可模式匹配的错误类别；未来版本可新增类别。
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    /// 本地参数不符合范围或形状。
    InvalidArgument,
    /// 当前线程没有 Tokio runtime。
    NoRuntime,
    /// 参数序列化失败，尚未发布。
    Encode,
    /// 匹配结果或事件的领域解码失败。
    Decode,
    /// 端点返回的协议错误，保留原始数据的存在性。
    Protocol {
        /// 端点的 i32 错误代码。
        code: i32,
        /// 端点提供的错误说明。
        message: String,
        /// 可缺失、明确 null 或其他 JSON 的附加信息。
        data: Option<RawJson>,
    },
    /// 本地合作式期限已到。
    Timeout,
    /// 明确资源预算耗尽。
    ResourceLimit {
        /// 固定资源名称，不包含用户参数。
        resource: &'static str,
    },
    /// 对应流量类别的有界准入队列已满。
    AdmissionFull,
    /// 本连接的正 i32 请求 ID 已耗尽。
    IdExhausted,
    /// 本地 Session 已关闭。
    SessionClosed,
    /// Session ID 已退出，不能复用。
    SessionIdRetired,
    /// 当前订阅发生事件或字节溢出。
    EventBufferFull,
    /// 入站 envelope 无效。
    InvalidMessage,
    /// 响应 Session 与该 request ID 已登记的路由不一致。
    ///
    /// 运行时拒绝该响应 payload，只终结对应请求并保持连接及其他路由可用。Chromium 在 child
    /// Session 退役后可能把迟到响应放到根 envelope；调用方仍须用 Session 生命周期事实判断远端
    /// 责任，不能把本错误解释为命令未执行。
    ResponseSessionMismatch,
    /// 底层传输失败。
    Transport,
    /// 本地连接关闭。
    ConnectionClosed,
    /// Driver 意外退出。
    DriverFailed,
}
/// 请求失败时保留的身份与提交证据；不包含参数全文。
#[derive(Clone, Debug)]
pub struct RequestContext {
    /// 完整协议方法名。
    pub method: String,
    /// 已分配的请求 ID；编码前可为空。
    pub request_id: Option<RequestId>,
    /// 本地路由身份。
    pub session_key: SessionKey,
    /// 保守提交证据。
    pub submission: Submission,
    /// 已消费响应的本地接收序号。
    pub receive_seq: Option<ReceiveSeq>,
}
/// 拥有型错误；Clone 共享底层原因及原始帧，协议说明和上下文中的拥有型字符串会复制。
#[derive(Clone)]
pub struct Error {
    /// 可结构化匹配的类别。
    pub kind: ErrorKind,
    /// 请求相关错误的有界上下文。
    pub context: Option<Box<RequestContext>>,
    source: Option<Arc<dyn std::error::Error + Send + Sync>>,
    frame: Option<crate::raw::Frame>,
}
impl Error {
    /// 创建无底层原因的分类错误，不发生 IO。
    /// ```
    /// use ydp_core::{Error, ErrorKind};
    /// let error = Error::new(ErrorKind::Timeout);
    /// assert!(matches!(error.kind, ErrorKind::Timeout));
    /// ```
    pub fn new(kind: ErrorKind) -> Self {
        Self {
            kind,
            context: None,
            source: None,
            frame: None,
        }
    }
    /// 保留类型化底层原因；默认 Debug 不展开原因，避免暴露传输凭据。
    /// ```
    /// use ydp_core::{Error, ErrorKind};
    /// let error = Error::with_source(ErrorKind::Transport, std::io::Error::other("失败"));
    /// assert!(std::error::Error::source(&error).is_some());
    /// ```
    pub fn with_source(
        kind: ErrorKind,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            kind,
            context: None,
            source: Some(Arc::new(source)),
            frame: None,
        }
    }
    /// 保留协议错误原始帧的独立字节许可；例如 `error.retain_frame(frame)` 后，直到错误及其克隆全部 Drop 才释放。
    /// 不引用连接控制器；运行时回归测试覆盖没有 data 的排队错误及 receiver Drop。
    pub(crate) fn retain_frame(&mut self, frame: crate::raw::Frame) {
        self.frame = Some(frame);
    }
}
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Error")
            .field("kind", &self.kind)
            .field("context", &self.context)
            .finish_non_exhaustive()
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CDP 通信错误：{:?}", self.kind)
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|s| s.as_ref() as _)
    }
}
