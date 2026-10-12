///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Security.event.certificateError/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CertificateErrorParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.event.certificateError/parameters/eventId。
    ///
    ///协议原文：
    ///
    ///<pre>The ID of the event.</pre>
    ///
    ///
    #[serde(rename = "eventId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub event_id: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.event.certificateError/parameters/errorType。
    ///
    ///协议原文：
    ///
    ///<pre>The type of the error.</pre>
    ///
    ///
    #[serde(rename = "errorType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub error_type: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.event.certificateError/parameters/requestURL。
    ///
    ///协议原文：
    ///
    ///<pre>The url that was requested.</pre>
    ///
    ///
    #[serde(rename = "requestURL")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_url: String,
}
impl<'de> serde::Deserialize<'de> for CertificateErrorParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CertificateErrorParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.event.certificateError/parameters/eventId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The ID of the event.</pre>
                    ///
                    ///
                    #[serde(rename = "eventId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub event_id: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.event.certificateError/parameters/errorType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The type of the error.</pre>
                    ///
                    ///
                    #[serde(rename = "errorType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub error_type: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.event.certificateError/parameters/requestURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The url that was requested.</pre>
                    ///
                    ///
                    #[serde(rename = "requestURL")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_url: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CertificateErrorParams {
                    event_id: fields.event_id,
                    error_type: fields.error_type,
                    request_url: fields.request_url,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Security.event.certificateError。
///
///协议原文：
///
///<pre>There is a certificate error. If overriding certificate errors is enabled, then it should be
///handled with the `handleCertificateError` command. Note: this event does not fire if the
///certificate error has been allowed internally. Only one client per target should override
///certificate errors at the same time.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct CertificateError;
impl ydp_core::Event for CertificateError {
    type Params = CertificateErrorParams;
    const METHOD: &'static str = "Security.certificateError";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Security.event.securityStateChanged/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SecurityStateChangedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.event.securityStateChanged/parameters/securityState。
    ///
    ///协议原文：
    ///
    ///<pre>Security state.</pre>
    ///
    ///
    #[serde(rename = "securityState")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub security_state: crate::security::types::SecurityState,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.event.securityStateChanged/parameters/schemeIsCryptographic。
    ///
    ///协议原文：
    ///
    ///<pre>True if the page was loaded over cryptographic transport such as HTTPS.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "schemeIsCryptographic")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub scheme_is_cryptographic: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.event.securityStateChanged/parameters/explanations。
    ///
    ///协议原文：
    ///
    ///<pre>Previously a list of explanations for the security state. Now always
    ///empty.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "explanations")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub explanations: Vec<crate::security::types::SecurityStateExplanation>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.event.securityStateChanged/parameters/insecureContentStatus。
    ///
    ///协议原文：
    ///
    ///<pre>Information about insecure content on the page.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "insecureContentStatus")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub insecure_content_status: crate::security::types::InsecureContentStatus,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Security.event.securityStateChanged/parameters/summary。
    ///
    ///协议原文：
    ///
    ///<pre>Overrides user-visible description of the state. Always omitted.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "summary")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub summary: Option<String>,
}
impl<'de> serde::Deserialize<'de> for SecurityStateChangedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SecurityStateChangedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.event.securityStateChanged/parameters/securityState。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Security state.</pre>
                    ///
                    ///
                    #[serde(rename = "securityState")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub security_state: crate::security::types::SecurityState,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.event.securityStateChanged/parameters/schemeIsCryptographic。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if the page was loaded over cryptographic transport such as HTTPS.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "schemeIsCryptographic")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub scheme_is_cryptographic: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.event.securityStateChanged/parameters/explanations。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Previously a list of explanations for the security state. Now always
                    ///empty.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "explanations")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub explanations: Vec<crate::security::types::SecurityStateExplanation>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.event.securityStateChanged/parameters/insecureContentStatus。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Information about insecure content on the page.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "insecureContentStatus")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub insecure_content_status: crate::security::types::InsecureContentStatus,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Security.event.securityStateChanged/parameters/summary。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Overrides user-visible description of the state. Always omitted.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "summary")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub summary: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SecurityStateChangedParams {
                    security_state: fields.security_state,
                    scheme_is_cryptographic: fields.scheme_is_cryptographic,
                    explanations: fields.explanations,
                    insecure_content_status: fields.insecure_content_status,
                    summary: fields.summary,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Security.event.securityStateChanged。
///
///协议原文：
///
///<pre>The security state of the page changed. No longer being sent.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct SecurityStateChanged;
impl ydp_core::Event for SecurityStateChanged {
    type Params = SecurityStateChangedParams;
    const METHOD: &'static str = "Security.securityStateChanged";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Security.event.visibleSecurityStateChanged/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct VisibleSecurityStateChangedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.event.visibleSecurityStateChanged/parameters/visibleSecurityState。
    ///
    ///协议原文：
    ///
    ///<pre>Security state information about the page.</pre>
    ///
    ///
    #[serde(rename = "visibleSecurityState")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub visible_security_state: crate::security::types::VisibleSecurityState,
}
impl<'de> serde::Deserialize<'de> for VisibleSecurityStateChangedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = VisibleSecurityStateChangedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.event.visibleSecurityStateChanged/parameters/visibleSecurityState。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Security state information about the page.</pre>
                    ///
                    ///
                    #[serde(rename = "visibleSecurityState")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub visible_security_state: crate::security::types::VisibleSecurityState,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(VisibleSecurityStateChangedParams {
                    visible_security_state: fields.visible_security_state,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Security.event.visibleSecurityStateChanged。
///
///协议原文：
///
///<pre>The security state of the page changed.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct VisibleSecurityStateChanged;
impl ydp_core::Event for VisibleSecurityStateChanged {
    type Params = VisibleSecurityStateChangedParams;
    const METHOD: &'static str = "Security.visibleSecurityStateChanged";
}
