//! 严格 CDP envelope 扫描与直接编码，不构造领域 Value 树。
use crate::{Error, ErrorKind, Result};
use serde::{
    Deserialize, Deserializer, Serialize,
    de::{self, MapAccess, Visitor},
};
use serde_json::value::RawValue;
use std::{borrow::Cow, fmt, io::Write};

/// 已验证的借用 envelope，所有 payload 生命周期受原消息限制。
#[derive(Debug)]
pub(crate) struct Envelope<'a> {
    /// 响应的正 i32 请求编号；事件没有编号。
    pub(crate) id: Option<i32>,
    /// 已解码的方法名，无转义时借用原消息。
    pub(crate) method: Option<Cow<'a, str>>,
    /// 已解码的可选 child 路由，根路由省略。
    pub(crate) session_id: Option<Cow<'a, str>>,
    /// 成功响应对象的原始片段。
    pub(crate) result: Option<&'a RawValue>,
    /// 事件参数对象；缺失时由运行时使用空对象。
    pub(crate) params: Option<&'a RawValue>,
    /// 协议错误的已验证字段。
    pub(crate) error: Option<ProtocolError<'a>>,
}
/// 端点错误，保留 data 缺失与明确 null 的区别。
#[derive(Debug)]
pub(crate) struct ProtocolError<'a> {
    /// 有符号 32 位协议错误码。
    pub(crate) code: i32,
    /// 端点提供的错误消息。
    pub(crate) message: Cow<'a, str>,
    /// 可选的任意 JSON 值，包括 null。
    pub(crate) data: Option<&'a RawValue>,
}

struct Text<'a>(Cow<'a, str>);
impl<'de> Deserialize<'de> for Text<'de> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        struct TextVisitor;
        impl<'de> Visitor<'de> for TextVisitor {
            type Value = Text<'de>;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("字符串")
            }
            fn visit_borrowed_str<E: de::Error>(
                self,
                v: &'de str,
            ) -> std::result::Result<Self::Value, E> {
                Ok(Text(Cow::Borrowed(v)))
            }
            fn visit_str<E: de::Error>(self, v: &str) -> std::result::Result<Self::Value, E> {
                Ok(Text(Cow::Owned(v.into())))
            }
            fn visit_string<E: de::Error>(self, v: String) -> std::result::Result<Self::Value, E> {
                Ok(Text(Cow::Owned(v)))
            }
        }
        d.deserialize_str(TextVisitor)
    }
}
impl<'de> Deserialize<'de> for Envelope<'de> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        struct EnvelopeVisitor;
        impl<'de> Visitor<'de> for EnvelopeVisitor {
            type Value = Envelope<'de>;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("CDP envelope 对象")
            }
            fn visit_map<M: MapAccess<'de>>(
                self,
                mut map: M,
            ) -> std::result::Result<Self::Value, M::Error> {
                let mut v = Envelope {
                    id: None,
                    method: None,
                    session_id: None,
                    result: None,
                    params: None,
                    error: None,
                };
                while let Some(Text(key)) = map.next_key::<Text<'de>>()? {
                    match key.as_ref() {
                        "id" => {
                            duplicate(v.id.is_some())?;
                            let id = map.next_value::<i32>()?;
                            if id <= 0 {
                                return Err(de::Error::custom("请求编号必须为正 i32"));
                            }
                            v.id = Some(id);
                        }
                        "method" => {
                            duplicate(v.method.is_some())?;
                            let Text(s) = map.next_value()?;
                            validate_text(&s, true)?;
                            v.method = Some(s);
                        }
                        "sessionId" => {
                            duplicate(v.session_id.is_some())?;
                            let Text(s) = map.next_value()?;
                            validate_text(&s, false)?;
                            v.session_id = Some(s);
                        }
                        "result" => {
                            duplicate(v.result.is_some())?;
                            let raw = map.next_value::<&RawValue>()?;
                            object(raw)?;
                            v.result = Some(raw);
                        }
                        "params" => {
                            duplicate(v.params.is_some())?;
                            let raw = map.next_value::<&RawValue>()?;
                            object(raw)?;
                            v.params = Some(raw);
                        }
                        "error" => {
                            duplicate(v.error.is_some())?;
                            v.error = Some(map.next_value()?);
                        }
                        _ => {
                            let _ = map.next_value::<&RawValue>()?;
                        }
                    }
                }
                if v.id.is_some() {
                    if v.method.is_some() || v.result.is_some() == v.error.is_some() {
                        return Err(de::Error::custom("响应必须且仅有 result 或 error"));
                    }
                } else if v.method.is_none() || v.result.is_some() || v.error.is_some() {
                    return Err(de::Error::custom("事件 envelope 字段不合法"));
                }
                Ok(v)
            }
        }
        d.deserialize_map(EnvelopeVisitor)
    }
}
impl<'de> Deserialize<'de> for ProtocolError<'de> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        struct ErrorVisitor;
        impl<'de> Visitor<'de> for ErrorVisitor {
            type Value = ProtocolError<'de>;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("协议 error 对象")
            }
            fn visit_map<M: MapAccess<'de>>(
                self,
                mut map: M,
            ) -> std::result::Result<Self::Value, M::Error> {
                let (mut code, mut message, mut data) = (None, None, None);
                while let Some(Text(key)) = map.next_key::<Text<'de>>()? {
                    match key.as_ref() {
                        "code" => {
                            duplicate(code.is_some())?;
                            code = Some(map.next_value::<i32>()?);
                        }
                        "message" => {
                            duplicate(message.is_some())?;
                            message = Some(map.next_value::<Text<'de>>()?.0);
                        }
                        "data" => {
                            duplicate(data.is_some())?;
                            data = Some(map.next_value::<&RawValue>()?);
                        }
                        _ => {
                            let _ = map.next_value::<&RawValue>()?;
                        }
                    }
                }
                Ok(ProtocolError {
                    code: code.ok_or_else(|| de::Error::missing_field("code"))?,
                    message: message.ok_or_else(|| de::Error::missing_field("message"))?,
                    data,
                })
            }
        }
        d.deserialize_map(ErrorVisitor)
    }
}
fn duplicate<E: de::Error>(present: bool) -> std::result::Result<(), E> {
    if present {
        Err(E::custom("重复的已知字段"))
    } else {
        Ok(())
    }
}
fn object<E: de::Error>(raw: &RawValue) -> std::result::Result<(), E> {
    if raw.get().starts_with('{') {
        Ok(())
    } else {
        Err(E::custom("字段必须为对象"))
    }
}
fn validate_text<E: de::Error>(value: &str, method: bool) -> std::result::Result<(), E> {
    if value.is_empty()
        || value.len() > 1024
        || (method && value.bytes().any(|b| b.is_ascii_control()))
    {
        Err(E::custom("方法或路由字符串不合法"))
    } else {
        Ok(())
    }
}

/// 扫描容器深度；示例调用 `check_depth(b"{}", 128)` 成功，单元测试覆盖边界。
/// 此函数不替代 JSON 语法解析；引号与转义内括号不计数。
pub(crate) fn check_depth(bytes: &[u8], max_depth: usize) -> Result<()> {
    let (mut depth, mut string, mut escape) = (0usize, false, false);
    for &b in bytes {
        if string {
            if escape {
                escape = false;
            } else if b == b'\\' {
                escape = true;
            } else if b == b'"' {
                string = false;
            }
        } else {
            match b {
                b'"' => string = true,
                b'{' | b'[' => {
                    depth += 1;
                    if depth > max_depth.min(128) {
                        return Err(Error::new(ErrorKind::InvalidMessage));
                    }
                }
                b'}' | b']' => depth = depth.saturating_sub(1),
                _ => {}
            }
        }
    }
    Ok(())
}
/// 校验消息字节、深度和完整 envelope；例如 `parse(b"{\"id\":1,\"result\":{}}", 1024, 128)`。
/// 返回值借用输入；语法和结构错误为 InvalidMessage，超过消息上限为 ResourceLimit。
pub(crate) fn parse(bytes: &[u8], max_message: usize, max_depth: usize) -> Result<Envelope<'_>> {
    if bytes.len() > max_message.min(128 * 1024 * 1024) {
        return Err(Error::new(ErrorKind::ResourceLimit {
            resource: "入站消息字节",
        }));
    }
    check_depth(bytes, max_depth)?;
    serde_json::from_slice(bytes)
        .map_err(|error| Error::with_source(ErrorKind::InvalidMessage, error))
}

/// 直接写入命令 envelope；例如把 `EmptyParams {}` 编码到有界 writer。
/// 参数仅编码一次；writer 的类型化配额原因由调用者保留并优先分类。
pub(crate) fn encode<P: Serialize, W: Write>(
    id: i32,
    method: &str,
    session_id: Option<&str>,
    params: &P,
    mut writer: W,
) -> Result<()> {
    if id <= 0
        || validate_text::<serde_json::Error>(method, true).is_err()
        || session_id.is_some_and(|s| validate_text::<serde_json::Error>(s, false).is_err())
    {
        return Err(Error::new(ErrorKind::InvalidArgument));
    }
    fn io(error: std::io::Error) -> Error {
        Error::with_source(ErrorKind::Encode, error)
    }
    fn json(error: serde_json::Error) -> Error {
        Error::with_source(ErrorKind::Encode, error)
    }
    write!(writer, "{{\"id\":{id},\"method\":").map_err(io)?;
    serde_json::to_writer(&mut writer, method).map_err(json)?;
    if let Some(session) = session_id {
        writer.write_all(b",\"sessionId\":").map_err(io)?;
        serde_json::to_writer(&mut writer, session).map_err(json)?;
    }
    writer.write_all(b",\"params\":").map_err(io)?;
    let mut shape = ObjectWriter {
        inner: &mut writer,
        seen: false,
        object: false,
    };
    serde_json::to_writer(&mut shape, params).map_err(json)?;
    if !shape.object {
        return Err(Error::new(ErrorKind::Encode));
    }
    writer.write_all(b"}").map_err(io)?;
    Ok(())
}
struct ObjectWriter<'a, W> {
    inner: &'a mut W,
    seen: bool,
    object: bool,
}
impl<W: Write> Write for ObjectWriter<'_, W> {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let written = self.inner.write(bytes)?;
        if !self.seen
            && let Some(b) = bytes[..written].iter().find(|b| !b.is_ascii_whitespace())
        {
            self.seen = true;
            self.object = *b == b'{';
        }
        Ok(written)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

#[cfg(test)]
#[path = "wire/fuzz_tests.rs"]
mod fuzz_tests;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid_and_escaped() {
        let v = parse(
            br#"{"method":"Echo.\u0070ing","sessionId":"a\u0062","params":{}}"#,
            1024,
            128,
        )
        .unwrap();
        assert_eq!(v.method.as_deref(), Some("Echo.ping"));
        assert_eq!(v.session_id.as_deref(), Some("ab"));
        let v = parse(
            br#"{"id":2147483647,"error":{"code":-1,"message":"bad","data":null}}"#,
            1024,
            128,
        )
        .unwrap();
        assert_eq!(v.error.unwrap().data.unwrap().get(), "null");
        assert!(
            parse(br#"{"method":"Echo.ping"}"#, 1024, 128)
                .unwrap()
                .params
                .is_none()
        );
    }
    #[test]
    fn malformed_envelopes() {
        for input in [
            r#"{"id":0,"result":{}}"#,
            r#"{"id":1.0,"result":{}}"#,
            r#"{"id":2147483648,"result":{}}"#,
            r#"{"id":null,"result":{}}"#,
            r#"{"id":1,"id":2,"result":{}}"#,
            r#"{"id":1}"#,
            r#"{"id":1,"result":null}"#,
            r#"{"id":1,"result":{},"error":{}}"#,
            r#"{"method":"a","params":null}"#,
            r#"{"method":"a","sessionId":null}"#,
            r#"{"method":"a","sessionId":""}"#,
            r#"{"method":"a","method":"a"}"#,
            r#"{"id":1,"error":{"code":0,"message":"a","data":null,"data":null}}"#,
            r#"{"method":"a","unknown":[}"#,
            r#"[]"#,
        ] {
            assert!(parse(input.as_bytes(), 1024, 128).is_err(), "{input}");
        }
        assert!(parse(&[0xff], 1024, 128).is_err());
    }
    #[test]
    fn bounds_and_borrowing() {
        assert!(check_depth(br#"{"value":"[[[\\\""}"#, 1).is_ok());
        assert!(parse(br#"{"method":"a","extra":[[]]}"#, 1024, 2).is_err());
        assert!(parse(br#"{"method":"a"}"#, 1, 128).is_err());
        let bytes = br#"{"id":1,"result":{"value":7}}"#;
        let v = parse(bytes, 1024, 128).unwrap();
        let raw = v.result.unwrap();
        assert!(raw.get().as_ptr() >= bytes.as_ptr());
        assert!(matches!(
            parse(br#"{"method":"Echo.ping"}"#, 1024, 128)
                .unwrap()
                .method,
            Some(Cow::Borrowed(_))
        ));
    }
    #[test]
    fn hard_depth_and_all_known_field_types() {
        let at_limit = format!(
            "{{\"method\":\"a\",\"extension\":{}0{}}}",
            "[".repeat(127),
            "]".repeat(127)
        );
        assert!(parse(at_limit.as_bytes(), 4096, 128).is_ok());
        let beyond = format!(
            "{{\"method\":\"a\",\"extension\":{}0{}}}",
            "[".repeat(128),
            "]".repeat(128)
        );
        assert!(parse(beyond.as_bytes(), 4096, usize::MAX).is_err());
        for input in [
            r#"{"id":-1,"result":{}}"#,
            r#"{"id":"1","result":{}}"#,
            r#"{"id":1e0,"result":{}}"#,
            r#"{"id":1,"result":{},"result":{}}"#,
            r#"{"id":1,"error":null}"#,
            r#"{"id":1,"error":[]}"#,
            r#"{"id":1,"error":{"code":2147483648,"message":"x"}}"#,
            r#"{"id":1,"error":{"code":0,"message":null}}"#,
            r#"{"id":1,"error":{"code":0,"message":"x","code":0}}"#,
            r#"{"id":1,"error":{"code":0,"message":"x","message":"x"}}"#,
            r#"{"id":1,"error":{"message":"x"}}"#,
            r#"{"id":1,"error":{"code":0}}"#,
            r#"{"method":null}"#,
            r#"{"method":[]}"#,
            r#"{"method":""}"#,
            r#"{"method":"a\u0000"}"#,
            r#"{"method":"a","params":[],"params":{}}"#,
            r#"{"method":"a","params":{},"params":{}}"#,
            r#"{"method":"a","sessionId":"b","sessionId":"b"}"#,
            r#"{"method":"a","extension":{"unfinished":}}"#,
        ] {
            assert!(parse(input.as_bytes(), 4096, 128).is_err(), "{input}");
        }
        let long_method = format!("{{\"method\":\"{}\"}}", "x".repeat(1025));
        assert!(parse(long_method.as_bytes(), 4096, 128).is_err());
        let absent = parse(
            br#"{"id":1,"error":{"code":-2147483648,"message":"x"}}"#,
            1024,
            128,
        )
        .unwrap()
        .error
        .unwrap();
        assert_eq!(absent.code, i32::MIN);
        assert_eq!(absent.message, "x");
        assert!(absent.data.is_none());
        assert!(parse(br#"{"method":"a","extension":1,"extension":2}"#, 1024, 128).is_ok());
    }

    #[test]
    fn encoding_objects_only() {
        let mut output = Vec::new();
        encode(
            1,
            "Echo.\"ping",
            Some("a\\b"),
            &crate::binding_support::EmptyParams {},
            &mut output,
        )
        .unwrap();
        let value: serde_json::Value = serde_json::from_slice(&output).unwrap();
        assert_eq!(value["method"], "Echo.\"ping");
        assert!(value["params"].is_object());
        assert!(encode(1, "a", None, &(), Vec::new()).is_err());
        assert!(
            encode(
                0,
                "a",
                None,
                &crate::binding_support::EmptyParams {},
                Vec::new()
            )
            .is_err()
        );
    }
}
