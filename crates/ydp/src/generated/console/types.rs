///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Console.type.ConsoleMessage/properties/source。
///
///协议原文：
///
///<pre>Message source.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConsoleMessageSource {
    ///协议值 "xml"。
    Xml,
    ///协议值 "javascript"。
    Javascript,
    ///协议值 "network"。
    Network,
    ///协议值 "console-api"。
    ConsoleApi,
    ///协议值 "storage"。
    Storage,
    ///协议值 "appcache"。
    Appcache,
    ///协议值 "rendering"。
    Rendering,
    ///协议值 "security"。
    Security,
    ///协议值 "other"。
    Other,
    ///协议值 "deprecation"。
    Deprecation,
    ///协议值 "worker"。
    Worker,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ConsoleMessageSource {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::console::types::ConsoleMessageSource::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Xml => "xml",
            Self::Javascript => "javascript",
            Self::Network => "network",
            Self::ConsoleApi => "console-api",
            Self::Storage => "storage",
            Self::Appcache => "appcache",
            Self::Rendering => "rendering",
            Self::Security => "security",
            Self::Other => "other",
            Self::Deprecation => "deprecation",
            Self::Worker => "worker",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ConsoleMessageSource {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ConsoleMessageSource {
    fn from(value: String) -> Self {
        match value.as_str() {
            "xml" => ConsoleMessageSource::Xml,
            "javascript" => ConsoleMessageSource::Javascript,
            "network" => ConsoleMessageSource::Network,
            "console-api" => ConsoleMessageSource::ConsoleApi,
            "storage" => ConsoleMessageSource::Storage,
            "appcache" => ConsoleMessageSource::Appcache,
            "rendering" => ConsoleMessageSource::Rendering,
            "security" => ConsoleMessageSource::Security,
            "other" => ConsoleMessageSource::Other,
            "deprecation" => ConsoleMessageSource::Deprecation,
            "worker" => ConsoleMessageSource::Worker,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ConsoleMessageSource {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ConsoleMessageSource {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ConsoleMessageSource;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "xml" => ConsoleMessageSource::Xml,
                    "javascript" => ConsoleMessageSource::Javascript,
                    "network" => ConsoleMessageSource::Network,
                    "console-api" => ConsoleMessageSource::ConsoleApi,
                    "storage" => ConsoleMessageSource::Storage,
                    "appcache" => ConsoleMessageSource::Appcache,
                    "rendering" => ConsoleMessageSource::Rendering,
                    "security" => ConsoleMessageSource::Security,
                    "other" => ConsoleMessageSource::Other,
                    "deprecation" => ConsoleMessageSource::Deprecation,
                    "worker" => ConsoleMessageSource::Worker,
                    _ => ConsoleMessageSource::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ConsoleMessageSource::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Console.type.ConsoleMessage/properties/level。
///
///协议原文：
///
///<pre>Message severity.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConsoleMessageLevel {
    ///协议值 "log"。
    Log,
    ///协议值 "warning"。
    Warning,
    ///协议值 "error"。
    Error,
    ///协议值 "debug"。
    Debug,
    ///协议值 "info"。
    Info,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ConsoleMessageLevel {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::console::types::ConsoleMessageLevel::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Log => "log",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ConsoleMessageLevel {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ConsoleMessageLevel {
    fn from(value: String) -> Self {
        match value.as_str() {
            "log" => ConsoleMessageLevel::Log,
            "warning" => ConsoleMessageLevel::Warning,
            "error" => ConsoleMessageLevel::Error,
            "debug" => ConsoleMessageLevel::Debug,
            "info" => ConsoleMessageLevel::Info,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ConsoleMessageLevel {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ConsoleMessageLevel {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ConsoleMessageLevel;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "log" => ConsoleMessageLevel::Log,
                    "warning" => ConsoleMessageLevel::Warning,
                    "error" => ConsoleMessageLevel::Error,
                    "debug" => ConsoleMessageLevel::Debug,
                    "info" => ConsoleMessageLevel::Info,
                    _ => ConsoleMessageLevel::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ConsoleMessageLevel::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Console.type.ConsoleMessage。
///
///协议原文：
///
///<pre>Console message.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ConsoleMessage {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Console.type.ConsoleMessage/properties/source。
    ///
    ///协议原文：
    ///
    ///<pre>Message source.</pre>
    ///
    ///
    #[serde(rename = "source")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub source: ConsoleMessageSource,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Console.type.ConsoleMessage/properties/level。
    ///
    ///协议原文：
    ///
    ///<pre>Message severity.</pre>
    ///
    ///
    #[serde(rename = "level")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub level: ConsoleMessageLevel,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Console.type.ConsoleMessage/properties/text。
    ///
    ///协议原文：
    ///
    ///<pre>Message text.</pre>
    ///
    ///
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Console.type.ConsoleMessage/properties/url。
    ///
    ///协议原文：
    ///
    ///<pre>URL of the message origin.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Console.type.ConsoleMessage/properties/line。
    ///
    ///协议原文：
    ///
    ///<pre>Line number in the resource that generated this message (1-based).</pre>
    ///
    ///
    #[serde(rename = "line")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub line: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Console.type.ConsoleMessage/properties/column。
    ///
    ///协议原文：
    ///
    ///<pre>Column number in the resource that generated this message (1-based).</pre>
    ///
    ///
    #[serde(rename = "column")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub column: Option<i64>,
}
impl<'de> serde::Deserialize<'de> for ConsoleMessage {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ConsoleMessage;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Console.type.ConsoleMessage/properties/source。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Message source.</pre>
                    ///
                    ///
                    #[serde(rename = "source")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub source: ConsoleMessageSource,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Console.type.ConsoleMessage/properties/level。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Message severity.</pre>
                    ///
                    ///
                    #[serde(rename = "level")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub level: ConsoleMessageLevel,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Console.type.ConsoleMessage/properties/text。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Message text.</pre>
                    ///
                    ///
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Console.type.ConsoleMessage/properties/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>URL of the message origin.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub url: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Console.type.ConsoleMessage/properties/line。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Line number in the resource that generated this message (1-based).</pre>
                    ///
                    ///
                    #[serde(rename = "line")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub line: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Console.type.ConsoleMessage/properties/column。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Column number in the resource that generated this message (1-based).</pre>
                    ///
                    ///
                    #[serde(rename = "column")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub column: Option<i64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ConsoleMessage {
                    source: fields.source,
                    level: fields.level,
                    text: fields.text,
                    url: fields.url,
                    line: fields.line,
                    column: fields.column,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
