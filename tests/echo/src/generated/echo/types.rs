///仅接受空 object 的协议类型。协议来源：Echo.type.Closed。
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct ClosedValue {}
impl<'de> serde::Deserialize<'de> for ClosedValue {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClosedValue;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("空 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                mut a: A,
            ) -> Result<Self::Value, A::Error> {
                if a.next_key::<serde::de::IgnoredAny>()?.is_some() {
                    return Err(serde::de::Error::custom("空 object 不允许字段"));
                }
                Ok(ClosedValue {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Echo.type.Closed。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Closed(ClosedValue);
impl Closed {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp_echo_tests::echo::types::Closed = serde_json::from_str("{}").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp_echo_tests::echo::types::Closed::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("{}").unwrap());
    ///```
    pub fn new(value: ClosedValue) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp_echo_tests::echo::types::Closed = serde_json::from_str("{}").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp_echo_tests::echo::types::Closed::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("{}").unwrap());
    ///```
    pub fn into_inner(self) -> ClosedValue {
        self.0
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Echo.type.Label。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Label(String);
impl Label {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp_echo_tests::echo::types::Label = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp_echo_tests::echo::types::Label::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp_echo_tests::echo::types::Label = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp_echo_tests::echo::types::Label::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp_echo_tests::echo::types::Label::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Echo.type.MaybeMode。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MaybeModeValue {
    ///协议值 "ready"。
    Ready,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl MaybeModeValue {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp_echo_tests::echo::types::MaybeModeValue::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ready => "ready",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for MaybeModeValue {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for MaybeModeValue {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ready" => MaybeModeValue::Ready,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for MaybeModeValue {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MaybeModeValue {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MaybeModeValue;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "ready" => MaybeModeValue::Ready,
                    _ => MaybeModeValue::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(MaybeModeValue::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Echo.type.MaybeMode。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct MaybeMode(ydp_core::binding_support::Nullable<MaybeModeValue>);
impl MaybeMode {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp_echo_tests::echo::types::MaybeMode = serde_json::from_str("null").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp_echo_tests::echo::types::MaybeMode::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("null").unwrap());
    ///```
    pub fn new(value: ydp_core::binding_support::Nullable<MaybeModeValue>) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp_echo_tests::echo::types::MaybeMode = serde_json::from_str("null").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp_echo_tests::echo::types::MaybeMode::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("null").unwrap());
    ///```
    pub fn into_inner(self) -> ydp_core::binding_support::Nullable<MaybeModeValue> {
        self.0
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Echo.type.MaybeRecord。
#[derive(Debug, Clone, serde::Serialize)]
pub struct MaybeRecordValue {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Echo.type.MaybeRecord/properties/text。
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: String,
}
impl<'de> serde::Deserialize<'de> for MaybeRecordValue {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MaybeRecordValue;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Echo.type.MaybeRecord/properties/text。
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(MaybeRecordValue { text: fields.text })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Echo.type.MaybeRecord。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct MaybeRecord(ydp_core::binding_support::Nullable<MaybeRecordValue>);
impl MaybeRecord {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp_echo_tests::echo::types::MaybeRecord = serde_json::from_str("null").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp_echo_tests::echo::types::MaybeRecord::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("null").unwrap());
    ///```
    pub fn new(value: ydp_core::binding_support::Nullable<MaybeRecordValue>) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp_echo_tests::echo::types::MaybeRecord = serde_json::from_str("null").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp_echo_tests::echo::types::MaybeRecord::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("null").unwrap());
    ///```
    pub fn into_inner(self) -> ydp_core::binding_support::Nullable<MaybeRecordValue> {
        self.0
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Echo.type.Mode。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Mode {
    ///协议值 "ready"。
    Ready,
    ///协议值 "Unknown"。
    KnownUnknown,
    ///协议值 "future"。
    Future,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl Mode {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp_echo_tests::echo::types::Mode::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ready => "ready",
            Self::KnownUnknown => "Unknown",
            Self::Future => "future",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for Mode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for Mode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ready" => Mode::Ready,
            "Unknown" => Mode::KnownUnknown,
            "future" => Mode::Future,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for Mode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for Mode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Mode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "ready" => Mode::Ready,
                    "Unknown" => Mode::KnownUnknown,
                    "future" => Mode::Future,
                    _ => Mode::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(Mode::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Echo.type.Record。
#[derive(Debug, Clone, serde::Serialize)]
pub struct Record {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Echo.type.Record/properties/text。
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Echo.type.Record/properties/note。
    #[serde(rename = "note")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub note: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Echo.type.Record/properties/extra。
    #[serde(rename = "extra")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub extra: Option<serde_json::Value>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Echo.type.Record/properties/maybe。
    #[serde(rename = "maybe")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub maybe: ydp_core::binding_support::Nullable<String>,
}
impl<'de> serde::Deserialize<'de> for Record {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Record;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Echo.type.Record/properties/text。
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Echo.type.Record/properties/note。
                    #[serde(rename = "note")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub note: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Echo.type.Record/properties/extra。
                    #[serde(rename = "extra")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub extra: Option<serde_json::Value>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Echo.type.Record/properties/maybe。
                    #[serde(rename = "maybe")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub maybe: ydp_core::binding_support::Nullable<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Record {
                    text: fields.text,
                    note: fields.note,
                    extra: fields.extra,
                    maybe: fields.maybe,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
