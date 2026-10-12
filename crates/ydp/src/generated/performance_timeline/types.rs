///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：PerformanceTimeline.type.LargestContentfulPaint。
///
///协议原文：
///
///<pre>See https://github.com/WICG/LargestContentfulPaint and largest_contentful_paint.idl</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct LargestContentfulPaint {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.type.LargestContentfulPaint/properties/renderTime。
    #[serde(rename = "renderTime")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub render_time: crate::network::types::TimeSinceEpoch,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.type.LargestContentfulPaint/properties/loadTime。
    #[serde(rename = "loadTime")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub load_time: crate::network::types::TimeSinceEpoch,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.type.LargestContentfulPaint/properties/size。
    ///
    ///协议原文：
    ///
    ///<pre>The number of pixels being painted.</pre>
    ///
    ///
    #[serde(rename = "size")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub size: ydp_core::FiniteF64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：PerformanceTimeline.type.LargestContentfulPaint/properties/elementId。
    ///
    ///协议原文：
    ///
    ///<pre>The id attribute of the element, if available.</pre>
    ///
    ///
    #[serde(rename = "elementId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub element_id: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：PerformanceTimeline.type.LargestContentfulPaint/properties/url。
    ///
    ///协议原文：
    ///
    ///<pre>The URL of the image (may be trimmed).</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：PerformanceTimeline.type.LargestContentfulPaint/properties/nodeId。
    #[serde(rename = "nodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub node_id: Option<crate::dom::types::BackendNodeId>,
}
impl<'de> serde::Deserialize<'de> for LargestContentfulPaint {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LargestContentfulPaint;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.type.LargestContentfulPaint/properties/renderTime。
                    #[serde(rename = "renderTime")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub render_time: crate::network::types::TimeSinceEpoch,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.type.LargestContentfulPaint/properties/loadTime。
                    #[serde(rename = "loadTime")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub load_time: crate::network::types::TimeSinceEpoch,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.type.LargestContentfulPaint/properties/size。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The number of pixels being painted.</pre>
                    ///
                    ///
                    #[serde(rename = "size")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub size: ydp_core::FiniteF64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：PerformanceTimeline.type.LargestContentfulPaint/properties/elementId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The id attribute of the element, if available.</pre>
                    ///
                    ///
                    #[serde(rename = "elementId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub element_id: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：PerformanceTimeline.type.LargestContentfulPaint/properties/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The URL of the image (may be trimmed).</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub url: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：PerformanceTimeline.type.LargestContentfulPaint/properties/nodeId。
                    #[serde(rename = "nodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub node_id: Option<crate::dom::types::BackendNodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LargestContentfulPaint {
                    render_time: fields.render_time,
                    load_time: fields.load_time,
                    size: fields.size,
                    element_id: fields.element_id,
                    url: fields.url,
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：PerformanceTimeline.type.LayoutShift。
///
///协议原文：
///
///<pre>See https://wicg.github.io/layout-instability/#sec-layout-shift and layout_shift.idl</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct LayoutShift {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.type.LayoutShift/properties/value。
    ///
    ///协议原文：
    ///
    ///<pre>Score increment produced by this event.</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.type.LayoutShift/properties/hadRecentInput。
    #[serde(rename = "hadRecentInput")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub had_recent_input: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.type.LayoutShift/properties/lastInputTime。
    #[serde(rename = "lastInputTime")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub last_input_time: crate::network::types::TimeSinceEpoch,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.type.LayoutShift/properties/sources。
    #[serde(rename = "sources")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub sources: Vec<crate::performance_timeline::types::LayoutShiftAttribution>,
}
impl<'de> serde::Deserialize<'de> for LayoutShift {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LayoutShift;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.type.LayoutShift/properties/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Score increment produced by this event.</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.type.LayoutShift/properties/hadRecentInput。
                    #[serde(rename = "hadRecentInput")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub had_recent_input: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.type.LayoutShift/properties/lastInputTime。
                    #[serde(rename = "lastInputTime")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub last_input_time: crate::network::types::TimeSinceEpoch,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.type.LayoutShift/properties/sources。
                    #[serde(rename = "sources")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub sources: Vec<crate::performance_timeline::types::LayoutShiftAttribution>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LayoutShift {
                    value: fields.value,
                    had_recent_input: fields.had_recent_input,
                    last_input_time: fields.last_input_time,
                    sources: fields.sources,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：PerformanceTimeline.type.LayoutShiftAttribution。
#[derive(Debug, Clone, serde::Serialize)]
pub struct LayoutShiftAttribution {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.type.LayoutShiftAttribution/properties/previousRect。
    #[serde(rename = "previousRect")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub previous_rect: crate::dom::types::Rect,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.type.LayoutShiftAttribution/properties/currentRect。
    #[serde(rename = "currentRect")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub current_rect: crate::dom::types::Rect,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：PerformanceTimeline.type.LayoutShiftAttribution/properties/nodeId。
    #[serde(rename = "nodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub node_id: Option<crate::dom::types::BackendNodeId>,
}
impl<'de> serde::Deserialize<'de> for LayoutShiftAttribution {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LayoutShiftAttribution;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.type.LayoutShiftAttribution/properties/previousRect。
                    #[serde(rename = "previousRect")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub previous_rect: crate::dom::types::Rect,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.type.LayoutShiftAttribution/properties/currentRect。
                    #[serde(rename = "currentRect")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub current_rect: crate::dom::types::Rect,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：PerformanceTimeline.type.LayoutShiftAttribution/properties/nodeId。
                    #[serde(rename = "nodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub node_id: Option<crate::dom::types::BackendNodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LayoutShiftAttribution {
                    previous_rect: fields.previous_rect,
                    current_rect: fields.current_rect,
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：PerformanceTimeline.type.TimelineEvent。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TimelineEvent {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.type.TimelineEvent/properties/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifies the frame that this event is related to. Empty for non-frame targets.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.type.TimelineEvent/properties/type。
    ///
    ///协议原文：
    ///
    ///<pre>The event type, as specified in https://w3c.github.io/performance-timeline/#dom-performanceentry-entrytype
    ///This determines which of the optional "details" fields is present.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.type.TimelineEvent/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Name may be empty depending on the type.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.type.TimelineEvent/properties/time。
    ///
    ///协议原文：
    ///
    ///<pre>Time in seconds since Epoch, monotonically increasing within document lifetime.</pre>
    ///
    ///
    #[serde(rename = "time")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub time: crate::network::types::TimeSinceEpoch,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：PerformanceTimeline.type.TimelineEvent/properties/duration。
    ///
    ///协议原文：
    ///
    ///<pre>Event duration, if applicable.</pre>
    ///
    ///
    #[serde(rename = "duration")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub duration: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：PerformanceTimeline.type.TimelineEvent/properties/lcpDetails。
    #[serde(rename = "lcpDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub lcp_details: Option<crate::performance_timeline::types::LargestContentfulPaint>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：PerformanceTimeline.type.TimelineEvent/properties/layoutShiftDetails。
    #[serde(rename = "layoutShiftDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub layout_shift_details: Option<crate::performance_timeline::types::LayoutShift>,
}
impl<'de> serde::Deserialize<'de> for TimelineEvent {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TimelineEvent;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.type.TimelineEvent/properties/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifies the frame that this event is related to. Empty for non-frame targets.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.type.TimelineEvent/properties/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The event type, as specified in https://w3c.github.io/performance-timeline/#dom-performanceentry-entrytype
                    ///This determines which of the optional "details" fields is present.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.type.TimelineEvent/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Name may be empty depending on the type.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PerformanceTimeline.type.TimelineEvent/properties/time。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Time in seconds since Epoch, monotonically increasing within document lifetime.</pre>
                    ///
                    ///
                    #[serde(rename = "time")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub time: crate::network::types::TimeSinceEpoch,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：PerformanceTimeline.type.TimelineEvent/properties/duration。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Event duration, if applicable.</pre>
                    ///
                    ///
                    #[serde(rename = "duration")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub duration: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：PerformanceTimeline.type.TimelineEvent/properties/lcpDetails。
                    #[serde(rename = "lcpDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub lcp_details:
                        Option<crate::performance_timeline::types::LargestContentfulPaint>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：PerformanceTimeline.type.TimelineEvent/properties/layoutShiftDetails。
                    #[serde(rename = "layoutShiftDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub layout_shift_details:
                        Option<crate::performance_timeline::types::LayoutShift>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TimelineEvent {
                    frame_id: fields.frame_id,
                    r#type: fields.r#type,
                    name: fields.name,
                    time: fields.time,
                    duration: fields.duration,
                    lcp_details: fields.lcp_details,
                    layout_shift_details: fields.layout_shift_details,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
