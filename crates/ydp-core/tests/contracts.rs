//! 公开并发与借用边界的编译期合同；测试体只负责让编译器实例化约束。

use bytes::Bytes;
use serde::ser::{Serialize, Serializer};
use std::{cell::Cell, future::pending};
use ydp_core::{
    Command, Connection, EventStream, MessageReader, MessageWriter, RawEventStream, RawJson,
    RawRequest, Request, Transport, TransportError,
};

struct SendOnlyParams(Cell<u64>);

impl Serialize for SendOnlyParams {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0.get())
    }
}

struct SendOnlyCommand;

impl Command for SendOnlyCommand {
    type Params = SendOnlyParams;
    type Response = String;
    const METHOD: &'static str = "Compile.sendOnly";
}

struct OwnedEvent;

impl ydp_core::Event for OwnedEvent {
    type Params = String;
    const METHOD: &'static str = "Compile.owned";
}

struct Reader;
struct Writer;
struct HandwrittenTransport;

impl MessageReader for Reader {
    async fn recv(&mut self) -> Result<Option<Bytes>, TransportError> {
        pending().await
    }
}

impl MessageWriter for Writer {
    async fn send(&mut self, _bytes: Bytes) -> Result<(), TransportError> {
        Ok(())
    }

    async fn flush(&mut self) -> Result<(), TransportError> {
        Ok(())
    }

    async fn close(&mut self) -> Result<(), TransportError> {
        Ok(())
    }
}

impl Transport for HandwrittenTransport {
    type Reader = Reader;
    type Writer = Writer;

    fn split(self) -> (Self::Reader, Self::Writer) {
        (Reader, Writer)
    }
}

fn assert_send<T: Send>() {}

fn borrow_json_bytes(value: &RawJson) -> &[u8] {
    value.as_bytes()
}

#[test]
fn public_async_owners_and_send_only_parameters_are_send() {
    assert_send::<Connection>();
    assert_send::<Request<SendOnlyCommand>>();
    assert_send::<RawRequest>();
    assert_send::<RawEventStream>();
    assert_send::<EventStream<OwnedEvent>>();
    assert_send::<HandwrittenTransport>();
}

#[test]
fn raw_json_borrow_is_tied_to_its_owner() {
    let value = RawJson::from_bytes(Bytes::from_static(br#"{"secret":"borrowed"}"#)).unwrap();
    let bytes = borrow_json_bytes(&value);
    assert_eq!(bytes, br#"{"secret":"borrowed"}"#);
}
