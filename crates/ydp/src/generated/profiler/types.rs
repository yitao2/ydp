///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Profiler.type.CoverageRange。
///
///协议原文：
///
///<pre>Coverage data for a source range.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoverageRange {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.CoverageRange/properties/startOffset。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript script source offset for the range start.</pre>
    ///
    ///
    #[serde(rename = "startOffset")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub start_offset: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.CoverageRange/properties/endOffset。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript script source offset for the range end.</pre>
    ///
    ///
    #[serde(rename = "endOffset")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub end_offset: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.CoverageRange/properties/count。
    ///
    ///协议原文：
    ///
    ///<pre>Collected execution count of the source range.</pre>
    ///
    ///
    #[serde(rename = "count")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub count: i64,
}
impl<'de> serde::Deserialize<'de> for CoverageRange {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CoverageRange;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.CoverageRange/properties/startOffset。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript script source offset for the range start.</pre>
                    ///
                    ///
                    #[serde(rename = "startOffset")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub start_offset: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.CoverageRange/properties/endOffset。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript script source offset for the range end.</pre>
                    ///
                    ///
                    #[serde(rename = "endOffset")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub end_offset: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.CoverageRange/properties/count。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Collected execution count of the source range.</pre>
                    ///
                    ///
                    #[serde(rename = "count")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub count: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CoverageRange {
                    start_offset: fields.start_offset,
                    end_offset: fields.end_offset,
                    count: fields.count,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Profiler.type.FunctionCoverage。
///
///协议原文：
///
///<pre>Coverage data for a JavaScript function.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct FunctionCoverage {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.FunctionCoverage/properties/functionName。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript function name.</pre>
    ///
    ///
    #[serde(rename = "functionName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub function_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.FunctionCoverage/properties/ranges。
    ///
    ///协议原文：
    ///
    ///<pre>Source ranges inside the function with coverage data.</pre>
    ///
    ///
    #[serde(rename = "ranges")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub ranges: Vec<crate::profiler::types::CoverageRange>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.FunctionCoverage/properties/isBlockCoverage。
    ///
    ///协议原文：
    ///
    ///<pre>Whether coverage data for this function has block granularity.</pre>
    ///
    ///
    #[serde(rename = "isBlockCoverage")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub is_block_coverage: bool,
}
impl<'de> serde::Deserialize<'de> for FunctionCoverage {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FunctionCoverage;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.FunctionCoverage/properties/functionName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript function name.</pre>
                    ///
                    ///
                    #[serde(rename = "functionName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub function_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.FunctionCoverage/properties/ranges。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Source ranges inside the function with coverage data.</pre>
                    ///
                    ///
                    #[serde(rename = "ranges")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub ranges: Vec<crate::profiler::types::CoverageRange>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.FunctionCoverage/properties/isBlockCoverage。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether coverage data for this function has block granularity.</pre>
                    ///
                    ///
                    #[serde(rename = "isBlockCoverage")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub is_block_coverage: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FunctionCoverage {
                    function_name: fields.function_name,
                    ranges: fields.ranges,
                    is_block_coverage: fields.is_block_coverage,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Profiler.type.PositionTickInfo。
///
///协议原文：
///
///<pre>Specifies a number of samples attributed to a certain source position.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct PositionTickInfo {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.PositionTickInfo/properties/line。
    ///
    ///协议原文：
    ///
    ///<pre>Source line number (1-based).</pre>
    ///
    ///
    #[serde(rename = "line")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub line: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.PositionTickInfo/properties/ticks。
    ///
    ///协议原文：
    ///
    ///<pre>Number of samples attributed to the source line.</pre>
    ///
    ///
    #[serde(rename = "ticks")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub ticks: i64,
}
impl<'de> serde::Deserialize<'de> for PositionTickInfo {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PositionTickInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.PositionTickInfo/properties/line。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Source line number (1-based).</pre>
                    ///
                    ///
                    #[serde(rename = "line")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub line: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.PositionTickInfo/properties/ticks。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Number of samples attributed to the source line.</pre>
                    ///
                    ///
                    #[serde(rename = "ticks")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub ticks: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PositionTickInfo {
                    line: fields.line,
                    ticks: fields.ticks,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Profiler.type.Profile。
///
///协议原文：
///
///<pre>Profile.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct Profile {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.Profile/properties/nodes。
    ///
    ///协议原文：
    ///
    ///<pre>The list of profile nodes. First item is the root node.</pre>
    ///
    ///
    #[serde(rename = "nodes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub nodes: Vec<crate::profiler::types::ProfileNode>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.Profile/properties/startTime。
    ///
    ///协议原文：
    ///
    ///<pre>Profiling start timestamp in microseconds.</pre>
    ///
    ///
    #[serde(rename = "startTime")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub start_time: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.Profile/properties/endTime。
    ///
    ///协议原文：
    ///
    ///<pre>Profiling end timestamp in microseconds.</pre>
    ///
    ///
    #[serde(rename = "endTime")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub end_time: ydp_core::FiniteF64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Profiler.type.Profile/properties/samples。
    ///
    ///协议原文：
    ///
    ///<pre>Ids of samples top nodes.</pre>
    ///
    ///
    #[serde(rename = "samples")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub samples: Option<Vec<i64>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Profiler.type.Profile/properties/timeDeltas。
    ///
    ///协议原文：
    ///
    ///<pre>Time intervals between adjacent samples in microseconds. The first delta is relative to the
    ///profile startTime.</pre>
    ///
    ///
    #[serde(rename = "timeDeltas")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub time_deltas: Option<Vec<i64>>,
}
impl<'de> serde::Deserialize<'de> for Profile {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Profile;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.Profile/properties/nodes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The list of profile nodes. First item is the root node.</pre>
                    ///
                    ///
                    #[serde(rename = "nodes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub nodes: Vec<crate::profiler::types::ProfileNode>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.Profile/properties/startTime。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Profiling start timestamp in microseconds.</pre>
                    ///
                    ///
                    #[serde(rename = "startTime")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub start_time: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.Profile/properties/endTime。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Profiling end timestamp in microseconds.</pre>
                    ///
                    ///
                    #[serde(rename = "endTime")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub end_time: ydp_core::FiniteF64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Profiler.type.Profile/properties/samples。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Ids of samples top nodes.</pre>
                    ///
                    ///
                    #[serde(rename = "samples")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub samples: Option<Vec<i64>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Profiler.type.Profile/properties/timeDeltas。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Time intervals between adjacent samples in microseconds. The first delta is relative to the
                    ///profile startTime.</pre>
                    ///
                    ///
                    #[serde(rename = "timeDeltas")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub time_deltas: Option<Vec<i64>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Profile {
                    nodes: fields.nodes,
                    start_time: fields.start_time,
                    end_time: fields.end_time,
                    samples: fields.samples,
                    time_deltas: fields.time_deltas,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Profiler.type.ProfileNode。
///
///协议原文：
///
///<pre>Profile node. Holds callsite information, execution statistics and child nodes.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProfileNode {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.ProfileNode/properties/id。
    ///
    ///协议原文：
    ///
    ///<pre>Unique id of the node.</pre>
    ///
    ///
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.ProfileNode/properties/callFrame。
    ///
    ///协议原文：
    ///
    ///<pre>Function location.</pre>
    ///
    ///
    #[serde(rename = "callFrame")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub call_frame: crate::runtime::types::CallFrame,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Profiler.type.ProfileNode/properties/hitCount。
    ///
    ///协议原文：
    ///
    ///<pre>Number of samples where this node was on top of the call stack.</pre>
    ///
    ///
    #[serde(rename = "hitCount")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub hit_count: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Profiler.type.ProfileNode/properties/children。
    ///
    ///协议原文：
    ///
    ///<pre>Child node ids.</pre>
    ///
    ///
    #[serde(rename = "children")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub children: Option<Vec<i64>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Profiler.type.ProfileNode/properties/deoptReason。
    ///
    ///协议原文：
    ///
    ///<pre>The reason of being not optimized. The function may be deoptimized or marked as don't
    ///optimize.</pre>
    ///
    ///
    #[serde(rename = "deoptReason")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub deopt_reason: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Profiler.type.ProfileNode/properties/positionTicks。
    ///
    ///协议原文：
    ///
    ///<pre>An array of source position ticks.</pre>
    ///
    ///
    #[serde(rename = "positionTicks")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub position_ticks: Option<Vec<crate::profiler::types::PositionTickInfo>>,
}
impl<'de> serde::Deserialize<'de> for ProfileNode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ProfileNode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.ProfileNode/properties/id。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Unique id of the node.</pre>
                    ///
                    ///
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.ProfileNode/properties/callFrame。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Function location.</pre>
                    ///
                    ///
                    #[serde(rename = "callFrame")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub call_frame: crate::runtime::types::CallFrame,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Profiler.type.ProfileNode/properties/hitCount。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Number of samples where this node was on top of the call stack.</pre>
                    ///
                    ///
                    #[serde(rename = "hitCount")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub hit_count: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Profiler.type.ProfileNode/properties/children。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Child node ids.</pre>
                    ///
                    ///
                    #[serde(rename = "children")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub children: Option<Vec<i64>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Profiler.type.ProfileNode/properties/deoptReason。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The reason of being not optimized. The function may be deoptimized or marked as don't
                    ///optimize.</pre>
                    ///
                    ///
                    #[serde(rename = "deoptReason")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub deopt_reason: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Profiler.type.ProfileNode/properties/positionTicks。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An array of source position ticks.</pre>
                    ///
                    ///
                    #[serde(rename = "positionTicks")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub position_ticks: Option<Vec<crate::profiler::types::PositionTickInfo>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ProfileNode {
                    id: fields.id,
                    call_frame: fields.call_frame,
                    hit_count: fields.hit_count,
                    children: fields.children,
                    deopt_reason: fields.deopt_reason,
                    position_ticks: fields.position_ticks,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Profiler.type.ScriptCoverage。
///
///协议原文：
///
///<pre>Coverage data for a JavaScript script.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ScriptCoverage {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.ScriptCoverage/properties/scriptId。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript script id.</pre>
    ///
    ///
    #[serde(rename = "scriptId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub script_id: crate::runtime::types::ScriptId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.ScriptCoverage/properties/url。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript script name or url.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.ScriptCoverage/properties/functions。
    ///
    ///协议原文：
    ///
    ///<pre>Functions contained in the script that has coverage data.</pre>
    ///
    ///
    #[serde(rename = "functions")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub functions: Vec<crate::profiler::types::FunctionCoverage>,
}
impl<'de> serde::Deserialize<'de> for ScriptCoverage {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScriptCoverage;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.ScriptCoverage/properties/scriptId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript script id.</pre>
                    ///
                    ///
                    #[serde(rename = "scriptId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub script_id: crate::runtime::types::ScriptId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.ScriptCoverage/properties/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript script name or url.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Profiler.type.ScriptCoverage/properties/functions。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Functions contained in the script that has coverage data.</pre>
                    ///
                    ///
                    #[serde(rename = "functions")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub functions: Vec<crate::profiler::types::FunctionCoverage>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ScriptCoverage {
                    script_id: fields.script_id,
                    url: fields.url,
                    functions: fields.functions,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
