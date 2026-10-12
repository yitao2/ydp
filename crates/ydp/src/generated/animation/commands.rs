///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.command.disable/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct DisableParams {}
impl<'de> serde::Deserialize<'de> for DisableParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DisableParams;
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
                Ok(DisableParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Animation.command.disable。
///
///协议原文：
///
///<pre>Disables animation domain notifications.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.command.disable/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DisableResponse {}
impl<'de> serde::Deserialize<'de> for DisableResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DisableResponse;
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
                Ok(DisableResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Disable {
    type Params = DisableParams;
    type Response = DisableResponse;
    const METHOD: &'static str = "Animation.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.command.enable/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct EnableParams {}
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
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EnableParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Animation.command.enable。
///
///协议原文：
///
///<pre>Enables animation domain notifications.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.command.enable/returns。
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
    const METHOD: &'static str = "Animation.enable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.command.getCurrentTime/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetCurrentTimeParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.command.getCurrentTime/parameters/id。
    ///
    ///协议原文：
    ///
    ///<pre>Id of animation.</pre>
    ///
    ///
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: String,
}
impl<'de> serde::Deserialize<'de> for GetCurrentTimeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetCurrentTimeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.command.getCurrentTime/parameters/id。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of animation.</pre>
                    ///
                    ///
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetCurrentTimeParams { id: fields.id })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Animation.command.getCurrentTime。
///
///协议原文：
///
///<pre>Returns the current time of the an animation.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetCurrentTime;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.command.getCurrentTime/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetCurrentTimeResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.command.getCurrentTime/returns/currentTime。
    ///
    ///协议原文：
    ///
    ///<pre>Current time of the page.</pre>
    ///
    ///
    #[serde(rename = "currentTime")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub current_time: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for GetCurrentTimeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetCurrentTimeResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.command.getCurrentTime/returns/currentTime。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Current time of the page.</pre>
                    ///
                    ///
                    #[serde(rename = "currentTime")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub current_time: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetCurrentTimeResponse {
                    current_time: fields.current_time,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetCurrentTime {
    type Params = GetCurrentTimeParams;
    type Response = GetCurrentTimeResponse;
    const METHOD: &'static str = "Animation.getCurrentTime";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.command.getPlaybackRate/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetPlaybackRateParams {}
impl<'de> serde::Deserialize<'de> for GetPlaybackRateParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetPlaybackRateParams;
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
                Ok(GetPlaybackRateParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Animation.command.getPlaybackRate。
///
///协议原文：
///
///<pre>Gets the playback rate of the document timeline.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetPlaybackRate;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.command.getPlaybackRate/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetPlaybackRateResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.command.getPlaybackRate/returns/playbackRate。
    ///
    ///协议原文：
    ///
    ///<pre>Playback rate for animations on page.</pre>
    ///
    ///
    #[serde(rename = "playbackRate")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub playback_rate: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for GetPlaybackRateResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetPlaybackRateResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.command.getPlaybackRate/returns/playbackRate。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Playback rate for animations on page.</pre>
                    ///
                    ///
                    #[serde(rename = "playbackRate")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub playback_rate: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetPlaybackRateResponse {
                    playback_rate: fields.playback_rate,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetPlaybackRate {
    type Params = GetPlaybackRateParams;
    type Response = GetPlaybackRateResponse;
    const METHOD: &'static str = "Animation.getPlaybackRate";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.command.releaseAnimations/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReleaseAnimationsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.command.releaseAnimations/parameters/animations。
    ///
    ///协议原文：
    ///
    ///<pre>List of animation ids to seek.</pre>
    ///
    ///
    #[serde(rename = "animations")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub animations: Vec<String>,
}
impl<'de> serde::Deserialize<'de> for ReleaseAnimationsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReleaseAnimationsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.command.releaseAnimations/parameters/animations。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>List of animation ids to seek.</pre>
                    ///
                    ///
                    #[serde(rename = "animations")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub animations: Vec<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReleaseAnimationsParams {
                    animations: fields.animations,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Animation.command.releaseAnimations。
///
///协议原文：
///
///<pre>Releases a set of animations to no longer be manipulated.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ReleaseAnimations;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.command.releaseAnimations/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReleaseAnimationsResponse {}
impl<'de> serde::Deserialize<'de> for ReleaseAnimationsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReleaseAnimationsResponse;
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
                Ok(ReleaseAnimationsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ReleaseAnimations {
    type Params = ReleaseAnimationsParams;
    type Response = ReleaseAnimationsResponse;
    const METHOD: &'static str = "Animation.releaseAnimations";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.command.resolveAnimation/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ResolveAnimationParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.command.resolveAnimation/parameters/animationId。
    ///
    ///协议原文：
    ///
    ///<pre>Animation id.</pre>
    ///
    ///
    #[serde(rename = "animationId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub animation_id: String,
}
impl<'de> serde::Deserialize<'de> for ResolveAnimationParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResolveAnimationParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.command.resolveAnimation/parameters/animationId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Animation id.</pre>
                    ///
                    ///
                    #[serde(rename = "animationId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub animation_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ResolveAnimationParams {
                    animation_id: fields.animation_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Animation.command.resolveAnimation。
///
///协议原文：
///
///<pre>Gets the remote object of the Animation.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ResolveAnimation;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.command.resolveAnimation/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ResolveAnimationResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.command.resolveAnimation/returns/remoteObject。
    ///
    ///协议原文：
    ///
    ///<pre>Corresponding remote object.</pre>
    ///
    ///
    #[serde(rename = "remoteObject")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub remote_object: crate::runtime::types::RemoteObject,
}
impl<'de> serde::Deserialize<'de> for ResolveAnimationResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResolveAnimationResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.command.resolveAnimation/returns/remoteObject。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Corresponding remote object.</pre>
                    ///
                    ///
                    #[serde(rename = "remoteObject")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub remote_object: crate::runtime::types::RemoteObject,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ResolveAnimationResponse {
                    remote_object: fields.remote_object,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ResolveAnimation {
    type Params = ResolveAnimationParams;
    type Response = ResolveAnimationResponse;
    const METHOD: &'static str = "Animation.resolveAnimation";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.command.seekAnimations/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SeekAnimationsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.command.seekAnimations/parameters/animations。
    ///
    ///协议原文：
    ///
    ///<pre>List of animation ids to seek.</pre>
    ///
    ///
    #[serde(rename = "animations")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub animations: Vec<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.command.seekAnimations/parameters/currentTime。
    ///
    ///协议原文：
    ///
    ///<pre>Set the current time of each animation.</pre>
    ///
    ///
    #[serde(rename = "currentTime")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub current_time: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for SeekAnimationsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SeekAnimationsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.command.seekAnimations/parameters/animations。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>List of animation ids to seek.</pre>
                    ///
                    ///
                    #[serde(rename = "animations")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub animations: Vec<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.command.seekAnimations/parameters/currentTime。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Set the current time of each animation.</pre>
                    ///
                    ///
                    #[serde(rename = "currentTime")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub current_time: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SeekAnimationsParams {
                    animations: fields.animations,
                    current_time: fields.current_time,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Animation.command.seekAnimations。
///
///协议原文：
///
///<pre>Seek a set of animations to a particular time within each animation.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SeekAnimations;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.command.seekAnimations/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SeekAnimationsResponse {}
impl<'de> serde::Deserialize<'de> for SeekAnimationsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SeekAnimationsResponse;
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
                Ok(SeekAnimationsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SeekAnimations {
    type Params = SeekAnimationsParams;
    type Response = SeekAnimationsResponse;
    const METHOD: &'static str = "Animation.seekAnimations";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.command.setPaused/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetPausedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.command.setPaused/parameters/animations。
    ///
    ///协议原文：
    ///
    ///<pre>Animations to set the pause state of.</pre>
    ///
    ///
    #[serde(rename = "animations")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub animations: Vec<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.command.setPaused/parameters/paused。
    ///
    ///协议原文：
    ///
    ///<pre>Paused state to set to.</pre>
    ///
    ///
    #[serde(rename = "paused")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub paused: bool,
}
impl<'de> serde::Deserialize<'de> for SetPausedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetPausedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.command.setPaused/parameters/animations。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Animations to set the pause state of.</pre>
                    ///
                    ///
                    #[serde(rename = "animations")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub animations: Vec<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.command.setPaused/parameters/paused。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Paused state to set to.</pre>
                    ///
                    ///
                    #[serde(rename = "paused")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub paused: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetPausedParams {
                    animations: fields.animations,
                    paused: fields.paused,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Animation.command.setPaused。
///
///协议原文：
///
///<pre>Sets the paused state of a set of animations.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetPaused;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.command.setPaused/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetPausedResponse {}
impl<'de> serde::Deserialize<'de> for SetPausedResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetPausedResponse;
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
                Ok(SetPausedResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetPaused {
    type Params = SetPausedParams;
    type Response = SetPausedResponse;
    const METHOD: &'static str = "Animation.setPaused";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.command.setPlaybackRate/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetPlaybackRateParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.command.setPlaybackRate/parameters/playbackRate。
    ///
    ///协议原文：
    ///
    ///<pre>Playback rate for animations on page</pre>
    ///
    ///
    #[serde(rename = "playbackRate")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub playback_rate: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for SetPlaybackRateParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetPlaybackRateParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.command.setPlaybackRate/parameters/playbackRate。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Playback rate for animations on page</pre>
                    ///
                    ///
                    #[serde(rename = "playbackRate")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub playback_rate: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetPlaybackRateParams {
                    playback_rate: fields.playback_rate,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Animation.command.setPlaybackRate。
///
///协议原文：
///
///<pre>Sets the playback rate of the document timeline.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetPlaybackRate;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.command.setPlaybackRate/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetPlaybackRateResponse {}
impl<'de> serde::Deserialize<'de> for SetPlaybackRateResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetPlaybackRateResponse;
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
                Ok(SetPlaybackRateResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetPlaybackRate {
    type Params = SetPlaybackRateParams;
    type Response = SetPlaybackRateResponse;
    const METHOD: &'static str = "Animation.setPlaybackRate";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.command.setTiming/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetTimingParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.command.setTiming/parameters/animationId。
    ///
    ///协议原文：
    ///
    ///<pre>Animation id.</pre>
    ///
    ///
    #[serde(rename = "animationId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub animation_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.command.setTiming/parameters/duration。
    ///
    ///协议原文：
    ///
    ///<pre>Duration of the animation.</pre>
    ///
    ///
    #[serde(rename = "duration")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub duration: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.command.setTiming/parameters/delay。
    ///
    ///协议原文：
    ///
    ///<pre>Delay of the animation.</pre>
    ///
    ///
    #[serde(rename = "delay")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub delay: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for SetTimingParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetTimingParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.command.setTiming/parameters/animationId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Animation id.</pre>
                    ///
                    ///
                    #[serde(rename = "animationId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub animation_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.command.setTiming/parameters/duration。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Duration of the animation.</pre>
                    ///
                    ///
                    #[serde(rename = "duration")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub duration: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Animation.command.setTiming/parameters/delay。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Delay of the animation.</pre>
                    ///
                    ///
                    #[serde(rename = "delay")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub delay: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetTimingParams {
                    animation_id: fields.animation_id,
                    duration: fields.duration,
                    delay: fields.delay,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Animation.command.setTiming。
///
///协议原文：
///
///<pre>Sets the timing of an animation node.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetTiming;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Animation.command.setTiming/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetTimingResponse {}
impl<'de> serde::Deserialize<'de> for SetTimingResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetTimingResponse;
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
                Ok(SetTimingResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetTiming {
    type Params = SetTimingParams;
    type Response = SetTimingResponse;
    const METHOD: &'static str = "Animation.setTiming";
}
