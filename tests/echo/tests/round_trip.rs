//! 通过真实 envelope 验证独立生成 Echo 绑定与生产 core 的通信契约。
mod support;

use bytes::Bytes;
use futures_util::{FutureExt, StreamExt};
use std::{
    sync::{Arc, atomic::Ordering},
    time::Duration,
};
use support::memory;
use tokio::sync::Semaphore;
use ydp_core::{
    Connection, ErrorKind, RequestOptions, RuntimeOptions, SessionId, Submission, Timeout,
};
use ydp_echo_tests::SessionDomains as _;
use ydp_echo_tests::echo::{
    commands::{Echo, EchoParams},
    events::Echoed,
};

// 端点的 JSON 树仅用于测试脚本检查完整命令；生产 typed path 不经过此函数。
fn decode_command(bytes: &[u8]) -> serde_json::Value {
    serde_json::from_slice(bytes).expect("核心应发送合法 JSON")
}

async fn round_trip() {
    let (transport, mut endpoint) = memory(None);
    endpoint
        .incoming
        .send(Bytes::from_static(
            br#"{"sessionId":"earliest","method":"Echo.echoed","params":{"text":"first"}}"#,
        ))
        .await
        .unwrap();
    let (connection, mut all) =
        Connection::from_transport(transport, RuntimeOptions::default()).unwrap();
    let earliest = all.next().await.unwrap().unwrap();
    assert_eq!(earliest.session_id().unwrap().as_str(), "earliest");
    assert_eq!(earliest.receive_seq().get(), 1);
    let child = connection
        .adopt_session(SessionId::new("earliest".into()).unwrap())
        .unwrap();
    let again = connection
        .adopt_session(SessionId::new("earliest".into()).unwrap())
        .unwrap();
    assert_eq!(child.key(), again.key());
    let mut typed = child.events::<Echoed>(Default::default()).unwrap();
    let request = child.echo().echo(EchoParams {
        text: "你好，Echo".into(),
    });
    tokio::pin!(request);
    assert_eq!(request.submission(), Submission::NotSubmitted);
    assert!(request.request_id().is_none());
    assert!(endpoint.outgoing.try_recv().is_err());
    let server = async {
        let command = decode_command(&endpoint.outgoing.recv().await.unwrap());
        assert_eq!(command["method"], "Echo.echo");
        assert_eq!(command["sessionId"], "earliest");
        assert_eq!(command["params"]["text"], "你好，Echo");
        let id = command["id"].as_i64().unwrap();
        endpoint
            .incoming
            .send(Bytes::from(format!(
                r#"{{"id":{id},"sessionId":"earliest","result":{{"text":"你好，Echo"}}}}"#
            )))
            .await
            .unwrap();
        endpoint
            .incoming
            .send(Bytes::from_static(
                br#"{"sessionId":"earliest","method":"Echo.echoed","params":{"text":"event"}}"#,
            ))
            .await
            .unwrap();
    };
    let (response, ()) = tokio::join!(&mut request, server);
    assert_eq!(response.unwrap().text, "你好，Echo");
    assert_eq!(request.submission(), Submission::Responded);
    assert!(request.request_id().is_some());
    assert!(request.as_mut().now_or_never().is_none());
    let event = typed.next().await.unwrap().unwrap();
    assert_eq!(event.params.text, "event");
    let raw = all.next().await.unwrap().unwrap();
    assert_eq!(raw.receive_seq().get(), 3);
    assert_eq!(
        raw.params().decode::<serde_json::Value>().unwrap()["text"],
        "event"
    );
    child.close();
    assert!(again.is_closed());
    assert!(matches!(
        connection
            .adopt_session(SessionId::new("earliest".into()).unwrap())
            .err()
            .expect("retired ID 必须拒绝")
            .kind,
        ErrorKind::SessionIdRetired
    ));
    let report = connection.close().await;
    assert!(report.driver_resources_released);
    assert_eq!(endpoint.dropped_halves.load(Ordering::SeqCst), 2);
    assert!(endpoint.flushes.load(Ordering::SeqCst) > 0);
}

#[tokio::test(flavor = "current_thread")]
async fn r01_r17_r19_r21_generated_echo_current_thread() {
    tokio::time::timeout(Duration::from_secs(10), round_trip())
        .await
        .unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn r01_r17_r19_r21_generated_echo_multi_thread() {
    tokio::time::timeout(Duration::from_secs(10), round_trip())
        .await
        .unwrap();
}

async fn blocked_writer_response() {
    let gate = Arc::new(Semaphore::new(0));
    let (transport, mut endpoint) = memory(Some(gate.clone()));
    let (connection, events) = Connection::from_transport(transport, Default::default()).unwrap();
    drop(events);
    let root = connection.root_session();
    let request = root.send::<Echo>(EchoParams {
        text: "blocked".into(),
    });
    let server = async {
        let command = decode_command(&endpoint.outgoing.recv().await.unwrap());
        endpoint
            .incoming
            .send(Bytes::from(format!(
                r#"{{"id":{},"result":{{"text":"received while send pending"}}}}"#,
                command["id"]
            )))
            .await
            .unwrap();
    };
    let (response, ()) = tokio::join!(request, server);
    assert_eq!(response.unwrap().text, "received while send pending");
    assert_eq!(gate.available_permits(), 0);
    assert_eq!(endpoint.flushes.load(Ordering::SeqCst), 0);
    gate.add_permits(1);
    connection.close().await;
    assert_eq!(endpoint.dropped_halves.load(Ordering::SeqCst), 2);
}

#[tokio::test(flavor = "current_thread")]
async fn r11_reader_progresses_during_pending_write_current_thread() {
    tokio::time::timeout(Duration::from_secs(10), blocked_writer_response())
        .await
        .unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn r11_reader_progresses_during_pending_write_multi_thread() {
    tokio::time::timeout(Duration::from_secs(10), blocked_writer_response())
        .await
        .unwrap();
}

#[tokio::test(start_paused = true)]
async fn r05_zero_timeout_never_submits() {
    let (transport, mut endpoint) = memory(None);
    let (connection, events) = Connection::from_transport(transport, Default::default()).unwrap();
    drop(events);
    let root = connection.root_session();
    let request = root.send_with::<Echo>(
        EchoParams {
            text: "expired".into(),
        },
        RequestOptions {
            timeout: Timeout::After(Duration::ZERO),
            ..Default::default()
        },
    );
    tokio::pin!(request);
    let error = (&mut request).await.unwrap_err();
    assert!(matches!(error.kind, ErrorKind::Timeout));
    assert_eq!(request.submission(), Submission::NotSubmitted);
    assert!(endpoint.outgoing.try_recv().is_err());
    connection.close().await;
}

#[test]
fn r33_transport_requires_tokio_runtime() {
    let (transport, _) = memory(None);
    let result = Connection::from_transport(transport, Default::default());
    match result {
        Err(error) => assert!(matches!(error.kind, ErrorKind::NoRuntime)),
        Ok(_) => panic!("无 Tokio runtime 不应启动连接"),
    }
}

#[tokio::test]
async fn r02_protocol_error_preserves_explicit_null_and_submission() {
    let (transport, mut endpoint) = memory(None);
    let (connection, events) = Connection::from_transport(transport, Default::default()).unwrap();
    drop(events);
    let root = connection.root_session();
    let request = root.send::<Echo>(EchoParams {
        text: "error".into(),
    });
    tokio::pin!(request);
    let server = async {
        let command = decode_command(&endpoint.outgoing.recv().await.unwrap());
        endpoint
            .incoming
            .send(Bytes::from(format!(
                r#"{{"id":{},"error":{{"code":-32000,"message":"endpoint error","data":null}}}}"#,
                command["id"]
            )))
            .await
            .unwrap();
    };
    let (response, ()) = tokio::join!(&mut request, server);
    let error = response.unwrap_err();
    match &error.kind {
        ErrorKind::Protocol {
            code,
            message,
            data,
        } => {
            assert_eq!(*code, -32000);
            assert_eq!(message, "endpoint error");
            assert_eq!(data.as_ref().unwrap().as_bytes(), b"null");
        }
        other => panic!("错误分类不符：{other:?}"),
    }
    assert_eq!(request.submission(), Submission::Responded);
    assert_eq!(
        error.context.as_ref().unwrap().submission,
        Submission::Responded
    );
    assert!(error.context.as_ref().unwrap().receive_seq.is_some());
    connection.close().await;
}

#[tokio::test]
async fn r22_overflow_is_visible_outside_full_queue_and_other_stream_survives() {
    let (transport, endpoint) = memory(None);
    let options = RuntimeOptions {
        initial_events: ydp_core::SubscriptionOptions {
            max_events: 1,
            max_bytes: 1024,
        },
        ..Default::default()
    };
    let (connection, mut initial) = Connection::from_transport(transport, options).unwrap();
    let root = connection.root_session();
    let mut other = root.events::<Echoed>(Default::default()).unwrap();
    for text in ["one", "two"] {
        endpoint
            .incoming
            .send(Bytes::from(format!(
                r#"{{"method":"Echo.echoed","params":{{"text":"{text}"}}}}"#,
            )))
            .await
            .unwrap();
    }
    // 第二个精确事件已交付是明确的读取进度屏障；初始流此时已经处理同一消息。
    assert_eq!(other.next().await.unwrap().unwrap().params.text, "one");
    assert_eq!(other.next().await.unwrap().unwrap().params.text, "two");
    assert!(matches!(
        initial.next().await.unwrap().unwrap_err().kind,
        ErrorKind::EventBufferFull
    ));
    assert!(initial.next().await.is_none());
    endpoint
        .incoming
        .send(Bytes::from_static(
            br#"{"method":"Echo.echoed","params":{"text":"three"}}"#,
        ))
        .await
        .unwrap();
    assert_eq!(other.next().await.unwrap().unwrap().params.text, "three");
    connection.close().await;
}

#[tokio::test]
async fn r24_typed_decode_failure_only_terminates_its_subscription() {
    let (transport, endpoint) = memory(None);
    let (connection, mut all) = Connection::from_transport(transport, Default::default()).unwrap();
    let root = connection.root_session();
    let mut typed = root.events::<Echoed>(Default::default()).unwrap();
    endpoint
        .incoming
        .send(Bytes::from_static(
            br#"{"method":"Echo.echoed","params":{"text":1}}"#,
        ))
        .await
        .unwrap();
    let raw = all.next().await.unwrap().unwrap();
    assert_eq!(
        raw.params().decode::<serde_json::Value>().unwrap()["text"],
        1
    );
    assert!(matches!(
        typed.next().await.unwrap().unwrap_err().kind,
        ErrorKind::Decode
    ));
    assert!(typed.next().await.is_none());
    assert_eq!(connection.state(), ydp_core::ConnectionState::Open);
    connection.close().await;
}

async fn response_route_mismatch_only_terminates_request() {
    let (transport, mut endpoint) = memory(None);
    let (connection, events) = Connection::from_transport(transport, Default::default()).unwrap();
    drop(events);
    let child = connection
        .adopt_session(SessionId::new("child".into()).unwrap())
        .unwrap();
    let request = child.send::<Echo>(EchoParams {
        text: "route".into(),
    });
    let server = async {
        let command = decode_command(&endpoint.outgoing.recv().await.unwrap());
        endpoint
            .incoming
            .send(Bytes::from(format!(
                r#"{{"id":{},"sessionId":"wrong","result":{{"text":"route"}}}}"#,
                command["id"]
            )))
            .await
            .unwrap();
    };
    let (response, ()) = tokio::join!(request, server);
    assert!(matches!(
        response.unwrap_err().kind,
        ErrorKind::ResponseSessionMismatch
    ));
    assert_eq!(connection.state(), ydp_core::ConnectionState::Open);

    // 错路由 payload 没有改投其他 Session；后继根请求仍通过同一 Driver 正常收敛。
    let root_request = connection.root_session().send::<Echo>(EchoParams {
        text: "after-mismatch".into(),
    });
    let root_server = async {
        let command = decode_command(&endpoint.outgoing.recv().await.unwrap());
        assert!(command.get("sessionId").is_none());
        endpoint
            .incoming
            .send(Bytes::from(format!(
                r#"{{"id":{},"result":{{"text":"after-mismatch"}}}}"#,
                command["id"]
            )))
            .await
            .unwrap();
    };
    let (root_response, ()) = tokio::join!(root_request, root_server);
    assert_eq!(root_response.unwrap().text, "after-mismatch");
    let report = connection.close().await;
    assert!(matches!(report.reason.kind, ErrorKind::ConnectionClosed));
    assert_eq!(endpoint.dropped_halves.load(Ordering::SeqCst), 2);
}

#[tokio::test(flavor = "current_thread")]
async fn r29_response_route_mismatch_current_thread() {
    response_route_mismatch_only_terminates_request().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn r29_response_route_mismatch_multi_thread() {
    response_route_mismatch_only_terminates_request().await;
}

async fn queued_delivery_then_drop() {
    let (transport, mut endpoint) = memory(None);
    let mut options = RuntimeOptions::default();
    options.limits.normal.requests = 1;
    let (connection, mut events) = Connection::from_transport(transport, options).unwrap();
    let root = connection.root_session();
    let mut first = Box::pin(root.send::<Echo>(EchoParams {
        text: "first".into(),
    }));
    assert!(futures_util::poll!(first.as_mut()).is_pending());
    let command = decode_command(&endpoint.outgoing.recv().await.unwrap());
    endpoint
        .incoming
        .send(Bytes::from(format!(
            r#"{{"id":{},"result":{{"text":"first"}}}}"#,
            command["id"]
        )))
        .await
        .unwrap();
    endpoint
        .incoming
        .send(Bytes::from_static(
            br#"{"method":"Echo.echoed","params":{"text":"barrier"}}"#,
        ))
        .await
        .unwrap();
    // 同一 Reader 已处理后继事件，故第一响应必已从 pending 移交，而调用者尚未消费。
    assert_eq!(events.next().await.unwrap().unwrap().receive_seq().get(), 2);
    let mut second = Box::pin(root.send::<Echo>(EchoParams {
        text: "second".into(),
    }));
    assert!(futures_util::poll!(second.as_mut()).is_pending());
    assert_eq!(second.submission(), Submission::NotSubmitted);
    drop(first);
    let server = async {
        let command = decode_command(&endpoint.outgoing.recv().await.unwrap());
        endpoint
            .incoming
            .send(Bytes::from(format!(
                r#"{{"id":{},"result":{{"text":"second"}}}}"#,
                command["id"]
            )))
            .await
            .unwrap();
    };
    let (response, ()) = tokio::join!(second, server);
    assert_eq!(response.unwrap().text, "second");
    connection.close().await;
}

#[tokio::test(flavor = "current_thread")]
async fn r04_queued_delivery_keeps_permit_until_receiver_drop_current_thread() {
    tokio::time::timeout(Duration::from_secs(10), queued_delivery_then_drop())
        .await
        .unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn r04_queued_delivery_keeps_permit_until_receiver_drop_multi_thread() {
    tokio::time::timeout(Duration::from_secs(10), queued_delivery_then_drop())
        .await
        .unwrap();
}

#[tokio::test(start_paused = true)]
async fn r05_expired_deadline_wins_over_queued_response() {
    let (transport, mut endpoint) = memory(None);
    let (connection, mut events) =
        Connection::from_transport(transport, Default::default()).unwrap();
    let root = connection.root_session();
    let request = root.send_with::<Echo>(
        EchoParams {
            text: "deadline".into(),
        },
        RequestOptions {
            timeout: Timeout::After(Duration::from_secs(1)),
            ..Default::default()
        },
    );
    tokio::pin!(request);
    assert!(futures_util::poll!(request.as_mut()).is_pending());
    let command = decode_command(&endpoint.outgoing.recv().await.unwrap());
    endpoint
        .incoming
        .send(Bytes::from(format!(
            r#"{{"id":{},"result":{{"text":"queued"}}}}"#,
            command["id"]
        )))
        .await
        .unwrap();
    endpoint
        .incoming
        .send(Bytes::from_static(
            br#"{"method":"Echo.echoed","params":{}}"#,
        ))
        .await
        .unwrap();
    assert_eq!(events.next().await.unwrap().unwrap().receive_seq().get(), 2);
    tokio::time::advance(Duration::from_secs(1)).await;
    let error = request.as_mut().await.unwrap_err();
    assert!(matches!(error.kind, ErrorKind::Timeout));
    assert_eq!(request.submission(), Submission::PossiblySent);
    connection.close().await;
}

#[tokio::test]
async fn r08_close_budget_overflow_is_rejected_before_driver_start() {
    let (transport, endpoint) = memory(None);
    let options = RuntimeOptions {
        close_timeout: Duration::MAX,
        ..Default::default()
    };
    let result = Connection::from_transport(transport, options);
    match result {
        Err(error) => assert!(matches!(error.kind, ErrorKind::InvalidArgument)),
        Ok(_) => panic!("溢出关闭预算应在连接启动前拒绝"),
    }
    assert_eq!(endpoint.dropped_halves.load(Ordering::SeqCst), 2);
}
