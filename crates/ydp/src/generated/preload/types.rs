///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Preload.type.PrefetchStatus。
///
///协议原文：
///
///<pre>TODO(https://crbug.com/1384419): revisit the list of PrefetchStatus and
///filter out the ones that aren't necessary to the developers.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PrefetchStatus {
    ///协议值 "PrefetchAllowed"。
    PrefetchAllowed,
    ///协议值 "PrefetchFailedIneligibleRedirect"。
    PrefetchFailedIneligibleRedirect,
    ///协议值 "PrefetchFailedInvalidRedirect"。
    PrefetchFailedInvalidRedirect,
    ///协议值 "PrefetchFailedMIMENotSupported"。
    PrefetchFailedMimeNotSupported,
    ///协议值 "PrefetchFailedNetError"。
    PrefetchFailedNetError,
    ///协议值 "PrefetchFailedNon2XX"。
    PrefetchFailedNon2Xx,
    ///协议值 "PrefetchEvictedAfterBrowsingDataRemoved"。
    PrefetchEvictedAfterBrowsingDataRemoved,
    ///协议值 "PrefetchEvictedAfterCandidateRemoved"。
    PrefetchEvictedAfterCandidateRemoved,
    ///协议值 "PrefetchEvictedForNewerPrefetch"。
    PrefetchEvictedForNewerPrefetch,
    ///协议值 "PrefetchHeldback"。
    PrefetchHeldback,
    ///协议值 "PrefetchIneligibleRetryAfter"。
    PrefetchIneligibleRetryAfter,
    ///协议值 "PrefetchIsPrivacyDecoy"。
    PrefetchIsPrivacyDecoy,
    ///协议值 "PrefetchIsStale"。
    PrefetchIsStale,
    ///协议值 "PrefetchNotEligibleBlockedByConnectionAllowlist"。
    PrefetchNotEligibleBlockedByConnectionAllowlist,
    ///协议值 "PrefetchNotEligibleBrowserContextOffTheRecord"。
    PrefetchNotEligibleBrowserContextOffTheRecord,
    ///协议值 "PrefetchNotEligibleCrossOrigin"。
    PrefetchNotEligibleCrossOrigin,
    ///协议值 "PrefetchNotEligibleDataSaverEnabled"。
    PrefetchNotEligibleDataSaverEnabled,
    ///协议值 "PrefetchNotEligibleExistingProxy"。
    PrefetchNotEligibleExistingProxy,
    ///协议值 "PrefetchNotEligibleHostIsNonUnique"。
    PrefetchNotEligibleHostIsNonUnique,
    ///协议值 "PrefetchNotEligibleNonDefaultStoragePartition"。
    PrefetchNotEligibleNonDefaultStoragePartition,
    ///协议值 "PrefetchNotEligibleSameSiteCrossOriginPrefetchRequiredProxy"。
    PrefetchNotEligibleSameSiteCrossOriginPrefetchRequiredProxy,
    ///协议值 "PrefetchNotEligibleSchemeIsNotHttps"。
    PrefetchNotEligibleSchemeIsNotHttps,
    ///协议值 "PrefetchNotEligibleUserHasCookies"。
    PrefetchNotEligibleUserHasCookies,
    ///协议值 "PrefetchNotEligibleUserHasServiceWorker"。
    PrefetchNotEligibleUserHasServiceWorker,
    ///协议值 "PrefetchNotEligibleUserHasServiceWorkerNoFetchHandler"。
    PrefetchNotEligibleUserHasServiceWorkerNoFetchHandler,
    ///协议值 "PrefetchNotEligibleRedirectFromServiceWorker"。
    PrefetchNotEligibleRedirectFromServiceWorker,
    ///协议值 "PrefetchNotEligibleRedirectToServiceWorker"。
    PrefetchNotEligibleRedirectToServiceWorker,
    ///协议值 "PrefetchNotEligibleBatterySaverEnabled"。
    PrefetchNotEligibleBatterySaverEnabled,
    ///协议值 "PrefetchNotEligiblePreloadingDisabled"。
    PrefetchNotEligiblePreloadingDisabled,
    ///协议值 "PrefetchNotFinishedInTime"。
    PrefetchNotFinishedInTime,
    ///协议值 "PrefetchNotStarted"。
    PrefetchNotStarted,
    ///协议值 "PrefetchNotUsedCookiesChanged"。
    PrefetchNotUsedCookiesChanged,
    ///协议值 "PrefetchProxyNotAvailable"。
    PrefetchProxyNotAvailable,
    ///协议值 "PrefetchResponseUsed"。
    PrefetchResponseUsed,
    ///协议值 "PrefetchSuccessfulButNotUsed"。
    PrefetchSuccessfulButNotUsed,
    ///协议值 "PrefetchNotUsedProbeFailed"。
    PrefetchNotUsedProbeFailed,
    ///协议值 "PrefetchCancelledOnUserNavigation"。
    PrefetchCancelledOnUserNavigation,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl PrefetchStatus {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::preload::types::PrefetchStatus::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::PrefetchAllowed => "PrefetchAllowed",
            Self::PrefetchFailedIneligibleRedirect => "PrefetchFailedIneligibleRedirect",
            Self::PrefetchFailedInvalidRedirect => "PrefetchFailedInvalidRedirect",
            Self::PrefetchFailedMimeNotSupported => "PrefetchFailedMIMENotSupported",
            Self::PrefetchFailedNetError => "PrefetchFailedNetError",
            Self::PrefetchFailedNon2Xx => "PrefetchFailedNon2XX",
            Self::PrefetchEvictedAfterBrowsingDataRemoved => {
                "PrefetchEvictedAfterBrowsingDataRemoved"
            }
            Self::PrefetchEvictedAfterCandidateRemoved => "PrefetchEvictedAfterCandidateRemoved",
            Self::PrefetchEvictedForNewerPrefetch => "PrefetchEvictedForNewerPrefetch",
            Self::PrefetchHeldback => "PrefetchHeldback",
            Self::PrefetchIneligibleRetryAfter => "PrefetchIneligibleRetryAfter",
            Self::PrefetchIsPrivacyDecoy => "PrefetchIsPrivacyDecoy",
            Self::PrefetchIsStale => "PrefetchIsStale",
            Self::PrefetchNotEligibleBlockedByConnectionAllowlist => {
                "PrefetchNotEligibleBlockedByConnectionAllowlist"
            }
            Self::PrefetchNotEligibleBrowserContextOffTheRecord => {
                "PrefetchNotEligibleBrowserContextOffTheRecord"
            }
            Self::PrefetchNotEligibleCrossOrigin => "PrefetchNotEligibleCrossOrigin",
            Self::PrefetchNotEligibleDataSaverEnabled => "PrefetchNotEligibleDataSaverEnabled",
            Self::PrefetchNotEligibleExistingProxy => "PrefetchNotEligibleExistingProxy",
            Self::PrefetchNotEligibleHostIsNonUnique => "PrefetchNotEligibleHostIsNonUnique",
            Self::PrefetchNotEligibleNonDefaultStoragePartition => {
                "PrefetchNotEligibleNonDefaultStoragePartition"
            }
            Self::PrefetchNotEligibleSameSiteCrossOriginPrefetchRequiredProxy => {
                "PrefetchNotEligibleSameSiteCrossOriginPrefetchRequiredProxy"
            }
            Self::PrefetchNotEligibleSchemeIsNotHttps => "PrefetchNotEligibleSchemeIsNotHttps",
            Self::PrefetchNotEligibleUserHasCookies => "PrefetchNotEligibleUserHasCookies",
            Self::PrefetchNotEligibleUserHasServiceWorker => {
                "PrefetchNotEligibleUserHasServiceWorker"
            }
            Self::PrefetchNotEligibleUserHasServiceWorkerNoFetchHandler => {
                "PrefetchNotEligibleUserHasServiceWorkerNoFetchHandler"
            }
            Self::PrefetchNotEligibleRedirectFromServiceWorker => {
                "PrefetchNotEligibleRedirectFromServiceWorker"
            }
            Self::PrefetchNotEligibleRedirectToServiceWorker => {
                "PrefetchNotEligibleRedirectToServiceWorker"
            }
            Self::PrefetchNotEligibleBatterySaverEnabled => {
                "PrefetchNotEligibleBatterySaverEnabled"
            }
            Self::PrefetchNotEligiblePreloadingDisabled => "PrefetchNotEligiblePreloadingDisabled",
            Self::PrefetchNotFinishedInTime => "PrefetchNotFinishedInTime",
            Self::PrefetchNotStarted => "PrefetchNotStarted",
            Self::PrefetchNotUsedCookiesChanged => "PrefetchNotUsedCookiesChanged",
            Self::PrefetchProxyNotAvailable => "PrefetchProxyNotAvailable",
            Self::PrefetchResponseUsed => "PrefetchResponseUsed",
            Self::PrefetchSuccessfulButNotUsed => "PrefetchSuccessfulButNotUsed",
            Self::PrefetchNotUsedProbeFailed => "PrefetchNotUsedProbeFailed",
            Self::PrefetchCancelledOnUserNavigation => "PrefetchCancelledOnUserNavigation",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for PrefetchStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for PrefetchStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "PrefetchAllowed" => PrefetchStatus::PrefetchAllowed,
            "PrefetchFailedIneligibleRedirect" => PrefetchStatus::PrefetchFailedIneligibleRedirect,
            "PrefetchFailedInvalidRedirect" => PrefetchStatus::PrefetchFailedInvalidRedirect,
            "PrefetchFailedMIMENotSupported" => PrefetchStatus::PrefetchFailedMimeNotSupported,
            "PrefetchFailedNetError" => PrefetchStatus::PrefetchFailedNetError,
            "PrefetchFailedNon2XX" => PrefetchStatus::PrefetchFailedNon2Xx,
            "PrefetchEvictedAfterBrowsingDataRemoved" => {
                PrefetchStatus::PrefetchEvictedAfterBrowsingDataRemoved
            }
            "PrefetchEvictedAfterCandidateRemoved" => {
                PrefetchStatus::PrefetchEvictedAfterCandidateRemoved
            }
            "PrefetchEvictedForNewerPrefetch" => PrefetchStatus::PrefetchEvictedForNewerPrefetch,
            "PrefetchHeldback" => PrefetchStatus::PrefetchHeldback,
            "PrefetchIneligibleRetryAfter" => PrefetchStatus::PrefetchIneligibleRetryAfter,
            "PrefetchIsPrivacyDecoy" => PrefetchStatus::PrefetchIsPrivacyDecoy,
            "PrefetchIsStale" => PrefetchStatus::PrefetchIsStale,
            "PrefetchNotEligibleBlockedByConnectionAllowlist" => {
                PrefetchStatus::PrefetchNotEligibleBlockedByConnectionAllowlist
            }
            "PrefetchNotEligibleBrowserContextOffTheRecord" => {
                PrefetchStatus::PrefetchNotEligibleBrowserContextOffTheRecord
            }
            "PrefetchNotEligibleCrossOrigin" => PrefetchStatus::PrefetchNotEligibleCrossOrigin,
            "PrefetchNotEligibleDataSaverEnabled" => {
                PrefetchStatus::PrefetchNotEligibleDataSaverEnabled
            }
            "PrefetchNotEligibleExistingProxy" => PrefetchStatus::PrefetchNotEligibleExistingProxy,
            "PrefetchNotEligibleHostIsNonUnique" => {
                PrefetchStatus::PrefetchNotEligibleHostIsNonUnique
            }
            "PrefetchNotEligibleNonDefaultStoragePartition" => {
                PrefetchStatus::PrefetchNotEligibleNonDefaultStoragePartition
            }
            "PrefetchNotEligibleSameSiteCrossOriginPrefetchRequiredProxy" => {
                PrefetchStatus::PrefetchNotEligibleSameSiteCrossOriginPrefetchRequiredProxy
            }
            "PrefetchNotEligibleSchemeIsNotHttps" => {
                PrefetchStatus::PrefetchNotEligibleSchemeIsNotHttps
            }
            "PrefetchNotEligibleUserHasCookies" => {
                PrefetchStatus::PrefetchNotEligibleUserHasCookies
            }
            "PrefetchNotEligibleUserHasServiceWorker" => {
                PrefetchStatus::PrefetchNotEligibleUserHasServiceWorker
            }
            "PrefetchNotEligibleUserHasServiceWorkerNoFetchHandler" => {
                PrefetchStatus::PrefetchNotEligibleUserHasServiceWorkerNoFetchHandler
            }
            "PrefetchNotEligibleRedirectFromServiceWorker" => {
                PrefetchStatus::PrefetchNotEligibleRedirectFromServiceWorker
            }
            "PrefetchNotEligibleRedirectToServiceWorker" => {
                PrefetchStatus::PrefetchNotEligibleRedirectToServiceWorker
            }
            "PrefetchNotEligibleBatterySaverEnabled" => {
                PrefetchStatus::PrefetchNotEligibleBatterySaverEnabled
            }
            "PrefetchNotEligiblePreloadingDisabled" => {
                PrefetchStatus::PrefetchNotEligiblePreloadingDisabled
            }
            "PrefetchNotFinishedInTime" => PrefetchStatus::PrefetchNotFinishedInTime,
            "PrefetchNotStarted" => PrefetchStatus::PrefetchNotStarted,
            "PrefetchNotUsedCookiesChanged" => PrefetchStatus::PrefetchNotUsedCookiesChanged,
            "PrefetchProxyNotAvailable" => PrefetchStatus::PrefetchProxyNotAvailable,
            "PrefetchResponseUsed" => PrefetchStatus::PrefetchResponseUsed,
            "PrefetchSuccessfulButNotUsed" => PrefetchStatus::PrefetchSuccessfulButNotUsed,
            "PrefetchNotUsedProbeFailed" => PrefetchStatus::PrefetchNotUsedProbeFailed,
            "PrefetchCancelledOnUserNavigation" => {
                PrefetchStatus::PrefetchCancelledOnUserNavigation
            }
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for PrefetchStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PrefetchStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PrefetchStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "PrefetchAllowed" => PrefetchStatus::PrefetchAllowed,
                    "PrefetchFailedIneligibleRedirect" => {
                        PrefetchStatus::PrefetchFailedIneligibleRedirect
                    }
                    "PrefetchFailedInvalidRedirect" => {
                        PrefetchStatus::PrefetchFailedInvalidRedirect
                    }
                    "PrefetchFailedMIMENotSupported" => {
                        PrefetchStatus::PrefetchFailedMimeNotSupported
                    }
                    "PrefetchFailedNetError" => PrefetchStatus::PrefetchFailedNetError,
                    "PrefetchFailedNon2XX" => PrefetchStatus::PrefetchFailedNon2Xx,
                    "PrefetchEvictedAfterBrowsingDataRemoved" => {
                        PrefetchStatus::PrefetchEvictedAfterBrowsingDataRemoved
                    }
                    "PrefetchEvictedAfterCandidateRemoved" => {
                        PrefetchStatus::PrefetchEvictedAfterCandidateRemoved
                    }
                    "PrefetchEvictedForNewerPrefetch" => {
                        PrefetchStatus::PrefetchEvictedForNewerPrefetch
                    }
                    "PrefetchHeldback" => PrefetchStatus::PrefetchHeldback,
                    "PrefetchIneligibleRetryAfter" => PrefetchStatus::PrefetchIneligibleRetryAfter,
                    "PrefetchIsPrivacyDecoy" => PrefetchStatus::PrefetchIsPrivacyDecoy,
                    "PrefetchIsStale" => PrefetchStatus::PrefetchIsStale,
                    "PrefetchNotEligibleBlockedByConnectionAllowlist" => {
                        PrefetchStatus::PrefetchNotEligibleBlockedByConnectionAllowlist
                    }
                    "PrefetchNotEligibleBrowserContextOffTheRecord" => {
                        PrefetchStatus::PrefetchNotEligibleBrowserContextOffTheRecord
                    }
                    "PrefetchNotEligibleCrossOrigin" => {
                        PrefetchStatus::PrefetchNotEligibleCrossOrigin
                    }
                    "PrefetchNotEligibleDataSaverEnabled" => {
                        PrefetchStatus::PrefetchNotEligibleDataSaverEnabled
                    }
                    "PrefetchNotEligibleExistingProxy" => {
                        PrefetchStatus::PrefetchNotEligibleExistingProxy
                    }
                    "PrefetchNotEligibleHostIsNonUnique" => {
                        PrefetchStatus::PrefetchNotEligibleHostIsNonUnique
                    }
                    "PrefetchNotEligibleNonDefaultStoragePartition" => {
                        PrefetchStatus::PrefetchNotEligibleNonDefaultStoragePartition
                    }
                    "PrefetchNotEligibleSameSiteCrossOriginPrefetchRequiredProxy" => {
                        PrefetchStatus::PrefetchNotEligibleSameSiteCrossOriginPrefetchRequiredProxy
                    }
                    "PrefetchNotEligibleSchemeIsNotHttps" => {
                        PrefetchStatus::PrefetchNotEligibleSchemeIsNotHttps
                    }
                    "PrefetchNotEligibleUserHasCookies" => {
                        PrefetchStatus::PrefetchNotEligibleUserHasCookies
                    }
                    "PrefetchNotEligibleUserHasServiceWorker" => {
                        PrefetchStatus::PrefetchNotEligibleUserHasServiceWorker
                    }
                    "PrefetchNotEligibleUserHasServiceWorkerNoFetchHandler" => {
                        PrefetchStatus::PrefetchNotEligibleUserHasServiceWorkerNoFetchHandler
                    }
                    "PrefetchNotEligibleRedirectFromServiceWorker" => {
                        PrefetchStatus::PrefetchNotEligibleRedirectFromServiceWorker
                    }
                    "PrefetchNotEligibleRedirectToServiceWorker" => {
                        PrefetchStatus::PrefetchNotEligibleRedirectToServiceWorker
                    }
                    "PrefetchNotEligibleBatterySaverEnabled" => {
                        PrefetchStatus::PrefetchNotEligibleBatterySaverEnabled
                    }
                    "PrefetchNotEligiblePreloadingDisabled" => {
                        PrefetchStatus::PrefetchNotEligiblePreloadingDisabled
                    }
                    "PrefetchNotFinishedInTime" => PrefetchStatus::PrefetchNotFinishedInTime,
                    "PrefetchNotStarted" => PrefetchStatus::PrefetchNotStarted,
                    "PrefetchNotUsedCookiesChanged" => {
                        PrefetchStatus::PrefetchNotUsedCookiesChanged
                    }
                    "PrefetchProxyNotAvailable" => PrefetchStatus::PrefetchProxyNotAvailable,
                    "PrefetchResponseUsed" => PrefetchStatus::PrefetchResponseUsed,
                    "PrefetchSuccessfulButNotUsed" => PrefetchStatus::PrefetchSuccessfulButNotUsed,
                    "PrefetchNotUsedProbeFailed" => PrefetchStatus::PrefetchNotUsedProbeFailed,
                    "PrefetchCancelledOnUserNavigation" => {
                        PrefetchStatus::PrefetchCancelledOnUserNavigation
                    }
                    _ => PrefetchStatus::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(PrefetchStatus::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Preload.type.PreloadPipelineId。
///
///协议原文：
///
///<pre>Chrome manages different types of preloads together using a
///concept of preloading pipeline. For example, if a site uses a
///SpeculationRules for prerender, Chrome first starts a prefetch and
///then upgrades it to prerender.
///
///CDP events for them are emitted separately but they share
///`PreloadPipelineId`.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct PreloadPipelineId(String);
impl PreloadPipelineId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::preload::types::PreloadPipelineId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::preload::types::PreloadPipelineId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::preload::types::PreloadPipelineId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::preload::types::PreloadPipelineId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::preload::types::PreloadPipelineId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Preload.type.PreloadingAttemptKey。
///
///协议原文：
///
///<pre>A key that identifies a preloading attempt.
///
///The url used is the url specified by the trigger (i.e. the initial URL), and
///not the final url that is navigated to. For example, prerendering allows
///same-origin main frame navigations during the attempt, but the attempt is
///still keyed with the initial URL.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct PreloadingAttemptKey {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.type.PreloadingAttemptKey/properties/loaderId。
    #[serde(rename = "loaderId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub loader_id: crate::network::types::LoaderId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.type.PreloadingAttemptKey/properties/action。
    #[serde(rename = "action")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub action: crate::preload::types::SpeculationAction,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.type.PreloadingAttemptKey/properties/url。
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Preload.type.PreloadingAttemptKey/properties/formSubmission。
    #[serde(rename = "formSubmission")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub form_submission: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Preload.type.PreloadingAttemptKey/properties/targetHint。
    #[serde(rename = "targetHint")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_hint: Option<crate::preload::types::SpeculationTargetHint>,
}
impl<'de> serde::Deserialize<'de> for PreloadingAttemptKey {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PreloadingAttemptKey;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.type.PreloadingAttemptKey/properties/loaderId。
                    #[serde(rename = "loaderId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub loader_id: crate::network::types::LoaderId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.type.PreloadingAttemptKey/properties/action。
                    #[serde(rename = "action")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub action: crate::preload::types::SpeculationAction,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.type.PreloadingAttemptKey/properties/url。
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Preload.type.PreloadingAttemptKey/properties/formSubmission。
                    #[serde(rename = "formSubmission")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub form_submission: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Preload.type.PreloadingAttemptKey/properties/targetHint。
                    #[serde(rename = "targetHint")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub target_hint: Option<crate::preload::types::SpeculationTargetHint>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PreloadingAttemptKey {
                    loader_id: fields.loader_id,
                    action: fields.action,
                    url: fields.url,
                    form_submission: fields.form_submission,
                    target_hint: fields.target_hint,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Preload.type.PreloadingAttemptSource。
///
///协议原文：
///
///<pre>Lists sources for a preloading attempt, specifically the ids of rule sets
///that had a speculation rule that triggered the attempt, and the
///BackendNodeIds of &lt;a href&gt; or &lt;area href&gt; elements that triggered the
///attempt (in the case of attempts triggered by a document rule). It is
///possible for multiple rule sets and links to trigger a single attempt.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct PreloadingAttemptSource {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.type.PreloadingAttemptSource/properties/key。
    #[serde(rename = "key")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub key: crate::preload::types::PreloadingAttemptKey,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.type.PreloadingAttemptSource/properties/ruleSetIds。
    #[serde(rename = "ruleSetIds")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub rule_set_ids: Vec<crate::preload::types::RuleSetId>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.type.PreloadingAttemptSource/properties/nodeIds。
    #[serde(rename = "nodeIds")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_ids: Vec<crate::dom::types::BackendNodeId>,
}
impl<'de> serde::Deserialize<'de> for PreloadingAttemptSource {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PreloadingAttemptSource;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.type.PreloadingAttemptSource/properties/key。
                    #[serde(rename = "key")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub key: crate::preload::types::PreloadingAttemptKey,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.type.PreloadingAttemptSource/properties/ruleSetIds。
                    #[serde(rename = "ruleSetIds")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub rule_set_ids: Vec<crate::preload::types::RuleSetId>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.type.PreloadingAttemptSource/properties/nodeIds。
                    #[serde(rename = "nodeIds")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_ids: Vec<crate::dom::types::BackendNodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PreloadingAttemptSource {
                    key: fields.key,
                    rule_set_ids: fields.rule_set_ids,
                    node_ids: fields.node_ids,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Preload.type.PreloadingStatus。
///
///协议原文：
///
///<pre>Preloading status values, see also PreloadingTriggeringOutcome. This
///status is shared by prefetchStatusUpdated and prerenderStatusUpdated.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PreloadingStatus {
    ///协议值 "Pending"。
    Pending,
    ///协议值 "Running"。
    Running,
    ///协议值 "Ready"。
    Ready,
    ///协议值 "Success"。
    Success,
    ///协议值 "Failure"。
    Failure,
    ///协议值 "NotSupported"。
    NotSupported,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl PreloadingStatus {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::preload::types::PreloadingStatus::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "Pending",
            Self::Running => "Running",
            Self::Ready => "Ready",
            Self::Success => "Success",
            Self::Failure => "Failure",
            Self::NotSupported => "NotSupported",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for PreloadingStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for PreloadingStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Pending" => PreloadingStatus::Pending,
            "Running" => PreloadingStatus::Running,
            "Ready" => PreloadingStatus::Ready,
            "Success" => PreloadingStatus::Success,
            "Failure" => PreloadingStatus::Failure,
            "NotSupported" => PreloadingStatus::NotSupported,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for PreloadingStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PreloadingStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PreloadingStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Pending" => PreloadingStatus::Pending,
                    "Running" => PreloadingStatus::Running,
                    "Ready" => PreloadingStatus::Ready,
                    "Success" => PreloadingStatus::Success,
                    "Failure" => PreloadingStatus::Failure,
                    "NotSupported" => PreloadingStatus::NotSupported,
                    _ => PreloadingStatus::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(PreloadingStatus::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Preload.type.PrerenderFinalStatus。
///
///协议原文：
///
///<pre>List of FinalStatus reasons for Prerender2.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PrerenderFinalStatus {
    ///协议值 "Activated"。
    Activated,
    ///协议值 "Destroyed"。
    Destroyed,
    ///协议值 "LowEndDevice"。
    LowEndDevice,
    ///协议值 "InvalidSchemeRedirect"。
    InvalidSchemeRedirect,
    ///协议值 "InvalidSchemeNavigation"。
    InvalidSchemeNavigation,
    ///协议值 "NavigationRequestBlockedByCsp"。
    NavigationRequestBlockedByCsp,
    ///协议值 "MojoBinderPolicy"。
    MojoBinderPolicy,
    ///协议值 "RendererProcessCrashed"。
    RendererProcessCrashed,
    ///协议值 "RendererProcessKilled"。
    RendererProcessKilled,
    ///协议值 "Download"。
    Download,
    ///协议值 "TriggerDestroyed"。
    TriggerDestroyed,
    ///协议值 "NavigationNotCommitted"。
    NavigationNotCommitted,
    ///协议值 "NavigationBadHttpStatus"。
    NavigationBadHttpStatus,
    ///协议值 "ClientCertRequested"。
    ClientCertRequested,
    ///协议值 "NavigationRequestNetworkError"。
    NavigationRequestNetworkError,
    ///协议值 "CancelAllHostsForTesting"。
    CancelAllHostsForTesting,
    ///协议值 "DidFailLoad"。
    DidFailLoad,
    ///协议值 "Stop"。
    Stop,
    ///协议值 "SslCertificateError"。
    SslCertificateError,
    ///协议值 "LoginAuthRequested"。
    LoginAuthRequested,
    ///协议值 "UaChangeRequiresReload"。
    UaChangeRequiresReload,
    ///协议值 "BlockedByClient"。
    BlockedByClient,
    ///协议值 "AudioOutputDeviceRequested"。
    AudioOutputDeviceRequested,
    ///协议值 "MixedContent"。
    MixedContent,
    ///协议值 "TriggerBackgrounded"。
    TriggerBackgrounded,
    ///协议值 "MemoryLimitExceeded"。
    MemoryLimitExceeded,
    ///协议值 "DataSaverEnabled"。
    DataSaverEnabled,
    ///协议值 "TriggerUrlHasEffectiveUrl"。
    TriggerUrlHasEffectiveUrl,
    ///协议值 "ActivatedBeforeStarted"。
    ActivatedBeforeStarted,
    ///协议值 "InactivePageRestriction"。
    InactivePageRestriction,
    ///协议值 "StartFailed"。
    StartFailed,
    ///协议值 "TimeoutBackgrounded"。
    TimeoutBackgrounded,
    ///协议值 "CrossSiteRedirectInInitialNavigation"。
    CrossSiteRedirectInInitialNavigation,
    ///协议值 "CrossSiteNavigationInInitialNavigation"。
    CrossSiteNavigationInInitialNavigation,
    ///协议值 "SameSiteCrossOriginRedirectNotOptInInInitialNavigation"。
    SameSiteCrossOriginRedirectNotOptInInInitialNavigation,
    ///协议值 "SameSiteCrossOriginNavigationNotOptInInInitialNavigation"。
    SameSiteCrossOriginNavigationNotOptInInInitialNavigation,
    ///协议值 "ActivationNavigationParameterMismatch"。
    ActivationNavigationParameterMismatch,
    ///协议值 "ActivatedInBackground"。
    ActivatedInBackground,
    ///协议值 "EmbedderHostDisallowed"。
    EmbedderHostDisallowed,
    ///协议值 "ActivationNavigationDestroyedBeforeSuccess"。
    ActivationNavigationDestroyedBeforeSuccess,
    ///协议值 "TabClosedByUserGesture"。
    TabClosedByUserGesture,
    ///协议值 "TabClosedWithoutUserGesture"。
    TabClosedWithoutUserGesture,
    ///协议值 "PrimaryMainFrameRendererProcessCrashed"。
    PrimaryMainFrameRendererProcessCrashed,
    ///协议值 "PrimaryMainFrameRendererProcessKilled"。
    PrimaryMainFrameRendererProcessKilled,
    ///协议值 "ActivationFramePolicyNotCompatible"。
    ActivationFramePolicyNotCompatible,
    ///协议值 "PreloadingDisabled"。
    PreloadingDisabled,
    ///协议值 "BatterySaverEnabled"。
    BatterySaverEnabled,
    ///协议值 "ActivatedDuringMainFrameNavigation"。
    ActivatedDuringMainFrameNavigation,
    ///协议值 "PreloadingUnsupportedByWebContents"。
    PreloadingUnsupportedByWebContents,
    ///协议值 "CrossSiteRedirectInMainFrameNavigation"。
    CrossSiteRedirectInMainFrameNavigation,
    ///协议值 "CrossSiteNavigationInMainFrameNavigation"。
    CrossSiteNavigationInMainFrameNavigation,
    ///协议值 "SameSiteCrossOriginRedirectNotOptInInMainFrameNavigation"。
    SameSiteCrossOriginRedirectNotOptInInMainFrameNavigation,
    ///协议值 "SameSiteCrossOriginNavigationNotOptInInMainFrameNavigation"。
    SameSiteCrossOriginNavigationNotOptInInMainFrameNavigation,
    ///协议值 "MemoryPressureOnTrigger"。
    MemoryPressureOnTrigger,
    ///协议值 "MemoryPressureAfterTriggered"。
    MemoryPressureAfterTriggered,
    ///协议值 "PrerenderingDisabledByDevTools"。
    PrerenderingDisabledByDevTools,
    ///协议值 "SpeculationRuleRemoved"。
    SpeculationRuleRemoved,
    ///协议值 "ActivatedWithAuxiliaryBrowsingContexts"。
    ActivatedWithAuxiliaryBrowsingContexts,
    ///协议值 "MaxNumOfRunningEagerPrerendersExceeded"。
    MaxNumOfRunningEagerPrerendersExceeded,
    ///协议值 "MaxNumOfRunningNonEagerPrerendersExceeded"。
    MaxNumOfRunningNonEagerPrerendersExceeded,
    ///协议值 "MaxNumOfRunningEmbedderPrerendersExceeded"。
    MaxNumOfRunningEmbedderPrerendersExceeded,
    ///协议值 "PrerenderingUrlHasEffectiveUrl"。
    PrerenderingUrlHasEffectiveUrl,
    ///协议值 "RedirectedPrerenderingUrlHasEffectiveUrl"。
    RedirectedPrerenderingUrlHasEffectiveUrl,
    ///协议值 "ActivationUrlHasEffectiveUrl"。
    ActivationUrlHasEffectiveUrl,
    ///协议值 "JavaScriptInterfaceAdded"。
    JavaScriptInterfaceAdded,
    ///协议值 "JavaScriptInterfaceRemoved"。
    JavaScriptInterfaceRemoved,
    ///协议值 "AllPrerenderingCanceled"。
    AllPrerenderingCanceled,
    ///协议值 "WindowClosed"。
    WindowClosed,
    ///协议值 "SlowNetwork"。
    SlowNetwork,
    ///协议值 "OtherPrerenderedPageActivated"。
    OtherPrerenderedPageActivated,
    ///协议值 "V8OptimizerDisabled"。
    V8OptimizerDisabled,
    ///协议值 "PrerenderFailedDuringPrefetch"。
    PrerenderFailedDuringPrefetch,
    ///协议值 "BrowsingDataRemoved"。
    BrowsingDataRemoved,
    ///协议值 "PrerenderHostReused"。
    PrerenderHostReused,
    ///协议值 "FormSubmitWhenPrerendering"。
    FormSubmitWhenPrerendering,
    ///协议值 "CrossDocumentRestart"。
    CrossDocumentRestart,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl PrerenderFinalStatus {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::preload::types::PrerenderFinalStatus::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Activated => "Activated",
            Self::Destroyed => "Destroyed",
            Self::LowEndDevice => "LowEndDevice",
            Self::InvalidSchemeRedirect => "InvalidSchemeRedirect",
            Self::InvalidSchemeNavigation => "InvalidSchemeNavigation",
            Self::NavigationRequestBlockedByCsp => "NavigationRequestBlockedByCsp",
            Self::MojoBinderPolicy => "MojoBinderPolicy",
            Self::RendererProcessCrashed => "RendererProcessCrashed",
            Self::RendererProcessKilled => "RendererProcessKilled",
            Self::Download => "Download",
            Self::TriggerDestroyed => "TriggerDestroyed",
            Self::NavigationNotCommitted => "NavigationNotCommitted",
            Self::NavigationBadHttpStatus => "NavigationBadHttpStatus",
            Self::ClientCertRequested => "ClientCertRequested",
            Self::NavigationRequestNetworkError => "NavigationRequestNetworkError",
            Self::CancelAllHostsForTesting => "CancelAllHostsForTesting",
            Self::DidFailLoad => "DidFailLoad",
            Self::Stop => "Stop",
            Self::SslCertificateError => "SslCertificateError",
            Self::LoginAuthRequested => "LoginAuthRequested",
            Self::UaChangeRequiresReload => "UaChangeRequiresReload",
            Self::BlockedByClient => "BlockedByClient",
            Self::AudioOutputDeviceRequested => "AudioOutputDeviceRequested",
            Self::MixedContent => "MixedContent",
            Self::TriggerBackgrounded => "TriggerBackgrounded",
            Self::MemoryLimitExceeded => "MemoryLimitExceeded",
            Self::DataSaverEnabled => "DataSaverEnabled",
            Self::TriggerUrlHasEffectiveUrl => "TriggerUrlHasEffectiveUrl",
            Self::ActivatedBeforeStarted => "ActivatedBeforeStarted",
            Self::InactivePageRestriction => "InactivePageRestriction",
            Self::StartFailed => "StartFailed",
            Self::TimeoutBackgrounded => "TimeoutBackgrounded",
            Self::CrossSiteRedirectInInitialNavigation => "CrossSiteRedirectInInitialNavigation",
            Self::CrossSiteNavigationInInitialNavigation => {
                "CrossSiteNavigationInInitialNavigation"
            }
            Self::SameSiteCrossOriginRedirectNotOptInInInitialNavigation => {
                "SameSiteCrossOriginRedirectNotOptInInInitialNavigation"
            }
            Self::SameSiteCrossOriginNavigationNotOptInInInitialNavigation => {
                "SameSiteCrossOriginNavigationNotOptInInInitialNavigation"
            }
            Self::ActivationNavigationParameterMismatch => "ActivationNavigationParameterMismatch",
            Self::ActivatedInBackground => "ActivatedInBackground",
            Self::EmbedderHostDisallowed => "EmbedderHostDisallowed",
            Self::ActivationNavigationDestroyedBeforeSuccess => {
                "ActivationNavigationDestroyedBeforeSuccess"
            }
            Self::TabClosedByUserGesture => "TabClosedByUserGesture",
            Self::TabClosedWithoutUserGesture => "TabClosedWithoutUserGesture",
            Self::PrimaryMainFrameRendererProcessCrashed => {
                "PrimaryMainFrameRendererProcessCrashed"
            }
            Self::PrimaryMainFrameRendererProcessKilled => "PrimaryMainFrameRendererProcessKilled",
            Self::ActivationFramePolicyNotCompatible => "ActivationFramePolicyNotCompatible",
            Self::PreloadingDisabled => "PreloadingDisabled",
            Self::BatterySaverEnabled => "BatterySaverEnabled",
            Self::ActivatedDuringMainFrameNavigation => "ActivatedDuringMainFrameNavigation",
            Self::PreloadingUnsupportedByWebContents => "PreloadingUnsupportedByWebContents",
            Self::CrossSiteRedirectInMainFrameNavigation => {
                "CrossSiteRedirectInMainFrameNavigation"
            }
            Self::CrossSiteNavigationInMainFrameNavigation => {
                "CrossSiteNavigationInMainFrameNavigation"
            }
            Self::SameSiteCrossOriginRedirectNotOptInInMainFrameNavigation => {
                "SameSiteCrossOriginRedirectNotOptInInMainFrameNavigation"
            }
            Self::SameSiteCrossOriginNavigationNotOptInInMainFrameNavigation => {
                "SameSiteCrossOriginNavigationNotOptInInMainFrameNavigation"
            }
            Self::MemoryPressureOnTrigger => "MemoryPressureOnTrigger",
            Self::MemoryPressureAfterTriggered => "MemoryPressureAfterTriggered",
            Self::PrerenderingDisabledByDevTools => "PrerenderingDisabledByDevTools",
            Self::SpeculationRuleRemoved => "SpeculationRuleRemoved",
            Self::ActivatedWithAuxiliaryBrowsingContexts => {
                "ActivatedWithAuxiliaryBrowsingContexts"
            }
            Self::MaxNumOfRunningEagerPrerendersExceeded => {
                "MaxNumOfRunningEagerPrerendersExceeded"
            }
            Self::MaxNumOfRunningNonEagerPrerendersExceeded => {
                "MaxNumOfRunningNonEagerPrerendersExceeded"
            }
            Self::MaxNumOfRunningEmbedderPrerendersExceeded => {
                "MaxNumOfRunningEmbedderPrerendersExceeded"
            }
            Self::PrerenderingUrlHasEffectiveUrl => "PrerenderingUrlHasEffectiveUrl",
            Self::RedirectedPrerenderingUrlHasEffectiveUrl => {
                "RedirectedPrerenderingUrlHasEffectiveUrl"
            }
            Self::ActivationUrlHasEffectiveUrl => "ActivationUrlHasEffectiveUrl",
            Self::JavaScriptInterfaceAdded => "JavaScriptInterfaceAdded",
            Self::JavaScriptInterfaceRemoved => "JavaScriptInterfaceRemoved",
            Self::AllPrerenderingCanceled => "AllPrerenderingCanceled",
            Self::WindowClosed => "WindowClosed",
            Self::SlowNetwork => "SlowNetwork",
            Self::OtherPrerenderedPageActivated => "OtherPrerenderedPageActivated",
            Self::V8OptimizerDisabled => "V8OptimizerDisabled",
            Self::PrerenderFailedDuringPrefetch => "PrerenderFailedDuringPrefetch",
            Self::BrowsingDataRemoved => "BrowsingDataRemoved",
            Self::PrerenderHostReused => "PrerenderHostReused",
            Self::FormSubmitWhenPrerendering => "FormSubmitWhenPrerendering",
            Self::CrossDocumentRestart => "CrossDocumentRestart",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for PrerenderFinalStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for PrerenderFinalStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Activated" => PrerenderFinalStatus::Activated,
            "Destroyed" => PrerenderFinalStatus::Destroyed,
            "LowEndDevice" => PrerenderFinalStatus::LowEndDevice,
            "InvalidSchemeRedirect" => PrerenderFinalStatus::InvalidSchemeRedirect,
            "InvalidSchemeNavigation" => PrerenderFinalStatus::InvalidSchemeNavigation,
            "NavigationRequestBlockedByCsp" => PrerenderFinalStatus::NavigationRequestBlockedByCsp,
            "MojoBinderPolicy" => PrerenderFinalStatus::MojoBinderPolicy,
            "RendererProcessCrashed" => PrerenderFinalStatus::RendererProcessCrashed,
            "RendererProcessKilled" => PrerenderFinalStatus::RendererProcessKilled,
            "Download" => PrerenderFinalStatus::Download,
            "TriggerDestroyed" => PrerenderFinalStatus::TriggerDestroyed,
            "NavigationNotCommitted" => PrerenderFinalStatus::NavigationNotCommitted,
            "NavigationBadHttpStatus" => PrerenderFinalStatus::NavigationBadHttpStatus,
            "ClientCertRequested" => PrerenderFinalStatus::ClientCertRequested,
            "NavigationRequestNetworkError" => PrerenderFinalStatus::NavigationRequestNetworkError,
            "CancelAllHostsForTesting" => PrerenderFinalStatus::CancelAllHostsForTesting,
            "DidFailLoad" => PrerenderFinalStatus::DidFailLoad,
            "Stop" => PrerenderFinalStatus::Stop,
            "SslCertificateError" => PrerenderFinalStatus::SslCertificateError,
            "LoginAuthRequested" => PrerenderFinalStatus::LoginAuthRequested,
            "UaChangeRequiresReload" => PrerenderFinalStatus::UaChangeRequiresReload,
            "BlockedByClient" => PrerenderFinalStatus::BlockedByClient,
            "AudioOutputDeviceRequested" => PrerenderFinalStatus::AudioOutputDeviceRequested,
            "MixedContent" => PrerenderFinalStatus::MixedContent,
            "TriggerBackgrounded" => PrerenderFinalStatus::TriggerBackgrounded,
            "MemoryLimitExceeded" => PrerenderFinalStatus::MemoryLimitExceeded,
            "DataSaverEnabled" => PrerenderFinalStatus::DataSaverEnabled,
            "TriggerUrlHasEffectiveUrl" => PrerenderFinalStatus::TriggerUrlHasEffectiveUrl,
            "ActivatedBeforeStarted" => PrerenderFinalStatus::ActivatedBeforeStarted,
            "InactivePageRestriction" => PrerenderFinalStatus::InactivePageRestriction,
            "StartFailed" => PrerenderFinalStatus::StartFailed,
            "TimeoutBackgrounded" => PrerenderFinalStatus::TimeoutBackgrounded,
            "CrossSiteRedirectInInitialNavigation" => {
                PrerenderFinalStatus::CrossSiteRedirectInInitialNavigation
            }
            "CrossSiteNavigationInInitialNavigation" => {
                PrerenderFinalStatus::CrossSiteNavigationInInitialNavigation
            }
            "SameSiteCrossOriginRedirectNotOptInInInitialNavigation" => {
                PrerenderFinalStatus::SameSiteCrossOriginRedirectNotOptInInInitialNavigation
            }
            "SameSiteCrossOriginNavigationNotOptInInInitialNavigation" => {
                PrerenderFinalStatus::SameSiteCrossOriginNavigationNotOptInInInitialNavigation
            }
            "ActivationNavigationParameterMismatch" => {
                PrerenderFinalStatus::ActivationNavigationParameterMismatch
            }
            "ActivatedInBackground" => PrerenderFinalStatus::ActivatedInBackground,
            "EmbedderHostDisallowed" => PrerenderFinalStatus::EmbedderHostDisallowed,
            "ActivationNavigationDestroyedBeforeSuccess" => {
                PrerenderFinalStatus::ActivationNavigationDestroyedBeforeSuccess
            }
            "TabClosedByUserGesture" => PrerenderFinalStatus::TabClosedByUserGesture,
            "TabClosedWithoutUserGesture" => PrerenderFinalStatus::TabClosedWithoutUserGesture,
            "PrimaryMainFrameRendererProcessCrashed" => {
                PrerenderFinalStatus::PrimaryMainFrameRendererProcessCrashed
            }
            "PrimaryMainFrameRendererProcessKilled" => {
                PrerenderFinalStatus::PrimaryMainFrameRendererProcessKilled
            }
            "ActivationFramePolicyNotCompatible" => {
                PrerenderFinalStatus::ActivationFramePolicyNotCompatible
            }
            "PreloadingDisabled" => PrerenderFinalStatus::PreloadingDisabled,
            "BatterySaverEnabled" => PrerenderFinalStatus::BatterySaverEnabled,
            "ActivatedDuringMainFrameNavigation" => {
                PrerenderFinalStatus::ActivatedDuringMainFrameNavigation
            }
            "PreloadingUnsupportedByWebContents" => {
                PrerenderFinalStatus::PreloadingUnsupportedByWebContents
            }
            "CrossSiteRedirectInMainFrameNavigation" => {
                PrerenderFinalStatus::CrossSiteRedirectInMainFrameNavigation
            }
            "CrossSiteNavigationInMainFrameNavigation" => {
                PrerenderFinalStatus::CrossSiteNavigationInMainFrameNavigation
            }
            "SameSiteCrossOriginRedirectNotOptInInMainFrameNavigation" => {
                PrerenderFinalStatus::SameSiteCrossOriginRedirectNotOptInInMainFrameNavigation
            }
            "SameSiteCrossOriginNavigationNotOptInInMainFrameNavigation" => {
                PrerenderFinalStatus::SameSiteCrossOriginNavigationNotOptInInMainFrameNavigation
            }
            "MemoryPressureOnTrigger" => PrerenderFinalStatus::MemoryPressureOnTrigger,
            "MemoryPressureAfterTriggered" => PrerenderFinalStatus::MemoryPressureAfterTriggered,
            "PrerenderingDisabledByDevTools" => {
                PrerenderFinalStatus::PrerenderingDisabledByDevTools
            }
            "SpeculationRuleRemoved" => PrerenderFinalStatus::SpeculationRuleRemoved,
            "ActivatedWithAuxiliaryBrowsingContexts" => {
                PrerenderFinalStatus::ActivatedWithAuxiliaryBrowsingContexts
            }
            "MaxNumOfRunningEagerPrerendersExceeded" => {
                PrerenderFinalStatus::MaxNumOfRunningEagerPrerendersExceeded
            }
            "MaxNumOfRunningNonEagerPrerendersExceeded" => {
                PrerenderFinalStatus::MaxNumOfRunningNonEagerPrerendersExceeded
            }
            "MaxNumOfRunningEmbedderPrerendersExceeded" => {
                PrerenderFinalStatus::MaxNumOfRunningEmbedderPrerendersExceeded
            }
            "PrerenderingUrlHasEffectiveUrl" => {
                PrerenderFinalStatus::PrerenderingUrlHasEffectiveUrl
            }
            "RedirectedPrerenderingUrlHasEffectiveUrl" => {
                PrerenderFinalStatus::RedirectedPrerenderingUrlHasEffectiveUrl
            }
            "ActivationUrlHasEffectiveUrl" => PrerenderFinalStatus::ActivationUrlHasEffectiveUrl,
            "JavaScriptInterfaceAdded" => PrerenderFinalStatus::JavaScriptInterfaceAdded,
            "JavaScriptInterfaceRemoved" => PrerenderFinalStatus::JavaScriptInterfaceRemoved,
            "AllPrerenderingCanceled" => PrerenderFinalStatus::AllPrerenderingCanceled,
            "WindowClosed" => PrerenderFinalStatus::WindowClosed,
            "SlowNetwork" => PrerenderFinalStatus::SlowNetwork,
            "OtherPrerenderedPageActivated" => PrerenderFinalStatus::OtherPrerenderedPageActivated,
            "V8OptimizerDisabled" => PrerenderFinalStatus::V8OptimizerDisabled,
            "PrerenderFailedDuringPrefetch" => PrerenderFinalStatus::PrerenderFailedDuringPrefetch,
            "BrowsingDataRemoved" => PrerenderFinalStatus::BrowsingDataRemoved,
            "PrerenderHostReused" => PrerenderFinalStatus::PrerenderHostReused,
            "FormSubmitWhenPrerendering" => PrerenderFinalStatus::FormSubmitWhenPrerendering,
            "CrossDocumentRestart" => PrerenderFinalStatus::CrossDocumentRestart,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for PrerenderFinalStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PrerenderFinalStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PrerenderFinalStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(
                    match v {
                        "Activated" => PrerenderFinalStatus::Activated,
                        "Destroyed" => PrerenderFinalStatus::Destroyed,
                        "LowEndDevice" => PrerenderFinalStatus::LowEndDevice,
                        "InvalidSchemeRedirect" => {
                            PrerenderFinalStatus::InvalidSchemeRedirect
                        }
                        "InvalidSchemeNavigation" => {
                            PrerenderFinalStatus::InvalidSchemeNavigation
                        }
                        "NavigationRequestBlockedByCsp" => {
                            PrerenderFinalStatus::NavigationRequestBlockedByCsp
                        }
                        "MojoBinderPolicy" => PrerenderFinalStatus::MojoBinderPolicy,
                        "RendererProcessCrashed" => {
                            PrerenderFinalStatus::RendererProcessCrashed
                        }
                        "RendererProcessKilled" => {
                            PrerenderFinalStatus::RendererProcessKilled
                        }
                        "Download" => PrerenderFinalStatus::Download,
                        "TriggerDestroyed" => PrerenderFinalStatus::TriggerDestroyed,
                        "NavigationNotCommitted" => {
                            PrerenderFinalStatus::NavigationNotCommitted
                        }
                        "NavigationBadHttpStatus" => {
                            PrerenderFinalStatus::NavigationBadHttpStatus
                        }
                        "ClientCertRequested" => {
                            PrerenderFinalStatus::ClientCertRequested
                        }
                        "NavigationRequestNetworkError" => {
                            PrerenderFinalStatus::NavigationRequestNetworkError
                        }
                        "CancelAllHostsForTesting" => {
                            PrerenderFinalStatus::CancelAllHostsForTesting
                        }
                        "DidFailLoad" => PrerenderFinalStatus::DidFailLoad,
                        "Stop" => PrerenderFinalStatus::Stop,
                        "SslCertificateError" => {
                            PrerenderFinalStatus::SslCertificateError
                        }
                        "LoginAuthRequested" => PrerenderFinalStatus::LoginAuthRequested,
                        "UaChangeRequiresReload" => {
                            PrerenderFinalStatus::UaChangeRequiresReload
                        }
                        "BlockedByClient" => PrerenderFinalStatus::BlockedByClient,
                        "AudioOutputDeviceRequested" => {
                            PrerenderFinalStatus::AudioOutputDeviceRequested
                        }
                        "MixedContent" => PrerenderFinalStatus::MixedContent,
                        "TriggerBackgrounded" => {
                            PrerenderFinalStatus::TriggerBackgrounded
                        }
                        "MemoryLimitExceeded" => {
                            PrerenderFinalStatus::MemoryLimitExceeded
                        }
                        "DataSaverEnabled" => PrerenderFinalStatus::DataSaverEnabled,
                        "TriggerUrlHasEffectiveUrl" => {
                            PrerenderFinalStatus::TriggerUrlHasEffectiveUrl
                        }
                        "ActivatedBeforeStarted" => {
                            PrerenderFinalStatus::ActivatedBeforeStarted
                        }
                        "InactivePageRestriction" => {
                            PrerenderFinalStatus::InactivePageRestriction
                        }
                        "StartFailed" => PrerenderFinalStatus::StartFailed,
                        "TimeoutBackgrounded" => {
                            PrerenderFinalStatus::TimeoutBackgrounded
                        }
                        "CrossSiteRedirectInInitialNavigation" => {
                            PrerenderFinalStatus::CrossSiteRedirectInInitialNavigation
                        }
                        "CrossSiteNavigationInInitialNavigation" => {
                            PrerenderFinalStatus::CrossSiteNavigationInInitialNavigation
                        }
                        "SameSiteCrossOriginRedirectNotOptInInInitialNavigation" => {
                            PrerenderFinalStatus::SameSiteCrossOriginRedirectNotOptInInInitialNavigation
                        }
                        "SameSiteCrossOriginNavigationNotOptInInInitialNavigation" => {
                            PrerenderFinalStatus::SameSiteCrossOriginNavigationNotOptInInInitialNavigation
                        }
                        "ActivationNavigationParameterMismatch" => {
                            PrerenderFinalStatus::ActivationNavigationParameterMismatch
                        }
                        "ActivatedInBackground" => {
                            PrerenderFinalStatus::ActivatedInBackground
                        }
                        "EmbedderHostDisallowed" => {
                            PrerenderFinalStatus::EmbedderHostDisallowed
                        }
                        "ActivationNavigationDestroyedBeforeSuccess" => {
                            PrerenderFinalStatus::ActivationNavigationDestroyedBeforeSuccess
                        }
                        "TabClosedByUserGesture" => {
                            PrerenderFinalStatus::TabClosedByUserGesture
                        }
                        "TabClosedWithoutUserGesture" => {
                            PrerenderFinalStatus::TabClosedWithoutUserGesture
                        }
                        "PrimaryMainFrameRendererProcessCrashed" => {
                            PrerenderFinalStatus::PrimaryMainFrameRendererProcessCrashed
                        }
                        "PrimaryMainFrameRendererProcessKilled" => {
                            PrerenderFinalStatus::PrimaryMainFrameRendererProcessKilled
                        }
                        "ActivationFramePolicyNotCompatible" => {
                            PrerenderFinalStatus::ActivationFramePolicyNotCompatible
                        }
                        "PreloadingDisabled" => PrerenderFinalStatus::PreloadingDisabled,
                        "BatterySaverEnabled" => {
                            PrerenderFinalStatus::BatterySaverEnabled
                        }
                        "ActivatedDuringMainFrameNavigation" => {
                            PrerenderFinalStatus::ActivatedDuringMainFrameNavigation
                        }
                        "PreloadingUnsupportedByWebContents" => {
                            PrerenderFinalStatus::PreloadingUnsupportedByWebContents
                        }
                        "CrossSiteRedirectInMainFrameNavigation" => {
                            PrerenderFinalStatus::CrossSiteRedirectInMainFrameNavigation
                        }
                        "CrossSiteNavigationInMainFrameNavigation" => {
                            PrerenderFinalStatus::CrossSiteNavigationInMainFrameNavigation
                        }
                        "SameSiteCrossOriginRedirectNotOptInInMainFrameNavigation" => {
                            PrerenderFinalStatus::SameSiteCrossOriginRedirectNotOptInInMainFrameNavigation
                        }
                        "SameSiteCrossOriginNavigationNotOptInInMainFrameNavigation" => {
                            PrerenderFinalStatus::SameSiteCrossOriginNavigationNotOptInInMainFrameNavigation
                        }
                        "MemoryPressureOnTrigger" => {
                            PrerenderFinalStatus::MemoryPressureOnTrigger
                        }
                        "MemoryPressureAfterTriggered" => {
                            PrerenderFinalStatus::MemoryPressureAfterTriggered
                        }
                        "PrerenderingDisabledByDevTools" => {
                            PrerenderFinalStatus::PrerenderingDisabledByDevTools
                        }
                        "SpeculationRuleRemoved" => {
                            PrerenderFinalStatus::SpeculationRuleRemoved
                        }
                        "ActivatedWithAuxiliaryBrowsingContexts" => {
                            PrerenderFinalStatus::ActivatedWithAuxiliaryBrowsingContexts
                        }
                        "MaxNumOfRunningEagerPrerendersExceeded" => {
                            PrerenderFinalStatus::MaxNumOfRunningEagerPrerendersExceeded
                        }
                        "MaxNumOfRunningNonEagerPrerendersExceeded" => {
                            PrerenderFinalStatus::MaxNumOfRunningNonEagerPrerendersExceeded
                        }
                        "MaxNumOfRunningEmbedderPrerendersExceeded" => {
                            PrerenderFinalStatus::MaxNumOfRunningEmbedderPrerendersExceeded
                        }
                        "PrerenderingUrlHasEffectiveUrl" => {
                            PrerenderFinalStatus::PrerenderingUrlHasEffectiveUrl
                        }
                        "RedirectedPrerenderingUrlHasEffectiveUrl" => {
                            PrerenderFinalStatus::RedirectedPrerenderingUrlHasEffectiveUrl
                        }
                        "ActivationUrlHasEffectiveUrl" => {
                            PrerenderFinalStatus::ActivationUrlHasEffectiveUrl
                        }
                        "JavaScriptInterfaceAdded" => {
                            PrerenderFinalStatus::JavaScriptInterfaceAdded
                        }
                        "JavaScriptInterfaceRemoved" => {
                            PrerenderFinalStatus::JavaScriptInterfaceRemoved
                        }
                        "AllPrerenderingCanceled" => {
                            PrerenderFinalStatus::AllPrerenderingCanceled
                        }
                        "WindowClosed" => PrerenderFinalStatus::WindowClosed,
                        "SlowNetwork" => PrerenderFinalStatus::SlowNetwork,
                        "OtherPrerenderedPageActivated" => {
                            PrerenderFinalStatus::OtherPrerenderedPageActivated
                        }
                        "V8OptimizerDisabled" => {
                            PrerenderFinalStatus::V8OptimizerDisabled
                        }
                        "PrerenderFailedDuringPrefetch" => {
                            PrerenderFinalStatus::PrerenderFailedDuringPrefetch
                        }
                        "BrowsingDataRemoved" => {
                            PrerenderFinalStatus::BrowsingDataRemoved
                        }
                        "PrerenderHostReused" => {
                            PrerenderFinalStatus::PrerenderHostReused
                        }
                        "FormSubmitWhenPrerendering" => {
                            PrerenderFinalStatus::FormSubmitWhenPrerendering
                        }
                        "CrossDocumentRestart" => {
                            PrerenderFinalStatus::CrossDocumentRestart
                        }
                        _ => PrerenderFinalStatus::Unknown(v.to_owned()),
                    },
                )
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(PrerenderFinalStatus::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Preload.type.PrerenderMismatchedHeaders。
///
///协议原文：
///
///<pre>Information of headers to be displayed when the header mismatch occurred.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct PrerenderMismatchedHeaders {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.type.PrerenderMismatchedHeaders/properties/headerName。
    #[serde(rename = "headerName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub header_name: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Preload.type.PrerenderMismatchedHeaders/properties/initialValue。
    #[serde(rename = "initialValue")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_value: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Preload.type.PrerenderMismatchedHeaders/properties/activationValue。
    #[serde(rename = "activationValue")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub activation_value: Option<String>,
}
impl<'de> serde::Deserialize<'de> for PrerenderMismatchedHeaders {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PrerenderMismatchedHeaders;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.type.PrerenderMismatchedHeaders/properties/headerName。
                    #[serde(rename = "headerName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub header_name: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Preload.type.PrerenderMismatchedHeaders/properties/initialValue。
                    #[serde(rename = "initialValue")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub initial_value: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Preload.type.PrerenderMismatchedHeaders/properties/activationValue。
                    #[serde(rename = "activationValue")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub activation_value: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PrerenderMismatchedHeaders {
                    header_name: fields.header_name,
                    initial_value: fields.initial_value,
                    activation_value: fields.activation_value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Preload.type.RuleSet。
///
///协议原文：
///
///<pre>Corresponds to SpeculationRuleSet</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct RuleSet {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.type.RuleSet/properties/id。
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: crate::preload::types::RuleSetId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.type.RuleSet/properties/loaderId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifies a document which the rule set is associated with.</pre>
    ///
    ///
    #[serde(rename = "loaderId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub loader_id: crate::network::types::LoaderId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.type.RuleSet/properties/sourceText。
    ///
    ///协议原文：
    ///
    ///<pre>Source text of JSON representing the rule set. If it comes from
    ///`&lt;script&gt;` tag, it is the textContent of the node. Note that it is
    ///a JSON for valid case.
    ///
    ///See also:
    ///- https://wicg.github.io/nav-speculation/speculation-rules.html
    ///- https://github.com/WICG/nav-speculation/blob/main/triggers.md</pre>
    ///
    ///
    #[serde(rename = "sourceText")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub source_text: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Preload.type.RuleSet/properties/backendNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>A speculation rule set is either added through an inline
    ///`&lt;script&gt;` tag or through an external resource via the
    ///'Speculation-Rules' HTTP header. For the first case, we include
    ///the BackendNodeId of the relevant `&lt;script&gt;` tag. For the second
    ///case, we include the external URL where the rule set was loaded
    ///from, and also RequestId if Network domain is enabled.
    ///
    ///See also:
    ///- https://wicg.github.io/nav-speculation/speculation-rules.html#speculation-rules-script
    ///- https://wicg.github.io/nav-speculation/speculation-rules.html#speculation-rules-header</pre>
    ///
    ///
    #[serde(rename = "backendNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Preload.type.RuleSet/properties/url。
    #[serde(rename = "url")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Preload.type.RuleSet/properties/requestId。
    #[serde(rename = "requestId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub request_id: Option<crate::network::types::RequestId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Preload.type.RuleSet/properties/errorType。
    ///
    ///协议原文：
    ///
    ///<pre>Error information
    ///`errorMessage` is null iff `errorType` is null.</pre>
    ///
    ///
    #[serde(rename = "errorType")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub error_type: Option<crate::preload::types::RuleSetErrorType>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Preload.type.RuleSet/properties/errorMessage。
    ///
    ///协议原文：
    ///
    ///<pre>TODO(https://crbug.com/1425354): Replace this property with structured error.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "errorMessage")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Preload.type.RuleSet/properties/tag。
    ///
    ///协议原文：
    ///
    ///<pre>For more details, see:
    ///https://github.com/WICG/nav-speculation/blob/main/speculation-rules-tags.md</pre>
    ///
    ///
    #[serde(rename = "tag")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub tag: Option<String>,
}
impl<'de> serde::Deserialize<'de> for RuleSet {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RuleSet;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.type.RuleSet/properties/id。
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: crate::preload::types::RuleSetId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.type.RuleSet/properties/loaderId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifies a document which the rule set is associated with.</pre>
                    ///
                    ///
                    #[serde(rename = "loaderId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub loader_id: crate::network::types::LoaderId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.type.RuleSet/properties/sourceText。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Source text of JSON representing the rule set. If it comes from
                    ///`&lt;script&gt;` tag, it is the textContent of the node. Note that it is
                    ///a JSON for valid case.
                    ///
                    ///See also:
                    ///- https://wicg.github.io/nav-speculation/speculation-rules.html
                    ///- https://github.com/WICG/nav-speculation/blob/main/triggers.md</pre>
                    ///
                    ///
                    #[serde(rename = "sourceText")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub source_text: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Preload.type.RuleSet/properties/backendNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A speculation rule set is either added through an inline
                    ///`&lt;script&gt;` tag or through an external resource via the
                    ///'Speculation-Rules' HTTP header. For the first case, we include
                    ///the BackendNodeId of the relevant `&lt;script&gt;` tag. For the second
                    ///case, we include the external URL where the rule set was loaded
                    ///from, and also RequestId if Network domain is enabled.
                    ///
                    ///See also:
                    ///- https://wicg.github.io/nav-speculation/speculation-rules.html#speculation-rules-script
                    ///- https://wicg.github.io/nav-speculation/speculation-rules.html#speculation-rules-header</pre>
                    ///
                    ///
                    #[serde(rename = "backendNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Preload.type.RuleSet/properties/url。
                    #[serde(rename = "url")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub url: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Preload.type.RuleSet/properties/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub request_id: Option<crate::network::types::RequestId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Preload.type.RuleSet/properties/errorType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Error information
                    ///`errorMessage` is null iff `errorType` is null.</pre>
                    ///
                    ///
                    #[serde(rename = "errorType")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub error_type: Option<crate::preload::types::RuleSetErrorType>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Preload.type.RuleSet/properties/errorMessage。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>TODO(https://crbug.com/1425354): Replace this property with structured error.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "errorMessage")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub error_message: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Preload.type.RuleSet/properties/tag。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>For more details, see:
                    ///https://github.com/WICG/nav-speculation/blob/main/speculation-rules-tags.md</pre>
                    ///
                    ///
                    #[serde(rename = "tag")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub tag: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RuleSet {
                    id: fields.id,
                    loader_id: fields.loader_id,
                    source_text: fields.source_text,
                    backend_node_id: fields.backend_node_id,
                    url: fields.url,
                    request_id: fields.request_id,
                    error_type: fields.error_type,
                    error_message: fields.error_message,
                    tag: fields.tag,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Preload.type.RuleSetErrorType。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RuleSetErrorType {
    ///协议值 "SourceIsNotJsonObject"。
    SourceIsNotJsonObject,
    ///协议值 "InvalidRulesSkipped"。
    InvalidRulesSkipped,
    ///协议值 "InvalidRulesetLevelTag"。
    InvalidRulesetLevelTag,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl RuleSetErrorType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::preload::types::RuleSetErrorType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::SourceIsNotJsonObject => "SourceIsNotJsonObject",
            Self::InvalidRulesSkipped => "InvalidRulesSkipped",
            Self::InvalidRulesetLevelTag => "InvalidRulesetLevelTag",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for RuleSetErrorType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for RuleSetErrorType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "SourceIsNotJsonObject" => RuleSetErrorType::SourceIsNotJsonObject,
            "InvalidRulesSkipped" => RuleSetErrorType::InvalidRulesSkipped,
            "InvalidRulesetLevelTag" => RuleSetErrorType::InvalidRulesetLevelTag,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for RuleSetErrorType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RuleSetErrorType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RuleSetErrorType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "SourceIsNotJsonObject" => RuleSetErrorType::SourceIsNotJsonObject,
                    "InvalidRulesSkipped" => RuleSetErrorType::InvalidRulesSkipped,
                    "InvalidRulesetLevelTag" => RuleSetErrorType::InvalidRulesetLevelTag,
                    _ => RuleSetErrorType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(RuleSetErrorType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Preload.type.RuleSetId。
///
///协议原文：
///
///<pre>Unique id</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct RuleSetId(String);
impl RuleSetId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::preload::types::RuleSetId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::preload::types::RuleSetId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::preload::types::RuleSetId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::preload::types::RuleSetId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::preload::types::RuleSetId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Preload.type.SpeculationAction。
///
///协议原文：
///
///<pre>The type of preloading attempted. It corresponds to
///mojom::SpeculationAction (although PrefetchWithSubresources is omitted as it
///isn't being used by clients).</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SpeculationAction {
    ///协议值 "Prefetch"。
    Prefetch,
    ///协议值 "Prerender"。
    Prerender,
    ///协议值 "PrerenderUntilScript"。
    PrerenderUntilScript,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl SpeculationAction {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::preload::types::SpeculationAction::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Prefetch => "Prefetch",
            Self::Prerender => "Prerender",
            Self::PrerenderUntilScript => "PrerenderUntilScript",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for SpeculationAction {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for SpeculationAction {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Prefetch" => SpeculationAction::Prefetch,
            "Prerender" => SpeculationAction::Prerender,
            "PrerenderUntilScript" => SpeculationAction::PrerenderUntilScript,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for SpeculationAction {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SpeculationAction {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SpeculationAction;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Prefetch" => SpeculationAction::Prefetch,
                    "Prerender" => SpeculationAction::Prerender,
                    "PrerenderUntilScript" => SpeculationAction::PrerenderUntilScript,
                    _ => SpeculationAction::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(SpeculationAction::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Preload.type.SpeculationTargetHint。
///
///协议原文：
///
///<pre>Corresponds to mojom::SpeculationTargetHint.
///See https://github.com/WICG/nav-speculation/blob/main/triggers.md#window-name-targeting-hints</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SpeculationTargetHint {
    ///协议值 "Blank"。
    Blank,
    ///协议值 "Self"。
    Self_,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl SpeculationTargetHint {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::preload::types::SpeculationTargetHint::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Blank => "Blank",
            Self::Self_ => "Self",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for SpeculationTargetHint {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for SpeculationTargetHint {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Blank" => SpeculationTargetHint::Blank,
            "Self" => SpeculationTargetHint::Self_,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for SpeculationTargetHint {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SpeculationTargetHint {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SpeculationTargetHint;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Blank" => SpeculationTargetHint::Blank,
                    "Self" => SpeculationTargetHint::Self_,
                    _ => SpeculationTargetHint::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(SpeculationTargetHint::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
