///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：PerformanceTimeline.event.timelineEventAdded/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TimelineEventAddedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.event.timelineEventAdded/parameters/event。
    #[serde(rename = "event")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub event: crate::performance_timeline::types::TimelineEvent,
}
impl<'de> serde::Deserialize<'de> for TimelineEventAddedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TimelineEventAddedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.event.timelineEventAdded/parameters/event。
                    #[serde(rename = "event")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub event: crate::performance_timeline::types::TimelineEvent,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TimelineEventAddedParams {
                    event: fields.event,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：PerformanceTimeline.event.timelineEventAdded。
///
///协议原文：
///
///<pre>Sent when a performance timeline event is added. See reportPerformanceTimeline method.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct TimelineEventAdded;
impl ydp_core::Event for TimelineEventAdded {
    type Params = TimelineEventAddedParams;
    const METHOD: &'static str = "PerformanceTimeline.timelineEventAdded";
}
