///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Cast.type.Sink。
#[derive(Debug, Clone, serde::Serialize)]
pub struct Sink {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Cast.type.Sink/properties/name。
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Cast.type.Sink/properties/id。
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Cast.type.Sink/properties/session。
    ///
    ///协议原文：
    ///
    ///<pre>Text describing the current session. Present only if there is an active
    ///session on the sink.</pre>
    ///
    ///
    #[serde(rename = "session")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub session: Option<String>,
}
impl<'de> serde::Deserialize<'de> for Sink {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Sink;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Cast.type.Sink/properties/name。
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Cast.type.Sink/properties/id。
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Cast.type.Sink/properties/session。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Text describing the current session. Present only if there is an active
                    ///session on the sink.</pre>
                    ///
                    ///
                    #[serde(rename = "session")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub session: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Sink {
                    name: fields.name,
                    id: fields.id,
                    session: fields.session,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
