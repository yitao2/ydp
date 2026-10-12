///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.event.issueAdded/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct IssueAddedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.event.issueAdded/parameters/issue。
    #[serde(rename = "issue")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub issue: crate::audits::types::InspectorIssue,
}
impl<'de> serde::Deserialize<'de> for IssueAddedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = IssueAddedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.event.issueAdded/parameters/issue。
                    #[serde(rename = "issue")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub issue: crate::audits::types::InspectorIssue,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(IssueAddedParams {
                    issue: fields.issue,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Audits.event.issueAdded。
#[derive(Debug, Clone, Copy)]
pub struct IssueAdded;
impl ydp_core::Event for IssueAdded {
    type Params = IssueAddedParams;
    const METHOD: &'static str = "Audits.issueAdded";
}
