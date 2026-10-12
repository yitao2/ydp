///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Animation.type.Animation/properties/type。
///
///协议原文：
///
///<pre>Animation type of `Animation`.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AnimationType {
    ///协议值 "CSSTransition"。
    CssTransition,
    ///协议值 "CSSAnimation"。
    CssAnimation,
    ///协议值 "WebAnimation"。
    WebAnimation,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl AnimationType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::animation::types::AnimationType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::CssTransition => "CSSTransition",
            Self::CssAnimation => "CSSAnimation",
            Self::WebAnimation => "WebAnimation",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for AnimationType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for AnimationType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "CSSTransition" => AnimationType::CssTransition,
            "CSSAnimation" => AnimationType::CssAnimation,
            "WebAnimation" => AnimationType::WebAnimation,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for AnimationType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AnimationType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AnimationType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "CSSTransition" => AnimationType::CssTransition,
                    "CSSAnimation" => AnimationType::CssAnimation,
                    "WebAnimation" => AnimationType::WebAnimation,
                    _ => AnimationType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(AnimationType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.type.Animation。
///
///协议原文：
///
///<pre>Animation instance.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct Animation {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.Animation/properties/id。
    ///
    ///协议原文：
    ///
    ///<pre>`Animation`'s id.</pre>
    ///
    ///
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.Animation/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>`Animation`'s name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.Animation/properties/pausedState。
    ///
    ///协议原文：
    ///
    ///<pre>`Animation`'s internal paused state.</pre>
    ///
    ///
    #[serde(rename = "pausedState")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub paused_state: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.Animation/properties/playState。
    ///
    ///协议原文：
    ///
    ///<pre>`Animation`'s play state.</pre>
    ///
    ///
    #[serde(rename = "playState")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub play_state: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.Animation/properties/playbackRate。
    ///
    ///协议原文：
    ///
    ///<pre>`Animation`'s playback rate.</pre>
    ///
    ///
    #[serde(rename = "playbackRate")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub playback_rate: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.Animation/properties/startTime。
    ///
    ///协议原文：
    ///
    ///<pre>`Animation`'s start time.
    ///Milliseconds for time based animations and
    ///percentage [0 - 100] for scroll driven animations
    ///(i.e. when viewOrScrollTimeline exists).</pre>
    ///
    ///
    #[serde(rename = "startTime")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub start_time: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.Animation/properties/currentTime。
    ///
    ///协议原文：
    ///
    ///<pre>`Animation`'s current time.</pre>
    ///
    ///
    #[serde(rename = "currentTime")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub current_time: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.Animation/properties/type。
    ///
    ///协议原文：
    ///
    ///<pre>Animation type of `Animation`.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: AnimationType,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Animation.type.Animation/properties/source。
    ///
    ///协议原文：
    ///
    ///<pre>`Animation`'s source animation node.</pre>
    ///
    ///
    #[serde(rename = "source")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub source: Option<crate::animation::types::AnimationEffect>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Animation.type.Animation/properties/cssId。
    ///
    ///协议原文：
    ///
    ///<pre>A unique ID for `Animation` representing the sources that triggered this CSS
    ///animation/transition.</pre>
    ///
    ///
    #[serde(rename = "cssId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub css_id: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Animation.type.Animation/properties/viewOrScrollTimeline。
    ///
    ///协议原文：
    ///
    ///<pre>View or scroll timeline</pre>
    ///
    ///
    #[serde(rename = "viewOrScrollTimeline")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub view_or_scroll_timeline: Option<crate::animation::types::ViewOrScrollTimeline>,
}
impl<'de> serde::Deserialize<'de> for Animation {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Animation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.Animation/properties/id。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`Animation`'s id.</pre>
                    ///
                    ///
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.Animation/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`Animation`'s name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.Animation/properties/pausedState。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`Animation`'s internal paused state.</pre>
                    ///
                    ///
                    #[serde(rename = "pausedState")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub paused_state: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.Animation/properties/playState。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`Animation`'s play state.</pre>
                    ///
                    ///
                    #[serde(rename = "playState")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub play_state: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.Animation/properties/playbackRate。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`Animation`'s playback rate.</pre>
                    ///
                    ///
                    #[serde(rename = "playbackRate")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub playback_rate: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.Animation/properties/startTime。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`Animation`'s start time.
                    ///Milliseconds for time based animations and
                    ///percentage [0 - 100] for scroll driven animations
                    ///(i.e. when viewOrScrollTimeline exists).</pre>
                    ///
                    ///
                    #[serde(rename = "startTime")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub start_time: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.Animation/properties/currentTime。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`Animation`'s current time.</pre>
                    ///
                    ///
                    #[serde(rename = "currentTime")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub current_time: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.Animation/properties/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Animation type of `Animation`.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: AnimationType,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Animation.type.Animation/properties/source。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`Animation`'s source animation node.</pre>
                    ///
                    ///
                    #[serde(rename = "source")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub source: Option<crate::animation::types::AnimationEffect>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Animation.type.Animation/properties/cssId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A unique ID for `Animation` representing the sources that triggered this CSS
                    ///animation/transition.</pre>
                    ///
                    ///
                    #[serde(rename = "cssId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub css_id: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Animation.type.Animation/properties/viewOrScrollTimeline。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>View or scroll timeline</pre>
                    ///
                    ///
                    #[serde(rename = "viewOrScrollTimeline")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub view_or_scroll_timeline:
                        Option<crate::animation::types::ViewOrScrollTimeline>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Animation {
                    id: fields.id,
                    name: fields.name,
                    paused_state: fields.paused_state,
                    play_state: fields.play_state,
                    playback_rate: fields.playback_rate,
                    start_time: fields.start_time,
                    current_time: fields.current_time,
                    r#type: fields.r#type,
                    source: fields.source,
                    css_id: fields.css_id,
                    view_or_scroll_timeline: fields.view_or_scroll_timeline,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.type.AnimationEffect。
///
///协议原文：
///
///<pre>AnimationEffect instance</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct AnimationEffect {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.AnimationEffect/properties/delay。
    ///
    ///协议原文：
    ///
    ///<pre>`AnimationEffect`'s delay.</pre>
    ///
    ///
    #[serde(rename = "delay")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub delay: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.AnimationEffect/properties/endDelay。
    ///
    ///协议原文：
    ///
    ///<pre>`AnimationEffect`'s end delay.</pre>
    ///
    ///
    #[serde(rename = "endDelay")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub end_delay: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.AnimationEffect/properties/iterationStart。
    ///
    ///协议原文：
    ///
    ///<pre>`AnimationEffect`'s iteration start.</pre>
    ///
    ///
    #[serde(rename = "iterationStart")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub iteration_start: ydp_core::FiniteF64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Animation.type.AnimationEffect/properties/iterations。
    ///
    ///协议原文：
    ///
    ///<pre>`AnimationEffect`'s iterations. Omitted if the value is infinite.</pre>
    ///
    ///
    #[serde(rename = "iterations")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub iterations: Option<ydp_core::FiniteF64>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.AnimationEffect/properties/duration。
    ///
    ///协议原文：
    ///
    ///<pre>`AnimationEffect`'s iteration duration.
    ///Milliseconds for time based animations and
    ///percentage [0 - 100] for scroll driven animations
    ///(i.e. when viewOrScrollTimeline exists).</pre>
    ///
    ///
    #[serde(rename = "duration")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub duration: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.AnimationEffect/properties/direction。
    ///
    ///协议原文：
    ///
    ///<pre>`AnimationEffect`'s playback direction.</pre>
    ///
    ///
    #[serde(rename = "direction")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub direction: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.AnimationEffect/properties/fill。
    ///
    ///协议原文：
    ///
    ///<pre>`AnimationEffect`'s fill mode.</pre>
    ///
    ///
    #[serde(rename = "fill")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub fill: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Animation.type.AnimationEffect/properties/backendNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>`AnimationEffect`'s target node.</pre>
    ///
    ///
    #[serde(rename = "backendNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Animation.type.AnimationEffect/properties/keyframesRule。
    ///
    ///协议原文：
    ///
    ///<pre>`AnimationEffect`'s keyframes.</pre>
    ///
    ///
    #[serde(rename = "keyframesRule")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub keyframes_rule: Option<crate::animation::types::KeyframesRule>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.AnimationEffect/properties/easing。
    ///
    ///协议原文：
    ///
    ///<pre>`AnimationEffect`'s timing function.</pre>
    ///
    ///
    #[serde(rename = "easing")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub easing: String,
}
impl<'de> serde::Deserialize<'de> for AnimationEffect {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AnimationEffect;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.AnimationEffect/properties/delay。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`AnimationEffect`'s delay.</pre>
                    ///
                    ///
                    #[serde(rename = "delay")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub delay: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.AnimationEffect/properties/endDelay。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`AnimationEffect`'s end delay.</pre>
                    ///
                    ///
                    #[serde(rename = "endDelay")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub end_delay: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.AnimationEffect/properties/iterationStart。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`AnimationEffect`'s iteration start.</pre>
                    ///
                    ///
                    #[serde(rename = "iterationStart")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub iteration_start: ydp_core::FiniteF64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Animation.type.AnimationEffect/properties/iterations。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`AnimationEffect`'s iterations. Omitted if the value is infinite.</pre>
                    ///
                    ///
                    #[serde(rename = "iterations")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub iterations: Option<ydp_core::FiniteF64>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.AnimationEffect/properties/duration。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`AnimationEffect`'s iteration duration.
                    ///Milliseconds for time based animations and
                    ///percentage [0 - 100] for scroll driven animations
                    ///(i.e. when viewOrScrollTimeline exists).</pre>
                    ///
                    ///
                    #[serde(rename = "duration")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub duration: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.AnimationEffect/properties/direction。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`AnimationEffect`'s playback direction.</pre>
                    ///
                    ///
                    #[serde(rename = "direction")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub direction: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.AnimationEffect/properties/fill。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`AnimationEffect`'s fill mode.</pre>
                    ///
                    ///
                    #[serde(rename = "fill")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub fill: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Animation.type.AnimationEffect/properties/backendNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`AnimationEffect`'s target node.</pre>
                    ///
                    ///
                    #[serde(rename = "backendNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Animation.type.AnimationEffect/properties/keyframesRule。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`AnimationEffect`'s keyframes.</pre>
                    ///
                    ///
                    #[serde(rename = "keyframesRule")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub keyframes_rule: Option<crate::animation::types::KeyframesRule>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.AnimationEffect/properties/easing。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`AnimationEffect`'s timing function.</pre>
                    ///
                    ///
                    #[serde(rename = "easing")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub easing: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AnimationEffect {
                    delay: fields.delay,
                    end_delay: fields.end_delay,
                    iteration_start: fields.iteration_start,
                    iterations: fields.iterations,
                    duration: fields.duration,
                    direction: fields.direction,
                    fill: fields.fill,
                    backend_node_id: fields.backend_node_id,
                    keyframes_rule: fields.keyframes_rule,
                    easing: fields.easing,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.type.KeyframeStyle。
///
///协议原文：
///
///<pre>Keyframe Style</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct KeyframeStyle {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.KeyframeStyle/properties/offset。
    ///
    ///协议原文：
    ///
    ///<pre>Keyframe's time offset.</pre>
    ///
    ///
    #[serde(rename = "offset")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub offset: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.KeyframeStyle/properties/easing。
    ///
    ///协议原文：
    ///
    ///<pre>`AnimationEffect`'s timing function.</pre>
    ///
    ///
    #[serde(rename = "easing")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub easing: String,
}
impl<'de> serde::Deserialize<'de> for KeyframeStyle {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = KeyframeStyle;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.KeyframeStyle/properties/offset。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Keyframe's time offset.</pre>
                    ///
                    ///
                    #[serde(rename = "offset")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub offset: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.KeyframeStyle/properties/easing。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`AnimationEffect`'s timing function.</pre>
                    ///
                    ///
                    #[serde(rename = "easing")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub easing: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(KeyframeStyle {
                    offset: fields.offset,
                    easing: fields.easing,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.type.KeyframesRule。
///
///协议原文：
///
///<pre>Keyframes Rule</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct KeyframesRule {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Animation.type.KeyframesRule/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>CSS keyframed animation's name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.KeyframesRule/properties/keyframes。
    ///
    ///协议原文：
    ///
    ///<pre>List of animation keyframes.</pre>
    ///
    ///
    #[serde(rename = "keyframes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub keyframes: Vec<crate::animation::types::KeyframeStyle>,
}
impl<'de> serde::Deserialize<'de> for KeyframesRule {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = KeyframesRule;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Animation.type.KeyframesRule/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>CSS keyframed animation's name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub name: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.KeyframesRule/properties/keyframes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>List of animation keyframes.</pre>
                    ///
                    ///
                    #[serde(rename = "keyframes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub keyframes: Vec<crate::animation::types::KeyframeStyle>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(KeyframesRule {
                    name: fields.name,
                    keyframes: fields.keyframes,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.type.ViewOrScrollTimeline。
///
///协议原文：
///
///<pre>Timeline instance</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ViewOrScrollTimeline {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Animation.type.ViewOrScrollTimeline/properties/sourceNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Scroll container node</pre>
    ///
    ///
    #[serde(rename = "sourceNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_node_id: Option<crate::dom::types::BackendNodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Animation.type.ViewOrScrollTimeline/properties/startOffset。
    ///
    ///协议原文：
    ///
    ///<pre>Represents the starting scroll position of the timeline
    ///as a length offset in pixels from scroll origin.</pre>
    ///
    ///
    #[serde(rename = "startOffset")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub start_offset: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Animation.type.ViewOrScrollTimeline/properties/endOffset。
    ///
    ///协议原文：
    ///
    ///<pre>Represents the ending scroll position of the timeline
    ///as a length offset in pixels from scroll origin.</pre>
    ///
    ///
    #[serde(rename = "endOffset")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub end_offset: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Animation.type.ViewOrScrollTimeline/properties/subjectNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>The element whose principal box's visibility in the
    ///scrollport defined the progress of the timeline.
    ///Does not exist for animations with ScrollTimeline</pre>
    ///
    ///
    #[serde(rename = "subjectNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub subject_node_id: Option<crate::dom::types::BackendNodeId>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.ViewOrScrollTimeline/properties/axis。
    ///
    ///协议原文：
    ///
    ///<pre>Orientation of the scroll</pre>
    ///
    ///
    #[serde(rename = "axis")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub axis: crate::dom::types::ScrollOrientation,
}
impl<'de> serde::Deserialize<'de> for ViewOrScrollTimeline {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ViewOrScrollTimeline;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Animation.type.ViewOrScrollTimeline/properties/sourceNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Scroll container node</pre>
                    ///
                    ///
                    #[serde(rename = "sourceNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub source_node_id: Option<crate::dom::types::BackendNodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Animation.type.ViewOrScrollTimeline/properties/startOffset。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Represents the starting scroll position of the timeline
                    ///as a length offset in pixels from scroll origin.</pre>
                    ///
                    ///
                    #[serde(rename = "startOffset")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub start_offset: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Animation.type.ViewOrScrollTimeline/properties/endOffset。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Represents the ending scroll position of the timeline
                    ///as a length offset in pixels from scroll origin.</pre>
                    ///
                    ///
                    #[serde(rename = "endOffset")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub end_offset: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Animation.type.ViewOrScrollTimeline/properties/subjectNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The element whose principal box's visibility in the
                    ///scrollport defined the progress of the timeline.
                    ///Does not exist for animations with ScrollTimeline</pre>
                    ///
                    ///
                    #[serde(rename = "subjectNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub subject_node_id: Option<crate::dom::types::BackendNodeId>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.type.ViewOrScrollTimeline/properties/axis。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Orientation of the scroll</pre>
                    ///
                    ///
                    #[serde(rename = "axis")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub axis: crate::dom::types::ScrollOrientation,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ViewOrScrollTimeline {
                    source_node_id: fields.source_node_id,
                    start_offset: fields.start_offset,
                    end_offset: fields.end_offset,
                    subject_node_id: fields.subject_node_id,
                    axis: fields.axis,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
