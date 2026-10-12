///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Security.type.CertificateErrorAction。
///
///协议原文：
///
///<pre>The action to take when a certificate error occurs. continue will continue processing the
///request and cancel will cancel the request.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CertificateErrorAction {
    ///协议值 "continue"。
    Continue,
    ///协议值 "cancel"。
    Cancel,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl CertificateErrorAction {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::security::types::CertificateErrorAction::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Continue => "continue",
            Self::Cancel => "cancel",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for CertificateErrorAction {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for CertificateErrorAction {
    fn from(value: String) -> Self {
        match value.as_str() {
            "continue" => CertificateErrorAction::Continue,
            "cancel" => CertificateErrorAction::Cancel,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for CertificateErrorAction {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CertificateErrorAction {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CertificateErrorAction;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "continue" => CertificateErrorAction::Continue,
                    "cancel" => CertificateErrorAction::Cancel,
                    _ => CertificateErrorAction::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(CertificateErrorAction::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Security.type.CertificateId。
///
///协议原文：
///
///<pre>An internal certificate ID value.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct CertificateId(i64);
impl CertificateId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::security::types::CertificateId = serde_json::from_str("0").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::security::types::CertificateId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("0").unwrap());
    ///```
    pub fn new(value: i64) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::security::types::CertificateId = serde_json::from_str("0").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::security::types::CertificateId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("0").unwrap());
    ///```
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Security.type.CertificateSecurityState。
///
///协议原文：
///
///<pre>Details about the security state of the page certificate.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CertificateSecurityState {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/protocol。
    ///
    ///协议原文：
    ///
    ///<pre>Protocol name (e.g. "TLS 1.2" or "QUIC").</pre>
    ///
    ///
    #[serde(rename = "protocol")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub protocol: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/keyExchange。
    ///
    ///协议原文：
    ///
    ///<pre>Key Exchange used by the connection, or the empty string if not applicable.</pre>
    ///
    ///
    #[serde(rename = "keyExchange")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub key_exchange: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Security.type.CertificateSecurityState/properties/keyExchangeGroup。
    ///
    ///协议原文：
    ///
    ///<pre>(EC)DH group used by the connection, if applicable.</pre>
    ///
    ///
    #[serde(rename = "keyExchangeGroup")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub key_exchange_group: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/cipher。
    ///
    ///协议原文：
    ///
    ///<pre>Cipher name.</pre>
    ///
    ///
    #[serde(rename = "cipher")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cipher: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Security.type.CertificateSecurityState/properties/mac。
    ///
    ///协议原文：
    ///
    ///<pre>TLS MAC. Note that AEAD ciphers do not have separate MACs.</pre>
    ///
    ///
    #[serde(rename = "mac")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub mac: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/certificate。
    ///
    ///协议原文：
    ///
    ///<pre>Page certificate.</pre>
    ///
    ///
    #[serde(rename = "certificate")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub certificate: Vec<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/subjectName。
    ///
    ///协议原文：
    ///
    ///<pre>Certificate subject name.</pre>
    ///
    ///
    #[serde(rename = "subjectName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub subject_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/issuer。
    ///
    ///协议原文：
    ///
    ///<pre>Name of the issuing CA.</pre>
    ///
    ///
    #[serde(rename = "issuer")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub issuer: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/validFrom。
    ///
    ///协议原文：
    ///
    ///<pre>Certificate valid from date.</pre>
    ///
    ///
    #[serde(rename = "validFrom")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub valid_from: crate::network::types::TimeSinceEpoch,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/validTo。
    ///
    ///协议原文：
    ///
    ///<pre>Certificate valid to (expiration) date</pre>
    ///
    ///
    #[serde(rename = "validTo")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub valid_to: crate::network::types::TimeSinceEpoch,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Security.type.CertificateSecurityState/properties/certificateNetworkError。
    ///
    ///协议原文：
    ///
    ///<pre>The highest priority network error code, if the certificate has an error.</pre>
    ///
    ///
    #[serde(rename = "certificateNetworkError")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub certificate_network_error: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/certificateHasWeakSignature。
    ///
    ///协议原文：
    ///
    ///<pre>True if the certificate uses a weak signature algorithm.</pre>
    ///
    ///
    #[serde(rename = "certificateHasWeakSignature")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub certificate_has_weak_signature: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/certificateHasSha1Signature。
    ///
    ///协议原文：
    ///
    ///<pre>True if the certificate has a SHA1 signature in the chain.</pre>
    ///
    ///
    #[serde(rename = "certificateHasSha1Signature")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub certificate_has_sha1_signature: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/modernSSL。
    ///
    ///协议原文：
    ///
    ///<pre>True if modern SSL</pre>
    ///
    ///
    #[serde(rename = "modernSSL")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub modern_ssl: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/obsoleteSslProtocol。
    ///
    ///协议原文：
    ///
    ///<pre>True if the connection is using an obsolete SSL protocol.</pre>
    ///
    ///
    #[serde(rename = "obsoleteSslProtocol")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub obsolete_ssl_protocol: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/obsoleteSslKeyExchange。
    ///
    ///协议原文：
    ///
    ///<pre>True if the connection is using an obsolete SSL key exchange.</pre>
    ///
    ///
    #[serde(rename = "obsoleteSslKeyExchange")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub obsolete_ssl_key_exchange: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/obsoleteSslCipher。
    ///
    ///协议原文：
    ///
    ///<pre>True if the connection is using an obsolete SSL cipher.</pre>
    ///
    ///
    #[serde(rename = "obsoleteSslCipher")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub obsolete_ssl_cipher: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/obsoleteSslSignature。
    ///
    ///协议原文：
    ///
    ///<pre>True if the connection is using an obsolete SSL signature.</pre>
    ///
    ///
    #[serde(rename = "obsoleteSslSignature")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub obsolete_ssl_signature: bool,
}
impl<'de> serde::Deserialize<'de> for CertificateSecurityState {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CertificateSecurityState;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/protocol。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Protocol name (e.g. "TLS 1.2" or "QUIC").</pre>
                    ///
                    ///
                    #[serde(rename = "protocol")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub protocol: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/keyExchange。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Key Exchange used by the connection, or the empty string if not applicable.</pre>
                    ///
                    ///
                    #[serde(rename = "keyExchange")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub key_exchange: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Security.type.CertificateSecurityState/properties/keyExchangeGroup。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>(EC)DH group used by the connection, if applicable.</pre>
                    ///
                    ///
                    #[serde(rename = "keyExchangeGroup")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub key_exchange_group: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/cipher。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cipher name.</pre>
                    ///
                    ///
                    #[serde(rename = "cipher")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cipher: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Security.type.CertificateSecurityState/properties/mac。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>TLS MAC. Note that AEAD ciphers do not have separate MACs.</pre>
                    ///
                    ///
                    #[serde(rename = "mac")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub mac: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/certificate。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Page certificate.</pre>
                    ///
                    ///
                    #[serde(rename = "certificate")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub certificate: Vec<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/subjectName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Certificate subject name.</pre>
                    ///
                    ///
                    #[serde(rename = "subjectName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub subject_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/issuer。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Name of the issuing CA.</pre>
                    ///
                    ///
                    #[serde(rename = "issuer")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub issuer: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/validFrom。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Certificate valid from date.</pre>
                    ///
                    ///
                    #[serde(rename = "validFrom")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub valid_from: crate::network::types::TimeSinceEpoch,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/validTo。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Certificate valid to (expiration) date</pre>
                    ///
                    ///
                    #[serde(rename = "validTo")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub valid_to: crate::network::types::TimeSinceEpoch,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Security.type.CertificateSecurityState/properties/certificateNetworkError。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The highest priority network error code, if the certificate has an error.</pre>
                    ///
                    ///
                    #[serde(rename = "certificateNetworkError")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub certificate_network_error: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/certificateHasWeakSignature。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if the certificate uses a weak signature algorithm.</pre>
                    ///
                    ///
                    #[serde(rename = "certificateHasWeakSignature")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub certificate_has_weak_signature: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/certificateHasSha1Signature。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if the certificate has a SHA1 signature in the chain.</pre>
                    ///
                    ///
                    #[serde(rename = "certificateHasSha1Signature")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub certificate_has_sha1_signature: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/modernSSL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if modern SSL</pre>
                    ///
                    ///
                    #[serde(rename = "modernSSL")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub modern_ssl: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/obsoleteSslProtocol。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if the connection is using an obsolete SSL protocol.</pre>
                    ///
                    ///
                    #[serde(rename = "obsoleteSslProtocol")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub obsolete_ssl_protocol: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/obsoleteSslKeyExchange。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if the connection is using an obsolete SSL key exchange.</pre>
                    ///
                    ///
                    #[serde(rename = "obsoleteSslKeyExchange")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub obsolete_ssl_key_exchange: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/obsoleteSslCipher。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if the connection is using an obsolete SSL cipher.</pre>
                    ///
                    ///
                    #[serde(rename = "obsoleteSslCipher")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub obsolete_ssl_cipher: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.CertificateSecurityState/properties/obsoleteSslSignature。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if the connection is using an obsolete SSL signature.</pre>
                    ///
                    ///
                    #[serde(rename = "obsoleteSslSignature")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub obsolete_ssl_signature: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CertificateSecurityState {
                    protocol: fields.protocol,
                    key_exchange: fields.key_exchange,
                    key_exchange_group: fields.key_exchange_group,
                    cipher: fields.cipher,
                    mac: fields.mac,
                    certificate: fields.certificate,
                    subject_name: fields.subject_name,
                    issuer: fields.issuer,
                    valid_from: fields.valid_from,
                    valid_to: fields.valid_to,
                    certificate_network_error: fields.certificate_network_error,
                    certificate_has_weak_signature: fields.certificate_has_weak_signature,
                    certificate_has_sha1_signature: fields.certificate_has_sha1_signature,
                    modern_ssl: fields.modern_ssl,
                    obsolete_ssl_protocol: fields.obsolete_ssl_protocol,
                    obsolete_ssl_key_exchange: fields.obsolete_ssl_key_exchange,
                    obsolete_ssl_cipher: fields.obsolete_ssl_cipher,
                    obsolete_ssl_signature: fields.obsolete_ssl_signature,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Security.type.InsecureContentStatus。
///
///协议原文：
///
///<pre>Information about insecure content on the page.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, serde::Serialize)]
pub struct InsecureContentStatus {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.InsecureContentStatus/properties/ranMixedContent。
    ///
    ///协议原文：
    ///
    ///<pre>Always false.</pre>
    ///
    ///
    #[serde(rename = "ranMixedContent")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub ran_mixed_content: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.InsecureContentStatus/properties/displayedMixedContent。
    ///
    ///协议原文：
    ///
    ///<pre>Always false.</pre>
    ///
    ///
    #[serde(rename = "displayedMixedContent")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub displayed_mixed_content: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.InsecureContentStatus/properties/containedMixedForm。
    ///
    ///协议原文：
    ///
    ///<pre>Always false.</pre>
    ///
    ///
    #[serde(rename = "containedMixedForm")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub contained_mixed_form: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.InsecureContentStatus/properties/ranContentWithCertErrors。
    ///
    ///协议原文：
    ///
    ///<pre>Always false.</pre>
    ///
    ///
    #[serde(rename = "ranContentWithCertErrors")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub ran_content_with_cert_errors: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.InsecureContentStatus/properties/displayedContentWithCertErrors。
    ///
    ///协议原文：
    ///
    ///<pre>Always false.</pre>
    ///
    ///
    #[serde(rename = "displayedContentWithCertErrors")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub displayed_content_with_cert_errors: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.InsecureContentStatus/properties/ranInsecureContentStyle。
    ///
    ///协议原文：
    ///
    ///<pre>Always set to unknown.</pre>
    ///
    ///
    #[serde(rename = "ranInsecureContentStyle")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub ran_insecure_content_style: crate::security::types::SecurityState,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.InsecureContentStatus/properties/displayedInsecureContentStyle。
    ///
    ///协议原文：
    ///
    ///<pre>Always set to unknown.</pre>
    ///
    ///
    #[serde(rename = "displayedInsecureContentStyle")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub displayed_insecure_content_style: crate::security::types::SecurityState,
}
impl<'de> serde::Deserialize<'de> for InsecureContentStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InsecureContentStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.InsecureContentStatus/properties/ranMixedContent。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Always false.</pre>
                    ///
                    ///
                    #[serde(rename = "ranMixedContent")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub ran_mixed_content: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.InsecureContentStatus/properties/displayedMixedContent。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Always false.</pre>
                    ///
                    ///
                    #[serde(rename = "displayedMixedContent")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub displayed_mixed_content: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.InsecureContentStatus/properties/containedMixedForm。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Always false.</pre>
                    ///
                    ///
                    #[serde(rename = "containedMixedForm")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub contained_mixed_form: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.InsecureContentStatus/properties/ranContentWithCertErrors。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Always false.</pre>
                    ///
                    ///
                    #[serde(rename = "ranContentWithCertErrors")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub ran_content_with_cert_errors: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.InsecureContentStatus/properties/displayedContentWithCertErrors。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Always false.</pre>
                    ///
                    ///
                    #[serde(rename = "displayedContentWithCertErrors")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub displayed_content_with_cert_errors: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.InsecureContentStatus/properties/ranInsecureContentStyle。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Always set to unknown.</pre>
                    ///
                    ///
                    #[serde(rename = "ranInsecureContentStyle")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub ran_insecure_content_style: crate::security::types::SecurityState,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.InsecureContentStatus/properties/displayedInsecureContentStyle。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Always set to unknown.</pre>
                    ///
                    ///
                    #[serde(rename = "displayedInsecureContentStyle")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub displayed_insecure_content_style: crate::security::types::SecurityState,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(InsecureContentStatus {
                    ran_mixed_content: fields.ran_mixed_content,
                    displayed_mixed_content: fields.displayed_mixed_content,
                    contained_mixed_form: fields.contained_mixed_form,
                    ran_content_with_cert_errors: fields.ran_content_with_cert_errors,
                    displayed_content_with_cert_errors: fields.displayed_content_with_cert_errors,
                    ran_insecure_content_style: fields.ran_insecure_content_style,
                    displayed_insecure_content_style: fields.displayed_insecure_content_style,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Security.type.MixedContentType。
///
///协议原文：
///
///<pre>A description of mixed content (HTTP resources on HTTPS pages), as defined by
///https://www.w3.org/TR/mixed-content/#categories</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MixedContentType {
    ///协议值 "blockable"。
    Blockable,
    ///协议值 "optionally-blockable"。
    OptionallyBlockable,
    ///协议值 "none"。
    None,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl MixedContentType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::security::types::MixedContentType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Blockable => "blockable",
            Self::OptionallyBlockable => "optionally-blockable",
            Self::None => "none",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for MixedContentType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for MixedContentType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "blockable" => MixedContentType::Blockable,
            "optionally-blockable" => MixedContentType::OptionallyBlockable,
            "none" => MixedContentType::None,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for MixedContentType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MixedContentType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MixedContentType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "blockable" => MixedContentType::Blockable,
                    "optionally-blockable" => MixedContentType::OptionallyBlockable,
                    "none" => MixedContentType::None,
                    _ => MixedContentType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(MixedContentType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Security.type.SafetyTipInfo。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SafetyTipInfo {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.SafetyTipInfo/properties/safetyTipStatus。
    ///
    ///协议原文：
    ///
    ///<pre>Describes whether the page triggers any safety tips or reputation warnings. Default is unknown.</pre>
    ///
    ///
    #[serde(rename = "safetyTipStatus")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub safety_tip_status: crate::security::types::SafetyTipStatus,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Security.type.SafetyTipInfo/properties/safeUrl。
    ///
    ///协议原文：
    ///
    ///<pre>The URL the safety tip suggested ("Did you mean?"). Only filled in for lookalike matches.</pre>
    ///
    ///
    #[serde(rename = "safeUrl")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub safe_url: Option<String>,
}
impl<'de> serde::Deserialize<'de> for SafetyTipInfo {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SafetyTipInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.SafetyTipInfo/properties/safetyTipStatus。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Describes whether the page triggers any safety tips or reputation warnings. Default is unknown.</pre>
                    ///
                    ///
                    #[serde(rename = "safetyTipStatus")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub safety_tip_status: crate::security::types::SafetyTipStatus,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Security.type.SafetyTipInfo/properties/safeUrl。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The URL the safety tip suggested ("Did you mean?"). Only filled in for lookalike matches.</pre>
                    ///
                    ///
                    #[serde(rename = "safeUrl")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub safe_url: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SafetyTipInfo {
                    safety_tip_status: fields.safety_tip_status,
                    safe_url: fields.safe_url,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Security.type.SafetyTipStatus。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SafetyTipStatus {
    ///协议值 "badReputation"。
    BadReputation,
    ///协议值 "lookalike"。
    Lookalike,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl SafetyTipStatus {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::security::types::SafetyTipStatus::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::BadReputation => "badReputation",
            Self::Lookalike => "lookalike",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for SafetyTipStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for SafetyTipStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "badReputation" => SafetyTipStatus::BadReputation,
            "lookalike" => SafetyTipStatus::Lookalike,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for SafetyTipStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SafetyTipStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SafetyTipStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "badReputation" => SafetyTipStatus::BadReputation,
                    "lookalike" => SafetyTipStatus::Lookalike,
                    _ => SafetyTipStatus::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(SafetyTipStatus::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Security.type.SecurityState。
///
///协议原文：
///
///<pre>The security level of a page or resource.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SecurityState {
    ///协议值 "unknown"。
    KnownUnknown,
    ///协议值 "neutral"。
    Neutral,
    ///协议值 "insecure"。
    Insecure,
    ///协议值 "secure"。
    Secure,
    ///协议值 "info"。
    Info,
    ///协议值 "insecure-broken"。
    InsecureBroken,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl SecurityState {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::security::types::SecurityState::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::KnownUnknown => "unknown",
            Self::Neutral => "neutral",
            Self::Insecure => "insecure",
            Self::Secure => "secure",
            Self::Info => "info",
            Self::InsecureBroken => "insecure-broken",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for SecurityState {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for SecurityState {
    fn from(value: String) -> Self {
        match value.as_str() {
            "unknown" => SecurityState::KnownUnknown,
            "neutral" => SecurityState::Neutral,
            "insecure" => SecurityState::Insecure,
            "secure" => SecurityState::Secure,
            "info" => SecurityState::Info,
            "insecure-broken" => SecurityState::InsecureBroken,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for SecurityState {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SecurityState {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SecurityState;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "unknown" => SecurityState::KnownUnknown,
                    "neutral" => SecurityState::Neutral,
                    "insecure" => SecurityState::Insecure,
                    "secure" => SecurityState::Secure,
                    "info" => SecurityState::Info,
                    "insecure-broken" => SecurityState::InsecureBroken,
                    _ => SecurityState::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(SecurityState::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Security.type.SecurityStateExplanation。
///
///协议原文：
///
///<pre>An explanation of an factor contributing to the security state.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct SecurityStateExplanation {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.SecurityStateExplanation/properties/securityState。
    ///
    ///协议原文：
    ///
    ///<pre>Security state representing the severity of the factor being explained.</pre>
    ///
    ///
    #[serde(rename = "securityState")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub security_state: crate::security::types::SecurityState,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.SecurityStateExplanation/properties/title。
    ///
    ///协议原文：
    ///
    ///<pre>Title describing the type of factor.</pre>
    ///
    ///
    #[serde(rename = "title")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub title: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.SecurityStateExplanation/properties/summary。
    ///
    ///协议原文：
    ///
    ///<pre>Short phrase describing the type of factor.</pre>
    ///
    ///
    #[serde(rename = "summary")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub summary: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.SecurityStateExplanation/properties/description。
    ///
    ///协议原文：
    ///
    ///<pre>Full text explanation of the factor.</pre>
    ///
    ///
    #[serde(rename = "description")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub description: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.SecurityStateExplanation/properties/mixedContentType。
    ///
    ///协议原文：
    ///
    ///<pre>The type of mixed content described by the explanation.</pre>
    ///
    ///
    #[serde(rename = "mixedContentType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub mixed_content_type: crate::security::types::MixedContentType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.SecurityStateExplanation/properties/certificate。
    ///
    ///协议原文：
    ///
    ///<pre>Page certificate.</pre>
    ///
    ///
    #[serde(rename = "certificate")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub certificate: Vec<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Security.type.SecurityStateExplanation/properties/recommendations。
    ///
    ///协议原文：
    ///
    ///<pre>Recommendations to fix any issues.</pre>
    ///
    ///
    #[serde(rename = "recommendations")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub recommendations: Option<Vec<String>>,
}
impl<'de> serde::Deserialize<'de> for SecurityStateExplanation {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SecurityStateExplanation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.SecurityStateExplanation/properties/securityState。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Security state representing the severity of the factor being explained.</pre>
                    ///
                    ///
                    #[serde(rename = "securityState")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub security_state: crate::security::types::SecurityState,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.SecurityStateExplanation/properties/title。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Title describing the type of factor.</pre>
                    ///
                    ///
                    #[serde(rename = "title")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub title: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.SecurityStateExplanation/properties/summary。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Short phrase describing the type of factor.</pre>
                    ///
                    ///
                    #[serde(rename = "summary")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub summary: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.SecurityStateExplanation/properties/description。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Full text explanation of the factor.</pre>
                    ///
                    ///
                    #[serde(rename = "description")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub description: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.SecurityStateExplanation/properties/mixedContentType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The type of mixed content described by the explanation.</pre>
                    ///
                    ///
                    #[serde(rename = "mixedContentType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub mixed_content_type: crate::security::types::MixedContentType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.SecurityStateExplanation/properties/certificate。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Page certificate.</pre>
                    ///
                    ///
                    #[serde(rename = "certificate")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub certificate: Vec<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Security.type.SecurityStateExplanation/properties/recommendations。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Recommendations to fix any issues.</pre>
                    ///
                    ///
                    #[serde(rename = "recommendations")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub recommendations: Option<Vec<String>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SecurityStateExplanation {
                    security_state: fields.security_state,
                    title: fields.title,
                    summary: fields.summary,
                    description: fields.description,
                    mixed_content_type: fields.mixed_content_type,
                    certificate: fields.certificate,
                    recommendations: fields.recommendations,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Security.type.VisibleSecurityState。
///
///协议原文：
///
///<pre>Security state information about the page.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct VisibleSecurityState {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.VisibleSecurityState/properties/securityState。
    ///
    ///协议原文：
    ///
    ///<pre>The security level of the page.</pre>
    ///
    ///
    #[serde(rename = "securityState")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub security_state: crate::security::types::SecurityState,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Security.type.VisibleSecurityState/properties/certificateSecurityState。
    ///
    ///协议原文：
    ///
    ///<pre>Security state details about the page certificate.</pre>
    ///
    ///
    #[serde(rename = "certificateSecurityState")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub certificate_security_state: Option<crate::security::types::CertificateSecurityState>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Security.type.VisibleSecurityState/properties/safetyTipInfo。
    ///
    ///协议原文：
    ///
    ///<pre>The type of Safety Tip triggered on the page. Note that this field will be set even if the Safety Tip UI was not actually shown.</pre>
    ///
    ///
    #[serde(rename = "safetyTipInfo")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub safety_tip_info: Option<crate::security::types::SafetyTipInfo>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.VisibleSecurityState/properties/securityStateIssueIds。
    ///
    ///协议原文：
    ///
    ///<pre>Array of security state issues ids.</pre>
    ///
    ///
    #[serde(rename = "securityStateIssueIds")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub security_state_issue_ids: Vec<String>,
}
impl<'de> serde::Deserialize<'de> for VisibleSecurityState {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = VisibleSecurityState;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.VisibleSecurityState/properties/securityState。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The security level of the page.</pre>
                    ///
                    ///
                    #[serde(rename = "securityState")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub security_state: crate::security::types::SecurityState,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Security.type.VisibleSecurityState/properties/certificateSecurityState。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Security state details about the page certificate.</pre>
                    ///
                    ///
                    #[serde(rename = "certificateSecurityState")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub certificate_security_state:
                        Option<crate::security::types::CertificateSecurityState>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Security.type.VisibleSecurityState/properties/safetyTipInfo。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The type of Safety Tip triggered on the page. Note that this field will be set even if the Safety Tip UI was not actually shown.</pre>
                    ///
                    ///
                    #[serde(rename = "safetyTipInfo")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub safety_tip_info: Option<crate::security::types::SafetyTipInfo>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.type.VisibleSecurityState/properties/securityStateIssueIds。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Array of security state issues ids.</pre>
                    ///
                    ///
                    #[serde(rename = "securityStateIssueIds")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub security_state_issue_ids: Vec<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(VisibleSecurityState {
                    security_state: fields.security_state,
                    certificate_security_state: fields.certificate_security_state,
                    safety_tip_info: fields.safety_tip_info,
                    security_state_issue_ids: fields.security_state_issue_ids,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
