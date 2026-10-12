///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：WebAuthn.type.AuthenticatorId。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct AuthenticatorId(String);
impl AuthenticatorId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::web_authn::types::AuthenticatorId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::web_authn::types::AuthenticatorId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::web_authn::types::AuthenticatorId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::web_authn::types::AuthenticatorId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::web_authn::types::AuthenticatorId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：WebAuthn.type.AuthenticatorProtocol。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AuthenticatorProtocol {
    ///协议值 "u2f"。
    U2f,
    ///协议值 "ctap2"。
    Ctap2,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl AuthenticatorProtocol {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::web_authn::types::AuthenticatorProtocol::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::U2f => "u2f",
            Self::Ctap2 => "ctap2",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for AuthenticatorProtocol {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for AuthenticatorProtocol {
    fn from(value: String) -> Self {
        match value.as_str() {
            "u2f" => AuthenticatorProtocol::U2f,
            "ctap2" => AuthenticatorProtocol::Ctap2,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for AuthenticatorProtocol {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AuthenticatorProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AuthenticatorProtocol;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "u2f" => AuthenticatorProtocol::U2f,
                    "ctap2" => AuthenticatorProtocol::Ctap2,
                    _ => AuthenticatorProtocol::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(AuthenticatorProtocol::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：WebAuthn.type.AuthenticatorTransport。
///
///协议原文：
///
///<pre>LINT.IfChange(AuthenticatorTransport)</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AuthenticatorTransport {
    ///协议值 "usb"。
    Usb,
    ///协议值 "nfc"。
    Nfc,
    ///协议值 "ble"。
    Ble,
    ///协议值 "cable"。
    Cable,
    ///协议值 "hybrid"。
    Hybrid,
    ///协议值 "smart-card"。
    SmartCard,
    ///协议值 "internal"。
    Internal,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl AuthenticatorTransport {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::web_authn::types::AuthenticatorTransport::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Usb => "usb",
            Self::Nfc => "nfc",
            Self::Ble => "ble",
            Self::Cable => "cable",
            Self::Hybrid => "hybrid",
            Self::SmartCard => "smart-card",
            Self::Internal => "internal",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for AuthenticatorTransport {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for AuthenticatorTransport {
    fn from(value: String) -> Self {
        match value.as_str() {
            "usb" => AuthenticatorTransport::Usb,
            "nfc" => AuthenticatorTransport::Nfc,
            "ble" => AuthenticatorTransport::Ble,
            "cable" => AuthenticatorTransport::Cable,
            "hybrid" => AuthenticatorTransport::Hybrid,
            "smart-card" => AuthenticatorTransport::SmartCard,
            "internal" => AuthenticatorTransport::Internal,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for AuthenticatorTransport {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AuthenticatorTransport {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AuthenticatorTransport;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "usb" => AuthenticatorTransport::Usb,
                    "nfc" => AuthenticatorTransport::Nfc,
                    "ble" => AuthenticatorTransport::Ble,
                    "cable" => AuthenticatorTransport::Cable,
                    "hybrid" => AuthenticatorTransport::Hybrid,
                    "smart-card" => AuthenticatorTransport::SmartCard,
                    "internal" => AuthenticatorTransport::Internal,
                    _ => AuthenticatorTransport::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(AuthenticatorTransport::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.type.Credential。
#[derive(Debug, Clone, serde::Serialize)]
pub struct Credential {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.type.Credential/properties/credentialId。
    #[serde(rename = "credentialId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub credential_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.type.Credential/properties/isResidentCredential。
    #[serde(rename = "isResidentCredential")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub is_resident_credential: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.Credential/properties/rpId。
    ///
    ///协议原文：
    ///
    ///<pre>Relying Party ID the credential is scoped to. Must be set when adding a
    ///credential.</pre>
    ///
    ///
    #[serde(rename = "rpId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub rp_id: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.type.Credential/properties/privateKey。
    ///
    ///协议原文：
    ///
    ///<pre>The ECDSA P-256 private key in PKCS#8 format. (Encoded as a base64 string when passed over JSON)</pre>
    ///
    ///
    #[serde(rename = "privateKey")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub private_key: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.Credential/properties/userHandle。
    ///
    ///协议原文：
    ///
    ///<pre>An opaque byte sequence with a maximum size of 64 bytes mapping the
    ///credential to a specific user. (Encoded as a base64 string when passed over JSON)</pre>
    ///
    ///
    #[serde(rename = "userHandle")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_handle: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.type.Credential/properties/signCount。
    ///
    ///协议原文：
    ///
    ///<pre>Signature counter. Must be equal to or greater than -1.
    ///If -1, the credential won't have an associated signature counter, and
    ///every assertion operation will report a value of 0.
    ///See https://w3c.github.io/webauthn/#signature-counter</pre>
    ///
    ///
    #[serde(rename = "signCount")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub sign_count: ydp_core::FiniteF64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.Credential/properties/largeBlob。
    ///
    ///协议原文：
    ///
    ///<pre>The large blob associated with the credential.
    ///See https://w3c.github.io/webauthn/#sctn-large-blob-extension (Encoded as a base64 string when passed over JSON)</pre>
    ///
    ///
    #[serde(rename = "largeBlob")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub large_blob: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.Credential/properties/backupEligibility。
    ///
    ///协议原文：
    ///
    ///<pre>Assertions returned by this credential will have the backup eligibility
    ///(BE) flag set to this value. Defaults to the authenticator's
    ///defaultBackupEligibility value.</pre>
    ///
    ///
    #[serde(rename = "backupEligibility")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub backup_eligibility: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.Credential/properties/backupState。
    ///
    ///协议原文：
    ///
    ///<pre>Assertions returned by this credential will have the backup state (BS)
    ///flag set to this value. Defaults to the authenticator's
    ///defaultBackupState value.</pre>
    ///
    ///
    #[serde(rename = "backupState")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub backup_state: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.Credential/properties/userName。
    ///
    ///协议原文：
    ///
    ///<pre>The credential's user.name property. Equivalent to empty if not set.
    ///https://w3c.github.io/webauthn/#dom-publickeycredentialentity-name</pre>
    ///
    ///
    #[serde(rename = "userName")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_name: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.Credential/properties/userDisplayName。
    ///
    ///协议原文：
    ///
    ///<pre>The credential's user.displayName property. Equivalent to empty if
    ///not set.
    ///https://w3c.github.io/webauthn/#dom-publickeycredentialuserentity-displayname</pre>
    ///
    ///
    #[serde(rename = "userDisplayName")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_display_name: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.Credential/properties/cmtgKeys。
    ///
    ///协议原文：
    ///
    ///<pre>The CMTG keys associated with the credential.</pre>
    ///
    ///
    #[serde(rename = "cmtgKeys")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub cmtg_keys: Option<Vec<String>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.Credential/properties/activeCmtgKeyIndex。
    ///
    ///协议原文：
    ///
    ///<pre>The 0-based index of the active key in cmtgKeys.</pre>
    ///
    ///
    #[serde(rename = "activeCmtgKeyIndex")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub active_cmtg_key_index: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.Credential/properties/generateCmtgKeyOnNextOperation。
    ///
    ///协议原文：
    ///
    ///<pre>If true, the authenticator will generate a new CMTG key on the next operation.</pre>
    ///
    ///
    #[serde(rename = "generateCmtgKeyOnNextOperation")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub generate_cmtg_key_on_next_operation: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for Credential {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Credential;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.type.Credential/properties/credentialId。
                    #[serde(rename = "credentialId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub credential_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.type.Credential/properties/isResidentCredential。
                    #[serde(rename = "isResidentCredential")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub is_resident_credential: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.Credential/properties/rpId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Relying Party ID the credential is scoped to. Must be set when adding a
                    ///credential.</pre>
                    ///
                    ///
                    #[serde(rename = "rpId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub rp_id: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.type.Credential/properties/privateKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The ECDSA P-256 private key in PKCS#8 format. (Encoded as a base64 string when passed over JSON)</pre>
                    ///
                    ///
                    #[serde(rename = "privateKey")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub private_key: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.Credential/properties/userHandle。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An opaque byte sequence with a maximum size of 64 bytes mapping the
                    ///credential to a specific user. (Encoded as a base64 string when passed over JSON)</pre>
                    ///
                    ///
                    #[serde(rename = "userHandle")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub user_handle: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.type.Credential/properties/signCount。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Signature counter. Must be equal to or greater than -1.
                    ///If -1, the credential won't have an associated signature counter, and
                    ///every assertion operation will report a value of 0.
                    ///See https://w3c.github.io/webauthn/#signature-counter</pre>
                    ///
                    ///
                    #[serde(rename = "signCount")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub sign_count: ydp_core::FiniteF64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.Credential/properties/largeBlob。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The large blob associated with the credential.
                    ///See https://w3c.github.io/webauthn/#sctn-large-blob-extension (Encoded as a base64 string when passed over JSON)</pre>
                    ///
                    ///
                    #[serde(rename = "largeBlob")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub large_blob: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.Credential/properties/backupEligibility。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Assertions returned by this credential will have the backup eligibility
                    ///(BE) flag set to this value. Defaults to the authenticator's
                    ///defaultBackupEligibility value.</pre>
                    ///
                    ///
                    #[serde(rename = "backupEligibility")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub backup_eligibility: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.Credential/properties/backupState。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Assertions returned by this credential will have the backup state (BS)
                    ///flag set to this value. Defaults to the authenticator's
                    ///defaultBackupState value.</pre>
                    ///
                    ///
                    #[serde(rename = "backupState")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub backup_state: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.Credential/properties/userName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The credential's user.name property. Equivalent to empty if not set.
                    ///https://w3c.github.io/webauthn/#dom-publickeycredentialentity-name</pre>
                    ///
                    ///
                    #[serde(rename = "userName")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub user_name: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.Credential/properties/userDisplayName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The credential's user.displayName property. Equivalent to empty if
                    ///not set.
                    ///https://w3c.github.io/webauthn/#dom-publickeycredentialuserentity-displayname</pre>
                    ///
                    ///
                    #[serde(rename = "userDisplayName")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub user_display_name: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.Credential/properties/cmtgKeys。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The CMTG keys associated with the credential.</pre>
                    ///
                    ///
                    #[serde(rename = "cmtgKeys")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub cmtg_keys: Option<Vec<String>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.Credential/properties/activeCmtgKeyIndex。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The 0-based index of the active key in cmtgKeys.</pre>
                    ///
                    ///
                    #[serde(rename = "activeCmtgKeyIndex")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub active_cmtg_key_index: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.Credential/properties/generateCmtgKeyOnNextOperation。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, the authenticator will generate a new CMTG key on the next operation.</pre>
                    ///
                    ///
                    #[serde(rename = "generateCmtgKeyOnNextOperation")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub generate_cmtg_key_on_next_operation: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Credential {
                    credential_id: fields.credential_id,
                    is_resident_credential: fields.is_resident_credential,
                    rp_id: fields.rp_id,
                    private_key: fields.private_key,
                    user_handle: fields.user_handle,
                    sign_count: fields.sign_count,
                    large_blob: fields.large_blob,
                    backup_eligibility: fields.backup_eligibility,
                    backup_state: fields.backup_state,
                    user_name: fields.user_name,
                    user_display_name: fields.user_display_name,
                    cmtg_keys: fields.cmtg_keys,
                    active_cmtg_key_index: fields.active_cmtg_key_index,
                    generate_cmtg_key_on_next_operation: fields.generate_cmtg_key_on_next_operation,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：WebAuthn.type.Ctap2Version。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Ctap2Version {
    ///协议值 "ctap2_0"。
    Ctap20,
    ///协议值 "ctap2_1"。
    Ctap21,
    ///协议值 "ctap2_2"。
    Ctap22,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl Ctap2Version {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::web_authn::types::Ctap2Version::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ctap20 => "ctap2_0",
            Self::Ctap21 => "ctap2_1",
            Self::Ctap22 => "ctap2_2",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for Ctap2Version {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for Ctap2Version {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ctap2_0" => Ctap2Version::Ctap20,
            "ctap2_1" => Ctap2Version::Ctap21,
            "ctap2_2" => Ctap2Version::Ctap22,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for Ctap2Version {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for Ctap2Version {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Ctap2Version;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "ctap2_0" => Ctap2Version::Ctap20,
                    "ctap2_1" => Ctap2Version::Ctap21,
                    "ctap2_2" => Ctap2Version::Ctap22,
                    _ => Ctap2Version::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(Ctap2Version::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebAuthn.type.VirtualAuthenticatorOptions。
#[derive(Debug, Clone, serde::Serialize)]
pub struct VirtualAuthenticatorOptions {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/protocol。
    #[serde(rename = "protocol")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub protocol: crate::web_authn::types::AuthenticatorProtocol,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/ctap2Version。
    ///
    ///协议原文：
    ///
    ///<pre>Defaults to ctap2_0. Ignored if |protocol| == u2f.</pre>
    ///
    ///
    #[serde(rename = "ctap2Version")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub ctap2_version: Option<crate::web_authn::types::Ctap2Version>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/transport。
    #[serde(rename = "transport")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub transport: crate::web_authn::types::AuthenticatorTransport,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/hasResidentKey。
    ///
    ///协议原文：
    ///
    ///<pre>Defaults to false.</pre>
    ///
    ///
    #[serde(rename = "hasResidentKey")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_resident_key: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/hasUserVerification。
    ///
    ///协议原文：
    ///
    ///<pre>Defaults to false.</pre>
    ///
    ///
    #[serde(rename = "hasUserVerification")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_user_verification: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/hasLargeBlob。
    ///
    ///协议原文：
    ///
    ///<pre>If set to true, the authenticator will support the largeBlob extension.
    ///https://w3c.github.io/webauthn#largeBlob
    ///Defaults to false.</pre>
    ///
    ///
    #[serde(rename = "hasLargeBlob")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_large_blob: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/hasCredBlob。
    ///
    ///协议原文：
    ///
    ///<pre>If set to true, the authenticator will support the credBlob extension.
    ///https://fidoalliance.org/specs/fido-v2.1-rd-20201208/fido-client-to-authenticator-protocol-v2.1-rd-20201208.html#sctn-credBlob-extension
    ///Defaults to false.</pre>
    ///
    ///
    #[serde(rename = "hasCredBlob")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_cred_blob: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/hasMinPinLength。
    ///
    ///协议原文：
    ///
    ///<pre>If set to true, the authenticator will support the minPinLength extension.
    ///https://fidoalliance.org/specs/fido-v2.1-ps-20210615/fido-client-to-authenticator-protocol-v2.1-ps-20210615.html#sctn-minpinlength-extension
    ///Defaults to false.</pre>
    ///
    ///
    #[serde(rename = "hasMinPinLength")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_min_pin_length: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/hasPrf。
    ///
    ///协议原文：
    ///
    ///<pre>If set to true, the authenticator will support the prf extension.
    ///https://w3c.github.io/webauthn/#prf-extension
    ///Defaults to false.</pre>
    ///
    ///
    #[serde(rename = "hasPrf")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_prf: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/hasHmacSecret。
    ///
    ///协议原文：
    ///
    ///<pre>If set to true, the authenticator will support the hmac-secret extension.
    ///https://fidoalliance.org/specs/fido-v2.1-ps-20210615/fido-client-to-authenticator-protocol-v2.1-ps-20210615.html#sctn-hmac-secret-extension
    ///Defaults to false.</pre>
    ///
    ///
    #[serde(rename = "hasHmacSecret")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_hmac_secret: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/hasHmacSecretMc。
    ///
    ///协议原文：
    ///
    ///<pre>If set to true, the authenticator will support the hmac-secret-mc extension.
    ///https://fidoalliance.org/specs/fido-v2.2-rd-20241003/fido-client-to-authenticator-protocol-v2.2-rd-20241003.html#sctn-hmac-secret-make-cred-extension
    ///Defaults to false.</pre>
    ///
    ///
    #[serde(rename = "hasHmacSecretMc")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_hmac_secret_mc: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/hasCmtgKey。
    ///
    ///协议原文：
    ///
    ///<pre>If set to true, the authenticator will support the cmtgKey (Credential
    ///Manager Trust Group Key) extension.
    ///https://github.com/w3c/webauthn/pull/2377
    ///Defaults to false.</pre>
    ///
    ///
    #[serde(rename = "hasCmtgKey")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_cmtg_key: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/automaticPresenceSimulation。
    ///
    ///协议原文：
    ///
    ///<pre>If set to true, tests of user presence will succeed immediately.
    ///Otherwise, they will not be resolved. Defaults to true.</pre>
    ///
    ///
    #[serde(rename = "automaticPresenceSimulation")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub automatic_presence_simulation: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/isUserVerified。
    ///
    ///协议原文：
    ///
    ///<pre>Sets whether User Verification succeeds or fails for an authenticator.
    ///Defaults to false.</pre>
    ///
    ///
    #[serde(rename = "isUserVerified")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_user_verified: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/defaultBackupEligibility。
    ///
    ///协议原文：
    ///
    ///<pre>Credentials created by this authenticator will have the backup
    ///eligibility (BE) flag set to this value. Defaults to false.
    ///https://w3c.github.io/webauthn/#sctn-credential-backup</pre>
    ///
    ///
    #[serde(rename = "defaultBackupEligibility")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_backup_eligibility: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/defaultBackupState。
    ///
    ///协议原文：
    ///
    ///<pre>Credentials created by this authenticator will have the backup state
    ///(BS) flag set to this value. Defaults to false.
    ///https://w3c.github.io/webauthn/#sctn-credential-backup</pre>
    ///
    ///
    #[serde(rename = "defaultBackupState")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_backup_state: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for VirtualAuthenticatorOptions {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = VirtualAuthenticatorOptions;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/protocol。
                    #[serde(rename = "protocol")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub protocol: crate::web_authn::types::AuthenticatorProtocol,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/ctap2Version。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Defaults to ctap2_0. Ignored if |protocol| == u2f.</pre>
                    ///
                    ///
                    #[serde(rename = "ctap2Version")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub ctap2_version: Option<crate::web_authn::types::Ctap2Version>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/transport。
                    #[serde(rename = "transport")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub transport: crate::web_authn::types::AuthenticatorTransport,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/hasResidentKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Defaults to false.</pre>
                    ///
                    ///
                    #[serde(rename = "hasResidentKey")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub has_resident_key: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/hasUserVerification。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Defaults to false.</pre>
                    ///
                    ///
                    #[serde(rename = "hasUserVerification")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub has_user_verification: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/hasLargeBlob。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If set to true, the authenticator will support the largeBlob extension.
                    ///https://w3c.github.io/webauthn#largeBlob
                    ///Defaults to false.</pre>
                    ///
                    ///
                    #[serde(rename = "hasLargeBlob")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub has_large_blob: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/hasCredBlob。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If set to true, the authenticator will support the credBlob extension.
                    ///https://fidoalliance.org/specs/fido-v2.1-rd-20201208/fido-client-to-authenticator-protocol-v2.1-rd-20201208.html#sctn-credBlob-extension
                    ///Defaults to false.</pre>
                    ///
                    ///
                    #[serde(rename = "hasCredBlob")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub has_cred_blob: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/hasMinPinLength。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If set to true, the authenticator will support the minPinLength extension.
                    ///https://fidoalliance.org/specs/fido-v2.1-ps-20210615/fido-client-to-authenticator-protocol-v2.1-ps-20210615.html#sctn-minpinlength-extension
                    ///Defaults to false.</pre>
                    ///
                    ///
                    #[serde(rename = "hasMinPinLength")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub has_min_pin_length: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/hasPrf。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If set to true, the authenticator will support the prf extension.
                    ///https://w3c.github.io/webauthn/#prf-extension
                    ///Defaults to false.</pre>
                    ///
                    ///
                    #[serde(rename = "hasPrf")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub has_prf: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/hasHmacSecret。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If set to true, the authenticator will support the hmac-secret extension.
                    ///https://fidoalliance.org/specs/fido-v2.1-ps-20210615/fido-client-to-authenticator-protocol-v2.1-ps-20210615.html#sctn-hmac-secret-extension
                    ///Defaults to false.</pre>
                    ///
                    ///
                    #[serde(rename = "hasHmacSecret")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub has_hmac_secret: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/hasHmacSecretMc。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If set to true, the authenticator will support the hmac-secret-mc extension.
                    ///https://fidoalliance.org/specs/fido-v2.2-rd-20241003/fido-client-to-authenticator-protocol-v2.2-rd-20241003.html#sctn-hmac-secret-make-cred-extension
                    ///Defaults to false.</pre>
                    ///
                    ///
                    #[serde(rename = "hasHmacSecretMc")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub has_hmac_secret_mc: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/hasCmtgKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If set to true, the authenticator will support the cmtgKey (Credential
                    ///Manager Trust Group Key) extension.
                    ///https://github.com/w3c/webauthn/pull/2377
                    ///Defaults to false.</pre>
                    ///
                    ///
                    #[serde(rename = "hasCmtgKey")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub has_cmtg_key: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/automaticPresenceSimulation。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If set to true, tests of user presence will succeed immediately.
                    ///Otherwise, they will not be resolved. Defaults to true.</pre>
                    ///
                    ///
                    #[serde(rename = "automaticPresenceSimulation")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub automatic_presence_simulation: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/isUserVerified。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Sets whether User Verification succeeds or fails for an authenticator.
                    ///Defaults to false.</pre>
                    ///
                    ///
                    #[serde(rename = "isUserVerified")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub is_user_verified: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/defaultBackupEligibility。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Credentials created by this authenticator will have the backup
                    ///eligibility (BE) flag set to this value. Defaults to false.
                    ///https://w3c.github.io/webauthn/#sctn-credential-backup</pre>
                    ///
                    ///
                    #[serde(rename = "defaultBackupEligibility")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub default_backup_eligibility: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebAuthn.type.VirtualAuthenticatorOptions/properties/defaultBackupState。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Credentials created by this authenticator will have the backup state
                    ///(BS) flag set to this value. Defaults to false.
                    ///https://w3c.github.io/webauthn/#sctn-credential-backup</pre>
                    ///
                    ///
                    #[serde(rename = "defaultBackupState")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub default_backup_state: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(VirtualAuthenticatorOptions {
                    protocol: fields.protocol,
                    ctap2_version: fields.ctap2_version,
                    transport: fields.transport,
                    has_resident_key: fields.has_resident_key,
                    has_user_verification: fields.has_user_verification,
                    has_large_blob: fields.has_large_blob,
                    has_cred_blob: fields.has_cred_blob,
                    has_min_pin_length: fields.has_min_pin_length,
                    has_prf: fields.has_prf,
                    has_hmac_secret: fields.has_hmac_secret,
                    has_hmac_secret_mc: fields.has_hmac_secret_mc,
                    has_cmtg_key: fields.has_cmtg_key,
                    automatic_presence_simulation: fields.automatic_presence_simulation,
                    is_user_verified: fields.is_user_verified,
                    default_backup_eligibility: fields.default_backup_eligibility,
                    default_backup_state: fields.default_backup_state,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
