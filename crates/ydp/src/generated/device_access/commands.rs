///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DeviceAccess.command.cancelPrompt/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CancelPromptParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DeviceAccess.command.cancelPrompt/parameters/id。
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: crate::device_access::types::RequestId,
}
impl<'de> serde::Deserialize<'de> for CancelPromptParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CancelPromptParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DeviceAccess.command.cancelPrompt/parameters/id。
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: crate::device_access::types::RequestId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CancelPromptParams { id: fields.id })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DeviceAccess.command.cancelPrompt。
///
///协议原文：
///
///<pre>Cancel a prompt in response to a DeviceAccess.deviceRequestPrompted event.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct CancelPrompt;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DeviceAccess.command.cancelPrompt/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CancelPromptResponse {}
impl<'de> serde::Deserialize<'de> for CancelPromptResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CancelPromptResponse;
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
                Ok(CancelPromptResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for CancelPrompt {
    type Params = CancelPromptParams;
    type Response = CancelPromptResponse;
    const METHOD: &'static str = "DeviceAccess.cancelPrompt";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DeviceAccess.command.disable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DeviceAccess.command.disable。
///
///协议原文：
///
///<pre>Disable events in this domain.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DeviceAccess.command.disable/returns。
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
    const METHOD: &'static str = "DeviceAccess.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DeviceAccess.command.enable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DeviceAccess.command.enable。
///
///协议原文：
///
///<pre>Enable events in this domain.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DeviceAccess.command.enable/returns。
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
    const METHOD: &'static str = "DeviceAccess.enable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DeviceAccess.command.selectPrompt/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SelectPromptParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DeviceAccess.command.selectPrompt/parameters/id。
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: crate::device_access::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DeviceAccess.command.selectPrompt/parameters/deviceId。
    #[serde(rename = "deviceId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub device_id: crate::device_access::types::DeviceId,
}
impl<'de> serde::Deserialize<'de> for SelectPromptParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SelectPromptParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DeviceAccess.command.selectPrompt/parameters/id。
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: crate::device_access::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DeviceAccess.command.selectPrompt/parameters/deviceId。
                    #[serde(rename = "deviceId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub device_id: crate::device_access::types::DeviceId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SelectPromptParams {
                    id: fields.id,
                    device_id: fields.device_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DeviceAccess.command.selectPrompt。
///
///协议原文：
///
///<pre>Select a device in response to a DeviceAccess.deviceRequestPrompted event.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SelectPrompt;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DeviceAccess.command.selectPrompt/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SelectPromptResponse {}
impl<'de> serde::Deserialize<'de> for SelectPromptResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SelectPromptResponse;
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
                Ok(SelectPromptResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SelectPrompt {
    type Params = SelectPromptParams;
    type Response = SelectPromptResponse;
    const METHOD: &'static str = "DeviceAccess.selectPrompt";
}
