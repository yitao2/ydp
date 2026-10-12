///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Performance.event.metrics/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct MetricsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Performance.event.metrics/parameters/metrics。
    ///
    ///协议原文：
    ///
    ///<pre>Current values of the metrics.</pre>
    ///
    ///
    #[serde(rename = "metrics")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub metrics: Vec<crate::performance::types::Metric>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Performance.event.metrics/parameters/title。
    ///
    ///协议原文：
    ///
    ///<pre>Timestamp title.</pre>
    ///
    ///
    #[serde(rename = "title")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub title: String,
}
impl<'de> serde::Deserialize<'de> for MetricsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MetricsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Performance.event.metrics/parameters/metrics。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Current values of the metrics.</pre>
                    ///
                    ///
                    #[serde(rename = "metrics")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub metrics: Vec<crate::performance::types::Metric>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Performance.event.metrics/parameters/title。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Timestamp title.</pre>
                    ///
                    ///
                    #[serde(rename = "title")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub title: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(MetricsParams {
                    metrics: fields.metrics,
                    title: fields.title,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Performance.event.metrics。
///
///协议原文：
///
///<pre>Current values of the metrics.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Metrics;
impl ydp_core::Event for Metrics {
    type Params = MetricsParams;
    const METHOD: &'static str = "Performance.metrics";
}
