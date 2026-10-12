///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.event.bindingCalled/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct BindingCalledParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.bindingCalled/parameters/name。
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.bindingCalled/parameters/payload。
    #[serde(rename = "payload")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub payload: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.bindingCalled/parameters/executionContextId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the context where the call was made.</pre>
    ///
    ///
    #[serde(rename = "executionContextId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub execution_context_id: crate::runtime::types::ExecutionContextId,
}
impl<'de> serde::Deserialize<'de> for BindingCalledParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BindingCalledParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.bindingCalled/parameters/name。
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.bindingCalled/parameters/payload。
                    #[serde(rename = "payload")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub payload: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.bindingCalled/parameters/executionContextId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the context where the call was made.</pre>
                    ///
                    ///
                    #[serde(rename = "executionContextId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub execution_context_id: crate::runtime::types::ExecutionContextId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(BindingCalledParams {
                    name: fields.name,
                    payload: fields.payload,
                    execution_context_id: fields.execution_context_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.event.bindingCalled。
///
///协议原文：
///
///<pre>Notification is issued every time when binding is called.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct BindingCalled;
impl ydp_core::Event for BindingCalled {
    type Params = BindingCalledParams;
    const METHOD: &'static str = "Runtime.bindingCalled";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Runtime.event.consoleAPICalled/parameters/type。
///
///协议原文：
///
///<pre>Type of the call.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConsoleApiCalledParamsType {
    ///协议值 "log"。
    Log,
    ///协议值 "debug"。
    Debug,
    ///协议值 "info"。
    Info,
    ///协议值 "error"。
    Error,
    ///协议值 "warning"。
    Warning,
    ///协议值 "dir"。
    Dir,
    ///协议值 "dirxml"。
    Dirxml,
    ///协议值 "table"。
    Table,
    ///协议值 "trace"。
    Trace,
    ///协议值 "clear"。
    Clear,
    ///协议值 "startGroup"。
    StartGroup,
    ///协议值 "startGroupCollapsed"。
    StartGroupCollapsed,
    ///协议值 "endGroup"。
    EndGroup,
    ///协议值 "assert"。
    Assert,
    ///协议值 "profile"。
    Profile,
    ///协议值 "profileEnd"。
    ProfileEnd,
    ///协议值 "count"。
    Count,
    ///协议值 "timeEnd"。
    TimeEnd,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ConsoleApiCalledParamsType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::runtime::events::ConsoleApiCalledParamsType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Log => "log",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Dir => "dir",
            Self::Dirxml => "dirxml",
            Self::Table => "table",
            Self::Trace => "trace",
            Self::Clear => "clear",
            Self::StartGroup => "startGroup",
            Self::StartGroupCollapsed => "startGroupCollapsed",
            Self::EndGroup => "endGroup",
            Self::Assert => "assert",
            Self::Profile => "profile",
            Self::ProfileEnd => "profileEnd",
            Self::Count => "count",
            Self::TimeEnd => "timeEnd",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ConsoleApiCalledParamsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ConsoleApiCalledParamsType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "log" => ConsoleApiCalledParamsType::Log,
            "debug" => ConsoleApiCalledParamsType::Debug,
            "info" => ConsoleApiCalledParamsType::Info,
            "error" => ConsoleApiCalledParamsType::Error,
            "warning" => ConsoleApiCalledParamsType::Warning,
            "dir" => ConsoleApiCalledParamsType::Dir,
            "dirxml" => ConsoleApiCalledParamsType::Dirxml,
            "table" => ConsoleApiCalledParamsType::Table,
            "trace" => ConsoleApiCalledParamsType::Trace,
            "clear" => ConsoleApiCalledParamsType::Clear,
            "startGroup" => ConsoleApiCalledParamsType::StartGroup,
            "startGroupCollapsed" => ConsoleApiCalledParamsType::StartGroupCollapsed,
            "endGroup" => ConsoleApiCalledParamsType::EndGroup,
            "assert" => ConsoleApiCalledParamsType::Assert,
            "profile" => ConsoleApiCalledParamsType::Profile,
            "profileEnd" => ConsoleApiCalledParamsType::ProfileEnd,
            "count" => ConsoleApiCalledParamsType::Count,
            "timeEnd" => ConsoleApiCalledParamsType::TimeEnd,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ConsoleApiCalledParamsType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ConsoleApiCalledParamsType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ConsoleApiCalledParamsType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "log" => ConsoleApiCalledParamsType::Log,
                    "debug" => ConsoleApiCalledParamsType::Debug,
                    "info" => ConsoleApiCalledParamsType::Info,
                    "error" => ConsoleApiCalledParamsType::Error,
                    "warning" => ConsoleApiCalledParamsType::Warning,
                    "dir" => ConsoleApiCalledParamsType::Dir,
                    "dirxml" => ConsoleApiCalledParamsType::Dirxml,
                    "table" => ConsoleApiCalledParamsType::Table,
                    "trace" => ConsoleApiCalledParamsType::Trace,
                    "clear" => ConsoleApiCalledParamsType::Clear,
                    "startGroup" => ConsoleApiCalledParamsType::StartGroup,
                    "startGroupCollapsed" => ConsoleApiCalledParamsType::StartGroupCollapsed,
                    "endGroup" => ConsoleApiCalledParamsType::EndGroup,
                    "assert" => ConsoleApiCalledParamsType::Assert,
                    "profile" => ConsoleApiCalledParamsType::Profile,
                    "profileEnd" => ConsoleApiCalledParamsType::ProfileEnd,
                    "count" => ConsoleApiCalledParamsType::Count,
                    "timeEnd" => ConsoleApiCalledParamsType::TimeEnd,
                    _ => ConsoleApiCalledParamsType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ConsoleApiCalledParamsType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.event.consoleAPICalled/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ConsoleApiCalledParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.consoleAPICalled/parameters/type。
    ///
    ///协议原文：
    ///
    ///<pre>Type of the call.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: ConsoleApiCalledParamsType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.consoleAPICalled/parameters/args。
    ///
    ///协议原文：
    ///
    ///<pre>Call arguments.</pre>
    ///
    ///
    #[serde(rename = "args")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub args: Vec<crate::runtime::types::RemoteObject>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.consoleAPICalled/parameters/executionContextId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the context where the call was made.</pre>
    ///
    ///
    #[serde(rename = "executionContextId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub execution_context_id: crate::runtime::types::ExecutionContextId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.consoleAPICalled/parameters/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Call timestamp.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::runtime::types::Timestamp,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.event.consoleAPICalled/parameters/stackTrace。
    ///
    ///协议原文：
    ///
    ///<pre>Stack trace captured when the call was made. The async stack chain is automatically reported for
    ///the following call types: `assert`, `error`, `trace`, `warning`. For other types the async call
    ///chain can be retrieved using `Debugger.getStackTrace` and `stackTrace.parentId` field.</pre>
    ///
    ///
    #[serde(rename = "stackTrace")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub stack_trace: Option<crate::runtime::types::StackTrace>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.event.consoleAPICalled/parameters/context。
    ///
    ///协议原文：
    ///
    ///<pre>Console context descriptor for calls on non-default console context (not console.*):
    ///'anonymous#unique-logger-id' for call on unnamed context, 'name#unique-logger-id' for call
    ///on named context.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "context")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub context: Option<String>,
}
impl<'de> serde::Deserialize<'de> for ConsoleApiCalledParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ConsoleApiCalledParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.consoleAPICalled/parameters/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Type of the call.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: ConsoleApiCalledParamsType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.consoleAPICalled/parameters/args。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Call arguments.</pre>
                    ///
                    ///
                    #[serde(rename = "args")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub args: Vec<crate::runtime::types::RemoteObject>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.consoleAPICalled/parameters/executionContextId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the context where the call was made.</pre>
                    ///
                    ///
                    #[serde(rename = "executionContextId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub execution_context_id: crate::runtime::types::ExecutionContextId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.consoleAPICalled/parameters/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Call timestamp.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::runtime::types::Timestamp,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.event.consoleAPICalled/parameters/stackTrace。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Stack trace captured when the call was made. The async stack chain is automatically reported for
                    ///the following call types: `assert`, `error`, `trace`, `warning`. For other types the async call
                    ///chain can be retrieved using `Debugger.getStackTrace` and `stackTrace.parentId` field.</pre>
                    ///
                    ///
                    #[serde(rename = "stackTrace")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub stack_trace: Option<crate::runtime::types::StackTrace>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.event.consoleAPICalled/parameters/context。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Console context descriptor for calls on non-default console context (not console.*):
                    ///'anonymous#unique-logger-id' for call on unnamed context, 'name#unique-logger-id' for call
                    ///on named context.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "context")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub context: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ConsoleApiCalledParams {
                    r#type: fields.r#type,
                    args: fields.args,
                    execution_context_id: fields.execution_context_id,
                    timestamp: fields.timestamp,
                    stack_trace: fields.stack_trace,
                    context: fields.context,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.event.consoleAPICalled。
///
///协议原文：
///
///<pre>Issued when console API was called.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ConsoleApiCalled;
impl ydp_core::Event for ConsoleApiCalled {
    type Params = ConsoleApiCalledParams;
    const METHOD: &'static str = "Runtime.consoleAPICalled";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.event.exceptionRevoked/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ExceptionRevokedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.exceptionRevoked/parameters/reason。
    ///
    ///协议原文：
    ///
    ///<pre>Reason describing why exception was revoked.</pre>
    ///
    ///
    #[serde(rename = "reason")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub reason: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.exceptionRevoked/parameters/exceptionId。
    ///
    ///协议原文：
    ///
    ///<pre>The id of revoked exception, as reported in `exceptionThrown`.</pre>
    ///
    ///
    #[serde(rename = "exceptionId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub exception_id: i64,
}
impl<'de> serde::Deserialize<'de> for ExceptionRevokedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExceptionRevokedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.exceptionRevoked/parameters/reason。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Reason describing why exception was revoked.</pre>
                    ///
                    ///
                    #[serde(rename = "reason")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub reason: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.exceptionRevoked/parameters/exceptionId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The id of revoked exception, as reported in `exceptionThrown`.</pre>
                    ///
                    ///
                    #[serde(rename = "exceptionId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub exception_id: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ExceptionRevokedParams {
                    reason: fields.reason,
                    exception_id: fields.exception_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.event.exceptionRevoked。
///
///协议原文：
///
///<pre>Issued when unhandled exception was revoked.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ExceptionRevoked;
impl ydp_core::Event for ExceptionRevoked {
    type Params = ExceptionRevokedParams;
    const METHOD: &'static str = "Runtime.exceptionRevoked";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.event.exceptionThrown/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ExceptionThrownParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.exceptionThrown/parameters/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Timestamp of the exception.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::runtime::types::Timestamp,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.exceptionThrown/parameters/exceptionDetails。
    #[serde(rename = "exceptionDetails")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub exception_details: crate::runtime::types::ExceptionDetails,
}
impl<'de> serde::Deserialize<'de> for ExceptionThrownParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExceptionThrownParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.exceptionThrown/parameters/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Timestamp of the exception.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::runtime::types::Timestamp,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.exceptionThrown/parameters/exceptionDetails。
                    #[serde(rename = "exceptionDetails")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub exception_details: crate::runtime::types::ExceptionDetails,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ExceptionThrownParams {
                    timestamp: fields.timestamp,
                    exception_details: fields.exception_details,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.event.exceptionThrown。
///
///协议原文：
///
///<pre>Issued when exception was thrown and unhandled.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ExceptionThrown;
impl ydp_core::Event for ExceptionThrown {
    type Params = ExceptionThrownParams;
    const METHOD: &'static str = "Runtime.exceptionThrown";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.event.executionContextCreated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ExecutionContextCreatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.executionContextCreated/parameters/context。
    ///
    ///协议原文：
    ///
    ///<pre>A newly created execution context.</pre>
    ///
    ///
    #[serde(rename = "context")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub context: crate::runtime::types::ExecutionContextDescription,
}
impl<'de> serde::Deserialize<'de> for ExecutionContextCreatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExecutionContextCreatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.executionContextCreated/parameters/context。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A newly created execution context.</pre>
                    ///
                    ///
                    #[serde(rename = "context")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub context: crate::runtime::types::ExecutionContextDescription,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ExecutionContextCreatedParams {
                    context: fields.context,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.event.executionContextCreated。
///
///协议原文：
///
///<pre>Issued when new execution context is created.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ExecutionContextCreated;
impl ydp_core::Event for ExecutionContextCreated {
    type Params = ExecutionContextCreatedParams;
    const METHOD: &'static str = "Runtime.executionContextCreated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.event.executionContextDestroyed/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ExecutionContextDestroyedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.executionContextDestroyed/parameters/executionContextId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the destroyed context</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "executionContextId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub execution_context_id: crate::runtime::types::ExecutionContextId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.executionContextDestroyed/parameters/executionContextUniqueId。
    ///
    ///协议原文：
    ///
    ///<pre>Unique Id of the destroyed context</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "executionContextUniqueId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub execution_context_unique_id: String,
}
impl<'de> serde::Deserialize<'de> for ExecutionContextDestroyedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExecutionContextDestroyedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.executionContextDestroyed/parameters/executionContextId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the destroyed context</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "executionContextId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub execution_context_id: crate::runtime::types::ExecutionContextId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.executionContextDestroyed/parameters/executionContextUniqueId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Unique Id of the destroyed context</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "executionContextUniqueId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub execution_context_unique_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ExecutionContextDestroyedParams {
                    execution_context_id: fields.execution_context_id,
                    execution_context_unique_id: fields.execution_context_unique_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.event.executionContextDestroyed。
///
///协议原文：
///
///<pre>Issued when execution context is destroyed.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ExecutionContextDestroyed;
impl ydp_core::Event for ExecutionContextDestroyed {
    type Params = ExecutionContextDestroyedParams;
    const METHOD: &'static str = "Runtime.executionContextDestroyed";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.event.executionContextsCleared/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ExecutionContextsClearedParams {}
impl<'de> serde::Deserialize<'de> for ExecutionContextsClearedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExecutionContextsClearedParams;
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
                Ok(ExecutionContextsClearedParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.event.executionContextsCleared。
///
///协议原文：
///
///<pre>Issued when all executionContexts were cleared in browser</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ExecutionContextsCleared;
impl ydp_core::Event for ExecutionContextsCleared {
    type Params = ExecutionContextsClearedParams;
    const METHOD: &'static str = "Runtime.executionContextsCleared";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.event.inspectRequested/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct InspectRequestedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.inspectRequested/parameters/object。
    #[serde(rename = "object")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub object: crate::runtime::types::RemoteObject,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.inspectRequested/parameters/hints。
    #[serde(rename = "hints")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub hints: std::collections::BTreeMap<String, serde_json::Value>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.event.inspectRequested/parameters/executionContextId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the context where the call was made.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "executionContextId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_context_id: Option<crate::runtime::types::ExecutionContextId>,
}
impl<'de> serde::Deserialize<'de> for InspectRequestedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InspectRequestedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.inspectRequested/parameters/object。
                    #[serde(rename = "object")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub object: crate::runtime::types::RemoteObject,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.event.inspectRequested/parameters/hints。
                    #[serde(rename = "hints")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub hints: std::collections::BTreeMap<String, serde_json::Value>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.event.inspectRequested/parameters/executionContextId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the context where the call was made.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "executionContextId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub execution_context_id: Option<crate::runtime::types::ExecutionContextId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(InspectRequestedParams {
                    object: fields.object,
                    hints: fields.hints,
                    execution_context_id: fields.execution_context_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Runtime.event.inspectRequested。
///
///协议原文：
///
///<pre>Issued when object should be inspected (for example, as a result of inspect() command line API
///call).</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct InspectRequested;
impl ydp_core::Event for InspectRequested {
    type Params = InspectRequestedParams;
    const METHOD: &'static str = "Runtime.inspectRequested";
}
