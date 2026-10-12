///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Profiler.command.disable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Profiler.command.disable。
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Profiler.command.disable/returns。
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
    const METHOD: &'static str = "Profiler.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Profiler.command.enable/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct EnableParams {}
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
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EnableParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Profiler.command.enable。
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Profiler.command.enable/returns。
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
    const METHOD: &'static str = "Profiler.enable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Profiler.command.getBestEffortCoverage/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetBestEffortCoverageParams {}
impl<'de> serde::Deserialize<'de> for GetBestEffortCoverageParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetBestEffortCoverageParams;
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
                Ok(GetBestEffortCoverageParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Profiler.command.getBestEffortCoverage。
///
///协议原文：
///
///<pre>Collect coverage data for the current isolate. The coverage data may be incomplete due to
///garbage collection.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetBestEffortCoverage;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Profiler.command.getBestEffortCoverage/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetBestEffortCoverageResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.command.getBestEffortCoverage/returns/result。
    ///
    ///协议原文：
    ///
    ///<pre>Coverage data for the current isolate.</pre>
    ///
    ///
    #[serde(rename = "result")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub result: Vec<crate::profiler::types::ScriptCoverage>,
}
impl<'de> serde::Deserialize<'de> for GetBestEffortCoverageResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetBestEffortCoverageResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.command.getBestEffortCoverage/returns/result。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Coverage data for the current isolate.</pre>
                    ///
                    ///
                    #[serde(rename = "result")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub result: Vec<crate::profiler::types::ScriptCoverage>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetBestEffortCoverageResponse {
                    result: fields.result,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetBestEffortCoverage {
    type Params = GetBestEffortCoverageParams;
    type Response = GetBestEffortCoverageResponse;
    const METHOD: &'static str = "Profiler.getBestEffortCoverage";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Profiler.command.setSamplingInterval/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetSamplingIntervalParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.command.setSamplingInterval/parameters/interval。
    ///
    ///协议原文：
    ///
    ///<pre>New sampling interval in microseconds.</pre>
    ///
    ///
    #[serde(rename = "interval")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub interval: i64,
}
impl<'de> serde::Deserialize<'de> for SetSamplingIntervalParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetSamplingIntervalParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.command.setSamplingInterval/parameters/interval。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>New sampling interval in microseconds.</pre>
                    ///
                    ///
                    #[serde(rename = "interval")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub interval: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetSamplingIntervalParams {
                    interval: fields.interval,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Profiler.command.setSamplingInterval。
///
///协议原文：
///
///<pre>Changes CPU profiler sampling interval. Must be called before CPU profiles recording started.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetSamplingInterval;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Profiler.command.setSamplingInterval/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetSamplingIntervalResponse {}
impl<'de> serde::Deserialize<'de> for SetSamplingIntervalResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetSamplingIntervalResponse;
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
                Ok(SetSamplingIntervalResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetSamplingInterval {
    type Params = SetSamplingIntervalParams;
    type Response = SetSamplingIntervalResponse;
    const METHOD: &'static str = "Profiler.setSamplingInterval";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Profiler.command.start/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct StartParams {}
impl<'de> serde::Deserialize<'de> for StartParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StartParams;
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
                Ok(StartParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Profiler.command.start。
#[derive(Debug, Clone, Copy)]
pub struct Start;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Profiler.command.start/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StartResponse {}
impl<'de> serde::Deserialize<'de> for StartResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StartResponse;
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
                Ok(StartResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Start {
    type Params = StartParams;
    type Response = StartResponse;
    const METHOD: &'static str = "Profiler.start";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Profiler.command.startPreciseCoverage/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct StartPreciseCoverageParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Profiler.command.startPreciseCoverage/parameters/callCount。
    ///
    ///协议原文：
    ///
    ///<pre>Collect accurate call counts beyond simple 'covered' or 'not covered'.</pre>
    ///
    ///
    #[serde(rename = "callCount")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub call_count: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Profiler.command.startPreciseCoverage/parameters/detailed。
    ///
    ///协议原文：
    ///
    ///<pre>Collect block-based coverage.</pre>
    ///
    ///
    #[serde(rename = "detailed")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub detailed: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Profiler.command.startPreciseCoverage/parameters/allowTriggeredUpdates。
    ///
    ///协议原文：
    ///
    ///<pre>Allow the backend to send updates on its own initiative</pre>
    ///
    ///
    #[serde(rename = "allowTriggeredUpdates")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_triggered_updates: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for StartPreciseCoverageParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StartPreciseCoverageParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Profiler.command.startPreciseCoverage/parameters/callCount。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Collect accurate call counts beyond simple 'covered' or 'not covered'.</pre>
                    ///
                    ///
                    #[serde(rename = "callCount")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub call_count: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Profiler.command.startPreciseCoverage/parameters/detailed。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Collect block-based coverage.</pre>
                    ///
                    ///
                    #[serde(rename = "detailed")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub detailed: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Profiler.command.startPreciseCoverage/parameters/allowTriggeredUpdates。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Allow the backend to send updates on its own initiative</pre>
                    ///
                    ///
                    #[serde(rename = "allowTriggeredUpdates")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub allow_triggered_updates: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StartPreciseCoverageParams {
                    call_count: fields.call_count,
                    detailed: fields.detailed,
                    allow_triggered_updates: fields.allow_triggered_updates,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Profiler.command.startPreciseCoverage。
///
///协议原文：
///
///<pre>Enable precise code coverage. Coverage data for JavaScript executed before enabling precise code
///coverage may be incomplete. Enabling prevents running optimized code and resets execution
///counters.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct StartPreciseCoverage;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Profiler.command.startPreciseCoverage/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StartPreciseCoverageResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.command.startPreciseCoverage/returns/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Monotonically increasing time (in seconds) when the coverage update was taken in the backend.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for StartPreciseCoverageResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StartPreciseCoverageResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.command.startPreciseCoverage/returns/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Monotonically increasing time (in seconds) when the coverage update was taken in the backend.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StartPreciseCoverageResponse {
                    timestamp: fields.timestamp,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for StartPreciseCoverage {
    type Params = StartPreciseCoverageParams;
    type Response = StartPreciseCoverageResponse;
    const METHOD: &'static str = "Profiler.startPreciseCoverage";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Profiler.command.stop/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct StopParams {}
impl<'de> serde::Deserialize<'de> for StopParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StopParams;
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
                Ok(StopParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Profiler.command.stop。
#[derive(Debug, Clone, Copy)]
pub struct Stop;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Profiler.command.stop/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StopResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.command.stop/returns/profile。
    ///
    ///协议原文：
    ///
    ///<pre>Recorded profile.</pre>
    ///
    ///
    #[serde(rename = "profile")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub profile: crate::profiler::types::Profile,
}
impl<'de> serde::Deserialize<'de> for StopResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StopResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.command.stop/returns/profile。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Recorded profile.</pre>
                    ///
                    ///
                    #[serde(rename = "profile")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub profile: crate::profiler::types::Profile,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StopResponse {
                    profile: fields.profile,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Stop {
    type Params = StopParams;
    type Response = StopResponse;
    const METHOD: &'static str = "Profiler.stop";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Profiler.command.stopPreciseCoverage/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct StopPreciseCoverageParams {}
impl<'de> serde::Deserialize<'de> for StopPreciseCoverageParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StopPreciseCoverageParams;
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
                Ok(StopPreciseCoverageParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Profiler.command.stopPreciseCoverage。
///
///协议原文：
///
///<pre>Disable precise code coverage. Disabling releases unnecessary execution count records and allows
///executing optimized code.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct StopPreciseCoverage;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Profiler.command.stopPreciseCoverage/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StopPreciseCoverageResponse {}
impl<'de> serde::Deserialize<'de> for StopPreciseCoverageResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StopPreciseCoverageResponse;
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
                Ok(StopPreciseCoverageResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for StopPreciseCoverage {
    type Params = StopPreciseCoverageParams;
    type Response = StopPreciseCoverageResponse;
    const METHOD: &'static str = "Profiler.stopPreciseCoverage";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Profiler.command.takePreciseCoverage/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct TakePreciseCoverageParams {}
impl<'de> serde::Deserialize<'de> for TakePreciseCoverageParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TakePreciseCoverageParams;
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
                Ok(TakePreciseCoverageParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Profiler.command.takePreciseCoverage。
///
///协议原文：
///
///<pre>Collect coverage data for the current isolate, and resets execution counters. Precise code
///coverage needs to have started.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct TakePreciseCoverage;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Profiler.command.takePreciseCoverage/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TakePreciseCoverageResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.command.takePreciseCoverage/returns/result。
    ///
    ///协议原文：
    ///
    ///<pre>Coverage data for the current isolate.</pre>
    ///
    ///
    #[serde(rename = "result")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub result: Vec<crate::profiler::types::ScriptCoverage>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.command.takePreciseCoverage/returns/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Monotonically increasing time (in seconds) when the coverage update was taken in the backend.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for TakePreciseCoverageResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TakePreciseCoverageResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.command.takePreciseCoverage/returns/result。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Coverage data for the current isolate.</pre>
                    ///
                    ///
                    #[serde(rename = "result")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub result: Vec<crate::profiler::types::ScriptCoverage>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.command.takePreciseCoverage/returns/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Monotonically increasing time (in seconds) when the coverage update was taken in the backend.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TakePreciseCoverageResponse {
                    result: fields.result,
                    timestamp: fields.timestamp,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for TakePreciseCoverage {
    type Params = TakePreciseCoverageParams;
    type Response = TakePreciseCoverageResponse;
    const METHOD: &'static str = "Profiler.takePreciseCoverage";
}
