///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Schema.command.getDomains/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetDomainsParams {}
impl<'de> serde::Deserialize<'de> for GetDomainsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetDomainsParams;
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
                Ok(GetDomainsParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Schema.command.getDomains。
///
///协议原文：
///
///<pre>Returns supported domains.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetDomains;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Schema.command.getDomains/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetDomainsResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Schema.command.getDomains/returns/domains。
    ///
    ///协议原文：
    ///
    ///<pre>List of supported domains.</pre>
    ///
    ///
    #[serde(rename = "domains")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub domains: Vec<crate::schema::types::Domain>,
}
impl<'de> serde::Deserialize<'de> for GetDomainsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetDomainsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Schema.command.getDomains/returns/domains。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>List of supported domains.</pre>
                    ///
                    ///
                    #[serde(rename = "domains")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub domains: Vec<crate::schema::types::Domain>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetDomainsResponse {
                    domains: fields.domains,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetDomains {
    type Params = GetDomainsParams;
    type Response = GetDomainsResponse;
    const METHOD: &'static str = "Schema.getDomains";
}
