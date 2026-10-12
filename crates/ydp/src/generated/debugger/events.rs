///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.event.breakpointResolved/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct BreakpointResolvedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.breakpointResolved/parameters/breakpointId。
    ///
    ///协议原文：
    ///
    ///<pre>Breakpoint unique identifier.</pre>
    ///
    ///
    #[serde(rename = "breakpointId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub breakpoint_id: crate::debugger::types::BreakpointId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.breakpointResolved/parameters/location。
    ///
    ///协议原文：
    ///
    ///<pre>Actual breakpoint location.</pre>
    ///
    ///
    #[serde(rename = "location")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub location: crate::debugger::types::Location,
}
impl<'de> serde::Deserialize<'de> for BreakpointResolvedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BreakpointResolvedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.breakpointResolved/parameters/breakpointId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Breakpoint unique identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "breakpointId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub breakpoint_id: crate::debugger::types::BreakpointId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.breakpointResolved/parameters/location。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Actual breakpoint location.</pre>
                    ///
                    ///
                    #[serde(rename = "location")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub location: crate::debugger::types::Location,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(BreakpointResolvedParams {
                    breakpoint_id: fields.breakpoint_id,
                    location: fields.location,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.event.breakpointResolved。
///
///协议原文：
///
///<pre>Fired when breakpoint is resolved to an actual script and location.
///Deprecated in favor of `resolvedBreakpoints` in the `scriptParsed` event.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct BreakpointResolved;
impl ydp_core::Event for BreakpointResolved {
    type Params = BreakpointResolvedParams;
    const METHOD: &'static str = "Debugger.breakpointResolved";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Debugger.event.paused/parameters/reason。
///
///协议原文：
///
///<pre>Pause reason.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PausedParamsReason {
    ///协议值 "ambiguous"。
    Ambiguous,
    ///协议值 "assert"。
    Assert,
    ///协议值 "CSPViolation"。
    CspViolation,
    ///协议值 "debugCommand"。
    DebugCommand,
    ///协议值 "DOM"。
    Dom,
    ///协议值 "EventListener"。
    EventListener,
    ///协议值 "exception"。
    Exception,
    ///协议值 "instrumentation"。
    Instrumentation,
    ///协议值 "OOM"。
    Oom,
    ///协议值 "other"。
    Other,
    ///协议值 "promiseRejection"。
    PromiseRejection,
    ///协议值 "XHR"。
    Xhr,
    ///协议值 "step"。
    Step,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl PausedParamsReason {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::debugger::events::PausedParamsReason::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ambiguous => "ambiguous",
            Self::Assert => "assert",
            Self::CspViolation => "CSPViolation",
            Self::DebugCommand => "debugCommand",
            Self::Dom => "DOM",
            Self::EventListener => "EventListener",
            Self::Exception => "exception",
            Self::Instrumentation => "instrumentation",
            Self::Oom => "OOM",
            Self::Other => "other",
            Self::PromiseRejection => "promiseRejection",
            Self::Xhr => "XHR",
            Self::Step => "step",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for PausedParamsReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for PausedParamsReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ambiguous" => PausedParamsReason::Ambiguous,
            "assert" => PausedParamsReason::Assert,
            "CSPViolation" => PausedParamsReason::CspViolation,
            "debugCommand" => PausedParamsReason::DebugCommand,
            "DOM" => PausedParamsReason::Dom,
            "EventListener" => PausedParamsReason::EventListener,
            "exception" => PausedParamsReason::Exception,
            "instrumentation" => PausedParamsReason::Instrumentation,
            "OOM" => PausedParamsReason::Oom,
            "other" => PausedParamsReason::Other,
            "promiseRejection" => PausedParamsReason::PromiseRejection,
            "XHR" => PausedParamsReason::Xhr,
            "step" => PausedParamsReason::Step,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for PausedParamsReason {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PausedParamsReason {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PausedParamsReason;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "ambiguous" => PausedParamsReason::Ambiguous,
                    "assert" => PausedParamsReason::Assert,
                    "CSPViolation" => PausedParamsReason::CspViolation,
                    "debugCommand" => PausedParamsReason::DebugCommand,
                    "DOM" => PausedParamsReason::Dom,
                    "EventListener" => PausedParamsReason::EventListener,
                    "exception" => PausedParamsReason::Exception,
                    "instrumentation" => PausedParamsReason::Instrumentation,
                    "OOM" => PausedParamsReason::Oom,
                    "other" => PausedParamsReason::Other,
                    "promiseRejection" => PausedParamsReason::PromiseRejection,
                    "XHR" => PausedParamsReason::Xhr,
                    "step" => PausedParamsReason::Step,
                    _ => PausedParamsReason::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(PausedParamsReason::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.event.paused/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PausedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.paused/parameters/callFrames。
    ///
    ///协议原文：
    ///
    ///<pre>Call stack the virtual machine stopped on.</pre>
    ///
    ///
    #[serde(rename = "callFrames")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub call_frames: Vec<crate::debugger::types::CallFrame>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.paused/parameters/reason。
    ///
    ///协议原文：
    ///
    ///<pre>Pause reason.</pre>
    ///
    ///
    #[serde(rename = "reason")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub reason: PausedParamsReason,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.paused/parameters/data。
    ///
    ///协议原文：
    ///
    ///<pre>Object containing break-specific auxiliary properties.</pre>
    ///
    ///
    #[serde(rename = "data")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub data: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.paused/parameters/hitBreakpoints。
    ///
    ///协议原文：
    ///
    ///<pre>Hit breakpoints IDs</pre>
    ///
    ///
    #[serde(rename = "hitBreakpoints")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub hit_breakpoints: Option<Vec<String>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.paused/parameters/asyncStackTrace。
    ///
    ///协议原文：
    ///
    ///<pre>Async stack trace, if any.</pre>
    ///
    ///
    #[serde(rename = "asyncStackTrace")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub async_stack_trace: Option<crate::runtime::types::StackTrace>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.paused/parameters/asyncStackTraceId。
    ///
    ///协议原文：
    ///
    ///<pre>Async stack trace, if any.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "asyncStackTraceId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub async_stack_trace_id: Option<crate::runtime::types::StackTraceId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.paused/parameters/asyncCallStackTraceId。
    ///
    ///协议原文：
    ///
    ///<pre>Never present, will be removed.</pre>
    ///
    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "asyncCallStackTraceId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub async_call_stack_trace_id: Option<crate::runtime::types::StackTraceId>,
}
impl<'de> serde::Deserialize<'de> for PausedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PausedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.paused/parameters/callFrames。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Call stack the virtual machine stopped on.</pre>
                    ///
                    ///
                    #[serde(rename = "callFrames")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub call_frames: Vec<crate::debugger::types::CallFrame>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.paused/parameters/reason。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Pause reason.</pre>
                    ///
                    ///
                    #[serde(rename = "reason")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub reason: PausedParamsReason,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.paused/parameters/data。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Object containing break-specific auxiliary properties.</pre>
                    ///
                    ///
                    #[serde(rename = "data")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub data: Option<std::collections::BTreeMap<String, serde_json::Value>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.paused/parameters/hitBreakpoints。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Hit breakpoints IDs</pre>
                    ///
                    ///
                    #[serde(rename = "hitBreakpoints")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub hit_breakpoints: Option<Vec<String>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.paused/parameters/asyncStackTrace。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Async stack trace, if any.</pre>
                    ///
                    ///
                    #[serde(rename = "asyncStackTrace")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub async_stack_trace: Option<crate::runtime::types::StackTrace>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.paused/parameters/asyncStackTraceId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Async stack trace, if any.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "asyncStackTraceId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub async_stack_trace_id: Option<crate::runtime::types::StackTraceId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.paused/parameters/asyncCallStackTraceId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Never present, will be removed.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "asyncCallStackTraceId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub async_call_stack_trace_id: Option<crate::runtime::types::StackTraceId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PausedParams {
                    call_frames: fields.call_frames,
                    reason: fields.reason,
                    data: fields.data,
                    hit_breakpoints: fields.hit_breakpoints,
                    async_stack_trace: fields.async_stack_trace,
                    async_stack_trace_id: fields.async_stack_trace_id,
                    async_call_stack_trace_id: fields.async_call_stack_trace_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.event.paused。
///
///协议原文：
///
///<pre>Fired when the virtual machine stopped on breakpoint or exception or any other stop criteria.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Paused;
impl ydp_core::Event for Paused {
    type Params = PausedParams;
    const METHOD: &'static str = "Debugger.paused";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.event.resumed/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ResumedParams {}
impl<'de> serde::Deserialize<'de> for ResumedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResumedParams;
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
                Ok(ResumedParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.event.resumed。
///
///协议原文：
///
///<pre>Fired when the virtual machine resumed execution.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Resumed;
impl ydp_core::Event for Resumed {
    type Params = ResumedParams;
    const METHOD: &'static str = "Debugger.resumed";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.event.scriptFailedToParse/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ScriptFailedToParseParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptFailedToParse/parameters/scriptId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the script parsed.</pre>
    ///
    ///
    #[serde(rename = "scriptId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub script_id: crate::runtime::types::ScriptId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptFailedToParse/parameters/url。
    ///
    ///协议原文：
    ///
    ///<pre>URL or name of the script parsed (if any).</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptFailedToParse/parameters/startLine。
    ///
    ///协议原文：
    ///
    ///<pre>Line offset of the script within the resource with given URL (for script tags).</pre>
    ///
    ///
    #[serde(rename = "startLine")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub start_line: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptFailedToParse/parameters/startColumn。
    ///
    ///协议原文：
    ///
    ///<pre>Column offset of the script within the resource with given URL.</pre>
    ///
    ///
    #[serde(rename = "startColumn")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub start_column: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptFailedToParse/parameters/endLine。
    ///
    ///协议原文：
    ///
    ///<pre>Last line of the script.</pre>
    ///
    ///
    #[serde(rename = "endLine")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub end_line: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptFailedToParse/parameters/endColumn。
    ///
    ///协议原文：
    ///
    ///<pre>Length of the last line of the script.</pre>
    ///
    ///
    #[serde(rename = "endColumn")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub end_column: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptFailedToParse/parameters/executionContextId。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies script creation context.</pre>
    ///
    ///
    #[serde(rename = "executionContextId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub execution_context_id: crate::runtime::types::ExecutionContextId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptFailedToParse/parameters/hash。
    ///
    ///协议原文：
    ///
    ///<pre>Content hash of the script, SHA-256.</pre>
    ///
    ///
    #[serde(rename = "hash")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub hash: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptFailedToParse/parameters/buildId。
    ///
    ///协议原文：
    ///
    ///<pre>For Wasm modules, the content of the `build_id` custom section. For JavaScript the `debugId` magic comment.</pre>
    ///
    ///
    #[serde(rename = "buildId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub build_id: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptFailedToParse/parameters/executionContextAuxData。
    ///
    ///协议原文：
    ///
    ///<pre>Embedder-specific auxiliary data likely matching {isDefault: boolean, type: 'default'|'isolated'|'worker', frameId: string}</pre>
    ///
    ///
    #[serde(rename = "executionContextAuxData")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_context_aux_data: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptFailedToParse/parameters/sourceMapURL。
    ///
    ///协议原文：
    ///
    ///<pre>URL of source map associated with script (if any).</pre>
    ///
    ///
    #[serde(rename = "sourceMapURL")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_map_url: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptFailedToParse/parameters/hasSourceURL。
    ///
    ///协议原文：
    ///
    ///<pre>True, if this script has sourceURL.</pre>
    ///
    ///
    #[serde(rename = "hasSourceURL")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_source_url: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptFailedToParse/parameters/isModule。
    ///
    ///协议原文：
    ///
    ///<pre>True, if this script is ES6 module.</pre>
    ///
    ///
    #[serde(rename = "isModule")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_module: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptFailedToParse/parameters/length。
    ///
    ///协议原文：
    ///
    ///<pre>This script length.</pre>
    ///
    ///
    #[serde(rename = "length")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub length: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptFailedToParse/parameters/stackTrace。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript top stack frame of where the script parsed event was triggered if available.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "stackTrace")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub stack_trace: Option<crate::runtime::types::StackTrace>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptFailedToParse/parameters/codeOffset。
    ///
    ///协议原文：
    ///
    ///<pre>If the scriptLanguage is WebAssembly, the code section offset in the module.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "codeOffset")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub code_offset: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptFailedToParse/parameters/scriptLanguage。
    ///
    ///协议原文：
    ///
    ///<pre>The language of the script.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "scriptLanguage")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub script_language: Option<crate::debugger::types::ScriptLanguage>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptFailedToParse/parameters/embedderName。
    ///
    ///协议原文：
    ///
    ///<pre>The name the embedder supplied for this script.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "embedderName")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub embedder_name: Option<String>,
}
impl<'de> serde::Deserialize<'de> for ScriptFailedToParseParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScriptFailedToParseParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptFailedToParse/parameters/scriptId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the script parsed.</pre>
                    ///
                    ///
                    #[serde(rename = "scriptId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub script_id: crate::runtime::types::ScriptId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptFailedToParse/parameters/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>URL or name of the script parsed (if any).</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptFailedToParse/parameters/startLine。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Line offset of the script within the resource with given URL (for script tags).</pre>
                    ///
                    ///
                    #[serde(rename = "startLine")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub start_line: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptFailedToParse/parameters/startColumn。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Column offset of the script within the resource with given URL.</pre>
                    ///
                    ///
                    #[serde(rename = "startColumn")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub start_column: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptFailedToParse/parameters/endLine。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Last line of the script.</pre>
                    ///
                    ///
                    #[serde(rename = "endLine")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub end_line: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptFailedToParse/parameters/endColumn。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Length of the last line of the script.</pre>
                    ///
                    ///
                    #[serde(rename = "endColumn")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub end_column: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptFailedToParse/parameters/executionContextId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies script creation context.</pre>
                    ///
                    ///
                    #[serde(rename = "executionContextId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub execution_context_id: crate::runtime::types::ExecutionContextId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptFailedToParse/parameters/hash。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Content hash of the script, SHA-256.</pre>
                    ///
                    ///
                    #[serde(rename = "hash")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub hash: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptFailedToParse/parameters/buildId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>For Wasm modules, the content of the `build_id` custom section. For JavaScript the `debugId` magic comment.</pre>
                    ///
                    ///
                    #[serde(rename = "buildId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub build_id: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptFailedToParse/parameters/executionContextAuxData。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Embedder-specific auxiliary data likely matching {isDefault: boolean, type: 'default'|'isolated'|'worker', frameId: string}</pre>
                    ///
                    ///
                    #[serde(rename = "executionContextAuxData")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub execution_context_aux_data:
                        Option<std::collections::BTreeMap<String, serde_json::Value>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptFailedToParse/parameters/sourceMapURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>URL of source map associated with script (if any).</pre>
                    ///
                    ///
                    #[serde(rename = "sourceMapURL")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub source_map_url: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptFailedToParse/parameters/hasSourceURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True, if this script has sourceURL.</pre>
                    ///
                    ///
                    #[serde(rename = "hasSourceURL")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub has_source_url: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptFailedToParse/parameters/isModule。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True, if this script is ES6 module.</pre>
                    ///
                    ///
                    #[serde(rename = "isModule")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub is_module: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptFailedToParse/parameters/length。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>This script length.</pre>
                    ///
                    ///
                    #[serde(rename = "length")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub length: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptFailedToParse/parameters/stackTrace。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript top stack frame of where the script parsed event was triggered if available.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "stackTrace")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub stack_trace: Option<crate::runtime::types::StackTrace>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptFailedToParse/parameters/codeOffset。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If the scriptLanguage is WebAssembly, the code section offset in the module.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "codeOffset")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub code_offset: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptFailedToParse/parameters/scriptLanguage。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The language of the script.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "scriptLanguage")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub script_language: Option<crate::debugger::types::ScriptLanguage>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptFailedToParse/parameters/embedderName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The name the embedder supplied for this script.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "embedderName")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub embedder_name: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ScriptFailedToParseParams {
                    script_id: fields.script_id,
                    url: fields.url,
                    start_line: fields.start_line,
                    start_column: fields.start_column,
                    end_line: fields.end_line,
                    end_column: fields.end_column,
                    execution_context_id: fields.execution_context_id,
                    hash: fields.hash,
                    build_id: fields.build_id,
                    execution_context_aux_data: fields.execution_context_aux_data,
                    source_map_url: fields.source_map_url,
                    has_source_url: fields.has_source_url,
                    is_module: fields.is_module,
                    length: fields.length,
                    stack_trace: fields.stack_trace,
                    code_offset: fields.code_offset,
                    script_language: fields.script_language,
                    embedder_name: fields.embedder_name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.event.scriptFailedToParse。
///
///协议原文：
///
///<pre>Fired when virtual machine fails to parse the script.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ScriptFailedToParse;
impl ydp_core::Event for ScriptFailedToParse {
    type Params = ScriptFailedToParseParams;
    const METHOD: &'static str = "Debugger.scriptFailedToParse";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.event.scriptParsed/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ScriptParsedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptParsed/parameters/scriptId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the script parsed.</pre>
    ///
    ///
    #[serde(rename = "scriptId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub script_id: crate::runtime::types::ScriptId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptParsed/parameters/url。
    ///
    ///协议原文：
    ///
    ///<pre>URL or name of the script parsed (if any).</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptParsed/parameters/startLine。
    ///
    ///协议原文：
    ///
    ///<pre>Line offset of the script within the resource with given URL (for script tags).</pre>
    ///
    ///
    #[serde(rename = "startLine")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub start_line: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptParsed/parameters/startColumn。
    ///
    ///协议原文：
    ///
    ///<pre>Column offset of the script within the resource with given URL.</pre>
    ///
    ///
    #[serde(rename = "startColumn")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub start_column: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptParsed/parameters/endLine。
    ///
    ///协议原文：
    ///
    ///<pre>Last line of the script.</pre>
    ///
    ///
    #[serde(rename = "endLine")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub end_line: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptParsed/parameters/endColumn。
    ///
    ///协议原文：
    ///
    ///<pre>Length of the last line of the script.</pre>
    ///
    ///
    #[serde(rename = "endColumn")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub end_column: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptParsed/parameters/executionContextId。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies script creation context.</pre>
    ///
    ///
    #[serde(rename = "executionContextId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub execution_context_id: crate::runtime::types::ExecutionContextId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptParsed/parameters/hash。
    ///
    ///协议原文：
    ///
    ///<pre>Content hash of the script, SHA-256.</pre>
    ///
    ///
    #[serde(rename = "hash")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub hash: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptParsed/parameters/buildId。
    ///
    ///协议原文：
    ///
    ///<pre>For Wasm modules, the content of the `build_id` custom section. For JavaScript the `debugId` magic comment.</pre>
    ///
    ///
    #[serde(rename = "buildId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub build_id: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptParsed/parameters/executionContextAuxData。
    ///
    ///协议原文：
    ///
    ///<pre>Embedder-specific auxiliary data likely matching {isDefault: boolean, type: 'default'|'isolated'|'worker', frameId: string}</pre>
    ///
    ///
    #[serde(rename = "executionContextAuxData")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_context_aux_data: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptParsed/parameters/isLiveEdit。
    ///
    ///协议原文：
    ///
    ///<pre>True, if this script is generated as a result of the live edit operation.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "isLiveEdit")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_live_edit: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptParsed/parameters/sourceMapURL。
    ///
    ///协议原文：
    ///
    ///<pre>URL of source map associated with script (if any).</pre>
    ///
    ///
    #[serde(rename = "sourceMapURL")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_map_url: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptParsed/parameters/hasSourceURL。
    ///
    ///协议原文：
    ///
    ///<pre>True, if this script has sourceURL.</pre>
    ///
    ///
    #[serde(rename = "hasSourceURL")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_source_url: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptParsed/parameters/isModule。
    ///
    ///协议原文：
    ///
    ///<pre>True, if this script is ES6 module.</pre>
    ///
    ///
    #[serde(rename = "isModule")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_module: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptParsed/parameters/length。
    ///
    ///协议原文：
    ///
    ///<pre>This script length.</pre>
    ///
    ///
    #[serde(rename = "length")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub length: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptParsed/parameters/stackTrace。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript top stack frame of where the script parsed event was triggered if available.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "stackTrace")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub stack_trace: Option<crate::runtime::types::StackTrace>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptParsed/parameters/codeOffset。
    ///
    ///协议原文：
    ///
    ///<pre>If the scriptLanguage is WebAssembly, the code section offset in the module.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "codeOffset")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub code_offset: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptParsed/parameters/scriptLanguage。
    ///
    ///协议原文：
    ///
    ///<pre>The language of the script.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "scriptLanguage")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub script_language: Option<crate::debugger::types::ScriptLanguage>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptParsed/parameters/debugSymbols。
    ///
    ///协议原文：
    ///
    ///<pre>If the scriptLanguage is WebAssembly, the source of debug symbols for the module.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "debugSymbols")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub debug_symbols: Option<Vec<crate::debugger::types::DebugSymbols>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptParsed/parameters/embedderName。
    ///
    ///协议原文：
    ///
    ///<pre>The name the embedder supplied for this script.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "embedderName")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub embedder_name: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptParsed/parameters/resolvedBreakpoints。
    ///
    ///协议原文：
    ///
    ///<pre>The list of set breakpoints in this script if calls to `setBreakpointByUrl`
    ///matches this script's URL or hash. Clients that use this list can ignore the
    ///`breakpointResolved` event. They are equivalent.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "resolvedBreakpoints")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub resolved_breakpoints: Option<Vec<crate::debugger::types::ResolvedBreakpoint>>,
}
impl<'de> serde::Deserialize<'de> for ScriptParsedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScriptParsedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptParsed/parameters/scriptId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the script parsed.</pre>
                    ///
                    ///
                    #[serde(rename = "scriptId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub script_id: crate::runtime::types::ScriptId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptParsed/parameters/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>URL or name of the script parsed (if any).</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptParsed/parameters/startLine。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Line offset of the script within the resource with given URL (for script tags).</pre>
                    ///
                    ///
                    #[serde(rename = "startLine")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub start_line: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptParsed/parameters/startColumn。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Column offset of the script within the resource with given URL.</pre>
                    ///
                    ///
                    #[serde(rename = "startColumn")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub start_column: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptParsed/parameters/endLine。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Last line of the script.</pre>
                    ///
                    ///
                    #[serde(rename = "endLine")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub end_line: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptParsed/parameters/endColumn。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Length of the last line of the script.</pre>
                    ///
                    ///
                    #[serde(rename = "endColumn")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub end_column: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptParsed/parameters/executionContextId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies script creation context.</pre>
                    ///
                    ///
                    #[serde(rename = "executionContextId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub execution_context_id: crate::runtime::types::ExecutionContextId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptParsed/parameters/hash。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Content hash of the script, SHA-256.</pre>
                    ///
                    ///
                    #[serde(rename = "hash")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub hash: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.event.scriptParsed/parameters/buildId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>For Wasm modules, the content of the `build_id` custom section. For JavaScript the `debugId` magic comment.</pre>
                    ///
                    ///
                    #[serde(rename = "buildId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub build_id: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptParsed/parameters/executionContextAuxData。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Embedder-specific auxiliary data likely matching {isDefault: boolean, type: 'default'|'isolated'|'worker', frameId: string}</pre>
                    ///
                    ///
                    #[serde(rename = "executionContextAuxData")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub execution_context_aux_data:
                        Option<std::collections::BTreeMap<String, serde_json::Value>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptParsed/parameters/isLiveEdit。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True, if this script is generated as a result of the live edit operation.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "isLiveEdit")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub is_live_edit: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptParsed/parameters/sourceMapURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>URL of source map associated with script (if any).</pre>
                    ///
                    ///
                    #[serde(rename = "sourceMapURL")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub source_map_url: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptParsed/parameters/hasSourceURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True, if this script has sourceURL.</pre>
                    ///
                    ///
                    #[serde(rename = "hasSourceURL")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub has_source_url: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptParsed/parameters/isModule。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True, if this script is ES6 module.</pre>
                    ///
                    ///
                    #[serde(rename = "isModule")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub is_module: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptParsed/parameters/length。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>This script length.</pre>
                    ///
                    ///
                    #[serde(rename = "length")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub length: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptParsed/parameters/stackTrace。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript top stack frame of where the script parsed event was triggered if available.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "stackTrace")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub stack_trace: Option<crate::runtime::types::StackTrace>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptParsed/parameters/codeOffset。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If the scriptLanguage is WebAssembly, the code section offset in the module.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "codeOffset")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub code_offset: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptParsed/parameters/scriptLanguage。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The language of the script.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "scriptLanguage")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub script_language: Option<crate::debugger::types::ScriptLanguage>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptParsed/parameters/debugSymbols。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If the scriptLanguage is WebAssembly, the source of debug symbols for the module.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "debugSymbols")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub debug_symbols: Option<Vec<crate::debugger::types::DebugSymbols>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptParsed/parameters/embedderName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The name the embedder supplied for this script.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "embedderName")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub embedder_name: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.event.scriptParsed/parameters/resolvedBreakpoints。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The list of set breakpoints in this script if calls to `setBreakpointByUrl`
                    ///matches this script's URL or hash. Clients that use this list can ignore the
                    ///`breakpointResolved` event. They are equivalent.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "resolvedBreakpoints")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub resolved_breakpoints:
                        Option<Vec<crate::debugger::types::ResolvedBreakpoint>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ScriptParsedParams {
                    script_id: fields.script_id,
                    url: fields.url,
                    start_line: fields.start_line,
                    start_column: fields.start_column,
                    end_line: fields.end_line,
                    end_column: fields.end_column,
                    execution_context_id: fields.execution_context_id,
                    hash: fields.hash,
                    build_id: fields.build_id,
                    execution_context_aux_data: fields.execution_context_aux_data,
                    is_live_edit: fields.is_live_edit,
                    source_map_url: fields.source_map_url,
                    has_source_url: fields.has_source_url,
                    is_module: fields.is_module,
                    length: fields.length,
                    stack_trace: fields.stack_trace,
                    code_offset: fields.code_offset,
                    script_language: fields.script_language,
                    debug_symbols: fields.debug_symbols,
                    embedder_name: fields.embedder_name,
                    resolved_breakpoints: fields.resolved_breakpoints,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.event.scriptParsed。
///
///协议原文：
///
///<pre>Fired when virtual machine parses script. This event is also fired for all known and uncollected
///scripts upon enabling debugger.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ScriptParsed;
impl ydp_core::Event for ScriptParsed {
    type Params = ScriptParsedParams;
    const METHOD: &'static str = "Debugger.scriptParsed";
}
