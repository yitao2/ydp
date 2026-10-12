///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：SmartCardEmulation.type.ConnectionState。
///
///协议原文：
///
///<pre>Maps to |SCARD_*| connection state values.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConnectionState {
    ///协议值 "absent"。
    Absent,
    ///协议值 "present"。
    Present,
    ///协议值 "swallowed"。
    Swallowed,
    ///协议值 "powered"。
    Powered,
    ///协议值 "negotiable"。
    Negotiable,
    ///协议值 "specific"。
    Specific,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ConnectionState {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::smart_card_emulation::types::ConnectionState::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Absent => "absent",
            Self::Present => "present",
            Self::Swallowed => "swallowed",
            Self::Powered => "powered",
            Self::Negotiable => "negotiable",
            Self::Specific => "specific",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ConnectionState {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ConnectionState {
    fn from(value: String) -> Self {
        match value.as_str() {
            "absent" => ConnectionState::Absent,
            "present" => ConnectionState::Present,
            "swallowed" => ConnectionState::Swallowed,
            "powered" => ConnectionState::Powered,
            "negotiable" => ConnectionState::Negotiable,
            "specific" => ConnectionState::Specific,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ConnectionState {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ConnectionState {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ConnectionState;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "absent" => ConnectionState::Absent,
                    "present" => ConnectionState::Present,
                    "swallowed" => ConnectionState::Swallowed,
                    "powered" => ConnectionState::Powered,
                    "negotiable" => ConnectionState::Negotiable,
                    "specific" => ConnectionState::Specific,
                    _ => ConnectionState::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ConnectionState::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：SmartCardEmulation.type.Disposition。
///
///协议原文：
///
///<pre>Indicates what the reader should do with the card.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Disposition {
    ///协议值 "leave-card"。
    LeaveCard,
    ///协议值 "reset-card"。
    ResetCard,
    ///协议值 "unpower-card"。
    UnpowerCard,
    ///协议值 "eject-card"。
    EjectCard,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl Disposition {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::smart_card_emulation::types::Disposition::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::LeaveCard => "leave-card",
            Self::ResetCard => "reset-card",
            Self::UnpowerCard => "unpower-card",
            Self::EjectCard => "eject-card",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for Disposition {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for Disposition {
    fn from(value: String) -> Self {
        match value.as_str() {
            "leave-card" => Disposition::LeaveCard,
            "reset-card" => Disposition::ResetCard,
            "unpower-card" => Disposition::UnpowerCard,
            "eject-card" => Disposition::EjectCard,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for Disposition {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for Disposition {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Disposition;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "leave-card" => Disposition::LeaveCard,
                    "reset-card" => Disposition::ResetCard,
                    "unpower-card" => Disposition::UnpowerCard,
                    "eject-card" => Disposition::EjectCard,
                    _ => Disposition::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(Disposition::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：SmartCardEmulation.type.Protocol。
///
///协议原文：
///
///<pre>Maps to the |SCARD_PROTOCOL_*| values.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Protocol {
    ///协议值 "t0"。
    T0,
    ///协议值 "t1"。
    T1,
    ///协议值 "raw"。
    Raw,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl Protocol {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::smart_card_emulation::types::Protocol::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::T0 => "t0",
            Self::T1 => "t1",
            Self::Raw => "raw",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for Protocol {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for Protocol {
    fn from(value: String) -> Self {
        match value.as_str() {
            "t0" => Protocol::T0,
            "t1" => Protocol::T1,
            "raw" => Protocol::Raw,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for Protocol {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for Protocol {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Protocol;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "t0" => Protocol::T0,
                    "t1" => Protocol::T1,
                    "raw" => Protocol::Raw,
                    _ => Protocol::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(Protocol::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.type.ProtocolSet。
///
///协议原文：
///
///<pre>Maps to the |SCARD_PROTOCOL_*| flags.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProtocolSet {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.type.ProtocolSet/properties/t0。
    #[serde(rename = "t0")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub t0: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.type.ProtocolSet/properties/t1。
    #[serde(rename = "t1")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub t1: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.type.ProtocolSet/properties/raw。
    #[serde(rename = "raw")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub raw: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for ProtocolSet {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ProtocolSet;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.type.ProtocolSet/properties/t0。
                    #[serde(rename = "t0")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub t0: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.type.ProtocolSet/properties/t1。
                    #[serde(rename = "t1")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub t1: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.type.ProtocolSet/properties/raw。
                    #[serde(rename = "raw")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub raw: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ProtocolSet {
                    t0: fields.t0,
                    t1: fields.t1,
                    raw: fields.raw,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.type.ReaderStateFlags。
///
///协议原文：
///
///<pre>Maps to the |SCARD_STATE_*| flags.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReaderStateFlags {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.type.ReaderStateFlags/properties/unaware。
    #[serde(rename = "unaware")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub unaware: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.type.ReaderStateFlags/properties/ignore。
    #[serde(rename = "ignore")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.type.ReaderStateFlags/properties/changed。
    #[serde(rename = "changed")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub changed: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.type.ReaderStateFlags/properties/unknown。
    #[serde(rename = "unknown")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub unknown: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.type.ReaderStateFlags/properties/unavailable。
    #[serde(rename = "unavailable")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub unavailable: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.type.ReaderStateFlags/properties/empty。
    #[serde(rename = "empty")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub empty: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.type.ReaderStateFlags/properties/present。
    #[serde(rename = "present")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub present: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.type.ReaderStateFlags/properties/exclusive。
    #[serde(rename = "exclusive")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub exclusive: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.type.ReaderStateFlags/properties/inuse。
    #[serde(rename = "inuse")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub inuse: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.type.ReaderStateFlags/properties/mute。
    #[serde(rename = "mute")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub mute: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.type.ReaderStateFlags/properties/unpowered。
    #[serde(rename = "unpowered")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub unpowered: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for ReaderStateFlags {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReaderStateFlags;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.type.ReaderStateFlags/properties/unaware。
                    #[serde(rename = "unaware")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub unaware: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.type.ReaderStateFlags/properties/ignore。
                    #[serde(rename = "ignore")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub ignore: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.type.ReaderStateFlags/properties/changed。
                    #[serde(rename = "changed")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub changed: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.type.ReaderStateFlags/properties/unknown。
                    #[serde(rename = "unknown")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub unknown: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.type.ReaderStateFlags/properties/unavailable。
                    #[serde(rename = "unavailable")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub unavailable: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.type.ReaderStateFlags/properties/empty。
                    #[serde(rename = "empty")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub empty: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.type.ReaderStateFlags/properties/present。
                    #[serde(rename = "present")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub present: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.type.ReaderStateFlags/properties/exclusive。
                    #[serde(rename = "exclusive")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub exclusive: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.type.ReaderStateFlags/properties/inuse。
                    #[serde(rename = "inuse")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub inuse: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.type.ReaderStateFlags/properties/mute。
                    #[serde(rename = "mute")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub mute: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.type.ReaderStateFlags/properties/unpowered。
                    #[serde(rename = "unpowered")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub unpowered: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReaderStateFlags {
                    unaware: fields.unaware,
                    ignore: fields.ignore,
                    changed: fields.changed,
                    unknown: fields.unknown,
                    unavailable: fields.unavailable,
                    empty: fields.empty,
                    present: fields.present,
                    exclusive: fields.exclusive,
                    inuse: fields.inuse,
                    mute: fields.mute,
                    unpowered: fields.unpowered,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.type.ReaderStateIn。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReaderStateIn {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.type.ReaderStateIn/properties/reader。
    #[serde(rename = "reader")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub reader: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.type.ReaderStateIn/properties/currentState。
    #[serde(rename = "currentState")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub current_state: crate::smart_card_emulation::types::ReaderStateFlags,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.type.ReaderStateIn/properties/currentInsertionCount。
    #[serde(rename = "currentInsertionCount")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub current_insertion_count: i64,
}
impl<'de> serde::Deserialize<'de> for ReaderStateIn {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReaderStateIn;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.type.ReaderStateIn/properties/reader。
                    #[serde(rename = "reader")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub reader: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.type.ReaderStateIn/properties/currentState。
                    #[serde(rename = "currentState")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub current_state: crate::smart_card_emulation::types::ReaderStateFlags,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.type.ReaderStateIn/properties/currentInsertionCount。
                    #[serde(rename = "currentInsertionCount")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub current_insertion_count: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReaderStateIn {
                    reader: fields.reader,
                    current_state: fields.current_state,
                    current_insertion_count: fields.current_insertion_count,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.type.ReaderStateOut。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReaderStateOut {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.type.ReaderStateOut/properties/reader。
    #[serde(rename = "reader")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub reader: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.type.ReaderStateOut/properties/eventState。
    #[serde(rename = "eventState")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub event_state: crate::smart_card_emulation::types::ReaderStateFlags,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.type.ReaderStateOut/properties/eventCount。
    #[serde(rename = "eventCount")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub event_count: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.type.ReaderStateOut/properties/atr。
    #[serde(rename = "atr")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub atr: String,
}
impl<'de> serde::Deserialize<'de> for ReaderStateOut {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReaderStateOut;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.type.ReaderStateOut/properties/reader。
                    #[serde(rename = "reader")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub reader: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.type.ReaderStateOut/properties/eventState。
                    #[serde(rename = "eventState")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub event_state: crate::smart_card_emulation::types::ReaderStateFlags,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.type.ReaderStateOut/properties/eventCount。
                    #[serde(rename = "eventCount")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub event_count: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.type.ReaderStateOut/properties/atr。
                    #[serde(rename = "atr")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub atr: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReaderStateOut {
                    reader: fields.reader,
                    event_state: fields.event_state,
                    event_count: fields.event_count,
                    atr: fields.atr,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：SmartCardEmulation.type.ResultCode。
///
///协议原文：
///
///<pre>Indicates the PC/SC error code.
///
///This maps to:
///PC/SC Lite: https://pcsclite.apdu.fr/api/group__ErrorCodes.html
///Microsoft: https://learn.microsoft.com/en-us/windows/win32/secauthn/authentication-return-values</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ResultCode {
    ///协议值 "success"。
    Success,
    ///协议值 "removed-card"。
    RemovedCard,
    ///协议值 "reset-card"。
    ResetCard,
    ///协议值 "unpowered-card"。
    UnpoweredCard,
    ///协议值 "unresponsive-card"。
    UnresponsiveCard,
    ///协议值 "unsupported-card"。
    UnsupportedCard,
    ///协议值 "reader-unavailable"。
    ReaderUnavailable,
    ///协议值 "sharing-violation"。
    SharingViolation,
    ///协议值 "not-transacted"。
    NotTransacted,
    ///协议值 "no-smartcard"。
    NoSmartcard,
    ///协议值 "proto-mismatch"。
    ProtoMismatch,
    ///协议值 "system-cancelled"。
    SystemCancelled,
    ///协议值 "not-ready"。
    NotReady,
    ///协议值 "cancelled"。
    Cancelled,
    ///协议值 "insufficient-buffer"。
    InsufficientBuffer,
    ///协议值 "invalid-handle"。
    InvalidHandle,
    ///协议值 "invalid-parameter"。
    InvalidParameter,
    ///协议值 "invalid-value"。
    InvalidValue,
    ///协议值 "no-memory"。
    NoMemory,
    ///协议值 "timeout"。
    Timeout,
    ///协议值 "unknown-reader"。
    UnknownReader,
    ///协议值 "unsupported-feature"。
    UnsupportedFeature,
    ///协议值 "no-readers-available"。
    NoReadersAvailable,
    ///协议值 "service-stopped"。
    ServiceStopped,
    ///协议值 "no-service"。
    NoService,
    ///协议值 "comm-error"。
    CommError,
    ///协议值 "internal-error"。
    InternalError,
    ///协议值 "server-too-busy"。
    ServerTooBusy,
    ///协议值 "unexpected"。
    Unexpected,
    ///协议值 "shutdown"。
    Shutdown,
    ///协议值 "unknown-card"。
    UnknownCard,
    ///协议值 "unknown"。
    KnownUnknown,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ResultCode {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::smart_card_emulation::types::ResultCode::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Success => "success",
            Self::RemovedCard => "removed-card",
            Self::ResetCard => "reset-card",
            Self::UnpoweredCard => "unpowered-card",
            Self::UnresponsiveCard => "unresponsive-card",
            Self::UnsupportedCard => "unsupported-card",
            Self::ReaderUnavailable => "reader-unavailable",
            Self::SharingViolation => "sharing-violation",
            Self::NotTransacted => "not-transacted",
            Self::NoSmartcard => "no-smartcard",
            Self::ProtoMismatch => "proto-mismatch",
            Self::SystemCancelled => "system-cancelled",
            Self::NotReady => "not-ready",
            Self::Cancelled => "cancelled",
            Self::InsufficientBuffer => "insufficient-buffer",
            Self::InvalidHandle => "invalid-handle",
            Self::InvalidParameter => "invalid-parameter",
            Self::InvalidValue => "invalid-value",
            Self::NoMemory => "no-memory",
            Self::Timeout => "timeout",
            Self::UnknownReader => "unknown-reader",
            Self::UnsupportedFeature => "unsupported-feature",
            Self::NoReadersAvailable => "no-readers-available",
            Self::ServiceStopped => "service-stopped",
            Self::NoService => "no-service",
            Self::CommError => "comm-error",
            Self::InternalError => "internal-error",
            Self::ServerTooBusy => "server-too-busy",
            Self::Unexpected => "unexpected",
            Self::Shutdown => "shutdown",
            Self::UnknownCard => "unknown-card",
            Self::KnownUnknown => "unknown",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ResultCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ResultCode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "success" => ResultCode::Success,
            "removed-card" => ResultCode::RemovedCard,
            "reset-card" => ResultCode::ResetCard,
            "unpowered-card" => ResultCode::UnpoweredCard,
            "unresponsive-card" => ResultCode::UnresponsiveCard,
            "unsupported-card" => ResultCode::UnsupportedCard,
            "reader-unavailable" => ResultCode::ReaderUnavailable,
            "sharing-violation" => ResultCode::SharingViolation,
            "not-transacted" => ResultCode::NotTransacted,
            "no-smartcard" => ResultCode::NoSmartcard,
            "proto-mismatch" => ResultCode::ProtoMismatch,
            "system-cancelled" => ResultCode::SystemCancelled,
            "not-ready" => ResultCode::NotReady,
            "cancelled" => ResultCode::Cancelled,
            "insufficient-buffer" => ResultCode::InsufficientBuffer,
            "invalid-handle" => ResultCode::InvalidHandle,
            "invalid-parameter" => ResultCode::InvalidParameter,
            "invalid-value" => ResultCode::InvalidValue,
            "no-memory" => ResultCode::NoMemory,
            "timeout" => ResultCode::Timeout,
            "unknown-reader" => ResultCode::UnknownReader,
            "unsupported-feature" => ResultCode::UnsupportedFeature,
            "no-readers-available" => ResultCode::NoReadersAvailable,
            "service-stopped" => ResultCode::ServiceStopped,
            "no-service" => ResultCode::NoService,
            "comm-error" => ResultCode::CommError,
            "internal-error" => ResultCode::InternalError,
            "server-too-busy" => ResultCode::ServerTooBusy,
            "unexpected" => ResultCode::Unexpected,
            "shutdown" => ResultCode::Shutdown,
            "unknown-card" => ResultCode::UnknownCard,
            "unknown" => ResultCode::KnownUnknown,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ResultCode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ResultCode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResultCode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "success" => ResultCode::Success,
                    "removed-card" => ResultCode::RemovedCard,
                    "reset-card" => ResultCode::ResetCard,
                    "unpowered-card" => ResultCode::UnpoweredCard,
                    "unresponsive-card" => ResultCode::UnresponsiveCard,
                    "unsupported-card" => ResultCode::UnsupportedCard,
                    "reader-unavailable" => ResultCode::ReaderUnavailable,
                    "sharing-violation" => ResultCode::SharingViolation,
                    "not-transacted" => ResultCode::NotTransacted,
                    "no-smartcard" => ResultCode::NoSmartcard,
                    "proto-mismatch" => ResultCode::ProtoMismatch,
                    "system-cancelled" => ResultCode::SystemCancelled,
                    "not-ready" => ResultCode::NotReady,
                    "cancelled" => ResultCode::Cancelled,
                    "insufficient-buffer" => ResultCode::InsufficientBuffer,
                    "invalid-handle" => ResultCode::InvalidHandle,
                    "invalid-parameter" => ResultCode::InvalidParameter,
                    "invalid-value" => ResultCode::InvalidValue,
                    "no-memory" => ResultCode::NoMemory,
                    "timeout" => ResultCode::Timeout,
                    "unknown-reader" => ResultCode::UnknownReader,
                    "unsupported-feature" => ResultCode::UnsupportedFeature,
                    "no-readers-available" => ResultCode::NoReadersAvailable,
                    "service-stopped" => ResultCode::ServiceStopped,
                    "no-service" => ResultCode::NoService,
                    "comm-error" => ResultCode::CommError,
                    "internal-error" => ResultCode::InternalError,
                    "server-too-busy" => ResultCode::ServerTooBusy,
                    "unexpected" => ResultCode::Unexpected,
                    "shutdown" => ResultCode::Shutdown,
                    "unknown-card" => ResultCode::UnknownCard,
                    "unknown" => ResultCode::KnownUnknown,
                    _ => ResultCode::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ResultCode::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：SmartCardEmulation.type.ShareMode。
///
///协议原文：
///
///<pre>Maps to the |SCARD_SHARE_*| values.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ShareMode {
    ///协议值 "shared"。
    Shared,
    ///协议值 "exclusive"。
    Exclusive,
    ///协议值 "direct"。
    Direct,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ShareMode {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::smart_card_emulation::types::ShareMode::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Shared => "shared",
            Self::Exclusive => "exclusive",
            Self::Direct => "direct",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ShareMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ShareMode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "shared" => ShareMode::Shared,
            "exclusive" => ShareMode::Exclusive,
            "direct" => ShareMode::Direct,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ShareMode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ShareMode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ShareMode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "shared" => ShareMode::Shared,
                    "exclusive" => ShareMode::Exclusive,
                    "direct" => ShareMode::Direct,
                    _ => ShareMode::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ShareMode::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
