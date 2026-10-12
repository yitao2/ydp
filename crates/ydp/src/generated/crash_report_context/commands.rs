///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CrashReportContext.command.getEntries/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetEntriesParams {}
impl<'de> serde::Deserialize<'de> for GetEntriesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetEntriesParams;
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
                Ok(GetEntriesParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CrashReportContext.command.getEntries。
///
///协议原文：
///
///<pre>Returns all entries in the CrashReportContext across all frames in the page.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetEntries;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CrashReportContext.command.getEntries/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetEntriesResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CrashReportContext.command.getEntries/returns/entries。
    #[serde(rename = "entries")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub entries: Vec<crate::crash_report_context::types::CrashReportContextEntry>,
}
impl<'de> serde::Deserialize<'de> for GetEntriesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetEntriesResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CrashReportContext.command.getEntries/returns/entries。
                    #[serde(rename = "entries")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub entries: Vec<crate::crash_report_context::types::CrashReportContextEntry>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetEntriesResponse {
                    entries: fields.entries,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetEntries {
    type Params = GetEntriesParams;
    type Response = GetEntriesResponse;
    const METHOD: &'static str = "CrashReportContext.getEntries";
}
