//! 本地 WebSocket 真实传输验收；endpoint 只编排协议消息，不隐藏核心捷径。
#![cfg(feature = "websocket")]
use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{
        Message,
        protocol::frame::{
            Frame,
            coding::{Data, OpCode},
        },
    },
};
#[cfg(feature = "rustls")]
use tokio_tungstenite::{
    accept_hdr_async,
    tungstenite::handshake::server::{
        ErrorResponse, Request as HandshakeRequest, Response as HandshakeResponse,
    },
};
use ydp_core::{Command, ConnectOptions, Connection, ErrorKind};

#[derive(serde::Serialize)]
struct Params {
    value: String,
}
#[derive(Debug, serde::Deserialize)]
struct Reply {
    value: String,
}
struct Echo;
impl Command for Echo {
    type Params = Params;
    type Response = Reply;
    const METHOD: &'static str = "Echo.echo";
}
async fn new_listener() -> (TcpListener, String) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("ws://{}/echo", listener.local_addr().unwrap());
    (listener, url)
}
async fn read_handshake(socket: &mut tokio::net::TcpStream) -> String {
    let mut bytes = Vec::new();
    while !bytes.windows(4).any(|part| part == b"\r\n\r\n") {
        let mut chunk = [0; 512];
        let n = socket.read(&mut chunk).await.unwrap();
        assert!(n > 0);
        bytes.extend_from_slice(&chunk[..n]);
        assert!(bytes.len() <= 65536);
    }
    String::from_utf8(bytes).unwrap()
}
fn padded_handshake(request: &str, total: usize) -> Vec<u8> {
    let key = request
        .lines()
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;
            name.eq_ignore_ascii_case("sec-websocket-key")
                .then(|| value.trim())
        })
        .unwrap();
    let accept = tokio_tungstenite::tungstenite::handshake::derive_accept_key(key.as_bytes());
    let prefix = format!(
        "HTTP/1.1 101 Switching Protocols\r\nConnection: Upgrade\r\nUpgrade: websocket\r\nSec-WebSocket-Accept: {accept}\r\nX-Pad: "
    );
    let response = format!("{prefix}{}\r\n\r\n", "x".repeat(total - prefix.len() - 4));
    assert_eq!(response.len(), total);
    response.into_bytes()
}
async fn bounded<T>(future: impl std::future::Future<Output = T>) -> T {
    tokio::time::timeout(Duration::from_secs(10), future)
        .await
        .expect("本地协议交互应完成")
}

#[cfg(feature = "rustls")]
#[allow(clippy::result_large_err)]
fn validate_remote_authorization(
    request: &HandshakeRequest,
    response: HandshakeResponse,
) -> std::result::Result<HandshakeResponse, ErrorResponse> {
    // tungstenite 的回调签名固定以完整 HTTP response 作为错误类型，测试侧无法缩小它。
    assert_eq!(
        request.headers().get("authorization").unwrap(),
        "Bearer remote-fixture-secret"
    );
    Ok(response)
}

#[tokio::test]
async fn fragmented_text_ping_and_bidirectional_close() {
    bounded(async {
        let (listener, url) = new_listener().await;
        let (pong_tx, pong_rx) = tokio::sync::oneshot::channel();
        let server = tokio::spawn(async move {
            let (socket, _) = listener.accept().await.unwrap();
            let mut ws = accept_async(socket).await.unwrap();
            ws.send(Message::Ping(Bytes::from_static(b"heartbeat"))).await.unwrap();
            assert_eq!(ws.next().await.unwrap().unwrap(), Message::Pong(Bytes::from_static(b"heartbeat")));
            pong_tx.send(()).unwrap();
            ws.send(Message::Frame(Frame::message(Bytes::from_static(br#"{"method":"Echo."#), OpCode::Data(Data::Text), false))).await.unwrap();
            ws.send(Message::Frame(Frame::message(Bytes::from_static(br#"ready","params":{"n":1}}"#), OpCode::Data(Data::Continue), true))).await.unwrap();
            let command: serde_json::Value = serde_json::from_str(ws.next().await.unwrap().unwrap().to_text().unwrap()).unwrap();
            assert_eq!(command["method"], "Echo.echo");
            ws.send(Message::Text(serde_json::json!({"id":command["id"],"result":{"value":command["params"]["value"]}}).to_string().into())).await.unwrap();
            assert!(matches!(ws.next().await.unwrap().unwrap(), Message::Close(_)));
            ws.flush().await.unwrap();
        });
        let (connection, mut events) = Connection::connect(&url, ConnectOptions::default()).await.unwrap();
        pong_rx.await.unwrap();
        let event = events.next().await.unwrap().unwrap();
        assert_eq!(event.method(), "Echo.ready");
        assert_eq!(event.params().decode::<serde_json::Value>().unwrap()["n"], 1);
        let response = connection.root_session().send::<Echo>(Params { value: "你好".into() }).await.unwrap();
        assert_eq!(response.value, "你好");
        assert!(connection.close().await.driver_resources_released);
        server.await.unwrap();
    }).await;
}

#[tokio::test]
async fn peer_close_is_answered_once() {
    bounded(async {
        let (listener, url) = new_listener().await;
        let server = tokio::spawn(async move {
            let (socket, _) = listener.accept().await.unwrap();
            let mut ws = accept_async(socket).await.unwrap();
            ws.send(Message::Close(None)).await.unwrap();
            let mut close_count = 0usize;
            while let Some(message) = ws.next().await {
                match message {
                    Ok(Message::Close(_)) => close_count += 1,
                    Err(tokio_tungstenite::tungstenite::Error::ConnectionClosed) => break,
                    Err(error) => panic!("关闭握手出现非预期错误：{error}"),
                    Ok(other) => panic!("关闭握手出现非预期消息：{other:?}"),
                }
            }
            close_count
        });
        let (connection, mut events) = Connection::connect(&url, ConnectOptions::default())
            .await
            .unwrap();
        assert!(events.next().await.unwrap().is_err());
        assert!(connection.closed().await.driver_resources_released);
        assert_eq!(server.await.unwrap(), 1);
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn peer_close_and_binary_rejection() {
    bounded(async {
        for binary in [false, true] {
            let (listener, url) = new_listener().await;
            let server = tokio::spawn(async move {
                let (socket, _) = listener.accept().await.unwrap();
                let mut ws = accept_async(socket).await.unwrap();
                if binary {
                    ws.send(Message::Binary(Bytes::from_static(b"{}")))
                        .await
                        .unwrap();
                } else {
                    ws.send(Message::Close(None)).await.unwrap();
                }
                while let Some(message) = ws.next().await {
                    match message {
                        Ok(Message::Close(_)) => {
                            let _ = ws.flush().await;
                            break;
                        }
                        Err(_) => break,
                        _ => {}
                    }
                }
            });
            let (connection, mut events) = Connection::connect(&url, ConnectOptions::default())
                .await
                .unwrap();
            let failure = events.next().await.unwrap().err().unwrap();
            if binary {
                assert!(matches!(failure.kind, ErrorKind::Transport));
            }
            let report = connection.closed().await;
            assert!(report.driver_resources_released);
            server.await.unwrap();
        }
    })
    .await;
}

#[tokio::test]
async fn headers_limits_and_redirect_are_not_followed() {
    bounded(async {
        for name in [
            "Host",
            "Upgrade",
            "Connection",
            "Sec-WebSocket-Key",
            "sEc-WeBsOcKeT-future",
        ] {
            let mut options = ConnectOptions::default();
            options.headers.push((name.into(), "secret".into()));
            assert!(matches!(
                Connection::connect("ws://127.0.0.1:1", options)
                    .await
                    .err()
                    .unwrap()
                    .kind,
                ErrorKind::InvalidArgument
            ));
        }
        let mut options = ConnectOptions::default();
        options
            .headers
            .push(("Authorization".into(), "secret\r\ninjected: yes".into()));
        assert!(!format!("{options:?}").contains("secret"));
        assert!(matches!(
            Connection::connect("ws://127.0.0.1:1", options)
                .await
                .err()
                .unwrap()
                .kind,
            ErrorKind::InvalidArgument
        ));
        let mut options = ConnectOptions::default();
        options.headers.push(("X-Large".into(), "x".repeat(65536)));
        assert!(matches!(
            Connection::connect("ws://127.0.0.1:1", options)
                .await
                .err()
                .unwrap()
                .kind,
            ErrorKind::ResourceLimit { .. }
        ));
        for url in ["http://127.0.0.1", "file:///tmp/socket", "relative"] {
            assert!(matches!(
                Connection::connect(url, ConnectOptions::default())
                    .await
                    .err()
                    .unwrap()
                    .kind,
                ErrorKind::InvalidArgument
            ));
        }
        let (listener, url) = new_listener().await;
        let target = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let redirect = format!(
            "HTTP/1.1 302 Found\r\nLocation: ws://{}/other\r\nContent-Length: 0\r\n\r\n",
            target.local_addr().unwrap()
        );
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            assert!(read_handshake(&mut socket).await.contains("Bearer fixture"));
            socket.write_all(redirect.as_bytes()).await.unwrap();
        });
        let mut options = ConnectOptions::default();
        options
            .headers
            .push(("Authorization".into(), "Bearer fixture".into()));
        let error = Connection::connect(&url, options).await.err().unwrap();
        assert!(matches!(error.kind, ErrorKind::Transport));
        assert!(!format!("{error:?}").contains("Bearer fixture"));
        server.await.unwrap();
        let accept = target.accept();
        tokio::pin!(accept);
        assert!(futures_util::poll!(accept.as_mut()).is_pending());
    })
    .await;
}

#[test]
fn diagnostics_hide_headers_ca_payload_and_transport_source() {
    let secret = "credential-and-payload-secret";
    let mut options = ConnectOptions::default();
    options
        .headers
        .push(("Authorization".into(), secret.into()));
    options.tls.additional_ca_pem = secret.as_bytes().to_vec();
    assert!(!format!("{options:?}").contains(secret));

    let payload = ydp_core::RawJson::from_bytes(Bytes::from(
        serde_json::json!({"token": secret}).to_string(),
    ))
    .unwrap();
    assert!(!format!("{payload:?}").contains(secret));
    let protocol = ydp_core::Error::new(ErrorKind::Protocol {
        code: -1,
        message: "拒绝".into(),
        data: Some(payload),
    });
    assert!(!format!("{protocol:?}").contains(secret));
    assert!(!protocol.to_string().contains(secret));

    let transport = ydp_core::TransportError::new(std::io::Error::other(secret));
    assert!(!format!("{transport:?}").contains(secret));
    assert!(!transport.to_string().contains(secret));
    let wrapped = ydp_core::Error::with_source(ErrorKind::Transport, transport);
    assert!(!format!("{wrapped:?}").contains(secret));
    assert!(!wrapped.to_string().contains(secret));
}

#[tokio::test]
async fn oversized_handshake_message_and_fragment_accumulation() {
    bounded(async {
        let (listener, url) = new_listener().await;
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let request = read_handshake(&mut socket).await;
            let response = padded_handshake(&request, 65537);
            let _ = socket.write_all(&response).await;
        });
        assert!(
            Connection::connect(&url, ConnectOptions::default())
                .await
                .is_err()
        );
        server.await.unwrap();
        for fragmented in [false, true] {
            let (listener, url) = new_listener().await;
            let server = tokio::spawn(async move {
                let (socket, _) = listener.accept().await.unwrap();
                let mut ws = accept_async(socket).await.unwrap();
                if fragmented {
                    for (part, opcode, final_frame) in [
                        ("a".repeat(40), Data::Text, false),
                        ("b".repeat(40), Data::Continue, true),
                    ] {
                        ws.send(Message::Frame(Frame::message(
                            Bytes::from(part),
                            OpCode::Data(opcode),
                            final_frame,
                        )))
                        .await
                        .unwrap();
                    }
                } else {
                    ws.send(Message::Text("x".repeat(65).into())).await.unwrap();
                }
                while let Some(Ok(message)) = ws.next().await {
                    if matches!(message, Message::Close(_)) {
                        let _ = ws.flush().await;
                        break;
                    }
                }
            });
            let mut options = ConnectOptions::default();
            options.runtime.limits.max_message_bytes = 64;
            let (connection, mut events) = Connection::connect(&url, options).await.unwrap();
            assert!(matches!(
                events.next().await.unwrap().err().unwrap().kind,
                ErrorKind::Transport
            ));
            connection.closed().await;
            server.await.unwrap();
        }
    })
    .await;
}

#[tokio::test(start_paused = true)]
async fn handshake_timeout_uses_one_budget() {
    let (listener, url) = new_listener().await;
    let options = ConnectOptions {
        connect_timeout: Duration::from_millis(10),
        ..Default::default()
    };
    let connect = Connection::connect(&url, options);
    tokio::pin!(connect);
    assert!(futures_util::poll!(connect.as_mut()).is_pending());
    let (socket, _) = listener.accept().await.unwrap();
    tokio::time::advance(Duration::from_millis(11)).await;
    assert!(matches!(
        connect.await.err().unwrap().kind,
        ErrorKind::Timeout
    ));
    drop(socket);
}

#[cfg(feature = "rustls")]
#[tokio::test]
async fn tls_ca_hostname_and_pem_validation() {
    bounded(async {
        for invalid in [
            b"not a PEM".as_slice(),
            b"-----BEGIN CERTIFICATE-----\n%%%\n-----END CERTIFICATE-----\n",
        ] {
            let mut options = ConnectOptions::default();
            options.tls.additional_ca_pem = invalid.to_vec();
            assert!(matches!(
                Connection::connect("wss://127.0.0.1:1", options)
                    .await
                    .err()
                    .unwrap()
                    .kind,
                ErrorKind::InvalidArgument
            ));
        }
        for (trust, matching_name) in [(true, true), (false, true), (true, false)] {
            let ca_key = rcgen::KeyPair::generate().unwrap();
            let mut ca_params = rcgen::CertificateParams::new(Vec::<String>::new()).unwrap();
            ca_params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
            ca_params.key_usages = vec![
                rcgen::KeyUsagePurpose::KeyCertSign,
                rcgen::KeyUsagePurpose::DigitalSignature,
            ];
            let ca = ca_params.self_signed(&ca_key).unwrap();
            let issuer = rcgen::Issuer::new(ca_params, ca_key);
            let leaf_key = rcgen::KeyPair::generate().unwrap();
            let mut leaf_params = rcgen::CertificateParams::new(vec![
                if matching_name {
                    "127.0.0.1"
                } else {
                    "wrong.example"
                }
                .into(),
            ])
            .unwrap();
            leaf_params.extended_key_usages = vec![rcgen::ExtendedKeyUsagePurpose::ServerAuth];
            let leaf = leaf_params.signed_by(&leaf_key, &issuer).unwrap();
            let config = rustls::ServerConfig::builder_with_provider(std::sync::Arc::new(
                rustls::crypto::ring::default_provider(),
            ))
            .with_safe_default_protocol_versions()
            .unwrap()
            .with_no_client_auth()
            .with_single_cert(
                vec![leaf.der().clone()],
                rustls::pki_types::PrivatePkcs8KeyDer::from(leaf_key.serialize_der()).into(),
            )
            .unwrap();
            let (listener, ws_url) = new_listener().await;
            let url = ws_url.replacen("ws:", "wss:", 1);
            let server = tokio::spawn(async move {
                let (socket, _) = listener.accept().await.unwrap();
                let acceptor = tokio_rustls::TlsAcceptor::from(std::sync::Arc::new(config));
                let tls = acceptor.accept(socket).await;
                if !trust || !matching_name {
                    assert!(tls.is_err());
                    return;
                }
                let mut ws = accept_async(tls.unwrap()).await.unwrap();
                let command: serde_json::Value =
                    serde_json::from_str(ws.next().await.unwrap().unwrap().to_text().unwrap())
                        .unwrap();
                ws.send(Message::Text(
                    serde_json::json!({"id":command["id"],"result":{"value":"tls"}})
                        .to_string()
                        .into(),
                ))
                .await
                .unwrap();
                assert!(matches!(
                    ws.next().await.unwrap().unwrap(),
                    Message::Close(_)
                ));
                ws.flush().await.unwrap();
            });
            let mut options = ConnectOptions::default();
            if trust {
                options.tls.additional_ca_pem = ca.pem().into_bytes();
            }
            let connection = Connection::connect(&url, options).await;
            if trust && matching_name {
                let (connection, events) = connection.unwrap();
                drop(events);
                assert_eq!(
                    connection
                        .root_session()
                        .send::<Echo>(Params {
                            value: "tls".into()
                        })
                        .await
                        .unwrap()
                        .value,
                    "tls"
                );
                connection.close().await;
            } else {
                assert!(matches!(
                    connection.err().unwrap().kind,
                    ErrorKind::Transport
                ));
            }
            server.await.unwrap();
        }
    })
    .await;
}

#[cfg(feature = "rustls")]
#[tokio::test]
#[ignore = "需要通过 YDP_REMOTE_TEST_HOST 指定本机非回环地址"]
async fn non_loopback_tls_and_authorization_header() {
    bounded(async {
        let host =
            std::env::var("YDP_REMOTE_TEST_HOST").expect("显式验收必须提供本机可达的非回环地址");
        assert_ne!(host, "127.0.0.1");
        assert_ne!(host, "::1");
        let ca_key = rcgen::KeyPair::generate().unwrap();
        let mut ca_params = rcgen::CertificateParams::new(Vec::<String>::new()).unwrap();
        ca_params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
        ca_params.key_usages = vec![
            rcgen::KeyUsagePurpose::KeyCertSign,
            rcgen::KeyUsagePurpose::DigitalSignature,
        ];
        let ca = ca_params.self_signed(&ca_key).unwrap();
        let issuer = rcgen::Issuer::new(ca_params, ca_key);
        let leaf_key = rcgen::KeyPair::generate().unwrap();
        let mut leaf_params = rcgen::CertificateParams::new(vec![host.clone()]).unwrap();
        leaf_params.extended_key_usages = vec![rcgen::ExtendedKeyUsagePurpose::ServerAuth];
        let leaf = leaf_params.signed_by(&leaf_key, &issuer).unwrap();
        let config = rustls::ServerConfig::builder_with_provider(std::sync::Arc::new(
            rustls::crypto::ring::default_provider(),
        ))
        .with_safe_default_protocol_versions()
        .unwrap()
        .with_no_client_auth()
        .with_single_cert(
            vec![leaf.der().clone()],
            rustls::pki_types::PrivatePkcs8KeyDer::from(leaf_key.serialize_der()).into(),
        )
        .unwrap();
        let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = tokio::spawn(async move {
            let (socket, peer) = listener.accept().await.unwrap();
            assert!(!peer.ip().is_loopback());
            let acceptor = tokio_rustls::TlsAcceptor::from(std::sync::Arc::new(config));
            let tls = acceptor.accept(socket).await.unwrap();
            let mut ws = accept_hdr_async(tls, validate_remote_authorization)
                .await
                .unwrap();
            let command: serde_json::Value =
                serde_json::from_str(ws.next().await.unwrap().unwrap().to_text().unwrap()).unwrap();
            ws.send(Message::Text(
                serde_json::json!({"id":command["id"],"result":{"value":"remote-tls"}})
                    .to_string()
                    .into(),
            ))
            .await
            .unwrap();
            assert!(matches!(
                ws.next().await.unwrap().unwrap(),
                Message::Close(_)
            ));
            ws.flush().await.unwrap();
        });
        let mut options = ConnectOptions::default();
        options.tls.additional_ca_pem = ca.pem().into_bytes();
        options.headers.push((
            "Authorization".into(),
            "Bearer remote-fixture-secret".into(),
        ));
        let url = format!("wss://{host}:{port}/echo");
        let (connection, events) = Connection::connect(&url, options).await.unwrap();
        drop(events);
        let response = connection
            .root_session()
            .send::<Echo>(Params {
                value: "remote-tls".into(),
            })
            .await
            .unwrap();
        assert_eq!(response.value, "remote-tls");
        assert!(connection.close().await.driver_resources_released);
        server.await.unwrap();
    })
    .await;
}

#[cfg(feature = "rustls")]
#[tokio::test]
#[ignore = "需要通过 YDP_EXTERNAL_WSS_URL、YDP_EXTERNAL_CA_PEM 和 YDP_EXTERNAL_AUTHORIZATION 指定异地主机 fixture"]
async fn external_tls_endpoint_validates_ca_hostname_and_authorization() {
    bounded(async {
        let url = std::env::var("YDP_EXTERNAL_WSS_URL").expect("必须提供异地主机 WSS URL");
        assert!(url.starts_with("wss://"));
        let ca_path = std::env::var("YDP_EXTERNAL_CA_PEM").expect("必须提供自定义 CA 文件路径");
        let authorization = std::env::var("YDP_EXTERNAL_AUTHORIZATION")
            .expect("必须提供异地主机 fixture 的 Authorization 值");
        let ca = std::fs::read(ca_path).expect("必须能读取自定义 CA 文件");
        let value = format!("ydp-external-tls-{}", std::process::id());
        let mut rejected = ConnectOptions::default();
        rejected.tls.additional_ca_pem = ca.clone();
        rejected
            .headers
            .push(("Authorization".into(), format!("{authorization}-invalid")));
        assert!(Connection::connect(&url, rejected).await.is_err());

        let mut options = ConnectOptions::default();
        options.tls.additional_ca_pem = ca;
        options
            .headers
            .push(("Authorization".into(), authorization));
        let (connection, events) = Connection::connect(&url, options).await.unwrap();
        drop(events);
        let response = connection
            .root_session()
            .send::<Echo>(Params {
                value: value.clone(),
            })
            .await
            .unwrap();
        assert_eq!(response.value, value);
        assert!(connection.close().await.driver_resources_released);
    })
    .await;
}

#[tokio::test]
async fn invalid_utf8_and_eof_terminate_pending_requests() {
    bounded(async {
        for invalid_utf8 in [false, true] {
            let (listener, url) = new_listener().await;
            let server = tokio::spawn(async move {
                let (socket, _) = listener.accept().await.unwrap();
                let mut ws = accept_async(socket).await.unwrap();
                assert!(matches!(
                    ws.next().await.unwrap().unwrap(),
                    Message::Text(_)
                ));
                if invalid_utf8 {
                    ws.send(Message::Frame(Frame::message(
                        Bytes::from_static(&[0xff]),
                        OpCode::Data(Data::Text),
                        true,
                    )))
                    .await
                    .unwrap();
                }
                // 销毁传输制造 EOF，不用等待定时器作为完成证据。
            });
            let (connection, events) = Connection::connect(&url, ConnectOptions::default())
                .await
                .unwrap();
            drop(events);
            let error = connection
                .root_session()
                .send::<Echo>(Params {
                    value: "pending".into(),
                })
                .await
                .err()
                .unwrap();
            assert!(matches!(error.kind, ErrorKind::Transport));
            assert!(connection.closed().await.driver_resources_released);
            server.await.unwrap();
        }
    })
    .await;
}

#[tokio::test]
async fn exact_inbound_handshake_limit_is_accepted() {
    bounded(async {
        let (listener, url) = new_listener().await;
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let request = read_handshake(&mut socket).await;
            socket
                .write_all(&padded_handshake(&request, 65536))
                .await
                .unwrap();
            let mut first = [0; 2];
            socket.read_exact(&mut first).await.unwrap();
            assert_eq!(first[0] & 0x0f, 8);
            socket.write_all(&[0x88, 0]).await.unwrap();
        });
        let (connection, events) = Connection::connect(&url, ConnectOptions::default())
            .await
            .unwrap();
        drop(events);
        assert!(connection.close().await.driver_resources_released);
        server.await.unwrap();
    })
    .await;
}

#[cfg(not(feature = "rustls"))]
#[tokio::test]
async fn disabled_tls_rejects_secure_endpoint_and_ca() {
    assert!(matches!(
        Connection::connect("wss://127.0.0.1:1", ConnectOptions::default())
            .await
            .err()
            .unwrap()
            .kind,
        ErrorKind::InvalidArgument
    ));
    let mut options = ConnectOptions::default();
    options.tls.additional_ca_pem = vec![1];
    assert!(matches!(
        Connection::connect("ws://127.0.0.1:1", options)
            .await
            .err()
            .unwrap()
            .kind,
        ErrorKind::InvalidArgument
    ));
}
