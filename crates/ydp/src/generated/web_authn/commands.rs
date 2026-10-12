///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.command.addCredential/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddCredentialParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.addCredential/parameters/authenticatorId。
    #[serde(rename = "authenticatorId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.addCredential/parameters/credential。
    #[serde(rename = "credential")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub credential: crate::web_authn::types::Credential,
}
impl<'de> serde::Deserialize<'de> for AddCredentialParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddCredentialParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.addCredential/parameters/authenticatorId。
                    #[serde(rename = "authenticatorId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.addCredential/parameters/credential。
                    #[serde(rename = "credential")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub credential: crate::web_authn::types::Credential,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AddCredentialParams {
                    authenticator_id: fields.authenticator_id,
                    credential: fields.credential,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAuthn.command.addCredential。
///
///协议原文：
///
///<pre>Adds the credential to the specified authenticator.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct AddCredential;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.command.addCredential/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddCredentialResponse {}
impl<'de> serde::Deserialize<'de> for AddCredentialResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddCredentialResponse;
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
                Ok(AddCredentialResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for AddCredential {
    type Params = AddCredentialParams;
    type Response = AddCredentialResponse;
    const METHOD: &'static str = "WebAuthn.addCredential";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.command.addVirtualAuthenticator/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddVirtualAuthenticatorParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.addVirtualAuthenticator/parameters/options。
    #[serde(rename = "options")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub options: crate::web_authn::types::VirtualAuthenticatorOptions,
}
impl<'de> serde::Deserialize<'de> for AddVirtualAuthenticatorParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddVirtualAuthenticatorParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.addVirtualAuthenticator/parameters/options。
                    #[serde(rename = "options")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub options: crate::web_authn::types::VirtualAuthenticatorOptions,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AddVirtualAuthenticatorParams {
                    options: fields.options,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAuthn.command.addVirtualAuthenticator。
///
///协议原文：
///
///<pre>Creates and adds a virtual authenticator.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct AddVirtualAuthenticator;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.command.addVirtualAuthenticator/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddVirtualAuthenticatorResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.addVirtualAuthenticator/returns/authenticatorId。
    #[serde(rename = "authenticatorId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
}
impl<'de> serde::Deserialize<'de> for AddVirtualAuthenticatorResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddVirtualAuthenticatorResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.addVirtualAuthenticator/returns/authenticatorId。
                    #[serde(rename = "authenticatorId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AddVirtualAuthenticatorResponse {
                    authenticator_id: fields.authenticator_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for AddVirtualAuthenticator {
    type Params = AddVirtualAuthenticatorParams;
    type Response = AddVirtualAuthenticatorResponse;
    const METHOD: &'static str = "WebAuthn.addVirtualAuthenticator";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.command.clearCredentials/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearCredentialsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.clearCredentials/parameters/authenticatorId。
    #[serde(rename = "authenticatorId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
}
impl<'de> serde::Deserialize<'de> for ClearCredentialsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearCredentialsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.clearCredentials/parameters/authenticatorId。
                    #[serde(rename = "authenticatorId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ClearCredentialsParams {
                    authenticator_id: fields.authenticator_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAuthn.command.clearCredentials。
///
///协议原文：
///
///<pre>Clears all the credentials from the specified device.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ClearCredentials;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.command.clearCredentials/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearCredentialsResponse {}
impl<'de> serde::Deserialize<'de> for ClearCredentialsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearCredentialsResponse;
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
                Ok(ClearCredentialsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ClearCredentials {
    type Params = ClearCredentialsParams;
    type Response = ClearCredentialsResponse;
    const METHOD: &'static str = "WebAuthn.clearCredentials";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.command.disable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAuthn.command.disable。
///
///协议原文：
///
///<pre>Disable the WebAuthn domain.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.command.disable/returns。
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
    const METHOD: &'static str = "WebAuthn.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.command.enable/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct EnableParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.command.enable/parameters/enableUI。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to enable the WebAuthn user interface. Enabling the UI is
    ///recommended for debugging and demo purposes, as it is closer to the real
    ///experience. Disabling the UI is recommended for automated testing.
    ///Supported at the embedder's discretion if UI is available.
    ///Defaults to false.</pre>
    ///
    ///
    #[serde(rename = "enableUI")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_ui: Option<bool>,
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
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.command.enable/parameters/enableUI。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to enable the WebAuthn user interface. Enabling the UI is
                    ///recommended for debugging and demo purposes, as it is closer to the real
                    ///experience. Disabling the UI is recommended for automated testing.
                    ///Supported at the embedder's discretion if UI is available.
                    ///Defaults to false.</pre>
                    ///
                    ///
                    #[serde(rename = "enableUI")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub enable_ui: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EnableParams {
                    enable_ui: fields.enable_ui,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAuthn.command.enable。
///
///协议原文：
///
///<pre>Enable the WebAuthn domain and start intercepting credential storage and
///retrieval with a virtual authenticator.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.command.enable/returns。
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
    const METHOD: &'static str = "WebAuthn.enable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.command.getCredential/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetCredentialParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.getCredential/parameters/authenticatorId。
    #[serde(rename = "authenticatorId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.getCredential/parameters/credentialId。
    #[serde(rename = "credentialId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub credential_id: String,
}
impl<'de> serde::Deserialize<'de> for GetCredentialParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetCredentialParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.getCredential/parameters/authenticatorId。
                    #[serde(rename = "authenticatorId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.getCredential/parameters/credentialId。
                    #[serde(rename = "credentialId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub credential_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetCredentialParams {
                    authenticator_id: fields.authenticator_id,
                    credential_id: fields.credential_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAuthn.command.getCredential。
///
///协议原文：
///
///<pre>Returns a single credential stored in the given virtual authenticator that
///matches the credential ID.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetCredential;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.command.getCredential/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetCredentialResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.getCredential/returns/credential。
    #[serde(rename = "credential")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub credential: crate::web_authn::types::Credential,
}
impl<'de> serde::Deserialize<'de> for GetCredentialResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetCredentialResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.getCredential/returns/credential。
                    #[serde(rename = "credential")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub credential: crate::web_authn::types::Credential,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetCredentialResponse {
                    credential: fields.credential,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetCredential {
    type Params = GetCredentialParams;
    type Response = GetCredentialResponse;
    const METHOD: &'static str = "WebAuthn.getCredential";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.command.getCredentials/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetCredentialsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.getCredentials/parameters/authenticatorId。
    #[serde(rename = "authenticatorId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
}
impl<'de> serde::Deserialize<'de> for GetCredentialsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetCredentialsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.getCredentials/parameters/authenticatorId。
                    #[serde(rename = "authenticatorId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetCredentialsParams {
                    authenticator_id: fields.authenticator_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAuthn.command.getCredentials。
///
///协议原文：
///
///<pre>Returns all the credentials stored in the given virtual authenticator.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetCredentials;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.command.getCredentials/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetCredentialsResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.getCredentials/returns/credentials。
    #[serde(rename = "credentials")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub credentials: Vec<crate::web_authn::types::Credential>,
}
impl<'de> serde::Deserialize<'de> for GetCredentialsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetCredentialsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.getCredentials/returns/credentials。
                    #[serde(rename = "credentials")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub credentials: Vec<crate::web_authn::types::Credential>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetCredentialsResponse {
                    credentials: fields.credentials,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetCredentials {
    type Params = GetCredentialsParams;
    type Response = GetCredentialsResponse;
    const METHOD: &'static str = "WebAuthn.getCredentials";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.command.removeCredential/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveCredentialParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.removeCredential/parameters/authenticatorId。
    #[serde(rename = "authenticatorId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.removeCredential/parameters/credentialId。
    #[serde(rename = "credentialId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub credential_id: String,
}
impl<'de> serde::Deserialize<'de> for RemoveCredentialParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveCredentialParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.removeCredential/parameters/authenticatorId。
                    #[serde(rename = "authenticatorId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.removeCredential/parameters/credentialId。
                    #[serde(rename = "credentialId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub credential_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RemoveCredentialParams {
                    authenticator_id: fields.authenticator_id,
                    credential_id: fields.credential_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAuthn.command.removeCredential。
///
///协议原文：
///
///<pre>Removes a credential from the authenticator.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RemoveCredential;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.command.removeCredential/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveCredentialResponse {}
impl<'de> serde::Deserialize<'de> for RemoveCredentialResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveCredentialResponse;
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
                Ok(RemoveCredentialResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RemoveCredential {
    type Params = RemoveCredentialParams;
    type Response = RemoveCredentialResponse;
    const METHOD: &'static str = "WebAuthn.removeCredential";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.command.removeVirtualAuthenticator/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveVirtualAuthenticatorParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.removeVirtualAuthenticator/parameters/authenticatorId。
    #[serde(rename = "authenticatorId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
}
impl<'de> serde::Deserialize<'de> for RemoveVirtualAuthenticatorParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveVirtualAuthenticatorParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.removeVirtualAuthenticator/parameters/authenticatorId。
                    #[serde(rename = "authenticatorId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RemoveVirtualAuthenticatorParams {
                    authenticator_id: fields.authenticator_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAuthn.command.removeVirtualAuthenticator。
///
///协议原文：
///
///<pre>Removes the given authenticator.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RemoveVirtualAuthenticator;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.command.removeVirtualAuthenticator/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveVirtualAuthenticatorResponse {}
impl<'de> serde::Deserialize<'de> for RemoveVirtualAuthenticatorResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveVirtualAuthenticatorResponse;
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
                Ok(RemoveVirtualAuthenticatorResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RemoveVirtualAuthenticator {
    type Params = RemoveVirtualAuthenticatorParams;
    type Response = RemoveVirtualAuthenticatorResponse;
    const METHOD: &'static str = "WebAuthn.removeVirtualAuthenticator";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.command.setAutomaticPresenceSimulation/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetAutomaticPresenceSimulationParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.setAutomaticPresenceSimulation/parameters/authenticatorId。
    #[serde(rename = "authenticatorId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.setAutomaticPresenceSimulation/parameters/enabled。
    #[serde(rename = "enabled")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub enabled: bool,
}
impl<'de> serde::Deserialize<'de> for SetAutomaticPresenceSimulationParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetAutomaticPresenceSimulationParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.setAutomaticPresenceSimulation/parameters/authenticatorId。
                    #[serde(rename = "authenticatorId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.setAutomaticPresenceSimulation/parameters/enabled。
                    #[serde(rename = "enabled")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub enabled: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetAutomaticPresenceSimulationParams {
                    authenticator_id: fields.authenticator_id,
                    enabled: fields.enabled,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAuthn.command.setAutomaticPresenceSimulation。
///
///协议原文：
///
///<pre>Sets whether tests of user presence will succeed immediately (if true) or fail to resolve (if false) for an authenticator.
///The default is true.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetAutomaticPresenceSimulation;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.command.setAutomaticPresenceSimulation/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetAutomaticPresenceSimulationResponse {}
impl<'de> serde::Deserialize<'de> for SetAutomaticPresenceSimulationResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetAutomaticPresenceSimulationResponse;
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
                Ok(SetAutomaticPresenceSimulationResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetAutomaticPresenceSimulation {
    type Params = SetAutomaticPresenceSimulationParams;
    type Response = SetAutomaticPresenceSimulationResponse;
    const METHOD: &'static str = "WebAuthn.setAutomaticPresenceSimulation";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.command.setCredentialProperties/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetCredentialPropertiesParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.setCredentialProperties/parameters/authenticatorId。
    #[serde(rename = "authenticatorId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.setCredentialProperties/parameters/credentialId。
    #[serde(rename = "credentialId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub credential_id: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.command.setCredentialProperties/parameters/backupEligibility。
    #[serde(rename = "backupEligibility")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub backup_eligibility: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.command.setCredentialProperties/parameters/backupState。
    #[serde(rename = "backupState")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub backup_state: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.command.setCredentialProperties/parameters/activeCmtgKeyIndex。
    #[serde(rename = "activeCmtgKeyIndex")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub active_cmtg_key_index: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.command.setCredentialProperties/parameters/generateCmtgKeyOnNextOperation。
    #[serde(rename = "generateCmtgKeyOnNextOperation")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub generate_cmtg_key_on_next_operation: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.command.setCredentialProperties/parameters/signCount。
    ///
    ///协议原文：
    ///
    ///<pre>Must be equal to or greater than -1.
    ///If -1, the signature counter is removed from the credential, and every
    ///assertion operation will report a value of 0.
    ///See https://w3c.github.io/webauthn/#signature-counter</pre>
    ///
    ///
    #[serde(rename = "signCount")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub sign_count: Option<ydp_core::FiniteF64>,
}
impl<'de> serde::Deserialize<'de> for SetCredentialPropertiesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetCredentialPropertiesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.setCredentialProperties/parameters/authenticatorId。
                    #[serde(rename = "authenticatorId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.setCredentialProperties/parameters/credentialId。
                    #[serde(rename = "credentialId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub credential_id: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.command.setCredentialProperties/parameters/backupEligibility。
                    #[serde(rename = "backupEligibility")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub backup_eligibility: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.command.setCredentialProperties/parameters/backupState。
                    #[serde(rename = "backupState")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub backup_state: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.command.setCredentialProperties/parameters/activeCmtgKeyIndex。
                    #[serde(rename = "activeCmtgKeyIndex")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub active_cmtg_key_index: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.command.setCredentialProperties/parameters/generateCmtgKeyOnNextOperation。
                    #[serde(rename = "generateCmtgKeyOnNextOperation")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub generate_cmtg_key_on_next_operation: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.command.setCredentialProperties/parameters/signCount。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Must be equal to or greater than -1.
                    ///If -1, the signature counter is removed from the credential, and every
                    ///assertion operation will report a value of 0.
                    ///See https://w3c.github.io/webauthn/#signature-counter</pre>
                    ///
                    ///
                    #[serde(rename = "signCount")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub sign_count: Option<ydp_core::FiniteF64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetCredentialPropertiesParams {
                    authenticator_id: fields.authenticator_id,
                    credential_id: fields.credential_id,
                    backup_eligibility: fields.backup_eligibility,
                    backup_state: fields.backup_state,
                    active_cmtg_key_index: fields.active_cmtg_key_index,
                    generate_cmtg_key_on_next_operation: fields.generate_cmtg_key_on_next_operation,
                    sign_count: fields.sign_count,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAuthn.command.setCredentialProperties。
///
///协议原文：
///
///<pre>Allows setting credential properties.
///https://w3c.github.io/webauthn/#sctn-automation-set-credential-properties</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetCredentialProperties;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.command.setCredentialProperties/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetCredentialPropertiesResponse {}
impl<'de> serde::Deserialize<'de> for SetCredentialPropertiesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetCredentialPropertiesResponse;
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
                Ok(SetCredentialPropertiesResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetCredentialProperties {
    type Params = SetCredentialPropertiesParams;
    type Response = SetCredentialPropertiesResponse;
    const METHOD: &'static str = "WebAuthn.setCredentialProperties";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.command.setResponseOverrideBits/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetResponseOverrideBitsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.setResponseOverrideBits/parameters/authenticatorId。
    #[serde(rename = "authenticatorId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.command.setResponseOverrideBits/parameters/isBogusSignature。
    ///
    ///协议原文：
    ///
    ///<pre>If isBogusSignature is set, overrides the signature in the authenticator response to be zero.
    ///Defaults to false.</pre>
    ///
    ///
    #[serde(rename = "isBogusSignature")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_bogus_signature: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.command.setResponseOverrideBits/parameters/isBadUV。
    ///
    ///协议原文：
    ///
    ///<pre>If isBadUV is set, overrides the UV bit in the flags in the authenticator response to
    ///be zero. Defaults to false.</pre>
    ///
    ///
    #[serde(rename = "isBadUV")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_bad_uv: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.command.setResponseOverrideBits/parameters/isBadUP。
    ///
    ///协议原文：
    ///
    ///<pre>If isBadUP is set, overrides the UP bit in the flags in the authenticator response to
    ///be zero. Defaults to false.</pre>
    ///
    ///
    #[serde(rename = "isBadUP")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_bad_up: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for SetResponseOverrideBitsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetResponseOverrideBitsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.setResponseOverrideBits/parameters/authenticatorId。
                    #[serde(rename = "authenticatorId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.command.setResponseOverrideBits/parameters/isBogusSignature。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If isBogusSignature is set, overrides the signature in the authenticator response to be zero.
                    ///Defaults to false.</pre>
                    ///
                    ///
                    #[serde(rename = "isBogusSignature")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub is_bogus_signature: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.command.setResponseOverrideBits/parameters/isBadUV。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If isBadUV is set, overrides the UV bit in the flags in the authenticator response to
                    ///be zero. Defaults to false.</pre>
                    ///
                    ///
                    #[serde(rename = "isBadUV")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub is_bad_uv: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.command.setResponseOverrideBits/parameters/isBadUP。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If isBadUP is set, overrides the UP bit in the flags in the authenticator response to
                    ///be zero. Defaults to false.</pre>
                    ///
                    ///
                    #[serde(rename = "isBadUP")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub is_bad_up: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetResponseOverrideBitsParams {
                    authenticator_id: fields.authenticator_id,
                    is_bogus_signature: fields.is_bogus_signature,
                    is_bad_uv: fields.is_bad_uv,
                    is_bad_up: fields.is_bad_up,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAuthn.command.setResponseOverrideBits。
///
///协议原文：
///
///<pre>Resets parameters isBogusSignature, isBadUV, isBadUP to false if they are not present.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetResponseOverrideBits;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.command.setResponseOverrideBits/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetResponseOverrideBitsResponse {}
impl<'de> serde::Deserialize<'de> for SetResponseOverrideBitsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetResponseOverrideBitsResponse;
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
                Ok(SetResponseOverrideBitsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetResponseOverrideBits {
    type Params = SetResponseOverrideBitsParams;
    type Response = SetResponseOverrideBitsResponse;
    const METHOD: &'static str = "WebAuthn.setResponseOverrideBits";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.command.setUserVerified/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetUserVerifiedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.setUserVerified/parameters/authenticatorId。
    #[serde(rename = "authenticatorId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.setUserVerified/parameters/isUserVerified。
    #[serde(rename = "isUserVerified")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub is_user_verified: bool,
}
impl<'de> serde::Deserialize<'de> for SetUserVerifiedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetUserVerifiedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.setUserVerified/parameters/authenticatorId。
                    #[serde(rename = "authenticatorId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.command.setUserVerified/parameters/isUserVerified。
                    #[serde(rename = "isUserVerified")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub is_user_verified: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetUserVerifiedParams {
                    authenticator_id: fields.authenticator_id,
                    is_user_verified: fields.is_user_verified,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAuthn.command.setUserVerified。
///
///协议原文：
///
///<pre>Sets whether User Verification succeeds or fails for an authenticator.
///The default is true.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetUserVerified;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.command.setUserVerified/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetUserVerifiedResponse {}
impl<'de> serde::Deserialize<'de> for SetUserVerifiedResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetUserVerifiedResponse;
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
                Ok(SetUserVerifiedResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetUserVerified {
    type Params = SetUserVerifiedParams;
    type Response = SetUserVerifiedResponse;
    const METHOD: &'static str = "WebAuthn.setUserVerified";
}
