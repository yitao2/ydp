///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Schema.type.Domain。
///
///协议原文：
///
///<pre>Description of the protocol domain.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct Domain {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Schema.type.Domain/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Domain name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Schema.type.Domain/properties/version。
    ///
    ///协议原文：
    ///
    ///<pre>Domain version.</pre>
    ///
    ///
    #[serde(rename = "version")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub version: String,
}
impl<'de> serde::Deserialize<'de> for Domain {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Domain;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Schema.type.Domain/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Domain name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Schema.type.Domain/properties/version。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Domain version.</pre>
                    ///
                    ///
                    #[serde(rename = "version")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub version: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Domain {
                    name: fields.name,
                    version: fields.version,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
