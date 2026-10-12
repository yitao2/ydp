use bytes::Bytes;
use std::{fmt, future::Future, sync::Arc};
/// 传输错误；默认格式不展开可能包含凭据的底层原因。
#[derive(Clone)]
pub struct TransportError(Arc<dyn std::error::Error + Send + Sync>);
impl TransportError {
    /// 保存拥有型、可跨任务传递的原因，不发生 IO。
    /// ```
    /// let error = ydp_core::TransportError::new(std::io::Error::other("连接关闭"));
    /// assert!(std::error::Error::source(&error).is_some());
    /// ```
    pub fn new(source: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self(Arc::new(source))
    }
}
impl fmt::Debug for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TransportError { source: 隐藏 }")
    }
}
impl fmt::Display for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("传输失败")
    }
}
impl std::error::Error for TransportError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.0.as_ref())
    }
}
/// 已建立的完整文本消息传输；两个方向必须可以在同一任务独立推进。
pub trait Transport: Send + 'static {
    /// 拥有型读半部。
    type Reader: MessageReader;
    /// 拥有型写半部。
    type Writer: MessageWriter;
    /// 消耗传输并转交两半；不建立新连接。
    /// ```no_run
    /// fn halves<T: ydp_core::Transport>(transport: T) { let (_reader, _writer) = transport.split(); }
    /// ```
    fn split(self) -> (Self::Reader, Self::Writer);
}
/// 完整消息读取边界；只有连接终结时运行时才丢弃进行中的操作。
pub trait MessageReader: Send + 'static {
    /// 异步取得完整 UTF-8 JSON 字节；EOF 返回 None；Pending 不得阻塞线程。
    /// ```no_run
    /// async fn read<R: ydp_core::MessageReader>(reader: &mut R) { let _ = reader.recv().await; }
    /// ```
    fn recv(
        &mut self,
    ) -> impl Future<Output = std::result::Result<Option<Bytes>, TransportError>> + Send;
}
/// 唯一 Writer owner 操作的写边界；不得在等待时阻挡 Reader。
pub trait MessageWriter: Send + 'static {
    /// 接受完整文本消息；成功不证明远端接收，资源可保持到 flush。
    /// ```no_run
    /// async fn write<W: ydp_core::MessageWriter>(writer: &mut W) { let _ = writer.send(bytes::Bytes::from_static(b"{}")).await; }
    /// ```
    fn send(
        &mut self,
        bytes: Bytes,
    ) -> impl Future<Output = std::result::Result<(), TransportError>> + Send;
    /// 推进已接受数据；成功不证明远端执行。
    /// ```no_run
    /// async fn flush<W: ydp_core::MessageWriter>(writer: &mut W) { let _ = writer.flush().await; }
    /// ```
    fn flush(&mut self) -> impl Future<Output = std::result::Result<(), TransportError>> + Send;
    /// 开始传输关闭；核心同时推进 Reader，终结时允许销毁两半。
    /// ```no_run
    /// async fn close<W: ydp_core::MessageWriter>(writer: &mut W) { let _ = writer.close().await; }
    /// ```
    fn close(&mut self) -> impl Future<Output = std::result::Result<(), TransportError>> + Send;
}
