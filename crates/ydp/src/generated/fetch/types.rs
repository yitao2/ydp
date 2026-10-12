///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Fetch.type.AuthChallenge/properties/source。
///
///协议原文：
///
///<pre>Source of the authentication challenge.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AuthChallengeSource {
    ///协议值 "Server"。
    Server,
    ///协议值 "Proxy"。
    Proxy,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl AuthChallengeSource {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::fetch::types::AuthChallengeSource::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Server => "Server",
            Self::Proxy => "Proxy",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for AuthChallengeSource {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for AuthChallengeSource {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Server" => AuthChallengeSource::Server,
            "Proxy" => AuthChallengeSource::Proxy,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for AuthChallengeSource {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AuthChallengeSource {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AuthChallengeSource;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Server" => AuthChallengeSource::Server,
                    "Proxy" => AuthChallengeSource::Proxy,
                    _ => AuthChallengeSource::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(AuthChallengeSource::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Fetch.type.AuthChallenge。
///
///协议原文：
///
///<pre>Authorization challenge for HTTP status code 401 or 407.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct AuthChallenge {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.type.AuthChallenge/properties/source。
    ///
    ///协议原文：
    ///
    ///<pre>Source of the authentication challenge.</pre>
    ///
    ///
    #[serde(rename = "source")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub source: Option<AuthChallengeSource>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.type.AuthChallenge/properties/origin。
    ///
    ///协议原文：
    ///
    ///<pre>Origin of the challenger.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.type.AuthChallenge/properties/scheme。
    ///
    ///协议原文：
    ///
    ///<pre>The authentication scheme used, such as basic or digest</pre>
    ///
    ///
    #[serde(rename = "scheme")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub scheme: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.type.AuthChallenge/properties/realm。
    ///
    ///协议原文：
    ///
    ///<pre>The realm of the challenge. May be empty.</pre>
    ///
    ///
    #[serde(rename = "realm")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub realm: String,
}
impl<'de> serde::Deserialize<'de> for AuthChallenge {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AuthChallenge;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.type.AuthChallenge/properties/source。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Source of the authentication challenge.</pre>
                    ///
                    ///
                    #[serde(rename = "source")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub source: Option<AuthChallengeSource>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.type.AuthChallenge/properties/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Origin of the challenger.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.type.AuthChallenge/properties/scheme。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The authentication scheme used, such as basic or digest</pre>
                    ///
                    ///
                    #[serde(rename = "scheme")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub scheme: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.type.AuthChallenge/properties/realm。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The realm of the challenge. May be empty.</pre>
                    ///
                    ///
                    #[serde(rename = "realm")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub realm: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AuthChallenge {
                    source: fields.source,
                    origin: fields.origin,
                    scheme: fields.scheme,
                    realm: fields.realm,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Fetch.type.AuthChallengeResponse/properties/response。
///
///协议原文：
///
///<pre>The decision on what to do in response to the authorization challenge.  Default means
///deferring to the default behavior of the net stack, which will likely either the Cancel
///authentication or display a popup dialog box.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AuthChallengeResponseResponse {
    ///协议值 "Default"。
    Default,
    ///协议值 "CancelAuth"。
    CancelAuth,
    ///协议值 "ProvideCredentials"。
    ProvideCredentials,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl AuthChallengeResponseResponse {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::fetch::types::AuthChallengeResponseResponse::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Default => "Default",
            Self::CancelAuth => "CancelAuth",
            Self::ProvideCredentials => "ProvideCredentials",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for AuthChallengeResponseResponse {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for AuthChallengeResponseResponse {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Default" => AuthChallengeResponseResponse::Default,
            "CancelAuth" => AuthChallengeResponseResponse::CancelAuth,
            "ProvideCredentials" => AuthChallengeResponseResponse::ProvideCredentials,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for AuthChallengeResponseResponse {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AuthChallengeResponseResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AuthChallengeResponseResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Default" => AuthChallengeResponseResponse::Default,
                    "CancelAuth" => AuthChallengeResponseResponse::CancelAuth,
                    "ProvideCredentials" => AuthChallengeResponseResponse::ProvideCredentials,
                    _ => AuthChallengeResponseResponse::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(AuthChallengeResponseResponse::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Fetch.type.AuthChallengeResponse。
///
///协议原文：
///
///<pre>Response to an AuthChallenge.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct AuthChallengeResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.type.AuthChallengeResponse/properties/response。
    ///
    ///协议原文：
    ///
    ///<pre>The decision on what to do in response to the authorization challenge.  Default means
    ///deferring to the default behavior of the net stack, which will likely either the Cancel
    ///authentication or display a popup dialog box.</pre>
    ///
    ///
    #[serde(rename = "response")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub response: AuthChallengeResponseResponse,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.type.AuthChallengeResponse/properties/username。
    ///
    ///协议原文：
    ///
    ///<pre>The username to provide, possibly empty. Should only be set if response is
    ///ProvideCredentials.</pre>
    ///
    ///
    #[serde(rename = "username")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub username: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.type.AuthChallengeResponse/properties/password。
    ///
    ///协议原文：
    ///
    ///<pre>The password to provide, possibly empty. Should only be set if response is
    ///ProvideCredentials.</pre>
    ///
    ///
    #[serde(rename = "password")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub password: Option<String>,
}
impl<'de> serde::Deserialize<'de> for AuthChallengeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AuthChallengeResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.type.AuthChallengeResponse/properties/response。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The decision on what to do in response to the authorization challenge.  Default means
                    ///deferring to the default behavior of the net stack, which will likely either the Cancel
                    ///authentication or display a popup dialog box.</pre>
                    ///
                    ///
                    #[serde(rename = "response")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub response: AuthChallengeResponseResponse,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.type.AuthChallengeResponse/properties/username。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The username to provide, possibly empty. Should only be set if response is
                    ///ProvideCredentials.</pre>
                    ///
                    ///
                    #[serde(rename = "username")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub username: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.type.AuthChallengeResponse/properties/password。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The password to provide, possibly empty. Should only be set if response is
                    ///ProvideCredentials.</pre>
                    ///
                    ///
                    #[serde(rename = "password")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub password: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AuthChallengeResponse {
                    response: fields.response,
                    username: fields.username,
                    password: fields.password,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Fetch.type.HeaderEntry。
///
///协议原文：
///
///<pre>Response HTTP header entry</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct HeaderEntry {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.type.HeaderEntry/properties/name。
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.type.HeaderEntry/properties/value。
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: String,
}
impl<'de> serde::Deserialize<'de> for HeaderEntry {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HeaderEntry;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.type.HeaderEntry/properties/name。
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.type.HeaderEntry/properties/value。
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(HeaderEntry {
                    name: fields.name,
                    value: fields.value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Fetch.type.RequestId。
///
///协议原文：
///
///<pre>Unique request identifier.
///Note that this does not identify individual HTTP requests that are part of
///a network request.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct RequestId(String);
impl RequestId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::fetch::types::RequestId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::fetch::types::RequestId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::fetch::types::RequestId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::fetch::types::RequestId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::fetch::types::RequestId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Fetch.type.RequestPattern。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequestPattern {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.type.RequestPattern/properties/urlPattern。
    ///
    ///协议原文：
    ///
    ///<pre>Wildcards (`'*'` -&gt; zero or more, `'?'` -&gt; exactly one) are allowed. Escape character is
    ///backslash. Omitting is equivalent to `"*"`.</pre>
    ///
    ///
    #[serde(rename = "urlPattern")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub url_pattern: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.type.RequestPattern/properties/resourceType。
    ///
    ///协议原文：
    ///
    ///<pre>If set, only requests for matching resource types will be intercepted.</pre>
    ///
    ///
    #[serde(rename = "resourceType")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_type: Option<crate::network::types::ResourceType>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.type.RequestPattern/properties/requestStage。
    ///
    ///协议原文：
    ///
    ///<pre>Stage at which to begin intercepting requests. Default is Request.</pre>
    ///
    ///
    #[serde(rename = "requestStage")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub request_stage: Option<crate::fetch::types::RequestStage>,
}
impl<'de> serde::Deserialize<'de> for RequestPattern {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RequestPattern;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.type.RequestPattern/properties/urlPattern。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Wildcards (`'*'` -&gt; zero or more, `'?'` -&gt; exactly one) are allowed. Escape character is
                    ///backslash. Omitting is equivalent to `"*"`.</pre>
                    ///
                    ///
                    #[serde(rename = "urlPattern")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub url_pattern: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.type.RequestPattern/properties/resourceType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If set, only requests for matching resource types will be intercepted.</pre>
                    ///
                    ///
                    #[serde(rename = "resourceType")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub resource_type: Option<crate::network::types::ResourceType>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.type.RequestPattern/properties/requestStage。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Stage at which to begin intercepting requests. Default is Request.</pre>
                    ///
                    ///
                    #[serde(rename = "requestStage")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub request_stage: Option<crate::fetch::types::RequestStage>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RequestPattern {
                    url_pattern: fields.url_pattern,
                    resource_type: fields.resource_type,
                    request_stage: fields.request_stage,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Fetch.type.RequestStage。
///
///协议原文：
///
///<pre>Stages of the request to handle. Request will intercept before the request is
///sent. Response will intercept after the response is received (but before response
///body is received).</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RequestStage {
    ///协议值 "Request"。
    Request,
    ///协议值 "Response"。
    Response,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl RequestStage {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::fetch::types::RequestStage::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Request => "Request",
            Self::Response => "Response",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for RequestStage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for RequestStage {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Request" => RequestStage::Request,
            "Response" => RequestStage::Response,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for RequestStage {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RequestStage {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RequestStage;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Request" => RequestStage::Request,
                    "Response" => RequestStage::Response,
                    _ => RequestStage::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(RequestStage::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
