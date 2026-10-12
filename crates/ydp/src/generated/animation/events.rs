///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.event.animationCanceled/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AnimationCanceledParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.event.animationCanceled/parameters/id。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the animation that was cancelled.</pre>
    ///
    ///
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: String,
}
impl<'de> serde::Deserialize<'de> for AnimationCanceledParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AnimationCanceledParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.event.animationCanceled/parameters/id。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the animation that was cancelled.</pre>
                    ///
                    ///
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AnimationCanceledParams { id: fields.id })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Animation.event.animationCanceled。
///
///协议原文：
///
///<pre>Event for when an animation has been cancelled.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct AnimationCanceled;
impl ydp_core::Event for AnimationCanceled {
    type Params = AnimationCanceledParams;
    const METHOD: &'static str = "Animation.animationCanceled";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.event.animationCreated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AnimationCreatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.event.animationCreated/parameters/id。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the animation that was created.</pre>
    ///
    ///
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: String,
}
impl<'de> serde::Deserialize<'de> for AnimationCreatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AnimationCreatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.event.animationCreated/parameters/id。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the animation that was created.</pre>
                    ///
                    ///
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AnimationCreatedParams { id: fields.id })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Animation.event.animationCreated。
///
///协议原文：
///
///<pre>Event for each animation that has been created.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct AnimationCreated;
impl ydp_core::Event for AnimationCreated {
    type Params = AnimationCreatedParams;
    const METHOD: &'static str = "Animation.animationCreated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.event.animationStarted/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AnimationStartedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.event.animationStarted/parameters/animation。
    ///
    ///协议原文：
    ///
    ///<pre>Animation that was started.</pre>
    ///
    ///
    #[serde(rename = "animation")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub animation: crate::animation::types::Animation,
}
impl<'de> serde::Deserialize<'de> for AnimationStartedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AnimationStartedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.event.animationStarted/parameters/animation。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Animation that was started.</pre>
                    ///
                    ///
                    #[serde(rename = "animation")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub animation: crate::animation::types::Animation,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AnimationStartedParams {
                    animation: fields.animation,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Animation.event.animationStarted。
///
///协议原文：
///
///<pre>Event for animation that has been started.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct AnimationStarted;
impl ydp_core::Event for AnimationStarted {
    type Params = AnimationStartedParams;
    const METHOD: &'static str = "Animation.animationStarted";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.event.animationUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AnimationUpdatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.event.animationUpdated/parameters/animation。
    ///
    ///协议原文：
    ///
    ///<pre>Animation that was updated.</pre>
    ///
    ///
    #[serde(rename = "animation")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub animation: crate::animation::types::Animation,
}
impl<'de> serde::Deserialize<'de> for AnimationUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AnimationUpdatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.event.animationUpdated/parameters/animation。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Animation that was updated.</pre>
                    ///
                    ///
                    #[serde(rename = "animation")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub animation: crate::animation::types::Animation,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AnimationUpdatedParams {
                    animation: fields.animation,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Animation.event.animationUpdated。
///
///协议原文：
///
///<pre>Event for animation that has been updated.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct AnimationUpdated;
impl ydp_core::Event for AnimationUpdated {
    type Params = AnimationUpdatedParams;
    const METHOD: &'static str = "Animation.animationUpdated";
}
