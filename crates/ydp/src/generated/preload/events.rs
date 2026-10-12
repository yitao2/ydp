///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Preload.event.prefetchStatusUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PrefetchStatusUpdatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.prefetchStatusUpdated/parameters/key。
    #[serde(rename = "key")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub key: crate::preload::types::PreloadingAttemptKey,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.prefetchStatusUpdated/parameters/pipelineId。
    #[serde(rename = "pipelineId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub pipeline_id: crate::preload::types::PreloadPipelineId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.prefetchStatusUpdated/parameters/initiatingFrameId。
    ///
    ///协议原文：
    ///
    ///<pre>The frame id of the frame initiating prefetch.</pre>
    ///
    ///
    #[serde(rename = "initiatingFrameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub initiating_frame_id: crate::page::types::FrameId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.prefetchStatusUpdated/parameters/prefetchUrl。
    #[serde(rename = "prefetchUrl")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub prefetch_url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.prefetchStatusUpdated/parameters/status。
    #[serde(rename = "status")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub status: crate::preload::types::PreloadingStatus,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.prefetchStatusUpdated/parameters/prefetchStatus。
    #[serde(rename = "prefetchStatus")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub prefetch_status: crate::preload::types::PrefetchStatus,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.prefetchStatusUpdated/parameters/requestId。
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::network::types::RequestId,
}
impl<'de> serde::Deserialize<'de> for PrefetchStatusUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PrefetchStatusUpdatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.prefetchStatusUpdated/parameters/key。
                    #[serde(rename = "key")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub key: crate::preload::types::PreloadingAttemptKey,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.prefetchStatusUpdated/parameters/pipelineId。
                    #[serde(rename = "pipelineId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub pipeline_id: crate::preload::types::PreloadPipelineId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.prefetchStatusUpdated/parameters/initiatingFrameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The frame id of the frame initiating prefetch.</pre>
                    ///
                    ///
                    #[serde(rename = "initiatingFrameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub initiating_frame_id: crate::page::types::FrameId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.prefetchStatusUpdated/parameters/prefetchUrl。
                    #[serde(rename = "prefetchUrl")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub prefetch_url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.prefetchStatusUpdated/parameters/status。
                    #[serde(rename = "status")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub status: crate::preload::types::PreloadingStatus,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.prefetchStatusUpdated/parameters/prefetchStatus。
                    #[serde(rename = "prefetchStatus")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub prefetch_status: crate::preload::types::PrefetchStatus,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.prefetchStatusUpdated/parameters/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::network::types::RequestId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PrefetchStatusUpdatedParams {
                    key: fields.key,
                    pipeline_id: fields.pipeline_id,
                    initiating_frame_id: fields.initiating_frame_id,
                    prefetch_url: fields.prefetch_url,
                    status: fields.status,
                    prefetch_status: fields.prefetch_status,
                    request_id: fields.request_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Preload.event.prefetchStatusUpdated。
///
///协议原文：
///
///<pre>Fired when a prefetch attempt is updated.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct PrefetchStatusUpdated;
impl ydp_core::Event for PrefetchStatusUpdated {
    type Params = PrefetchStatusUpdatedParams;
    const METHOD: &'static str = "Preload.prefetchStatusUpdated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Preload.event.preloadEnabledStateUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PreloadEnabledStateUpdatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.preloadEnabledStateUpdated/parameters/disabledByPreference。
    #[serde(rename = "disabledByPreference")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub disabled_by_preference: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.preloadEnabledStateUpdated/parameters/disabledByDataSaver。
    #[serde(rename = "disabledByDataSaver")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub disabled_by_data_saver: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.preloadEnabledStateUpdated/parameters/disabledByBatterySaver。
    #[serde(rename = "disabledByBatterySaver")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub disabled_by_battery_saver: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.preloadEnabledStateUpdated/parameters/disabledByHoldbackPrefetchSpeculationRules。
    #[serde(rename = "disabledByHoldbackPrefetchSpeculationRules")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub disabled_by_holdback_prefetch_speculation_rules: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.preloadEnabledStateUpdated/parameters/disabledByHoldbackPrerenderSpeculationRules。
    #[serde(rename = "disabledByHoldbackPrerenderSpeculationRules")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub disabled_by_holdback_prerender_speculation_rules: bool,
}
impl<'de> serde::Deserialize<'de> for PreloadEnabledStateUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PreloadEnabledStateUpdatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.preloadEnabledStateUpdated/parameters/disabledByPreference。
                    #[serde(rename = "disabledByPreference")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub disabled_by_preference: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.preloadEnabledStateUpdated/parameters/disabledByDataSaver。
                    #[serde(rename = "disabledByDataSaver")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub disabled_by_data_saver: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.preloadEnabledStateUpdated/parameters/disabledByBatterySaver。
                    #[serde(rename = "disabledByBatterySaver")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub disabled_by_battery_saver: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.preloadEnabledStateUpdated/parameters/disabledByHoldbackPrefetchSpeculationRules。
                    #[serde(rename = "disabledByHoldbackPrefetchSpeculationRules")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub disabled_by_holdback_prefetch_speculation_rules: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.preloadEnabledStateUpdated/parameters/disabledByHoldbackPrerenderSpeculationRules。
                    #[serde(rename = "disabledByHoldbackPrerenderSpeculationRules")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub disabled_by_holdback_prerender_speculation_rules: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PreloadEnabledStateUpdatedParams {
                    disabled_by_preference: fields.disabled_by_preference,
                    disabled_by_data_saver: fields.disabled_by_data_saver,
                    disabled_by_battery_saver: fields.disabled_by_battery_saver,
                    disabled_by_holdback_prefetch_speculation_rules: fields
                        .disabled_by_holdback_prefetch_speculation_rules,
                    disabled_by_holdback_prerender_speculation_rules: fields
                        .disabled_by_holdback_prerender_speculation_rules,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Preload.event.preloadEnabledStateUpdated。
///
///协议原文：
///
///<pre>Fired when a preload enabled state is updated.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct PreloadEnabledStateUpdated;
impl ydp_core::Event for PreloadEnabledStateUpdated {
    type Params = PreloadEnabledStateUpdatedParams;
    const METHOD: &'static str = "Preload.preloadEnabledStateUpdated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Preload.event.preloadingAttemptSourcesUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PreloadingAttemptSourcesUpdatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.preloadingAttemptSourcesUpdated/parameters/loaderId。
    #[serde(rename = "loaderId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub loader_id: crate::network::types::LoaderId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.preloadingAttemptSourcesUpdated/parameters/preloadingAttemptSources。
    #[serde(rename = "preloadingAttemptSources")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub preloading_attempt_sources: Vec<crate::preload::types::PreloadingAttemptSource>,
}
impl<'de> serde::Deserialize<'de> for PreloadingAttemptSourcesUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PreloadingAttemptSourcesUpdatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.preloadingAttemptSourcesUpdated/parameters/loaderId。
                    #[serde(rename = "loaderId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub loader_id: crate::network::types::LoaderId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.preloadingAttemptSourcesUpdated/parameters/preloadingAttemptSources。
                    #[serde(rename = "preloadingAttemptSources")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub preloading_attempt_sources:
                        Vec<crate::preload::types::PreloadingAttemptSource>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PreloadingAttemptSourcesUpdatedParams {
                    loader_id: fields.loader_id,
                    preloading_attempt_sources: fields.preloading_attempt_sources,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Preload.event.preloadingAttemptSourcesUpdated。
///
///协议原文：
///
///<pre>Send a list of sources for all preloading attempts in a document.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct PreloadingAttemptSourcesUpdated;
impl ydp_core::Event for PreloadingAttemptSourcesUpdated {
    type Params = PreloadingAttemptSourcesUpdatedParams;
    const METHOD: &'static str = "Preload.preloadingAttemptSourcesUpdated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Preload.event.prerenderStatusUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PrerenderStatusUpdatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.prerenderStatusUpdated/parameters/key。
    #[serde(rename = "key")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub key: crate::preload::types::PreloadingAttemptKey,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.prerenderStatusUpdated/parameters/pipelineId。
    #[serde(rename = "pipelineId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub pipeline_id: crate::preload::types::PreloadPipelineId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.prerenderStatusUpdated/parameters/status。
    #[serde(rename = "status")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub status: crate::preload::types::PreloadingStatus,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Preload.event.prerenderStatusUpdated/parameters/prerenderStatus。
    #[serde(rename = "prerenderStatus")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub prerender_status: Option<crate::preload::types::PrerenderFinalStatus>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Preload.event.prerenderStatusUpdated/parameters/disallowedMojoInterface。
    ///
    ///协议原文：
    ///
    ///<pre>This is used to give users more information about the name of Mojo interface
    ///that is incompatible with prerender and has caused the cancellation of the attempt.</pre>
    ///
    ///
    #[serde(rename = "disallowedMojoInterface")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub disallowed_mojo_interface: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Preload.event.prerenderStatusUpdated/parameters/mismatchedHeaders。
    #[serde(rename = "mismatchedHeaders")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub mismatched_headers: Option<Vec<crate::preload::types::PrerenderMismatchedHeaders>>,
}
impl<'de> serde::Deserialize<'de> for PrerenderStatusUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PrerenderStatusUpdatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.prerenderStatusUpdated/parameters/key。
                    #[serde(rename = "key")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub key: crate::preload::types::PreloadingAttemptKey,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.prerenderStatusUpdated/parameters/pipelineId。
                    #[serde(rename = "pipelineId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub pipeline_id: crate::preload::types::PreloadPipelineId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.prerenderStatusUpdated/parameters/status。
                    #[serde(rename = "status")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub status: crate::preload::types::PreloadingStatus,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Preload.event.prerenderStatusUpdated/parameters/prerenderStatus。
                    #[serde(rename = "prerenderStatus")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub prerender_status: Option<crate::preload::types::PrerenderFinalStatus>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Preload.event.prerenderStatusUpdated/parameters/disallowedMojoInterface。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>This is used to give users more information about the name of Mojo interface
                    ///that is incompatible with prerender and has caused the cancellation of the attempt.</pre>
                    ///
                    ///
                    #[serde(rename = "disallowedMojoInterface")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub disallowed_mojo_interface: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Preload.event.prerenderStatusUpdated/parameters/mismatchedHeaders。
                    #[serde(rename = "mismatchedHeaders")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub mismatched_headers:
                        Option<Vec<crate::preload::types::PrerenderMismatchedHeaders>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PrerenderStatusUpdatedParams {
                    key: fields.key,
                    pipeline_id: fields.pipeline_id,
                    status: fields.status,
                    prerender_status: fields.prerender_status,
                    disallowed_mojo_interface: fields.disallowed_mojo_interface,
                    mismatched_headers: fields.mismatched_headers,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Preload.event.prerenderStatusUpdated。
///
///协议原文：
///
///<pre>Fired when a prerender attempt is updated.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct PrerenderStatusUpdated;
impl ydp_core::Event for PrerenderStatusUpdated {
    type Params = PrerenderStatusUpdatedParams;
    const METHOD: &'static str = "Preload.prerenderStatusUpdated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Preload.event.ruleSetRemoved/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RuleSetRemovedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.ruleSetRemoved/parameters/id。
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: crate::preload::types::RuleSetId,
}
impl<'de> serde::Deserialize<'de> for RuleSetRemovedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RuleSetRemovedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.ruleSetRemoved/parameters/id。
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: crate::preload::types::RuleSetId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RuleSetRemovedParams { id: fields.id })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Preload.event.ruleSetRemoved。
#[derive(Debug, Clone, Copy)]
pub struct RuleSetRemoved;
impl ydp_core::Event for RuleSetRemoved {
    type Params = RuleSetRemovedParams;
    const METHOD: &'static str = "Preload.ruleSetRemoved";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Preload.event.ruleSetUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RuleSetUpdatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.ruleSetUpdated/parameters/ruleSet。
    #[serde(rename = "ruleSet")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub rule_set: crate::preload::types::RuleSet,
}
impl<'de> serde::Deserialize<'de> for RuleSetUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RuleSetUpdatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Preload.event.ruleSetUpdated/parameters/ruleSet。
                    #[serde(rename = "ruleSet")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub rule_set: crate::preload::types::RuleSet,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RuleSetUpdatedParams {
                    rule_set: fields.rule_set,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Preload.event.ruleSetUpdated。
///
///协议原文：
///
///<pre>Upsert. Currently, it is only emitted when a rule set added.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RuleSetUpdated;
impl ydp_core::Event for RuleSetUpdated {
    type Params = RuleSetUpdatedParams;
    const METHOD: &'static str = "Preload.ruleSetUpdated";
}
