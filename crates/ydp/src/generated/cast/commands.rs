///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Cast.command.disable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Cast.command.disable。
///
///协议原文：
///
///<pre>Stops observing for sinks and issues.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Cast.command.disable/returns。
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
    const METHOD: &'static str = "Cast.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Cast.command.enable/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct EnableParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Cast.command.enable/parameters/presentationUrl。
    #[serde(rename = "presentationUrl")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub presentation_url: Option<String>,
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
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Cast.command.enable/parameters/presentationUrl。
                    #[serde(rename = "presentationUrl")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub presentation_url: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EnableParams {
                    presentation_url: fields.presentation_url,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Cast.command.enable。
///
///协议原文：
///
///<pre>Starts observing for sinks that can be used for tab mirroring, and if set,
///sinks compatible with |presentationUrl| as well. When sinks are found, a
///|sinksUpdated| event is fired.
///Also starts observing for issue messages. When an issue is added or removed,
///an |issueUpdated| event is fired.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Cast.command.enable/returns。
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
    const METHOD: &'static str = "Cast.enable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Cast.command.setSinkToUse/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetSinkToUseParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Cast.command.setSinkToUse/parameters/sinkName。
    #[serde(rename = "sinkName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub sink_name: String,
}
impl<'de> serde::Deserialize<'de> for SetSinkToUseParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetSinkToUseParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Cast.command.setSinkToUse/parameters/sinkName。
                    #[serde(rename = "sinkName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub sink_name: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetSinkToUseParams {
                    sink_name: fields.sink_name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Cast.command.setSinkToUse。
///
///协议原文：
///
///<pre>Sets a sink to be used when the web page requests the browser to choose a
///sink via Presentation API, Remote Playback API, or Cast SDK.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetSinkToUse;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Cast.command.setSinkToUse/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetSinkToUseResponse {}
impl<'de> serde::Deserialize<'de> for SetSinkToUseResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetSinkToUseResponse;
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
                Ok(SetSinkToUseResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetSinkToUse {
    type Params = SetSinkToUseParams;
    type Response = SetSinkToUseResponse;
    const METHOD: &'static str = "Cast.setSinkToUse";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Cast.command.startDesktopMirroring/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StartDesktopMirroringParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Cast.command.startDesktopMirroring/parameters/sinkName。
    #[serde(rename = "sinkName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub sink_name: String,
}
impl<'de> serde::Deserialize<'de> for StartDesktopMirroringParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StartDesktopMirroringParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Cast.command.startDesktopMirroring/parameters/sinkName。
                    #[serde(rename = "sinkName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub sink_name: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StartDesktopMirroringParams {
                    sink_name: fields.sink_name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Cast.command.startDesktopMirroring。
///
///协议原文：
///
///<pre>Starts mirroring the desktop to the sink.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct StartDesktopMirroring;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Cast.command.startDesktopMirroring/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StartDesktopMirroringResponse {}
impl<'de> serde::Deserialize<'de> for StartDesktopMirroringResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StartDesktopMirroringResponse;
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
                Ok(StartDesktopMirroringResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for StartDesktopMirroring {
    type Params = StartDesktopMirroringParams;
    type Response = StartDesktopMirroringResponse;
    const METHOD: &'static str = "Cast.startDesktopMirroring";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Cast.command.startTabMirroring/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StartTabMirroringParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Cast.command.startTabMirroring/parameters/sinkName。
    #[serde(rename = "sinkName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub sink_name: String,
}
impl<'de> serde::Deserialize<'de> for StartTabMirroringParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StartTabMirroringParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Cast.command.startTabMirroring/parameters/sinkName。
                    #[serde(rename = "sinkName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub sink_name: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StartTabMirroringParams {
                    sink_name: fields.sink_name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Cast.command.startTabMirroring。
///
///协议原文：
///
///<pre>Starts mirroring the tab to the sink.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct StartTabMirroring;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Cast.command.startTabMirroring/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StartTabMirroringResponse {}
impl<'de> serde::Deserialize<'de> for StartTabMirroringResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StartTabMirroringResponse;
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
                Ok(StartTabMirroringResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for StartTabMirroring {
    type Params = StartTabMirroringParams;
    type Response = StartTabMirroringResponse;
    const METHOD: &'static str = "Cast.startTabMirroring";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Cast.command.stopCasting/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StopCastingParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Cast.command.stopCasting/parameters/sinkName。
    #[serde(rename = "sinkName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub sink_name: String,
}
impl<'de> serde::Deserialize<'de> for StopCastingParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StopCastingParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Cast.command.stopCasting/parameters/sinkName。
                    #[serde(rename = "sinkName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub sink_name: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StopCastingParams {
                    sink_name: fields.sink_name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Cast.command.stopCasting。
///
///协议原文：
///
///<pre>Stops the active Cast session on the sink.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct StopCasting;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Cast.command.stopCasting/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StopCastingResponse {}
impl<'de> serde::Deserialize<'de> for StopCastingResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StopCastingResponse;
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
                Ok(StopCastingResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for StopCasting {
    type Params = StopCastingParams;
    type Response = StopCastingResponse;
    const METHOD: &'static str = "Cast.stopCasting";
}
