///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DigitalCredentials.command.setVirtualWalletBehavior/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetVirtualWalletBehaviorParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DigitalCredentials.command.setVirtualWalletBehavior/parameters/action。
    ///
    ///协议原文：
    ///
    ///<pre>The action of the virtual wallet.</pre>
    ///
    ///
    #[serde(rename = "action")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub action: crate::digital_credentials::types::VirtualWalletAction,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DigitalCredentials.command.setVirtualWalletBehavior/parameters/protocol。
    ///
    ///协议原文：
    ///
    ///<pre>The protocol identifier (e.g. "openid4vp"). Required when |action| is
    ///"respond", forbidden otherwise.</pre>
    ///
    ///
    #[serde(rename = "protocol")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub protocol: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DigitalCredentials.command.setVirtualWalletBehavior/parameters/response。
    ///
    ///协议原文：
    ///
    ///<pre>The response data object returned by the wallet.
    ///Required when |action| is "respond", forbidden otherwise.</pre>
    ///
    ///
    #[serde(rename = "response")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub response: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DigitalCredentials.command.setVirtualWalletBehavior/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>The frame to scope the virtual wallet behavior to.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub frame_id: Option<crate::page::types::FrameId>,
}
impl<'de> serde::Deserialize<'de> for SetVirtualWalletBehaviorParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetVirtualWalletBehaviorParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DigitalCredentials.command.setVirtualWalletBehavior/parameters/action。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The action of the virtual wallet.</pre>
                    ///
                    ///
                    #[serde(rename = "action")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub action: crate::digital_credentials::types::VirtualWalletAction,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DigitalCredentials.command.setVirtualWalletBehavior/parameters/protocol。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The protocol identifier (e.g. "openid4vp"). Required when |action| is
                    ///"respond", forbidden otherwise.</pre>
                    ///
                    ///
                    #[serde(rename = "protocol")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub protocol: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DigitalCredentials.command.setVirtualWalletBehavior/parameters/response。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The response data object returned by the wallet.
                    ///Required when |action| is "respond", forbidden otherwise.</pre>
                    ///
                    ///
                    #[serde(rename = "response")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub response: Option<std::collections::BTreeMap<String, serde_json::Value>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DigitalCredentials.command.setVirtualWalletBehavior/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The frame to scope the virtual wallet behavior to.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub frame_id: Option<crate::page::types::FrameId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetVirtualWalletBehaviorParams {
                    action: fields.action,
                    protocol: fields.protocol,
                    response: fields.response,
                    frame_id: fields.frame_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DigitalCredentials.command.setVirtualWalletBehavior。
///
///协议原文：
///
///<pre>Sets the behavior of the virtual wallet for digital credential requests
///issued from this frame.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetVirtualWalletBehavior;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DigitalCredentials.command.setVirtualWalletBehavior/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetVirtualWalletBehaviorResponse {}
impl<'de> serde::Deserialize<'de> for SetVirtualWalletBehaviorResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetVirtualWalletBehaviorResponse;
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
                Ok(SetVirtualWalletBehaviorResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetVirtualWalletBehavior {
    type Params = SetVirtualWalletBehaviorParams;
    type Response = SetVirtualWalletBehaviorResponse;
    const METHOD: &'static str = "DigitalCredentials.setVirtualWalletBehavior";
}
