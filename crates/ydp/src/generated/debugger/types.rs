///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Debugger.type.BreakLocation/properties/type。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum BreakLocationType {
    ///协议值 "debuggerStatement"。
    DebuggerStatement,
    ///协议值 "call"。
    Call,
    ///协议值 "return"。
    Return,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl BreakLocationType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::debugger::types::BreakLocationType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::DebuggerStatement => "debuggerStatement",
            Self::Call => "call",
            Self::Return => "return",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for BreakLocationType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for BreakLocationType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "debuggerStatement" => BreakLocationType::DebuggerStatement,
            "call" => BreakLocationType::Call,
            "return" => BreakLocationType::Return,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for BreakLocationType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BreakLocationType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BreakLocationType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "debuggerStatement" => BreakLocationType::DebuggerStatement,
                    "call" => BreakLocationType::Call,
                    "return" => BreakLocationType::Return,
                    _ => BreakLocationType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(BreakLocationType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.type.BreakLocation。
#[derive(Debug, Clone, serde::Serialize)]
pub struct BreakLocation {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.BreakLocation/properties/scriptId。
    ///
    ///协议原文：
    ///
    ///<pre>Script identifier as reported in the `Debugger.scriptParsed`.</pre>
    ///
    ///
    #[serde(rename = "scriptId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub script_id: crate::runtime::types::ScriptId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.BreakLocation/properties/lineNumber。
    ///
    ///协议原文：
    ///
    ///<pre>Line number in the script (0-based).</pre>
    ///
    ///
    #[serde(rename = "lineNumber")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub line_number: i64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.type.BreakLocation/properties/columnNumber。
    ///
    ///协议原文：
    ///
    ///<pre>Column number in the script (0-based).</pre>
    ///
    ///
    #[serde(rename = "columnNumber")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub column_number: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.type.BreakLocation/properties/type。
    #[serde(rename = "type")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<BreakLocationType>,
}
impl<'de> serde::Deserialize<'de> for BreakLocation {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BreakLocation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.BreakLocation/properties/scriptId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Script identifier as reported in the `Debugger.scriptParsed`.</pre>
                    ///
                    ///
                    #[serde(rename = "scriptId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub script_id: crate::runtime::types::ScriptId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.BreakLocation/properties/lineNumber。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Line number in the script (0-based).</pre>
                    ///
                    ///
                    #[serde(rename = "lineNumber")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub line_number: i64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.type.BreakLocation/properties/columnNumber。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Column number in the script (0-based).</pre>
                    ///
                    ///
                    #[serde(rename = "columnNumber")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub column_number: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.type.BreakLocation/properties/type。
                    #[serde(rename = "type")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub r#type: Option<BreakLocationType>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(BreakLocation {
                    script_id: fields.script_id,
                    line_number: fields.line_number,
                    column_number: fields.column_number,
                    r#type: fields.r#type,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Debugger.type.BreakpointId。
///
///协议原文：
///
///<pre>Breakpoint identifier.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct BreakpointId(String);
impl BreakpointId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::debugger::types::BreakpointId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::debugger::types::BreakpointId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::debugger::types::BreakpointId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::debugger::types::BreakpointId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::debugger::types::BreakpointId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.type.CallFrame。
///
///协议原文：
///
///<pre>JavaScript call frame. Array of call frames form the call stack.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CallFrame {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.CallFrame/properties/callFrameId。
    ///
    ///协议原文：
    ///
    ///<pre>Call frame identifier. This identifier is only valid while the virtual machine is paused.</pre>
    ///
    ///
    #[serde(rename = "callFrameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub call_frame_id: crate::debugger::types::CallFrameId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.CallFrame/properties/functionName。
    ///
    ///协议原文：
    ///
    ///<pre>Name of the JavaScript function called on this call frame.</pre>
    ///
    ///
    #[serde(rename = "functionName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub function_name: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.type.CallFrame/properties/functionLocation。
    ///
    ///协议原文：
    ///
    ///<pre>Location in the source code.</pre>
    ///
    ///
    #[serde(rename = "functionLocation")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub function_location: Option<crate::debugger::types::Location>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.CallFrame/properties/location。
    ///
    ///协议原文：
    ///
    ///<pre>Location in the source code.</pre>
    ///
    ///
    #[serde(rename = "location")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub location: crate::debugger::types::Location,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.CallFrame/properties/url。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript script name or url.
    ///Deprecated in favor of using the `location.scriptId` to resolve the URL via a previously
    ///sent `Debugger.scriptParsed` event.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.CallFrame/properties/scopeChain。
    ///
    ///协议原文：
    ///
    ///<pre>Scope chain for this call frame.</pre>
    ///
    ///
    #[serde(rename = "scopeChain")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub scope_chain: Vec<crate::debugger::types::Scope>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.CallFrame/properties/this。
    ///
    ///协议原文：
    ///
    ///<pre>`this` object for this call frame.</pre>
    ///
    ///
    #[serde(rename = "this")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub this: crate::runtime::types::RemoteObject,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.type.CallFrame/properties/returnValue。
    ///
    ///协议原文：
    ///
    ///<pre>The value being returned, if the function is at return point.</pre>
    ///
    ///
    #[serde(rename = "returnValue")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub return_value: Option<crate::runtime::types::RemoteObject>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.type.CallFrame/properties/canBeRestarted。
    ///
    ///协议原文：
    ///
    ///<pre>Valid only while the VM is paused and indicates whether this frame
    ///can be restarted or not. Note that a `true` value here does not
    ///guarantee that Debugger#restartFrame with this CallFrameId will be
    ///successful, but it is very likely.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "canBeRestarted")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_be_restarted: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for CallFrame {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CallFrame;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.CallFrame/properties/callFrameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Call frame identifier. This identifier is only valid while the virtual machine is paused.</pre>
                    ///
                    ///
                    #[serde(rename = "callFrameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub call_frame_id: crate::debugger::types::CallFrameId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.CallFrame/properties/functionName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Name of the JavaScript function called on this call frame.</pre>
                    ///
                    ///
                    #[serde(rename = "functionName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub function_name: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.type.CallFrame/properties/functionLocation。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Location in the source code.</pre>
                    ///
                    ///
                    #[serde(rename = "functionLocation")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub function_location: Option<crate::debugger::types::Location>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.CallFrame/properties/location。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Location in the source code.</pre>
                    ///
                    ///
                    #[serde(rename = "location")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub location: crate::debugger::types::Location,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.CallFrame/properties/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript script name or url.
                    ///Deprecated in favor of using the `location.scriptId` to resolve the URL via a previously
                    ///sent `Debugger.scriptParsed` event.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.CallFrame/properties/scopeChain。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Scope chain for this call frame.</pre>
                    ///
                    ///
                    #[serde(rename = "scopeChain")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub scope_chain: Vec<crate::debugger::types::Scope>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.CallFrame/properties/this。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`this` object for this call frame.</pre>
                    ///
                    ///
                    #[serde(rename = "this")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub this: crate::runtime::types::RemoteObject,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.type.CallFrame/properties/returnValue。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The value being returned, if the function is at return point.</pre>
                    ///
                    ///
                    #[serde(rename = "returnValue")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub return_value: Option<crate::runtime::types::RemoteObject>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.type.CallFrame/properties/canBeRestarted。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Valid only while the VM is paused and indicates whether this frame
                    ///can be restarted or not. Note that a `true` value here does not
                    ///guarantee that Debugger#restartFrame with this CallFrameId will be
                    ///successful, but it is very likely.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "canBeRestarted")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub can_be_restarted: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CallFrame {
                    call_frame_id: fields.call_frame_id,
                    function_name: fields.function_name,
                    function_location: fields.function_location,
                    location: fields.location,
                    url: fields.url,
                    scope_chain: fields.scope_chain,
                    this: fields.this,
                    return_value: fields.return_value,
                    can_be_restarted: fields.can_be_restarted,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Debugger.type.CallFrameId。
///
///协议原文：
///
///<pre>Call frame identifier.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct CallFrameId(String);
impl CallFrameId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::debugger::types::CallFrameId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::debugger::types::CallFrameId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::debugger::types::CallFrameId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::debugger::types::CallFrameId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::debugger::types::CallFrameId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Debugger.type.DebugSymbols/properties/type。
///
///协议原文：
///
///<pre>Type of the debug symbols.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DebugSymbolsType {
    ///协议值 "SourceMap"。
    SourceMap,
    ///协议值 "EmbeddedDWARF"。
    EmbeddedDwarf,
    ///协议值 "ExternalDWARF"。
    ExternalDwarf,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl DebugSymbolsType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::debugger::types::DebugSymbolsType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::SourceMap => "SourceMap",
            Self::EmbeddedDwarf => "EmbeddedDWARF",
            Self::ExternalDwarf => "ExternalDWARF",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for DebugSymbolsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for DebugSymbolsType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "SourceMap" => DebugSymbolsType::SourceMap,
            "EmbeddedDWARF" => DebugSymbolsType::EmbeddedDwarf,
            "ExternalDWARF" => DebugSymbolsType::ExternalDwarf,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for DebugSymbolsType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DebugSymbolsType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DebugSymbolsType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "SourceMap" => DebugSymbolsType::SourceMap,
                    "EmbeddedDWARF" => DebugSymbolsType::EmbeddedDwarf,
                    "ExternalDWARF" => DebugSymbolsType::ExternalDwarf,
                    _ => DebugSymbolsType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(DebugSymbolsType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.type.DebugSymbols。
///
///协议原文：
///
///<pre>Debug symbols available for a wasm script.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct DebugSymbols {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.DebugSymbols/properties/type。
    ///
    ///协议原文：
    ///
    ///<pre>Type of the debug symbols.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: DebugSymbolsType,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.type.DebugSymbols/properties/externalURL。
    ///
    ///协议原文：
    ///
    ///<pre>URL of the external symbol source.</pre>
    ///
    ///
    #[serde(rename = "externalURL")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_url: Option<String>,
}
impl<'de> serde::Deserialize<'de> for DebugSymbols {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DebugSymbols;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.DebugSymbols/properties/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Type of the debug symbols.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: DebugSymbolsType,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.type.DebugSymbols/properties/externalURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>URL of the external symbol source.</pre>
                    ///
                    ///
                    #[serde(rename = "externalURL")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub external_url: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DebugSymbols {
                    r#type: fields.r#type,
                    external_url: fields.external_url,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.type.Location。
///
///协议原文：
///
///<pre>Location in the source code.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct Location {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.Location/properties/scriptId。
    ///
    ///协议原文：
    ///
    ///<pre>Script identifier as reported in the `Debugger.scriptParsed`.</pre>
    ///
    ///
    #[serde(rename = "scriptId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub script_id: crate::runtime::types::ScriptId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.Location/properties/lineNumber。
    ///
    ///协议原文：
    ///
    ///<pre>Line number in the script (0-based).</pre>
    ///
    ///
    #[serde(rename = "lineNumber")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub line_number: i64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.type.Location/properties/columnNumber。
    ///
    ///协议原文：
    ///
    ///<pre>Column number in the script (0-based).</pre>
    ///
    ///
    #[serde(rename = "columnNumber")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub column_number: Option<i64>,
}
impl<'de> serde::Deserialize<'de> for Location {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Location;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.Location/properties/scriptId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Script identifier as reported in the `Debugger.scriptParsed`.</pre>
                    ///
                    ///
                    #[serde(rename = "scriptId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub script_id: crate::runtime::types::ScriptId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.Location/properties/lineNumber。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Line number in the script (0-based).</pre>
                    ///
                    ///
                    #[serde(rename = "lineNumber")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub line_number: i64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.type.Location/properties/columnNumber。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Column number in the script (0-based).</pre>
                    ///
                    ///
                    #[serde(rename = "columnNumber")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub column_number: Option<i64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Location {
                    script_id: fields.script_id,
                    line_number: fields.line_number,
                    column_number: fields.column_number,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.type.LocationRange。
///
///协议原文：
///
///<pre>Location range within one script.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct LocationRange {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.LocationRange/properties/scriptId。
    #[serde(rename = "scriptId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub script_id: crate::runtime::types::ScriptId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.LocationRange/properties/start。
    #[serde(rename = "start")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub start: crate::debugger::types::ScriptPosition,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.LocationRange/properties/end。
    #[serde(rename = "end")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub end: crate::debugger::types::ScriptPosition,
}
impl<'de> serde::Deserialize<'de> for LocationRange {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LocationRange;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.LocationRange/properties/scriptId。
                    #[serde(rename = "scriptId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub script_id: crate::runtime::types::ScriptId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.LocationRange/properties/start。
                    #[serde(rename = "start")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub start: crate::debugger::types::ScriptPosition,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.LocationRange/properties/end。
                    #[serde(rename = "end")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub end: crate::debugger::types::ScriptPosition,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LocationRange {
                    script_id: fields.script_id,
                    start: fields.start,
                    end: fields.end,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.type.ResolvedBreakpoint。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ResolvedBreakpoint {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.ResolvedBreakpoint/properties/breakpointId。
    ///
    ///协议原文：
    ///
    ///<pre>Breakpoint unique identifier.</pre>
    ///
    ///
    #[serde(rename = "breakpointId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub breakpoint_id: crate::debugger::types::BreakpointId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.ResolvedBreakpoint/properties/location。
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
impl<'de> serde::Deserialize<'de> for ResolvedBreakpoint {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResolvedBreakpoint;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.ResolvedBreakpoint/properties/breakpointId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Breakpoint unique identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "breakpointId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub breakpoint_id: crate::debugger::types::BreakpointId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.ResolvedBreakpoint/properties/location。
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
                Ok(ResolvedBreakpoint {
                    breakpoint_id: fields.breakpoint_id,
                    location: fields.location,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Debugger.type.Scope/properties/type。
///
///协议原文：
///
///<pre>Scope type.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ScopeType {
    ///协议值 "global"。
    Global,
    ///协议值 "local"。
    Local,
    ///协议值 "with"。
    With,
    ///协议值 "closure"。
    Closure,
    ///协议值 "catch"。
    Catch,
    ///协议值 "block"。
    Block,
    ///协议值 "script"。
    Script,
    ///协议值 "eval"。
    Eval,
    ///协议值 "module"。
    Module,
    ///协议值 "wasm-expression-stack"。
    WasmExpressionStack,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ScopeType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::debugger::types::ScopeType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Global => "global",
            Self::Local => "local",
            Self::With => "with",
            Self::Closure => "closure",
            Self::Catch => "catch",
            Self::Block => "block",
            Self::Script => "script",
            Self::Eval => "eval",
            Self::Module => "module",
            Self::WasmExpressionStack => "wasm-expression-stack",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ScopeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ScopeType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "global" => ScopeType::Global,
            "local" => ScopeType::Local,
            "with" => ScopeType::With,
            "closure" => ScopeType::Closure,
            "catch" => ScopeType::Catch,
            "block" => ScopeType::Block,
            "script" => ScopeType::Script,
            "eval" => ScopeType::Eval,
            "module" => ScopeType::Module,
            "wasm-expression-stack" => ScopeType::WasmExpressionStack,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ScopeType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ScopeType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScopeType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "global" => ScopeType::Global,
                    "local" => ScopeType::Local,
                    "with" => ScopeType::With,
                    "closure" => ScopeType::Closure,
                    "catch" => ScopeType::Catch,
                    "block" => ScopeType::Block,
                    "script" => ScopeType::Script,
                    "eval" => ScopeType::Eval,
                    "module" => ScopeType::Module,
                    "wasm-expression-stack" => ScopeType::WasmExpressionStack,
                    _ => ScopeType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ScopeType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.type.Scope。
///
///协议原文：
///
///<pre>Scope description.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct Scope {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.Scope/properties/type。
    ///
    ///协议原文：
    ///
    ///<pre>Scope type.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: ScopeType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.Scope/properties/object。
    ///
    ///协议原文：
    ///
    ///<pre>Object representing the scope. For `global` and `with` scopes it represents the actual
    ///object; for the rest of the scopes, it is artificial transient object enumerating scope
    ///variables as its properties.</pre>
    ///
    ///
    #[serde(rename = "object")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub object: crate::runtime::types::RemoteObject,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.type.Scope/properties/name。
    #[serde(rename = "name")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.type.Scope/properties/startLocation。
    ///
    ///协议原文：
    ///
    ///<pre>Location in the source code where scope starts</pre>
    ///
    ///
    #[serde(rename = "startLocation")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub start_location: Option<crate::debugger::types::Location>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.type.Scope/properties/endLocation。
    ///
    ///协议原文：
    ///
    ///<pre>Location in the source code where scope ends</pre>
    ///
    ///
    #[serde(rename = "endLocation")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub end_location: Option<crate::debugger::types::Location>,
}
impl<'de> serde::Deserialize<'de> for Scope {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Scope;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.Scope/properties/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Scope type.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: ScopeType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.Scope/properties/object。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Object representing the scope. For `global` and `with` scopes it represents the actual
                    ///object; for the rest of the scopes, it is artificial transient object enumerating scope
                    ///variables as its properties.</pre>
                    ///
                    ///
                    #[serde(rename = "object")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub object: crate::runtime::types::RemoteObject,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.type.Scope/properties/name。
                    #[serde(rename = "name")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub name: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.type.Scope/properties/startLocation。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Location in the source code where scope starts</pre>
                    ///
                    ///
                    #[serde(rename = "startLocation")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub start_location: Option<crate::debugger::types::Location>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.type.Scope/properties/endLocation。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Location in the source code where scope ends</pre>
                    ///
                    ///
                    #[serde(rename = "endLocation")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub end_location: Option<crate::debugger::types::Location>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Scope {
                    r#type: fields.r#type,
                    object: fields.object,
                    name: fields.name,
                    start_location: fields.start_location,
                    end_location: fields.end_location,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Debugger.type.ScriptLanguage。
///
///协议原文：
///
///<pre>Enum of possible script languages.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ScriptLanguage {
    ///协议值 "JavaScript"。
    JavaScript,
    ///协议值 "WebAssembly"。
    WebAssembly,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ScriptLanguage {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::debugger::types::ScriptLanguage::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::JavaScript => "JavaScript",
            Self::WebAssembly => "WebAssembly",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ScriptLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ScriptLanguage {
    fn from(value: String) -> Self {
        match value.as_str() {
            "JavaScript" => ScriptLanguage::JavaScript,
            "WebAssembly" => ScriptLanguage::WebAssembly,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ScriptLanguage {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ScriptLanguage {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScriptLanguage;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "JavaScript" => ScriptLanguage::JavaScript,
                    "WebAssembly" => ScriptLanguage::WebAssembly,
                    _ => ScriptLanguage::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ScriptLanguage::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.type.ScriptPosition。
///
///协议原文：
///
///<pre>Location in the source code.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ScriptPosition {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.ScriptPosition/properties/lineNumber。
    #[serde(rename = "lineNumber")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub line_number: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.ScriptPosition/properties/columnNumber。
    #[serde(rename = "columnNumber")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub column_number: i64,
}
impl<'de> serde::Deserialize<'de> for ScriptPosition {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScriptPosition;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.ScriptPosition/properties/lineNumber。
                    #[serde(rename = "lineNumber")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub line_number: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.ScriptPosition/properties/columnNumber。
                    #[serde(rename = "columnNumber")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub column_number: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ScriptPosition {
                    line_number: fields.line_number,
                    column_number: fields.column_number,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.type.SearchMatch。
///
///协议原文：
///
///<pre>Search match for resource.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct SearchMatch {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.SearchMatch/properties/lineNumber。
    ///
    ///协议原文：
    ///
    ///<pre>Line number in resource content.</pre>
    ///
    ///
    #[serde(rename = "lineNumber")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub line_number: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.SearchMatch/properties/lineContent。
    ///
    ///协议原文：
    ///
    ///<pre>Line with match content.</pre>
    ///
    ///
    #[serde(rename = "lineContent")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub line_content: String,
}
impl<'de> serde::Deserialize<'de> for SearchMatch {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SearchMatch;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.SearchMatch/properties/lineNumber。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Line number in resource content.</pre>
                    ///
                    ///
                    #[serde(rename = "lineNumber")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub line_number: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.SearchMatch/properties/lineContent。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Line with match content.</pre>
                    ///
                    ///
                    #[serde(rename = "lineContent")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub line_content: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SearchMatch {
                    line_number: fields.line_number,
                    line_content: fields.line_content,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.type.WasmDisassemblyChunk。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct WasmDisassemblyChunk {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.WasmDisassemblyChunk/properties/lines。
    ///
    ///协议原文：
    ///
    ///<pre>The next chunk of disassembled lines.</pre>
    ///
    ///
    #[serde(rename = "lines")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub lines: Vec<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.WasmDisassemblyChunk/properties/bytecodeOffsets。
    ///
    ///协议原文：
    ///
    ///<pre>The bytecode offsets describing the start of each line.</pre>
    ///
    ///
    #[serde(rename = "bytecodeOffsets")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub bytecode_offsets: Vec<i64>,
}
impl<'de> serde::Deserialize<'de> for WasmDisassemblyChunk {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WasmDisassemblyChunk;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.WasmDisassemblyChunk/properties/lines。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The next chunk of disassembled lines.</pre>
                    ///
                    ///
                    #[serde(rename = "lines")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub lines: Vec<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.type.WasmDisassemblyChunk/properties/bytecodeOffsets。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The bytecode offsets describing the start of each line.</pre>
                    ///
                    ///
                    #[serde(rename = "bytecodeOffsets")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub bytecode_offsets: Vec<i64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(WasmDisassemblyChunk {
                    lines: fields.lines,
                    bytecode_offsets: fields.bytecode_offsets,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
