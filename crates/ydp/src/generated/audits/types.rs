///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.AffectedCookie。
///
///协议原文：
///
///<pre>Information about a cookie that is affected by an inspector issue.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct AffectedCookie {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.AffectedCookie/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>The following three properties uniquely identify a cookie</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.AffectedCookie/properties/path。
    #[serde(rename = "path")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub path: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.AffectedCookie/properties/domain。
    #[serde(rename = "domain")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub domain: String,
}
impl<'de> serde::Deserialize<'de> for AffectedCookie {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AffectedCookie;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.AffectedCookie/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The following three properties uniquely identify a cookie</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.AffectedCookie/properties/path。
                    #[serde(rename = "path")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub path: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.AffectedCookie/properties/domain。
                    #[serde(rename = "domain")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub domain: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AffectedCookie {
                    name: fields.name,
                    path: fields.path,
                    domain: fields.domain,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.AffectedFrame。
///
///协议原文：
///
///<pre>Information about the frame affected by an inspector issue.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct AffectedFrame {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.AffectedFrame/properties/frameId。
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
}
impl<'de> serde::Deserialize<'de> for AffectedFrame {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AffectedFrame;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.AffectedFrame/properties/frameId。
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AffectedFrame {
                    frame_id: fields.frame_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.AffectedRequest。
///
///协议原文：
///
///<pre>Information about a request that is affected by an inspector issue.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct AffectedRequest {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.AffectedRequest/properties/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>The unique request id.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub request_id: Option<crate::network::types::RequestId>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.AffectedRequest/properties/url。
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
}
impl<'de> serde::Deserialize<'de> for AffectedRequest {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AffectedRequest;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.AffectedRequest/properties/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The unique request id.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub request_id: Option<crate::network::types::RequestId>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.AffectedRequest/properties/url。
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AffectedRequest {
                    request_id: fields.request_id,
                    url: fields.url,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.BlockedByResponseIssueDetails。
///
///协议原文：
///
///<pre>Details for a request that has been blocked with the BLOCKED_BY_RESPONSE
///code. Currently only used for COEP/COOP, but may be extended to include
///some CSP errors in the future.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct BlockedByResponseIssueDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.BlockedByResponseIssueDetails/properties/request。
    #[serde(rename = "request")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request: crate::audits::types::AffectedRequest,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.BlockedByResponseIssueDetails/properties/parentFrame。
    #[serde(rename = "parentFrame")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_frame: Option<crate::audits::types::AffectedFrame>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.BlockedByResponseIssueDetails/properties/blockedFrame。
    #[serde(rename = "blockedFrame")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub blocked_frame: Option<crate::audits::types::AffectedFrame>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.BlockedByResponseIssueDetails/properties/reason。
    #[serde(rename = "reason")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub reason: crate::audits::types::BlockedByResponseReason,
}
impl<'de> serde::Deserialize<'de> for BlockedByResponseIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BlockedByResponseIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.BlockedByResponseIssueDetails/properties/request。
                    #[serde(rename = "request")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request: crate::audits::types::AffectedRequest,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.BlockedByResponseIssueDetails/properties/parentFrame。
                    #[serde(rename = "parentFrame")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub parent_frame: Option<crate::audits::types::AffectedFrame>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.BlockedByResponseIssueDetails/properties/blockedFrame。
                    #[serde(rename = "blockedFrame")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub blocked_frame: Option<crate::audits::types::AffectedFrame>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.BlockedByResponseIssueDetails/properties/reason。
                    #[serde(rename = "reason")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub reason: crate::audits::types::BlockedByResponseReason,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(BlockedByResponseIssueDetails {
                    request: fields.request,
                    parent_frame: fields.parent_frame,
                    blocked_frame: fields.blocked_frame,
                    reason: fields.reason,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.type.BlockedByResponseReason。
///
///协议原文：
///
///<pre>Enum indicating the reason a response has been blocked. These reasons are
///refinements of the net error BLOCKED_BY_RESPONSE.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum BlockedByResponseReason {
    ///协议值 "CoepFrameResourceNeedsCoepHeader"。
    CoepFrameResourceNeedsCoepHeader,
    ///协议值 "CoopSandboxedIFrameCannotNavigateToCoopPage"。
    CoopSandboxedIFrameCannotNavigateToCoopPage,
    ///协议值 "CorpNotSameOrigin"。
    CorpNotSameOrigin,
    ///协议值 "CorpNotSameOriginAfterDefaultedToSameOriginByCoep"。
    CorpNotSameOriginAfterDefaultedToSameOriginByCoep,
    ///协议值 "CorpNotSameOriginAfterDefaultedToSameOriginByDip"。
    CorpNotSameOriginAfterDefaultedToSameOriginByDip,
    ///协议值 "CorpNotSameOriginAfterDefaultedToSameOriginByCoepAndDip"。
    CorpNotSameOriginAfterDefaultedToSameOriginByCoepAndDip,
    ///协议值 "CorpNotSameSite"。
    CorpNotSameSite,
    ///协议值 "SRIMessageSignatureMismatch"。
    SriMessageSignatureMismatch,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl BlockedByResponseReason {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::types::BlockedByResponseReason::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::CoepFrameResourceNeedsCoepHeader => "CoepFrameResourceNeedsCoepHeader",
            Self::CoopSandboxedIFrameCannotNavigateToCoopPage => {
                "CoopSandboxedIFrameCannotNavigateToCoopPage"
            }
            Self::CorpNotSameOrigin => "CorpNotSameOrigin",
            Self::CorpNotSameOriginAfterDefaultedToSameOriginByCoep => {
                "CorpNotSameOriginAfterDefaultedToSameOriginByCoep"
            }
            Self::CorpNotSameOriginAfterDefaultedToSameOriginByDip => {
                "CorpNotSameOriginAfterDefaultedToSameOriginByDip"
            }
            Self::CorpNotSameOriginAfterDefaultedToSameOriginByCoepAndDip => {
                "CorpNotSameOriginAfterDefaultedToSameOriginByCoepAndDip"
            }
            Self::CorpNotSameSite => "CorpNotSameSite",
            Self::SriMessageSignatureMismatch => "SRIMessageSignatureMismatch",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for BlockedByResponseReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for BlockedByResponseReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "CoepFrameResourceNeedsCoepHeader" => {
                BlockedByResponseReason::CoepFrameResourceNeedsCoepHeader
            }
            "CoopSandboxedIFrameCannotNavigateToCoopPage" => {
                BlockedByResponseReason::CoopSandboxedIFrameCannotNavigateToCoopPage
            }
            "CorpNotSameOrigin" => BlockedByResponseReason::CorpNotSameOrigin,
            "CorpNotSameOriginAfterDefaultedToSameOriginByCoep" => {
                BlockedByResponseReason::CorpNotSameOriginAfterDefaultedToSameOriginByCoep
            }
            "CorpNotSameOriginAfterDefaultedToSameOriginByDip" => {
                BlockedByResponseReason::CorpNotSameOriginAfterDefaultedToSameOriginByDip
            }
            "CorpNotSameOriginAfterDefaultedToSameOriginByCoepAndDip" => {
                BlockedByResponseReason::CorpNotSameOriginAfterDefaultedToSameOriginByCoepAndDip
            }
            "CorpNotSameSite" => BlockedByResponseReason::CorpNotSameSite,
            "SRIMessageSignatureMismatch" => BlockedByResponseReason::SriMessageSignatureMismatch,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for BlockedByResponseReason {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BlockedByResponseReason {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BlockedByResponseReason;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(
                    match v {
                        "CoepFrameResourceNeedsCoepHeader" => {
                            BlockedByResponseReason::CoepFrameResourceNeedsCoepHeader
                        }
                        "CoopSandboxedIFrameCannotNavigateToCoopPage" => {
                            BlockedByResponseReason::CoopSandboxedIFrameCannotNavigateToCoopPage
                        }
                        "CorpNotSameOrigin" => BlockedByResponseReason::CorpNotSameOrigin,
                        "CorpNotSameOriginAfterDefaultedToSameOriginByCoep" => {
                            BlockedByResponseReason::CorpNotSameOriginAfterDefaultedToSameOriginByCoep
                        }
                        "CorpNotSameOriginAfterDefaultedToSameOriginByDip" => {
                            BlockedByResponseReason::CorpNotSameOriginAfterDefaultedToSameOriginByDip
                        }
                        "CorpNotSameOriginAfterDefaultedToSameOriginByCoepAndDip" => {
                            BlockedByResponseReason::CorpNotSameOriginAfterDefaultedToSameOriginByCoepAndDip
                        }
                        "CorpNotSameSite" => BlockedByResponseReason::CorpNotSameSite,
                        "SRIMessageSignatureMismatch" => {
                            BlockedByResponseReason::SriMessageSignatureMismatch
                        }
                        _ => BlockedByResponseReason::Unknown(v.to_owned()),
                    },
                )
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(BlockedByResponseReason::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.BounceTrackingIssueDetails。
///
///协议原文：
///
///<pre>This issue warns about sites in the redirect chain of a finished navigation
///that may be flagged as trackers and have their state cleared if they don't
///receive a user interaction. Note that in this context 'site' means eTLD+1.
///For example, if the URL `https://example.test:80/bounce` was in the
///redirect chain, the site reported would be `example.test`.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct BounceTrackingIssueDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.BounceTrackingIssueDetails/properties/trackingSites。
    #[serde(rename = "trackingSites")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub tracking_sites: Vec<String>,
}
impl<'de> serde::Deserialize<'de> for BounceTrackingIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BounceTrackingIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.BounceTrackingIssueDetails/properties/trackingSites。
                    #[serde(rename = "trackingSites")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub tracking_sites: Vec<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(BounceTrackingIssueDetails {
                    tracking_sites: fields.tracking_sites,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.ClientHintIssueDetails。
///
///协议原文：
///
///<pre>This issue tracks client hints related issues. It's used to deprecate old
///features, encourage the use of new ones, and provide general guidance.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClientHintIssueDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.ClientHintIssueDetails/properties/sourceCodeLocation。
    #[serde(rename = "sourceCodeLocation")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub source_code_location: crate::audits::types::SourceCodeLocation,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.ClientHintIssueDetails/properties/clientHintIssueReason。
    #[serde(rename = "clientHintIssueReason")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub client_hint_issue_reason: crate::audits::types::ClientHintIssueReason,
}
impl<'de> serde::Deserialize<'de> for ClientHintIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClientHintIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.ClientHintIssueDetails/properties/sourceCodeLocation。
                    #[serde(rename = "sourceCodeLocation")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub source_code_location: crate::audits::types::SourceCodeLocation,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.ClientHintIssueDetails/properties/clientHintIssueReason。
                    #[serde(rename = "clientHintIssueReason")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub client_hint_issue_reason: crate::audits::types::ClientHintIssueReason,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ClientHintIssueDetails {
                    source_code_location: fields.source_code_location,
                    client_hint_issue_reason: fields.client_hint_issue_reason,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.type.ClientHintIssueReason。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ClientHintIssueReason {
    ///协议值 "MetaTagAllowListInvalidOrigin"。
    MetaTagAllowListInvalidOrigin,
    ///协议值 "MetaTagModifiedHTML"。
    MetaTagModifiedHtml,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ClientHintIssueReason {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::types::ClientHintIssueReason::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::MetaTagAllowListInvalidOrigin => "MetaTagAllowListInvalidOrigin",
            Self::MetaTagModifiedHtml => "MetaTagModifiedHTML",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ClientHintIssueReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ClientHintIssueReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "MetaTagAllowListInvalidOrigin" => ClientHintIssueReason::MetaTagAllowListInvalidOrigin,
            "MetaTagModifiedHTML" => ClientHintIssueReason::MetaTagModifiedHtml,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ClientHintIssueReason {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ClientHintIssueReason {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClientHintIssueReason;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "MetaTagAllowListInvalidOrigin" => {
                        ClientHintIssueReason::MetaTagAllowListInvalidOrigin
                    }
                    "MetaTagModifiedHTML" => ClientHintIssueReason::MetaTagModifiedHtml,
                    _ => ClientHintIssueReason::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ClientHintIssueReason::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.type.ConnectionAllowlistError。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConnectionAllowlistError {
    ///协议值 "InvalidHeader"。
    InvalidHeader,
    ///协议值 "MoreThanOneList"。
    MoreThanOneList,
    ///协议值 "ItemNotInnerList"。
    ItemNotInnerList,
    ///协议值 "InvalidAllowlistItemType"。
    InvalidAllowlistItemType,
    ///协议值 "ReportingEndpointNotToken"。
    ReportingEndpointNotToken,
    ///协议值 "InvalidUrlPattern"。
    InvalidUrlPattern,
    ///协议值 "IFrameAttributeLoosensEmbeddingRequirement"。
    IFrameAttributeLoosensEmbeddingRequirement,
    ///协议值 "InvalidAllowConnectionAllowlistFrom"。
    InvalidAllowConnectionAllowlistFrom,
    ///协议值 "EmbeddingRequirementNotSatisfied"。
    EmbeddingRequirementNotSatisfied,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ConnectionAllowlistError {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::types::ConnectionAllowlistError::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::InvalidHeader => "InvalidHeader",
            Self::MoreThanOneList => "MoreThanOneList",
            Self::ItemNotInnerList => "ItemNotInnerList",
            Self::InvalidAllowlistItemType => "InvalidAllowlistItemType",
            Self::ReportingEndpointNotToken => "ReportingEndpointNotToken",
            Self::InvalidUrlPattern => "InvalidUrlPattern",
            Self::IFrameAttributeLoosensEmbeddingRequirement => {
                "IFrameAttributeLoosensEmbeddingRequirement"
            }
            Self::InvalidAllowConnectionAllowlistFrom => "InvalidAllowConnectionAllowlistFrom",
            Self::EmbeddingRequirementNotSatisfied => "EmbeddingRequirementNotSatisfied",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ConnectionAllowlistError {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ConnectionAllowlistError {
    fn from(value: String) -> Self {
        match value.as_str() {
            "InvalidHeader" => ConnectionAllowlistError::InvalidHeader,
            "MoreThanOneList" => ConnectionAllowlistError::MoreThanOneList,
            "ItemNotInnerList" => ConnectionAllowlistError::ItemNotInnerList,
            "InvalidAllowlistItemType" => ConnectionAllowlistError::InvalidAllowlistItemType,
            "ReportingEndpointNotToken" => ConnectionAllowlistError::ReportingEndpointNotToken,
            "InvalidUrlPattern" => ConnectionAllowlistError::InvalidUrlPattern,
            "IFrameAttributeLoosensEmbeddingRequirement" => {
                ConnectionAllowlistError::IFrameAttributeLoosensEmbeddingRequirement
            }
            "InvalidAllowConnectionAllowlistFrom" => {
                ConnectionAllowlistError::InvalidAllowConnectionAllowlistFrom
            }
            "EmbeddingRequirementNotSatisfied" => {
                ConnectionAllowlistError::EmbeddingRequirementNotSatisfied
            }
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ConnectionAllowlistError {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ConnectionAllowlistError {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ConnectionAllowlistError;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "InvalidHeader" => ConnectionAllowlistError::InvalidHeader,
                    "MoreThanOneList" => ConnectionAllowlistError::MoreThanOneList,
                    "ItemNotInnerList" => ConnectionAllowlistError::ItemNotInnerList,
                    "InvalidAllowlistItemType" => {
                        ConnectionAllowlistError::InvalidAllowlistItemType
                    }
                    "ReportingEndpointNotToken" => {
                        ConnectionAllowlistError::ReportingEndpointNotToken
                    }
                    "InvalidUrlPattern" => ConnectionAllowlistError::InvalidUrlPattern,
                    "IFrameAttributeLoosensEmbeddingRequirement" => {
                        ConnectionAllowlistError::IFrameAttributeLoosensEmbeddingRequirement
                    }
                    "InvalidAllowConnectionAllowlistFrom" => {
                        ConnectionAllowlistError::InvalidAllowConnectionAllowlistFrom
                    }
                    "EmbeddingRequirementNotSatisfied" => {
                        ConnectionAllowlistError::EmbeddingRequirementNotSatisfied
                    }
                    _ => ConnectionAllowlistError::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ConnectionAllowlistError::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.ConnectionAllowlistIssueDetails。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ConnectionAllowlistIssueDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.ConnectionAllowlistIssueDetails/properties/error。
    #[serde(rename = "error")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub error: crate::audits::types::ConnectionAllowlistError,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.ConnectionAllowlistIssueDetails/properties/request。
    #[serde(rename = "request")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request: crate::audits::types::AffectedRequest,
}
impl<'de> serde::Deserialize<'de> for ConnectionAllowlistIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ConnectionAllowlistIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.ConnectionAllowlistIssueDetails/properties/error。
                    #[serde(rename = "error")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub error: crate::audits::types::ConnectionAllowlistError,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.ConnectionAllowlistIssueDetails/properties/request。
                    #[serde(rename = "request")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request: crate::audits::types::AffectedRequest,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ConnectionAllowlistIssueDetails {
                    error: fields.error,
                    request: fields.request,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.ContentSecurityPolicyIssueDetails。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ContentSecurityPolicyIssueDetails {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.ContentSecurityPolicyIssueDetails/properties/blockedURL。
    ///
    ///协议原文：
    ///
    ///<pre>The url not included in allowed sources.</pre>
    ///
    ///
    #[serde(rename = "blockedURL")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub blocked_url: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.ContentSecurityPolicyIssueDetails/properties/violatedDirective。
    ///
    ///协议原文：
    ///
    ///<pre>Specific directive that is violated, causing the CSP issue.</pre>
    ///
    ///
    #[serde(rename = "violatedDirective")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub violated_directive: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.ContentSecurityPolicyIssueDetails/properties/isReportOnly。
    #[serde(rename = "isReportOnly")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub is_report_only: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.ContentSecurityPolicyIssueDetails/properties/contentSecurityPolicyViolationType。
    #[serde(rename = "contentSecurityPolicyViolationType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub content_security_policy_violation_type:
        crate::audits::types::ContentSecurityPolicyViolationType,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.ContentSecurityPolicyIssueDetails/properties/frameAncestor。
    #[serde(rename = "frameAncestor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub frame_ancestor: Option<crate::audits::types::AffectedFrame>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.ContentSecurityPolicyIssueDetails/properties/sourceCodeLocation。
    #[serde(rename = "sourceCodeLocation")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_code_location: Option<crate::audits::types::SourceCodeLocation>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.ContentSecurityPolicyIssueDetails/properties/violatingNodeId。
    #[serde(rename = "violatingNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub violating_node_id: Option<crate::dom::types::BackendNodeId>,
}
impl<'de> serde::Deserialize<'de> for ContentSecurityPolicyIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContentSecurityPolicyIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.ContentSecurityPolicyIssueDetails/properties/blockedURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The url not included in allowed sources.</pre>
                    ///
                    ///
                    #[serde(rename = "blockedURL")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub blocked_url: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.ContentSecurityPolicyIssueDetails/properties/violatedDirective。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specific directive that is violated, causing the CSP issue.</pre>
                    ///
                    ///
                    #[serde(rename = "violatedDirective")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub violated_directive: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.ContentSecurityPolicyIssueDetails/properties/isReportOnly。
                    #[serde(rename = "isReportOnly")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub is_report_only: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.ContentSecurityPolicyIssueDetails/properties/contentSecurityPolicyViolationType。
                    #[serde(rename = "contentSecurityPolicyViolationType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub content_security_policy_violation_type:
                        crate::audits::types::ContentSecurityPolicyViolationType,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.ContentSecurityPolicyIssueDetails/properties/frameAncestor。
                    #[serde(rename = "frameAncestor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub frame_ancestor: Option<crate::audits::types::AffectedFrame>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.ContentSecurityPolicyIssueDetails/properties/sourceCodeLocation。
                    #[serde(rename = "sourceCodeLocation")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub source_code_location: Option<crate::audits::types::SourceCodeLocation>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.ContentSecurityPolicyIssueDetails/properties/violatingNodeId。
                    #[serde(rename = "violatingNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub violating_node_id: Option<crate::dom::types::BackendNodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ContentSecurityPolicyIssueDetails {
                    blocked_url: fields.blocked_url,
                    violated_directive: fields.violated_directive,
                    is_report_only: fields.is_report_only,
                    content_security_policy_violation_type: fields
                        .content_security_policy_violation_type,
                    frame_ancestor: fields.frame_ancestor,
                    source_code_location: fields.source_code_location,
                    violating_node_id: fields.violating_node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.type.ContentSecurityPolicyViolationType。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ContentSecurityPolicyViolationType {
    ///协议值 "kInlineViolation"。
    KInlineViolation,
    ///协议值 "kEvalViolation"。
    KEvalViolation,
    ///协议值 "kURLViolation"。
    KUrlViolation,
    ///协议值 "kSRIViolation"。
    KSriViolation,
    ///协议值 "kTrustedTypesSinkViolation"。
    KTrustedTypesSinkViolation,
    ///协议值 "kTrustedTypesPolicyViolation"。
    KTrustedTypesPolicyViolation,
    ///协议值 "kWasmEvalViolation"。
    KWasmEvalViolation,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ContentSecurityPolicyViolationType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::types::ContentSecurityPolicyViolationType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::KInlineViolation => "kInlineViolation",
            Self::KEvalViolation => "kEvalViolation",
            Self::KUrlViolation => "kURLViolation",
            Self::KSriViolation => "kSRIViolation",
            Self::KTrustedTypesSinkViolation => "kTrustedTypesSinkViolation",
            Self::KTrustedTypesPolicyViolation => "kTrustedTypesPolicyViolation",
            Self::KWasmEvalViolation => "kWasmEvalViolation",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ContentSecurityPolicyViolationType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ContentSecurityPolicyViolationType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "kInlineViolation" => ContentSecurityPolicyViolationType::KInlineViolation,
            "kEvalViolation" => ContentSecurityPolicyViolationType::KEvalViolation,
            "kURLViolation" => ContentSecurityPolicyViolationType::KUrlViolation,
            "kSRIViolation" => ContentSecurityPolicyViolationType::KSriViolation,
            "kTrustedTypesSinkViolation" => {
                ContentSecurityPolicyViolationType::KTrustedTypesSinkViolation
            }
            "kTrustedTypesPolicyViolation" => {
                ContentSecurityPolicyViolationType::KTrustedTypesPolicyViolation
            }
            "kWasmEvalViolation" => ContentSecurityPolicyViolationType::KWasmEvalViolation,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ContentSecurityPolicyViolationType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ContentSecurityPolicyViolationType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContentSecurityPolicyViolationType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "kInlineViolation" => ContentSecurityPolicyViolationType::KInlineViolation,
                    "kEvalViolation" => ContentSecurityPolicyViolationType::KEvalViolation,
                    "kURLViolation" => ContentSecurityPolicyViolationType::KUrlViolation,
                    "kSRIViolation" => ContentSecurityPolicyViolationType::KSriViolation,
                    "kTrustedTypesSinkViolation" => {
                        ContentSecurityPolicyViolationType::KTrustedTypesSinkViolation
                    }
                    "kTrustedTypesPolicyViolation" => {
                        ContentSecurityPolicyViolationType::KTrustedTypesPolicyViolation
                    }
                    "kWasmEvalViolation" => ContentSecurityPolicyViolationType::KWasmEvalViolation,
                    _ => ContentSecurityPolicyViolationType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ContentSecurityPolicyViolationType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.CookieDeprecationMetadataIssueDetails。
///
///协议原文：
///
///<pre>This issue warns about third-party sites that are accessing cookies on the
///current page, and have been permitted due to having a global metadata grant.
///Note that in this context 'site' means eTLD+1. For example, if the URL
///`https://example.test:80/web_page` was accessing cookies, the site reported
///would be `example.test`.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CookieDeprecationMetadataIssueDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.CookieDeprecationMetadataIssueDetails/properties/allowedSites。
    #[serde(rename = "allowedSites")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub allowed_sites: Vec<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.CookieDeprecationMetadataIssueDetails/properties/optOutPercentage。
    #[serde(rename = "optOutPercentage")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub opt_out_percentage: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.CookieDeprecationMetadataIssueDetails/properties/isOptOutTopLevel。
    #[serde(rename = "isOptOutTopLevel")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub is_opt_out_top_level: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.CookieDeprecationMetadataIssueDetails/properties/operation。
    #[serde(rename = "operation")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub operation: crate::audits::types::CookieOperation,
}
impl<'de> serde::Deserialize<'de> for CookieDeprecationMetadataIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CookieDeprecationMetadataIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.CookieDeprecationMetadataIssueDetails/properties/allowedSites。
                    #[serde(rename = "allowedSites")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub allowed_sites: Vec<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.CookieDeprecationMetadataIssueDetails/properties/optOutPercentage。
                    #[serde(rename = "optOutPercentage")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub opt_out_percentage: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.CookieDeprecationMetadataIssueDetails/properties/isOptOutTopLevel。
                    #[serde(rename = "isOptOutTopLevel")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub is_opt_out_top_level: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.CookieDeprecationMetadataIssueDetails/properties/operation。
                    #[serde(rename = "operation")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub operation: crate::audits::types::CookieOperation,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CookieDeprecationMetadataIssueDetails {
                    allowed_sites: fields.allowed_sites,
                    opt_out_percentage: fields.opt_out_percentage,
                    is_opt_out_top_level: fields.is_opt_out_top_level,
                    operation: fields.operation,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.type.CookieExclusionReason。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CookieExclusionReason {
    ///协议值 "ExcludeSameSiteUnspecifiedTreatedAsLax"。
    ExcludeSameSiteUnspecifiedTreatedAsLax,
    ///协议值 "ExcludeSameSiteNoneInsecure"。
    ExcludeSameSiteNoneInsecure,
    ///协议值 "ExcludeSameSiteLax"。
    ExcludeSameSiteLax,
    ///协议值 "ExcludeSameSiteStrict"。
    ExcludeSameSiteStrict,
    ///协议值 "ExcludeDomainNonASCII"。
    ExcludeDomainNonAscii,
    ///协议值 "ExcludeThirdPartyCookieBlockedInFirstPartySet"。
    ExcludeThirdPartyCookieBlockedInFirstPartySet,
    ///协议值 "ExcludeThirdPartyPhaseout"。
    ExcludeThirdPartyPhaseout,
    ///协议值 "ExcludePortMismatch"。
    ExcludePortMismatch,
    ///协议值 "ExcludeSchemeMismatch"。
    ExcludeSchemeMismatch,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl CookieExclusionReason {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::types::CookieExclusionReason::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::ExcludeSameSiteUnspecifiedTreatedAsLax => {
                "ExcludeSameSiteUnspecifiedTreatedAsLax"
            }
            Self::ExcludeSameSiteNoneInsecure => "ExcludeSameSiteNoneInsecure",
            Self::ExcludeSameSiteLax => "ExcludeSameSiteLax",
            Self::ExcludeSameSiteStrict => "ExcludeSameSiteStrict",
            Self::ExcludeDomainNonAscii => "ExcludeDomainNonASCII",
            Self::ExcludeThirdPartyCookieBlockedInFirstPartySet => {
                "ExcludeThirdPartyCookieBlockedInFirstPartySet"
            }
            Self::ExcludeThirdPartyPhaseout => "ExcludeThirdPartyPhaseout",
            Self::ExcludePortMismatch => "ExcludePortMismatch",
            Self::ExcludeSchemeMismatch => "ExcludeSchemeMismatch",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for CookieExclusionReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for CookieExclusionReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ExcludeSameSiteUnspecifiedTreatedAsLax" => {
                CookieExclusionReason::ExcludeSameSiteUnspecifiedTreatedAsLax
            }
            "ExcludeSameSiteNoneInsecure" => CookieExclusionReason::ExcludeSameSiteNoneInsecure,
            "ExcludeSameSiteLax" => CookieExclusionReason::ExcludeSameSiteLax,
            "ExcludeSameSiteStrict" => CookieExclusionReason::ExcludeSameSiteStrict,
            "ExcludeDomainNonASCII" => CookieExclusionReason::ExcludeDomainNonAscii,
            "ExcludeThirdPartyCookieBlockedInFirstPartySet" => {
                CookieExclusionReason::ExcludeThirdPartyCookieBlockedInFirstPartySet
            }
            "ExcludeThirdPartyPhaseout" => CookieExclusionReason::ExcludeThirdPartyPhaseout,
            "ExcludePortMismatch" => CookieExclusionReason::ExcludePortMismatch,
            "ExcludeSchemeMismatch" => CookieExclusionReason::ExcludeSchemeMismatch,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for CookieExclusionReason {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CookieExclusionReason {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CookieExclusionReason;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "ExcludeSameSiteUnspecifiedTreatedAsLax" => {
                        CookieExclusionReason::ExcludeSameSiteUnspecifiedTreatedAsLax
                    }
                    "ExcludeSameSiteNoneInsecure" => {
                        CookieExclusionReason::ExcludeSameSiteNoneInsecure
                    }
                    "ExcludeSameSiteLax" => CookieExclusionReason::ExcludeSameSiteLax,
                    "ExcludeSameSiteStrict" => CookieExclusionReason::ExcludeSameSiteStrict,
                    "ExcludeDomainNonASCII" => CookieExclusionReason::ExcludeDomainNonAscii,
                    "ExcludeThirdPartyCookieBlockedInFirstPartySet" => {
                        CookieExclusionReason::ExcludeThirdPartyCookieBlockedInFirstPartySet
                    }
                    "ExcludeThirdPartyPhaseout" => CookieExclusionReason::ExcludeThirdPartyPhaseout,
                    "ExcludePortMismatch" => CookieExclusionReason::ExcludePortMismatch,
                    "ExcludeSchemeMismatch" => CookieExclusionReason::ExcludeSchemeMismatch,
                    _ => CookieExclusionReason::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(CookieExclusionReason::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.CookieIssueDetails。
///
///协议原文：
///
///<pre>This information is currently necessary, as the front-end has a difficult
///time finding a specific cookie. With this, we can convey specific error
///information without the cookie.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CookieIssueDetails {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.CookieIssueDetails/properties/cookie。
    ///
    ///协议原文：
    ///
    ///<pre>If AffectedCookie is not set then rawCookieLine contains the raw
    ///Set-Cookie header string. This hints at a problem where the
    ///cookie line is syntactically or semantically malformed in a way
    ///that no valid cookie could be created.</pre>
    ///
    ///
    #[serde(rename = "cookie")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub cookie: Option<crate::audits::types::AffectedCookie>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.CookieIssueDetails/properties/rawCookieLine。
    #[serde(rename = "rawCookieLine")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub raw_cookie_line: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.CookieIssueDetails/properties/cookieWarningReasons。
    #[serde(rename = "cookieWarningReasons")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cookie_warning_reasons: Vec<crate::audits::types::CookieWarningReason>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.CookieIssueDetails/properties/cookieExclusionReasons。
    #[serde(rename = "cookieExclusionReasons")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cookie_exclusion_reasons: Vec<crate::audits::types::CookieExclusionReason>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.CookieIssueDetails/properties/operation。
    ///
    ///协议原文：
    ///
    ///<pre>Optionally identifies the site-for-cookies and the cookie url, which
    ///may be used by the front-end as additional context.</pre>
    ///
    ///
    #[serde(rename = "operation")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub operation: crate::audits::types::CookieOperation,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.CookieIssueDetails/properties/siteForCookies。
    #[serde(rename = "siteForCookies")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub site_for_cookies: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.CookieIssueDetails/properties/cookieUrl。
    #[serde(rename = "cookieUrl")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub cookie_url: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.CookieIssueDetails/properties/request。
    #[serde(rename = "request")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub request: Option<crate::audits::types::AffectedRequest>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.CookieIssueDetails/properties/insight。
    ///
    ///协议原文：
    ///
    ///<pre>The recommended solution to the issue.</pre>
    ///
    ///
    #[serde(rename = "insight")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub insight: Option<crate::audits::types::CookieIssueInsight>,
}
impl<'de> serde::Deserialize<'de> for CookieIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CookieIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.CookieIssueDetails/properties/cookie。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If AffectedCookie is not set then rawCookieLine contains the raw
                    ///Set-Cookie header string. This hints at a problem where the
                    ///cookie line is syntactically or semantically malformed in a way
                    ///that no valid cookie could be created.</pre>
                    ///
                    ///
                    #[serde(rename = "cookie")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub cookie: Option<crate::audits::types::AffectedCookie>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.CookieIssueDetails/properties/rawCookieLine。
                    #[serde(rename = "rawCookieLine")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub raw_cookie_line: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.CookieIssueDetails/properties/cookieWarningReasons。
                    #[serde(rename = "cookieWarningReasons")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cookie_warning_reasons: Vec<crate::audits::types::CookieWarningReason>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.CookieIssueDetails/properties/cookieExclusionReasons。
                    #[serde(rename = "cookieExclusionReasons")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cookie_exclusion_reasons: Vec<crate::audits::types::CookieExclusionReason>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.CookieIssueDetails/properties/operation。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Optionally identifies the site-for-cookies and the cookie url, which
                    ///may be used by the front-end as additional context.</pre>
                    ///
                    ///
                    #[serde(rename = "operation")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub operation: crate::audits::types::CookieOperation,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.CookieIssueDetails/properties/siteForCookies。
                    #[serde(rename = "siteForCookies")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub site_for_cookies: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.CookieIssueDetails/properties/cookieUrl。
                    #[serde(rename = "cookieUrl")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub cookie_url: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.CookieIssueDetails/properties/request。
                    #[serde(rename = "request")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub request: Option<crate::audits::types::AffectedRequest>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.CookieIssueDetails/properties/insight。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The recommended solution to the issue.</pre>
                    ///
                    ///
                    #[serde(rename = "insight")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub insight: Option<crate::audits::types::CookieIssueInsight>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CookieIssueDetails {
                    cookie: fields.cookie,
                    raw_cookie_line: fields.raw_cookie_line,
                    cookie_warning_reasons: fields.cookie_warning_reasons,
                    cookie_exclusion_reasons: fields.cookie_exclusion_reasons,
                    operation: fields.operation,
                    site_for_cookies: fields.site_for_cookies,
                    cookie_url: fields.cookie_url,
                    request: fields.request,
                    insight: fields.insight,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.CookieIssueInsight。
///
///协议原文：
///
///<pre>Information about the suggested solution to a cookie issue.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CookieIssueInsight {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.CookieIssueInsight/properties/type。
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: crate::audits::types::InsightType,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.CookieIssueInsight/properties/tableEntryUrl。
    ///
    ///协议原文：
    ///
    ///<pre>Link to table entry in third-party cookie migration readiness list.</pre>
    ///
    ///
    #[serde(rename = "tableEntryUrl")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub table_entry_url: Option<String>,
}
impl<'de> serde::Deserialize<'de> for CookieIssueInsight {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CookieIssueInsight;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.CookieIssueInsight/properties/type。
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: crate::audits::types::InsightType,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.CookieIssueInsight/properties/tableEntryUrl。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Link to table entry in third-party cookie migration readiness list.</pre>
                    ///
                    ///
                    #[serde(rename = "tableEntryUrl")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub table_entry_url: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CookieIssueInsight {
                    r#type: fields.r#type,
                    table_entry_url: fields.table_entry_url,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.type.CookieOperation。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CookieOperation {
    ///协议值 "SetCookie"。
    SetCookie,
    ///协议值 "ReadCookie"。
    ReadCookie,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl CookieOperation {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::types::CookieOperation::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::SetCookie => "SetCookie",
            Self::ReadCookie => "ReadCookie",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for CookieOperation {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for CookieOperation {
    fn from(value: String) -> Self {
        match value.as_str() {
            "SetCookie" => CookieOperation::SetCookie,
            "ReadCookie" => CookieOperation::ReadCookie,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for CookieOperation {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CookieOperation {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CookieOperation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "SetCookie" => CookieOperation::SetCookie,
                    "ReadCookie" => CookieOperation::ReadCookie,
                    _ => CookieOperation::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(CookieOperation::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.type.CookieWarningReason。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CookieWarningReason {
    ///协议值 "WarnSameSiteUnspecifiedCrossSiteContext"。
    WarnSameSiteUnspecifiedCrossSiteContext,
    ///协议值 "WarnSameSiteNoneInsecure"。
    WarnSameSiteNoneInsecure,
    ///协议值 "WarnSameSiteUnspecifiedLaxAllowUnsafe"。
    WarnSameSiteUnspecifiedLaxAllowUnsafe,
    ///协议值 "WarnSameSiteStrictLaxDowngradeStrict"。
    WarnSameSiteStrictLaxDowngradeStrict,
    ///协议值 "WarnSameSiteStrictCrossDowngradeStrict"。
    WarnSameSiteStrictCrossDowngradeStrict,
    ///协议值 "WarnSameSiteStrictCrossDowngradeLax"。
    WarnSameSiteStrictCrossDowngradeLax,
    ///协议值 "WarnSameSiteLaxCrossDowngradeStrict"。
    WarnSameSiteLaxCrossDowngradeStrict,
    ///协议值 "WarnSameSiteLaxCrossDowngradeLax"。
    WarnSameSiteLaxCrossDowngradeLax,
    ///协议值 "WarnAttributeValueExceedsMaxSize"。
    WarnAttributeValueExceedsMaxSize,
    ///协议值 "WarnDomainNonASCII"。
    WarnDomainNonAscii,
    ///协议值 "WarnThirdPartyPhaseout"。
    WarnThirdPartyPhaseout,
    ///协议值 "WarnCrossSiteRedirectDowngradeChangesInclusion"。
    WarnCrossSiteRedirectDowngradeChangesInclusion,
    ///协议值 "WarnDeprecationTrialMetadata"。
    WarnDeprecationTrialMetadata,
    ///协议值 "WarnThirdPartyCookieHeuristic"。
    WarnThirdPartyCookieHeuristic,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl CookieWarningReason {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::types::CookieWarningReason::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::WarnSameSiteUnspecifiedCrossSiteContext => {
                "WarnSameSiteUnspecifiedCrossSiteContext"
            }
            Self::WarnSameSiteNoneInsecure => "WarnSameSiteNoneInsecure",
            Self::WarnSameSiteUnspecifiedLaxAllowUnsafe => "WarnSameSiteUnspecifiedLaxAllowUnsafe",
            Self::WarnSameSiteStrictLaxDowngradeStrict => "WarnSameSiteStrictLaxDowngradeStrict",
            Self::WarnSameSiteStrictCrossDowngradeStrict => {
                "WarnSameSiteStrictCrossDowngradeStrict"
            }
            Self::WarnSameSiteStrictCrossDowngradeLax => "WarnSameSiteStrictCrossDowngradeLax",
            Self::WarnSameSiteLaxCrossDowngradeStrict => "WarnSameSiteLaxCrossDowngradeStrict",
            Self::WarnSameSiteLaxCrossDowngradeLax => "WarnSameSiteLaxCrossDowngradeLax",
            Self::WarnAttributeValueExceedsMaxSize => "WarnAttributeValueExceedsMaxSize",
            Self::WarnDomainNonAscii => "WarnDomainNonASCII",
            Self::WarnThirdPartyPhaseout => "WarnThirdPartyPhaseout",
            Self::WarnCrossSiteRedirectDowngradeChangesInclusion => {
                "WarnCrossSiteRedirectDowngradeChangesInclusion"
            }
            Self::WarnDeprecationTrialMetadata => "WarnDeprecationTrialMetadata",
            Self::WarnThirdPartyCookieHeuristic => "WarnThirdPartyCookieHeuristic",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for CookieWarningReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for CookieWarningReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "WarnSameSiteUnspecifiedCrossSiteContext" => {
                CookieWarningReason::WarnSameSiteUnspecifiedCrossSiteContext
            }
            "WarnSameSiteNoneInsecure" => CookieWarningReason::WarnSameSiteNoneInsecure,
            "WarnSameSiteUnspecifiedLaxAllowUnsafe" => {
                CookieWarningReason::WarnSameSiteUnspecifiedLaxAllowUnsafe
            }
            "WarnSameSiteStrictLaxDowngradeStrict" => {
                CookieWarningReason::WarnSameSiteStrictLaxDowngradeStrict
            }
            "WarnSameSiteStrictCrossDowngradeStrict" => {
                CookieWarningReason::WarnSameSiteStrictCrossDowngradeStrict
            }
            "WarnSameSiteStrictCrossDowngradeLax" => {
                CookieWarningReason::WarnSameSiteStrictCrossDowngradeLax
            }
            "WarnSameSiteLaxCrossDowngradeStrict" => {
                CookieWarningReason::WarnSameSiteLaxCrossDowngradeStrict
            }
            "WarnSameSiteLaxCrossDowngradeLax" => {
                CookieWarningReason::WarnSameSiteLaxCrossDowngradeLax
            }
            "WarnAttributeValueExceedsMaxSize" => {
                CookieWarningReason::WarnAttributeValueExceedsMaxSize
            }
            "WarnDomainNonASCII" => CookieWarningReason::WarnDomainNonAscii,
            "WarnThirdPartyPhaseout" => CookieWarningReason::WarnThirdPartyPhaseout,
            "WarnCrossSiteRedirectDowngradeChangesInclusion" => {
                CookieWarningReason::WarnCrossSiteRedirectDowngradeChangesInclusion
            }
            "WarnDeprecationTrialMetadata" => CookieWarningReason::WarnDeprecationTrialMetadata,
            "WarnThirdPartyCookieHeuristic" => CookieWarningReason::WarnThirdPartyCookieHeuristic,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for CookieWarningReason {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CookieWarningReason {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CookieWarningReason;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "WarnSameSiteUnspecifiedCrossSiteContext" => {
                        CookieWarningReason::WarnSameSiteUnspecifiedCrossSiteContext
                    }
                    "WarnSameSiteNoneInsecure" => CookieWarningReason::WarnSameSiteNoneInsecure,
                    "WarnSameSiteUnspecifiedLaxAllowUnsafe" => {
                        CookieWarningReason::WarnSameSiteUnspecifiedLaxAllowUnsafe
                    }
                    "WarnSameSiteStrictLaxDowngradeStrict" => {
                        CookieWarningReason::WarnSameSiteStrictLaxDowngradeStrict
                    }
                    "WarnSameSiteStrictCrossDowngradeStrict" => {
                        CookieWarningReason::WarnSameSiteStrictCrossDowngradeStrict
                    }
                    "WarnSameSiteStrictCrossDowngradeLax" => {
                        CookieWarningReason::WarnSameSiteStrictCrossDowngradeLax
                    }
                    "WarnSameSiteLaxCrossDowngradeStrict" => {
                        CookieWarningReason::WarnSameSiteLaxCrossDowngradeStrict
                    }
                    "WarnSameSiteLaxCrossDowngradeLax" => {
                        CookieWarningReason::WarnSameSiteLaxCrossDowngradeLax
                    }
                    "WarnAttributeValueExceedsMaxSize" => {
                        CookieWarningReason::WarnAttributeValueExceedsMaxSize
                    }
                    "WarnDomainNonASCII" => CookieWarningReason::WarnDomainNonAscii,
                    "WarnThirdPartyPhaseout" => CookieWarningReason::WarnThirdPartyPhaseout,
                    "WarnCrossSiteRedirectDowngradeChangesInclusion" => {
                        CookieWarningReason::WarnCrossSiteRedirectDowngradeChangesInclusion
                    }
                    "WarnDeprecationTrialMetadata" => {
                        CookieWarningReason::WarnDeprecationTrialMetadata
                    }
                    "WarnThirdPartyCookieHeuristic" => {
                        CookieWarningReason::WarnThirdPartyCookieHeuristic
                    }
                    _ => CookieWarningReason::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(CookieWarningReason::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.CorsIssueDetails。
///
///协议原文：
///
///<pre>Details for a CORS related issue, e.g. a warning or error related to
///CORS RFC1918 enforcement.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CorsIssueDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.CorsIssueDetails/properties/corsErrorStatus。
    #[serde(rename = "corsErrorStatus")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cors_error_status: crate::network::types::CorsErrorStatus,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.CorsIssueDetails/properties/isWarning。
    #[serde(rename = "isWarning")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub is_warning: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.CorsIssueDetails/properties/request。
    #[serde(rename = "request")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request: crate::audits::types::AffectedRequest,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.CorsIssueDetails/properties/location。
    #[serde(rename = "location")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub location: Option<crate::audits::types::SourceCodeLocation>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.CorsIssueDetails/properties/initiatorOrigin。
    #[serde(rename = "initiatorOrigin")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub initiator_origin: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.CorsIssueDetails/properties/resourceIPAddressSpace。
    #[serde(rename = "resourceIPAddressSpace")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_ip_address_space: Option<crate::network::types::IpAddressSpace>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.CorsIssueDetails/properties/clientSecurityState。
    #[serde(rename = "clientSecurityState")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub client_security_state: Option<crate::network::types::ClientSecurityState>,
}
impl<'de> serde::Deserialize<'de> for CorsIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CorsIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.CorsIssueDetails/properties/corsErrorStatus。
                    #[serde(rename = "corsErrorStatus")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cors_error_status: crate::network::types::CorsErrorStatus,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.CorsIssueDetails/properties/isWarning。
                    #[serde(rename = "isWarning")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub is_warning: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.CorsIssueDetails/properties/request。
                    #[serde(rename = "request")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request: crate::audits::types::AffectedRequest,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.CorsIssueDetails/properties/location。
                    #[serde(rename = "location")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub location: Option<crate::audits::types::SourceCodeLocation>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.CorsIssueDetails/properties/initiatorOrigin。
                    #[serde(rename = "initiatorOrigin")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub initiator_origin: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.CorsIssueDetails/properties/resourceIPAddressSpace。
                    #[serde(rename = "resourceIPAddressSpace")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub resource_ip_address_space: Option<crate::network::types::IpAddressSpace>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.CorsIssueDetails/properties/clientSecurityState。
                    #[serde(rename = "clientSecurityState")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub client_security_state: Option<crate::network::types::ClientSecurityState>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CorsIssueDetails {
                    cors_error_status: fields.cors_error_status,
                    is_warning: fields.is_warning,
                    request: fields.request,
                    location: fields.location,
                    initiator_origin: fields.initiator_origin,
                    resource_ip_address_space: fields.resource_ip_address_space,
                    client_security_state: fields.client_security_state,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.DeprecationIssueDetails。
///
///协议原文：
///
///<pre>This issue tracks information needed to print a deprecation message.
///https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/frame/third_party/blink/renderer/core/frame/deprecation/README.md</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeprecationIssueDetails {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.DeprecationIssueDetails/properties/affectedFrame。
    #[serde(rename = "affectedFrame")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub affected_frame: Option<crate::audits::types::AffectedFrame>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.DeprecationIssueDetails/properties/sourceCodeLocation。
    #[serde(rename = "sourceCodeLocation")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub source_code_location: crate::audits::types::SourceCodeLocation,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.DeprecationIssueDetails/properties/type。
    ///
    ///协议原文：
    ///
    ///<pre>One of the deprecation names from third_party/blink/renderer/core/frame/deprecation/deprecation.json5</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: String,
}
impl<'de> serde::Deserialize<'de> for DeprecationIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeprecationIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.DeprecationIssueDetails/properties/affectedFrame。
                    #[serde(rename = "affectedFrame")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub affected_frame: Option<crate::audits::types::AffectedFrame>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.DeprecationIssueDetails/properties/sourceCodeLocation。
                    #[serde(rename = "sourceCodeLocation")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub source_code_location: crate::audits::types::SourceCodeLocation,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.DeprecationIssueDetails/properties/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>One of the deprecation names from third_party/blink/renderer/core/frame/deprecation/deprecation.json5</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DeprecationIssueDetails {
                    affected_frame: fields.affected_frame,
                    source_code_location: fields.source_code_location,
                    r#type: fields.r#type,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.ElementAccessibilityIssueDetails。
///
///协议原文：
///
///<pre>This issue warns about errors in the select or summary element content model.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ElementAccessibilityIssueDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.ElementAccessibilityIssueDetails/properties/nodeId。
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::BackendNodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.ElementAccessibilityIssueDetails/properties/elementAccessibilityIssueReason。
    #[serde(rename = "elementAccessibilityIssueReason")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub element_accessibility_issue_reason: crate::audits::types::ElementAccessibilityIssueReason,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.ElementAccessibilityIssueDetails/properties/hasDisallowedAttributes。
    #[serde(rename = "hasDisallowedAttributes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub has_disallowed_attributes: bool,
}
impl<'de> serde::Deserialize<'de> for ElementAccessibilityIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ElementAccessibilityIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.ElementAccessibilityIssueDetails/properties/nodeId。
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::BackendNodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.ElementAccessibilityIssueDetails/properties/elementAccessibilityIssueReason。
                    #[serde(rename = "elementAccessibilityIssueReason")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub element_accessibility_issue_reason:
                        crate::audits::types::ElementAccessibilityIssueReason,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.ElementAccessibilityIssueDetails/properties/hasDisallowedAttributes。
                    #[serde(rename = "hasDisallowedAttributes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub has_disallowed_attributes: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ElementAccessibilityIssueDetails {
                    node_id: fields.node_id,
                    element_accessibility_issue_reason: fields.element_accessibility_issue_reason,
                    has_disallowed_attributes: fields.has_disallowed_attributes,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.type.ElementAccessibilityIssueReason。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ElementAccessibilityIssueReason {
    ///协议值 "DisallowedSelectChild"。
    DisallowedSelectChild,
    ///协议值 "DisallowedOptGroupChild"。
    DisallowedOptGroupChild,
    ///协议值 "NonPhrasingContentOptionChild"。
    NonPhrasingContentOptionChild,
    ///协议值 "InteractiveContentOptionChild"。
    InteractiveContentOptionChild,
    ///协议值 "InteractiveContentLegendChild"。
    InteractiveContentLegendChild,
    ///协议值 "InteractiveContentSummaryDescendant"。
    InteractiveContentSummaryDescendant,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ElementAccessibilityIssueReason {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::types::ElementAccessibilityIssueReason::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::DisallowedSelectChild => "DisallowedSelectChild",
            Self::DisallowedOptGroupChild => "DisallowedOptGroupChild",
            Self::NonPhrasingContentOptionChild => "NonPhrasingContentOptionChild",
            Self::InteractiveContentOptionChild => "InteractiveContentOptionChild",
            Self::InteractiveContentLegendChild => "InteractiveContentLegendChild",
            Self::InteractiveContentSummaryDescendant => "InteractiveContentSummaryDescendant",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ElementAccessibilityIssueReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ElementAccessibilityIssueReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "DisallowedSelectChild" => ElementAccessibilityIssueReason::DisallowedSelectChild,
            "DisallowedOptGroupChild" => ElementAccessibilityIssueReason::DisallowedOptGroupChild,
            "NonPhrasingContentOptionChild" => {
                ElementAccessibilityIssueReason::NonPhrasingContentOptionChild
            }
            "InteractiveContentOptionChild" => {
                ElementAccessibilityIssueReason::InteractiveContentOptionChild
            }
            "InteractiveContentLegendChild" => {
                ElementAccessibilityIssueReason::InteractiveContentLegendChild
            }
            "InteractiveContentSummaryDescendant" => {
                ElementAccessibilityIssueReason::InteractiveContentSummaryDescendant
            }
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ElementAccessibilityIssueReason {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ElementAccessibilityIssueReason {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ElementAccessibilityIssueReason;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "DisallowedSelectChild" => {
                        ElementAccessibilityIssueReason::DisallowedSelectChild
                    }
                    "DisallowedOptGroupChild" => {
                        ElementAccessibilityIssueReason::DisallowedOptGroupChild
                    }
                    "NonPhrasingContentOptionChild" => {
                        ElementAccessibilityIssueReason::NonPhrasingContentOptionChild
                    }
                    "InteractiveContentOptionChild" => {
                        ElementAccessibilityIssueReason::InteractiveContentOptionChild
                    }
                    "InteractiveContentLegendChild" => {
                        ElementAccessibilityIssueReason::InteractiveContentLegendChild
                    }
                    "InteractiveContentSummaryDescendant" => {
                        ElementAccessibilityIssueReason::InteractiveContentSummaryDescendant
                    }
                    _ => ElementAccessibilityIssueReason::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ElementAccessibilityIssueReason::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.EmailVerificationRequestIssueDetails。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EmailVerificationRequestIssueDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.EmailVerificationRequestIssueDetails/properties/emailVerificationRequestIssueReason。
    #[serde(rename = "emailVerificationRequestIssueReason")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub email_verification_request_issue_reason:
        crate::audits::types::EmailVerificationRequestIssueReason,
}
impl<'de> serde::Deserialize<'de> for EmailVerificationRequestIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EmailVerificationRequestIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.EmailVerificationRequestIssueDetails/properties/emailVerificationRequestIssueReason。
                    #[serde(rename = "emailVerificationRequestIssueReason")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub email_verification_request_issue_reason:
                        crate::audits::types::EmailVerificationRequestIssueReason,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EmailVerificationRequestIssueDetails {
                    email_verification_request_issue_reason: fields
                        .email_verification_request_issue_reason,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.type.EmailVerificationRequestIssueReason。
///
///协议原文：
///
///<pre>Represents the failure reason when an email verification request fails.
///Should be updated alongside EmailVerificationRequestResult in
///third_party/blink/public/mojom/devtools/inspector_issue.mojom.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum EmailVerificationRequestIssueReason {
    ///协议值 "InvalidEmail"。
    InvalidEmail,
    ///协议值 "DnsFetchFailed"。
    DnsFetchFailed,
    ///协议值 "DnsInvalidRecord"。
    DnsInvalidRecord,
    ///协议值 "WellKnownHttpNotFound"。
    WellKnownHttpNotFound,
    ///协议值 "WellKnownNoResponse"。
    WellKnownNoResponse,
    ///协议值 "WellKnownInvalidResponse"。
    WellKnownInvalidResponse,
    ///协议值 "WellKnownListEmpty"。
    WellKnownListEmpty,
    ///协议值 "WellKnownInvalidContentType"。
    WellKnownInvalidContentType,
    ///协议值 "WellKnownMissingIssuanceEndpoint"。
    WellKnownMissingIssuanceEndpoint,
    ///协议值 "WellKnownIssuanceEndpointCrossOrigin"。
    WellKnownIssuanceEndpointCrossOrigin,
    ///协议值 "WellKnownUnsupportedSigningAlgorithm"。
    WellKnownUnsupportedSigningAlgorithm,
    ///协议值 "TokenHttpNotFound"。
    TokenHttpNotFound,
    ///协议值 "TokenNoResponse"。
    TokenNoResponse,
    ///协议值 "TokenInvalidResponse"。
    TokenInvalidResponse,
    ///协议值 "TokenInvalidContentType"。
    TokenInvalidContentType,
    ///协议值 "TokenMalformedSdJwt"。
    TokenMalformedSdJwt,
    ///协议值 "TokenInvalidSdJwt"。
    TokenInvalidSdJwt,
    ///协议值 "KeyBindingSigningFailed"。
    KeyBindingSigningFailed,
    ///协议值 "RpOriginIsOpaque"。
    RpOriginIsOpaque,
    ///协议值 "WellKnownMissingAccountsEndpoint"。
    WellKnownMissingAccountsEndpoint,
    ///协议值 "UserLoggedOut"。
    UserLoggedOut,
    ///协议值 "WellKnownAccountsEndpointCrossOrigin"。
    WellKnownAccountsEndpointCrossOrigin,
    ///协议值 "AccountsHttpNotFound"。
    AccountsHttpNotFound,
    ///协议值 "AccountsNoResponse"。
    AccountsNoResponse,
    ///协议值 "AccountsInvalidResponse"。
    AccountsInvalidResponse,
    ///协议值 "AccountsInvalidContentType"。
    AccountsInvalidContentType,
    ///协议值 "AccountsEmptyList"。
    AccountsEmptyList,
    ///协议值 "EmailVerificationWellKnownHttpNotFound"。
    EmailVerificationWellKnownHttpNotFound,
    ///协议值 "EmailVerificationWellKnownNoResponse"。
    EmailVerificationWellKnownNoResponse,
    ///协议值 "EmailVerificationWellKnownInvalidResponse"。
    EmailVerificationWellKnownInvalidResponse,
    ///协议值 "EmailVerificationWellKnownInvalidContentType"。
    EmailVerificationWellKnownInvalidContentType,
    ///协议值 "JwksHttpNotFound"。
    JwksHttpNotFound,
    ///协议值 "JwksInvalidResponse"。
    JwksInvalidResponse,
    ///协议值 "TokenVerificationSdJwtUnsupportedHeaderAlg"。
    TokenVerificationSdJwtUnsupportedHeaderAlg,
    ///协议值 "TokenVerificationSdJwtInvalidTyp"。
    TokenVerificationSdJwtInvalidTyp,
    ///协议值 "TokenVerificationSdJwtMissingIss"。
    TokenVerificationSdJwtMissingIss,
    ///协议值 "TokenVerificationSdJwtMissingIat"。
    TokenVerificationSdJwtMissingIat,
    ///协议值 "TokenVerificationSdJwtMissingCnf"。
    TokenVerificationSdJwtMissingCnf,
    ///协议值 "TokenVerificationSdJwtMissingEmail"。
    TokenVerificationSdJwtMissingEmail,
    ///协议值 "TokenVerificationSdJwtInvalidIssuedAt"。
    TokenVerificationSdJwtInvalidIssuedAt,
    ///协议值 "TokenVerificationSdJwtInvalidIssuer"。
    TokenVerificationSdJwtInvalidIssuer,
    ///协议值 "TokenVerificationSdJwtJwksMissingKeys"。
    TokenVerificationSdJwtJwksMissingKeys,
    ///协议值 "TokenVerificationSdJwtSignatureFailed"。
    TokenVerificationSdJwtSignatureFailed,
    ///协议值 "TokenVerificationSdJwtInvalidEmailVerified"。
    TokenVerificationSdJwtInvalidEmailVerified,
    ///协议值 "TokenVerificationSdJwtInvalidEmail"。
    TokenVerificationSdJwtInvalidEmail,
    ///协议值 "TokenVerificationSdJwtInvalidHolderKey"。
    TokenVerificationSdJwtInvalidHolderKey,
    ///协议值 "TokenVerificationKbInvalidTyp"。
    TokenVerificationKbInvalidTyp,
    ///协议值 "TokenVerificationKbMissingAud"。
    TokenVerificationKbMissingAud,
    ///协议值 "TokenVerificationKbMissingNonce"。
    TokenVerificationKbMissingNonce,
    ///协议值 "TokenVerificationKbMissingIat"。
    TokenVerificationKbMissingIat,
    ///协议值 "TokenVerificationKbMissingSdHash"。
    TokenVerificationKbMissingSdHash,
    ///协议值 "TokenVerificationKbInvalidIssuedAt"。
    TokenVerificationKbInvalidIssuedAt,
    ///协议值 "TokenVerificationKbInvalidAudience"。
    TokenVerificationKbInvalidAudience,
    ///协议值 "TokenVerificationKbInvalidNonce"。
    TokenVerificationKbInvalidNonce,
    ///协议值 "TokenVerificationKbInvalidSdHash"。
    TokenVerificationKbInvalidSdHash,
    ///协议值 "TokenVerificationKbMissingCnf"。
    TokenVerificationKbMissingCnf,
    ///协议值 "TokenVerificationKbSignatureFailed"。
    TokenVerificationKbSignatureFailed,
    ///协议值 "CrossOriginIframeNotSupported"。
    CrossOriginIframeNotSupported,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl EmailVerificationRequestIssueReason {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::types::EmailVerificationRequestIssueReason::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::InvalidEmail => "InvalidEmail",
            Self::DnsFetchFailed => "DnsFetchFailed",
            Self::DnsInvalidRecord => "DnsInvalidRecord",
            Self::WellKnownHttpNotFound => "WellKnownHttpNotFound",
            Self::WellKnownNoResponse => "WellKnownNoResponse",
            Self::WellKnownInvalidResponse => "WellKnownInvalidResponse",
            Self::WellKnownListEmpty => "WellKnownListEmpty",
            Self::WellKnownInvalidContentType => "WellKnownInvalidContentType",
            Self::WellKnownMissingIssuanceEndpoint => "WellKnownMissingIssuanceEndpoint",
            Self::WellKnownIssuanceEndpointCrossOrigin => "WellKnownIssuanceEndpointCrossOrigin",
            Self::WellKnownUnsupportedSigningAlgorithm => "WellKnownUnsupportedSigningAlgorithm",
            Self::TokenHttpNotFound => "TokenHttpNotFound",
            Self::TokenNoResponse => "TokenNoResponse",
            Self::TokenInvalidResponse => "TokenInvalidResponse",
            Self::TokenInvalidContentType => "TokenInvalidContentType",
            Self::TokenMalformedSdJwt => "TokenMalformedSdJwt",
            Self::TokenInvalidSdJwt => "TokenInvalidSdJwt",
            Self::KeyBindingSigningFailed => "KeyBindingSigningFailed",
            Self::RpOriginIsOpaque => "RpOriginIsOpaque",
            Self::WellKnownMissingAccountsEndpoint => "WellKnownMissingAccountsEndpoint",
            Self::UserLoggedOut => "UserLoggedOut",
            Self::WellKnownAccountsEndpointCrossOrigin => "WellKnownAccountsEndpointCrossOrigin",
            Self::AccountsHttpNotFound => "AccountsHttpNotFound",
            Self::AccountsNoResponse => "AccountsNoResponse",
            Self::AccountsInvalidResponse => "AccountsInvalidResponse",
            Self::AccountsInvalidContentType => "AccountsInvalidContentType",
            Self::AccountsEmptyList => "AccountsEmptyList",
            Self::EmailVerificationWellKnownHttpNotFound => {
                "EmailVerificationWellKnownHttpNotFound"
            }
            Self::EmailVerificationWellKnownNoResponse => "EmailVerificationWellKnownNoResponse",
            Self::EmailVerificationWellKnownInvalidResponse => {
                "EmailVerificationWellKnownInvalidResponse"
            }
            Self::EmailVerificationWellKnownInvalidContentType => {
                "EmailVerificationWellKnownInvalidContentType"
            }
            Self::JwksHttpNotFound => "JwksHttpNotFound",
            Self::JwksInvalidResponse => "JwksInvalidResponse",
            Self::TokenVerificationSdJwtUnsupportedHeaderAlg => {
                "TokenVerificationSdJwtUnsupportedHeaderAlg"
            }
            Self::TokenVerificationSdJwtInvalidTyp => "TokenVerificationSdJwtInvalidTyp",
            Self::TokenVerificationSdJwtMissingIss => "TokenVerificationSdJwtMissingIss",
            Self::TokenVerificationSdJwtMissingIat => "TokenVerificationSdJwtMissingIat",
            Self::TokenVerificationSdJwtMissingCnf => "TokenVerificationSdJwtMissingCnf",
            Self::TokenVerificationSdJwtMissingEmail => "TokenVerificationSdJwtMissingEmail",
            Self::TokenVerificationSdJwtInvalidIssuedAt => "TokenVerificationSdJwtInvalidIssuedAt",
            Self::TokenVerificationSdJwtInvalidIssuer => "TokenVerificationSdJwtInvalidIssuer",
            Self::TokenVerificationSdJwtJwksMissingKeys => "TokenVerificationSdJwtJwksMissingKeys",
            Self::TokenVerificationSdJwtSignatureFailed => "TokenVerificationSdJwtSignatureFailed",
            Self::TokenVerificationSdJwtInvalidEmailVerified => {
                "TokenVerificationSdJwtInvalidEmailVerified"
            }
            Self::TokenVerificationSdJwtInvalidEmail => "TokenVerificationSdJwtInvalidEmail",
            Self::TokenVerificationSdJwtInvalidHolderKey => {
                "TokenVerificationSdJwtInvalidHolderKey"
            }
            Self::TokenVerificationKbInvalidTyp => "TokenVerificationKbInvalidTyp",
            Self::TokenVerificationKbMissingAud => "TokenVerificationKbMissingAud",
            Self::TokenVerificationKbMissingNonce => "TokenVerificationKbMissingNonce",
            Self::TokenVerificationKbMissingIat => "TokenVerificationKbMissingIat",
            Self::TokenVerificationKbMissingSdHash => "TokenVerificationKbMissingSdHash",
            Self::TokenVerificationKbInvalidIssuedAt => "TokenVerificationKbInvalidIssuedAt",
            Self::TokenVerificationKbInvalidAudience => "TokenVerificationKbInvalidAudience",
            Self::TokenVerificationKbInvalidNonce => "TokenVerificationKbInvalidNonce",
            Self::TokenVerificationKbInvalidSdHash => "TokenVerificationKbInvalidSdHash",
            Self::TokenVerificationKbMissingCnf => "TokenVerificationKbMissingCnf",
            Self::TokenVerificationKbSignatureFailed => "TokenVerificationKbSignatureFailed",
            Self::CrossOriginIframeNotSupported => "CrossOriginIframeNotSupported",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for EmailVerificationRequestIssueReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for EmailVerificationRequestIssueReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "InvalidEmail" => EmailVerificationRequestIssueReason::InvalidEmail,
            "DnsFetchFailed" => EmailVerificationRequestIssueReason::DnsFetchFailed,
            "DnsInvalidRecord" => EmailVerificationRequestIssueReason::DnsInvalidRecord,
            "WellKnownHttpNotFound" => EmailVerificationRequestIssueReason::WellKnownHttpNotFound,
            "WellKnownNoResponse" => EmailVerificationRequestIssueReason::WellKnownNoResponse,
            "WellKnownInvalidResponse" => {
                EmailVerificationRequestIssueReason::WellKnownInvalidResponse
            }
            "WellKnownListEmpty" => EmailVerificationRequestIssueReason::WellKnownListEmpty,
            "WellKnownInvalidContentType" => {
                EmailVerificationRequestIssueReason::WellKnownInvalidContentType
            }
            "WellKnownMissingIssuanceEndpoint" => {
                EmailVerificationRequestIssueReason::WellKnownMissingIssuanceEndpoint
            }
            "WellKnownIssuanceEndpointCrossOrigin" => {
                EmailVerificationRequestIssueReason::WellKnownIssuanceEndpointCrossOrigin
            }
            "WellKnownUnsupportedSigningAlgorithm" => {
                EmailVerificationRequestIssueReason::WellKnownUnsupportedSigningAlgorithm
            }
            "TokenHttpNotFound" => EmailVerificationRequestIssueReason::TokenHttpNotFound,
            "TokenNoResponse" => EmailVerificationRequestIssueReason::TokenNoResponse,
            "TokenInvalidResponse" => EmailVerificationRequestIssueReason::TokenInvalidResponse,
            "TokenInvalidContentType" => {
                EmailVerificationRequestIssueReason::TokenInvalidContentType
            }
            "TokenMalformedSdJwt" => EmailVerificationRequestIssueReason::TokenMalformedSdJwt,
            "TokenInvalidSdJwt" => EmailVerificationRequestIssueReason::TokenInvalidSdJwt,
            "KeyBindingSigningFailed" => {
                EmailVerificationRequestIssueReason::KeyBindingSigningFailed
            }
            "RpOriginIsOpaque" => EmailVerificationRequestIssueReason::RpOriginIsOpaque,
            "WellKnownMissingAccountsEndpoint" => {
                EmailVerificationRequestIssueReason::WellKnownMissingAccountsEndpoint
            }
            "UserLoggedOut" => EmailVerificationRequestIssueReason::UserLoggedOut,
            "WellKnownAccountsEndpointCrossOrigin" => {
                EmailVerificationRequestIssueReason::WellKnownAccountsEndpointCrossOrigin
            }
            "AccountsHttpNotFound" => EmailVerificationRequestIssueReason::AccountsHttpNotFound,
            "AccountsNoResponse" => EmailVerificationRequestIssueReason::AccountsNoResponse,
            "AccountsInvalidResponse" => {
                EmailVerificationRequestIssueReason::AccountsInvalidResponse
            }
            "AccountsInvalidContentType" => {
                EmailVerificationRequestIssueReason::AccountsInvalidContentType
            }
            "AccountsEmptyList" => EmailVerificationRequestIssueReason::AccountsEmptyList,
            "EmailVerificationWellKnownHttpNotFound" => {
                EmailVerificationRequestIssueReason::EmailVerificationWellKnownHttpNotFound
            }
            "EmailVerificationWellKnownNoResponse" => {
                EmailVerificationRequestIssueReason::EmailVerificationWellKnownNoResponse
            }
            "EmailVerificationWellKnownInvalidResponse" => {
                EmailVerificationRequestIssueReason::EmailVerificationWellKnownInvalidResponse
            }
            "EmailVerificationWellKnownInvalidContentType" => {
                EmailVerificationRequestIssueReason::EmailVerificationWellKnownInvalidContentType
            }
            "JwksHttpNotFound" => EmailVerificationRequestIssueReason::JwksHttpNotFound,
            "JwksInvalidResponse" => EmailVerificationRequestIssueReason::JwksInvalidResponse,
            "TokenVerificationSdJwtUnsupportedHeaderAlg" => {
                EmailVerificationRequestIssueReason::TokenVerificationSdJwtUnsupportedHeaderAlg
            }
            "TokenVerificationSdJwtInvalidTyp" => {
                EmailVerificationRequestIssueReason::TokenVerificationSdJwtInvalidTyp
            }
            "TokenVerificationSdJwtMissingIss" => {
                EmailVerificationRequestIssueReason::TokenVerificationSdJwtMissingIss
            }
            "TokenVerificationSdJwtMissingIat" => {
                EmailVerificationRequestIssueReason::TokenVerificationSdJwtMissingIat
            }
            "TokenVerificationSdJwtMissingCnf" => {
                EmailVerificationRequestIssueReason::TokenVerificationSdJwtMissingCnf
            }
            "TokenVerificationSdJwtMissingEmail" => {
                EmailVerificationRequestIssueReason::TokenVerificationSdJwtMissingEmail
            }
            "TokenVerificationSdJwtInvalidIssuedAt" => {
                EmailVerificationRequestIssueReason::TokenVerificationSdJwtInvalidIssuedAt
            }
            "TokenVerificationSdJwtInvalidIssuer" => {
                EmailVerificationRequestIssueReason::TokenVerificationSdJwtInvalidIssuer
            }
            "TokenVerificationSdJwtJwksMissingKeys" => {
                EmailVerificationRequestIssueReason::TokenVerificationSdJwtJwksMissingKeys
            }
            "TokenVerificationSdJwtSignatureFailed" => {
                EmailVerificationRequestIssueReason::TokenVerificationSdJwtSignatureFailed
            }
            "TokenVerificationSdJwtInvalidEmailVerified" => {
                EmailVerificationRequestIssueReason::TokenVerificationSdJwtInvalidEmailVerified
            }
            "TokenVerificationSdJwtInvalidEmail" => {
                EmailVerificationRequestIssueReason::TokenVerificationSdJwtInvalidEmail
            }
            "TokenVerificationSdJwtInvalidHolderKey" => {
                EmailVerificationRequestIssueReason::TokenVerificationSdJwtInvalidHolderKey
            }
            "TokenVerificationKbInvalidTyp" => {
                EmailVerificationRequestIssueReason::TokenVerificationKbInvalidTyp
            }
            "TokenVerificationKbMissingAud" => {
                EmailVerificationRequestIssueReason::TokenVerificationKbMissingAud
            }
            "TokenVerificationKbMissingNonce" => {
                EmailVerificationRequestIssueReason::TokenVerificationKbMissingNonce
            }
            "TokenVerificationKbMissingIat" => {
                EmailVerificationRequestIssueReason::TokenVerificationKbMissingIat
            }
            "TokenVerificationKbMissingSdHash" => {
                EmailVerificationRequestIssueReason::TokenVerificationKbMissingSdHash
            }
            "TokenVerificationKbInvalidIssuedAt" => {
                EmailVerificationRequestIssueReason::TokenVerificationKbInvalidIssuedAt
            }
            "TokenVerificationKbInvalidAudience" => {
                EmailVerificationRequestIssueReason::TokenVerificationKbInvalidAudience
            }
            "TokenVerificationKbInvalidNonce" => {
                EmailVerificationRequestIssueReason::TokenVerificationKbInvalidNonce
            }
            "TokenVerificationKbInvalidSdHash" => {
                EmailVerificationRequestIssueReason::TokenVerificationKbInvalidSdHash
            }
            "TokenVerificationKbMissingCnf" => {
                EmailVerificationRequestIssueReason::TokenVerificationKbMissingCnf
            }
            "TokenVerificationKbSignatureFailed" => {
                EmailVerificationRequestIssueReason::TokenVerificationKbSignatureFailed
            }
            "CrossOriginIframeNotSupported" => {
                EmailVerificationRequestIssueReason::CrossOriginIframeNotSupported
            }
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for EmailVerificationRequestIssueReason {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for EmailVerificationRequestIssueReason {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EmailVerificationRequestIssueReason;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(
                    match v {
                        "InvalidEmail" => {
                            EmailVerificationRequestIssueReason::InvalidEmail
                        }
                        "DnsFetchFailed" => {
                            EmailVerificationRequestIssueReason::DnsFetchFailed
                        }
                        "DnsInvalidRecord" => {
                            EmailVerificationRequestIssueReason::DnsInvalidRecord
                        }
                        "WellKnownHttpNotFound" => {
                            EmailVerificationRequestIssueReason::WellKnownHttpNotFound
                        }
                        "WellKnownNoResponse" => {
                            EmailVerificationRequestIssueReason::WellKnownNoResponse
                        }
                        "WellKnownInvalidResponse" => {
                            EmailVerificationRequestIssueReason::WellKnownInvalidResponse
                        }
                        "WellKnownListEmpty" => {
                            EmailVerificationRequestIssueReason::WellKnownListEmpty
                        }
                        "WellKnownInvalidContentType" => {
                            EmailVerificationRequestIssueReason::WellKnownInvalidContentType
                        }
                        "WellKnownMissingIssuanceEndpoint" => {
                            EmailVerificationRequestIssueReason::WellKnownMissingIssuanceEndpoint
                        }
                        "WellKnownIssuanceEndpointCrossOrigin" => {
                            EmailVerificationRequestIssueReason::WellKnownIssuanceEndpointCrossOrigin
                        }
                        "WellKnownUnsupportedSigningAlgorithm" => {
                            EmailVerificationRequestIssueReason::WellKnownUnsupportedSigningAlgorithm
                        }
                        "TokenHttpNotFound" => {
                            EmailVerificationRequestIssueReason::TokenHttpNotFound
                        }
                        "TokenNoResponse" => {
                            EmailVerificationRequestIssueReason::TokenNoResponse
                        }
                        "TokenInvalidResponse" => {
                            EmailVerificationRequestIssueReason::TokenInvalidResponse
                        }
                        "TokenInvalidContentType" => {
                            EmailVerificationRequestIssueReason::TokenInvalidContentType
                        }
                        "TokenMalformedSdJwt" => {
                            EmailVerificationRequestIssueReason::TokenMalformedSdJwt
                        }
                        "TokenInvalidSdJwt" => {
                            EmailVerificationRequestIssueReason::TokenInvalidSdJwt
                        }
                        "KeyBindingSigningFailed" => {
                            EmailVerificationRequestIssueReason::KeyBindingSigningFailed
                        }
                        "RpOriginIsOpaque" => {
                            EmailVerificationRequestIssueReason::RpOriginIsOpaque
                        }
                        "WellKnownMissingAccountsEndpoint" => {
                            EmailVerificationRequestIssueReason::WellKnownMissingAccountsEndpoint
                        }
                        "UserLoggedOut" => {
                            EmailVerificationRequestIssueReason::UserLoggedOut
                        }
                        "WellKnownAccountsEndpointCrossOrigin" => {
                            EmailVerificationRequestIssueReason::WellKnownAccountsEndpointCrossOrigin
                        }
                        "AccountsHttpNotFound" => {
                            EmailVerificationRequestIssueReason::AccountsHttpNotFound
                        }
                        "AccountsNoResponse" => {
                            EmailVerificationRequestIssueReason::AccountsNoResponse
                        }
                        "AccountsInvalidResponse" => {
                            EmailVerificationRequestIssueReason::AccountsInvalidResponse
                        }
                        "AccountsInvalidContentType" => {
                            EmailVerificationRequestIssueReason::AccountsInvalidContentType
                        }
                        "AccountsEmptyList" => {
                            EmailVerificationRequestIssueReason::AccountsEmptyList
                        }
                        "EmailVerificationWellKnownHttpNotFound" => {
                            EmailVerificationRequestIssueReason::EmailVerificationWellKnownHttpNotFound
                        }
                        "EmailVerificationWellKnownNoResponse" => {
                            EmailVerificationRequestIssueReason::EmailVerificationWellKnownNoResponse
                        }
                        "EmailVerificationWellKnownInvalidResponse" => {
                            EmailVerificationRequestIssueReason::EmailVerificationWellKnownInvalidResponse
                        }
                        "EmailVerificationWellKnownInvalidContentType" => {
                            EmailVerificationRequestIssueReason::EmailVerificationWellKnownInvalidContentType
                        }
                        "JwksHttpNotFound" => {
                            EmailVerificationRequestIssueReason::JwksHttpNotFound
                        }
                        "JwksInvalidResponse" => {
                            EmailVerificationRequestIssueReason::JwksInvalidResponse
                        }
                        "TokenVerificationSdJwtUnsupportedHeaderAlg" => {
                            EmailVerificationRequestIssueReason::TokenVerificationSdJwtUnsupportedHeaderAlg
                        }
                        "TokenVerificationSdJwtInvalidTyp" => {
                            EmailVerificationRequestIssueReason::TokenVerificationSdJwtInvalidTyp
                        }
                        "TokenVerificationSdJwtMissingIss" => {
                            EmailVerificationRequestIssueReason::TokenVerificationSdJwtMissingIss
                        }
                        "TokenVerificationSdJwtMissingIat" => {
                            EmailVerificationRequestIssueReason::TokenVerificationSdJwtMissingIat
                        }
                        "TokenVerificationSdJwtMissingCnf" => {
                            EmailVerificationRequestIssueReason::TokenVerificationSdJwtMissingCnf
                        }
                        "TokenVerificationSdJwtMissingEmail" => {
                            EmailVerificationRequestIssueReason::TokenVerificationSdJwtMissingEmail
                        }
                        "TokenVerificationSdJwtInvalidIssuedAt" => {
                            EmailVerificationRequestIssueReason::TokenVerificationSdJwtInvalidIssuedAt
                        }
                        "TokenVerificationSdJwtInvalidIssuer" => {
                            EmailVerificationRequestIssueReason::TokenVerificationSdJwtInvalidIssuer
                        }
                        "TokenVerificationSdJwtJwksMissingKeys" => {
                            EmailVerificationRequestIssueReason::TokenVerificationSdJwtJwksMissingKeys
                        }
                        "TokenVerificationSdJwtSignatureFailed" => {
                            EmailVerificationRequestIssueReason::TokenVerificationSdJwtSignatureFailed
                        }
                        "TokenVerificationSdJwtInvalidEmailVerified" => {
                            EmailVerificationRequestIssueReason::TokenVerificationSdJwtInvalidEmailVerified
                        }
                        "TokenVerificationSdJwtInvalidEmail" => {
                            EmailVerificationRequestIssueReason::TokenVerificationSdJwtInvalidEmail
                        }
                        "TokenVerificationSdJwtInvalidHolderKey" => {
                            EmailVerificationRequestIssueReason::TokenVerificationSdJwtInvalidHolderKey
                        }
                        "TokenVerificationKbInvalidTyp" => {
                            EmailVerificationRequestIssueReason::TokenVerificationKbInvalidTyp
                        }
                        "TokenVerificationKbMissingAud" => {
                            EmailVerificationRequestIssueReason::TokenVerificationKbMissingAud
                        }
                        "TokenVerificationKbMissingNonce" => {
                            EmailVerificationRequestIssueReason::TokenVerificationKbMissingNonce
                        }
                        "TokenVerificationKbMissingIat" => {
                            EmailVerificationRequestIssueReason::TokenVerificationKbMissingIat
                        }
                        "TokenVerificationKbMissingSdHash" => {
                            EmailVerificationRequestIssueReason::TokenVerificationKbMissingSdHash
                        }
                        "TokenVerificationKbInvalidIssuedAt" => {
                            EmailVerificationRequestIssueReason::TokenVerificationKbInvalidIssuedAt
                        }
                        "TokenVerificationKbInvalidAudience" => {
                            EmailVerificationRequestIssueReason::TokenVerificationKbInvalidAudience
                        }
                        "TokenVerificationKbInvalidNonce" => {
                            EmailVerificationRequestIssueReason::TokenVerificationKbInvalidNonce
                        }
                        "TokenVerificationKbInvalidSdHash" => {
                            EmailVerificationRequestIssueReason::TokenVerificationKbInvalidSdHash
                        }
                        "TokenVerificationKbMissingCnf" => {
                            EmailVerificationRequestIssueReason::TokenVerificationKbMissingCnf
                        }
                        "TokenVerificationKbSignatureFailed" => {
                            EmailVerificationRequestIssueReason::TokenVerificationKbSignatureFailed
                        }
                        "CrossOriginIframeNotSupported" => {
                            EmailVerificationRequestIssueReason::CrossOriginIframeNotSupported
                        }
                        _ => EmailVerificationRequestIssueReason::Unknown(v.to_owned()),
                    },
                )
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(EmailVerificationRequestIssueReason::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.FailedRequestInfo。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FailedRequestInfo {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.FailedRequestInfo/properties/url。
    ///
    ///协议原文：
    ///
    ///<pre>The URL that failed to load.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.FailedRequestInfo/properties/failureMessage。
    ///
    ///协议原文：
    ///
    ///<pre>The failure message for the failed request.</pre>
    ///
    ///
    #[serde(rename = "failureMessage")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub failure_message: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.FailedRequestInfo/properties/requestId。
    #[serde(rename = "requestId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub request_id: Option<crate::network::types::RequestId>,
}
impl<'de> serde::Deserialize<'de> for FailedRequestInfo {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FailedRequestInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.FailedRequestInfo/properties/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The URL that failed to load.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.FailedRequestInfo/properties/failureMessage。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The failure message for the failed request.</pre>
                    ///
                    ///
                    #[serde(rename = "failureMessage")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub failure_message: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.FailedRequestInfo/properties/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub request_id: Option<crate::network::types::RequestId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FailedRequestInfo {
                    url: fields.url,
                    failure_message: fields.failure_message,
                    request_id: fields.request_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.FederatedAuthRequestIssueDetails。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FederatedAuthRequestIssueDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.FederatedAuthRequestIssueDetails/properties/federatedAuthRequestIssueReason。
    #[serde(rename = "federatedAuthRequestIssueReason")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub federated_auth_request_issue_reason: crate::audits::types::FederatedAuthRequestIssueReason,
}
impl<'de> serde::Deserialize<'de> for FederatedAuthRequestIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FederatedAuthRequestIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.FederatedAuthRequestIssueDetails/properties/federatedAuthRequestIssueReason。
                    #[serde(rename = "federatedAuthRequestIssueReason")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub federated_auth_request_issue_reason:
                        crate::audits::types::FederatedAuthRequestIssueReason,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FederatedAuthRequestIssueDetails {
                    federated_auth_request_issue_reason: fields.federated_auth_request_issue_reason,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.type.FederatedAuthRequestIssueReason。
///
///协议原文：
///
///<pre>Represents the failure reason when a federated authentication reason fails.
///Should be updated alongside RequestIdTokenStatus in
///third_party/blink/public/mojom/devtools/inspector_issue.mojom to include
///all cases except for success.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FederatedAuthRequestIssueReason {
    ///协议值 "ShouldEmbargo"。
    ShouldEmbargo,
    ///协议值 "TooManyRequests"。
    TooManyRequests,
    ///协议值 "WellKnownHttpNotFound"。
    WellKnownHttpNotFound,
    ///协议值 "WellKnownNoResponse"。
    WellKnownNoResponse,
    ///协议值 "WellKnownBlockedByConnectionAllowlist"。
    WellKnownBlockedByConnectionAllowlist,
    ///协议值 "WellKnownInvalidResponse"。
    WellKnownInvalidResponse,
    ///协议值 "WellKnownListEmpty"。
    WellKnownListEmpty,
    ///协议值 "WellKnownInvalidContentType"。
    WellKnownInvalidContentType,
    ///协议值 "ConfigNotInWellKnown"。
    ConfigNotInWellKnown,
    ///协议值 "WellKnownTooBig"。
    WellKnownTooBig,
    ///协议值 "ConfigHttpNotFound"。
    ConfigHttpNotFound,
    ///协议值 "ConfigNoResponse"。
    ConfigNoResponse,
    ///协议值 "ConfigBlockedByConnectionAllowlist"。
    ConfigBlockedByConnectionAllowlist,
    ///协议值 "ConfigInvalidResponse"。
    ConfigInvalidResponse,
    ///协议值 "ConfigInvalidContentType"。
    ConfigInvalidContentType,
    ///协议值 "IdpNotPotentiallyTrustworthy"。
    IdpNotPotentiallyTrustworthy,
    ///协议值 "DisabledInSettings"。
    DisabledInSettings,
    ///协议值 "DisabledInFlags"。
    DisabledInFlags,
    ///协议值 "ErrorFetchingSignin"。
    ErrorFetchingSignin,
    ///协议值 "InvalidSigninResponse"。
    InvalidSigninResponse,
    ///协议值 "AccountsHttpNotFound"。
    AccountsHttpNotFound,
    ///协议值 "AccountsNoResponse"。
    AccountsNoResponse,
    ///协议值 "AccountsBlockedByConnectionAllowlist"。
    AccountsBlockedByConnectionAllowlist,
    ///协议值 "AccountsInvalidResponse"。
    AccountsInvalidResponse,
    ///协议值 "AccountsListEmpty"。
    AccountsListEmpty,
    ///协议值 "AccountsInvalidContentType"。
    AccountsInvalidContentType,
    ///协议值 "IdTokenHttpNotFound"。
    IdTokenHttpNotFound,
    ///协议值 "IdTokenNoResponse"。
    IdTokenNoResponse,
    ///协议值 "IdTokenBlockedByConnectionAllowlist"。
    IdTokenBlockedByConnectionAllowlist,
    ///协议值 "IdTokenInvalidResponse"。
    IdTokenInvalidResponse,
    ///协议值 "IdTokenIdpErrorResponse"。
    IdTokenIdpErrorResponse,
    ///协议值 "IdTokenCrossSiteIdpErrorResponse"。
    IdTokenCrossSiteIdpErrorResponse,
    ///协议值 "IdTokenInvalidRequest"。
    IdTokenInvalidRequest,
    ///协议值 "IdTokenInvalidContentType"。
    IdTokenInvalidContentType,
    ///协议值 "ErrorIdToken"。
    ErrorIdToken,
    ///协议值 "Canceled"。
    Canceled,
    ///协议值 "RpPageNotVisible"。
    RpPageNotVisible,
    ///协议值 "SilentMediationFailure"。
    SilentMediationFailure,
    ///协议值 "NotSignedInWithIdp"。
    NotSignedInWithIdp,
    ///协议值 "MissingTransientUserActivation"。
    MissingTransientUserActivation,
    ///协议值 "ReplacedByActiveMode"。
    ReplacedByActiveMode,
    ///协议值 "RelyingPartyOriginIsOpaque"。
    RelyingPartyOriginIsOpaque,
    ///协议值 "TypeNotMatching"。
    TypeNotMatching,
    ///协议值 "UiDismissedNoEmbargo"。
    UiDismissedNoEmbargo,
    ///协议值 "CorsError"。
    CorsError,
    ///协议值 "SuppressedBySegmentationPlatform"。
    SuppressedBySegmentationPlatform,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl FederatedAuthRequestIssueReason {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::types::FederatedAuthRequestIssueReason::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::ShouldEmbargo => "ShouldEmbargo",
            Self::TooManyRequests => "TooManyRequests",
            Self::WellKnownHttpNotFound => "WellKnownHttpNotFound",
            Self::WellKnownNoResponse => "WellKnownNoResponse",
            Self::WellKnownBlockedByConnectionAllowlist => "WellKnownBlockedByConnectionAllowlist",
            Self::WellKnownInvalidResponse => "WellKnownInvalidResponse",
            Self::WellKnownListEmpty => "WellKnownListEmpty",
            Self::WellKnownInvalidContentType => "WellKnownInvalidContentType",
            Self::ConfigNotInWellKnown => "ConfigNotInWellKnown",
            Self::WellKnownTooBig => "WellKnownTooBig",
            Self::ConfigHttpNotFound => "ConfigHttpNotFound",
            Self::ConfigNoResponse => "ConfigNoResponse",
            Self::ConfigBlockedByConnectionAllowlist => "ConfigBlockedByConnectionAllowlist",
            Self::ConfigInvalidResponse => "ConfigInvalidResponse",
            Self::ConfigInvalidContentType => "ConfigInvalidContentType",
            Self::IdpNotPotentiallyTrustworthy => "IdpNotPotentiallyTrustworthy",
            Self::DisabledInSettings => "DisabledInSettings",
            Self::DisabledInFlags => "DisabledInFlags",
            Self::ErrorFetchingSignin => "ErrorFetchingSignin",
            Self::InvalidSigninResponse => "InvalidSigninResponse",
            Self::AccountsHttpNotFound => "AccountsHttpNotFound",
            Self::AccountsNoResponse => "AccountsNoResponse",
            Self::AccountsBlockedByConnectionAllowlist => "AccountsBlockedByConnectionAllowlist",
            Self::AccountsInvalidResponse => "AccountsInvalidResponse",
            Self::AccountsListEmpty => "AccountsListEmpty",
            Self::AccountsInvalidContentType => "AccountsInvalidContentType",
            Self::IdTokenHttpNotFound => "IdTokenHttpNotFound",
            Self::IdTokenNoResponse => "IdTokenNoResponse",
            Self::IdTokenBlockedByConnectionAllowlist => "IdTokenBlockedByConnectionAllowlist",
            Self::IdTokenInvalidResponse => "IdTokenInvalidResponse",
            Self::IdTokenIdpErrorResponse => "IdTokenIdpErrorResponse",
            Self::IdTokenCrossSiteIdpErrorResponse => "IdTokenCrossSiteIdpErrorResponse",
            Self::IdTokenInvalidRequest => "IdTokenInvalidRequest",
            Self::IdTokenInvalidContentType => "IdTokenInvalidContentType",
            Self::ErrorIdToken => "ErrorIdToken",
            Self::Canceled => "Canceled",
            Self::RpPageNotVisible => "RpPageNotVisible",
            Self::SilentMediationFailure => "SilentMediationFailure",
            Self::NotSignedInWithIdp => "NotSignedInWithIdp",
            Self::MissingTransientUserActivation => "MissingTransientUserActivation",
            Self::ReplacedByActiveMode => "ReplacedByActiveMode",
            Self::RelyingPartyOriginIsOpaque => "RelyingPartyOriginIsOpaque",
            Self::TypeNotMatching => "TypeNotMatching",
            Self::UiDismissedNoEmbargo => "UiDismissedNoEmbargo",
            Self::CorsError => "CorsError",
            Self::SuppressedBySegmentationPlatform => "SuppressedBySegmentationPlatform",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for FederatedAuthRequestIssueReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for FederatedAuthRequestIssueReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ShouldEmbargo" => FederatedAuthRequestIssueReason::ShouldEmbargo,
            "TooManyRequests" => FederatedAuthRequestIssueReason::TooManyRequests,
            "WellKnownHttpNotFound" => FederatedAuthRequestIssueReason::WellKnownHttpNotFound,
            "WellKnownNoResponse" => FederatedAuthRequestIssueReason::WellKnownNoResponse,
            "WellKnownBlockedByConnectionAllowlist" => {
                FederatedAuthRequestIssueReason::WellKnownBlockedByConnectionAllowlist
            }
            "WellKnownInvalidResponse" => FederatedAuthRequestIssueReason::WellKnownInvalidResponse,
            "WellKnownListEmpty" => FederatedAuthRequestIssueReason::WellKnownListEmpty,
            "WellKnownInvalidContentType" => {
                FederatedAuthRequestIssueReason::WellKnownInvalidContentType
            }
            "ConfigNotInWellKnown" => FederatedAuthRequestIssueReason::ConfigNotInWellKnown,
            "WellKnownTooBig" => FederatedAuthRequestIssueReason::WellKnownTooBig,
            "ConfigHttpNotFound" => FederatedAuthRequestIssueReason::ConfigHttpNotFound,
            "ConfigNoResponse" => FederatedAuthRequestIssueReason::ConfigNoResponse,
            "ConfigBlockedByConnectionAllowlist" => {
                FederatedAuthRequestIssueReason::ConfigBlockedByConnectionAllowlist
            }
            "ConfigInvalidResponse" => FederatedAuthRequestIssueReason::ConfigInvalidResponse,
            "ConfigInvalidContentType" => FederatedAuthRequestIssueReason::ConfigInvalidContentType,
            "IdpNotPotentiallyTrustworthy" => {
                FederatedAuthRequestIssueReason::IdpNotPotentiallyTrustworthy
            }
            "DisabledInSettings" => FederatedAuthRequestIssueReason::DisabledInSettings,
            "DisabledInFlags" => FederatedAuthRequestIssueReason::DisabledInFlags,
            "ErrorFetchingSignin" => FederatedAuthRequestIssueReason::ErrorFetchingSignin,
            "InvalidSigninResponse" => FederatedAuthRequestIssueReason::InvalidSigninResponse,
            "AccountsHttpNotFound" => FederatedAuthRequestIssueReason::AccountsHttpNotFound,
            "AccountsNoResponse" => FederatedAuthRequestIssueReason::AccountsNoResponse,
            "AccountsBlockedByConnectionAllowlist" => {
                FederatedAuthRequestIssueReason::AccountsBlockedByConnectionAllowlist
            }
            "AccountsInvalidResponse" => FederatedAuthRequestIssueReason::AccountsInvalidResponse,
            "AccountsListEmpty" => FederatedAuthRequestIssueReason::AccountsListEmpty,
            "AccountsInvalidContentType" => {
                FederatedAuthRequestIssueReason::AccountsInvalidContentType
            }
            "IdTokenHttpNotFound" => FederatedAuthRequestIssueReason::IdTokenHttpNotFound,
            "IdTokenNoResponse" => FederatedAuthRequestIssueReason::IdTokenNoResponse,
            "IdTokenBlockedByConnectionAllowlist" => {
                FederatedAuthRequestIssueReason::IdTokenBlockedByConnectionAllowlist
            }
            "IdTokenInvalidResponse" => FederatedAuthRequestIssueReason::IdTokenInvalidResponse,
            "IdTokenIdpErrorResponse" => FederatedAuthRequestIssueReason::IdTokenIdpErrorResponse,
            "IdTokenCrossSiteIdpErrorResponse" => {
                FederatedAuthRequestIssueReason::IdTokenCrossSiteIdpErrorResponse
            }
            "IdTokenInvalidRequest" => FederatedAuthRequestIssueReason::IdTokenInvalidRequest,
            "IdTokenInvalidContentType" => {
                FederatedAuthRequestIssueReason::IdTokenInvalidContentType
            }
            "ErrorIdToken" => FederatedAuthRequestIssueReason::ErrorIdToken,
            "Canceled" => FederatedAuthRequestIssueReason::Canceled,
            "RpPageNotVisible" => FederatedAuthRequestIssueReason::RpPageNotVisible,
            "SilentMediationFailure" => FederatedAuthRequestIssueReason::SilentMediationFailure,
            "NotSignedInWithIdp" => FederatedAuthRequestIssueReason::NotSignedInWithIdp,
            "MissingTransientUserActivation" => {
                FederatedAuthRequestIssueReason::MissingTransientUserActivation
            }
            "ReplacedByActiveMode" => FederatedAuthRequestIssueReason::ReplacedByActiveMode,
            "RelyingPartyOriginIsOpaque" => {
                FederatedAuthRequestIssueReason::RelyingPartyOriginIsOpaque
            }
            "TypeNotMatching" => FederatedAuthRequestIssueReason::TypeNotMatching,
            "UiDismissedNoEmbargo" => FederatedAuthRequestIssueReason::UiDismissedNoEmbargo,
            "CorsError" => FederatedAuthRequestIssueReason::CorsError,
            "SuppressedBySegmentationPlatform" => {
                FederatedAuthRequestIssueReason::SuppressedBySegmentationPlatform
            }
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for FederatedAuthRequestIssueReason {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FederatedAuthRequestIssueReason {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FederatedAuthRequestIssueReason;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "ShouldEmbargo" => FederatedAuthRequestIssueReason::ShouldEmbargo,
                    "TooManyRequests" => FederatedAuthRequestIssueReason::TooManyRequests,
                    "WellKnownHttpNotFound" => {
                        FederatedAuthRequestIssueReason::WellKnownHttpNotFound
                    }
                    "WellKnownNoResponse" => FederatedAuthRequestIssueReason::WellKnownNoResponse,
                    "WellKnownBlockedByConnectionAllowlist" => {
                        FederatedAuthRequestIssueReason::WellKnownBlockedByConnectionAllowlist
                    }
                    "WellKnownInvalidResponse" => {
                        FederatedAuthRequestIssueReason::WellKnownInvalidResponse
                    }
                    "WellKnownListEmpty" => FederatedAuthRequestIssueReason::WellKnownListEmpty,
                    "WellKnownInvalidContentType" => {
                        FederatedAuthRequestIssueReason::WellKnownInvalidContentType
                    }
                    "ConfigNotInWellKnown" => FederatedAuthRequestIssueReason::ConfigNotInWellKnown,
                    "WellKnownTooBig" => FederatedAuthRequestIssueReason::WellKnownTooBig,
                    "ConfigHttpNotFound" => FederatedAuthRequestIssueReason::ConfigHttpNotFound,
                    "ConfigNoResponse" => FederatedAuthRequestIssueReason::ConfigNoResponse,
                    "ConfigBlockedByConnectionAllowlist" => {
                        FederatedAuthRequestIssueReason::ConfigBlockedByConnectionAllowlist
                    }
                    "ConfigInvalidResponse" => {
                        FederatedAuthRequestIssueReason::ConfigInvalidResponse
                    }
                    "ConfigInvalidContentType" => {
                        FederatedAuthRequestIssueReason::ConfigInvalidContentType
                    }
                    "IdpNotPotentiallyTrustworthy" => {
                        FederatedAuthRequestIssueReason::IdpNotPotentiallyTrustworthy
                    }
                    "DisabledInSettings" => FederatedAuthRequestIssueReason::DisabledInSettings,
                    "DisabledInFlags" => FederatedAuthRequestIssueReason::DisabledInFlags,
                    "ErrorFetchingSignin" => FederatedAuthRequestIssueReason::ErrorFetchingSignin,
                    "InvalidSigninResponse" => {
                        FederatedAuthRequestIssueReason::InvalidSigninResponse
                    }
                    "AccountsHttpNotFound" => FederatedAuthRequestIssueReason::AccountsHttpNotFound,
                    "AccountsNoResponse" => FederatedAuthRequestIssueReason::AccountsNoResponse,
                    "AccountsBlockedByConnectionAllowlist" => {
                        FederatedAuthRequestIssueReason::AccountsBlockedByConnectionAllowlist
                    }
                    "AccountsInvalidResponse" => {
                        FederatedAuthRequestIssueReason::AccountsInvalidResponse
                    }
                    "AccountsListEmpty" => FederatedAuthRequestIssueReason::AccountsListEmpty,
                    "AccountsInvalidContentType" => {
                        FederatedAuthRequestIssueReason::AccountsInvalidContentType
                    }
                    "IdTokenHttpNotFound" => FederatedAuthRequestIssueReason::IdTokenHttpNotFound,
                    "IdTokenNoResponse" => FederatedAuthRequestIssueReason::IdTokenNoResponse,
                    "IdTokenBlockedByConnectionAllowlist" => {
                        FederatedAuthRequestIssueReason::IdTokenBlockedByConnectionAllowlist
                    }
                    "IdTokenInvalidResponse" => {
                        FederatedAuthRequestIssueReason::IdTokenInvalidResponse
                    }
                    "IdTokenIdpErrorResponse" => {
                        FederatedAuthRequestIssueReason::IdTokenIdpErrorResponse
                    }
                    "IdTokenCrossSiteIdpErrorResponse" => {
                        FederatedAuthRequestIssueReason::IdTokenCrossSiteIdpErrorResponse
                    }
                    "IdTokenInvalidRequest" => {
                        FederatedAuthRequestIssueReason::IdTokenInvalidRequest
                    }
                    "IdTokenInvalidContentType" => {
                        FederatedAuthRequestIssueReason::IdTokenInvalidContentType
                    }
                    "ErrorIdToken" => FederatedAuthRequestIssueReason::ErrorIdToken,
                    "Canceled" => FederatedAuthRequestIssueReason::Canceled,
                    "RpPageNotVisible" => FederatedAuthRequestIssueReason::RpPageNotVisible,
                    "SilentMediationFailure" => {
                        FederatedAuthRequestIssueReason::SilentMediationFailure
                    }
                    "NotSignedInWithIdp" => FederatedAuthRequestIssueReason::NotSignedInWithIdp,
                    "MissingTransientUserActivation" => {
                        FederatedAuthRequestIssueReason::MissingTransientUserActivation
                    }
                    "ReplacedByActiveMode" => FederatedAuthRequestIssueReason::ReplacedByActiveMode,
                    "RelyingPartyOriginIsOpaque" => {
                        FederatedAuthRequestIssueReason::RelyingPartyOriginIsOpaque
                    }
                    "TypeNotMatching" => FederatedAuthRequestIssueReason::TypeNotMatching,
                    "UiDismissedNoEmbargo" => FederatedAuthRequestIssueReason::UiDismissedNoEmbargo,
                    "CorsError" => FederatedAuthRequestIssueReason::CorsError,
                    "SuppressedBySegmentationPlatform" => {
                        FederatedAuthRequestIssueReason::SuppressedBySegmentationPlatform
                    }
                    _ => FederatedAuthRequestIssueReason::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(FederatedAuthRequestIssueReason::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.FederatedAuthUserInfoRequestIssueDetails。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FederatedAuthUserInfoRequestIssueDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.FederatedAuthUserInfoRequestIssueDetails/properties/federatedAuthUserInfoRequestIssueReason。
    #[serde(rename = "federatedAuthUserInfoRequestIssueReason")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub federated_auth_user_info_request_issue_reason:
        crate::audits::types::FederatedAuthUserInfoRequestIssueReason,
}
impl<'de> serde::Deserialize<'de> for FederatedAuthUserInfoRequestIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FederatedAuthUserInfoRequestIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.FederatedAuthUserInfoRequestIssueDetails/properties/federatedAuthUserInfoRequestIssueReason。
                    #[serde(rename = "federatedAuthUserInfoRequestIssueReason")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub federated_auth_user_info_request_issue_reason:
                        crate::audits::types::FederatedAuthUserInfoRequestIssueReason,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FederatedAuthUserInfoRequestIssueDetails {
                    federated_auth_user_info_request_issue_reason: fields
                        .federated_auth_user_info_request_issue_reason,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.type.FederatedAuthUserInfoRequestIssueReason。
///
///协议原文：
///
///<pre>Represents the failure reason when a getUserInfo() call fails.
///Should be updated alongside FederatedAuthUserInfoRequestResult in
///third_party/blink/public/mojom/devtools/inspector_issue.mojom.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FederatedAuthUserInfoRequestIssueReason {
    ///协议值 "NotSameOrigin"。
    NotSameOrigin,
    ///协议值 "NotIframe"。
    NotIframe,
    ///协议值 "NotPotentiallyTrustworthy"。
    NotPotentiallyTrustworthy,
    ///协议值 "NoApiPermission"。
    NoApiPermission,
    ///协议值 "NotSignedInWithIdp"。
    NotSignedInWithIdp,
    ///协议值 "NoAccountSharingPermission"。
    NoAccountSharingPermission,
    ///协议值 "InvalidConfigOrWellKnown"。
    InvalidConfigOrWellKnown,
    ///协议值 "InvalidAccountsResponse"。
    InvalidAccountsResponse,
    ///协议值 "NoReturningUserFromFetchedAccounts"。
    NoReturningUserFromFetchedAccounts,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl FederatedAuthUserInfoRequestIssueReason {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::types::FederatedAuthUserInfoRequestIssueReason::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::NotSameOrigin => "NotSameOrigin",
            Self::NotIframe => "NotIframe",
            Self::NotPotentiallyTrustworthy => "NotPotentiallyTrustworthy",
            Self::NoApiPermission => "NoApiPermission",
            Self::NotSignedInWithIdp => "NotSignedInWithIdp",
            Self::NoAccountSharingPermission => "NoAccountSharingPermission",
            Self::InvalidConfigOrWellKnown => "InvalidConfigOrWellKnown",
            Self::InvalidAccountsResponse => "InvalidAccountsResponse",
            Self::NoReturningUserFromFetchedAccounts => "NoReturningUserFromFetchedAccounts",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for FederatedAuthUserInfoRequestIssueReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for FederatedAuthUserInfoRequestIssueReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "NotSameOrigin" => FederatedAuthUserInfoRequestIssueReason::NotSameOrigin,
            "NotIframe" => FederatedAuthUserInfoRequestIssueReason::NotIframe,
            "NotPotentiallyTrustworthy" => {
                FederatedAuthUserInfoRequestIssueReason::NotPotentiallyTrustworthy
            }
            "NoApiPermission" => FederatedAuthUserInfoRequestIssueReason::NoApiPermission,
            "NotSignedInWithIdp" => FederatedAuthUserInfoRequestIssueReason::NotSignedInWithIdp,
            "NoAccountSharingPermission" => {
                FederatedAuthUserInfoRequestIssueReason::NoAccountSharingPermission
            }
            "InvalidConfigOrWellKnown" => {
                FederatedAuthUserInfoRequestIssueReason::InvalidConfigOrWellKnown
            }
            "InvalidAccountsResponse" => {
                FederatedAuthUserInfoRequestIssueReason::InvalidAccountsResponse
            }
            "NoReturningUserFromFetchedAccounts" => {
                FederatedAuthUserInfoRequestIssueReason::NoReturningUserFromFetchedAccounts
            }
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for FederatedAuthUserInfoRequestIssueReason {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FederatedAuthUserInfoRequestIssueReason {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FederatedAuthUserInfoRequestIssueReason;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "NotSameOrigin" => FederatedAuthUserInfoRequestIssueReason::NotSameOrigin,
                    "NotIframe" => FederatedAuthUserInfoRequestIssueReason::NotIframe,
                    "NotPotentiallyTrustworthy" => {
                        FederatedAuthUserInfoRequestIssueReason::NotPotentiallyTrustworthy
                    }
                    "NoApiPermission" => FederatedAuthUserInfoRequestIssueReason::NoApiPermission,
                    "NotSignedInWithIdp" => {
                        FederatedAuthUserInfoRequestIssueReason::NotSignedInWithIdp
                    }
                    "NoAccountSharingPermission" => {
                        FederatedAuthUserInfoRequestIssueReason::NoAccountSharingPermission
                    }
                    "InvalidConfigOrWellKnown" => {
                        FederatedAuthUserInfoRequestIssueReason::InvalidConfigOrWellKnown
                    }
                    "InvalidAccountsResponse" => {
                        FederatedAuthUserInfoRequestIssueReason::InvalidAccountsResponse
                    }
                    "NoReturningUserFromFetchedAccounts" => {
                        FederatedAuthUserInfoRequestIssueReason::NoReturningUserFromFetchedAccounts
                    }
                    _ => FederatedAuthUserInfoRequestIssueReason::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(FederatedAuthUserInfoRequestIssueReason::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.GenericIssueDetails。
///
///协议原文：
///
///<pre>Depending on the concrete errorType, different properties are set.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct GenericIssueDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.GenericIssueDetails/properties/errorType。
    ///
    ///协议原文：
    ///
    ///<pre>Issues with the same errorType are aggregated in the frontend.</pre>
    ///
    ///
    #[serde(rename = "errorType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub error_type: crate::audits::types::GenericIssueErrorType,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.GenericIssueDetails/properties/frameId。
    #[serde(rename = "frameId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub frame_id: Option<crate::page::types::FrameId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.GenericIssueDetails/properties/violatingNodeId。
    #[serde(rename = "violatingNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub violating_node_id: Option<crate::dom::types::BackendNodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.GenericIssueDetails/properties/violatingNodeAttribute。
    #[serde(rename = "violatingNodeAttribute")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub violating_node_attribute: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.GenericIssueDetails/properties/request。
    #[serde(rename = "request")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub request: Option<crate::audits::types::AffectedRequest>,
}
impl<'de> serde::Deserialize<'de> for GenericIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GenericIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.GenericIssueDetails/properties/errorType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Issues with the same errorType are aggregated in the frontend.</pre>
                    ///
                    ///
                    #[serde(rename = "errorType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub error_type: crate::audits::types::GenericIssueErrorType,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.GenericIssueDetails/properties/frameId。
                    #[serde(rename = "frameId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub frame_id: Option<crate::page::types::FrameId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.GenericIssueDetails/properties/violatingNodeId。
                    #[serde(rename = "violatingNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub violating_node_id: Option<crate::dom::types::BackendNodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.GenericIssueDetails/properties/violatingNodeAttribute。
                    #[serde(rename = "violatingNodeAttribute")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub violating_node_attribute: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.GenericIssueDetails/properties/request。
                    #[serde(rename = "request")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub request: Option<crate::audits::types::AffectedRequest>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GenericIssueDetails {
                    error_type: fields.error_type,
                    frame_id: fields.frame_id,
                    violating_node_id: fields.violating_node_id,
                    violating_node_attribute: fields.violating_node_attribute,
                    request: fields.request,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.type.GenericIssueErrorType。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum GenericIssueErrorType {
    ///协议值 "FormLabelForNameError"。
    FormLabelForNameError,
    ///协议值 "FormDuplicateIdForInputError"。
    FormDuplicateIdForInputError,
    ///协议值 "FormInputWithNoLabelError"。
    FormInputWithNoLabelError,
    ///协议值 "FormAutocompleteAttributeEmptyError"。
    FormAutocompleteAttributeEmptyError,
    ///协议值 "FormEmptyIdAndNameAttributesForInputError"。
    FormEmptyIdAndNameAttributesForInputError,
    ///协议值 "FormAriaLabelledByToNonExistingIdError"。
    FormAriaLabelledByToNonExistingIdError,
    ///协议值 "FormInputAssignedAutocompleteValueToIdOrNameAttributeError"。
    FormInputAssignedAutocompleteValueToIdOrNameAttributeError,
    ///协议值 "FormLabelHasNeitherForNorNestedInputError"。
    FormLabelHasNeitherForNorNestedInputError,
    ///协议值 "FormLabelForMatchesNonExistingIdError"。
    FormLabelForMatchesNonExistingIdError,
    ///协议值 "FormInputHasWrongButWellIntendedAutocompleteValueError"。
    FormInputHasWrongButWellIntendedAutocompleteValueError,
    ///协议值 "ResponseWasBlockedByORB"。
    ResponseWasBlockedByOrb,
    ///协议值 "NavigationEntryMarkedSkippable"。
    NavigationEntryMarkedSkippable,
    ///协议值 "BackUINavigationWouldSkipAd"。
    BackUiNavigationWouldSkipAd,
    ///协议值 "AutofillAndManualTextPolicyControlledFeaturesInfo"。
    AutofillAndManualTextPolicyControlledFeaturesInfo,
    ///协议值 "AutofillPolicyControlledFeatureInfo"。
    AutofillPolicyControlledFeatureInfo,
    ///协议值 "ManualTextPolicyControlledFeatureInfo"。
    ManualTextPolicyControlledFeatureInfo,
    ///协议值 "FormModelContextParameterMissingTitleAndDescription"。
    FormModelContextParameterMissingTitleAndDescription,
    ///协议值 "FormModelContextMissingToolName"。
    FormModelContextMissingToolName,
    ///协议值 "FormModelContextMissingToolDescription"。
    FormModelContextMissingToolDescription,
    ///协议值 "FormModelContextRequiredParameterMissingName"。
    FormModelContextRequiredParameterMissingName,
    ///协议值 "FormModelContextParameterMissingName"。
    FormModelContextParameterMissingName,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl GenericIssueErrorType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::types::GenericIssueErrorType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::FormLabelForNameError => "FormLabelForNameError",
            Self::FormDuplicateIdForInputError => "FormDuplicateIdForInputError",
            Self::FormInputWithNoLabelError => "FormInputWithNoLabelError",
            Self::FormAutocompleteAttributeEmptyError => "FormAutocompleteAttributeEmptyError",
            Self::FormEmptyIdAndNameAttributesForInputError => {
                "FormEmptyIdAndNameAttributesForInputError"
            }
            Self::FormAriaLabelledByToNonExistingIdError => {
                "FormAriaLabelledByToNonExistingIdError"
            }
            Self::FormInputAssignedAutocompleteValueToIdOrNameAttributeError => {
                "FormInputAssignedAutocompleteValueToIdOrNameAttributeError"
            }
            Self::FormLabelHasNeitherForNorNestedInputError => {
                "FormLabelHasNeitherForNorNestedInputError"
            }
            Self::FormLabelForMatchesNonExistingIdError => "FormLabelForMatchesNonExistingIdError",
            Self::FormInputHasWrongButWellIntendedAutocompleteValueError => {
                "FormInputHasWrongButWellIntendedAutocompleteValueError"
            }
            Self::ResponseWasBlockedByOrb => "ResponseWasBlockedByORB",
            Self::NavigationEntryMarkedSkippable => "NavigationEntryMarkedSkippable",
            Self::BackUiNavigationWouldSkipAd => "BackUINavigationWouldSkipAd",
            Self::AutofillAndManualTextPolicyControlledFeaturesInfo => {
                "AutofillAndManualTextPolicyControlledFeaturesInfo"
            }
            Self::AutofillPolicyControlledFeatureInfo => "AutofillPolicyControlledFeatureInfo",
            Self::ManualTextPolicyControlledFeatureInfo => "ManualTextPolicyControlledFeatureInfo",
            Self::FormModelContextParameterMissingTitleAndDescription => {
                "FormModelContextParameterMissingTitleAndDescription"
            }
            Self::FormModelContextMissingToolName => "FormModelContextMissingToolName",
            Self::FormModelContextMissingToolDescription => {
                "FormModelContextMissingToolDescription"
            }
            Self::FormModelContextRequiredParameterMissingName => {
                "FormModelContextRequiredParameterMissingName"
            }
            Self::FormModelContextParameterMissingName => "FormModelContextParameterMissingName",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for GenericIssueErrorType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for GenericIssueErrorType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "FormLabelForNameError" => GenericIssueErrorType::FormLabelForNameError,
            "FormDuplicateIdForInputError" => GenericIssueErrorType::FormDuplicateIdForInputError,
            "FormInputWithNoLabelError" => GenericIssueErrorType::FormInputWithNoLabelError,
            "FormAutocompleteAttributeEmptyError" => {
                GenericIssueErrorType::FormAutocompleteAttributeEmptyError
            }
            "FormEmptyIdAndNameAttributesForInputError" => {
                GenericIssueErrorType::FormEmptyIdAndNameAttributesForInputError
            }
            "FormAriaLabelledByToNonExistingIdError" => {
                GenericIssueErrorType::FormAriaLabelledByToNonExistingIdError
            }
            "FormInputAssignedAutocompleteValueToIdOrNameAttributeError" => {
                GenericIssueErrorType::FormInputAssignedAutocompleteValueToIdOrNameAttributeError
            }
            "FormLabelHasNeitherForNorNestedInputError" => {
                GenericIssueErrorType::FormLabelHasNeitherForNorNestedInputError
            }
            "FormLabelForMatchesNonExistingIdError" => {
                GenericIssueErrorType::FormLabelForMatchesNonExistingIdError
            }
            "FormInputHasWrongButWellIntendedAutocompleteValueError" => {
                GenericIssueErrorType::FormInputHasWrongButWellIntendedAutocompleteValueError
            }
            "ResponseWasBlockedByORB" => GenericIssueErrorType::ResponseWasBlockedByOrb,
            "NavigationEntryMarkedSkippable" => {
                GenericIssueErrorType::NavigationEntryMarkedSkippable
            }
            "BackUINavigationWouldSkipAd" => GenericIssueErrorType::BackUiNavigationWouldSkipAd,
            "AutofillAndManualTextPolicyControlledFeaturesInfo" => {
                GenericIssueErrorType::AutofillAndManualTextPolicyControlledFeaturesInfo
            }
            "AutofillPolicyControlledFeatureInfo" => {
                GenericIssueErrorType::AutofillPolicyControlledFeatureInfo
            }
            "ManualTextPolicyControlledFeatureInfo" => {
                GenericIssueErrorType::ManualTextPolicyControlledFeatureInfo
            }
            "FormModelContextParameterMissingTitleAndDescription" => {
                GenericIssueErrorType::FormModelContextParameterMissingTitleAndDescription
            }
            "FormModelContextMissingToolName" => {
                GenericIssueErrorType::FormModelContextMissingToolName
            }
            "FormModelContextMissingToolDescription" => {
                GenericIssueErrorType::FormModelContextMissingToolDescription
            }
            "FormModelContextRequiredParameterMissingName" => {
                GenericIssueErrorType::FormModelContextRequiredParameterMissingName
            }
            "FormModelContextParameterMissingName" => {
                GenericIssueErrorType::FormModelContextParameterMissingName
            }
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for GenericIssueErrorType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GenericIssueErrorType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GenericIssueErrorType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(
                    match v {
                        "FormLabelForNameError" => {
                            GenericIssueErrorType::FormLabelForNameError
                        }
                        "FormDuplicateIdForInputError" => {
                            GenericIssueErrorType::FormDuplicateIdForInputError
                        }
                        "FormInputWithNoLabelError" => {
                            GenericIssueErrorType::FormInputWithNoLabelError
                        }
                        "FormAutocompleteAttributeEmptyError" => {
                            GenericIssueErrorType::FormAutocompleteAttributeEmptyError
                        }
                        "FormEmptyIdAndNameAttributesForInputError" => {
                            GenericIssueErrorType::FormEmptyIdAndNameAttributesForInputError
                        }
                        "FormAriaLabelledByToNonExistingIdError" => {
                            GenericIssueErrorType::FormAriaLabelledByToNonExistingIdError
                        }
                        "FormInputAssignedAutocompleteValueToIdOrNameAttributeError" => {
                            GenericIssueErrorType::FormInputAssignedAutocompleteValueToIdOrNameAttributeError
                        }
                        "FormLabelHasNeitherForNorNestedInputError" => {
                            GenericIssueErrorType::FormLabelHasNeitherForNorNestedInputError
                        }
                        "FormLabelForMatchesNonExistingIdError" => {
                            GenericIssueErrorType::FormLabelForMatchesNonExistingIdError
                        }
                        "FormInputHasWrongButWellIntendedAutocompleteValueError" => {
                            GenericIssueErrorType::FormInputHasWrongButWellIntendedAutocompleteValueError
                        }
                        "ResponseWasBlockedByORB" => {
                            GenericIssueErrorType::ResponseWasBlockedByOrb
                        }
                        "NavigationEntryMarkedSkippable" => {
                            GenericIssueErrorType::NavigationEntryMarkedSkippable
                        }
                        "BackUINavigationWouldSkipAd" => {
                            GenericIssueErrorType::BackUiNavigationWouldSkipAd
                        }
                        "AutofillAndManualTextPolicyControlledFeaturesInfo" => {
                            GenericIssueErrorType::AutofillAndManualTextPolicyControlledFeaturesInfo
                        }
                        "AutofillPolicyControlledFeatureInfo" => {
                            GenericIssueErrorType::AutofillPolicyControlledFeatureInfo
                        }
                        "ManualTextPolicyControlledFeatureInfo" => {
                            GenericIssueErrorType::ManualTextPolicyControlledFeatureInfo
                        }
                        "FormModelContextParameterMissingTitleAndDescription" => {
                            GenericIssueErrorType::FormModelContextParameterMissingTitleAndDescription
                        }
                        "FormModelContextMissingToolName" => {
                            GenericIssueErrorType::FormModelContextMissingToolName
                        }
                        "FormModelContextMissingToolDescription" => {
                            GenericIssueErrorType::FormModelContextMissingToolDescription
                        }
                        "FormModelContextRequiredParameterMissingName" => {
                            GenericIssueErrorType::FormModelContextRequiredParameterMissingName
                        }
                        "FormModelContextParameterMissingName" => {
                            GenericIssueErrorType::FormModelContextParameterMissingName
                        }
                        _ => GenericIssueErrorType::Unknown(v.to_owned()),
                    },
                )
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(GenericIssueErrorType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.HeavyAdIssueDetails。
#[derive(Debug, Clone, serde::Serialize)]
pub struct HeavyAdIssueDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.HeavyAdIssueDetails/properties/resolution。
    ///
    ///协议原文：
    ///
    ///<pre>The resolution status, either blocking the content or warning.</pre>
    ///
    ///
    #[serde(rename = "resolution")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub resolution: crate::audits::types::HeavyAdResolutionStatus,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.HeavyAdIssueDetails/properties/reason。
    ///
    ///协议原文：
    ///
    ///<pre>The reason the ad was blocked, total network or cpu or peak cpu.</pre>
    ///
    ///
    #[serde(rename = "reason")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub reason: crate::audits::types::HeavyAdReason,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.HeavyAdIssueDetails/properties/frame。
    ///
    ///协议原文：
    ///
    ///<pre>The frame that was blocked.</pre>
    ///
    ///
    #[serde(rename = "frame")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame: crate::audits::types::AffectedFrame,
}
impl<'de> serde::Deserialize<'de> for HeavyAdIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HeavyAdIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.HeavyAdIssueDetails/properties/resolution。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The resolution status, either blocking the content or warning.</pre>
                    ///
                    ///
                    #[serde(rename = "resolution")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub resolution: crate::audits::types::HeavyAdResolutionStatus,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.HeavyAdIssueDetails/properties/reason。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The reason the ad was blocked, total network or cpu or peak cpu.</pre>
                    ///
                    ///
                    #[serde(rename = "reason")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub reason: crate::audits::types::HeavyAdReason,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.HeavyAdIssueDetails/properties/frame。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The frame that was blocked.</pre>
                    ///
                    ///
                    #[serde(rename = "frame")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame: crate::audits::types::AffectedFrame,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(HeavyAdIssueDetails {
                    resolution: fields.resolution,
                    reason: fields.reason,
                    frame: fields.frame,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.type.HeavyAdReason。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum HeavyAdReason {
    ///协议值 "NetworkTotalLimit"。
    NetworkTotalLimit,
    ///协议值 "CpuTotalLimit"。
    CpuTotalLimit,
    ///协议值 "CpuPeakLimit"。
    CpuPeakLimit,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl HeavyAdReason {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::types::HeavyAdReason::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::NetworkTotalLimit => "NetworkTotalLimit",
            Self::CpuTotalLimit => "CpuTotalLimit",
            Self::CpuPeakLimit => "CpuPeakLimit",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for HeavyAdReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for HeavyAdReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "NetworkTotalLimit" => HeavyAdReason::NetworkTotalLimit,
            "CpuTotalLimit" => HeavyAdReason::CpuTotalLimit,
            "CpuPeakLimit" => HeavyAdReason::CpuPeakLimit,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for HeavyAdReason {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HeavyAdReason {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HeavyAdReason;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "NetworkTotalLimit" => HeavyAdReason::NetworkTotalLimit,
                    "CpuTotalLimit" => HeavyAdReason::CpuTotalLimit,
                    "CpuPeakLimit" => HeavyAdReason::CpuPeakLimit,
                    _ => HeavyAdReason::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(HeavyAdReason::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.type.HeavyAdResolutionStatus。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum HeavyAdResolutionStatus {
    ///协议值 "HeavyAdBlocked"。
    HeavyAdBlocked,
    ///协议值 "HeavyAdWarning"。
    HeavyAdWarning,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl HeavyAdResolutionStatus {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::types::HeavyAdResolutionStatus::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::HeavyAdBlocked => "HeavyAdBlocked",
            Self::HeavyAdWarning => "HeavyAdWarning",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for HeavyAdResolutionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for HeavyAdResolutionStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "HeavyAdBlocked" => HeavyAdResolutionStatus::HeavyAdBlocked,
            "HeavyAdWarning" => HeavyAdResolutionStatus::HeavyAdWarning,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for HeavyAdResolutionStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HeavyAdResolutionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HeavyAdResolutionStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "HeavyAdBlocked" => HeavyAdResolutionStatus::HeavyAdBlocked,
                    "HeavyAdWarning" => HeavyAdResolutionStatus::HeavyAdWarning,
                    _ => HeavyAdResolutionStatus::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(HeavyAdResolutionStatus::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.type.InsightType。
///
///协议原文：
///
///<pre>Represents the category of insight that a cookie issue falls under.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum InsightType {
    ///协议值 "GitHubResource"。
    GitHubResource,
    ///协议值 "GracePeriod"。
    GracePeriod,
    ///协议值 "Heuristics"。
    Heuristics,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl InsightType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::types::InsightType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::GitHubResource => "GitHubResource",
            Self::GracePeriod => "GracePeriod",
            Self::Heuristics => "Heuristics",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for InsightType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for InsightType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "GitHubResource" => InsightType::GitHubResource,
            "GracePeriod" => InsightType::GracePeriod,
            "Heuristics" => InsightType::Heuristics,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for InsightType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InsightType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InsightType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "GitHubResource" => InsightType::GitHubResource,
                    "GracePeriod" => InsightType::GracePeriod,
                    "Heuristics" => InsightType::Heuristics,
                    _ => InsightType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(InsightType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.InspectorIssue。
///
///协议原文：
///
///<pre>An inspector issue reported from the back-end.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct InspectorIssue {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.InspectorIssue/properties/code。
    #[serde(rename = "code")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub code: crate::audits::types::InspectorIssueCode,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.InspectorIssue/properties/details。
    #[serde(rename = "details")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub details: crate::audits::types::InspectorIssueDetails,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssue/properties/issueId。
    ///
    ///协议原文：
    ///
    ///<pre>A unique id for this issue. May be omitted if no other entity (e.g.
    ///exception, CDP message, etc.) is referencing this issue.</pre>
    ///
    ///
    #[serde(rename = "issueId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub issue_id: Option<crate::audits::types::IssueId>,
}
impl<'de> serde::Deserialize<'de> for InspectorIssue {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InspectorIssue;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.InspectorIssue/properties/code。
                    #[serde(rename = "code")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub code: crate::audits::types::InspectorIssueCode,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.InspectorIssue/properties/details。
                    #[serde(rename = "details")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub details: crate::audits::types::InspectorIssueDetails,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssue/properties/issueId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A unique id for this issue. May be omitted if no other entity (e.g.
                    ///exception, CDP message, etc.) is referencing this issue.</pre>
                    ///
                    ///
                    #[serde(rename = "issueId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub issue_id: Option<crate::audits::types::IssueId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(InspectorIssue {
                    code: fields.code,
                    details: fields.details,
                    issue_id: fields.issue_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.type.InspectorIssueCode。
///
///协议原文：
///
///<pre>A unique identifier for the type of issue. Each type may use one of the
///optional fields in InspectorIssueDetails to convey more specific
///information about the kind of issue.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum InspectorIssueCode {
    ///协议值 "CookieIssue"。
    CookieIssue,
    ///协议值 "MixedContentIssue"。
    MixedContentIssue,
    ///协议值 "BlockedByResponseIssue"。
    BlockedByResponseIssue,
    ///协议值 "HeavyAdIssue"。
    HeavyAdIssue,
    ///协议值 "ContentSecurityPolicyIssue"。
    ContentSecurityPolicyIssue,
    ///协议值 "SharedArrayBufferIssue"。
    SharedArrayBufferIssue,
    ///协议值 "CorsIssue"。
    CorsIssue,
    ///协议值 "QuirksModeIssue"。
    QuirksModeIssue,
    ///协议值 "PartitioningBlobURLIssue"。
    PartitioningBlobUrlIssue,
    ///协议值 "NavigatorUserAgentIssue"。
    NavigatorUserAgentIssue,
    ///协议值 "GenericIssue"。
    GenericIssue,
    ///协议值 "DeprecationIssue"。
    DeprecationIssue,
    ///协议值 "ClientHintIssue"。
    ClientHintIssue,
    ///协议值 "FederatedAuthRequestIssue"。
    FederatedAuthRequestIssue,
    ///协议值 "BounceTrackingIssue"。
    BounceTrackingIssue,
    ///协议值 "CookieDeprecationMetadataIssue"。
    CookieDeprecationMetadataIssue,
    ///协议值 "StylesheetLoadingIssue"。
    StylesheetLoadingIssue,
    ///协议值 "FederatedAuthUserInfoRequestIssue"。
    FederatedAuthUserInfoRequestIssue,
    ///协议值 "PropertyRuleIssue"。
    PropertyRuleIssue,
    ///协议值 "SharedDictionaryIssue"。
    SharedDictionaryIssue,
    ///协议值 "ElementAccessibilityIssue"。
    ElementAccessibilityIssue,
    ///协议值 "SRIMessageSignatureIssue"。
    SriMessageSignatureIssue,
    ///协议值 "UnencodedDigestIssue"。
    UnencodedDigestIssue,
    ///协议值 "ConnectionAllowlistIssue"。
    ConnectionAllowlistIssue,
    ///协议值 "UserReidentificationIssue"。
    UserReidentificationIssue,
    ///协议值 "PermissionElementIssue"。
    PermissionElementIssue,
    ///协议值 "PerformanceIssue"。
    PerformanceIssue,
    ///协议值 "SelectivePermissionsInterventionIssue"。
    SelectivePermissionsInterventionIssue,
    ///协议值 "EmailVerificationRequestIssue"。
    EmailVerificationRequestIssue,
    ///协议值 "LazyLoadImageIssue"。
    LazyLoadImageIssue,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl InspectorIssueCode {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::types::InspectorIssueCode::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::CookieIssue => "CookieIssue",
            Self::MixedContentIssue => "MixedContentIssue",
            Self::BlockedByResponseIssue => "BlockedByResponseIssue",
            Self::HeavyAdIssue => "HeavyAdIssue",
            Self::ContentSecurityPolicyIssue => "ContentSecurityPolicyIssue",
            Self::SharedArrayBufferIssue => "SharedArrayBufferIssue",
            Self::CorsIssue => "CorsIssue",
            Self::QuirksModeIssue => "QuirksModeIssue",
            Self::PartitioningBlobUrlIssue => "PartitioningBlobURLIssue",
            Self::NavigatorUserAgentIssue => "NavigatorUserAgentIssue",
            Self::GenericIssue => "GenericIssue",
            Self::DeprecationIssue => "DeprecationIssue",
            Self::ClientHintIssue => "ClientHintIssue",
            Self::FederatedAuthRequestIssue => "FederatedAuthRequestIssue",
            Self::BounceTrackingIssue => "BounceTrackingIssue",
            Self::CookieDeprecationMetadataIssue => "CookieDeprecationMetadataIssue",
            Self::StylesheetLoadingIssue => "StylesheetLoadingIssue",
            Self::FederatedAuthUserInfoRequestIssue => "FederatedAuthUserInfoRequestIssue",
            Self::PropertyRuleIssue => "PropertyRuleIssue",
            Self::SharedDictionaryIssue => "SharedDictionaryIssue",
            Self::ElementAccessibilityIssue => "ElementAccessibilityIssue",
            Self::SriMessageSignatureIssue => "SRIMessageSignatureIssue",
            Self::UnencodedDigestIssue => "UnencodedDigestIssue",
            Self::ConnectionAllowlistIssue => "ConnectionAllowlistIssue",
            Self::UserReidentificationIssue => "UserReidentificationIssue",
            Self::PermissionElementIssue => "PermissionElementIssue",
            Self::PerformanceIssue => "PerformanceIssue",
            Self::SelectivePermissionsInterventionIssue => "SelectivePermissionsInterventionIssue",
            Self::EmailVerificationRequestIssue => "EmailVerificationRequestIssue",
            Self::LazyLoadImageIssue => "LazyLoadImageIssue",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for InspectorIssueCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for InspectorIssueCode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "CookieIssue" => InspectorIssueCode::CookieIssue,
            "MixedContentIssue" => InspectorIssueCode::MixedContentIssue,
            "BlockedByResponseIssue" => InspectorIssueCode::BlockedByResponseIssue,
            "HeavyAdIssue" => InspectorIssueCode::HeavyAdIssue,
            "ContentSecurityPolicyIssue" => InspectorIssueCode::ContentSecurityPolicyIssue,
            "SharedArrayBufferIssue" => InspectorIssueCode::SharedArrayBufferIssue,
            "CorsIssue" => InspectorIssueCode::CorsIssue,
            "QuirksModeIssue" => InspectorIssueCode::QuirksModeIssue,
            "PartitioningBlobURLIssue" => InspectorIssueCode::PartitioningBlobUrlIssue,
            "NavigatorUserAgentIssue" => InspectorIssueCode::NavigatorUserAgentIssue,
            "GenericIssue" => InspectorIssueCode::GenericIssue,
            "DeprecationIssue" => InspectorIssueCode::DeprecationIssue,
            "ClientHintIssue" => InspectorIssueCode::ClientHintIssue,
            "FederatedAuthRequestIssue" => InspectorIssueCode::FederatedAuthRequestIssue,
            "BounceTrackingIssue" => InspectorIssueCode::BounceTrackingIssue,
            "CookieDeprecationMetadataIssue" => InspectorIssueCode::CookieDeprecationMetadataIssue,
            "StylesheetLoadingIssue" => InspectorIssueCode::StylesheetLoadingIssue,
            "FederatedAuthUserInfoRequestIssue" => {
                InspectorIssueCode::FederatedAuthUserInfoRequestIssue
            }
            "PropertyRuleIssue" => InspectorIssueCode::PropertyRuleIssue,
            "SharedDictionaryIssue" => InspectorIssueCode::SharedDictionaryIssue,
            "ElementAccessibilityIssue" => InspectorIssueCode::ElementAccessibilityIssue,
            "SRIMessageSignatureIssue" => InspectorIssueCode::SriMessageSignatureIssue,
            "UnencodedDigestIssue" => InspectorIssueCode::UnencodedDigestIssue,
            "ConnectionAllowlistIssue" => InspectorIssueCode::ConnectionAllowlistIssue,
            "UserReidentificationIssue" => InspectorIssueCode::UserReidentificationIssue,
            "PermissionElementIssue" => InspectorIssueCode::PermissionElementIssue,
            "PerformanceIssue" => InspectorIssueCode::PerformanceIssue,
            "SelectivePermissionsInterventionIssue" => {
                InspectorIssueCode::SelectivePermissionsInterventionIssue
            }
            "EmailVerificationRequestIssue" => InspectorIssueCode::EmailVerificationRequestIssue,
            "LazyLoadImageIssue" => InspectorIssueCode::LazyLoadImageIssue,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for InspectorIssueCode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InspectorIssueCode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InspectorIssueCode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "CookieIssue" => InspectorIssueCode::CookieIssue,
                    "MixedContentIssue" => InspectorIssueCode::MixedContentIssue,
                    "BlockedByResponseIssue" => InspectorIssueCode::BlockedByResponseIssue,
                    "HeavyAdIssue" => InspectorIssueCode::HeavyAdIssue,
                    "ContentSecurityPolicyIssue" => InspectorIssueCode::ContentSecurityPolicyIssue,
                    "SharedArrayBufferIssue" => InspectorIssueCode::SharedArrayBufferIssue,
                    "CorsIssue" => InspectorIssueCode::CorsIssue,
                    "QuirksModeIssue" => InspectorIssueCode::QuirksModeIssue,
                    "PartitioningBlobURLIssue" => InspectorIssueCode::PartitioningBlobUrlIssue,
                    "NavigatorUserAgentIssue" => InspectorIssueCode::NavigatorUserAgentIssue,
                    "GenericIssue" => InspectorIssueCode::GenericIssue,
                    "DeprecationIssue" => InspectorIssueCode::DeprecationIssue,
                    "ClientHintIssue" => InspectorIssueCode::ClientHintIssue,
                    "FederatedAuthRequestIssue" => InspectorIssueCode::FederatedAuthRequestIssue,
                    "BounceTrackingIssue" => InspectorIssueCode::BounceTrackingIssue,
                    "CookieDeprecationMetadataIssue" => {
                        InspectorIssueCode::CookieDeprecationMetadataIssue
                    }
                    "StylesheetLoadingIssue" => InspectorIssueCode::StylesheetLoadingIssue,
                    "FederatedAuthUserInfoRequestIssue" => {
                        InspectorIssueCode::FederatedAuthUserInfoRequestIssue
                    }
                    "PropertyRuleIssue" => InspectorIssueCode::PropertyRuleIssue,
                    "SharedDictionaryIssue" => InspectorIssueCode::SharedDictionaryIssue,
                    "ElementAccessibilityIssue" => InspectorIssueCode::ElementAccessibilityIssue,
                    "SRIMessageSignatureIssue" => InspectorIssueCode::SriMessageSignatureIssue,
                    "UnencodedDigestIssue" => InspectorIssueCode::UnencodedDigestIssue,
                    "ConnectionAllowlistIssue" => InspectorIssueCode::ConnectionAllowlistIssue,
                    "UserReidentificationIssue" => InspectorIssueCode::UserReidentificationIssue,
                    "PermissionElementIssue" => InspectorIssueCode::PermissionElementIssue,
                    "PerformanceIssue" => InspectorIssueCode::PerformanceIssue,
                    "SelectivePermissionsInterventionIssue" => {
                        InspectorIssueCode::SelectivePermissionsInterventionIssue
                    }
                    "EmailVerificationRequestIssue" => {
                        InspectorIssueCode::EmailVerificationRequestIssue
                    }
                    "LazyLoadImageIssue" => InspectorIssueCode::LazyLoadImageIssue,
                    _ => InspectorIssueCode::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(InspectorIssueCode::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.InspectorIssueDetails。
///
///协议原文：
///
///<pre>This struct holds a list of optional fields with additional information
///specific to the kind of issue. When adding a new issue code, please also
///add a new optional field to this type.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct InspectorIssueDetails {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/cookieIssueDetails。
    #[serde(rename = "cookieIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub cookie_issue_details: Option<crate::audits::types::CookieIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/mixedContentIssueDetails。
    #[serde(rename = "mixedContentIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub mixed_content_issue_details: Option<crate::audits::types::MixedContentIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/blockedByResponseIssueDetails。
    #[serde(rename = "blockedByResponseIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub blocked_by_response_issue_details:
        Option<crate::audits::types::BlockedByResponseIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/heavyAdIssueDetails。
    #[serde(rename = "heavyAdIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub heavy_ad_issue_details: Option<crate::audits::types::HeavyAdIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/contentSecurityPolicyIssueDetails。
    #[serde(rename = "contentSecurityPolicyIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub content_security_policy_issue_details:
        Option<crate::audits::types::ContentSecurityPolicyIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/sharedArrayBufferIssueDetails。
    #[serde(rename = "sharedArrayBufferIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub shared_array_buffer_issue_details:
        Option<crate::audits::types::SharedArrayBufferIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/corsIssueDetails。
    #[serde(rename = "corsIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub cors_issue_details: Option<crate::audits::types::CorsIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/quirksModeIssueDetails。
    #[serde(rename = "quirksModeIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub quirks_mode_issue_details: Option<crate::audits::types::QuirksModeIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/partitioningBlobURLIssueDetails。
    #[serde(rename = "partitioningBlobURLIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub partitioning_blob_url_issue_details:
        Option<crate::audits::types::PartitioningBlobUrlIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/navigatorUserAgentIssueDetails。此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "navigatorUserAgentIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub navigator_user_agent_issue_details:
        Option<crate::audits::types::NavigatorUserAgentIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/genericIssueDetails。
    #[serde(rename = "genericIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub generic_issue_details: Option<crate::audits::types::GenericIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/deprecationIssueDetails。
    #[serde(rename = "deprecationIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub deprecation_issue_details: Option<crate::audits::types::DeprecationIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/clientHintIssueDetails。
    #[serde(rename = "clientHintIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub client_hint_issue_details: Option<crate::audits::types::ClientHintIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/federatedAuthRequestIssueDetails。
    #[serde(rename = "federatedAuthRequestIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub federated_auth_request_issue_details:
        Option<crate::audits::types::FederatedAuthRequestIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/bounceTrackingIssueDetails。
    #[serde(rename = "bounceTrackingIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub bounce_tracking_issue_details: Option<crate::audits::types::BounceTrackingIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/cookieDeprecationMetadataIssueDetails。
    #[serde(rename = "cookieDeprecationMetadataIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub cookie_deprecation_metadata_issue_details:
        Option<crate::audits::types::CookieDeprecationMetadataIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/stylesheetLoadingIssueDetails。
    #[serde(rename = "stylesheetLoadingIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub stylesheet_loading_issue_details:
        Option<crate::audits::types::StylesheetLoadingIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/propertyRuleIssueDetails。
    #[serde(rename = "propertyRuleIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub property_rule_issue_details: Option<crate::audits::types::PropertyRuleIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/federatedAuthUserInfoRequestIssueDetails。
    #[serde(rename = "federatedAuthUserInfoRequestIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub federated_auth_user_info_request_issue_details:
        Option<crate::audits::types::FederatedAuthUserInfoRequestIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/sharedDictionaryIssueDetails。
    #[serde(rename = "sharedDictionaryIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub shared_dictionary_issue_details: Option<crate::audits::types::SharedDictionaryIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/elementAccessibilityIssueDetails。
    #[serde(rename = "elementAccessibilityIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub element_accessibility_issue_details:
        Option<crate::audits::types::ElementAccessibilityIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/sriMessageSignatureIssueDetails。
    #[serde(rename = "sriMessageSignatureIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub sri_message_signature_issue_details:
        Option<crate::audits::types::SriMessageSignatureIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/unencodedDigestIssueDetails。
    #[serde(rename = "unencodedDigestIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub unencoded_digest_issue_details: Option<crate::audits::types::UnencodedDigestIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/connectionAllowlistIssueDetails。
    #[serde(rename = "connectionAllowlistIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub connection_allowlist_issue_details:
        Option<crate::audits::types::ConnectionAllowlistIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/userReidentificationIssueDetails。
    #[serde(rename = "userReidentificationIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_reidentification_issue_details:
        Option<crate::audits::types::UserReidentificationIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/permissionElementIssueDetails。
    #[serde(rename = "permissionElementIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub permission_element_issue_details:
        Option<crate::audits::types::PermissionElementIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/performanceIssueDetails。
    #[serde(rename = "performanceIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub performance_issue_details: Option<crate::audits::types::PerformanceIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/selectivePermissionsInterventionIssueDetails。
    #[serde(rename = "selectivePermissionsInterventionIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub selective_permissions_intervention_issue_details:
        Option<crate::audits::types::SelectivePermissionsInterventionIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/emailVerificationRequestIssueDetails。
    #[serde(rename = "emailVerificationRequestIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub email_verification_request_issue_details:
        Option<crate::audits::types::EmailVerificationRequestIssueDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/lazyLoadImageIssueDetails。
    #[serde(rename = "lazyLoadImageIssueDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub lazy_load_image_issue_details: Option<crate::audits::types::LazyLoadImageIssueDetails>,
}
impl<'de> serde::Deserialize<'de> for InspectorIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InspectorIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/cookieIssueDetails。
                    #[serde(rename = "cookieIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub cookie_issue_details: Option<crate::audits::types::CookieIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/mixedContentIssueDetails。
                    #[serde(rename = "mixedContentIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub mixed_content_issue_details:
                        Option<crate::audits::types::MixedContentIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/blockedByResponseIssueDetails。
                    #[serde(rename = "blockedByResponseIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub blocked_by_response_issue_details:
                        Option<crate::audits::types::BlockedByResponseIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/heavyAdIssueDetails。
                    #[serde(rename = "heavyAdIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub heavy_ad_issue_details: Option<crate::audits::types::HeavyAdIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/contentSecurityPolicyIssueDetails。
                    #[serde(rename = "contentSecurityPolicyIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub content_security_policy_issue_details:
                        Option<crate::audits::types::ContentSecurityPolicyIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/sharedArrayBufferIssueDetails。
                    #[serde(rename = "sharedArrayBufferIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub shared_array_buffer_issue_details:
                        Option<crate::audits::types::SharedArrayBufferIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/corsIssueDetails。
                    #[serde(rename = "corsIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub cors_issue_details: Option<crate::audits::types::CorsIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/quirksModeIssueDetails。
                    #[serde(rename = "quirksModeIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub quirks_mode_issue_details:
                        Option<crate::audits::types::QuirksModeIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/partitioningBlobURLIssueDetails。
                    #[serde(rename = "partitioningBlobURLIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub partitioning_blob_url_issue_details:
                        Option<crate::audits::types::PartitioningBlobUrlIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/navigatorUserAgentIssueDetails。此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "navigatorUserAgentIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub navigator_user_agent_issue_details:
                        Option<crate::audits::types::NavigatorUserAgentIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/genericIssueDetails。
                    #[serde(rename = "genericIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub generic_issue_details: Option<crate::audits::types::GenericIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/deprecationIssueDetails。
                    #[serde(rename = "deprecationIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub deprecation_issue_details:
                        Option<crate::audits::types::DeprecationIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/clientHintIssueDetails。
                    #[serde(rename = "clientHintIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub client_hint_issue_details:
                        Option<crate::audits::types::ClientHintIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/federatedAuthRequestIssueDetails。
                    #[serde(rename = "federatedAuthRequestIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub federated_auth_request_issue_details:
                        Option<crate::audits::types::FederatedAuthRequestIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/bounceTrackingIssueDetails。
                    #[serde(rename = "bounceTrackingIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub bounce_tracking_issue_details:
                        Option<crate::audits::types::BounceTrackingIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/cookieDeprecationMetadataIssueDetails。
                    #[serde(rename = "cookieDeprecationMetadataIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub cookie_deprecation_metadata_issue_details:
                        Option<crate::audits::types::CookieDeprecationMetadataIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/stylesheetLoadingIssueDetails。
                    #[serde(rename = "stylesheetLoadingIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub stylesheet_loading_issue_details:
                        Option<crate::audits::types::StylesheetLoadingIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/propertyRuleIssueDetails。
                    #[serde(rename = "propertyRuleIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub property_rule_issue_details:
                        Option<crate::audits::types::PropertyRuleIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/federatedAuthUserInfoRequestIssueDetails。
                    #[serde(rename = "federatedAuthUserInfoRequestIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub federated_auth_user_info_request_issue_details:
                        Option<crate::audits::types::FederatedAuthUserInfoRequestIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/sharedDictionaryIssueDetails。
                    #[serde(rename = "sharedDictionaryIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub shared_dictionary_issue_details:
                        Option<crate::audits::types::SharedDictionaryIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/elementAccessibilityIssueDetails。
                    #[serde(rename = "elementAccessibilityIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub element_accessibility_issue_details:
                        Option<crate::audits::types::ElementAccessibilityIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/sriMessageSignatureIssueDetails。
                    #[serde(rename = "sriMessageSignatureIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub sri_message_signature_issue_details:
                        Option<crate::audits::types::SriMessageSignatureIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/unencodedDigestIssueDetails。
                    #[serde(rename = "unencodedDigestIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub unencoded_digest_issue_details:
                        Option<crate::audits::types::UnencodedDigestIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/connectionAllowlistIssueDetails。
                    #[serde(rename = "connectionAllowlistIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub connection_allowlist_issue_details:
                        Option<crate::audits::types::ConnectionAllowlistIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/userReidentificationIssueDetails。
                    #[serde(rename = "userReidentificationIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub user_reidentification_issue_details:
                        Option<crate::audits::types::UserReidentificationIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/permissionElementIssueDetails。
                    #[serde(rename = "permissionElementIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub permission_element_issue_details:
                        Option<crate::audits::types::PermissionElementIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/performanceIssueDetails。
                    #[serde(rename = "performanceIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub performance_issue_details:
                        Option<crate::audits::types::PerformanceIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/selectivePermissionsInterventionIssueDetails。
                    #[serde(rename = "selectivePermissionsInterventionIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub selective_permissions_intervention_issue_details:
                        Option<crate::audits::types::SelectivePermissionsInterventionIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/emailVerificationRequestIssueDetails。
                    #[serde(rename = "emailVerificationRequestIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub email_verification_request_issue_details:
                        Option<crate::audits::types::EmailVerificationRequestIssueDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.InspectorIssueDetails/properties/lazyLoadImageIssueDetails。
                    #[serde(rename = "lazyLoadImageIssueDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub lazy_load_image_issue_details:
                        Option<crate::audits::types::LazyLoadImageIssueDetails>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(InspectorIssueDetails {
                    cookie_issue_details: fields.cookie_issue_details,
                    mixed_content_issue_details: fields.mixed_content_issue_details,
                    blocked_by_response_issue_details: fields.blocked_by_response_issue_details,
                    heavy_ad_issue_details: fields.heavy_ad_issue_details,
                    content_security_policy_issue_details: fields
                        .content_security_policy_issue_details,
                    shared_array_buffer_issue_details: fields.shared_array_buffer_issue_details,
                    cors_issue_details: fields.cors_issue_details,
                    quirks_mode_issue_details: fields.quirks_mode_issue_details,
                    partitioning_blob_url_issue_details: fields.partitioning_blob_url_issue_details,
                    navigator_user_agent_issue_details: fields.navigator_user_agent_issue_details,
                    generic_issue_details: fields.generic_issue_details,
                    deprecation_issue_details: fields.deprecation_issue_details,
                    client_hint_issue_details: fields.client_hint_issue_details,
                    federated_auth_request_issue_details: fields
                        .federated_auth_request_issue_details,
                    bounce_tracking_issue_details: fields.bounce_tracking_issue_details,
                    cookie_deprecation_metadata_issue_details: fields
                        .cookie_deprecation_metadata_issue_details,
                    stylesheet_loading_issue_details: fields.stylesheet_loading_issue_details,
                    property_rule_issue_details: fields.property_rule_issue_details,
                    federated_auth_user_info_request_issue_details: fields
                        .federated_auth_user_info_request_issue_details,
                    shared_dictionary_issue_details: fields.shared_dictionary_issue_details,
                    element_accessibility_issue_details: fields.element_accessibility_issue_details,
                    sri_message_signature_issue_details: fields.sri_message_signature_issue_details,
                    unencoded_digest_issue_details: fields.unencoded_digest_issue_details,
                    connection_allowlist_issue_details: fields.connection_allowlist_issue_details,
                    user_reidentification_issue_details: fields.user_reidentification_issue_details,
                    permission_element_issue_details: fields.permission_element_issue_details,
                    performance_issue_details: fields.performance_issue_details,
                    selective_permissions_intervention_issue_details: fields
                        .selective_permissions_intervention_issue_details,
                    email_verification_request_issue_details: fields
                        .email_verification_request_issue_details,
                    lazy_load_image_issue_details: fields.lazy_load_image_issue_details,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Audits.type.IssueId。
///
///协议原文：
///
///<pre>A unique id for a DevTools inspector issue. Allows other entities (e.g.
///exceptions, CDP message, console messages, etc.) to reference an issue.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct IssueId(String);
impl IssueId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::audits::types::IssueId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::audits::types::IssueId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::audits::types::IssueId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::audits::types::IssueId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::audits::types::IssueId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.LazyLoadImageIssueDetails。
///
///协议原文：
///
///<pre>Details for issues about lazy-loaded images without explicit dimensions.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct LazyLoadImageIssueDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.LazyLoadImageIssueDetails/properties/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>DOM node of the problematic HTMLImageElement.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::BackendNodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.LazyLoadImageIssueDetails/properties/url。
    ///
    ///协议原文：
    ///
    ///<pre>URL or src attribute of the image.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.LazyLoadImageIssueDetails/properties/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Frame containing the image.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
}
impl<'de> serde::Deserialize<'de> for LazyLoadImageIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LazyLoadImageIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.LazyLoadImageIssueDetails/properties/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>DOM node of the problematic HTMLImageElement.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::BackendNodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.LazyLoadImageIssueDetails/properties/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>URL or src attribute of the image.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.LazyLoadImageIssueDetails/properties/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame containing the image.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LazyLoadImageIssueDetails {
                    node_id: fields.node_id,
                    url: fields.url,
                    frame_id: fields.frame_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.MixedContentIssueDetails。
#[derive(Debug, Clone, serde::Serialize)]
pub struct MixedContentIssueDetails {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.MixedContentIssueDetails/properties/resourceType。
    ///
    ///协议原文：
    ///
    ///<pre>The type of resource causing the mixed content issue (css, js, iframe,
    ///form,...). Marked as optional because it is mapped to from
    ///blink::mojom::RequestContextType, which will be replaced
    ///by network::mojom::RequestDestination</pre>
    ///
    ///
    #[serde(rename = "resourceType")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_type: Option<crate::audits::types::MixedContentResourceType>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.MixedContentIssueDetails/properties/resolutionStatus。
    ///
    ///协议原文：
    ///
    ///<pre>The way the mixed content issue is being resolved.</pre>
    ///
    ///
    #[serde(rename = "resolutionStatus")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub resolution_status: crate::audits::types::MixedContentResolutionStatus,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.MixedContentIssueDetails/properties/insecureURL。
    ///
    ///协议原文：
    ///
    ///<pre>The unsafe http url causing the mixed content issue.</pre>
    ///
    ///
    #[serde(rename = "insecureURL")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub insecure_url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.MixedContentIssueDetails/properties/mainResourceURL。
    ///
    ///协议原文：
    ///
    ///<pre>The url responsible for the call to an unsafe url.</pre>
    ///
    ///
    #[serde(rename = "mainResourceURL")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub main_resource_url: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.MixedContentIssueDetails/properties/request。
    ///
    ///协议原文：
    ///
    ///<pre>The mixed content request.
    ///Does not always exist (e.g. for unsafe form submission urls).</pre>
    ///
    ///
    #[serde(rename = "request")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub request: Option<crate::audits::types::AffectedRequest>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.MixedContentIssueDetails/properties/frame。
    ///
    ///协议原文：
    ///
    ///<pre>Optional because not every mixed content issue is necessarily linked to a frame.</pre>
    ///
    ///
    #[serde(rename = "frame")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub frame: Option<crate::audits::types::AffectedFrame>,
}
impl<'de> serde::Deserialize<'de> for MixedContentIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MixedContentIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.MixedContentIssueDetails/properties/resourceType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The type of resource causing the mixed content issue (css, js, iframe,
                    ///form,...). Marked as optional because it is mapped to from
                    ///blink::mojom::RequestContextType, which will be replaced
                    ///by network::mojom::RequestDestination</pre>
                    ///
                    ///
                    #[serde(rename = "resourceType")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub resource_type: Option<crate::audits::types::MixedContentResourceType>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.MixedContentIssueDetails/properties/resolutionStatus。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The way the mixed content issue is being resolved.</pre>
                    ///
                    ///
                    #[serde(rename = "resolutionStatus")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub resolution_status: crate::audits::types::MixedContentResolutionStatus,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.MixedContentIssueDetails/properties/insecureURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The unsafe http url causing the mixed content issue.</pre>
                    ///
                    ///
                    #[serde(rename = "insecureURL")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub insecure_url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.MixedContentIssueDetails/properties/mainResourceURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The url responsible for the call to an unsafe url.</pre>
                    ///
                    ///
                    #[serde(rename = "mainResourceURL")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub main_resource_url: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.MixedContentIssueDetails/properties/request。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The mixed content request.
                    ///Does not always exist (e.g. for unsafe form submission urls).</pre>
                    ///
                    ///
                    #[serde(rename = "request")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub request: Option<crate::audits::types::AffectedRequest>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.MixedContentIssueDetails/properties/frame。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Optional because not every mixed content issue is necessarily linked to a frame.</pre>
                    ///
                    ///
                    #[serde(rename = "frame")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub frame: Option<crate::audits::types::AffectedFrame>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(MixedContentIssueDetails {
                    resource_type: fields.resource_type,
                    resolution_status: fields.resolution_status,
                    insecure_url: fields.insecure_url,
                    main_resource_url: fields.main_resource_url,
                    request: fields.request,
                    frame: fields.frame,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.type.MixedContentResolutionStatus。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MixedContentResolutionStatus {
    ///协议值 "MixedContentBlocked"。
    MixedContentBlocked,
    ///协议值 "MixedContentAutomaticallyUpgraded"。
    MixedContentAutomaticallyUpgraded,
    ///协议值 "MixedContentWarning"。
    MixedContentWarning,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl MixedContentResolutionStatus {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::types::MixedContentResolutionStatus::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::MixedContentBlocked => "MixedContentBlocked",
            Self::MixedContentAutomaticallyUpgraded => "MixedContentAutomaticallyUpgraded",
            Self::MixedContentWarning => "MixedContentWarning",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for MixedContentResolutionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for MixedContentResolutionStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "MixedContentBlocked" => MixedContentResolutionStatus::MixedContentBlocked,
            "MixedContentAutomaticallyUpgraded" => {
                MixedContentResolutionStatus::MixedContentAutomaticallyUpgraded
            }
            "MixedContentWarning" => MixedContentResolutionStatus::MixedContentWarning,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for MixedContentResolutionStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MixedContentResolutionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MixedContentResolutionStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "MixedContentBlocked" => MixedContentResolutionStatus::MixedContentBlocked,
                    "MixedContentAutomaticallyUpgraded" => {
                        MixedContentResolutionStatus::MixedContentAutomaticallyUpgraded
                    }
                    "MixedContentWarning" => MixedContentResolutionStatus::MixedContentWarning,
                    _ => MixedContentResolutionStatus::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(MixedContentResolutionStatus::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.type.MixedContentResourceType。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MixedContentResourceType {
    ///协议值 "Audio"。
    Audio,
    ///协议值 "Beacon"。
    Beacon,
    ///协议值 "CSPReport"。
    CspReport,
    ///协议值 "Download"。
    Download,
    ///协议值 "EventSource"。
    EventSource,
    ///协议值 "Favicon"。
    Favicon,
    ///协议值 "Font"。
    Font,
    ///协议值 "Form"。
    Form,
    ///协议值 "Frame"。
    Frame,
    ///协议值 "Image"。
    Image,
    ///协议值 "Import"。
    Import,
    ///协议值 "JSON"。
    Json,
    ///协议值 "Manifest"。
    Manifest,
    ///协议值 "Ping"。
    Ping,
    ///协议值 "PluginData"。
    PluginData,
    ///协议值 "PluginResource"。
    PluginResource,
    ///协议值 "Prefetch"。
    Prefetch,
    ///协议值 "Resource"。
    Resource,
    ///协议值 "Script"。
    Script,
    ///协议值 "ServiceWorker"。
    ServiceWorker,
    ///协议值 "SharedWorker"。
    SharedWorker,
    ///协议值 "SpeculationRules"。
    SpeculationRules,
    ///协议值 "Stylesheet"。
    Stylesheet,
    ///协议值 "Track"。
    Track,
    ///协议值 "Video"。
    Video,
    ///协议值 "Worker"。
    Worker,
    ///协议值 "XMLHttpRequest"。
    XmlHttpRequest,
    ///协议值 "XSLT"。
    Xslt,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl MixedContentResourceType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::types::MixedContentResourceType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Audio => "Audio",
            Self::Beacon => "Beacon",
            Self::CspReport => "CSPReport",
            Self::Download => "Download",
            Self::EventSource => "EventSource",
            Self::Favicon => "Favicon",
            Self::Font => "Font",
            Self::Form => "Form",
            Self::Frame => "Frame",
            Self::Image => "Image",
            Self::Import => "Import",
            Self::Json => "JSON",
            Self::Manifest => "Manifest",
            Self::Ping => "Ping",
            Self::PluginData => "PluginData",
            Self::PluginResource => "PluginResource",
            Self::Prefetch => "Prefetch",
            Self::Resource => "Resource",
            Self::Script => "Script",
            Self::ServiceWorker => "ServiceWorker",
            Self::SharedWorker => "SharedWorker",
            Self::SpeculationRules => "SpeculationRules",
            Self::Stylesheet => "Stylesheet",
            Self::Track => "Track",
            Self::Video => "Video",
            Self::Worker => "Worker",
            Self::XmlHttpRequest => "XMLHttpRequest",
            Self::Xslt => "XSLT",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for MixedContentResourceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for MixedContentResourceType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Audio" => MixedContentResourceType::Audio,
            "Beacon" => MixedContentResourceType::Beacon,
            "CSPReport" => MixedContentResourceType::CspReport,
            "Download" => MixedContentResourceType::Download,
            "EventSource" => MixedContentResourceType::EventSource,
            "Favicon" => MixedContentResourceType::Favicon,
            "Font" => MixedContentResourceType::Font,
            "Form" => MixedContentResourceType::Form,
            "Frame" => MixedContentResourceType::Frame,
            "Image" => MixedContentResourceType::Image,
            "Import" => MixedContentResourceType::Import,
            "JSON" => MixedContentResourceType::Json,
            "Manifest" => MixedContentResourceType::Manifest,
            "Ping" => MixedContentResourceType::Ping,
            "PluginData" => MixedContentResourceType::PluginData,
            "PluginResource" => MixedContentResourceType::PluginResource,
            "Prefetch" => MixedContentResourceType::Prefetch,
            "Resource" => MixedContentResourceType::Resource,
            "Script" => MixedContentResourceType::Script,
            "ServiceWorker" => MixedContentResourceType::ServiceWorker,
            "SharedWorker" => MixedContentResourceType::SharedWorker,
            "SpeculationRules" => MixedContentResourceType::SpeculationRules,
            "Stylesheet" => MixedContentResourceType::Stylesheet,
            "Track" => MixedContentResourceType::Track,
            "Video" => MixedContentResourceType::Video,
            "Worker" => MixedContentResourceType::Worker,
            "XMLHttpRequest" => MixedContentResourceType::XmlHttpRequest,
            "XSLT" => MixedContentResourceType::Xslt,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for MixedContentResourceType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MixedContentResourceType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MixedContentResourceType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Audio" => MixedContentResourceType::Audio,
                    "Beacon" => MixedContentResourceType::Beacon,
                    "CSPReport" => MixedContentResourceType::CspReport,
                    "Download" => MixedContentResourceType::Download,
                    "EventSource" => MixedContentResourceType::EventSource,
                    "Favicon" => MixedContentResourceType::Favicon,
                    "Font" => MixedContentResourceType::Font,
                    "Form" => MixedContentResourceType::Form,
                    "Frame" => MixedContentResourceType::Frame,
                    "Image" => MixedContentResourceType::Image,
                    "Import" => MixedContentResourceType::Import,
                    "JSON" => MixedContentResourceType::Json,
                    "Manifest" => MixedContentResourceType::Manifest,
                    "Ping" => MixedContentResourceType::Ping,
                    "PluginData" => MixedContentResourceType::PluginData,
                    "PluginResource" => MixedContentResourceType::PluginResource,
                    "Prefetch" => MixedContentResourceType::Prefetch,
                    "Resource" => MixedContentResourceType::Resource,
                    "Script" => MixedContentResourceType::Script,
                    "ServiceWorker" => MixedContentResourceType::ServiceWorker,
                    "SharedWorker" => MixedContentResourceType::SharedWorker,
                    "SpeculationRules" => MixedContentResourceType::SpeculationRules,
                    "Stylesheet" => MixedContentResourceType::Stylesheet,
                    "Track" => MixedContentResourceType::Track,
                    "Video" => MixedContentResourceType::Video,
                    "Worker" => MixedContentResourceType::Worker,
                    "XMLHttpRequest" => MixedContentResourceType::XmlHttpRequest,
                    "XSLT" => MixedContentResourceType::Xslt,
                    _ => MixedContentResourceType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(MixedContentResourceType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.NavigatorUserAgentIssueDetails。此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, serde::Serialize)]
pub struct NavigatorUserAgentIssueDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.NavigatorUserAgentIssueDetails/properties/url。
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.NavigatorUserAgentIssueDetails/properties/location。
    #[serde(rename = "location")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub location: Option<crate::audits::types::SourceCodeLocation>,
}
impl<'de> serde::Deserialize<'de> for NavigatorUserAgentIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NavigatorUserAgentIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.NavigatorUserAgentIssueDetails/properties/url。
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.NavigatorUserAgentIssueDetails/properties/location。
                    #[serde(rename = "location")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub location: Option<crate::audits::types::SourceCodeLocation>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(NavigatorUserAgentIssueDetails {
                    url: fields.url,
                    location: fields.location,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.type.PartitioningBlobURLInfo。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PartitioningBlobUrlInfo {
    ///协议值 "BlockedCrossPartitionFetching"。
    BlockedCrossPartitionFetching,
    ///协议值 "EnforceNoopenerForNavigation"。
    EnforceNoopenerForNavigation,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl PartitioningBlobUrlInfo {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::types::PartitioningBlobUrlInfo::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::BlockedCrossPartitionFetching => "BlockedCrossPartitionFetching",
            Self::EnforceNoopenerForNavigation => "EnforceNoopenerForNavigation",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for PartitioningBlobUrlInfo {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for PartitioningBlobUrlInfo {
    fn from(value: String) -> Self {
        match value.as_str() {
            "BlockedCrossPartitionFetching" => {
                PartitioningBlobUrlInfo::BlockedCrossPartitionFetching
            }
            "EnforceNoopenerForNavigation" => PartitioningBlobUrlInfo::EnforceNoopenerForNavigation,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for PartitioningBlobUrlInfo {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PartitioningBlobUrlInfo {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PartitioningBlobUrlInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "BlockedCrossPartitionFetching" => {
                        PartitioningBlobUrlInfo::BlockedCrossPartitionFetching
                    }
                    "EnforceNoopenerForNavigation" => {
                        PartitioningBlobUrlInfo::EnforceNoopenerForNavigation
                    }
                    _ => PartitioningBlobUrlInfo::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(PartitioningBlobUrlInfo::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.PartitioningBlobURLIssueDetails。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PartitioningBlobUrlIssueDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.PartitioningBlobURLIssueDetails/properties/url。
    ///
    ///协议原文：
    ///
    ///<pre>The BlobURL that failed to load.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.PartitioningBlobURLIssueDetails/properties/partitioningBlobURLInfo。
    ///
    ///协议原文：
    ///
    ///<pre>Additional information about the Partitioning Blob URL issue.</pre>
    ///
    ///
    #[serde(rename = "partitioningBlobURLInfo")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub partitioning_blob_url_info: crate::audits::types::PartitioningBlobUrlInfo,
}
impl<'de> serde::Deserialize<'de> for PartitioningBlobUrlIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PartitioningBlobUrlIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.PartitioningBlobURLIssueDetails/properties/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The BlobURL that failed to load.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.PartitioningBlobURLIssueDetails/properties/partitioningBlobURLInfo。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Additional information about the Partitioning Blob URL issue.</pre>
                    ///
                    ///
                    #[serde(rename = "partitioningBlobURLInfo")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub partitioning_blob_url_info: crate::audits::types::PartitioningBlobUrlInfo,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PartitioningBlobUrlIssueDetails {
                    url: fields.url,
                    partitioning_blob_url_info: fields.partitioning_blob_url_info,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.PerformanceIssueDetails。
///
///协议原文：
///
///<pre>Details for a performance issue.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct PerformanceIssueDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.PerformanceIssueDetails/properties/performanceIssueType。
    #[serde(rename = "performanceIssueType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub performance_issue_type: crate::audits::types::PerformanceIssueType,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.PerformanceIssueDetails/properties/sourceCodeLocation。
    #[serde(rename = "sourceCodeLocation")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_code_location: Option<crate::audits::types::SourceCodeLocation>,
}
impl<'de> serde::Deserialize<'de> for PerformanceIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PerformanceIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.PerformanceIssueDetails/properties/performanceIssueType。
                    #[serde(rename = "performanceIssueType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub performance_issue_type: crate::audits::types::PerformanceIssueType,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.PerformanceIssueDetails/properties/sourceCodeLocation。
                    #[serde(rename = "sourceCodeLocation")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub source_code_location: Option<crate::audits::types::SourceCodeLocation>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PerformanceIssueDetails {
                    performance_issue_type: fields.performance_issue_type,
                    source_code_location: fields.source_code_location,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.type.PerformanceIssueType。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PerformanceIssueType {
    ///协议值 "DocumentCookie"。
    DocumentCookie,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl PerformanceIssueType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::types::PerformanceIssueType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::DocumentCookie => "DocumentCookie",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for PerformanceIssueType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for PerformanceIssueType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "DocumentCookie" => PerformanceIssueType::DocumentCookie,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for PerformanceIssueType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PerformanceIssueType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PerformanceIssueType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "DocumentCookie" => PerformanceIssueType::DocumentCookie,
                    _ => PerformanceIssueType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(PerformanceIssueType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.PermissionElementIssueDetails。
///
///协议原文：
///
///<pre>This issue warns about improper usage of the &lt;permission&gt; element.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct PermissionElementIssueDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.PermissionElementIssueDetails/properties/issueType。
    #[serde(rename = "issueType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub issue_type: crate::audits::types::PermissionElementIssueType,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.PermissionElementIssueDetails/properties/type。
    ///
    ///协议原文：
    ///
    ///<pre>The value of the type attribute.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.PermissionElementIssueDetails/properties/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>The node ID of the &lt;permission&gt; element.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub node_id: Option<crate::dom::types::BackendNodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.PermissionElementIssueDetails/properties/isWarning。
    ///
    ///协议原文：
    ///
    ///<pre>True if the issue is a warning, false if it is an error.</pre>
    ///
    ///
    #[serde(rename = "isWarning")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_warning: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.PermissionElementIssueDetails/properties/permissionName。
    ///
    ///协议原文：
    ///
    ///<pre>Fields for message construction:
    ///Used for messages that reference a specific permission name</pre>
    ///
    ///
    #[serde(rename = "permissionName")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub permission_name: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.PermissionElementIssueDetails/properties/occluderNodeInfo。
    ///
    ///协议原文：
    ///
    ///<pre>Used for messages about occlusion</pre>
    ///
    ///
    #[serde(rename = "occluderNodeInfo")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub occluder_node_info: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.PermissionElementIssueDetails/properties/occluderParentNodeInfo。
    ///
    ///协议原文：
    ///
    ///<pre>Used for messages about occluder's parent</pre>
    ///
    ///
    #[serde(rename = "occluderParentNodeInfo")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub occluder_parent_node_info: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.PermissionElementIssueDetails/properties/disableReason。
    ///
    ///协议原文：
    ///
    ///<pre>Used for messages about activation disabled reason</pre>
    ///
    ///
    #[serde(rename = "disableReason")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_reason: Option<String>,
}
impl<'de> serde::Deserialize<'de> for PermissionElementIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PermissionElementIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.PermissionElementIssueDetails/properties/issueType。
                    #[serde(rename = "issueType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub issue_type: crate::audits::types::PermissionElementIssueType,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.PermissionElementIssueDetails/properties/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The value of the type attribute.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub r#type: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.PermissionElementIssueDetails/properties/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The node ID of the &lt;permission&gt; element.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub node_id: Option<crate::dom::types::BackendNodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.PermissionElementIssueDetails/properties/isWarning。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if the issue is a warning, false if it is an error.</pre>
                    ///
                    ///
                    #[serde(rename = "isWarning")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub is_warning: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.PermissionElementIssueDetails/properties/permissionName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Fields for message construction:
                    ///Used for messages that reference a specific permission name</pre>
                    ///
                    ///
                    #[serde(rename = "permissionName")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub permission_name: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.PermissionElementIssueDetails/properties/occluderNodeInfo。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Used for messages about occlusion</pre>
                    ///
                    ///
                    #[serde(rename = "occluderNodeInfo")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub occluder_node_info: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.PermissionElementIssueDetails/properties/occluderParentNodeInfo。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Used for messages about occluder's parent</pre>
                    ///
                    ///
                    #[serde(rename = "occluderParentNodeInfo")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub occluder_parent_node_info: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.PermissionElementIssueDetails/properties/disableReason。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Used for messages about activation disabled reason</pre>
                    ///
                    ///
                    #[serde(rename = "disableReason")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub disable_reason: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PermissionElementIssueDetails {
                    issue_type: fields.issue_type,
                    r#type: fields.r#type,
                    node_id: fields.node_id,
                    is_warning: fields.is_warning,
                    permission_name: fields.permission_name,
                    occluder_node_info: fields.occluder_node_info,
                    occluder_parent_node_info: fields.occluder_parent_node_info,
                    disable_reason: fields.disable_reason,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.type.PermissionElementIssueType。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PermissionElementIssueType {
    ///协议值 "InvalidType"。
    InvalidType,
    ///协议值 "FencedFrameDisallowed"。
    FencedFrameDisallowed,
    ///协议值 "CspFrameAncestorsMissing"。
    CspFrameAncestorsMissing,
    ///协议值 "PermissionsPolicyBlocked"。
    PermissionsPolicyBlocked,
    ///协议值 "PaddingRightUnsupported"。
    PaddingRightUnsupported,
    ///协议值 "PaddingBottomUnsupported"。
    PaddingBottomUnsupported,
    ///协议值 "InsetBoxShadowUnsupported"。
    InsetBoxShadowUnsupported,
    ///协议值 "RequestInProgress"。
    RequestInProgress,
    ///协议值 "UntrustedEvent"。
    UntrustedEvent,
    ///协议值 "RegistrationFailed"。
    RegistrationFailed,
    ///协议值 "TypeNotSupported"。
    TypeNotSupported,
    ///协议值 "InvalidTypeActivation"。
    InvalidTypeActivation,
    ///协议值 "SecurityChecksFailed"。
    SecurityChecksFailed,
    ///协议值 "ActivationDisabled"。
    ActivationDisabled,
    ///协议值 "GeolocationDeprecated"。
    GeolocationDeprecated,
    ///协议值 "InvalidDisplayStyle"。
    InvalidDisplayStyle,
    ///协议值 "NonOpaqueColor"。
    NonOpaqueColor,
    ///协议值 "LowContrast"。
    LowContrast,
    ///协议值 "FontSizeTooSmall"。
    FontSizeTooSmall,
    ///协议值 "FontSizeTooLarge"。
    FontSizeTooLarge,
    ///协议值 "InvalidSizeValue"。
    InvalidSizeValue,
    ///协议值 "NonSecureContext"。
    NonSecureContext,
    ///协议值 "MissingTransientUserActivation"。
    MissingTransientUserActivation,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl PermissionElementIssueType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::types::PermissionElementIssueType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::InvalidType => "InvalidType",
            Self::FencedFrameDisallowed => "FencedFrameDisallowed",
            Self::CspFrameAncestorsMissing => "CspFrameAncestorsMissing",
            Self::PermissionsPolicyBlocked => "PermissionsPolicyBlocked",
            Self::PaddingRightUnsupported => "PaddingRightUnsupported",
            Self::PaddingBottomUnsupported => "PaddingBottomUnsupported",
            Self::InsetBoxShadowUnsupported => "InsetBoxShadowUnsupported",
            Self::RequestInProgress => "RequestInProgress",
            Self::UntrustedEvent => "UntrustedEvent",
            Self::RegistrationFailed => "RegistrationFailed",
            Self::TypeNotSupported => "TypeNotSupported",
            Self::InvalidTypeActivation => "InvalidTypeActivation",
            Self::SecurityChecksFailed => "SecurityChecksFailed",
            Self::ActivationDisabled => "ActivationDisabled",
            Self::GeolocationDeprecated => "GeolocationDeprecated",
            Self::InvalidDisplayStyle => "InvalidDisplayStyle",
            Self::NonOpaqueColor => "NonOpaqueColor",
            Self::LowContrast => "LowContrast",
            Self::FontSizeTooSmall => "FontSizeTooSmall",
            Self::FontSizeTooLarge => "FontSizeTooLarge",
            Self::InvalidSizeValue => "InvalidSizeValue",
            Self::NonSecureContext => "NonSecureContext",
            Self::MissingTransientUserActivation => "MissingTransientUserActivation",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for PermissionElementIssueType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for PermissionElementIssueType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "InvalidType" => PermissionElementIssueType::InvalidType,
            "FencedFrameDisallowed" => PermissionElementIssueType::FencedFrameDisallowed,
            "CspFrameAncestorsMissing" => PermissionElementIssueType::CspFrameAncestorsMissing,
            "PermissionsPolicyBlocked" => PermissionElementIssueType::PermissionsPolicyBlocked,
            "PaddingRightUnsupported" => PermissionElementIssueType::PaddingRightUnsupported,
            "PaddingBottomUnsupported" => PermissionElementIssueType::PaddingBottomUnsupported,
            "InsetBoxShadowUnsupported" => PermissionElementIssueType::InsetBoxShadowUnsupported,
            "RequestInProgress" => PermissionElementIssueType::RequestInProgress,
            "UntrustedEvent" => PermissionElementIssueType::UntrustedEvent,
            "RegistrationFailed" => PermissionElementIssueType::RegistrationFailed,
            "TypeNotSupported" => PermissionElementIssueType::TypeNotSupported,
            "InvalidTypeActivation" => PermissionElementIssueType::InvalidTypeActivation,
            "SecurityChecksFailed" => PermissionElementIssueType::SecurityChecksFailed,
            "ActivationDisabled" => PermissionElementIssueType::ActivationDisabled,
            "GeolocationDeprecated" => PermissionElementIssueType::GeolocationDeprecated,
            "InvalidDisplayStyle" => PermissionElementIssueType::InvalidDisplayStyle,
            "NonOpaqueColor" => PermissionElementIssueType::NonOpaqueColor,
            "LowContrast" => PermissionElementIssueType::LowContrast,
            "FontSizeTooSmall" => PermissionElementIssueType::FontSizeTooSmall,
            "FontSizeTooLarge" => PermissionElementIssueType::FontSizeTooLarge,
            "InvalidSizeValue" => PermissionElementIssueType::InvalidSizeValue,
            "NonSecureContext" => PermissionElementIssueType::NonSecureContext,
            "MissingTransientUserActivation" => {
                PermissionElementIssueType::MissingTransientUserActivation
            }
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for PermissionElementIssueType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PermissionElementIssueType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PermissionElementIssueType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "InvalidType" => PermissionElementIssueType::InvalidType,
                    "FencedFrameDisallowed" => PermissionElementIssueType::FencedFrameDisallowed,
                    "CspFrameAncestorsMissing" => {
                        PermissionElementIssueType::CspFrameAncestorsMissing
                    }
                    "PermissionsPolicyBlocked" => {
                        PermissionElementIssueType::PermissionsPolicyBlocked
                    }
                    "PaddingRightUnsupported" => {
                        PermissionElementIssueType::PaddingRightUnsupported
                    }
                    "PaddingBottomUnsupported" => {
                        PermissionElementIssueType::PaddingBottomUnsupported
                    }
                    "InsetBoxShadowUnsupported" => {
                        PermissionElementIssueType::InsetBoxShadowUnsupported
                    }
                    "RequestInProgress" => PermissionElementIssueType::RequestInProgress,
                    "UntrustedEvent" => PermissionElementIssueType::UntrustedEvent,
                    "RegistrationFailed" => PermissionElementIssueType::RegistrationFailed,
                    "TypeNotSupported" => PermissionElementIssueType::TypeNotSupported,
                    "InvalidTypeActivation" => PermissionElementIssueType::InvalidTypeActivation,
                    "SecurityChecksFailed" => PermissionElementIssueType::SecurityChecksFailed,
                    "ActivationDisabled" => PermissionElementIssueType::ActivationDisabled,
                    "GeolocationDeprecated" => PermissionElementIssueType::GeolocationDeprecated,
                    "InvalidDisplayStyle" => PermissionElementIssueType::InvalidDisplayStyle,
                    "NonOpaqueColor" => PermissionElementIssueType::NonOpaqueColor,
                    "LowContrast" => PermissionElementIssueType::LowContrast,
                    "FontSizeTooSmall" => PermissionElementIssueType::FontSizeTooSmall,
                    "FontSizeTooLarge" => PermissionElementIssueType::FontSizeTooLarge,
                    "InvalidSizeValue" => PermissionElementIssueType::InvalidSizeValue,
                    "NonSecureContext" => PermissionElementIssueType::NonSecureContext,
                    "MissingTransientUserActivation" => {
                        PermissionElementIssueType::MissingTransientUserActivation
                    }
                    _ => PermissionElementIssueType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(PermissionElementIssueType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.PropertyRuleIssueDetails。
///
///协议原文：
///
///<pre>This issue warns about errors in property rules that lead to property
///registrations being ignored.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct PropertyRuleIssueDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.PropertyRuleIssueDetails/properties/sourceCodeLocation。
    ///
    ///协议原文：
    ///
    ///<pre>Source code position of the property rule.</pre>
    ///
    ///
    #[serde(rename = "sourceCodeLocation")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub source_code_location: crate::audits::types::SourceCodeLocation,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.PropertyRuleIssueDetails/properties/propertyRuleIssueReason。
    ///
    ///协议原文：
    ///
    ///<pre>Reason why the property rule was discarded.</pre>
    ///
    ///
    #[serde(rename = "propertyRuleIssueReason")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub property_rule_issue_reason: crate::audits::types::PropertyRuleIssueReason,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.PropertyRuleIssueDetails/properties/propertyValue。
    ///
    ///协议原文：
    ///
    ///<pre>The value of the property rule property that failed to parse</pre>
    ///
    ///
    #[serde(rename = "propertyValue")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub property_value: Option<String>,
}
impl<'de> serde::Deserialize<'de> for PropertyRuleIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PropertyRuleIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.PropertyRuleIssueDetails/properties/sourceCodeLocation。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Source code position of the property rule.</pre>
                    ///
                    ///
                    #[serde(rename = "sourceCodeLocation")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub source_code_location: crate::audits::types::SourceCodeLocation,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.PropertyRuleIssueDetails/properties/propertyRuleIssueReason。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Reason why the property rule was discarded.</pre>
                    ///
                    ///
                    #[serde(rename = "propertyRuleIssueReason")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub property_rule_issue_reason: crate::audits::types::PropertyRuleIssueReason,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.PropertyRuleIssueDetails/properties/propertyValue。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The value of the property rule property that failed to parse</pre>
                    ///
                    ///
                    #[serde(rename = "propertyValue")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub property_value: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PropertyRuleIssueDetails {
                    source_code_location: fields.source_code_location,
                    property_rule_issue_reason: fields.property_rule_issue_reason,
                    property_value: fields.property_value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.type.PropertyRuleIssueReason。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PropertyRuleIssueReason {
    ///协议值 "InvalidSyntax"。
    InvalidSyntax,
    ///协议值 "InvalidInitialValue"。
    InvalidInitialValue,
    ///协议值 "InvalidInherits"。
    InvalidInherits,
    ///协议值 "InvalidName"。
    InvalidName,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl PropertyRuleIssueReason {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::types::PropertyRuleIssueReason::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::InvalidSyntax => "InvalidSyntax",
            Self::InvalidInitialValue => "InvalidInitialValue",
            Self::InvalidInherits => "InvalidInherits",
            Self::InvalidName => "InvalidName",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for PropertyRuleIssueReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for PropertyRuleIssueReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "InvalidSyntax" => PropertyRuleIssueReason::InvalidSyntax,
            "InvalidInitialValue" => PropertyRuleIssueReason::InvalidInitialValue,
            "InvalidInherits" => PropertyRuleIssueReason::InvalidInherits,
            "InvalidName" => PropertyRuleIssueReason::InvalidName,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for PropertyRuleIssueReason {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PropertyRuleIssueReason {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PropertyRuleIssueReason;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "InvalidSyntax" => PropertyRuleIssueReason::InvalidSyntax,
                    "InvalidInitialValue" => PropertyRuleIssueReason::InvalidInitialValue,
                    "InvalidInherits" => PropertyRuleIssueReason::InvalidInherits,
                    "InvalidName" => PropertyRuleIssueReason::InvalidName,
                    _ => PropertyRuleIssueReason::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(PropertyRuleIssueReason::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.QuirksModeIssueDetails。
///
///协议原文：
///
///<pre>Details for issues about documents in Quirks Mode
///or Limited Quirks Mode that affects page layouting.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct QuirksModeIssueDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.QuirksModeIssueDetails/properties/isLimitedQuirksMode。
    ///
    ///协议原文：
    ///
    ///<pre>If false, it means the document's mode is "quirks"
    ///instead of "limited-quirks".</pre>
    ///
    ///
    #[serde(rename = "isLimitedQuirksMode")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub is_limited_quirks_mode: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.QuirksModeIssueDetails/properties/documentNodeId。
    #[serde(rename = "documentNodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub document_node_id: crate::dom::types::BackendNodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.QuirksModeIssueDetails/properties/url。
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.QuirksModeIssueDetails/properties/frameId。
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.QuirksModeIssueDetails/properties/loaderId。
    #[serde(rename = "loaderId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub loader_id: crate::network::types::LoaderId,
}
impl<'de> serde::Deserialize<'de> for QuirksModeIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = QuirksModeIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.QuirksModeIssueDetails/properties/isLimitedQuirksMode。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If false, it means the document's mode is "quirks"
                    ///instead of "limited-quirks".</pre>
                    ///
                    ///
                    #[serde(rename = "isLimitedQuirksMode")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub is_limited_quirks_mode: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.QuirksModeIssueDetails/properties/documentNodeId。
                    #[serde(rename = "documentNodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub document_node_id: crate::dom::types::BackendNodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.QuirksModeIssueDetails/properties/url。
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.QuirksModeIssueDetails/properties/frameId。
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.QuirksModeIssueDetails/properties/loaderId。
                    #[serde(rename = "loaderId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub loader_id: crate::network::types::LoaderId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(QuirksModeIssueDetails {
                    is_limited_quirks_mode: fields.is_limited_quirks_mode,
                    document_node_id: fields.document_node_id,
                    url: fields.url,
                    frame_id: fields.frame_id,
                    loader_id: fields.loader_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.type.SRIMessageSignatureError。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SriMessageSignatureError {
    ///协议值 "MissingSignatureHeader"。
    MissingSignatureHeader,
    ///协议值 "MissingSignatureInputHeader"。
    MissingSignatureInputHeader,
    ///协议值 "InvalidSignatureHeader"。
    InvalidSignatureHeader,
    ///协议值 "InvalidSignatureInputHeader"。
    InvalidSignatureInputHeader,
    ///协议值 "SignatureHeaderValueIsNotByteSequence"。
    SignatureHeaderValueIsNotByteSequence,
    ///协议值 "SignatureHeaderValueIsParameterized"。
    SignatureHeaderValueIsParameterized,
    ///协议值 "SignatureHeaderValueIsIncorrectLength"。
    SignatureHeaderValueIsIncorrectLength,
    ///协议值 "SignatureInputHeaderMissingLabel"。
    SignatureInputHeaderMissingLabel,
    ///协议值 "SignatureInputHeaderValueNotInnerList"。
    SignatureInputHeaderValueNotInnerList,
    ///协议值 "SignatureInputHeaderValueMissingComponents"。
    SignatureInputHeaderValueMissingComponents,
    ///协议值 "SignatureInputHeaderInvalidComponentType"。
    SignatureInputHeaderInvalidComponentType,
    ///协议值 "SignatureInputHeaderInvalidComponentName"。
    SignatureInputHeaderInvalidComponentName,
    ///协议值 "SignatureInputHeaderInvalidHeaderComponentParameter"。
    SignatureInputHeaderInvalidHeaderComponentParameter,
    ///协议值 "SignatureInputHeaderInvalidDerivedComponentParameter"。
    SignatureInputHeaderInvalidDerivedComponentParameter,
    ///协议值 "SignatureInputHeaderKeyIdLength"。
    SignatureInputHeaderKeyIdLength,
    ///协议值 "SignatureInputHeaderInvalidParameter"。
    SignatureInputHeaderInvalidParameter,
    ///协议值 "SignatureInputHeaderMissingRequiredParameters"。
    SignatureInputHeaderMissingRequiredParameters,
    ///协议值 "ValidationFailedSignatureExpired"。
    ValidationFailedSignatureExpired,
    ///协议值 "ValidationFailedInvalidLength"。
    ValidationFailedInvalidLength,
    ///协议值 "ValidationFailedSignatureMismatch"。
    ValidationFailedSignatureMismatch,
    ///协议值 "ValidationFailedIntegrityMismatch"。
    ValidationFailedIntegrityMismatch,
    ///协议值 "SignatureBaseUnknownDerivedComponent"。
    SignatureBaseUnknownDerivedComponent,
    ///协议值 "SignatureBaseMissingHeader"。
    SignatureBaseMissingHeader,
    ///协议值 "SignatureBaseInvalidUnencodedDigest"。
    SignatureBaseInvalidUnencodedDigest,
    ///协议值 "SignatureBaseUnsupportedComponent"。
    SignatureBaseUnsupportedComponent,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl SriMessageSignatureError {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::types::SriMessageSignatureError::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::MissingSignatureHeader => "MissingSignatureHeader",
            Self::MissingSignatureInputHeader => "MissingSignatureInputHeader",
            Self::InvalidSignatureHeader => "InvalidSignatureHeader",
            Self::InvalidSignatureInputHeader => "InvalidSignatureInputHeader",
            Self::SignatureHeaderValueIsNotByteSequence => "SignatureHeaderValueIsNotByteSequence",
            Self::SignatureHeaderValueIsParameterized => "SignatureHeaderValueIsParameterized",
            Self::SignatureHeaderValueIsIncorrectLength => "SignatureHeaderValueIsIncorrectLength",
            Self::SignatureInputHeaderMissingLabel => "SignatureInputHeaderMissingLabel",
            Self::SignatureInputHeaderValueNotInnerList => "SignatureInputHeaderValueNotInnerList",
            Self::SignatureInputHeaderValueMissingComponents => {
                "SignatureInputHeaderValueMissingComponents"
            }
            Self::SignatureInputHeaderInvalidComponentType => {
                "SignatureInputHeaderInvalidComponentType"
            }
            Self::SignatureInputHeaderInvalidComponentName => {
                "SignatureInputHeaderInvalidComponentName"
            }
            Self::SignatureInputHeaderInvalidHeaderComponentParameter => {
                "SignatureInputHeaderInvalidHeaderComponentParameter"
            }
            Self::SignatureInputHeaderInvalidDerivedComponentParameter => {
                "SignatureInputHeaderInvalidDerivedComponentParameter"
            }
            Self::SignatureInputHeaderKeyIdLength => "SignatureInputHeaderKeyIdLength",
            Self::SignatureInputHeaderInvalidParameter => "SignatureInputHeaderInvalidParameter",
            Self::SignatureInputHeaderMissingRequiredParameters => {
                "SignatureInputHeaderMissingRequiredParameters"
            }
            Self::ValidationFailedSignatureExpired => "ValidationFailedSignatureExpired",
            Self::ValidationFailedInvalidLength => "ValidationFailedInvalidLength",
            Self::ValidationFailedSignatureMismatch => "ValidationFailedSignatureMismatch",
            Self::ValidationFailedIntegrityMismatch => "ValidationFailedIntegrityMismatch",
            Self::SignatureBaseUnknownDerivedComponent => "SignatureBaseUnknownDerivedComponent",
            Self::SignatureBaseMissingHeader => "SignatureBaseMissingHeader",
            Self::SignatureBaseInvalidUnencodedDigest => "SignatureBaseInvalidUnencodedDigest",
            Self::SignatureBaseUnsupportedComponent => "SignatureBaseUnsupportedComponent",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for SriMessageSignatureError {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for SriMessageSignatureError {
    fn from(value: String) -> Self {
        match value.as_str() {
            "MissingSignatureHeader" => SriMessageSignatureError::MissingSignatureHeader,
            "MissingSignatureInputHeader" => SriMessageSignatureError::MissingSignatureInputHeader,
            "InvalidSignatureHeader" => SriMessageSignatureError::InvalidSignatureHeader,
            "InvalidSignatureInputHeader" => SriMessageSignatureError::InvalidSignatureInputHeader,
            "SignatureHeaderValueIsNotByteSequence" => {
                SriMessageSignatureError::SignatureHeaderValueIsNotByteSequence
            }
            "SignatureHeaderValueIsParameterized" => {
                SriMessageSignatureError::SignatureHeaderValueIsParameterized
            }
            "SignatureHeaderValueIsIncorrectLength" => {
                SriMessageSignatureError::SignatureHeaderValueIsIncorrectLength
            }
            "SignatureInputHeaderMissingLabel" => {
                SriMessageSignatureError::SignatureInputHeaderMissingLabel
            }
            "SignatureInputHeaderValueNotInnerList" => {
                SriMessageSignatureError::SignatureInputHeaderValueNotInnerList
            }
            "SignatureInputHeaderValueMissingComponents" => {
                SriMessageSignatureError::SignatureInputHeaderValueMissingComponents
            }
            "SignatureInputHeaderInvalidComponentType" => {
                SriMessageSignatureError::SignatureInputHeaderInvalidComponentType
            }
            "SignatureInputHeaderInvalidComponentName" => {
                SriMessageSignatureError::SignatureInputHeaderInvalidComponentName
            }
            "SignatureInputHeaderInvalidHeaderComponentParameter" => {
                SriMessageSignatureError::SignatureInputHeaderInvalidHeaderComponentParameter
            }
            "SignatureInputHeaderInvalidDerivedComponentParameter" => {
                SriMessageSignatureError::SignatureInputHeaderInvalidDerivedComponentParameter
            }
            "SignatureInputHeaderKeyIdLength" => {
                SriMessageSignatureError::SignatureInputHeaderKeyIdLength
            }
            "SignatureInputHeaderInvalidParameter" => {
                SriMessageSignatureError::SignatureInputHeaderInvalidParameter
            }
            "SignatureInputHeaderMissingRequiredParameters" => {
                SriMessageSignatureError::SignatureInputHeaderMissingRequiredParameters
            }
            "ValidationFailedSignatureExpired" => {
                SriMessageSignatureError::ValidationFailedSignatureExpired
            }
            "ValidationFailedInvalidLength" => {
                SriMessageSignatureError::ValidationFailedInvalidLength
            }
            "ValidationFailedSignatureMismatch" => {
                SriMessageSignatureError::ValidationFailedSignatureMismatch
            }
            "ValidationFailedIntegrityMismatch" => {
                SriMessageSignatureError::ValidationFailedIntegrityMismatch
            }
            "SignatureBaseUnknownDerivedComponent" => {
                SriMessageSignatureError::SignatureBaseUnknownDerivedComponent
            }
            "SignatureBaseMissingHeader" => SriMessageSignatureError::SignatureBaseMissingHeader,
            "SignatureBaseInvalidUnencodedDigest" => {
                SriMessageSignatureError::SignatureBaseInvalidUnencodedDigest
            }
            "SignatureBaseUnsupportedComponent" => {
                SriMessageSignatureError::SignatureBaseUnsupportedComponent
            }
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for SriMessageSignatureError {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SriMessageSignatureError {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SriMessageSignatureError;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(
                    match v {
                        "MissingSignatureHeader" => {
                            SriMessageSignatureError::MissingSignatureHeader
                        }
                        "MissingSignatureInputHeader" => {
                            SriMessageSignatureError::MissingSignatureInputHeader
                        }
                        "InvalidSignatureHeader" => {
                            SriMessageSignatureError::InvalidSignatureHeader
                        }
                        "InvalidSignatureInputHeader" => {
                            SriMessageSignatureError::InvalidSignatureInputHeader
                        }
                        "SignatureHeaderValueIsNotByteSequence" => {
                            SriMessageSignatureError::SignatureHeaderValueIsNotByteSequence
                        }
                        "SignatureHeaderValueIsParameterized" => {
                            SriMessageSignatureError::SignatureHeaderValueIsParameterized
                        }
                        "SignatureHeaderValueIsIncorrectLength" => {
                            SriMessageSignatureError::SignatureHeaderValueIsIncorrectLength
                        }
                        "SignatureInputHeaderMissingLabel" => {
                            SriMessageSignatureError::SignatureInputHeaderMissingLabel
                        }
                        "SignatureInputHeaderValueNotInnerList" => {
                            SriMessageSignatureError::SignatureInputHeaderValueNotInnerList
                        }
                        "SignatureInputHeaderValueMissingComponents" => {
                            SriMessageSignatureError::SignatureInputHeaderValueMissingComponents
                        }
                        "SignatureInputHeaderInvalidComponentType" => {
                            SriMessageSignatureError::SignatureInputHeaderInvalidComponentType
                        }
                        "SignatureInputHeaderInvalidComponentName" => {
                            SriMessageSignatureError::SignatureInputHeaderInvalidComponentName
                        }
                        "SignatureInputHeaderInvalidHeaderComponentParameter" => {
                            SriMessageSignatureError::SignatureInputHeaderInvalidHeaderComponentParameter
                        }
                        "SignatureInputHeaderInvalidDerivedComponentParameter" => {
                            SriMessageSignatureError::SignatureInputHeaderInvalidDerivedComponentParameter
                        }
                        "SignatureInputHeaderKeyIdLength" => {
                            SriMessageSignatureError::SignatureInputHeaderKeyIdLength
                        }
                        "SignatureInputHeaderInvalidParameter" => {
                            SriMessageSignatureError::SignatureInputHeaderInvalidParameter
                        }
                        "SignatureInputHeaderMissingRequiredParameters" => {
                            SriMessageSignatureError::SignatureInputHeaderMissingRequiredParameters
                        }
                        "ValidationFailedSignatureExpired" => {
                            SriMessageSignatureError::ValidationFailedSignatureExpired
                        }
                        "ValidationFailedInvalidLength" => {
                            SriMessageSignatureError::ValidationFailedInvalidLength
                        }
                        "ValidationFailedSignatureMismatch" => {
                            SriMessageSignatureError::ValidationFailedSignatureMismatch
                        }
                        "ValidationFailedIntegrityMismatch" => {
                            SriMessageSignatureError::ValidationFailedIntegrityMismatch
                        }
                        "SignatureBaseUnknownDerivedComponent" => {
                            SriMessageSignatureError::SignatureBaseUnknownDerivedComponent
                        }
                        "SignatureBaseMissingHeader" => {
                            SriMessageSignatureError::SignatureBaseMissingHeader
                        }
                        "SignatureBaseInvalidUnencodedDigest" => {
                            SriMessageSignatureError::SignatureBaseInvalidUnencodedDigest
                        }
                        "SignatureBaseUnsupportedComponent" => {
                            SriMessageSignatureError::SignatureBaseUnsupportedComponent
                        }
                        _ => SriMessageSignatureError::Unknown(v.to_owned()),
                    },
                )
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(SriMessageSignatureError::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.SRIMessageSignatureIssueDetails。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SriMessageSignatureIssueDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.SRIMessageSignatureIssueDetails/properties/error。
    #[serde(rename = "error")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub error: crate::audits::types::SriMessageSignatureError,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.SRIMessageSignatureIssueDetails/properties/signatureBase。
    #[serde(rename = "signatureBase")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub signature_base: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.SRIMessageSignatureIssueDetails/properties/integrityAssertions。
    #[serde(rename = "integrityAssertions")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub integrity_assertions: Vec<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.SRIMessageSignatureIssueDetails/properties/request。
    #[serde(rename = "request")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request: crate::audits::types::AffectedRequest,
}
impl<'de> serde::Deserialize<'de> for SriMessageSignatureIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SriMessageSignatureIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.SRIMessageSignatureIssueDetails/properties/error。
                    #[serde(rename = "error")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub error: crate::audits::types::SriMessageSignatureError,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.SRIMessageSignatureIssueDetails/properties/signatureBase。
                    #[serde(rename = "signatureBase")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub signature_base: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.SRIMessageSignatureIssueDetails/properties/integrityAssertions。
                    #[serde(rename = "integrityAssertions")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub integrity_assertions: Vec<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.SRIMessageSignatureIssueDetails/properties/request。
                    #[serde(rename = "request")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request: crate::audits::types::AffectedRequest,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SriMessageSignatureIssueDetails {
                    error: fields.error,
                    signature_base: fields.signature_base,
                    integrity_assertions: fields.integrity_assertions,
                    request: fields.request,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.SelectivePermissionsInterventionIssueDetails。
///
///协议原文：
///
///<pre>The issue warns about blocked calls to privacy sensitive APIs via the
///Selective Permissions Intervention.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct SelectivePermissionsInterventionIssueDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.SelectivePermissionsInterventionIssueDetails/properties/apiName。
    ///
    ///协议原文：
    ///
    ///<pre>Which API was intervened on.</pre>
    ///
    ///
    #[serde(rename = "apiName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub api_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.SelectivePermissionsInterventionIssueDetails/properties/adAncestry。
    ///
    ///协议原文：
    ///
    ///<pre>Why the ad script using the API is considered an ad.</pre>
    ///
    ///
    #[serde(rename = "adAncestry")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub ad_ancestry: crate::network::types::AdAncestry,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.SelectivePermissionsInterventionIssueDetails/properties/stackTrace。
    ///
    ///协议原文：
    ///
    ///<pre>The stack trace at the time of the intervention.</pre>
    ///
    ///
    #[serde(rename = "stackTrace")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub stack_trace: Option<crate::runtime::types::StackTrace>,
}
impl<'de> serde::Deserialize<'de> for SelectivePermissionsInterventionIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SelectivePermissionsInterventionIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.SelectivePermissionsInterventionIssueDetails/properties/apiName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Which API was intervened on.</pre>
                    ///
                    ///
                    #[serde(rename = "apiName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub api_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.SelectivePermissionsInterventionIssueDetails/properties/adAncestry。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Why the ad script using the API is considered an ad.</pre>
                    ///
                    ///
                    #[serde(rename = "adAncestry")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub ad_ancestry: crate::network::types::AdAncestry,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.SelectivePermissionsInterventionIssueDetails/properties/stackTrace。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The stack trace at the time of the intervention.</pre>
                    ///
                    ///
                    #[serde(rename = "stackTrace")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub stack_trace: Option<crate::runtime::types::StackTrace>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SelectivePermissionsInterventionIssueDetails {
                    api_name: fields.api_name,
                    ad_ancestry: fields.ad_ancestry,
                    stack_trace: fields.stack_trace,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.SharedArrayBufferIssueDetails。
///
///协议原文：
///
///<pre>Details for a issue arising from an SAB being instantiated in, or
///transferred to a context that is not cross-origin isolated.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct SharedArrayBufferIssueDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.SharedArrayBufferIssueDetails/properties/sourceCodeLocation。
    #[serde(rename = "sourceCodeLocation")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub source_code_location: crate::audits::types::SourceCodeLocation,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.SharedArrayBufferIssueDetails/properties/isWarning。
    #[serde(rename = "isWarning")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub is_warning: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.SharedArrayBufferIssueDetails/properties/type。
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: crate::audits::types::SharedArrayBufferIssueType,
}
impl<'de> serde::Deserialize<'de> for SharedArrayBufferIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SharedArrayBufferIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.SharedArrayBufferIssueDetails/properties/sourceCodeLocation。
                    #[serde(rename = "sourceCodeLocation")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub source_code_location: crate::audits::types::SourceCodeLocation,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.SharedArrayBufferIssueDetails/properties/isWarning。
                    #[serde(rename = "isWarning")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub is_warning: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.SharedArrayBufferIssueDetails/properties/type。
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: crate::audits::types::SharedArrayBufferIssueType,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SharedArrayBufferIssueDetails {
                    source_code_location: fields.source_code_location,
                    is_warning: fields.is_warning,
                    r#type: fields.r#type,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.type.SharedArrayBufferIssueType。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SharedArrayBufferIssueType {
    ///协议值 "TransferIssue"。
    TransferIssue,
    ///协议值 "CreationIssue"。
    CreationIssue,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl SharedArrayBufferIssueType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::types::SharedArrayBufferIssueType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::TransferIssue => "TransferIssue",
            Self::CreationIssue => "CreationIssue",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for SharedArrayBufferIssueType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for SharedArrayBufferIssueType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "TransferIssue" => SharedArrayBufferIssueType::TransferIssue,
            "CreationIssue" => SharedArrayBufferIssueType::CreationIssue,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for SharedArrayBufferIssueType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SharedArrayBufferIssueType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SharedArrayBufferIssueType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "TransferIssue" => SharedArrayBufferIssueType::TransferIssue,
                    "CreationIssue" => SharedArrayBufferIssueType::CreationIssue,
                    _ => SharedArrayBufferIssueType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(SharedArrayBufferIssueType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.type.SharedDictionaryError。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SharedDictionaryError {
    ///协议值 "UseErrorCrossOriginNoCorsRequest"。
    UseErrorCrossOriginNoCorsRequest,
    ///协议值 "UseErrorDictionaryLoadFailure"。
    UseErrorDictionaryLoadFailure,
    ///协议值 "UseErrorMatchingDictionaryNotUsed"。
    UseErrorMatchingDictionaryNotUsed,
    ///协议值 "UseErrorUnexpectedContentDictionaryHeader"。
    UseErrorUnexpectedContentDictionaryHeader,
    ///协议值 "WriteErrorCossOriginNoCorsRequest"。
    WriteErrorCossOriginNoCorsRequest,
    ///协议值 "WriteErrorDisallowedBySettings"。
    WriteErrorDisallowedBySettings,
    ///协议值 "WriteErrorExpiredResponse"。
    WriteErrorExpiredResponse,
    ///协议值 "WriteErrorFeatureDisabled"。
    WriteErrorFeatureDisabled,
    ///协议值 "WriteErrorInsufficientResources"。
    WriteErrorInsufficientResources,
    ///协议值 "WriteErrorInvalidMatchField"。
    WriteErrorInvalidMatchField,
    ///协议值 "WriteErrorInvalidStructuredHeader"。
    WriteErrorInvalidStructuredHeader,
    ///协议值 "WriteErrorInvalidTTLField"。
    WriteErrorInvalidTtlField,
    ///协议值 "WriteErrorNavigationRequest"。
    WriteErrorNavigationRequest,
    ///协议值 "WriteErrorNoMatchField"。
    WriteErrorNoMatchField,
    ///协议值 "WriteErrorNonIntegerTTLField"。
    WriteErrorNonIntegerTtlField,
    ///协议值 "WriteErrorNonListMatchDestField"。
    WriteErrorNonListMatchDestField,
    ///协议值 "WriteErrorNonSecureContext"。
    WriteErrorNonSecureContext,
    ///协议值 "WriteErrorNonStringIdField"。
    WriteErrorNonStringIdField,
    ///协议值 "WriteErrorNonStringInMatchDestList"。
    WriteErrorNonStringInMatchDestList,
    ///协议值 "WriteErrorInvalidMatchDestList"。
    WriteErrorInvalidMatchDestList,
    ///协议值 "WriteErrorNonStringMatchField"。
    WriteErrorNonStringMatchField,
    ///协议值 "WriteErrorNonTokenTypeField"。
    WriteErrorNonTokenTypeField,
    ///协议值 "WriteErrorRequestAborted"。
    WriteErrorRequestAborted,
    ///协议值 "WriteErrorShuttingDown"。
    WriteErrorShuttingDown,
    ///协议值 "WriteErrorTooLongIdField"。
    WriteErrorTooLongIdField,
    ///协议值 "WriteErrorUnsupportedType"。
    WriteErrorUnsupportedType,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl SharedDictionaryError {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::types::SharedDictionaryError::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::UseErrorCrossOriginNoCorsRequest => "UseErrorCrossOriginNoCorsRequest",
            Self::UseErrorDictionaryLoadFailure => "UseErrorDictionaryLoadFailure",
            Self::UseErrorMatchingDictionaryNotUsed => "UseErrorMatchingDictionaryNotUsed",
            Self::UseErrorUnexpectedContentDictionaryHeader => {
                "UseErrorUnexpectedContentDictionaryHeader"
            }
            Self::WriteErrorCossOriginNoCorsRequest => "WriteErrorCossOriginNoCorsRequest",
            Self::WriteErrorDisallowedBySettings => "WriteErrorDisallowedBySettings",
            Self::WriteErrorExpiredResponse => "WriteErrorExpiredResponse",
            Self::WriteErrorFeatureDisabled => "WriteErrorFeatureDisabled",
            Self::WriteErrorInsufficientResources => "WriteErrorInsufficientResources",
            Self::WriteErrorInvalidMatchField => "WriteErrorInvalidMatchField",
            Self::WriteErrorInvalidStructuredHeader => "WriteErrorInvalidStructuredHeader",
            Self::WriteErrorInvalidTtlField => "WriteErrorInvalidTTLField",
            Self::WriteErrorNavigationRequest => "WriteErrorNavigationRequest",
            Self::WriteErrorNoMatchField => "WriteErrorNoMatchField",
            Self::WriteErrorNonIntegerTtlField => "WriteErrorNonIntegerTTLField",
            Self::WriteErrorNonListMatchDestField => "WriteErrorNonListMatchDestField",
            Self::WriteErrorNonSecureContext => "WriteErrorNonSecureContext",
            Self::WriteErrorNonStringIdField => "WriteErrorNonStringIdField",
            Self::WriteErrorNonStringInMatchDestList => "WriteErrorNonStringInMatchDestList",
            Self::WriteErrorInvalidMatchDestList => "WriteErrorInvalidMatchDestList",
            Self::WriteErrorNonStringMatchField => "WriteErrorNonStringMatchField",
            Self::WriteErrorNonTokenTypeField => "WriteErrorNonTokenTypeField",
            Self::WriteErrorRequestAborted => "WriteErrorRequestAborted",
            Self::WriteErrorShuttingDown => "WriteErrorShuttingDown",
            Self::WriteErrorTooLongIdField => "WriteErrorTooLongIdField",
            Self::WriteErrorUnsupportedType => "WriteErrorUnsupportedType",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for SharedDictionaryError {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for SharedDictionaryError {
    fn from(value: String) -> Self {
        match value.as_str() {
            "UseErrorCrossOriginNoCorsRequest" => {
                SharedDictionaryError::UseErrorCrossOriginNoCorsRequest
            }
            "UseErrorDictionaryLoadFailure" => SharedDictionaryError::UseErrorDictionaryLoadFailure,
            "UseErrorMatchingDictionaryNotUsed" => {
                SharedDictionaryError::UseErrorMatchingDictionaryNotUsed
            }
            "UseErrorUnexpectedContentDictionaryHeader" => {
                SharedDictionaryError::UseErrorUnexpectedContentDictionaryHeader
            }
            "WriteErrorCossOriginNoCorsRequest" => {
                SharedDictionaryError::WriteErrorCossOriginNoCorsRequest
            }
            "WriteErrorDisallowedBySettings" => {
                SharedDictionaryError::WriteErrorDisallowedBySettings
            }
            "WriteErrorExpiredResponse" => SharedDictionaryError::WriteErrorExpiredResponse,
            "WriteErrorFeatureDisabled" => SharedDictionaryError::WriteErrorFeatureDisabled,
            "WriteErrorInsufficientResources" => {
                SharedDictionaryError::WriteErrorInsufficientResources
            }
            "WriteErrorInvalidMatchField" => SharedDictionaryError::WriteErrorInvalidMatchField,
            "WriteErrorInvalidStructuredHeader" => {
                SharedDictionaryError::WriteErrorInvalidStructuredHeader
            }
            "WriteErrorInvalidTTLField" => SharedDictionaryError::WriteErrorInvalidTtlField,
            "WriteErrorNavigationRequest" => SharedDictionaryError::WriteErrorNavigationRequest,
            "WriteErrorNoMatchField" => SharedDictionaryError::WriteErrorNoMatchField,
            "WriteErrorNonIntegerTTLField" => SharedDictionaryError::WriteErrorNonIntegerTtlField,
            "WriteErrorNonListMatchDestField" => {
                SharedDictionaryError::WriteErrorNonListMatchDestField
            }
            "WriteErrorNonSecureContext" => SharedDictionaryError::WriteErrorNonSecureContext,
            "WriteErrorNonStringIdField" => SharedDictionaryError::WriteErrorNonStringIdField,
            "WriteErrorNonStringInMatchDestList" => {
                SharedDictionaryError::WriteErrorNonStringInMatchDestList
            }
            "WriteErrorInvalidMatchDestList" => {
                SharedDictionaryError::WriteErrorInvalidMatchDestList
            }
            "WriteErrorNonStringMatchField" => SharedDictionaryError::WriteErrorNonStringMatchField,
            "WriteErrorNonTokenTypeField" => SharedDictionaryError::WriteErrorNonTokenTypeField,
            "WriteErrorRequestAborted" => SharedDictionaryError::WriteErrorRequestAborted,
            "WriteErrorShuttingDown" => SharedDictionaryError::WriteErrorShuttingDown,
            "WriteErrorTooLongIdField" => SharedDictionaryError::WriteErrorTooLongIdField,
            "WriteErrorUnsupportedType" => SharedDictionaryError::WriteErrorUnsupportedType,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for SharedDictionaryError {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SharedDictionaryError {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SharedDictionaryError;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "UseErrorCrossOriginNoCorsRequest" => {
                        SharedDictionaryError::UseErrorCrossOriginNoCorsRequest
                    }
                    "UseErrorDictionaryLoadFailure" => {
                        SharedDictionaryError::UseErrorDictionaryLoadFailure
                    }
                    "UseErrorMatchingDictionaryNotUsed" => {
                        SharedDictionaryError::UseErrorMatchingDictionaryNotUsed
                    }
                    "UseErrorUnexpectedContentDictionaryHeader" => {
                        SharedDictionaryError::UseErrorUnexpectedContentDictionaryHeader
                    }
                    "WriteErrorCossOriginNoCorsRequest" => {
                        SharedDictionaryError::WriteErrorCossOriginNoCorsRequest
                    }
                    "WriteErrorDisallowedBySettings" => {
                        SharedDictionaryError::WriteErrorDisallowedBySettings
                    }
                    "WriteErrorExpiredResponse" => SharedDictionaryError::WriteErrorExpiredResponse,
                    "WriteErrorFeatureDisabled" => SharedDictionaryError::WriteErrorFeatureDisabled,
                    "WriteErrorInsufficientResources" => {
                        SharedDictionaryError::WriteErrorInsufficientResources
                    }
                    "WriteErrorInvalidMatchField" => {
                        SharedDictionaryError::WriteErrorInvalidMatchField
                    }
                    "WriteErrorInvalidStructuredHeader" => {
                        SharedDictionaryError::WriteErrorInvalidStructuredHeader
                    }
                    "WriteErrorInvalidTTLField" => SharedDictionaryError::WriteErrorInvalidTtlField,
                    "WriteErrorNavigationRequest" => {
                        SharedDictionaryError::WriteErrorNavigationRequest
                    }
                    "WriteErrorNoMatchField" => SharedDictionaryError::WriteErrorNoMatchField,
                    "WriteErrorNonIntegerTTLField" => {
                        SharedDictionaryError::WriteErrorNonIntegerTtlField
                    }
                    "WriteErrorNonListMatchDestField" => {
                        SharedDictionaryError::WriteErrorNonListMatchDestField
                    }
                    "WriteErrorNonSecureContext" => {
                        SharedDictionaryError::WriteErrorNonSecureContext
                    }
                    "WriteErrorNonStringIdField" => {
                        SharedDictionaryError::WriteErrorNonStringIdField
                    }
                    "WriteErrorNonStringInMatchDestList" => {
                        SharedDictionaryError::WriteErrorNonStringInMatchDestList
                    }
                    "WriteErrorInvalidMatchDestList" => {
                        SharedDictionaryError::WriteErrorInvalidMatchDestList
                    }
                    "WriteErrorNonStringMatchField" => {
                        SharedDictionaryError::WriteErrorNonStringMatchField
                    }
                    "WriteErrorNonTokenTypeField" => {
                        SharedDictionaryError::WriteErrorNonTokenTypeField
                    }
                    "WriteErrorRequestAborted" => SharedDictionaryError::WriteErrorRequestAborted,
                    "WriteErrorShuttingDown" => SharedDictionaryError::WriteErrorShuttingDown,
                    "WriteErrorTooLongIdField" => SharedDictionaryError::WriteErrorTooLongIdField,
                    "WriteErrorUnsupportedType" => SharedDictionaryError::WriteErrorUnsupportedType,
                    _ => SharedDictionaryError::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(SharedDictionaryError::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.SharedDictionaryIssueDetails。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SharedDictionaryIssueDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.SharedDictionaryIssueDetails/properties/sharedDictionaryError。
    #[serde(rename = "sharedDictionaryError")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub shared_dictionary_error: crate::audits::types::SharedDictionaryError,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.SharedDictionaryIssueDetails/properties/request。
    #[serde(rename = "request")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request: crate::audits::types::AffectedRequest,
}
impl<'de> serde::Deserialize<'de> for SharedDictionaryIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SharedDictionaryIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.SharedDictionaryIssueDetails/properties/sharedDictionaryError。
                    #[serde(rename = "sharedDictionaryError")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub shared_dictionary_error: crate::audits::types::SharedDictionaryError,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.SharedDictionaryIssueDetails/properties/request。
                    #[serde(rename = "request")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request: crate::audits::types::AffectedRequest,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SharedDictionaryIssueDetails {
                    shared_dictionary_error: fields.shared_dictionary_error,
                    request: fields.request,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.SourceCodeLocation。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SourceCodeLocation {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.SourceCodeLocation/properties/scriptId。
    #[serde(rename = "scriptId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub script_id: Option<crate::runtime::types::ScriptId>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.SourceCodeLocation/properties/url。
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.SourceCodeLocation/properties/lineNumber。
    #[serde(rename = "lineNumber")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub line_number: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.SourceCodeLocation/properties/columnNumber。
    #[serde(rename = "columnNumber")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub column_number: i64,
}
impl<'de> serde::Deserialize<'de> for SourceCodeLocation {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SourceCodeLocation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.SourceCodeLocation/properties/scriptId。
                    #[serde(rename = "scriptId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub script_id: Option<crate::runtime::types::ScriptId>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.SourceCodeLocation/properties/url。
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.SourceCodeLocation/properties/lineNumber。
                    #[serde(rename = "lineNumber")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub line_number: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.SourceCodeLocation/properties/columnNumber。
                    #[serde(rename = "columnNumber")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub column_number: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SourceCodeLocation {
                    script_id: fields.script_id,
                    url: fields.url,
                    line_number: fields.line_number,
                    column_number: fields.column_number,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.type.StyleSheetLoadingIssueReason。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StyleSheetLoadingIssueReason {
    ///协议值 "LateImportRule"。
    LateImportRule,
    ///协议值 "RequestFailed"。
    RequestFailed,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl StyleSheetLoadingIssueReason {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::types::StyleSheetLoadingIssueReason::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::LateImportRule => "LateImportRule",
            Self::RequestFailed => "RequestFailed",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for StyleSheetLoadingIssueReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for StyleSheetLoadingIssueReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "LateImportRule" => StyleSheetLoadingIssueReason::LateImportRule,
            "RequestFailed" => StyleSheetLoadingIssueReason::RequestFailed,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for StyleSheetLoadingIssueReason {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StyleSheetLoadingIssueReason {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StyleSheetLoadingIssueReason;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "LateImportRule" => StyleSheetLoadingIssueReason::LateImportRule,
                    "RequestFailed" => StyleSheetLoadingIssueReason::RequestFailed,
                    _ => StyleSheetLoadingIssueReason::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(StyleSheetLoadingIssueReason::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.StylesheetLoadingIssueDetails。
///
///协议原文：
///
///<pre>This issue warns when a referenced stylesheet couldn't be loaded.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct StylesheetLoadingIssueDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.StylesheetLoadingIssueDetails/properties/sourceCodeLocation。
    ///
    ///协议原文：
    ///
    ///<pre>Source code position that referenced the failing stylesheet.</pre>
    ///
    ///
    #[serde(rename = "sourceCodeLocation")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub source_code_location: crate::audits::types::SourceCodeLocation,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.StylesheetLoadingIssueDetails/properties/styleSheetLoadingIssueReason。
    ///
    ///协议原文：
    ///
    ///<pre>Reason why the stylesheet couldn't be loaded.</pre>
    ///
    ///
    #[serde(rename = "styleSheetLoadingIssueReason")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub style_sheet_loading_issue_reason: crate::audits::types::StyleSheetLoadingIssueReason,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.StylesheetLoadingIssueDetails/properties/failedRequestInfo。
    ///
    ///协议原文：
    ///
    ///<pre>Contains additional info when the failure was due to a request.</pre>
    ///
    ///
    #[serde(rename = "failedRequestInfo")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub failed_request_info: Option<crate::audits::types::FailedRequestInfo>,
}
impl<'de> serde::Deserialize<'de> for StylesheetLoadingIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StylesheetLoadingIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.StylesheetLoadingIssueDetails/properties/sourceCodeLocation。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Source code position that referenced the failing stylesheet.</pre>
                    ///
                    ///
                    #[serde(rename = "sourceCodeLocation")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub source_code_location: crate::audits::types::SourceCodeLocation,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.StylesheetLoadingIssueDetails/properties/styleSheetLoadingIssueReason。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Reason why the stylesheet couldn't be loaded.</pre>
                    ///
                    ///
                    #[serde(rename = "styleSheetLoadingIssueReason")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub style_sheet_loading_issue_reason:
                        crate::audits::types::StyleSheetLoadingIssueReason,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.StylesheetLoadingIssueDetails/properties/failedRequestInfo。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Contains additional info when the failure was due to a request.</pre>
                    ///
                    ///
                    #[serde(rename = "failedRequestInfo")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub failed_request_info: Option<crate::audits::types::FailedRequestInfo>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StylesheetLoadingIssueDetails {
                    source_code_location: fields.source_code_location,
                    style_sheet_loading_issue_reason: fields.style_sheet_loading_issue_reason,
                    failed_request_info: fields.failed_request_info,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.type.UnencodedDigestError。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum UnencodedDigestError {
    ///协议值 "MalformedDictionary"。
    MalformedDictionary,
    ///协议值 "UnknownAlgorithm"。
    UnknownAlgorithm,
    ///协议值 "IncorrectDigestType"。
    IncorrectDigestType,
    ///协议值 "IncorrectDigestLength"。
    IncorrectDigestLength,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl UnencodedDigestError {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::types::UnencodedDigestError::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::MalformedDictionary => "MalformedDictionary",
            Self::UnknownAlgorithm => "UnknownAlgorithm",
            Self::IncorrectDigestType => "IncorrectDigestType",
            Self::IncorrectDigestLength => "IncorrectDigestLength",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for UnencodedDigestError {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for UnencodedDigestError {
    fn from(value: String) -> Self {
        match value.as_str() {
            "MalformedDictionary" => UnencodedDigestError::MalformedDictionary,
            "UnknownAlgorithm" => UnencodedDigestError::UnknownAlgorithm,
            "IncorrectDigestType" => UnencodedDigestError::IncorrectDigestType,
            "IncorrectDigestLength" => UnencodedDigestError::IncorrectDigestLength,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for UnencodedDigestError {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UnencodedDigestError {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UnencodedDigestError;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "MalformedDictionary" => UnencodedDigestError::MalformedDictionary,
                    "UnknownAlgorithm" => UnencodedDigestError::UnknownAlgorithm,
                    "IncorrectDigestType" => UnencodedDigestError::IncorrectDigestType,
                    "IncorrectDigestLength" => UnencodedDigestError::IncorrectDigestLength,
                    _ => UnencodedDigestError::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(UnencodedDigestError::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.UnencodedDigestIssueDetails。
#[derive(Debug, Clone, serde::Serialize)]
pub struct UnencodedDigestIssueDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.UnencodedDigestIssueDetails/properties/error。
    #[serde(rename = "error")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub error: crate::audits::types::UnencodedDigestError,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.UnencodedDigestIssueDetails/properties/request。
    #[serde(rename = "request")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request: crate::audits::types::AffectedRequest,
}
impl<'de> serde::Deserialize<'de> for UnencodedDigestIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UnencodedDigestIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.UnencodedDigestIssueDetails/properties/error。
                    #[serde(rename = "error")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub error: crate::audits::types::UnencodedDigestError,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.UnencodedDigestIssueDetails/properties/request。
                    #[serde(rename = "request")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request: crate::audits::types::AffectedRequest,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(UnencodedDigestIssueDetails {
                    error: fields.error,
                    request: fields.request,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.type.UserReidentificationIssueDetails。
///
///协议原文：
///
///<pre>This issue warns about uses of APIs that may be considered misuse to
///re-identify users.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct UserReidentificationIssueDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.UserReidentificationIssueDetails/properties/type。
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: crate::audits::types::UserReidentificationIssueType,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.UserReidentificationIssueDetails/properties/request。
    ///
    ///协议原文：
    ///
    ///<pre>Applies to BlockedFrameNavigation and BlockedSubresource issue types.</pre>
    ///
    ///
    #[serde(rename = "request")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub request: Option<crate::audits::types::AffectedRequest>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.UserReidentificationIssueDetails/properties/sourceCodeLocation。
    ///
    ///协议原文：
    ///
    ///<pre>Applies to NoisedCanvasReadback issue type.</pre>
    ///
    ///
    #[serde(rename = "sourceCodeLocation")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_code_location: Option<crate::audits::types::SourceCodeLocation>,
}
impl<'de> serde::Deserialize<'de> for UserReidentificationIssueDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UserReidentificationIssueDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.type.UserReidentificationIssueDetails/properties/type。
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: crate::audits::types::UserReidentificationIssueType,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.UserReidentificationIssueDetails/properties/request。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Applies to BlockedFrameNavigation and BlockedSubresource issue types.</pre>
                    ///
                    ///
                    #[serde(rename = "request")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub request: Option<crate::audits::types::AffectedRequest>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.type.UserReidentificationIssueDetails/properties/sourceCodeLocation。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Applies to NoisedCanvasReadback issue type.</pre>
                    ///
                    ///
                    #[serde(rename = "sourceCodeLocation")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub source_code_location: Option<crate::audits::types::SourceCodeLocation>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(UserReidentificationIssueDetails {
                    r#type: fields.r#type,
                    request: fields.request,
                    source_code_location: fields.source_code_location,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.type.UserReidentificationIssueType。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum UserReidentificationIssueType {
    ///协议值 "BlockedFrameNavigation"。
    BlockedFrameNavigation,
    ///协议值 "BlockedSubresource"。
    BlockedSubresource,
    ///协议值 "NoisedCanvasReadback"。
    NoisedCanvasReadback,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl UserReidentificationIssueType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::types::UserReidentificationIssueType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::BlockedFrameNavigation => "BlockedFrameNavigation",
            Self::BlockedSubresource => "BlockedSubresource",
            Self::NoisedCanvasReadback => "NoisedCanvasReadback",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for UserReidentificationIssueType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for UserReidentificationIssueType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "BlockedFrameNavigation" => UserReidentificationIssueType::BlockedFrameNavigation,
            "BlockedSubresource" => UserReidentificationIssueType::BlockedSubresource,
            "NoisedCanvasReadback" => UserReidentificationIssueType::NoisedCanvasReadback,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for UserReidentificationIssueType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UserReidentificationIssueType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UserReidentificationIssueType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "BlockedFrameNavigation" => {
                        UserReidentificationIssueType::BlockedFrameNavigation
                    }
                    "BlockedSubresource" => UserReidentificationIssueType::BlockedSubresource,
                    "NoisedCanvasReadback" => UserReidentificationIssueType::NoisedCanvasReadback,
                    _ => UserReidentificationIssueType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(UserReidentificationIssueType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
