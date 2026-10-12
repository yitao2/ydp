///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Fetch.event.authRequired/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AuthRequiredParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.event.authRequired/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Each request the page makes will have a unique id.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::fetch::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.event.authRequired/parameters/request。
    ///
    ///协议原文：
    ///
    ///<pre>The details of the request.</pre>
    ///
    ///
    #[serde(rename = "request")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request: crate::network::types::Request,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.event.authRequired/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>The id of the frame that initiated the request.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.event.authRequired/parameters/resourceType。
    ///
    ///协议原文：
    ///
    ///<pre>How the requested resource will be used.</pre>
    ///
    ///
    #[serde(rename = "resourceType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub resource_type: crate::network::types::ResourceType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.event.authRequired/parameters/authChallenge。
    ///
    ///协议原文：
    ///
    ///<pre>Details of the Authorization Challenge encountered.
    ///If this is set, client should respond with continueRequest that
    ///contains AuthChallengeResponse.</pre>
    ///
    ///
    #[serde(rename = "authChallenge")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub auth_challenge: crate::fetch::types::AuthChallenge,
}
impl<'de> serde::Deserialize<'de> for AuthRequiredParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AuthRequiredParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.event.authRequired/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Each request the page makes will have a unique id.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::fetch::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.event.authRequired/parameters/request。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The details of the request.</pre>
                    ///
                    ///
                    #[serde(rename = "request")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request: crate::network::types::Request,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.event.authRequired/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The id of the frame that initiated the request.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.event.authRequired/parameters/resourceType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>How the requested resource will be used.</pre>
                    ///
                    ///
                    #[serde(rename = "resourceType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub resource_type: crate::network::types::ResourceType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.event.authRequired/parameters/authChallenge。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Details of the Authorization Challenge encountered.
                    ///If this is set, client should respond with continueRequest that
                    ///contains AuthChallengeResponse.</pre>
                    ///
                    ///
                    #[serde(rename = "authChallenge")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub auth_challenge: crate::fetch::types::AuthChallenge,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AuthRequiredParams {
                    request_id: fields.request_id,
                    request: fields.request,
                    frame_id: fields.frame_id,
                    resource_type: fields.resource_type,
                    auth_challenge: fields.auth_challenge,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Fetch.event.authRequired。
///
///协议原文：
///
///<pre>Issued when the domain is enabled with handleAuthRequests set to true.
///The request is paused until client responds with continueWithAuth.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct AuthRequired;
impl ydp_core::Event for AuthRequired {
    type Params = AuthRequiredParams;
    const METHOD: &'static str = "Fetch.authRequired";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Fetch.event.requestPaused/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequestPausedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.event.requestPaused/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Each request the page makes will have a unique id.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::fetch::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.event.requestPaused/parameters/request。
    ///
    ///协议原文：
    ///
    ///<pre>The details of the request.</pre>
    ///
    ///
    #[serde(rename = "request")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request: crate::network::types::Request,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.event.requestPaused/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>The id of the frame that initiated the request.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.event.requestPaused/parameters/resourceType。
    ///
    ///协议原文：
    ///
    ///<pre>How the requested resource will be used.</pre>
    ///
    ///
    #[serde(rename = "resourceType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub resource_type: crate::network::types::ResourceType,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.event.requestPaused/parameters/responseErrorReason。
    ///
    ///协议原文：
    ///
    ///<pre>Response error if intercepted at response stage.</pre>
    ///
    ///
    #[serde(rename = "responseErrorReason")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_error_reason: Option<crate::network::types::ErrorReason>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.event.requestPaused/parameters/responseStatusCode。
    ///
    ///协议原文：
    ///
    ///<pre>Response code if intercepted at response stage.</pre>
    ///
    ///
    #[serde(rename = "responseStatusCode")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_status_code: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.event.requestPaused/parameters/responseStatusText。
    ///
    ///协议原文：
    ///
    ///<pre>Response status text if intercepted at response stage.</pre>
    ///
    ///
    #[serde(rename = "responseStatusText")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_status_text: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.event.requestPaused/parameters/responseHeaders。
    ///
    ///协议原文：
    ///
    ///<pre>Response headers if intercepted at the response stage.</pre>
    ///
    ///
    #[serde(rename = "responseHeaders")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_headers: Option<Vec<crate::fetch::types::HeaderEntry>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.event.requestPaused/parameters/networkId。
    ///
    ///协议原文：
    ///
    ///<pre>If the intercepted request had a corresponding Network.requestWillBeSent event fired for it,
    ///then this networkId will be the same as the requestId present in the requestWillBeSent event.</pre>
    ///
    ///
    #[serde(rename = "networkId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub network_id: Option<crate::network::types::RequestId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.event.requestPaused/parameters/redirectedRequestId。
    ///
    ///协议原文：
    ///
    ///<pre>If the request is due to a redirect response from the server, the id of the request that
    ///has caused the redirect.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "redirectedRequestId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub redirected_request_id: Option<crate::fetch::types::RequestId>,
}
impl<'de> serde::Deserialize<'de> for RequestPausedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RequestPausedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.event.requestPaused/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Each request the page makes will have a unique id.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::fetch::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.event.requestPaused/parameters/request。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The details of the request.</pre>
                    ///
                    ///
                    #[serde(rename = "request")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request: crate::network::types::Request,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.event.requestPaused/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The id of the frame that initiated the request.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Fetch.event.requestPaused/parameters/resourceType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>How the requested resource will be used.</pre>
                    ///
                    ///
                    #[serde(rename = "resourceType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub resource_type: crate::network::types::ResourceType,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.event.requestPaused/parameters/responseErrorReason。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Response error if intercepted at response stage.</pre>
                    ///
                    ///
                    #[serde(rename = "responseErrorReason")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub response_error_reason: Option<crate::network::types::ErrorReason>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.event.requestPaused/parameters/responseStatusCode。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Response code if intercepted at response stage.</pre>
                    ///
                    ///
                    #[serde(rename = "responseStatusCode")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub response_status_code: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.event.requestPaused/parameters/responseStatusText。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Response status text if intercepted at response stage.</pre>
                    ///
                    ///
                    #[serde(rename = "responseStatusText")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub response_status_text: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.event.requestPaused/parameters/responseHeaders。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Response headers if intercepted at the response stage.</pre>
                    ///
                    ///
                    #[serde(rename = "responseHeaders")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub response_headers: Option<Vec<crate::fetch::types::HeaderEntry>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.event.requestPaused/parameters/networkId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If the intercepted request had a corresponding Network.requestWillBeSent event fired for it,
                    ///then this networkId will be the same as the requestId present in the requestWillBeSent event.</pre>
                    ///
                    ///
                    #[serde(rename = "networkId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub network_id: Option<crate::network::types::RequestId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Fetch.event.requestPaused/parameters/redirectedRequestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If the request is due to a redirect response from the server, the id of the request that
                    ///has caused the redirect.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "redirectedRequestId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub redirected_request_id: Option<crate::fetch::types::RequestId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RequestPausedParams {
                    request_id: fields.request_id,
                    request: fields.request,
                    frame_id: fields.frame_id,
                    resource_type: fields.resource_type,
                    response_error_reason: fields.response_error_reason,
                    response_status_code: fields.response_status_code,
                    response_status_text: fields.response_status_text,
                    response_headers: fields.response_headers,
                    network_id: fields.network_id,
                    redirected_request_id: fields.redirected_request_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Fetch.event.requestPaused。
///
///协议原文：
///
///<pre>Issued when the domain is enabled and the request URL matches the
///specified filter. The request is paused until the client responds
///with one of continueRequest, failRequest or fulfillRequest.
///The stage of the request can be determined by presence of responseErrorReason
///and responseStatusCode -- the request is at the response stage if either
///of these fields is present and in the request stage otherwise.
///Redirect responses and subsequent requests are reported similarly to regular
///responses and requests. Redirect responses may be distinguished by the value
///of `responseStatusCode` (which is one of 301, 302, 303, 307, 308) along with
///presence of the `location` header. Requests resulting from a redirect will
///have `redirectedRequestId` field set.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RequestPaused;
impl ydp_core::Event for RequestPaused {
    type Params = RequestPausedParams;
    const METHOD: &'static str = "Fetch.requestPaused";
}
