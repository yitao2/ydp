//! WebSocket 消息适配器；只连接明确 endpoint，不发现、重连或启动浏览器。
#[cfg(feature = "rustls")]
use std::sync::Arc;
use std::time::Duration;

use bytes::Bytes;
use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_tungstenite::{
    WebSocketStream,
    tungstenite::{
        self, Message,
        client::IntoClientRequest,
        http::{HeaderName, HeaderValue},
        protocol::WebSocketConfig,
    },
};

use crate::{
    Connection, Error, ErrorKind, MessageReader, MessageWriter, RawEventStream, Result,
    RuntimeOptions, Transport, TransportError,
};

/// TLS 信任配置；默认使用固定依赖的 webpki 根，不修改全进程 provider。
#[derive(Clone, Default)]
pub struct TlsOptions {
    /// 额外信任的 PEM 证书集合，可在一个字节数组中串联多个 CA。
    /// 空数组表示只用默认根；无证书、损坏 PEM 或非法证书在联网前失败。
    pub additional_ca_pem: Vec<u8>,
}
impl std::fmt::Debug for TlsOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TlsOptions")
            .field("additional_ca_bytes", &self.additional_ca_pem.len())
            .finish()
    }
}

/// 明确 WebSocket endpoint 的连接选项；不含重试、跳过证书验证或重定向设置。
#[derive(Clone)]
pub struct ConnectOptions {
    /// 连接内资源、初始 All 订阅和关闭预算。
    pub runtime: RuntimeOptions,
    /// DNS、TCP、TLS 和 WebSocket 握手共享的总期限，默认 10 秒。
    pub connect_timeout: Duration,
    /// 保留输入顺序的附加 HTTP 头；值不能含 CR/LF，禁止覆盖握手保留头。
    pub headers: Vec<(String, String)>,
    /// 默认根与可选额外 CA；不保存客户端私钥或认证状态。
    pub tls: TlsOptions,
}
impl Default for ConnectOptions {
    fn default() -> Self {
        Self {
            runtime: RuntimeOptions::default(),
            connect_timeout: Duration::from_secs(10),
            headers: Vec::new(),
            tls: TlsOptions::default(),
        }
    }
}
impl std::fmt::Debug for ConnectOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConnectOptions")
            .field("runtime", &self.runtime)
            .field("connect_timeout", &self.connect_timeout)
            .field("header_count", &self.headers.len())
            .field("tls", &self.tls)
            .finish()
    }
}

impl Connection {
    /// 异步连接完整 ws/wss URL，成功返回连接和已经注册的初始 All 事件流。
    /// 握手前校验选项、头字段和 CA；失败不重试、不跟随 HTTP 重定向。
    /// 认证头只发给指定 endpoint，Debug 不输出头值或 URL。
    /// 需要 Tokio runtime 启用 IO 与 time；缺 Handle 返回 NoRuntime，禁用 driver
    /// 属调用方配置错误，Tokio 可能 panic。丢弃此 Future 会销毁尚未交付的传输。
    /// 关闭 `rustls` feature 后只接受 ws，附加 CA 配置也会被拒绝。
    /// ```no_run
    /// # async fn example() -> ydp_core::Result<()> {
    /// let (connection, events) = ydp_core::Connection::connect(
    ///     "ws://127.0.0.1:9222/devtools/browser/example",
    ///     ydp_core::ConnectOptions::default(),
    /// ).await?;
    /// drop(events);
    /// let report = connection.close().await;
    /// assert!(report.driver_resources_released);
    /// # Ok(()) }
    /// ```
    pub async fn connect(
        endpoint: &str,
        options: ConnectOptions,
    ) -> Result<(Self, RawEventStream)> {
        if tokio::runtime::Handle::try_current().is_err() {
            return Err(Error::new(ErrorKind::NoRuntime));
        }
        crate::runtime::validate_options(&options.runtime)?;
        let deadline = tokio::time::Instant::now()
            .checked_add(options.connect_timeout)
            .ok_or_else(|| Error::new(ErrorKind::InvalidArgument))?;
        if options.connect_timeout.is_zero() {
            return Err(Error::new(ErrorKind::Timeout));
        }
        let request = prepare_request(endpoint, &options.headers)?;
        #[cfg(feature = "rustls")]
        let connector = Some(tokio_tungstenite::Connector::Rustls(tls_config(
            &options.tls,
        )?));
        #[cfg(not(feature = "rustls"))]
        if request.uri().scheme_str() == Some("wss") || !options.tls.additional_ca_pem.is_empty() {
            return Err(Error::new(ErrorKind::InvalidArgument));
        }
        let limits = &options.runtime.limits;
        let write_limit = limits
            .normal
            .max_command_bytes
            .max(limits.control.max_command_bytes)
            .checked_add(64 * 1024 + 1024)
            .ok_or_else(|| Error::new(ErrorKind::InvalidArgument))?;
        let config = WebSocketConfig::default()
            .max_message_size(Some(limits.max_message_bytes))
            .max_frame_size(Some(limits.max_message_bytes))
            .write_buffer_size(64 * 1024)
            .max_write_buffer_size(write_limit);
        let handshake = async {
            #[cfg(feature = "rustls")]
            let result = tokio_tungstenite::connect_async_tls_with_config(
                request,
                Some(config),
                false,
                connector,
            )
            .await;
            #[cfg(not(feature = "rustls"))]
            let result =
                tokio_tungstenite::connect_async_with_config(request, Some(config), false).await;
            result.map_err(|error| Error::with_source(ErrorKind::Transport, error))
        };
        let (socket, _response) = tokio::time::timeout_at(deadline, handshake)
            .await
            .map_err(|_| Error::new(ErrorKind::Timeout))??;
        Self::from_transport(SocketTransport(socket), options.runtime)
    }
}

const HEADER_LIMIT: usize = 64 * 1024;
fn prepare_request(
    endpoint: &str,
    headers: &[(String, String)],
) -> Result<tungstenite::handshake::client::Request> {
    let mut size = endpoint.len();
    if size > HEADER_LIMIT {
        return Err(Error::new(ErrorKind::ResourceLimit {
            resource: "出站握手头字节",
        }));
    }
    for (name, value) in headers {
        size = size
            .checked_add(name.len())
            .and_then(|n| n.checked_add(value.len()))
            .and_then(|n| n.checked_add(4))
            .ok_or_else(|| {
                Error::new(ErrorKind::ResourceLimit {
                    resource: "出站握手头字节",
                })
            })?;
        if size > HEADER_LIMIT {
            return Err(Error::new(ErrorKind::ResourceLimit {
                resource: "出站握手头字节",
            }));
        }
        if value.bytes().any(|b| b == b'\r' || b == b'\n') {
            return Err(Error::new(ErrorKind::InvalidArgument));
        }
        let lower = name.to_ascii_lowercase();
        if [
            "host",
            "upgrade",
            "connection",
            "content-length",
            "transfer-encoding",
        ]
        .contains(&lower.as_str())
            || lower.starts_with("sec-websocket-")
        {
            return Err(Error::new(ErrorKind::InvalidArgument));
        }
    }
    let mut request = endpoint
        .into_client_request()
        .map_err(|e| Error::with_source(ErrorKind::InvalidArgument, e))?;
    if !matches!(request.uri().scheme_str(), Some("ws" | "wss")) || request.uri().host().is_none() {
        return Err(Error::new(ErrorKind::InvalidArgument));
    }
    for (name, value) in headers {
        let name = HeaderName::from_bytes(name.as_bytes())
            .map_err(|e| Error::with_source(ErrorKind::InvalidArgument, e))?;
        let mut value = HeaderValue::from_str(value)
            .map_err(|e| Error::with_source(ErrorKind::InvalidArgument, e))?;
        value.set_sensitive(true);
        request.headers_mut().append(name, value);
    }
    // 精确校验库实际生成的请求大小；此额外编码仅发生在连接握手冷路径。
    let (encoded, _) = tungstenite::handshake::client::generate_request(request.clone())
        .map_err(|e| Error::with_source(ErrorKind::InvalidArgument, e))?;
    if encoded.len() > HEADER_LIMIT {
        return Err(Error::new(ErrorKind::ResourceLimit {
            resource: "出站握手头字节",
        }));
    }
    Ok(request)
}

#[cfg(feature = "rustls")]
fn tls_config(options: &TlsOptions) -> Result<Arc<rustls::ClientConfig>> {
    let mut roots = rustls::RootCertStore::empty();
    roots.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    if !options.additional_ca_pem.is_empty() {
        let mut input = options.additional_ca_pem.as_slice();
        let certificates = rustls_pemfile::certs(&mut input)
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| Error::with_source(ErrorKind::InvalidArgument, e))?;
        if certificates.is_empty() {
            return Err(Error::new(ErrorKind::InvalidArgument));
        }
        for certificate in certificates {
            roots
                .add(certificate)
                .map_err(|e| Error::with_source(ErrorKind::InvalidArgument, e))?;
        }
    }
    let config = rustls::ClientConfig::builder_with_provider(Arc::new(
        rustls::crypto::ring::default_provider(),
    ))
    .with_safe_default_protocol_versions()
    .map_err(|e| Error::with_source(ErrorKind::InvalidArgument, e))?
    .with_root_certificates(roots)
    .with_no_client_auth();
    Ok(Arc::new(config))
}

struct SocketTransport<S>(WebSocketStream<S>);
struct SocketReader<S> {
    stream: SplitStream<WebSocketStream<S>>,
}
struct SocketWriter<S> {
    sink: SplitSink<WebSocketStream<S>, Message>,
}
impl<S: AsyncRead + AsyncWrite + Send + Unpin + 'static> Transport for SocketTransport<S> {
    type Reader = SocketReader<S>;
    type Writer = SocketWriter<S>;
    fn split(self) -> (Self::Reader, Self::Writer) {
        let (sink, stream) = self.0.split();
        (SocketReader { stream }, SocketWriter { sink })
    }
}
impl<S: AsyncRead + AsyncWrite + Send + Unpin + 'static> MessageReader for SocketReader<S> {
    async fn recv(&mut self) -> std::result::Result<Option<Bytes>, TransportError> {
        let mut controls = 0usize;
        loop {
            match self.stream.next().await {
                Some(Ok(Message::Text(text))) => return Ok(Some(text.into())),
                Some(Ok(Message::Binary(_))) => {
                    return Err(TransportError::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "不支持 WebSocket binary 消息",
                    )));
                }
                Some(Ok(Message::Close(_))) | None => return Ok(None),
                Some(Ok(Message::Ping(_) | Message::Pong(_))) => {
                    // 持续读 poll 复用依赖的有界 Pong 槽；不手动发送第二个 Pong。
                    controls += 1;
                    if controls == 256 {
                        controls = 0;
                        tokio::task::yield_now().await;
                    }
                }
                Some(Ok(Message::Frame(_))) => {
                    return Err(TransportError::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "适配器收到未组装 frame",
                    )));
                }
                Some(Err(
                    tungstenite::Error::ConnectionClosed | tungstenite::Error::AlreadyClosed,
                )) => return Ok(None),
                Some(Err(error)) => return Err(TransportError::new(error)),
            }
        }
    }
}
impl<S: AsyncRead + AsyncWrite + Send + Unpin + 'static> MessageWriter for SocketWriter<S> {
    async fn send(&mut self, bytes: Bytes) -> std::result::Result<(), TransportError> {
        let text = tungstenite::Utf8Bytes::try_from(bytes).map_err(TransportError::new)?;
        self.sink
            .feed(Message::Text(text))
            .await
            .map_err(TransportError::new)
    }
    async fn flush(&mut self) -> std::result::Result<(), TransportError> {
        self.sink.flush().await.map_err(TransportError::new)
    }
    async fn close(&mut self) -> std::result::Result<(), TransportError> {
        match self.sink.close().await {
            Ok(())
            | Err(tungstenite::Error::ConnectionClosed | tungstenite::Error::AlreadyClosed) => {
                Ok(())
            }
            Err(error) => Err(TransportError::new(error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tungstenite::protocol::Role;

    #[tokio::test]
    async fn blocked_framed_writer_keeps_reader_and_pong_progress() {
        tokio::time::timeout(Duration::from_secs(5), async {
            // 小容量双向字节管道强制部分帧写入 Pending，仍运行真实 WebSocket 编解码。
            let (client_io, server_io) = tokio::io::duplex(128);
            let client = WebSocketStream::from_raw_socket(client_io, Role::Client, None).await;
            let mut server = WebSocketStream::from_raw_socket(server_io, Role::Server, None).await;
            let (mut reader, mut writer) = SocketTransport(client).split();
            let payload = Bytes::from(vec![b'x'; 256 * 1024]);
            let write = async {
                writer.send(payload).await?;
                writer.flush().await
            };
            tokio::pin!(write);
            assert!(futures_util::poll!(write.as_mut()).is_pending());
            server
                .send(Message::Ping(Bytes::from_static(b"ping")))
                .await
                .unwrap();
            server
                .send(Message::Text("response while blocked".into()))
                .await
                .unwrap();
            let received = reader.recv().await.unwrap().unwrap();
            assert_eq!(&received[..], b"response while blocked");
            assert!(futures_util::poll!(write.as_mut()).is_pending());
            let receive = async {
                let mut pong = false;
                let mut data = false;
                while !(pong && data) {
                    match server.next().await.unwrap().unwrap() {
                        Message::Pong(bytes) => {
                            assert_eq!(&bytes[..], b"ping");
                            pong = true;
                        }
                        Message::Text(text) => {
                            assert_eq!(text.len(), 256 * 1024);
                            data = true;
                        }
                        other => panic!("非预期测试消息：{other:?}"),
                    }
                }
            };
            // 释放读取背压后，原 write Future 与自动 Pong 均必须完成，无重发。
            let (_, result) = tokio::join!(receive, write);
            result.unwrap();
        })
        .await
        .unwrap();
    }

    #[tokio::test(start_paused = true)]
    async fn connection_close_destroys_partial_frame_after_total_budget() {
        use tokio::io::AsyncReadExt;
        let (client_io, mut peer_io) = tokio::io::duplex(128);
        let client = WebSocketStream::from_raw_socket(client_io, Role::Client, None).await;
        let options = RuntimeOptions {
            close_timeout: Duration::from_millis(10),
            ..Default::default()
        };
        let (connection, events) =
            Connection::from_transport(SocketTransport(client), options).unwrap();
        drop(events);
        let params = crate::RawJson::from_bytes(Bytes::from(format!(
            "{{\"text\":\"{}\"}}",
            "x".repeat(256 * 1024)
        )))
        .unwrap();
        let request = connection.root_session().send_raw(
            crate::MethodName::new("Echo.large".into()).unwrap(),
            params,
            Default::default(),
        );
        tokio::pin!(request);
        assert!(futures_util::poll!(request.as_mut()).is_pending());
        let mut first = [0; 1];
        peer_io.read_exact(&mut first).await.unwrap();
        // 已观察到帧首字节后停止读取，强制关闭面对部分写入。
        let close = connection.close();
        tokio::pin!(close);
        assert!(futures_util::poll!(close.as_mut()).is_pending());
        tokio::time::advance(Duration::from_millis(11)).await;
        let report = close.await;
        assert!(report.driver_resources_released);
        assert_eq!(report.transport_end, crate::TransportEnd::Aborted);
        assert!(request.await.is_err());
        let mut remaining = Vec::new();
        peer_io.read_to_end(&mut remaining).await.unwrap();
        assert!(remaining.len() <= 128);
        assert_eq!(first[0] & 0x0f, 1);
        // 首个大文本帧仍未写完，传输即被销毁；字节流中没有追加第二个 Close 帧。
        assert!(remaining.len() < 256 * 1024);
    }

    #[test]
    fn exact_outbound_header_boundary_and_utf8_sharing() {
        let request = prepare_request("ws://127.0.0.1/", &[]).unwrap();
        let base = tungstenite::handshake::client::generate_request(request)
            .unwrap()
            .0
            .len();
        let name = "x-boundary";
        let value = "x".repeat(HEADER_LIMIT - base - name.len() - 4);
        assert!(prepare_request("ws://127.0.0.1/", &[(name.into(), value.clone())]).is_ok());
        assert!(prepare_request("ws://127.0.0.1/", &[(name.into(), value + "x")]).is_err());
        let bytes = Bytes::from(String::from("UTF8 中文"));
        let ptr = bytes.as_ptr();
        let text = tungstenite::Utf8Bytes::try_from(bytes).unwrap();
        let back: Bytes = text.into();
        assert_eq!(back.as_ptr(), ptr);
    }
}
