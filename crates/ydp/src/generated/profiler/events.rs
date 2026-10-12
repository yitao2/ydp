///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Profiler.event.consoleProfileFinished/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ConsoleProfileFinishedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.event.consoleProfileFinished/parameters/id。
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.event.consoleProfileFinished/parameters/location。
    ///
    ///协议原文：
    ///
    ///<pre>Location of console.profileEnd().</pre>
    ///
    ///
    #[serde(rename = "location")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub location: crate::debugger::types::Location,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.event.consoleProfileFinished/parameters/profile。
    #[serde(rename = "profile")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub profile: crate::profiler::types::Profile,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Profiler.event.consoleProfileFinished/parameters/title。
    ///
    ///协议原文：
    ///
    ///<pre>Profile title passed as an argument to console.profile().</pre>
    ///
    ///
    #[serde(rename = "title")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<String>,
}
impl<'de> serde::Deserialize<'de> for ConsoleProfileFinishedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ConsoleProfileFinishedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.event.consoleProfileFinished/parameters/id。
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.event.consoleProfileFinished/parameters/location。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Location of console.profileEnd().</pre>
                    ///
                    ///
                    #[serde(rename = "location")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub location: crate::debugger::types::Location,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.event.consoleProfileFinished/parameters/profile。
                    #[serde(rename = "profile")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub profile: crate::profiler::types::Profile,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Profiler.event.consoleProfileFinished/parameters/title。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Profile title passed as an argument to console.profile().</pre>
                    ///
                    ///
                    #[serde(rename = "title")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub title: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ConsoleProfileFinishedParams {
                    id: fields.id,
                    location: fields.location,
                    profile: fields.profile,
                    title: fields.title,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Profiler.event.consoleProfileFinished。
#[derive(Debug, Clone, Copy)]
pub struct ConsoleProfileFinished;
impl ydp_core::Event for ConsoleProfileFinished {
    type Params = ConsoleProfileFinishedParams;
    const METHOD: &'static str = "Profiler.consoleProfileFinished";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Profiler.event.consoleProfileStarted/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ConsoleProfileStartedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.event.consoleProfileStarted/parameters/id。
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.event.consoleProfileStarted/parameters/location。
    ///
    ///协议原文：
    ///
    ///<pre>Location of console.profile().</pre>
    ///
    ///
    #[serde(rename = "location")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub location: crate::debugger::types::Location,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Profiler.event.consoleProfileStarted/parameters/title。
    ///
    ///协议原文：
    ///
    ///<pre>Profile title passed as an argument to console.profile().</pre>
    ///
    ///
    #[serde(rename = "title")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<String>,
}
impl<'de> serde::Deserialize<'de> for ConsoleProfileStartedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ConsoleProfileStartedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.event.consoleProfileStarted/parameters/id。
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.event.consoleProfileStarted/parameters/location。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Location of console.profile().</pre>
                    ///
                    ///
                    #[serde(rename = "location")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub location: crate::debugger::types::Location,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Profiler.event.consoleProfileStarted/parameters/title。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Profile title passed as an argument to console.profile().</pre>
                    ///
                    ///
                    #[serde(rename = "title")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub title: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ConsoleProfileStartedParams {
                    id: fields.id,
                    location: fields.location,
                    title: fields.title,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Profiler.event.consoleProfileStarted。
///
///协议原文：
///
///<pre>Sent when new profile recording is started using console.profile() call.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ConsoleProfileStarted;
impl ydp_core::Event for ConsoleProfileStarted {
    type Params = ConsoleProfileStartedParams;
    const METHOD: &'static str = "Profiler.consoleProfileStarted";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Profiler.event.preciseCoverageDeltaUpdate/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PreciseCoverageDeltaUpdateParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.event.preciseCoverageDeltaUpdate/parameters/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Monotonically increasing time (in seconds) when the coverage update was taken in the backend.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.event.preciseCoverageDeltaUpdate/parameters/occasion。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier for distinguishing coverage events.</pre>
    ///
    ///
    #[serde(rename = "occasion")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub occasion: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.event.preciseCoverageDeltaUpdate/parameters/result。
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
impl<'de> serde::Deserialize<'de> for PreciseCoverageDeltaUpdateParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PreciseCoverageDeltaUpdateParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.event.preciseCoverageDeltaUpdate/parameters/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Monotonically increasing time (in seconds) when the coverage update was taken in the backend.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.event.preciseCoverageDeltaUpdate/parameters/occasion。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier for distinguishing coverage events.</pre>
                    ///
                    ///
                    #[serde(rename = "occasion")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub occasion: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.event.preciseCoverageDeltaUpdate/parameters/result。
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
                Ok(PreciseCoverageDeltaUpdateParams {
                    timestamp: fields.timestamp,
                    occasion: fields.occasion,
                    result: fields.result,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Profiler.event.preciseCoverageDeltaUpdate。
///
///协议原文：
///
///<pre>Reports coverage delta since the last poll (either from an event like this, or from
///`takePreciseCoverage` for the current isolate. May only be sent if precise code
///coverage has been started. This event can be trigged by the embedder to, for example,
///trigger collection of coverage data immediately at a certain point in time.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct PreciseCoverageDeltaUpdate;
impl ydp_core::Event for PreciseCoverageDeltaUpdate {
    type Params = PreciseCoverageDeltaUpdateParams;
    const METHOD: &'static str = "Profiler.preciseCoverageDeltaUpdate";
}
