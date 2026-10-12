///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Cast.event.issueUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct IssueUpdatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Cast.event.issueUpdated/parameters/issueMessage。
    #[serde(rename = "issueMessage")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub issue_message: String,
}
impl<'de> serde::Deserialize<'de> for IssueUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = IssueUpdatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Cast.event.issueUpdated/parameters/issueMessage。
                    #[serde(rename = "issueMessage")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub issue_message: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(IssueUpdatedParams {
                    issue_message: fields.issue_message,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Cast.event.issueUpdated。
///
///协议原文：
///
///<pre>This is fired whenever the outstanding issue/error message changes.
///|issueMessage| is empty if there is no issue.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct IssueUpdated;
impl ydp_core::Event for IssueUpdated {
    type Params = IssueUpdatedParams;
    const METHOD: &'static str = "Cast.issueUpdated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Cast.event.sinksUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SinksUpdatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Cast.event.sinksUpdated/parameters/sinks。
    #[serde(rename = "sinks")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub sinks: Vec<crate::cast::types::Sink>,
}
impl<'de> serde::Deserialize<'de> for SinksUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SinksUpdatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Cast.event.sinksUpdated/parameters/sinks。
                    #[serde(rename = "sinks")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub sinks: Vec<crate::cast::types::Sink>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SinksUpdatedParams {
                    sinks: fields.sinks,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Cast.event.sinksUpdated。
///
///协议原文：
///
///<pre>This is fired whenever the list of available sinks changes. A sink is a
///device or a software surface that you can cast to.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SinksUpdated;
impl ydp_core::Event for SinksUpdated {
    type Params = SinksUpdatedParams;
    const METHOD: &'static str = "Cast.sinksUpdated";
}
