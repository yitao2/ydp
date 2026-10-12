///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Tracing.type.MemoryDumpConfig。
///
///协议原文：
///
///<pre>Configuration for memory dump. Used only when "memory-infra" category is enabled.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct MemoryDumpConfig(std::collections::BTreeMap<String, serde_json::Value>);
impl MemoryDumpConfig {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::tracing::types::MemoryDumpConfig = serde_json::from_str("{}").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::tracing::types::MemoryDumpConfig::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("{}").unwrap());
    ///```
    pub fn new(value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::tracing::types::MemoryDumpConfig = serde_json::from_str("{}").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::tracing::types::MemoryDumpConfig::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("{}").unwrap());
    ///```
    pub fn into_inner(self) -> std::collections::BTreeMap<String, serde_json::Value> {
        self.0
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Tracing.type.MemoryDumpLevelOfDetail。
///
///协议原文：
///
///<pre>Details exposed when memory request explicitly declared.
///Keep consistent with memory_dump_request_args.h and
///memory_instrumentation.mojom</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MemoryDumpLevelOfDetail {
    ///协议值 "background"。
    Background,
    ///协议值 "light"。
    Light,
    ///协议值 "detailed"。
    Detailed,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl MemoryDumpLevelOfDetail {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::tracing::types::MemoryDumpLevelOfDetail::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Background => "background",
            Self::Light => "light",
            Self::Detailed => "detailed",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for MemoryDumpLevelOfDetail {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for MemoryDumpLevelOfDetail {
    fn from(value: String) -> Self {
        match value.as_str() {
            "background" => MemoryDumpLevelOfDetail::Background,
            "light" => MemoryDumpLevelOfDetail::Light,
            "detailed" => MemoryDumpLevelOfDetail::Detailed,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for MemoryDumpLevelOfDetail {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MemoryDumpLevelOfDetail {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MemoryDumpLevelOfDetail;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "background" => MemoryDumpLevelOfDetail::Background,
                    "light" => MemoryDumpLevelOfDetail::Light,
                    "detailed" => MemoryDumpLevelOfDetail::Detailed,
                    _ => MemoryDumpLevelOfDetail::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(MemoryDumpLevelOfDetail::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Tracing.type.StreamCompression。
///
///协议原文：
///
///<pre>Compression type to use for traces returned via streams.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StreamCompression {
    ///协议值 "none"。
    None,
    ///协议值 "gzip"。
    Gzip,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl StreamCompression {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::tracing::types::StreamCompression::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::None => "none",
            Self::Gzip => "gzip",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for StreamCompression {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for StreamCompression {
    fn from(value: String) -> Self {
        match value.as_str() {
            "none" => StreamCompression::None,
            "gzip" => StreamCompression::Gzip,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for StreamCompression {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StreamCompression {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StreamCompression;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "none" => StreamCompression::None,
                    "gzip" => StreamCompression::Gzip,
                    _ => StreamCompression::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(StreamCompression::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Tracing.type.StreamFormat。
///
///协议原文：
///
///<pre>Data format of a trace. Can be either the legacy JSON format or the
///protocol buffer format. Note that the JSON format will be deprecated soon.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StreamFormat {
    ///协议值 "json"。
    Json,
    ///协议值 "proto"。
    Proto,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl StreamFormat {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::tracing::types::StreamFormat::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Json => "json",
            Self::Proto => "proto",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for StreamFormat {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for StreamFormat {
    fn from(value: String) -> Self {
        match value.as_str() {
            "json" => StreamFormat::Json,
            "proto" => StreamFormat::Proto,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for StreamFormat {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StreamFormat {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StreamFormat;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "json" => StreamFormat::Json,
                    "proto" => StreamFormat::Proto,
                    _ => StreamFormat::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(StreamFormat::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Tracing.type.TraceConfig/properties/recordMode。
///
///协议原文：
///
///<pre>Controls how the trace buffer stores data. The default is `recordUntilFull`.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TraceConfigRecordMode {
    ///协议值 "recordUntilFull"。
    RecordUntilFull,
    ///协议值 "recordContinuously"。
    RecordContinuously,
    ///协议值 "recordAsMuchAsPossible"。
    RecordAsMuchAsPossible,
    ///协议值 "echoToConsole"。
    EchoToConsole,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl TraceConfigRecordMode {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::tracing::types::TraceConfigRecordMode::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::RecordUntilFull => "recordUntilFull",
            Self::RecordContinuously => "recordContinuously",
            Self::RecordAsMuchAsPossible => "recordAsMuchAsPossible",
            Self::EchoToConsole => "echoToConsole",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for TraceConfigRecordMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for TraceConfigRecordMode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "recordUntilFull" => TraceConfigRecordMode::RecordUntilFull,
            "recordContinuously" => TraceConfigRecordMode::RecordContinuously,
            "recordAsMuchAsPossible" => TraceConfigRecordMode::RecordAsMuchAsPossible,
            "echoToConsole" => TraceConfigRecordMode::EchoToConsole,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for TraceConfigRecordMode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TraceConfigRecordMode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TraceConfigRecordMode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "recordUntilFull" => TraceConfigRecordMode::RecordUntilFull,
                    "recordContinuously" => TraceConfigRecordMode::RecordContinuously,
                    "recordAsMuchAsPossible" => TraceConfigRecordMode::RecordAsMuchAsPossible,
                    "echoToConsole" => TraceConfigRecordMode::EchoToConsole,
                    _ => TraceConfigRecordMode::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(TraceConfigRecordMode::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Tracing.type.TraceConfig。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TraceConfig {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.type.TraceConfig/properties/recordMode。
    ///
    ///协议原文：
    ///
    ///<pre>Controls how the trace buffer stores data. The default is `recordUntilFull`.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "recordMode")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub record_mode: Option<TraceConfigRecordMode>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.type.TraceConfig/properties/traceBufferSizeInKb。
    ///
    ///协议原文：
    ///
    ///<pre>Size of the trace buffer in kilobytes. If not specified or zero is passed, a default value
    ///of 200 MB would be used.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "traceBufferSizeInKb")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub trace_buffer_size_in_kb: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.type.TraceConfig/properties/enableSampling。
    ///
    ///协议原文：
    ///
    ///<pre>Turns on JavaScript stack sampling.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "enableSampling")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_sampling: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.type.TraceConfig/properties/enableSystrace。
    ///
    ///协议原文：
    ///
    ///<pre>Turns on system tracing.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "enableSystrace")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_systrace: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.type.TraceConfig/properties/enableArgumentFilter。
    ///
    ///协议原文：
    ///
    ///<pre>Turns on argument filter.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "enableArgumentFilter")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_argument_filter: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.type.TraceConfig/properties/includedCategories。
    ///
    ///协议原文：
    ///
    ///<pre>Included category filters.</pre>
    ///
    ///
    #[serde(rename = "includedCategories")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub included_categories: Option<Vec<String>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.type.TraceConfig/properties/excludedCategories。
    ///
    ///协议原文：
    ///
    ///<pre>Excluded category filters.</pre>
    ///
    ///
    #[serde(rename = "excludedCategories")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub excluded_categories: Option<Vec<String>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.type.TraceConfig/properties/syntheticDelays。
    ///
    ///协议原文：
    ///
    ///<pre>Configuration to synthesize the delays in tracing.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "syntheticDelays")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub synthetic_delays: Option<Vec<String>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.type.TraceConfig/properties/memoryDumpConfig。
    ///
    ///协议原文：
    ///
    ///<pre>Configuration for memory dump triggers. Used only when "memory-infra" category is enabled.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "memoryDumpConfig")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub memory_dump_config: Option<crate::tracing::types::MemoryDumpConfig>,
}
impl<'de> serde::Deserialize<'de> for TraceConfig {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TraceConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.type.TraceConfig/properties/recordMode。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Controls how the trace buffer stores data. The default is `recordUntilFull`.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "recordMode")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub record_mode: Option<TraceConfigRecordMode>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.type.TraceConfig/properties/traceBufferSizeInKb。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Size of the trace buffer in kilobytes. If not specified or zero is passed, a default value
                    ///of 200 MB would be used.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "traceBufferSizeInKb")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub trace_buffer_size_in_kb: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.type.TraceConfig/properties/enableSampling。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Turns on JavaScript stack sampling.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "enableSampling")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub enable_sampling: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.type.TraceConfig/properties/enableSystrace。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Turns on system tracing.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "enableSystrace")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub enable_systrace: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.type.TraceConfig/properties/enableArgumentFilter。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Turns on argument filter.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "enableArgumentFilter")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub enable_argument_filter: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.type.TraceConfig/properties/includedCategories。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Included category filters.</pre>
                    ///
                    ///
                    #[serde(rename = "includedCategories")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub included_categories: Option<Vec<String>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.type.TraceConfig/properties/excludedCategories。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Excluded category filters.</pre>
                    ///
                    ///
                    #[serde(rename = "excludedCategories")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub excluded_categories: Option<Vec<String>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.type.TraceConfig/properties/syntheticDelays。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Configuration to synthesize the delays in tracing.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "syntheticDelays")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub synthetic_delays: Option<Vec<String>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.type.TraceConfig/properties/memoryDumpConfig。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Configuration for memory dump triggers. Used only when "memory-infra" category is enabled.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "memoryDumpConfig")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub memory_dump_config: Option<crate::tracing::types::MemoryDumpConfig>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TraceConfig {
                    record_mode: fields.record_mode,
                    trace_buffer_size_in_kb: fields.trace_buffer_size_in_kb,
                    enable_sampling: fields.enable_sampling,
                    enable_systrace: fields.enable_systrace,
                    enable_argument_filter: fields.enable_argument_filter,
                    included_categories: fields.included_categories,
                    excluded_categories: fields.excluded_categories,
                    synthetic_delays: fields.synthetic_delays,
                    memory_dump_config: fields.memory_dump_config,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Tracing.type.TracingBackend。
///
///协议原文：
///
///<pre>Backend type to use for tracing. `chrome` uses the Chrome-integrated
///tracing service and is supported on all platforms. `system` is only
///supported on Chrome OS and uses the Perfetto system tracing service.
///`auto` chooses `system` when the perfettoConfig provided to Tracing.start
///specifies at least one non-Chrome data source; otherwise uses `chrome`.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TracingBackend {
    ///协议值 "auto"。
    Auto,
    ///协议值 "chrome"。
    Chrome,
    ///协议值 "system"。
    System,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl TracingBackend {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::tracing::types::TracingBackend::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Auto => "auto",
            Self::Chrome => "chrome",
            Self::System => "system",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for TracingBackend {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for TracingBackend {
    fn from(value: String) -> Self {
        match value.as_str() {
            "auto" => TracingBackend::Auto,
            "chrome" => TracingBackend::Chrome,
            "system" => TracingBackend::System,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for TracingBackend {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TracingBackend {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TracingBackend;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "auto" => TracingBackend::Auto,
                    "chrome" => TracingBackend::Chrome,
                    "system" => TracingBackend::System,
                    _ => TracingBackend::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(TracingBackend::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
