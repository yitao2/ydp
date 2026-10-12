///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Log.type.LogEntry/properties/source。
///
///协议原文：
///
///<pre>Log entry source.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum LogEntrySource {
    ///协议值 "xml"。
    Xml,
    ///协议值 "javascript"。
    Javascript,
    ///协议值 "network"。
    Network,
    ///协议值 "storage"。
    Storage,
    ///协议值 "appcache"。
    Appcache,
    ///协议值 "rendering"。
    Rendering,
    ///协议值 "security"。
    Security,
    ///协议值 "deprecation"。
    Deprecation,
    ///协议值 "worker"。
    Worker,
    ///协议值 "violation"。
    Violation,
    ///协议值 "intervention"。
    Intervention,
    ///协议值 "recommendation"。
    Recommendation,
    ///协议值 "other"。
    Other,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl LogEntrySource {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::log::types::LogEntrySource::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Xml => "xml",
            Self::Javascript => "javascript",
            Self::Network => "network",
            Self::Storage => "storage",
            Self::Appcache => "appcache",
            Self::Rendering => "rendering",
            Self::Security => "security",
            Self::Deprecation => "deprecation",
            Self::Worker => "worker",
            Self::Violation => "violation",
            Self::Intervention => "intervention",
            Self::Recommendation => "recommendation",
            Self::Other => "other",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for LogEntrySource {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for LogEntrySource {
    fn from(value: String) -> Self {
        match value.as_str() {
            "xml" => LogEntrySource::Xml,
            "javascript" => LogEntrySource::Javascript,
            "network" => LogEntrySource::Network,
            "storage" => LogEntrySource::Storage,
            "appcache" => LogEntrySource::Appcache,
            "rendering" => LogEntrySource::Rendering,
            "security" => LogEntrySource::Security,
            "deprecation" => LogEntrySource::Deprecation,
            "worker" => LogEntrySource::Worker,
            "violation" => LogEntrySource::Violation,
            "intervention" => LogEntrySource::Intervention,
            "recommendation" => LogEntrySource::Recommendation,
            "other" => LogEntrySource::Other,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for LogEntrySource {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LogEntrySource {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LogEntrySource;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "xml" => LogEntrySource::Xml,
                    "javascript" => LogEntrySource::Javascript,
                    "network" => LogEntrySource::Network,
                    "storage" => LogEntrySource::Storage,
                    "appcache" => LogEntrySource::Appcache,
                    "rendering" => LogEntrySource::Rendering,
                    "security" => LogEntrySource::Security,
                    "deprecation" => LogEntrySource::Deprecation,
                    "worker" => LogEntrySource::Worker,
                    "violation" => LogEntrySource::Violation,
                    "intervention" => LogEntrySource::Intervention,
                    "recommendation" => LogEntrySource::Recommendation,
                    "other" => LogEntrySource::Other,
                    _ => LogEntrySource::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(LogEntrySource::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Log.type.LogEntry/properties/level。
///
///协议原文：
///
///<pre>Log entry severity.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum LogEntryLevel {
    ///协议值 "verbose"。
    Verbose,
    ///协议值 "info"。
    Info,
    ///协议值 "warning"。
    Warning,
    ///协议值 "error"。
    Error,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl LogEntryLevel {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::log::types::LogEntryLevel::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Verbose => "verbose",
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for LogEntryLevel {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for LogEntryLevel {
    fn from(value: String) -> Self {
        match value.as_str() {
            "verbose" => LogEntryLevel::Verbose,
            "info" => LogEntryLevel::Info,
            "warning" => LogEntryLevel::Warning,
            "error" => LogEntryLevel::Error,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for LogEntryLevel {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LogEntryLevel {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LogEntryLevel;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "verbose" => LogEntryLevel::Verbose,
                    "info" => LogEntryLevel::Info,
                    "warning" => LogEntryLevel::Warning,
                    "error" => LogEntryLevel::Error,
                    _ => LogEntryLevel::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(LogEntryLevel::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Log.type.LogEntry/properties/category。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum LogEntryCategory {
    ///协议值 "cors"。
    Cors,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl LogEntryCategory {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::log::types::LogEntryCategory::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Cors => "cors",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for LogEntryCategory {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for LogEntryCategory {
    fn from(value: String) -> Self {
        match value.as_str() {
            "cors" => LogEntryCategory::Cors,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for LogEntryCategory {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LogEntryCategory {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LogEntryCategory;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "cors" => LogEntryCategory::Cors,
                    _ => LogEntryCategory::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(LogEntryCategory::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Log.type.LogEntry。
///
///协议原文：
///
///<pre>Log entry.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct LogEntry {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Log.type.LogEntry/properties/source。
    ///
    ///协议原文：
    ///
    ///<pre>Log entry source.</pre>
    ///
    ///
    #[serde(rename = "source")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub source: LogEntrySource,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Log.type.LogEntry/properties/level。
    ///
    ///协议原文：
    ///
    ///<pre>Log entry severity.</pre>
    ///
    ///
    #[serde(rename = "level")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub level: LogEntryLevel,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Log.type.LogEntry/properties/text。
    ///
    ///协议原文：
    ///
    ///<pre>Logged text.</pre>
    ///
    ///
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Log.type.LogEntry/properties/category。
    #[serde(rename = "category")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub category: Option<LogEntryCategory>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Log.type.LogEntry/properties/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Timestamp when this entry was added.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::runtime::types::Timestamp,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Log.type.LogEntry/properties/url。
    ///
    ///协议原文：
    ///
    ///<pre>URL of the resource if known.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Log.type.LogEntry/properties/lineNumber。
    ///
    ///协议原文：
    ///
    ///<pre>Line number in the resource.</pre>
    ///
    ///
    #[serde(rename = "lineNumber")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub line_number: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Log.type.LogEntry/properties/stackTrace。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript stack trace.</pre>
    ///
    ///
    #[serde(rename = "stackTrace")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub stack_trace: Option<crate::runtime::types::StackTrace>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Log.type.LogEntry/properties/networkRequestId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the network request associated with this entry.</pre>
    ///
    ///
    #[serde(rename = "networkRequestId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub network_request_id: Option<crate::network::types::RequestId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Log.type.LogEntry/properties/workerId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the worker associated with this entry.</pre>
    ///
    ///
    #[serde(rename = "workerId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub worker_id: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Log.type.LogEntry/properties/args。
    ///
    ///协议原文：
    ///
    ///<pre>Call arguments.</pre>
    ///
    ///
    #[serde(rename = "args")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub args: Option<Vec<crate::runtime::types::RemoteObject>>,
}
impl<'de> serde::Deserialize<'de> for LogEntry {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LogEntry;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Log.type.LogEntry/properties/source。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Log entry source.</pre>
                    ///
                    ///
                    #[serde(rename = "source")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub source: LogEntrySource,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Log.type.LogEntry/properties/level。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Log entry severity.</pre>
                    ///
                    ///
                    #[serde(rename = "level")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub level: LogEntryLevel,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Log.type.LogEntry/properties/text。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Logged text.</pre>
                    ///
                    ///
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Log.type.LogEntry/properties/category。
                    #[serde(rename = "category")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub category: Option<LogEntryCategory>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Log.type.LogEntry/properties/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Timestamp when this entry was added.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::runtime::types::Timestamp,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Log.type.LogEntry/properties/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>URL of the resource if known.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub url: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Log.type.LogEntry/properties/lineNumber。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Line number in the resource.</pre>
                    ///
                    ///
                    #[serde(rename = "lineNumber")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub line_number: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Log.type.LogEntry/properties/stackTrace。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript stack trace.</pre>
                    ///
                    ///
                    #[serde(rename = "stackTrace")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub stack_trace: Option<crate::runtime::types::StackTrace>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Log.type.LogEntry/properties/networkRequestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the network request associated with this entry.</pre>
                    ///
                    ///
                    #[serde(rename = "networkRequestId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub network_request_id: Option<crate::network::types::RequestId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Log.type.LogEntry/properties/workerId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the worker associated with this entry.</pre>
                    ///
                    ///
                    #[serde(rename = "workerId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub worker_id: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Log.type.LogEntry/properties/args。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Call arguments.</pre>
                    ///
                    ///
                    #[serde(rename = "args")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub args: Option<Vec<crate::runtime::types::RemoteObject>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LogEntry {
                    source: fields.source,
                    level: fields.level,
                    text: fields.text,
                    category: fields.category,
                    timestamp: fields.timestamp,
                    url: fields.url,
                    line_number: fields.line_number,
                    stack_trace: fields.stack_trace,
                    network_request_id: fields.network_request_id,
                    worker_id: fields.worker_id,
                    args: fields.args,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Log.type.ViolationSetting/properties/name。
///
///协议原文：
///
///<pre>Violation type.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ViolationSettingName {
    ///协议值 "longTask"。
    LongTask,
    ///协议值 "longLayout"。
    LongLayout,
    ///协议值 "blockedEvent"。
    BlockedEvent,
    ///协议值 "blockedParser"。
    BlockedParser,
    ///协议值 "discouragedAPIUse"。
    DiscouragedApiUse,
    ///协议值 "handler"。
    Handler,
    ///协议值 "recurringHandler"。
    RecurringHandler,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ViolationSettingName {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::log::types::ViolationSettingName::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::LongTask => "longTask",
            Self::LongLayout => "longLayout",
            Self::BlockedEvent => "blockedEvent",
            Self::BlockedParser => "blockedParser",
            Self::DiscouragedApiUse => "discouragedAPIUse",
            Self::Handler => "handler",
            Self::RecurringHandler => "recurringHandler",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ViolationSettingName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ViolationSettingName {
    fn from(value: String) -> Self {
        match value.as_str() {
            "longTask" => ViolationSettingName::LongTask,
            "longLayout" => ViolationSettingName::LongLayout,
            "blockedEvent" => ViolationSettingName::BlockedEvent,
            "blockedParser" => ViolationSettingName::BlockedParser,
            "discouragedAPIUse" => ViolationSettingName::DiscouragedApiUse,
            "handler" => ViolationSettingName::Handler,
            "recurringHandler" => ViolationSettingName::RecurringHandler,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ViolationSettingName {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ViolationSettingName {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ViolationSettingName;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "longTask" => ViolationSettingName::LongTask,
                    "longLayout" => ViolationSettingName::LongLayout,
                    "blockedEvent" => ViolationSettingName::BlockedEvent,
                    "blockedParser" => ViolationSettingName::BlockedParser,
                    "discouragedAPIUse" => ViolationSettingName::DiscouragedApiUse,
                    "handler" => ViolationSettingName::Handler,
                    "recurringHandler" => ViolationSettingName::RecurringHandler,
                    _ => ViolationSettingName::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ViolationSettingName::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Log.type.ViolationSetting。
///
///协议原文：
///
///<pre>Violation configuration setting.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ViolationSetting {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Log.type.ViolationSetting/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Violation type.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: ViolationSettingName,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Log.type.ViolationSetting/properties/threshold。
    ///
    ///协议原文：
    ///
    ///<pre>Time threshold to trigger upon.</pre>
    ///
    ///
    #[serde(rename = "threshold")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub threshold: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for ViolationSetting {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ViolationSetting;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Log.type.ViolationSetting/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Violation type.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: ViolationSettingName,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Log.type.ViolationSetting/properties/threshold。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Time threshold to trigger upon.</pre>
                    ///
                    ///
                    #[serde(rename = "threshold")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub threshold: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ViolationSetting {
                    name: fields.name,
                    threshold: fields.threshold,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
