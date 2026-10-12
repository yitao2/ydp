///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Fetch.command.continueRequest/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ContinueRequestParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.command.continueRequest/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>An id the client received in requestPaused event.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::fetch::types::RequestId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.continueRequest/parameters/url。
    ///
    ///协议原文：
    ///
    ///<pre>If set, the request url will be modified in a way that's not observable by page.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.continueRequest/parameters/method。
    ///
    ///协议原文：
    ///
    ///<pre>If set, the request method is overridden.</pre>
    ///
    ///
    #[serde(rename = "method")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub method: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.continueRequest/parameters/postData。
    ///
    ///协议原文：
    ///
    ///<pre>If set, overrides the post data in the request. (Encoded as a base64 string when passed over JSON)</pre>
    ///
    ///
    #[serde(rename = "postData")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub post_data: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.continueRequest/parameters/headers。
    ///
    ///协议原文：
    ///
    ///<pre>If set, overrides the request headers. Note that the overrides do not
    ///extend to subsequent redirect hops, if a redirect happens. Another override
    ///may be applied to a different request produced by a redirect.</pre>
    ///
    ///
    #[serde(rename = "headers")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub headers: Option<Vec<crate::fetch::types::HeaderEntry>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.continueRequest/parameters/interceptResponse。
    ///
    ///协议原文：
    ///
    ///<pre>If set, overrides response interception behavior for this request.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "interceptResponse")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub intercept_response: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for ContinueRequestParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContinueRequestParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.command.continueRequest/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An id the client received in requestPaused event.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::fetch::types::RequestId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.continueRequest/parameters/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If set, the request url will be modified in a way that's not observable by page.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub url: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.continueRequest/parameters/method。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If set, the request method is overridden.</pre>
                    ///
                    ///
                    #[serde(rename = "method")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub method: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.continueRequest/parameters/postData。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If set, overrides the post data in the request. (Encoded as a base64 string when passed over JSON)</pre>
                    ///
                    ///
                    #[serde(rename = "postData")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub post_data: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.continueRequest/parameters/headers。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If set, overrides the request headers. Note that the overrides do not
                    ///extend to subsequent redirect hops, if a redirect happens. Another override
                    ///may be applied to a different request produced by a redirect.</pre>
                    ///
                    ///
                    #[serde(rename = "headers")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub headers: Option<Vec<crate::fetch::types::HeaderEntry>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.continueRequest/parameters/interceptResponse。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If set, overrides response interception behavior for this request.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "interceptResponse")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub intercept_response: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ContinueRequestParams {
                    request_id: fields.request_id,
                    url: fields.url,
                    method: fields.method,
                    post_data: fields.post_data,
                    headers: fields.headers,
                    intercept_response: fields.intercept_response,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Fetch.command.continueRequest。
///
///协议原文：
///
///<pre>Continues the request, optionally modifying some of its parameters.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ContinueRequest;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Fetch.command.continueRequest/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ContinueRequestResponse {}
impl<'de> serde::Deserialize<'de> for ContinueRequestResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContinueRequestResponse;
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
                Ok(ContinueRequestResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ContinueRequest {
    type Params = ContinueRequestParams;
    type Response = ContinueRequestResponse;
    const METHOD: &'static str = "Fetch.continueRequest";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Fetch.command.continueResponse/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ContinueResponseParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.command.continueResponse/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>An id the client received in requestPaused event.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::fetch::types::RequestId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.continueResponse/parameters/responseCode。
    ///
    ///协议原文：
    ///
    ///<pre>An HTTP response code. If absent, original response code will be used.</pre>
    ///
    ///
    #[serde(rename = "responseCode")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_code: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.continueResponse/parameters/responsePhrase。
    ///
    ///协议原文：
    ///
    ///<pre>A textual representation of responseCode.
    ///If absent, a standard phrase matching responseCode is used.</pre>
    ///
    ///
    #[serde(rename = "responsePhrase")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_phrase: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.continueResponse/parameters/responseHeaders。
    ///
    ///协议原文：
    ///
    ///<pre>Response headers. If absent, original response headers will be used.</pre>
    ///
    ///
    #[serde(rename = "responseHeaders")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_headers: Option<Vec<crate::fetch::types::HeaderEntry>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.continueResponse/parameters/binaryResponseHeaders。
    ///
    ///协议原文：
    ///
    ///<pre>Alternative way of specifying response headers as a \0-separated
    ///series of name: value pairs. Prefer the above method unless you
    ///need to represent some non-UTF8 values that can't be transmitted
    ///over the protocol as text. (Encoded as a base64 string when passed over JSON)</pre>
    ///
    ///
    #[serde(rename = "binaryResponseHeaders")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub binary_response_headers: Option<String>,
}
impl<'de> serde::Deserialize<'de> for ContinueResponseParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContinueResponseParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.command.continueResponse/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An id the client received in requestPaused event.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::fetch::types::RequestId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.continueResponse/parameters/responseCode。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An HTTP response code. If absent, original response code will be used.</pre>
                    ///
                    ///
                    #[serde(rename = "responseCode")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub response_code: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.continueResponse/parameters/responsePhrase。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A textual representation of responseCode.
                    ///If absent, a standard phrase matching responseCode is used.</pre>
                    ///
                    ///
                    #[serde(rename = "responsePhrase")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub response_phrase: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.continueResponse/parameters/responseHeaders。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Response headers. If absent, original response headers will be used.</pre>
                    ///
                    ///
                    #[serde(rename = "responseHeaders")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub response_headers: Option<Vec<crate::fetch::types::HeaderEntry>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.continueResponse/parameters/binaryResponseHeaders。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Alternative way of specifying response headers as a \0-separated
                    ///series of name: value pairs. Prefer the above method unless you
                    ///need to represent some non-UTF8 values that can't be transmitted
                    ///over the protocol as text. (Encoded as a base64 string when passed over JSON)</pre>
                    ///
                    ///
                    #[serde(rename = "binaryResponseHeaders")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub binary_response_headers: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ContinueResponseParams {
                    request_id: fields.request_id,
                    response_code: fields.response_code,
                    response_phrase: fields.response_phrase,
                    response_headers: fields.response_headers,
                    binary_response_headers: fields.binary_response_headers,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Fetch.command.continueResponse。
///
///协议原文：
///
///<pre>Continues loading of the paused response, optionally modifying the
///response headers. If either responseCode or headers are modified, all of them
///must be present.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ContinueResponse;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Fetch.command.continueResponse/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ContinueResponseResponse {}
impl<'de> serde::Deserialize<'de> for ContinueResponseResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContinueResponseResponse;
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
                Ok(ContinueResponseResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ContinueResponse {
    type Params = ContinueResponseParams;
    type Response = ContinueResponseResponse;
    const METHOD: &'static str = "Fetch.continueResponse";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Fetch.command.continueWithAuth/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ContinueWithAuthParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.command.continueWithAuth/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>An id the client received in authRequired event.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::fetch::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.command.continueWithAuth/parameters/authChallengeResponse。
    ///
    ///协议原文：
    ///
    ///<pre>Response to  with an authChallenge.</pre>
    ///
    ///
    #[serde(rename = "authChallengeResponse")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub auth_challenge_response: crate::fetch::types::AuthChallengeResponse,
}
impl<'de> serde::Deserialize<'de> for ContinueWithAuthParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContinueWithAuthParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.command.continueWithAuth/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An id the client received in authRequired event.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::fetch::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.command.continueWithAuth/parameters/authChallengeResponse。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Response to  with an authChallenge.</pre>
                    ///
                    ///
                    #[serde(rename = "authChallengeResponse")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub auth_challenge_response: crate::fetch::types::AuthChallengeResponse,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ContinueWithAuthParams {
                    request_id: fields.request_id,
                    auth_challenge_response: fields.auth_challenge_response,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Fetch.command.continueWithAuth。
///
///协议原文：
///
///<pre>Continues a request supplying authChallengeResponse following authRequired event.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ContinueWithAuth;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Fetch.command.continueWithAuth/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ContinueWithAuthResponse {}
impl<'de> serde::Deserialize<'de> for ContinueWithAuthResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContinueWithAuthResponse;
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
                Ok(ContinueWithAuthResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ContinueWithAuth {
    type Params = ContinueWithAuthParams;
    type Response = ContinueWithAuthResponse;
    const METHOD: &'static str = "Fetch.continueWithAuth";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Fetch.command.disable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Fetch.command.disable。
///
///协议原文：
///
///<pre>Disables the fetch domain.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Fetch.command.disable/returns。
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
    const METHOD: &'static str = "Fetch.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Fetch.command.enable/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct EnableParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.enable/parameters/patterns。
    ///
    ///协议原文：
    ///
    ///<pre>If specified, only requests matching any of these patterns will produce
    ///fetchRequested event and will be paused until clients response. If not set,
    ///all requests will be affected.</pre>
    ///
    ///
    #[serde(rename = "patterns")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub patterns: Option<Vec<crate::fetch::types::RequestPattern>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.enable/parameters/handleAuthRequests。
    ///
    ///协议原文：
    ///
    ///<pre>If true, authRequired events will be issued and requests will be paused
    ///expecting a call to continueWithAuth.</pre>
    ///
    ///
    #[serde(rename = "handleAuthRequests")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub handle_auth_requests: Option<bool>,
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
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.enable/parameters/patterns。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If specified, only requests matching any of these patterns will produce
                    ///fetchRequested event and will be paused until clients response. If not set,
                    ///all requests will be affected.</pre>
                    ///
                    ///
                    #[serde(rename = "patterns")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub patterns: Option<Vec<crate::fetch::types::RequestPattern>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.enable/parameters/handleAuthRequests。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, authRequired events will be issued and requests will be paused
                    ///expecting a call to continueWithAuth.</pre>
                    ///
                    ///
                    #[serde(rename = "handleAuthRequests")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub handle_auth_requests: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EnableParams {
                    patterns: fields.patterns,
                    handle_auth_requests: fields.handle_auth_requests,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Fetch.command.enable。
///
///协议原文：
///
///<pre>Enables issuing of requestPaused events. A request will be paused until client
///calls one of failRequest, fulfillRequest or continueRequest/continueWithAuth.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Fetch.command.enable/returns。
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
    const METHOD: &'static str = "Fetch.enable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Fetch.command.failRequest/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FailRequestParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.command.failRequest/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>An id the client received in requestPaused event.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::fetch::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.command.failRequest/parameters/errorReason。
    ///
    ///协议原文：
    ///
    ///<pre>Causes the request to fail with the given reason.</pre>
    ///
    ///
    #[serde(rename = "errorReason")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub error_reason: crate::network::types::ErrorReason,
}
impl<'de> serde::Deserialize<'de> for FailRequestParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FailRequestParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.command.failRequest/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An id the client received in requestPaused event.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::fetch::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.command.failRequest/parameters/errorReason。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Causes the request to fail with the given reason.</pre>
                    ///
                    ///
                    #[serde(rename = "errorReason")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub error_reason: crate::network::types::ErrorReason,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FailRequestParams {
                    request_id: fields.request_id,
                    error_reason: fields.error_reason,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Fetch.command.failRequest。
///
///协议原文：
///
///<pre>Causes the request to fail with specified reason.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct FailRequest;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Fetch.command.failRequest/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FailRequestResponse {}
impl<'de> serde::Deserialize<'de> for FailRequestResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FailRequestResponse;
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
                Ok(FailRequestResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for FailRequest {
    type Params = FailRequestParams;
    type Response = FailRequestResponse;
    const METHOD: &'static str = "Fetch.failRequest";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Fetch.command.fulfillRequest/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FulfillRequestParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.command.fulfillRequest/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>An id the client received in requestPaused event.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::fetch::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.command.fulfillRequest/parameters/responseCode。
    ///
    ///协议原文：
    ///
    ///<pre>An HTTP response code.</pre>
    ///
    ///
    #[serde(rename = "responseCode")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub response_code: i64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.fulfillRequest/parameters/responseHeaders。
    ///
    ///协议原文：
    ///
    ///<pre>Response headers.</pre>
    ///
    ///
    #[serde(rename = "responseHeaders")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_headers: Option<Vec<crate::fetch::types::HeaderEntry>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.fulfillRequest/parameters/binaryResponseHeaders。
    ///
    ///协议原文：
    ///
    ///<pre>Alternative way of specifying response headers as a \0-separated
    ///series of name: value pairs. Prefer the above method unless you
    ///need to represent some non-UTF8 values that can't be transmitted
    ///over the protocol as text. (Encoded as a base64 string when passed over JSON)</pre>
    ///
    ///
    #[serde(rename = "binaryResponseHeaders")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub binary_response_headers: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.fulfillRequest/parameters/body。
    ///
    ///协议原文：
    ///
    ///<pre>A response body. If absent, original response body will be used if
    ///the request is intercepted at the response stage and empty body
    ///will be used if the request is intercepted at the request stage. (Encoded as a base64 string when passed over JSON)</pre>
    ///
    ///
    #[serde(rename = "body")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub body: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.fulfillRequest/parameters/responsePhrase。
    ///
    ///协议原文：
    ///
    ///<pre>A textual representation of responseCode.
    ///If absent, a standard phrase matching responseCode is used.</pre>
    ///
    ///
    #[serde(rename = "responsePhrase")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_phrase: Option<String>,
}
impl<'de> serde::Deserialize<'de> for FulfillRequestParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FulfillRequestParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.command.fulfillRequest/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An id the client received in requestPaused event.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::fetch::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.command.fulfillRequest/parameters/responseCode。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An HTTP response code.</pre>
                    ///
                    ///
                    #[serde(rename = "responseCode")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub response_code: i64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.fulfillRequest/parameters/responseHeaders。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Response headers.</pre>
                    ///
                    ///
                    #[serde(rename = "responseHeaders")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub response_headers: Option<Vec<crate::fetch::types::HeaderEntry>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.fulfillRequest/parameters/binaryResponseHeaders。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Alternative way of specifying response headers as a \0-separated
                    ///series of name: value pairs. Prefer the above method unless you
                    ///need to represent some non-UTF8 values that can't be transmitted
                    ///over the protocol as text. (Encoded as a base64 string when passed over JSON)</pre>
                    ///
                    ///
                    #[serde(rename = "binaryResponseHeaders")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub binary_response_headers: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.fulfillRequest/parameters/body。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A response body. If absent, original response body will be used if
                    ///the request is intercepted at the response stage and empty body
                    ///will be used if the request is intercepted at the request stage. (Encoded as a base64 string when passed over JSON)</pre>
                    ///
                    ///
                    #[serde(rename = "body")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub body: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.command.fulfillRequest/parameters/responsePhrase。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A textual representation of responseCode.
                    ///If absent, a standard phrase matching responseCode is used.</pre>
                    ///
                    ///
                    #[serde(rename = "responsePhrase")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub response_phrase: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FulfillRequestParams {
                    request_id: fields.request_id,
                    response_code: fields.response_code,
                    response_headers: fields.response_headers,
                    binary_response_headers: fields.binary_response_headers,
                    body: fields.body,
                    response_phrase: fields.response_phrase,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Fetch.command.fulfillRequest。
///
///协议原文：
///
///<pre>Provides response to the request.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct FulfillRequest;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Fetch.command.fulfillRequest/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FulfillRequestResponse {}
impl<'de> serde::Deserialize<'de> for FulfillRequestResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FulfillRequestResponse;
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
                Ok(FulfillRequestResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for FulfillRequest {
    type Params = FulfillRequestParams;
    type Response = FulfillRequestResponse;
    const METHOD: &'static str = "Fetch.fulfillRequest";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Fetch.command.getResponseBody/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetResponseBodyParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.command.getResponseBody/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier for the intercepted request to get body for.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::fetch::types::RequestId,
}
impl<'de> serde::Deserialize<'de> for GetResponseBodyParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetResponseBodyParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.command.getResponseBody/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier for the intercepted request to get body for.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::fetch::types::RequestId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetResponseBodyParams {
                    request_id: fields.request_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Fetch.command.getResponseBody。
///
///协议原文：
///
///<pre>Causes the body of the response to be received from the server and
///returned as a single string. May only be issued for a request that
///is paused in the Response stage and is mutually exclusive with
///takeResponseBodyForInterceptionAsStream. Calling other methods that
///affect the request or disabling fetch domain before body is received
///results in an undefined behavior.
///Note that the response body is not available for redirects. Requests
///paused in the _redirect received_ state may be differentiated by
///`responseCode` and presence of `location` response header, see
///comments to `requestPaused` for details.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetResponseBody;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Fetch.command.getResponseBody/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetResponseBodyResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.command.getResponseBody/returns/body。
    ///
    ///协议原文：
    ///
    ///<pre>Response body.</pre>
    ///
    ///
    #[serde(rename = "body")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub body: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.command.getResponseBody/returns/base64Encoded。
    ///
    ///协议原文：
    ///
    ///<pre>True, if content was sent as base64.</pre>
    ///
    ///
    #[serde(rename = "base64Encoded")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub base64_encoded: bool,
}
impl<'de> serde::Deserialize<'de> for GetResponseBodyResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetResponseBodyResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.command.getResponseBody/returns/body。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Response body.</pre>
                    ///
                    ///
                    #[serde(rename = "body")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub body: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.command.getResponseBody/returns/base64Encoded。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True, if content was sent as base64.</pre>
                    ///
                    ///
                    #[serde(rename = "base64Encoded")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub base64_encoded: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetResponseBodyResponse {
                    body: fields.body,
                    base64_encoded: fields.base64_encoded,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetResponseBody {
    type Params = GetResponseBodyParams;
    type Response = GetResponseBodyResponse;
    const METHOD: &'static str = "Fetch.getResponseBody";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Fetch.command.takeResponseBodyAsStream/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TakeResponseBodyAsStreamParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.command.takeResponseBodyAsStream/parameters/requestId。
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::fetch::types::RequestId,
}
impl<'de> serde::Deserialize<'de> for TakeResponseBodyAsStreamParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TakeResponseBodyAsStreamParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.command.takeResponseBodyAsStream/parameters/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::fetch::types::RequestId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TakeResponseBodyAsStreamParams {
                    request_id: fields.request_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Fetch.command.takeResponseBodyAsStream。
///
///协议原文：
///
///<pre>Returns a handle to the stream representing the response body.
///The request must be paused in the HeadersReceived stage.
///Note that after this command the request can't be continued
///as is -- client either needs to cancel it or to provide the
///response body.
///The stream only supports sequential read, IO.read will fail if the position
///is specified.
///This method is mutually exclusive with getResponseBody.
///Calling other methods that affect the request or disabling fetch
///domain before body is received results in an undefined behavior.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct TakeResponseBodyAsStream;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Fetch.command.takeResponseBodyAsStream/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TakeResponseBodyAsStreamResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.command.takeResponseBodyAsStream/returns/stream。
    #[serde(rename = "stream")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub stream: crate::io::types::StreamHandle,
}
impl<'de> serde::Deserialize<'de> for TakeResponseBodyAsStreamResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TakeResponseBodyAsStreamResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.command.takeResponseBodyAsStream/returns/stream。
                    #[serde(rename = "stream")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub stream: crate::io::types::StreamHandle,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TakeResponseBodyAsStreamResponse {
                    stream: fields.stream,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for TakeResponseBodyAsStream {
    type Params = TakeResponseBodyAsStreamParams;
    type Response = TakeResponseBodyAsStreamResponse;
    const METHOD: &'static str = "Fetch.takeResponseBodyAsStream";
}
