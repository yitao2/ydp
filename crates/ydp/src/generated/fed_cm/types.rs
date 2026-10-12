///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：FedCm.type.Account。
///
///协议原文：
///
///<pre>Corresponds to IdentityRequestAccount</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct Account {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.type.Account/properties/accountId。
    #[serde(rename = "accountId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub account_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.type.Account/properties/email。
    #[serde(rename = "email")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub email: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.type.Account/properties/name。
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.type.Account/properties/givenName。
    #[serde(rename = "givenName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub given_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.type.Account/properties/pictureUrl。
    #[serde(rename = "pictureUrl")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub picture_url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.type.Account/properties/idpConfigUrl。
    #[serde(rename = "idpConfigUrl")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub idp_config_url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.type.Account/properties/idpLoginUrl。
    #[serde(rename = "idpLoginUrl")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub idp_login_url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.type.Account/properties/loginState。
    #[serde(rename = "loginState")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub login_state: crate::fed_cm::types::LoginState,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：FedCm.type.Account/properties/termsOfServiceUrl。
    ///
    ///协议原文：
    ///
    ///<pre>These two are only set if the loginState is signUp</pre>
    ///
    ///
    #[serde(rename = "termsOfServiceUrl")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub terms_of_service_url: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：FedCm.type.Account/properties/privacyPolicyUrl。
    #[serde(rename = "privacyPolicyUrl")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub privacy_policy_url: Option<String>,
}
impl<'de> serde::Deserialize<'de> for Account {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Account;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.type.Account/properties/accountId。
                    #[serde(rename = "accountId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub account_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.type.Account/properties/email。
                    #[serde(rename = "email")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub email: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.type.Account/properties/name。
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.type.Account/properties/givenName。
                    #[serde(rename = "givenName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub given_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.type.Account/properties/pictureUrl。
                    #[serde(rename = "pictureUrl")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub picture_url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.type.Account/properties/idpConfigUrl。
                    #[serde(rename = "idpConfigUrl")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub idp_config_url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.type.Account/properties/idpLoginUrl。
                    #[serde(rename = "idpLoginUrl")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub idp_login_url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.type.Account/properties/loginState。
                    #[serde(rename = "loginState")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub login_state: crate::fed_cm::types::LoginState,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：FedCm.type.Account/properties/termsOfServiceUrl。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>These two are only set if the loginState is signUp</pre>
                    ///
                    ///
                    #[serde(rename = "termsOfServiceUrl")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub terms_of_service_url: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：FedCm.type.Account/properties/privacyPolicyUrl。
                    #[serde(rename = "privacyPolicyUrl")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub privacy_policy_url: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Account {
                    account_id: fields.account_id,
                    email: fields.email,
                    name: fields.name,
                    given_name: fields.given_name,
                    picture_url: fields.picture_url,
                    idp_config_url: fields.idp_config_url,
                    idp_login_url: fields.idp_login_url,
                    login_state: fields.login_state,
                    terms_of_service_url: fields.terms_of_service_url,
                    privacy_policy_url: fields.privacy_policy_url,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：FedCm.type.AccountUrlType。
///
///协议原文：
///
///<pre>The URLs that each account has</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AccountUrlType {
    ///协议值 "TermsOfService"。
    TermsOfService,
    ///协议值 "PrivacyPolicy"。
    PrivacyPolicy,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl AccountUrlType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::fed_cm::types::AccountUrlType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::TermsOfService => "TermsOfService",
            Self::PrivacyPolicy => "PrivacyPolicy",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for AccountUrlType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for AccountUrlType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "TermsOfService" => AccountUrlType::TermsOfService,
            "PrivacyPolicy" => AccountUrlType::PrivacyPolicy,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for AccountUrlType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountUrlType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AccountUrlType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "TermsOfService" => AccountUrlType::TermsOfService,
                    "PrivacyPolicy" => AccountUrlType::PrivacyPolicy,
                    _ => AccountUrlType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(AccountUrlType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：FedCm.type.DialogButton。
///
///协议原文：
///
///<pre>The buttons on the FedCM dialog.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DialogButton {
    ///协议值 "ConfirmIdpLoginContinue"。
    ConfirmIdpLoginContinue,
    ///协议值 "ErrorGotIt"。
    ErrorGotIt,
    ///协议值 "ErrorMoreDetails"。
    ErrorMoreDetails,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl DialogButton {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::fed_cm::types::DialogButton::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::ConfirmIdpLoginContinue => "ConfirmIdpLoginContinue",
            Self::ErrorGotIt => "ErrorGotIt",
            Self::ErrorMoreDetails => "ErrorMoreDetails",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for DialogButton {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for DialogButton {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ConfirmIdpLoginContinue" => DialogButton::ConfirmIdpLoginContinue,
            "ErrorGotIt" => DialogButton::ErrorGotIt,
            "ErrorMoreDetails" => DialogButton::ErrorMoreDetails,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for DialogButton {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DialogButton {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DialogButton;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "ConfirmIdpLoginContinue" => DialogButton::ConfirmIdpLoginContinue,
                    "ErrorGotIt" => DialogButton::ErrorGotIt,
                    "ErrorMoreDetails" => DialogButton::ErrorMoreDetails,
                    _ => DialogButton::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(DialogButton::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：FedCm.type.DialogType。
///
///协议原文：
///
///<pre>The types of FedCM dialogs.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DialogType {
    ///协议值 "AccountChooser"。
    AccountChooser,
    ///协议值 "AutoReauthn"。
    AutoReauthn,
    ///协议值 "ConfirmIdpLogin"。
    ConfirmIdpLogin,
    ///协议值 "Error"。
    Error,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl DialogType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::fed_cm::types::DialogType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::AccountChooser => "AccountChooser",
            Self::AutoReauthn => "AutoReauthn",
            Self::ConfirmIdpLogin => "ConfirmIdpLogin",
            Self::Error => "Error",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for DialogType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for DialogType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "AccountChooser" => DialogType::AccountChooser,
            "AutoReauthn" => DialogType::AutoReauthn,
            "ConfirmIdpLogin" => DialogType::ConfirmIdpLogin,
            "Error" => DialogType::Error,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for DialogType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DialogType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DialogType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "AccountChooser" => DialogType::AccountChooser,
                    "AutoReauthn" => DialogType::AutoReauthn,
                    "ConfirmIdpLogin" => DialogType::ConfirmIdpLogin,
                    "Error" => DialogType::Error,
                    _ => DialogType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(DialogType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：FedCm.type.LoginState。
///
///协议原文：
///
///<pre>Whether this is a sign-up or sign-in action for this account, i.e.
///whether this account has ever been used to sign in to this RP before.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum LoginState {
    ///协议值 "SignIn"。
    SignIn,
    ///协议值 "SignUp"。
    SignUp,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl LoginState {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::fed_cm::types::LoginState::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::SignIn => "SignIn",
            Self::SignUp => "SignUp",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for LoginState {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for LoginState {
    fn from(value: String) -> Self {
        match value.as_str() {
            "SignIn" => LoginState::SignIn,
            "SignUp" => LoginState::SignUp,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for LoginState {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LoginState {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LoginState;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "SignIn" => LoginState::SignIn,
                    "SignUp" => LoginState::SignUp,
                    _ => LoginState::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(LoginState::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
