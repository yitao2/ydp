///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAudio.type.AudioListener。
///
///协议原文：
///
///<pre>Protocol object for AudioListener</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct AudioListener {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioListener/properties/listenerId。
    #[serde(rename = "listenerId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub listener_id: crate::web_audio::types::GraphObjectId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioListener/properties/contextId。
    #[serde(rename = "contextId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub context_id: crate::web_audio::types::GraphObjectId,
}
impl<'de> serde::Deserialize<'de> for AudioListener {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AudioListener;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioListener/properties/listenerId。
                    #[serde(rename = "listenerId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub listener_id: crate::web_audio::types::GraphObjectId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioListener/properties/contextId。
                    #[serde(rename = "contextId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub context_id: crate::web_audio::types::GraphObjectId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AudioListener {
                    listener_id: fields.listener_id,
                    context_id: fields.context_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAudio.type.AudioNode。
///
///协议原文：
///
///<pre>Protocol object for AudioNode</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct AudioNode {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioNode/properties/nodeId。
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::web_audio::types::GraphObjectId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioNode/properties/contextId。
    #[serde(rename = "contextId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub context_id: crate::web_audio::types::GraphObjectId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioNode/properties/nodeType。
    #[serde(rename = "nodeType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_type: crate::web_audio::types::NodeType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioNode/properties/numberOfInputs。
    #[serde(rename = "numberOfInputs")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub number_of_inputs: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioNode/properties/numberOfOutputs。
    #[serde(rename = "numberOfOutputs")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub number_of_outputs: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioNode/properties/channelCount。
    #[serde(rename = "channelCount")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub channel_count: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioNode/properties/channelCountMode。
    #[serde(rename = "channelCountMode")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub channel_count_mode: crate::web_audio::types::ChannelCountMode,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioNode/properties/channelInterpretation。
    #[serde(rename = "channelInterpretation")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub channel_interpretation: crate::web_audio::types::ChannelInterpretation,
}
impl<'de> serde::Deserialize<'de> for AudioNode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AudioNode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioNode/properties/nodeId。
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::web_audio::types::GraphObjectId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioNode/properties/contextId。
                    #[serde(rename = "contextId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub context_id: crate::web_audio::types::GraphObjectId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioNode/properties/nodeType。
                    #[serde(rename = "nodeType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_type: crate::web_audio::types::NodeType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioNode/properties/numberOfInputs。
                    #[serde(rename = "numberOfInputs")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub number_of_inputs: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioNode/properties/numberOfOutputs。
                    #[serde(rename = "numberOfOutputs")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub number_of_outputs: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioNode/properties/channelCount。
                    #[serde(rename = "channelCount")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub channel_count: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioNode/properties/channelCountMode。
                    #[serde(rename = "channelCountMode")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub channel_count_mode: crate::web_audio::types::ChannelCountMode,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioNode/properties/channelInterpretation。
                    #[serde(rename = "channelInterpretation")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub channel_interpretation: crate::web_audio::types::ChannelInterpretation,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AudioNode {
                    node_id: fields.node_id,
                    context_id: fields.context_id,
                    node_type: fields.node_type,
                    number_of_inputs: fields.number_of_inputs,
                    number_of_outputs: fields.number_of_outputs,
                    channel_count: fields.channel_count,
                    channel_count_mode: fields.channel_count_mode,
                    channel_interpretation: fields.channel_interpretation,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAudio.type.AudioParam。
///
///协议原文：
///
///<pre>Protocol object for AudioParam</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct AudioParam {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioParam/properties/paramId。
    #[serde(rename = "paramId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub param_id: crate::web_audio::types::GraphObjectId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioParam/properties/nodeId。
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::web_audio::types::GraphObjectId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioParam/properties/contextId。
    #[serde(rename = "contextId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub context_id: crate::web_audio::types::GraphObjectId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioParam/properties/paramType。
    #[serde(rename = "paramType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub param_type: crate::web_audio::types::ParamType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioParam/properties/rate。
    #[serde(rename = "rate")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub rate: crate::web_audio::types::AutomationRate,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioParam/properties/defaultValue。
    #[serde(rename = "defaultValue")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub default_value: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioParam/properties/minValue。
    #[serde(rename = "minValue")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub min_value: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioParam/properties/maxValue。
    #[serde(rename = "maxValue")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub max_value: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for AudioParam {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AudioParam;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioParam/properties/paramId。
                    #[serde(rename = "paramId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub param_id: crate::web_audio::types::GraphObjectId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioParam/properties/nodeId。
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::web_audio::types::GraphObjectId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioParam/properties/contextId。
                    #[serde(rename = "contextId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub context_id: crate::web_audio::types::GraphObjectId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioParam/properties/paramType。
                    #[serde(rename = "paramType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub param_type: crate::web_audio::types::ParamType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioParam/properties/rate。
                    #[serde(rename = "rate")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub rate: crate::web_audio::types::AutomationRate,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioParam/properties/defaultValue。
                    #[serde(rename = "defaultValue")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub default_value: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioParam/properties/minValue。
                    #[serde(rename = "minValue")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub min_value: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.AudioParam/properties/maxValue。
                    #[serde(rename = "maxValue")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub max_value: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AudioParam {
                    param_id: fields.param_id,
                    node_id: fields.node_id,
                    context_id: fields.context_id,
                    param_type: fields.param_type,
                    rate: fields.rate,
                    default_value: fields.default_value,
                    min_value: fields.min_value,
                    max_value: fields.max_value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：WebAudio.type.AutomationRate。
///
///协议原文：
///
///<pre>Enum of AudioParam::AutomationRate from the spec</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AutomationRate {
    ///协议值 "a-rate"。
    ARate,
    ///协议值 "k-rate"。
    KRate,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl AutomationRate {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::web_audio::types::AutomationRate::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::ARate => "a-rate",
            Self::KRate => "k-rate",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for AutomationRate {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for AutomationRate {
    fn from(value: String) -> Self {
        match value.as_str() {
            "a-rate" => AutomationRate::ARate,
            "k-rate" => AutomationRate::KRate,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for AutomationRate {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AutomationRate {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AutomationRate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "a-rate" => AutomationRate::ARate,
                    "k-rate" => AutomationRate::KRate,
                    _ => AutomationRate::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(AutomationRate::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAudio.type.BaseAudioContext。
///
///协议原文：
///
///<pre>Protocol object for BaseAudioContext</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct BaseAudioContext {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.BaseAudioContext/properties/contextId。
    #[serde(rename = "contextId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub context_id: crate::web_audio::types::GraphObjectId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.BaseAudioContext/properties/contextType。
    #[serde(rename = "contextType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub context_type: crate::web_audio::types::ContextType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.BaseAudioContext/properties/contextState。
    #[serde(rename = "contextState")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub context_state: crate::web_audio::types::ContextState,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAudio.type.BaseAudioContext/properties/realtimeData。
    #[serde(rename = "realtimeData")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub realtime_data: Option<crate::web_audio::types::ContextRealtimeData>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.BaseAudioContext/properties/callbackBufferSize。
    ///
    ///协议原文：
    ///
    ///<pre>Platform-dependent callback buffer size.</pre>
    ///
    ///
    #[serde(rename = "callbackBufferSize")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub callback_buffer_size: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.BaseAudioContext/properties/maxOutputChannelCount。
    ///
    ///协议原文：
    ///
    ///<pre>Number of output channels supported by audio hardware in use.</pre>
    ///
    ///
    #[serde(rename = "maxOutputChannelCount")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub max_output_channel_count: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.BaseAudioContext/properties/sampleRate。
    ///
    ///协议原文：
    ///
    ///<pre>Context sample rate.</pre>
    ///
    ///
    #[serde(rename = "sampleRate")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub sample_rate: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.BaseAudioContext/properties/renderQuantumSize。
    #[serde(rename = "renderQuantumSize")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub render_quantum_size: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for BaseAudioContext {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BaseAudioContext;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.BaseAudioContext/properties/contextId。
                    #[serde(rename = "contextId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub context_id: crate::web_audio::types::GraphObjectId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.BaseAudioContext/properties/contextType。
                    #[serde(rename = "contextType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub context_type: crate::web_audio::types::ContextType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.BaseAudioContext/properties/contextState。
                    #[serde(rename = "contextState")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub context_state: crate::web_audio::types::ContextState,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAudio.type.BaseAudioContext/properties/realtimeData。
                    #[serde(rename = "realtimeData")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub realtime_data: Option<crate::web_audio::types::ContextRealtimeData>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.BaseAudioContext/properties/callbackBufferSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Platform-dependent callback buffer size.</pre>
                    ///
                    ///
                    #[serde(rename = "callbackBufferSize")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub callback_buffer_size: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.BaseAudioContext/properties/maxOutputChannelCount。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Number of output channels supported by audio hardware in use.</pre>
                    ///
                    ///
                    #[serde(rename = "maxOutputChannelCount")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub max_output_channel_count: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.BaseAudioContext/properties/sampleRate。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Context sample rate.</pre>
                    ///
                    ///
                    #[serde(rename = "sampleRate")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub sample_rate: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.BaseAudioContext/properties/renderQuantumSize。
                    #[serde(rename = "renderQuantumSize")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub render_quantum_size: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(BaseAudioContext {
                    context_id: fields.context_id,
                    context_type: fields.context_type,
                    context_state: fields.context_state,
                    realtime_data: fields.realtime_data,
                    callback_buffer_size: fields.callback_buffer_size,
                    max_output_channel_count: fields.max_output_channel_count,
                    sample_rate: fields.sample_rate,
                    render_quantum_size: fields.render_quantum_size,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：WebAudio.type.ChannelCountMode。
///
///协议原文：
///
///<pre>Enum of AudioNode::ChannelCountMode from the spec</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ChannelCountMode {
    ///协议值 "clamped-max"。
    ClampedMax,
    ///协议值 "explicit"。
    Explicit,
    ///协议值 "max"。
    Max,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ChannelCountMode {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::web_audio::types::ChannelCountMode::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::ClampedMax => "clamped-max",
            Self::Explicit => "explicit",
            Self::Max => "max",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ChannelCountMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ChannelCountMode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "clamped-max" => ChannelCountMode::ClampedMax,
            "explicit" => ChannelCountMode::Explicit,
            "max" => ChannelCountMode::Max,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ChannelCountMode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ChannelCountMode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ChannelCountMode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "clamped-max" => ChannelCountMode::ClampedMax,
                    "explicit" => ChannelCountMode::Explicit,
                    "max" => ChannelCountMode::Max,
                    _ => ChannelCountMode::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ChannelCountMode::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：WebAudio.type.ChannelInterpretation。
///
///协议原文：
///
///<pre>Enum of AudioNode::ChannelInterpretation from the spec</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ChannelInterpretation {
    ///协议值 "discrete"。
    Discrete,
    ///协议值 "speakers"。
    Speakers,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ChannelInterpretation {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::web_audio::types::ChannelInterpretation::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Discrete => "discrete",
            Self::Speakers => "speakers",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ChannelInterpretation {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ChannelInterpretation {
    fn from(value: String) -> Self {
        match value.as_str() {
            "discrete" => ChannelInterpretation::Discrete,
            "speakers" => ChannelInterpretation::Speakers,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ChannelInterpretation {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ChannelInterpretation {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ChannelInterpretation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "discrete" => ChannelInterpretation::Discrete,
                    "speakers" => ChannelInterpretation::Speakers,
                    _ => ChannelInterpretation::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ChannelInterpretation::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAudio.type.ContextRealtimeData。
///
///协议原文：
///
///<pre>Fields in AudioContext that change in real-time.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ContextRealtimeData {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.ContextRealtimeData/properties/currentTime。
    ///
    ///协议原文：
    ///
    ///<pre>The current context time in second in BaseAudioContext.</pre>
    ///
    ///
    #[serde(rename = "currentTime")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub current_time: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.ContextRealtimeData/properties/renderCapacity。
    ///
    ///协议原文：
    ///
    ///<pre>The time spent on rendering graph divided by render quantum duration,
    ///and multiplied by 100. 100 means the audio renderer reached the full
    ///capacity and glitch may occur.</pre>
    ///
    ///
    #[serde(rename = "renderCapacity")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub render_capacity: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.ContextRealtimeData/properties/callbackIntervalMean。
    ///
    ///协议原文：
    ///
    ///<pre>A running mean of callback interval.</pre>
    ///
    ///
    #[serde(rename = "callbackIntervalMean")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub callback_interval_mean: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.ContextRealtimeData/properties/callbackIntervalVariance。
    ///
    ///协议原文：
    ///
    ///<pre>A running variance of callback interval.</pre>
    ///
    ///
    #[serde(rename = "callbackIntervalVariance")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub callback_interval_variance: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for ContextRealtimeData {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContextRealtimeData;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.ContextRealtimeData/properties/currentTime。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The current context time in second in BaseAudioContext.</pre>
                    ///
                    ///
                    #[serde(rename = "currentTime")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub current_time: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.ContextRealtimeData/properties/renderCapacity。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The time spent on rendering graph divided by render quantum duration,
                    ///and multiplied by 100. 100 means the audio renderer reached the full
                    ///capacity and glitch may occur.</pre>
                    ///
                    ///
                    #[serde(rename = "renderCapacity")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub render_capacity: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.ContextRealtimeData/properties/callbackIntervalMean。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A running mean of callback interval.</pre>
                    ///
                    ///
                    #[serde(rename = "callbackIntervalMean")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub callback_interval_mean: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAudio.type.ContextRealtimeData/properties/callbackIntervalVariance。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A running variance of callback interval.</pre>
                    ///
                    ///
                    #[serde(rename = "callbackIntervalVariance")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub callback_interval_variance: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ContextRealtimeData {
                    current_time: fields.current_time,
                    render_capacity: fields.render_capacity,
                    callback_interval_mean: fields.callback_interval_mean,
                    callback_interval_variance: fields.callback_interval_variance,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：WebAudio.type.ContextState。
///
///协议原文：
///
///<pre>Enum of AudioContextState from the spec</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ContextState {
    ///协议值 "suspended"。
    Suspended,
    ///协议值 "running"。
    Running,
    ///协议值 "closed"。
    Closed,
    ///协议值 "interrupted"。
    Interrupted,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ContextState {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::web_audio::types::ContextState::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Suspended => "suspended",
            Self::Running => "running",
            Self::Closed => "closed",
            Self::Interrupted => "interrupted",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ContextState {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ContextState {
    fn from(value: String) -> Self {
        match value.as_str() {
            "suspended" => ContextState::Suspended,
            "running" => ContextState::Running,
            "closed" => ContextState::Closed,
            "interrupted" => ContextState::Interrupted,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ContextState {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ContextState {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContextState;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "suspended" => ContextState::Suspended,
                    "running" => ContextState::Running,
                    "closed" => ContextState::Closed,
                    "interrupted" => ContextState::Interrupted,
                    _ => ContextState::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ContextState::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：WebAudio.type.ContextType。
///
///协议原文：
///
///<pre>Enum of BaseAudioContext types</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ContextType {
    ///协议值 "realtime"。
    Realtime,
    ///协议值 "offline"。
    Offline,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ContextType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::web_audio::types::ContextType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Realtime => "realtime",
            Self::Offline => "offline",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ContextType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ContextType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "realtime" => ContextType::Realtime,
            "offline" => ContextType::Offline,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ContextType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ContextType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContextType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "realtime" => ContextType::Realtime,
                    "offline" => ContextType::Offline,
                    _ => ContextType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ContextType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：WebAudio.type.GraphObjectId。
///
///协议原文：
///
///<pre>An unique ID for a graph object (AudioContext, AudioNode, AudioParam) in Web Audio API</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct GraphObjectId(String);
impl GraphObjectId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::web_audio::types::GraphObjectId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::web_audio::types::GraphObjectId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::web_audio::types::GraphObjectId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::web_audio::types::GraphObjectId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::web_audio::types::GraphObjectId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：WebAudio.type.NodeType。
///
///协议原文：
///
///<pre>Enum of AudioNode types</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct NodeType(String);
impl NodeType {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::web_audio::types::NodeType = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::web_audio::types::NodeType::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::web_audio::types::NodeType = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::web_audio::types::NodeType::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::web_audio::types::NodeType::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：WebAudio.type.ParamType。
///
///协议原文：
///
///<pre>Enum of AudioParam types</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ParamType(String);
impl ParamType {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::web_audio::types::ParamType = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::web_audio::types::ParamType::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::web_audio::types::ParamType = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::web_audio::types::ParamType::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::web_audio::types::ParamType::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
