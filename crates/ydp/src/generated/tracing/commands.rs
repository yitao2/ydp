///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Tracing.command.end/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct EndParams {}
impl<'de> serde::Deserialize<'de> for EndParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EndParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EndParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Tracing.command.end。
///
///协议原文：
///
///<pre>Stop trace events collection.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct End;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Tracing.command.end/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EndResponse {}
impl<'de> serde::Deserialize<'de> for EndResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EndResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EndResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for End {
    type Params = EndParams;
    type Response = EndResponse;
    const METHOD: &'static str = "Tracing.end";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Tracing.command.getCategories/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetCategoriesParams {}
impl<'de> serde::Deserialize<'de> for GetCategoriesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetCategoriesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetCategoriesParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Tracing.command.getCategories。
///
///协议原文：
///
///<pre>Gets supported tracing categories.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetCategories;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Tracing.command.getCategories/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetCategoriesResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Tracing.command.getCategories/returns/categories。
    ///
    ///协议原文：
    ///
    ///<pre>A list of supported tracing categories.</pre>
    ///
    ///
    #[serde(rename = "categories")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub categories: Vec<String>,
}
impl<'de> serde::Deserialize<'de> for GetCategoriesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetCategoriesResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Tracing.command.getCategories/returns/categories。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A list of supported tracing categories.</pre>
                    ///
                    ///
                    #[serde(rename = "categories")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub categories: Vec<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetCategoriesResponse {
                    categories: fields.categories,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetCategories {
    type Params = GetCategoriesParams;
    type Response = GetCategoriesResponse;
    const METHOD: &'static str = "Tracing.getCategories";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Tracing.command.getTrackEventDescriptor/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetTrackEventDescriptorParams {}
impl<'de> serde::Deserialize<'de> for GetTrackEventDescriptorParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetTrackEventDescriptorParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetTrackEventDescriptorParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Tracing.command.getTrackEventDescriptor。
///
///协议原文：
///
///<pre>Return a descriptor for all available tracing categories.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetTrackEventDescriptor;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Tracing.command.getTrackEventDescriptor/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetTrackEventDescriptorResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Tracing.command.getTrackEventDescriptor/returns/descriptor。
    ///
    ///协议原文：
    ///
    ///<pre>Base64-encoded serialized perfetto.protos.TrackEventDescriptor protobuf message. (Encoded as a base64 string when passed over JSON)</pre>
    ///
    ///
    #[serde(rename = "descriptor")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub descriptor: String,
}
impl<'de> serde::Deserialize<'de> for GetTrackEventDescriptorResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetTrackEventDescriptorResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Tracing.command.getTrackEventDescriptor/returns/descriptor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Base64-encoded serialized perfetto.protos.TrackEventDescriptor protobuf message. (Encoded as a base64 string when passed over JSON)</pre>
                    ///
                    ///
                    #[serde(rename = "descriptor")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub descriptor: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetTrackEventDescriptorResponse {
                    descriptor: fields.descriptor,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetTrackEventDescriptor {
    type Params = GetTrackEventDescriptorParams;
    type Response = GetTrackEventDescriptorResponse;
    const METHOD: &'static str = "Tracing.getTrackEventDescriptor";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Tracing.command.recordClockSyncMarker/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RecordClockSyncMarkerParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Tracing.command.recordClockSyncMarker/parameters/syncId。
    ///
    ///协议原文：
    ///
    ///<pre>The ID of this clock sync marker</pre>
    ///
    ///
    #[serde(rename = "syncId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub sync_id: String,
}
impl<'de> serde::Deserialize<'de> for RecordClockSyncMarkerParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RecordClockSyncMarkerParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Tracing.command.recordClockSyncMarker/parameters/syncId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The ID of this clock sync marker</pre>
                    ///
                    ///
                    #[serde(rename = "syncId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub sync_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RecordClockSyncMarkerParams {
                    sync_id: fields.sync_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Tracing.command.recordClockSyncMarker。
///
///协议原文：
///
///<pre>Record a clock sync marker in the trace.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct RecordClockSyncMarker;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Tracing.command.recordClockSyncMarker/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RecordClockSyncMarkerResponse {}
impl<'de> serde::Deserialize<'de> for RecordClockSyncMarkerResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RecordClockSyncMarkerResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RecordClockSyncMarkerResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RecordClockSyncMarker {
    type Params = RecordClockSyncMarkerParams;
    type Response = RecordClockSyncMarkerResponse;
    const METHOD: &'static str = "Tracing.recordClockSyncMarker";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Tracing.command.requestMemoryDump/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct RequestMemoryDumpParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.command.requestMemoryDump/parameters/deterministic。
    ///
    ///协议原文：
    ///
    ///<pre>Enables more deterministic results by forcing garbage collection</pre>
    ///
    ///
    #[serde(rename = "deterministic")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub deterministic: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.command.requestMemoryDump/parameters/levelOfDetail。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies level of details in memory dump. Defaults to "detailed".</pre>
    ///
    ///
    #[serde(rename = "levelOfDetail")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub level_of_detail: Option<crate::tracing::types::MemoryDumpLevelOfDetail>,
}
impl<'de> serde::Deserialize<'de> for RequestMemoryDumpParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RequestMemoryDumpParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.command.requestMemoryDump/parameters/deterministic。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Enables more deterministic results by forcing garbage collection</pre>
                    ///
                    ///
                    #[serde(rename = "deterministic")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub deterministic: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.command.requestMemoryDump/parameters/levelOfDetail。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies level of details in memory dump. Defaults to "detailed".</pre>
                    ///
                    ///
                    #[serde(rename = "levelOfDetail")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub level_of_detail: Option<crate::tracing::types::MemoryDumpLevelOfDetail>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RequestMemoryDumpParams {
                    deterministic: fields.deterministic,
                    level_of_detail: fields.level_of_detail,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Tracing.command.requestMemoryDump。
///
///协议原文：
///
///<pre>Request a global memory dump.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct RequestMemoryDump;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Tracing.command.requestMemoryDump/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequestMemoryDumpResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Tracing.command.requestMemoryDump/returns/dumpGuid。
    ///
    ///协议原文：
    ///
    ///<pre>GUID of the resulting global memory dump.</pre>
    ///
    ///
    #[serde(rename = "dumpGuid")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub dump_guid: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Tracing.command.requestMemoryDump/returns/success。
    ///
    ///协议原文：
    ///
    ///<pre>True iff the global memory dump succeeded.</pre>
    ///
    ///
    #[serde(rename = "success")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub success: bool,
}
impl<'de> serde::Deserialize<'de> for RequestMemoryDumpResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RequestMemoryDumpResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Tracing.command.requestMemoryDump/returns/dumpGuid。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>GUID of the resulting global memory dump.</pre>
                    ///
                    ///
                    #[serde(rename = "dumpGuid")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub dump_guid: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Tracing.command.requestMemoryDump/returns/success。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True iff the global memory dump succeeded.</pre>
                    ///
                    ///
                    #[serde(rename = "success")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub success: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RequestMemoryDumpResponse {
                    dump_guid: fields.dump_guid,
                    success: fields.success,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RequestMemoryDump {
    type Params = RequestMemoryDumpParams;
    type Response = RequestMemoryDumpResponse;
    const METHOD: &'static str = "Tracing.requestMemoryDump";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Tracing.command.start/parameters/transferMode。
///
///协议原文：
///
///<pre>Whether to report trace events as series of dataCollected events or to save trace to a
///stream (defaults to `ReportEvents`).</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StartParamsTransferMode {
    ///协议值 "ReportEvents"。
    ReportEvents,
    ///协议值 "ReturnAsStream"。
    ReturnAsStream,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl StartParamsTransferMode {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::tracing::commands::StartParamsTransferMode::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::ReportEvents => "ReportEvents",
            Self::ReturnAsStream => "ReturnAsStream",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for StartParamsTransferMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for StartParamsTransferMode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ReportEvents" => StartParamsTransferMode::ReportEvents,
            "ReturnAsStream" => StartParamsTransferMode::ReturnAsStream,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for StartParamsTransferMode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StartParamsTransferMode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StartParamsTransferMode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "ReportEvents" => StartParamsTransferMode::ReportEvents,
                    "ReturnAsStream" => StartParamsTransferMode::ReturnAsStream,
                    _ => StartParamsTransferMode::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(StartParamsTransferMode::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Tracing.command.start/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct StartParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.command.start/parameters/categories。
    ///
    ///协议原文：
    ///
    ///<pre>Category/tag filter</pre>
    ///
    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "categories")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub categories: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.command.start/parameters/options。
    ///
    ///协议原文：
    ///
    ///<pre>Tracing options</pre>
    ///
    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "options")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub options: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.command.start/parameters/bufferUsageReportingInterval。
    ///
    ///协议原文：
    ///
    ///<pre>If set, the agent will issue bufferUsage events at this interval, specified in milliseconds</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "bufferUsageReportingInterval")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub buffer_usage_reporting_interval: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.command.start/parameters/transferMode。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to report trace events as series of dataCollected events or to save trace to a
    ///stream (defaults to `ReportEvents`).</pre>
    ///
    ///
    #[serde(rename = "transferMode")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub transfer_mode: Option<StartParamsTransferMode>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.command.start/parameters/streamFormat。
    ///
    ///协议原文：
    ///
    ///<pre>Trace data format to use. This only applies when using `ReturnAsStream`
    ///transfer mode (defaults to `json`).</pre>
    ///
    ///
    #[serde(rename = "streamFormat")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub stream_format: Option<crate::tracing::types::StreamFormat>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.command.start/parameters/streamCompression。
    ///
    ///协议原文：
    ///
    ///<pre>Compression format to use. This only applies when using `ReturnAsStream`
    ///transfer mode (defaults to `none`)</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "streamCompression")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub stream_compression: Option<crate::tracing::types::StreamCompression>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.command.start/parameters/traceConfig。
    #[serde(rename = "traceConfig")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub trace_config: Option<crate::tracing::types::TraceConfig>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.command.start/parameters/perfettoConfig。
    ///
    ///协议原文：
    ///
    ///<pre>Base64-encoded serialized perfetto.protos.TraceConfig protobuf message
    ///When specified, the parameters `categories`, `options`, `traceConfig`
    ///are ignored. (Encoded as a base64 string when passed over JSON)</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "perfettoConfig")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub perfetto_config: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.command.start/parameters/tracingBackend。
    ///
    ///协议原文：
    ///
    ///<pre>Backend type (defaults to `auto`)</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "tracingBackend")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub tracing_backend: Option<crate::tracing::types::TracingBackend>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.command.start/parameters/screenshotMaxSize。
    ///
    ///协议原文：
    ///
    ///<pre>Maximum width and height (in pixels) of each captured screenshot.
    ///Only used when the `disabled-by-default-devtools.screenshot` category is
    ///enabled. Defaults to 500. The combined memory footprint of screenshots
    ///(`screenshotMaxSize` * `screenshotMaxSize` * 4 * `screenshotMaxCount`)
    ///is clamped to the existing per-session budget.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "screenshotMaxSize")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub screenshot_max_size: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.command.start/parameters/screenshotMaxCount。
    ///
    ///协议原文：
    ///
    ///<pre>Maximum number of screenshots captured during a single tracing session.
    ///Only used when the `disabled-by-default-devtools.screenshot` category is
    ///enabled. Defaults to 450. Clamped together with `screenshotMaxSize` to
    ///stay within the per-session screenshot memory budget.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "screenshotMaxCount")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub screenshot_max_count: Option<i64>,
}
impl<'de> serde::Deserialize<'de> for StartParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StartParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.command.start/parameters/categories。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Category/tag filter</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "categories")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub categories: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.command.start/parameters/options。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Tracing options</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "options")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub options: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.command.start/parameters/bufferUsageReportingInterval。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If set, the agent will issue bufferUsage events at this interval, specified in milliseconds</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "bufferUsageReportingInterval")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub buffer_usage_reporting_interval: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.command.start/parameters/transferMode。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to report trace events as series of dataCollected events or to save trace to a
                    ///stream (defaults to `ReportEvents`).</pre>
                    ///
                    ///
                    #[serde(rename = "transferMode")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub transfer_mode: Option<StartParamsTransferMode>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.command.start/parameters/streamFormat。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Trace data format to use. This only applies when using `ReturnAsStream`
                    ///transfer mode (defaults to `json`).</pre>
                    ///
                    ///
                    #[serde(rename = "streamFormat")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub stream_format: Option<crate::tracing::types::StreamFormat>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.command.start/parameters/streamCompression。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Compression format to use. This only applies when using `ReturnAsStream`
                    ///transfer mode (defaults to `none`)</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "streamCompression")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub stream_compression: Option<crate::tracing::types::StreamCompression>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.command.start/parameters/traceConfig。
                    #[serde(rename = "traceConfig")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub trace_config: Option<crate::tracing::types::TraceConfig>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.command.start/parameters/perfettoConfig。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Base64-encoded serialized perfetto.protos.TraceConfig protobuf message
                    ///When specified, the parameters `categories`, `options`, `traceConfig`
                    ///are ignored. (Encoded as a base64 string when passed over JSON)</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "perfettoConfig")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub perfetto_config: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.command.start/parameters/tracingBackend。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Backend type (defaults to `auto`)</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "tracingBackend")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub tracing_backend: Option<crate::tracing::types::TracingBackend>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.command.start/parameters/screenshotMaxSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Maximum width and height (in pixels) of each captured screenshot.
                    ///Only used when the `disabled-by-default-devtools.screenshot` category is
                    ///enabled. Defaults to 500. The combined memory footprint of screenshots
                    ///(`screenshotMaxSize` * `screenshotMaxSize` * 4 * `screenshotMaxCount`)
                    ///is clamped to the existing per-session budget.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "screenshotMaxSize")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub screenshot_max_size: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Tracing.command.start/parameters/screenshotMaxCount。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Maximum number of screenshots captured during a single tracing session.
                    ///Only used when the `disabled-by-default-devtools.screenshot` category is
                    ///enabled. Defaults to 450. Clamped together with `screenshotMaxSize` to
                    ///stay within the per-session screenshot memory budget.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "screenshotMaxCount")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub screenshot_max_count: Option<i64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StartParams {
                    categories: fields.categories,
                    options: fields.options,
                    buffer_usage_reporting_interval: fields.buffer_usage_reporting_interval,
                    transfer_mode: fields.transfer_mode,
                    stream_format: fields.stream_format,
                    stream_compression: fields.stream_compression,
                    trace_config: fields.trace_config,
                    perfetto_config: fields.perfetto_config,
                    tracing_backend: fields.tracing_backend,
                    screenshot_max_size: fields.screenshot_max_size,
                    screenshot_max_count: fields.screenshot_max_count,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Tracing.command.start。
///
///协议原文：
///
///<pre>Start trace events collection.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Start;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Tracing.command.start/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StartResponse {}
impl<'de> serde::Deserialize<'de> for StartResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StartResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StartResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Start {
    type Params = StartParams;
    type Response = StartResponse;
    const METHOD: &'static str = "Tracing.start";
}
