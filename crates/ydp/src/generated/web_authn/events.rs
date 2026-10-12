///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.event.credentialAdded/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CredentialAddedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.event.credentialAdded/parameters/authenticatorId。
    #[serde(rename = "authenticatorId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.event.credentialAdded/parameters/credential。
    #[serde(rename = "credential")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub credential: crate::web_authn::types::Credential,
}
impl<'de> serde::Deserialize<'de> for CredentialAddedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CredentialAddedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.event.credentialAdded/parameters/authenticatorId。
                    #[serde(rename = "authenticatorId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.event.credentialAdded/parameters/credential。
                    #[serde(rename = "credential")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub credential: crate::web_authn::types::Credential,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CredentialAddedParams {
                    authenticator_id: fields.authenticator_id,
                    credential: fields.credential,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAuthn.event.credentialAdded。
///
///协议原文：
///
///<pre>Triggered when a credential is added to an authenticator.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct CredentialAdded;
impl ydp_core::Event for CredentialAdded {
    type Params = CredentialAddedParams;
    const METHOD: &'static str = "WebAuthn.credentialAdded";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.event.credentialAsserted/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CredentialAssertedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.event.credentialAsserted/parameters/authenticatorId。
    #[serde(rename = "authenticatorId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.event.credentialAsserted/parameters/credential。
    #[serde(rename = "credential")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub credential: crate::web_authn::types::Credential,
}
impl<'de> serde::Deserialize<'de> for CredentialAssertedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CredentialAssertedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.event.credentialAsserted/parameters/authenticatorId。
                    #[serde(rename = "authenticatorId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.event.credentialAsserted/parameters/credential。
                    #[serde(rename = "credential")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub credential: crate::web_authn::types::Credential,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CredentialAssertedParams {
                    authenticator_id: fields.authenticator_id,
                    credential: fields.credential,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAuthn.event.credentialAsserted。
///
///协议原文：
///
///<pre>Triggered when a credential is used in a webauthn assertion.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct CredentialAsserted;
impl ydp_core::Event for CredentialAsserted {
    type Params = CredentialAssertedParams;
    const METHOD: &'static str = "WebAuthn.credentialAsserted";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.event.credentialDeleted/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CredentialDeletedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.event.credentialDeleted/parameters/authenticatorId。
    #[serde(rename = "authenticatorId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.event.credentialDeleted/parameters/credentialId。
    #[serde(rename = "credentialId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub credential_id: String,
}
impl<'de> serde::Deserialize<'de> for CredentialDeletedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CredentialDeletedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.event.credentialDeleted/parameters/authenticatorId。
                    #[serde(rename = "authenticatorId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.event.credentialDeleted/parameters/credentialId。
                    #[serde(rename = "credentialId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub credential_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CredentialDeletedParams {
                    authenticator_id: fields.authenticator_id,
                    credential_id: fields.credential_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAuthn.event.credentialDeleted。
///
///协议原文：
///
///<pre>Triggered when a credential is deleted, e.g. through
///PublicKeyCredential.signalUnknownCredential().</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct CredentialDeleted;
impl ydp_core::Event for CredentialDeleted {
    type Params = CredentialDeletedParams;
    const METHOD: &'static str = "WebAuthn.credentialDeleted";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.event.credentialUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CredentialUpdatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.event.credentialUpdated/parameters/authenticatorId。
    #[serde(rename = "authenticatorId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.event.credentialUpdated/parameters/credential。
    #[serde(rename = "credential")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub credential: crate::web_authn::types::Credential,
}
impl<'de> serde::Deserialize<'de> for CredentialUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CredentialUpdatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.event.credentialUpdated/parameters/authenticatorId。
                    #[serde(rename = "authenticatorId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub authenticator_id: crate::web_authn::types::AuthenticatorId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.event.credentialUpdated/parameters/credential。
                    #[serde(rename = "credential")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub credential: crate::web_authn::types::Credential,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CredentialUpdatedParams {
                    authenticator_id: fields.authenticator_id,
                    credential: fields.credential,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebAuthn.event.credentialUpdated。
///
///协议原文：
///
///<pre>Triggered when a credential is updated, e.g. through
///PublicKeyCredential.signalCurrentUserDetails().</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct CredentialUpdated;
impl ydp_core::Event for CredentialUpdated {
    type Params = CredentialUpdatedParams;
    const METHOD: &'static str = "WebAuthn.credentialUpdated";
}
