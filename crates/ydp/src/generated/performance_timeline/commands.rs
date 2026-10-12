///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：PerformanceTimeline.command.enable/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EnableParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.command.enable/parameters/eventTypes。
    ///
    ///协议原文：
    ///
    ///<pre>The types of event to report, as specified in
    ///https://w3c.github.io/performance-timeline/#dom-performanceentry-entrytype
    ///The specified filter overrides any previous filters, passing empty
    ///filter disables recording.
    ///Note that not all types exposed to the web platform are currently supported.</pre>
    ///
    ///
    #[serde(rename = "eventTypes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub event_types: Vec<String>,
}
impl<'de> serde::Deserialize<'de> for EnableParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EnableParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.command.enable/parameters/eventTypes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The types of event to report, as specified in
                    ///https://w3c.github.io/performance-timeline/#dom-performanceentry-entrytype
                    ///The specified filter overrides any previous filters, passing empty
                    ///filter disables recording.
                    ///Note that not all types exposed to the web platform are currently supported.</pre>
                    ///
                    ///
                    #[serde(rename = "eventTypes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub event_types: Vec<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EnableParams {
                    event_types: fields.event_types,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：PerformanceTimeline.command.enable。
///
///协议原文：
///
///<pre>Previously buffered events would be reported before method returns.
///See also: timelineEventAdded</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：PerformanceTimeline.command.enable/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EnableResponse {}
impl<'de> serde::Deserialize<'de> for EnableResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EnableResponse;
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
                Ok(EnableResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Enable {
    type Params = EnableParams;
    type Response = EnableResponse;
    const METHOD: &'static str = "PerformanceTimeline.enable";
}
