///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAudio.event.audioListenerCreated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AudioListenerCreatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.audioListenerCreated/parameters/listener。
    #[serde(rename = "listener")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub listener: crate::web_audio::types::AudioListener,
}
impl<'de> serde::Deserialize<'de> for AudioListenerCreatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AudioListenerCreatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.audioListenerCreated/parameters/listener。
                    #[serde(rename = "listener")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub listener: crate::web_audio::types::AudioListener,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AudioListenerCreatedParams {
                    listener: fields.listener,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAudio.event.audioListenerCreated。
///
///协议原文：
///
///<pre>Notifies that the construction of an AudioListener has finished.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct AudioListenerCreated;
impl ydp_core::Event for AudioListenerCreated {
    type Params = AudioListenerCreatedParams;
    const METHOD: &'static str = "WebAudio.audioListenerCreated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAudio.event.audioListenerWillBeDestroyed/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AudioListenerWillBeDestroyedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.audioListenerWillBeDestroyed/parameters/contextId。
    #[serde(rename = "contextId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub context_id: crate::web_audio::types::GraphObjectId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.audioListenerWillBeDestroyed/parameters/listenerId。
    #[serde(rename = "listenerId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub listener_id: crate::web_audio::types::GraphObjectId,
}
impl<'de> serde::Deserialize<'de> for AudioListenerWillBeDestroyedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AudioListenerWillBeDestroyedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.audioListenerWillBeDestroyed/parameters/contextId。
                    #[serde(rename = "contextId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub context_id: crate::web_audio::types::GraphObjectId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.audioListenerWillBeDestroyed/parameters/listenerId。
                    #[serde(rename = "listenerId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub listener_id: crate::web_audio::types::GraphObjectId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AudioListenerWillBeDestroyedParams {
                    context_id: fields.context_id,
                    listener_id: fields.listener_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAudio.event.audioListenerWillBeDestroyed。
///
///协议原文：
///
///<pre>Notifies that a new AudioListener has been created.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct AudioListenerWillBeDestroyed;
impl ydp_core::Event for AudioListenerWillBeDestroyed {
    type Params = AudioListenerWillBeDestroyedParams;
    const METHOD: &'static str = "WebAudio.audioListenerWillBeDestroyed";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAudio.event.audioNodeCreated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AudioNodeCreatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.audioNodeCreated/parameters/node。
    #[serde(rename = "node")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node: crate::web_audio::types::AudioNode,
}
impl<'de> serde::Deserialize<'de> for AudioNodeCreatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AudioNodeCreatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.audioNodeCreated/parameters/node。
                    #[serde(rename = "node")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node: crate::web_audio::types::AudioNode,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AudioNodeCreatedParams { node: fields.node })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAudio.event.audioNodeCreated。
///
///协议原文：
///
///<pre>Notifies that a new AudioNode has been created.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct AudioNodeCreated;
impl ydp_core::Event for AudioNodeCreated {
    type Params = AudioNodeCreatedParams;
    const METHOD: &'static str = "WebAudio.audioNodeCreated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAudio.event.audioNodeWillBeDestroyed/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AudioNodeWillBeDestroyedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.audioNodeWillBeDestroyed/parameters/contextId。
    #[serde(rename = "contextId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub context_id: crate::web_audio::types::GraphObjectId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.audioNodeWillBeDestroyed/parameters/nodeId。
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::web_audio::types::GraphObjectId,
}
impl<'de> serde::Deserialize<'de> for AudioNodeWillBeDestroyedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AudioNodeWillBeDestroyedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.audioNodeWillBeDestroyed/parameters/contextId。
                    #[serde(rename = "contextId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub context_id: crate::web_audio::types::GraphObjectId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.audioNodeWillBeDestroyed/parameters/nodeId。
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::web_audio::types::GraphObjectId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AudioNodeWillBeDestroyedParams {
                    context_id: fields.context_id,
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAudio.event.audioNodeWillBeDestroyed。
///
///协议原文：
///
///<pre>Notifies that an existing AudioNode has been destroyed.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct AudioNodeWillBeDestroyed;
impl ydp_core::Event for AudioNodeWillBeDestroyed {
    type Params = AudioNodeWillBeDestroyedParams;
    const METHOD: &'static str = "WebAudio.audioNodeWillBeDestroyed";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAudio.event.audioParamCreated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AudioParamCreatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.audioParamCreated/parameters/param。
    #[serde(rename = "param")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub param: crate::web_audio::types::AudioParam,
}
impl<'de> serde::Deserialize<'de> for AudioParamCreatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AudioParamCreatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.audioParamCreated/parameters/param。
                    #[serde(rename = "param")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub param: crate::web_audio::types::AudioParam,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AudioParamCreatedParams {
                    param: fields.param,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAudio.event.audioParamCreated。
///
///协议原文：
///
///<pre>Notifies that a new AudioParam has been created.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct AudioParamCreated;
impl ydp_core::Event for AudioParamCreated {
    type Params = AudioParamCreatedParams;
    const METHOD: &'static str = "WebAudio.audioParamCreated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAudio.event.audioParamWillBeDestroyed/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AudioParamWillBeDestroyedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.audioParamWillBeDestroyed/parameters/contextId。
    #[serde(rename = "contextId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub context_id: crate::web_audio::types::GraphObjectId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.audioParamWillBeDestroyed/parameters/nodeId。
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::web_audio::types::GraphObjectId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.audioParamWillBeDestroyed/parameters/paramId。
    #[serde(rename = "paramId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub param_id: crate::web_audio::types::GraphObjectId,
}
impl<'de> serde::Deserialize<'de> for AudioParamWillBeDestroyedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AudioParamWillBeDestroyedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.audioParamWillBeDestroyed/parameters/contextId。
                    #[serde(rename = "contextId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub context_id: crate::web_audio::types::GraphObjectId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.audioParamWillBeDestroyed/parameters/nodeId。
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::web_audio::types::GraphObjectId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.audioParamWillBeDestroyed/parameters/paramId。
                    #[serde(rename = "paramId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub param_id: crate::web_audio::types::GraphObjectId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AudioParamWillBeDestroyedParams {
                    context_id: fields.context_id,
                    node_id: fields.node_id,
                    param_id: fields.param_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAudio.event.audioParamWillBeDestroyed。
///
///协议原文：
///
///<pre>Notifies that an existing AudioParam has been destroyed.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct AudioParamWillBeDestroyed;
impl ydp_core::Event for AudioParamWillBeDestroyed {
    type Params = AudioParamWillBeDestroyedParams;
    const METHOD: &'static str = "WebAudio.audioParamWillBeDestroyed";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAudio.event.contextChanged/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ContextChangedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.contextChanged/parameters/context。
    #[serde(rename = "context")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub context: crate::web_audio::types::BaseAudioContext,
}
impl<'de> serde::Deserialize<'de> for ContextChangedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContextChangedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.contextChanged/parameters/context。
                    #[serde(rename = "context")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub context: crate::web_audio::types::BaseAudioContext,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ContextChangedParams {
                    context: fields.context,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAudio.event.contextChanged。
///
///协议原文：
///
///<pre>Notifies that existing BaseAudioContext has changed some properties (id stays the same)..</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ContextChanged;
impl ydp_core::Event for ContextChanged {
    type Params = ContextChangedParams;
    const METHOD: &'static str = "WebAudio.contextChanged";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAudio.event.contextCreated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ContextCreatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.contextCreated/parameters/context。
    #[serde(rename = "context")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub context: crate::web_audio::types::BaseAudioContext,
}
impl<'de> serde::Deserialize<'de> for ContextCreatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContextCreatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.contextCreated/parameters/context。
                    #[serde(rename = "context")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub context: crate::web_audio::types::BaseAudioContext,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ContextCreatedParams {
                    context: fields.context,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAudio.event.contextCreated。
///
///协议原文：
///
///<pre>Notifies that a new BaseAudioContext has been created.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ContextCreated;
impl ydp_core::Event for ContextCreated {
    type Params = ContextCreatedParams;
    const METHOD: &'static str = "WebAudio.contextCreated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAudio.event.contextWillBeDestroyed/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ContextWillBeDestroyedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.contextWillBeDestroyed/parameters/contextId。
    #[serde(rename = "contextId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub context_id: crate::web_audio::types::GraphObjectId,
}
impl<'de> serde::Deserialize<'de> for ContextWillBeDestroyedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContextWillBeDestroyedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.contextWillBeDestroyed/parameters/contextId。
                    #[serde(rename = "contextId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub context_id: crate::web_audio::types::GraphObjectId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ContextWillBeDestroyedParams {
                    context_id: fields.context_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAudio.event.contextWillBeDestroyed。
///
///协议原文：
///
///<pre>Notifies that an existing BaseAudioContext will be destroyed.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ContextWillBeDestroyed;
impl ydp_core::Event for ContextWillBeDestroyed {
    type Params = ContextWillBeDestroyedParams;
    const METHOD: &'static str = "WebAudio.contextWillBeDestroyed";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAudio.event.nodeParamConnected/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeParamConnectedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.nodeParamConnected/parameters/contextId。
    #[serde(rename = "contextId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub context_id: crate::web_audio::types::GraphObjectId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.nodeParamConnected/parameters/sourceId。
    #[serde(rename = "sourceId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub source_id: crate::web_audio::types::GraphObjectId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.nodeParamConnected/parameters/destinationId。
    #[serde(rename = "destinationId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub destination_id: crate::web_audio::types::GraphObjectId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAudio.event.nodeParamConnected/parameters/sourceOutputIndex。
    #[serde(rename = "sourceOutputIndex")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_output_index: Option<ydp_core::FiniteF64>,
}
impl<'de> serde::Deserialize<'de> for NodeParamConnectedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NodeParamConnectedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.nodeParamConnected/parameters/contextId。
                    #[serde(rename = "contextId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub context_id: crate::web_audio::types::GraphObjectId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.nodeParamConnected/parameters/sourceId。
                    #[serde(rename = "sourceId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub source_id: crate::web_audio::types::GraphObjectId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.nodeParamConnected/parameters/destinationId。
                    #[serde(rename = "destinationId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub destination_id: crate::web_audio::types::GraphObjectId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAudio.event.nodeParamConnected/parameters/sourceOutputIndex。
                    #[serde(rename = "sourceOutputIndex")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub source_output_index: Option<ydp_core::FiniteF64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(NodeParamConnectedParams {
                    context_id: fields.context_id,
                    source_id: fields.source_id,
                    destination_id: fields.destination_id,
                    source_output_index: fields.source_output_index,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAudio.event.nodeParamConnected。
///
///协议原文：
///
///<pre>Notifies that an AudioNode is connected to an AudioParam.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct NodeParamConnected;
impl ydp_core::Event for NodeParamConnected {
    type Params = NodeParamConnectedParams;
    const METHOD: &'static str = "WebAudio.nodeParamConnected";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAudio.event.nodeParamDisconnected/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeParamDisconnectedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.nodeParamDisconnected/parameters/contextId。
    #[serde(rename = "contextId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub context_id: crate::web_audio::types::GraphObjectId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.nodeParamDisconnected/parameters/sourceId。
    #[serde(rename = "sourceId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub source_id: crate::web_audio::types::GraphObjectId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.nodeParamDisconnected/parameters/destinationId。
    #[serde(rename = "destinationId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub destination_id: crate::web_audio::types::GraphObjectId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAudio.event.nodeParamDisconnected/parameters/sourceOutputIndex。
    #[serde(rename = "sourceOutputIndex")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_output_index: Option<ydp_core::FiniteF64>,
}
impl<'de> serde::Deserialize<'de> for NodeParamDisconnectedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NodeParamDisconnectedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.nodeParamDisconnected/parameters/contextId。
                    #[serde(rename = "contextId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub context_id: crate::web_audio::types::GraphObjectId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.nodeParamDisconnected/parameters/sourceId。
                    #[serde(rename = "sourceId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub source_id: crate::web_audio::types::GraphObjectId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.nodeParamDisconnected/parameters/destinationId。
                    #[serde(rename = "destinationId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub destination_id: crate::web_audio::types::GraphObjectId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAudio.event.nodeParamDisconnected/parameters/sourceOutputIndex。
                    #[serde(rename = "sourceOutputIndex")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub source_output_index: Option<ydp_core::FiniteF64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(NodeParamDisconnectedParams {
                    context_id: fields.context_id,
                    source_id: fields.source_id,
                    destination_id: fields.destination_id,
                    source_output_index: fields.source_output_index,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAudio.event.nodeParamDisconnected。
///
///协议原文：
///
///<pre>Notifies that an AudioNode is disconnected to an AudioParam.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct NodeParamDisconnected;
impl ydp_core::Event for NodeParamDisconnected {
    type Params = NodeParamDisconnectedParams;
    const METHOD: &'static str = "WebAudio.nodeParamDisconnected";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAudio.event.nodesConnected/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodesConnectedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.nodesConnected/parameters/contextId。
    #[serde(rename = "contextId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub context_id: crate::web_audio::types::GraphObjectId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.nodesConnected/parameters/sourceId。
    #[serde(rename = "sourceId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub source_id: crate::web_audio::types::GraphObjectId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.nodesConnected/parameters/destinationId。
    #[serde(rename = "destinationId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub destination_id: crate::web_audio::types::GraphObjectId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAudio.event.nodesConnected/parameters/sourceOutputIndex。
    #[serde(rename = "sourceOutputIndex")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_output_index: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAudio.event.nodesConnected/parameters/destinationInputIndex。
    #[serde(rename = "destinationInputIndex")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_input_index: Option<ydp_core::FiniteF64>,
}
impl<'de> serde::Deserialize<'de> for NodesConnectedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NodesConnectedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.nodesConnected/parameters/contextId。
                    #[serde(rename = "contextId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub context_id: crate::web_audio::types::GraphObjectId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.nodesConnected/parameters/sourceId。
                    #[serde(rename = "sourceId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub source_id: crate::web_audio::types::GraphObjectId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.nodesConnected/parameters/destinationId。
                    #[serde(rename = "destinationId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub destination_id: crate::web_audio::types::GraphObjectId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAudio.event.nodesConnected/parameters/sourceOutputIndex。
                    #[serde(rename = "sourceOutputIndex")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub source_output_index: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAudio.event.nodesConnected/parameters/destinationInputIndex。
                    #[serde(rename = "destinationInputIndex")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub destination_input_index: Option<ydp_core::FiniteF64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(NodesConnectedParams {
                    context_id: fields.context_id,
                    source_id: fields.source_id,
                    destination_id: fields.destination_id,
                    source_output_index: fields.source_output_index,
                    destination_input_index: fields.destination_input_index,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAudio.event.nodesConnected。
///
///协议原文：
///
///<pre>Notifies that two AudioNodes are connected.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct NodesConnected;
impl ydp_core::Event for NodesConnected {
    type Params = NodesConnectedParams;
    const METHOD: &'static str = "WebAudio.nodesConnected";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAudio.event.nodesDisconnected/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodesDisconnectedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.nodesDisconnected/parameters/contextId。
    #[serde(rename = "contextId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub context_id: crate::web_audio::types::GraphObjectId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.nodesDisconnected/parameters/sourceId。
    #[serde(rename = "sourceId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub source_id: crate::web_audio::types::GraphObjectId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.nodesDisconnected/parameters/destinationId。
    #[serde(rename = "destinationId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub destination_id: crate::web_audio::types::GraphObjectId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAudio.event.nodesDisconnected/parameters/sourceOutputIndex。
    #[serde(rename = "sourceOutputIndex")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_output_index: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAudio.event.nodesDisconnected/parameters/destinationInputIndex。
    #[serde(rename = "destinationInputIndex")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_input_index: Option<ydp_core::FiniteF64>,
}
impl<'de> serde::Deserialize<'de> for NodesDisconnectedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NodesDisconnectedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.nodesDisconnected/parameters/contextId。
                    #[serde(rename = "contextId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub context_id: crate::web_audio::types::GraphObjectId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.nodesDisconnected/parameters/sourceId。
                    #[serde(rename = "sourceId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub source_id: crate::web_audio::types::GraphObjectId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.event.nodesDisconnected/parameters/destinationId。
                    #[serde(rename = "destinationId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub destination_id: crate::web_audio::types::GraphObjectId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAudio.event.nodesDisconnected/parameters/sourceOutputIndex。
                    #[serde(rename = "sourceOutputIndex")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub source_output_index: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAudio.event.nodesDisconnected/parameters/destinationInputIndex。
                    #[serde(rename = "destinationInputIndex")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub destination_input_index: Option<ydp_core::FiniteF64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(NodesDisconnectedParams {
                    context_id: fields.context_id,
                    source_id: fields.source_id,
                    destination_id: fields.destination_id,
                    source_output_index: fields.source_output_index,
                    destination_input_index: fields.destination_input_index,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAudio.event.nodesDisconnected。
///
///协议原文：
///
///<pre>Notifies that AudioNodes are disconnected. The destination can be null, and it means all the outgoing connections from the source are disconnected.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct NodesDisconnected;
impl ydp_core::Event for NodesDisconnected {
    type Params = NodesDisconnectedParams;
    const METHOD: &'static str = "WebAudio.nodesDisconnected";
}
