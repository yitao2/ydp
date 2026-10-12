///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Performance.type.Metric。
///
///协议原文：
///
///<pre>Run-time execution metric.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct Metric {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Performance.type.Metric/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Metric name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Performance.type.Metric/properties/value。
    ///
    ///协议原文：
    ///
    ///<pre>Metric value.</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for Metric {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Metric;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Performance.type.Metric/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Metric name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Performance.type.Metric/properties/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Metric value.</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Metric {
                    name: fields.name,
                    value: fields.value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
