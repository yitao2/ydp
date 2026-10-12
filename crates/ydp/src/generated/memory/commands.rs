///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Memory.command.forciblyPurgeJavaScriptMemory/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ForciblyPurgeJavaScriptMemoryParams {}
impl<'de> serde::Deserialize<'de> for ForciblyPurgeJavaScriptMemoryParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ForciblyPurgeJavaScriptMemoryParams;
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
                Ok(ForciblyPurgeJavaScriptMemoryParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Memory.command.forciblyPurgeJavaScriptMemory。
///
///协议原文：
///
///<pre>Simulate OomIntervention by purging V8 memory.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ForciblyPurgeJavaScriptMemory;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Memory.command.forciblyPurgeJavaScriptMemory/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ForciblyPurgeJavaScriptMemoryResponse {}
impl<'de> serde::Deserialize<'de> for ForciblyPurgeJavaScriptMemoryResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ForciblyPurgeJavaScriptMemoryResponse;
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
                Ok(ForciblyPurgeJavaScriptMemoryResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ForciblyPurgeJavaScriptMemory {
    type Params = ForciblyPurgeJavaScriptMemoryParams;
    type Response = ForciblyPurgeJavaScriptMemoryResponse;
    const METHOD: &'static str = "Memory.forciblyPurgeJavaScriptMemory";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Memory.command.getAllTimeSamplingProfile/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetAllTimeSamplingProfileParams {}
impl<'de> serde::Deserialize<'de> for GetAllTimeSamplingProfileParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetAllTimeSamplingProfileParams;
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
                Ok(GetAllTimeSamplingProfileParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Memory.command.getAllTimeSamplingProfile。
///
///协议原文：
///
///<pre>Retrieve native memory allocations profile
///collected since renderer process startup.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetAllTimeSamplingProfile;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Memory.command.getAllTimeSamplingProfile/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetAllTimeSamplingProfileResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.command.getAllTimeSamplingProfile/returns/profile。
    #[serde(rename = "profile")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub profile: crate::memory::types::SamplingProfile,
}
impl<'de> serde::Deserialize<'de> for GetAllTimeSamplingProfileResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetAllTimeSamplingProfileResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.command.getAllTimeSamplingProfile/returns/profile。
                    #[serde(rename = "profile")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub profile: crate::memory::types::SamplingProfile,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetAllTimeSamplingProfileResponse {
                    profile: fields.profile,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetAllTimeSamplingProfile {
    type Params = GetAllTimeSamplingProfileParams;
    type Response = GetAllTimeSamplingProfileResponse;
    const METHOD: &'static str = "Memory.getAllTimeSamplingProfile";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Memory.command.getBrowserSamplingProfile/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetBrowserSamplingProfileParams {}
impl<'de> serde::Deserialize<'de> for GetBrowserSamplingProfileParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetBrowserSamplingProfileParams;
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
                Ok(GetBrowserSamplingProfileParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Memory.command.getBrowserSamplingProfile。
///
///协议原文：
///
///<pre>Retrieve native memory allocations profile
///collected since browser process startup.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetBrowserSamplingProfile;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Memory.command.getBrowserSamplingProfile/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetBrowserSamplingProfileResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.command.getBrowserSamplingProfile/returns/profile。
    #[serde(rename = "profile")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub profile: crate::memory::types::SamplingProfile,
}
impl<'de> serde::Deserialize<'de> for GetBrowserSamplingProfileResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetBrowserSamplingProfileResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.command.getBrowserSamplingProfile/returns/profile。
                    #[serde(rename = "profile")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub profile: crate::memory::types::SamplingProfile,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetBrowserSamplingProfileResponse {
                    profile: fields.profile,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetBrowserSamplingProfile {
    type Params = GetBrowserSamplingProfileParams;
    type Response = GetBrowserSamplingProfileResponse;
    const METHOD: &'static str = "Memory.getBrowserSamplingProfile";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Memory.command.getDOMCounters/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetDomCountersParams {}
impl<'de> serde::Deserialize<'de> for GetDomCountersParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetDomCountersParams;
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
                Ok(GetDomCountersParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Memory.command.getDOMCounters。
///
///协议原文：
///
///<pre>Retruns current DOM object counters.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetDomCounters;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Memory.command.getDOMCounters/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetDomCountersResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.command.getDOMCounters/returns/documents。
    #[serde(rename = "documents")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub documents: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.command.getDOMCounters/returns/nodes。
    #[serde(rename = "nodes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub nodes: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.command.getDOMCounters/returns/jsEventListeners。
    #[serde(rename = "jsEventListeners")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub js_event_listeners: i64,
}
impl<'de> serde::Deserialize<'de> for GetDomCountersResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetDomCountersResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.command.getDOMCounters/returns/documents。
                    #[serde(rename = "documents")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub documents: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.command.getDOMCounters/returns/nodes。
                    #[serde(rename = "nodes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub nodes: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.command.getDOMCounters/returns/jsEventListeners。
                    #[serde(rename = "jsEventListeners")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub js_event_listeners: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetDomCountersResponse {
                    documents: fields.documents,
                    nodes: fields.nodes,
                    js_event_listeners: fields.js_event_listeners,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetDomCounters {
    type Params = GetDomCountersParams;
    type Response = GetDomCountersResponse;
    const METHOD: &'static str = "Memory.getDOMCounters";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Memory.command.getDOMCountersForLeakDetection/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetDomCountersForLeakDetectionParams {}
impl<'de> serde::Deserialize<'de> for GetDomCountersForLeakDetectionParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetDomCountersForLeakDetectionParams;
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
                Ok(GetDomCountersForLeakDetectionParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Memory.command.getDOMCountersForLeakDetection。
///
///协议原文：
///
///<pre>Retruns DOM object counters after preparing renderer for leak detection.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetDomCountersForLeakDetection;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Memory.command.getDOMCountersForLeakDetection/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetDomCountersForLeakDetectionResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.command.getDOMCountersForLeakDetection/returns/counters。
    ///
    ///协议原文：
    ///
    ///<pre>DOM object counters.</pre>
    ///
    ///
    #[serde(rename = "counters")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub counters: Vec<crate::memory::types::DomCounter>,
}
impl<'de> serde::Deserialize<'de> for GetDomCountersForLeakDetectionResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetDomCountersForLeakDetectionResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.command.getDOMCountersForLeakDetection/returns/counters。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>DOM object counters.</pre>
                    ///
                    ///
                    #[serde(rename = "counters")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub counters: Vec<crate::memory::types::DomCounter>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetDomCountersForLeakDetectionResponse {
                    counters: fields.counters,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetDomCountersForLeakDetection {
    type Params = GetDomCountersForLeakDetectionParams;
    type Response = GetDomCountersForLeakDetectionResponse;
    const METHOD: &'static str = "Memory.getDOMCountersForLeakDetection";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Memory.command.getSamplingProfile/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetSamplingProfileParams {}
impl<'de> serde::Deserialize<'de> for GetSamplingProfileParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetSamplingProfileParams;
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
                Ok(GetSamplingProfileParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Memory.command.getSamplingProfile。
///
///协议原文：
///
///<pre>Retrieve native memory allocations profile collected since last
///`startSampling` call.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetSamplingProfile;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Memory.command.getSamplingProfile/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetSamplingProfileResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.command.getSamplingProfile/returns/profile。
    #[serde(rename = "profile")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub profile: crate::memory::types::SamplingProfile,
}
impl<'de> serde::Deserialize<'de> for GetSamplingProfileResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetSamplingProfileResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.command.getSamplingProfile/returns/profile。
                    #[serde(rename = "profile")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub profile: crate::memory::types::SamplingProfile,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetSamplingProfileResponse {
                    profile: fields.profile,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetSamplingProfile {
    type Params = GetSamplingProfileParams;
    type Response = GetSamplingProfileResponse;
    const METHOD: &'static str = "Memory.getSamplingProfile";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Memory.command.prepareForLeakDetection/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct PrepareForLeakDetectionParams {}
impl<'de> serde::Deserialize<'de> for PrepareForLeakDetectionParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PrepareForLeakDetectionParams;
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
                Ok(PrepareForLeakDetectionParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Memory.command.prepareForLeakDetection。
///
///协议原文：
///
///<pre>Prepares for leak detection by terminating workers, stopping spellcheckers,
///dropping non-essential internal caches, running garbage collections, etc.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct PrepareForLeakDetection;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Memory.command.prepareForLeakDetection/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PrepareForLeakDetectionResponse {}
impl<'de> serde::Deserialize<'de> for PrepareForLeakDetectionResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PrepareForLeakDetectionResponse;
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
                Ok(PrepareForLeakDetectionResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for PrepareForLeakDetection {
    type Params = PrepareForLeakDetectionParams;
    type Response = PrepareForLeakDetectionResponse;
    const METHOD: &'static str = "Memory.prepareForLeakDetection";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Memory.command.setPressureNotificationsSuppressed/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetPressureNotificationsSuppressedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.command.setPressureNotificationsSuppressed/parameters/suppressed。
    ///
    ///协议原文：
    ///
    ///<pre>If true, memory pressure notifications will be suppressed.</pre>
    ///
    ///
    #[serde(rename = "suppressed")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub suppressed: bool,
}
impl<'de> serde::Deserialize<'de> for SetPressureNotificationsSuppressedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetPressureNotificationsSuppressedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.command.setPressureNotificationsSuppressed/parameters/suppressed。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, memory pressure notifications will be suppressed.</pre>
                    ///
                    ///
                    #[serde(rename = "suppressed")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub suppressed: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetPressureNotificationsSuppressedParams {
                    suppressed: fields.suppressed,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Memory.command.setPressureNotificationsSuppressed。
///
///协议原文：
///
///<pre>Enable/disable suppressing memory pressure notifications in all processes.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetPressureNotificationsSuppressed;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Memory.command.setPressureNotificationsSuppressed/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetPressureNotificationsSuppressedResponse {}
impl<'de> serde::Deserialize<'de> for SetPressureNotificationsSuppressedResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetPressureNotificationsSuppressedResponse;
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
                Ok(SetPressureNotificationsSuppressedResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetPressureNotificationsSuppressed {
    type Params = SetPressureNotificationsSuppressedParams;
    type Response = SetPressureNotificationsSuppressedResponse;
    const METHOD: &'static str = "Memory.setPressureNotificationsSuppressed";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Memory.command.simulatePressureNotification/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SimulatePressureNotificationParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.command.simulatePressureNotification/parameters/level。
    ///
    ///协议原文：
    ///
    ///<pre>Memory pressure level of the notification.</pre>
    ///
    ///
    #[serde(rename = "level")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub level: crate::memory::types::PressureLevel,
}
impl<'de> serde::Deserialize<'de> for SimulatePressureNotificationParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SimulatePressureNotificationParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.command.simulatePressureNotification/parameters/level。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Memory pressure level of the notification.</pre>
                    ///
                    ///
                    #[serde(rename = "level")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub level: crate::memory::types::PressureLevel,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SimulatePressureNotificationParams {
                    level: fields.level,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Memory.command.simulatePressureNotification。
///
///协议原文：
///
///<pre>Simulate a memory pressure notification in all processes.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SimulatePressureNotification;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Memory.command.simulatePressureNotification/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SimulatePressureNotificationResponse {}
impl<'de> serde::Deserialize<'de> for SimulatePressureNotificationResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SimulatePressureNotificationResponse;
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
                Ok(SimulatePressureNotificationResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SimulatePressureNotification {
    type Params = SimulatePressureNotificationParams;
    type Response = SimulatePressureNotificationResponse;
    const METHOD: &'static str = "Memory.simulatePressureNotification";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Memory.command.startSampling/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct StartSamplingParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Memory.command.startSampling/parameters/samplingInterval。
    ///
    ///协议原文：
    ///
    ///<pre>Average number of bytes between samples.</pre>
    ///
    ///
    #[serde(rename = "samplingInterval")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub sampling_interval: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Memory.command.startSampling/parameters/suppressRandomness。
    ///
    ///协议原文：
    ///
    ///<pre>Do not randomize intervals between samples.</pre>
    ///
    ///
    #[serde(rename = "suppressRandomness")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub suppress_randomness: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for StartSamplingParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StartSamplingParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Memory.command.startSampling/parameters/samplingInterval。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Average number of bytes between samples.</pre>
                    ///
                    ///
                    #[serde(rename = "samplingInterval")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub sampling_interval: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Memory.command.startSampling/parameters/suppressRandomness。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Do not randomize intervals between samples.</pre>
                    ///
                    ///
                    #[serde(rename = "suppressRandomness")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub suppress_randomness: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StartSamplingParams {
                    sampling_interval: fields.sampling_interval,
                    suppress_randomness: fields.suppress_randomness,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Memory.command.startSampling。
///
///协议原文：
///
///<pre>Start collecting native memory profile.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct StartSampling;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Memory.command.startSampling/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StartSamplingResponse {}
impl<'de> serde::Deserialize<'de> for StartSamplingResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StartSamplingResponse;
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
                Ok(StartSamplingResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for StartSampling {
    type Params = StartSamplingParams;
    type Response = StartSamplingResponse;
    const METHOD: &'static str = "Memory.startSampling";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Memory.command.stopSampling/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct StopSamplingParams {}
impl<'de> serde::Deserialize<'de> for StopSamplingParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StopSamplingParams;
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
                Ok(StopSamplingParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Memory.command.stopSampling。
///
///协议原文：
///
///<pre>Stop collecting native memory profile.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct StopSampling;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Memory.command.stopSampling/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StopSamplingResponse {}
impl<'de> serde::Deserialize<'de> for StopSamplingResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StopSamplingResponse;
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
                Ok(StopSamplingResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for StopSampling {
    type Params = StopSamplingParams;
    type Response = StopSamplingResponse;
    const METHOD: &'static str = "Memory.stopSampling";
}
