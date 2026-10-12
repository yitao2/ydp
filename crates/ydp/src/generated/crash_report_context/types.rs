///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CrashReportContext.type.CrashReportContextEntry。
///
///协议原文：
///
///<pre>Key-value pair in CrashReportContext.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CrashReportContextEntry {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CrashReportContext.type.CrashReportContextEntry/properties/key。
    #[serde(rename = "key")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub key: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CrashReportContext.type.CrashReportContextEntry/properties/value。
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CrashReportContext.type.CrashReportContextEntry/properties/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>The ID of the frame where the key-value pair was set.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
}
impl<'de> serde::Deserialize<'de> for CrashReportContextEntry {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CrashReportContextEntry;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CrashReportContext.type.CrashReportContextEntry/properties/key。
                    #[serde(rename = "key")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub key: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CrashReportContext.type.CrashReportContextEntry/properties/value。
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CrashReportContext.type.CrashReportContextEntry/properties/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The ID of the frame where the key-value pair was set.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CrashReportContextEntry {
                    key: fields.key,
                    value: fields.value,
                    frame_id: fields.frame_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
