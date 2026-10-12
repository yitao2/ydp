//! 纯异步 CDP 兼容协议通信核心；不管理浏览器语义或重试。
#![forbid(unsafe_code)]
/// 生成绑定共享的版本化序列化基础设施。
pub mod binding_support;
mod error;
mod raw;
mod runtime;
mod transport;
mod types;
mod wire;
/// 有限协议数值。
pub use binding_support::FiniteF64;
/// 结构化通信错误与请求上下文。
pub use error::{Error, ErrorKind, RequestContext, Result};
/// 共享原始 JSON 字节视图。
pub use raw::RawJson;
/// 异步连接、Session、请求与订阅。
pub use runtime::*;
/// 完整文本消息传输契约。
pub use transport::{MessageReader, MessageWriter, Transport, TransportError};
/// 协议身份、选项和静态 marker 契约。
pub use types::*;
#[cfg(feature = "websocket")]
mod websocket;
/// WebSocket 连接与 TLS 信任选项。
#[cfg(feature = "websocket")]
pub use websocket::{ConnectOptions, TlsOptions};
