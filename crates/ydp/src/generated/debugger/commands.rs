///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Debugger.command.continueToLocation/parameters/targetCallFrames。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ContinueToLocationParamsTargetCallFrames {
    ///协议值 "any"。
    Any,
    ///协议值 "current"。
    Current,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ContinueToLocationParamsTargetCallFrames {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::debugger::commands::ContinueToLocationParamsTargetCallFrames::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Any => "any",
            Self::Current => "current",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ContinueToLocationParamsTargetCallFrames {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ContinueToLocationParamsTargetCallFrames {
    fn from(value: String) -> Self {
        match value.as_str() {
            "any" => ContinueToLocationParamsTargetCallFrames::Any,
            "current" => ContinueToLocationParamsTargetCallFrames::Current,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ContinueToLocationParamsTargetCallFrames {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ContinueToLocationParamsTargetCallFrames {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContinueToLocationParamsTargetCallFrames;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "any" => ContinueToLocationParamsTargetCallFrames::Any,
                    "current" => ContinueToLocationParamsTargetCallFrames::Current,
                    _ => ContinueToLocationParamsTargetCallFrames::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ContinueToLocationParamsTargetCallFrames::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.continueToLocation/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ContinueToLocationParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.continueToLocation/parameters/location。
    ///
    ///协议原文：
    ///
    ///<pre>Location to continue to.</pre>
    ///
    ///
    #[serde(rename = "location")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub location: crate::debugger::types::Location,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.continueToLocation/parameters/targetCallFrames。
    #[serde(rename = "targetCallFrames")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_call_frames: Option<ContinueToLocationParamsTargetCallFrames>,
}
impl<'de> serde::Deserialize<'de> for ContinueToLocationParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContinueToLocationParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.continueToLocation/parameters/location。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Location to continue to.</pre>
                    ///
                    ///
                    #[serde(rename = "location")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub location: crate::debugger::types::Location,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.continueToLocation/parameters/targetCallFrames。
                    #[serde(rename = "targetCallFrames")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub target_call_frames: Option<ContinueToLocationParamsTargetCallFrames>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ContinueToLocationParams {
                    location: fields.location,
                    target_call_frames: fields.target_call_frames,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.continueToLocation。
///
///协议原文：
///
///<pre>Continues execution until specific location is reached.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ContinueToLocation;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.continueToLocation/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ContinueToLocationResponse {}
impl<'de> serde::Deserialize<'de> for ContinueToLocationResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContinueToLocationResponse;
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
                Ok(ContinueToLocationResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ContinueToLocation {
    type Params = ContinueToLocationParams;
    type Response = ContinueToLocationResponse;
    const METHOD: &'static str = "Debugger.continueToLocation";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.disable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.disable。
///
///协议原文：
///
///<pre>Disables debugger for given page.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.disable/returns。
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
    const METHOD: &'static str = "Debugger.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.disassembleWasmModule/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DisassembleWasmModuleParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.disassembleWasmModule/parameters/scriptId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the script to disassemble</pre>
    ///
    ///
    #[serde(rename = "scriptId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub script_id: crate::runtime::types::ScriptId,
}
impl<'de> serde::Deserialize<'de> for DisassembleWasmModuleParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DisassembleWasmModuleParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.disassembleWasmModule/parameters/scriptId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the script to disassemble</pre>
                    ///
                    ///
                    #[serde(rename = "scriptId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub script_id: crate::runtime::types::ScriptId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DisassembleWasmModuleParams {
                    script_id: fields.script_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.disassembleWasmModule。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct DisassembleWasmModule;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.disassembleWasmModule/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DisassembleWasmModuleResponse {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.disassembleWasmModule/returns/streamId。
    ///
    ///协议原文：
    ///
    ///<pre>For large modules, return a stream from which additional chunks of
    ///disassembly can be read successively.</pre>
    ///
    ///
    #[serde(rename = "streamId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub stream_id: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.disassembleWasmModule/returns/totalNumberOfLines。
    ///
    ///协议原文：
    ///
    ///<pre>The total number of lines in the disassembly text.</pre>
    ///
    ///
    #[serde(rename = "totalNumberOfLines")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub total_number_of_lines: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.disassembleWasmModule/returns/functionBodyOffsets。
    ///
    ///协议原文：
    ///
    ///<pre>The offsets of all function bodies, in the format [start1, end1,
    ///start2, end2, ...] where all ends are exclusive.</pre>
    ///
    ///
    #[serde(rename = "functionBodyOffsets")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub function_body_offsets: Vec<i64>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.disassembleWasmModule/returns/chunk。
    ///
    ///协议原文：
    ///
    ///<pre>The first chunk of disassembly.</pre>
    ///
    ///
    #[serde(rename = "chunk")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub chunk: crate::debugger::types::WasmDisassemblyChunk,
}
impl<'de> serde::Deserialize<'de> for DisassembleWasmModuleResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DisassembleWasmModuleResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.disassembleWasmModule/returns/streamId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>For large modules, return a stream from which additional chunks of
                    ///disassembly can be read successively.</pre>
                    ///
                    ///
                    #[serde(rename = "streamId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub stream_id: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.disassembleWasmModule/returns/totalNumberOfLines。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The total number of lines in the disassembly text.</pre>
                    ///
                    ///
                    #[serde(rename = "totalNumberOfLines")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub total_number_of_lines: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.disassembleWasmModule/returns/functionBodyOffsets。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The offsets of all function bodies, in the format [start1, end1,
                    ///start2, end2, ...] where all ends are exclusive.</pre>
                    ///
                    ///
                    #[serde(rename = "functionBodyOffsets")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub function_body_offsets: Vec<i64>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.disassembleWasmModule/returns/chunk。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The first chunk of disassembly.</pre>
                    ///
                    ///
                    #[serde(rename = "chunk")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub chunk: crate::debugger::types::WasmDisassemblyChunk,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DisassembleWasmModuleResponse {
                    stream_id: fields.stream_id,
                    total_number_of_lines: fields.total_number_of_lines,
                    function_body_offsets: fields.function_body_offsets,
                    chunk: fields.chunk,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for DisassembleWasmModule {
    type Params = DisassembleWasmModuleParams;
    type Response = DisassembleWasmModuleResponse;
    const METHOD: &'static str = "Debugger.disassembleWasmModule";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.enable/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct EnableParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.enable/parameters/maxScriptsCacheSize。
    ///
    ///协议原文：
    ///
    ///<pre>The maximum size in bytes of collected scripts (not referenced by other heap objects)
    ///the debugger can hold. Puts no limit if parameter is omitted.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "maxScriptsCacheSize")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_scripts_cache_size: Option<ydp_core::FiniteF64>,
}
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
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.enable/parameters/maxScriptsCacheSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The maximum size in bytes of collected scripts (not referenced by other heap objects)
                    ///the debugger can hold. Puts no limit if parameter is omitted.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "maxScriptsCacheSize")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub max_scripts_cache_size: Option<ydp_core::FiniteF64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EnableParams {
                    max_scripts_cache_size: fields.max_scripts_cache_size,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.enable。
///
///协议原文：
///
///<pre>Enables debugger for the given page. Clients should not assume that the debugging has been
///enabled until the result for this command is received.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.enable/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EnableResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.enable/returns/debuggerId。
    ///
    ///协议原文：
    ///
    ///<pre>Unique identifier of the debugger.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "debuggerId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub debugger_id: crate::runtime::types::UniqueDebuggerId,
}
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
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.enable/returns/debuggerId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Unique identifier of the debugger.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "debuggerId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub debugger_id: crate::runtime::types::UniqueDebuggerId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EnableResponse {
                    debugger_id: fields.debugger_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Enable {
    type Params = EnableParams;
    type Response = EnableResponse;
    const METHOD: &'static str = "Debugger.enable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.evaluateOnCallFrame/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EvaluateOnCallFrameParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.evaluateOnCallFrame/parameters/callFrameId。
    ///
    ///协议原文：
    ///
    ///<pre>Call frame identifier to evaluate on.</pre>
    ///
    ///
    #[serde(rename = "callFrameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub call_frame_id: crate::debugger::types::CallFrameId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.evaluateOnCallFrame/parameters/expression。
    ///
    ///协议原文：
    ///
    ///<pre>Expression to evaluate.</pre>
    ///
    ///
    #[serde(rename = "expression")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub expression: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.evaluateOnCallFrame/parameters/objectGroup。
    ///
    ///协议原文：
    ///
    ///<pre>String object group name to put result into (allows rapid releasing resulting object handles
    ///using `releaseObjectGroup`).</pre>
    ///
    ///
    #[serde(rename = "objectGroup")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub object_group: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.evaluateOnCallFrame/parameters/includeCommandLineAPI。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies whether command line API should be available to the evaluated expression, defaults
    ///to false.</pre>
    ///
    ///
    #[serde(rename = "includeCommandLineAPI")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_command_line_api: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.evaluateOnCallFrame/parameters/silent。
    ///
    ///协议原文：
    ///
    ///<pre>In silent mode exceptions thrown during evaluation are not reported and do not pause
    ///execution. Overrides `setPauseOnException` state.</pre>
    ///
    ///
    #[serde(rename = "silent")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub silent: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.evaluateOnCallFrame/parameters/returnByValue。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the result is expected to be a JSON object that should be sent by value.</pre>
    ///
    ///
    #[serde(rename = "returnByValue")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub return_by_value: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.evaluateOnCallFrame/parameters/generatePreview。
    ///
    ///协议原文：
    ///
    ///<pre>Whether preview should be generated for the result.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "generatePreview")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub generate_preview: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.evaluateOnCallFrame/parameters/throwOnSideEffect。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to throw an exception if side effect cannot be ruled out during evaluation.</pre>
    ///
    ///
    #[serde(rename = "throwOnSideEffect")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub throw_on_side_effect: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.evaluateOnCallFrame/parameters/timeout。
    ///
    ///协议原文：
    ///
    ///<pre>Terminate execution after timing out (number of milliseconds).</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "timeout")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub timeout: Option<crate::runtime::types::TimeDelta>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.evaluateOnCallFrame/parameters/scopeNumber。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies the scope number to evaluate the expression in (default: 0, innermost scope).</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "scopeNumber")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub scope_number: Option<i64>,
}
impl<'de> serde::Deserialize<'de> for EvaluateOnCallFrameParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EvaluateOnCallFrameParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.evaluateOnCallFrame/parameters/callFrameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Call frame identifier to evaluate on.</pre>
                    ///
                    ///
                    #[serde(rename = "callFrameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub call_frame_id: crate::debugger::types::CallFrameId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.evaluateOnCallFrame/parameters/expression。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Expression to evaluate.</pre>
                    ///
                    ///
                    #[serde(rename = "expression")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub expression: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.evaluateOnCallFrame/parameters/objectGroup。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>String object group name to put result into (allows rapid releasing resulting object handles
                    ///using `releaseObjectGroup`).</pre>
                    ///
                    ///
                    #[serde(rename = "objectGroup")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub object_group: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.evaluateOnCallFrame/parameters/includeCommandLineAPI。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies whether command line API should be available to the evaluated expression, defaults
                    ///to false.</pre>
                    ///
                    ///
                    #[serde(rename = "includeCommandLineAPI")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub include_command_line_api: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.evaluateOnCallFrame/parameters/silent。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>In silent mode exceptions thrown during evaluation are not reported and do not pause
                    ///execution. Overrides `setPauseOnException` state.</pre>
                    ///
                    ///
                    #[serde(rename = "silent")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub silent: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.evaluateOnCallFrame/parameters/returnByValue。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the result is expected to be a JSON object that should be sent by value.</pre>
                    ///
                    ///
                    #[serde(rename = "returnByValue")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub return_by_value: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.evaluateOnCallFrame/parameters/generatePreview。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether preview should be generated for the result.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "generatePreview")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub generate_preview: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.evaluateOnCallFrame/parameters/throwOnSideEffect。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to throw an exception if side effect cannot be ruled out during evaluation.</pre>
                    ///
                    ///
                    #[serde(rename = "throwOnSideEffect")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub throw_on_side_effect: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.evaluateOnCallFrame/parameters/timeout。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Terminate execution after timing out (number of milliseconds).</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "timeout")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub timeout: Option<crate::runtime::types::TimeDelta>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.evaluateOnCallFrame/parameters/scopeNumber。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies the scope number to evaluate the expression in (default: 0, innermost scope).</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "scopeNumber")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub scope_number: Option<i64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EvaluateOnCallFrameParams {
                    call_frame_id: fields.call_frame_id,
                    expression: fields.expression,
                    object_group: fields.object_group,
                    include_command_line_api: fields.include_command_line_api,
                    silent: fields.silent,
                    return_by_value: fields.return_by_value,
                    generate_preview: fields.generate_preview,
                    throw_on_side_effect: fields.throw_on_side_effect,
                    timeout: fields.timeout,
                    scope_number: fields.scope_number,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.evaluateOnCallFrame。
///
///协议原文：
///
///<pre>Evaluates expression on a given call frame.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct EvaluateOnCallFrame;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.evaluateOnCallFrame/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EvaluateOnCallFrameResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.evaluateOnCallFrame/returns/result。
    ///
    ///协议原文：
    ///
    ///<pre>Object wrapper for the evaluation result.</pre>
    ///
    ///
    #[serde(rename = "result")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub result: crate::runtime::types::RemoteObject,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.evaluateOnCallFrame/returns/exceptionDetails。
    ///
    ///协议原文：
    ///
    ///<pre>Exception details.</pre>
    ///
    ///
    #[serde(rename = "exceptionDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub exception_details: Option<crate::runtime::types::ExceptionDetails>,
}
impl<'de> serde::Deserialize<'de> for EvaluateOnCallFrameResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EvaluateOnCallFrameResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.evaluateOnCallFrame/returns/result。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Object wrapper for the evaluation result.</pre>
                    ///
                    ///
                    #[serde(rename = "result")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub result: crate::runtime::types::RemoteObject,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.evaluateOnCallFrame/returns/exceptionDetails。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Exception details.</pre>
                    ///
                    ///
                    #[serde(rename = "exceptionDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub exception_details: Option<crate::runtime::types::ExceptionDetails>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EvaluateOnCallFrameResponse {
                    result: fields.result,
                    exception_details: fields.exception_details,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for EvaluateOnCallFrame {
    type Params = EvaluateOnCallFrameParams;
    type Response = EvaluateOnCallFrameResponse;
    const METHOD: &'static str = "Debugger.evaluateOnCallFrame";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.getPossibleBreakpoints/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetPossibleBreakpointsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.getPossibleBreakpoints/parameters/start。
    ///
    ///协议原文：
    ///
    ///<pre>Start of range to search possible breakpoint locations in.</pre>
    ///
    ///
    #[serde(rename = "start")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub start: crate::debugger::types::Location,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.getPossibleBreakpoints/parameters/end。
    ///
    ///协议原文：
    ///
    ///<pre>End of range to search possible breakpoint locations in (excluding). When not specified, end
    ///of scripts is used as end of range.</pre>
    ///
    ///
    #[serde(rename = "end")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub end: Option<crate::debugger::types::Location>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.getPossibleBreakpoints/parameters/restrictToFunction。
    ///
    ///协议原文：
    ///
    ///<pre>Only consider locations which are in the same (non-nested) function as start.</pre>
    ///
    ///
    #[serde(rename = "restrictToFunction")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub restrict_to_function: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for GetPossibleBreakpointsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetPossibleBreakpointsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.getPossibleBreakpoints/parameters/start。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Start of range to search possible breakpoint locations in.</pre>
                    ///
                    ///
                    #[serde(rename = "start")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub start: crate::debugger::types::Location,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.getPossibleBreakpoints/parameters/end。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>End of range to search possible breakpoint locations in (excluding). When not specified, end
                    ///of scripts is used as end of range.</pre>
                    ///
                    ///
                    #[serde(rename = "end")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub end: Option<crate::debugger::types::Location>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.getPossibleBreakpoints/parameters/restrictToFunction。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Only consider locations which are in the same (non-nested) function as start.</pre>
                    ///
                    ///
                    #[serde(rename = "restrictToFunction")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub restrict_to_function: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetPossibleBreakpointsParams {
                    start: fields.start,
                    end: fields.end,
                    restrict_to_function: fields.restrict_to_function,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.getPossibleBreakpoints。
///
///协议原文：
///
///<pre>Returns possible locations for breakpoint. scriptId in start and end range locations should be
///the same.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetPossibleBreakpoints;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.getPossibleBreakpoints/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetPossibleBreakpointsResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.getPossibleBreakpoints/returns/locations。
    ///
    ///协议原文：
    ///
    ///<pre>List of the possible breakpoint locations.</pre>
    ///
    ///
    #[serde(rename = "locations")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub locations: Vec<crate::debugger::types::BreakLocation>,
}
impl<'de> serde::Deserialize<'de> for GetPossibleBreakpointsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetPossibleBreakpointsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.getPossibleBreakpoints/returns/locations。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>List of the possible breakpoint locations.</pre>
                    ///
                    ///
                    #[serde(rename = "locations")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub locations: Vec<crate::debugger::types::BreakLocation>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetPossibleBreakpointsResponse {
                    locations: fields.locations,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetPossibleBreakpoints {
    type Params = GetPossibleBreakpointsParams;
    type Response = GetPossibleBreakpointsResponse;
    const METHOD: &'static str = "Debugger.getPossibleBreakpoints";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.getScriptSource/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetScriptSourceParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.getScriptSource/parameters/scriptId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the script to get source for.</pre>
    ///
    ///
    #[serde(rename = "scriptId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub script_id: crate::runtime::types::ScriptId,
}
impl<'de> serde::Deserialize<'de> for GetScriptSourceParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetScriptSourceParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.getScriptSource/parameters/scriptId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the script to get source for.</pre>
                    ///
                    ///
                    #[serde(rename = "scriptId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub script_id: crate::runtime::types::ScriptId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetScriptSourceParams {
                    script_id: fields.script_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.getScriptSource。
///
///协议原文：
///
///<pre>Returns source for the script with given id.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetScriptSource;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.getScriptSource/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetScriptSourceResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.getScriptSource/returns/scriptSource。
    ///
    ///协议原文：
    ///
    ///<pre>Script source (empty in case of Wasm bytecode).</pre>
    ///
    ///
    #[serde(rename = "scriptSource")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub script_source: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.getScriptSource/returns/bytecode。
    ///
    ///协议原文：
    ///
    ///<pre>Wasm bytecode. (Encoded as a base64 string when passed over JSON)</pre>
    ///
    ///
    #[serde(rename = "bytecode")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub bytecode: Option<String>,
}
impl<'de> serde::Deserialize<'de> for GetScriptSourceResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetScriptSourceResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.getScriptSource/returns/scriptSource。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Script source (empty in case of Wasm bytecode).</pre>
                    ///
                    ///
                    #[serde(rename = "scriptSource")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub script_source: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.getScriptSource/returns/bytecode。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Wasm bytecode. (Encoded as a base64 string when passed over JSON)</pre>
                    ///
                    ///
                    #[serde(rename = "bytecode")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub bytecode: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetScriptSourceResponse {
                    script_source: fields.script_source,
                    bytecode: fields.bytecode,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetScriptSource {
    type Params = GetScriptSourceParams;
    type Response = GetScriptSourceResponse;
    const METHOD: &'static str = "Debugger.getScriptSource";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.getStackTrace/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetStackTraceParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.getStackTrace/parameters/stackTraceId。
    #[serde(rename = "stackTraceId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub stack_trace_id: crate::runtime::types::StackTraceId,
}
impl<'de> serde::Deserialize<'de> for GetStackTraceParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetStackTraceParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.getStackTrace/parameters/stackTraceId。
                    #[serde(rename = "stackTraceId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub stack_trace_id: crate::runtime::types::StackTraceId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetStackTraceParams {
                    stack_trace_id: fields.stack_trace_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.getStackTrace。
///
///协议原文：
///
///<pre>Returns stack trace with given `stackTraceId`.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetStackTrace;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.getStackTrace/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetStackTraceResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.getStackTrace/returns/stackTrace。
    #[serde(rename = "stackTrace")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub stack_trace: crate::runtime::types::StackTrace,
}
impl<'de> serde::Deserialize<'de> for GetStackTraceResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetStackTraceResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.getStackTrace/returns/stackTrace。
                    #[serde(rename = "stackTrace")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub stack_trace: crate::runtime::types::StackTrace,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetStackTraceResponse {
                    stack_trace: fields.stack_trace,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetStackTrace {
    type Params = GetStackTraceParams;
    type Response = GetStackTraceResponse;
    const METHOD: &'static str = "Debugger.getStackTrace";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.getWasmBytecode/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetWasmBytecodeParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.getWasmBytecode/parameters/scriptId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the Wasm script to get source for.</pre>
    ///
    ///
    #[serde(rename = "scriptId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub script_id: crate::runtime::types::ScriptId,
}
impl<'de> serde::Deserialize<'de> for GetWasmBytecodeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetWasmBytecodeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.getWasmBytecode/parameters/scriptId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the Wasm script to get source for.</pre>
                    ///
                    ///
                    #[serde(rename = "scriptId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub script_id: crate::runtime::types::ScriptId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetWasmBytecodeParams {
                    script_id: fields.script_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.getWasmBytecode。
///
///协议原文：
///
///<pre>This command is deprecated. Use getScriptSource instead.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct GetWasmBytecode;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.getWasmBytecode/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetWasmBytecodeResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.getWasmBytecode/returns/bytecode。
    ///
    ///协议原文：
    ///
    ///<pre>Script source. (Encoded as a base64 string when passed over JSON)</pre>
    ///
    ///
    #[serde(rename = "bytecode")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub bytecode: String,
}
impl<'de> serde::Deserialize<'de> for GetWasmBytecodeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetWasmBytecodeResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.getWasmBytecode/returns/bytecode。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Script source. (Encoded as a base64 string when passed over JSON)</pre>
                    ///
                    ///
                    #[serde(rename = "bytecode")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub bytecode: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetWasmBytecodeResponse {
                    bytecode: fields.bytecode,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetWasmBytecode {
    type Params = GetWasmBytecodeParams;
    type Response = GetWasmBytecodeResponse;
    const METHOD: &'static str = "Debugger.getWasmBytecode";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.nextWasmDisassemblyChunk/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct NextWasmDisassemblyChunkParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.nextWasmDisassemblyChunk/parameters/streamId。
    #[serde(rename = "streamId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub stream_id: String,
}
impl<'de> serde::Deserialize<'de> for NextWasmDisassemblyChunkParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NextWasmDisassemblyChunkParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.nextWasmDisassemblyChunk/parameters/streamId。
                    #[serde(rename = "streamId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub stream_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(NextWasmDisassemblyChunkParams {
                    stream_id: fields.stream_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.nextWasmDisassemblyChunk。
///
///协议原文：
///
///<pre>Disassemble the next chunk of lines for the module corresponding to the
///stream. If disassembly is complete, this API will invalidate the streamId
///and return an empty chunk. Any subsequent calls for the now invalid stream
///will return errors.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct NextWasmDisassemblyChunk;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.nextWasmDisassemblyChunk/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct NextWasmDisassemblyChunkResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.nextWasmDisassemblyChunk/returns/chunk。
    ///
    ///协议原文：
    ///
    ///<pre>The next chunk of disassembly.</pre>
    ///
    ///
    #[serde(rename = "chunk")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub chunk: crate::debugger::types::WasmDisassemblyChunk,
}
impl<'de> serde::Deserialize<'de> for NextWasmDisassemblyChunkResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NextWasmDisassemblyChunkResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.nextWasmDisassemblyChunk/returns/chunk。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The next chunk of disassembly.</pre>
                    ///
                    ///
                    #[serde(rename = "chunk")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub chunk: crate::debugger::types::WasmDisassemblyChunk,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(NextWasmDisassemblyChunkResponse {
                    chunk: fields.chunk,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for NextWasmDisassemblyChunk {
    type Params = NextWasmDisassemblyChunkParams;
    type Response = NextWasmDisassemblyChunkResponse;
    const METHOD: &'static str = "Debugger.nextWasmDisassemblyChunk";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.pause/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct PauseParams {}
impl<'de> serde::Deserialize<'de> for PauseParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PauseParams;
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
                Ok(PauseParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.pause。
///
///协议原文：
///
///<pre>Stops on the next JavaScript statement.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Pause;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.pause/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PauseResponse {}
impl<'de> serde::Deserialize<'de> for PauseResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PauseResponse;
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
                Ok(PauseResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Pause {
    type Params = PauseParams;
    type Response = PauseResponse;
    const METHOD: &'static str = "Debugger.pause";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.pauseOnAsyncCall/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PauseOnAsyncCallParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.pauseOnAsyncCall/parameters/parentStackTraceId。
    ///
    ///协议原文：
    ///
    ///<pre>Debugger will pause when async call with given stack trace is started.</pre>
    ///
    ///
    #[serde(rename = "parentStackTraceId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub parent_stack_trace_id: crate::runtime::types::StackTraceId,
}
impl<'de> serde::Deserialize<'de> for PauseOnAsyncCallParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PauseOnAsyncCallParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.pauseOnAsyncCall/parameters/parentStackTraceId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Debugger will pause when async call with given stack trace is started.</pre>
                    ///
                    ///
                    #[serde(rename = "parentStackTraceId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub parent_stack_trace_id: crate::runtime::types::StackTraceId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PauseOnAsyncCallParams {
                    parent_stack_trace_id: fields.parent_stack_trace_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.pauseOnAsyncCall。此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct PauseOnAsyncCall;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.pauseOnAsyncCall/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PauseOnAsyncCallResponse {}
impl<'de> serde::Deserialize<'de> for PauseOnAsyncCallResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PauseOnAsyncCallResponse;
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
                Ok(PauseOnAsyncCallResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for PauseOnAsyncCall {
    type Params = PauseOnAsyncCallParams;
    type Response = PauseOnAsyncCallResponse;
    const METHOD: &'static str = "Debugger.pauseOnAsyncCall";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.removeBreakpoint/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveBreakpointParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.removeBreakpoint/parameters/breakpointId。
    #[serde(rename = "breakpointId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub breakpoint_id: crate::debugger::types::BreakpointId,
}
impl<'de> serde::Deserialize<'de> for RemoveBreakpointParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveBreakpointParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.removeBreakpoint/parameters/breakpointId。
                    #[serde(rename = "breakpointId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub breakpoint_id: crate::debugger::types::BreakpointId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RemoveBreakpointParams {
                    breakpoint_id: fields.breakpoint_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.removeBreakpoint。
///
///协议原文：
///
///<pre>Removes JavaScript breakpoint.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RemoveBreakpoint;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.removeBreakpoint/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveBreakpointResponse {}
impl<'de> serde::Deserialize<'de> for RemoveBreakpointResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveBreakpointResponse;
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
                Ok(RemoveBreakpointResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RemoveBreakpoint {
    type Params = RemoveBreakpointParams;
    type Response = RemoveBreakpointResponse;
    const METHOD: &'static str = "Debugger.removeBreakpoint";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Debugger.command.restartFrame/parameters/mode。
///
///协议原文：
///
///<pre>The `mode` parameter must be present and set to 'StepInto', otherwise
///`restartFrame` will error out.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RestartFrameParamsMode {
    ///协议值 "StepInto"。
    StepInto,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl RestartFrameParamsMode {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::debugger::commands::RestartFrameParamsMode::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::StepInto => "StepInto",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for RestartFrameParamsMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for RestartFrameParamsMode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "StepInto" => RestartFrameParamsMode::StepInto,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for RestartFrameParamsMode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RestartFrameParamsMode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RestartFrameParamsMode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "StepInto" => RestartFrameParamsMode::StepInto,
                    _ => RestartFrameParamsMode::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(RestartFrameParamsMode::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.restartFrame/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RestartFrameParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.restartFrame/parameters/callFrameId。
    ///
    ///协议原文：
    ///
    ///<pre>Call frame identifier to evaluate on.</pre>
    ///
    ///
    #[serde(rename = "callFrameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub call_frame_id: crate::debugger::types::CallFrameId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.restartFrame/parameters/mode。
    ///
    ///协议原文：
    ///
    ///<pre>The `mode` parameter must be present and set to 'StepInto', otherwise
    ///`restartFrame` will error out.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "mode")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub mode: Option<RestartFrameParamsMode>,
}
impl<'de> serde::Deserialize<'de> for RestartFrameParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RestartFrameParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.restartFrame/parameters/callFrameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Call frame identifier to evaluate on.</pre>
                    ///
                    ///
                    #[serde(rename = "callFrameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub call_frame_id: crate::debugger::types::CallFrameId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.restartFrame/parameters/mode。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The `mode` parameter must be present and set to 'StepInto', otherwise
                    ///`restartFrame` will error out.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "mode")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub mode: Option<RestartFrameParamsMode>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RestartFrameParams {
                    call_frame_id: fields.call_frame_id,
                    mode: fields.mode,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.restartFrame。
///
///协议原文：
///
///<pre>Restarts particular call frame from the beginning. The old, deprecated
///behavior of `restartFrame` is to stay paused and allow further CDP commands
///after a restart was scheduled. This can cause problems with restarting, so
///we now continue execution immediatly after it has been scheduled until we
///reach the beginning of the restarted frame.
///
///To stay back-wards compatible, `restartFrame` now expects a `mode`
///parameter to be present. If the `mode` parameter is missing, `restartFrame`
///errors out.
///
///The various return values are deprecated and `callFrames` is always empty.
///Use the call frames from the `Debugger#paused` events instead, that fires
///once V8 pauses at the beginning of the restarted function.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RestartFrame;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.restartFrame/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RestartFrameResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.restartFrame/returns/callFrames。
    ///
    ///协议原文：
    ///
    ///<pre>New stack trace.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "callFrames")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub call_frames: Vec<crate::debugger::types::CallFrame>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.restartFrame/returns/asyncStackTrace。
    ///
    ///协议原文：
    ///
    ///<pre>Async stack trace, if any.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "asyncStackTrace")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub async_stack_trace: Option<crate::runtime::types::StackTrace>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.restartFrame/returns/asyncStackTraceId。
    ///
    ///协议原文：
    ///
    ///<pre>Async stack trace, if any.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "asyncStackTraceId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub async_stack_trace_id: Option<crate::runtime::types::StackTraceId>,
}
impl<'de> serde::Deserialize<'de> for RestartFrameResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RestartFrameResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.restartFrame/returns/callFrames。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>New stack trace.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "callFrames")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub call_frames: Vec<crate::debugger::types::CallFrame>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.restartFrame/returns/asyncStackTrace。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Async stack trace, if any.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "asyncStackTrace")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub async_stack_trace: Option<crate::runtime::types::StackTrace>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.restartFrame/returns/asyncStackTraceId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Async stack trace, if any.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "asyncStackTraceId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub async_stack_trace_id: Option<crate::runtime::types::StackTraceId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RestartFrameResponse {
                    call_frames: fields.call_frames,
                    async_stack_trace: fields.async_stack_trace,
                    async_stack_trace_id: fields.async_stack_trace_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RestartFrame {
    type Params = RestartFrameParams;
    type Response = RestartFrameResponse;
    const METHOD: &'static str = "Debugger.restartFrame";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.resume/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ResumeParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.resume/parameters/terminateOnResume。
    ///
    ///协议原文：
    ///
    ///<pre>Set to true to terminate execution upon resuming execution. In contrast
    ///to Runtime.terminateExecution, this will allows to execute further
    ///JavaScript (i.e. via evaluation) until execution of the paused code
    ///is actually resumed, at which point termination is triggered.
    ///If execution is currently not paused, this parameter has no effect.</pre>
    ///
    ///
    #[serde(rename = "terminateOnResume")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub terminate_on_resume: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for ResumeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResumeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.resume/parameters/terminateOnResume。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Set to true to terminate execution upon resuming execution. In contrast
                    ///to Runtime.terminateExecution, this will allows to execute further
                    ///JavaScript (i.e. via evaluation) until execution of the paused code
                    ///is actually resumed, at which point termination is triggered.
                    ///If execution is currently not paused, this parameter has no effect.</pre>
                    ///
                    ///
                    #[serde(rename = "terminateOnResume")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub terminate_on_resume: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ResumeParams {
                    terminate_on_resume: fields.terminate_on_resume,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.resume。
///
///协议原文：
///
///<pre>Resumes JavaScript execution.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Resume;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.resume/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ResumeResponse {}
impl<'de> serde::Deserialize<'de> for ResumeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResumeResponse;
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
                Ok(ResumeResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Resume {
    type Params = ResumeParams;
    type Response = ResumeResponse;
    const METHOD: &'static str = "Debugger.resume";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.searchInContent/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SearchInContentParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.searchInContent/parameters/scriptId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the script to search in.</pre>
    ///
    ///
    #[serde(rename = "scriptId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub script_id: crate::runtime::types::ScriptId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.searchInContent/parameters/query。
    ///
    ///协议原文：
    ///
    ///<pre>String to search for.</pre>
    ///
    ///
    #[serde(rename = "query")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub query: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.searchInContent/parameters/caseSensitive。
    ///
    ///协议原文：
    ///
    ///<pre>If true, search is case sensitive.</pre>
    ///
    ///
    #[serde(rename = "caseSensitive")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub case_sensitive: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.searchInContent/parameters/isRegex。
    ///
    ///协议原文：
    ///
    ///<pre>If true, treats string parameter as regex.</pre>
    ///
    ///
    #[serde(rename = "isRegex")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_regex: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for SearchInContentParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SearchInContentParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.searchInContent/parameters/scriptId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the script to search in.</pre>
                    ///
                    ///
                    #[serde(rename = "scriptId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub script_id: crate::runtime::types::ScriptId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.searchInContent/parameters/query。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>String to search for.</pre>
                    ///
                    ///
                    #[serde(rename = "query")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub query: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.searchInContent/parameters/caseSensitive。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, search is case sensitive.</pre>
                    ///
                    ///
                    #[serde(rename = "caseSensitive")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub case_sensitive: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.searchInContent/parameters/isRegex。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, treats string parameter as regex.</pre>
                    ///
                    ///
                    #[serde(rename = "isRegex")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub is_regex: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SearchInContentParams {
                    script_id: fields.script_id,
                    query: fields.query,
                    case_sensitive: fields.case_sensitive,
                    is_regex: fields.is_regex,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.searchInContent。
///
///协议原文：
///
///<pre>Searches for given string in script content.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SearchInContent;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.searchInContent/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SearchInContentResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.searchInContent/returns/result。
    ///
    ///协议原文：
    ///
    ///<pre>List of search matches.</pre>
    ///
    ///
    #[serde(rename = "result")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub result: Vec<crate::debugger::types::SearchMatch>,
}
impl<'de> serde::Deserialize<'de> for SearchInContentResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SearchInContentResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.searchInContent/returns/result。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>List of search matches.</pre>
                    ///
                    ///
                    #[serde(rename = "result")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub result: Vec<crate::debugger::types::SearchMatch>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SearchInContentResponse {
                    result: fields.result,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SearchInContent {
    type Params = SearchInContentParams;
    type Response = SearchInContentResponse;
    const METHOD: &'static str = "Debugger.searchInContent";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.setAsyncCallStackDepth/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetAsyncCallStackDepthParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setAsyncCallStackDepth/parameters/maxDepth。
    ///
    ///协议原文：
    ///
    ///<pre>Maximum depth of async call stacks. Setting to `0` will effectively disable collecting async
    ///call stacks (default).</pre>
    ///
    ///
    #[serde(rename = "maxDepth")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub max_depth: i64,
}
impl<'de> serde::Deserialize<'de> for SetAsyncCallStackDepthParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetAsyncCallStackDepthParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setAsyncCallStackDepth/parameters/maxDepth。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Maximum depth of async call stacks. Setting to `0` will effectively disable collecting async
                    ///call stacks (default).</pre>
                    ///
                    ///
                    #[serde(rename = "maxDepth")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub max_depth: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetAsyncCallStackDepthParams {
                    max_depth: fields.max_depth,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.setAsyncCallStackDepth。
///
///协议原文：
///
///<pre>Enables or disables async call stacks tracking.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetAsyncCallStackDepth;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.setAsyncCallStackDepth/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetAsyncCallStackDepthResponse {}
impl<'de> serde::Deserialize<'de> for SetAsyncCallStackDepthResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetAsyncCallStackDepthResponse;
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
                Ok(SetAsyncCallStackDepthResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetAsyncCallStackDepth {
    type Params = SetAsyncCallStackDepthParams;
    type Response = SetAsyncCallStackDepthResponse;
    const METHOD: &'static str = "Debugger.setAsyncCallStackDepth";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.setBlackboxExecutionContexts/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetBlackboxExecutionContextsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setBlackboxExecutionContexts/parameters/uniqueIds。
    ///
    ///协议原文：
    ///
    ///<pre>Array of execution context unique ids for the debugger to ignore.</pre>
    ///
    ///
    #[serde(rename = "uniqueIds")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub unique_ids: Vec<String>,
}
impl<'de> serde::Deserialize<'de> for SetBlackboxExecutionContextsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetBlackboxExecutionContextsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setBlackboxExecutionContexts/parameters/uniqueIds。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Array of execution context unique ids for the debugger to ignore.</pre>
                    ///
                    ///
                    #[serde(rename = "uniqueIds")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub unique_ids: Vec<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetBlackboxExecutionContextsParams {
                    unique_ids: fields.unique_ids,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.setBlackboxExecutionContexts。
///
///协议原文：
///
///<pre>Replace previous blackbox execution contexts with passed ones. Forces backend to skip
///stepping/pausing in scripts in these execution contexts. VM will try to leave blackboxed script by
///performing 'step in' several times, finally resorting to 'step out' if unsuccessful.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetBlackboxExecutionContexts;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.setBlackboxExecutionContexts/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetBlackboxExecutionContextsResponse {}
impl<'de> serde::Deserialize<'de> for SetBlackboxExecutionContextsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetBlackboxExecutionContextsResponse;
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
                Ok(SetBlackboxExecutionContextsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetBlackboxExecutionContexts {
    type Params = SetBlackboxExecutionContextsParams;
    type Response = SetBlackboxExecutionContextsResponse;
    const METHOD: &'static str = "Debugger.setBlackboxExecutionContexts";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.setBlackboxPatterns/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetBlackboxPatternsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setBlackboxPatterns/parameters/patterns。
    ///
    ///协议原文：
    ///
    ///<pre>Array of regexps that will be used to check script url for blackbox state.</pre>
    ///
    ///
    #[serde(rename = "patterns")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub patterns: Vec<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setBlackboxPatterns/parameters/skipAnonymous。
    ///
    ///协议原文：
    ///
    ///<pre>If true, also ignore scripts with no source url.</pre>
    ///
    ///
    #[serde(rename = "skipAnonymous")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub skip_anonymous: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for SetBlackboxPatternsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetBlackboxPatternsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setBlackboxPatterns/parameters/patterns。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Array of regexps that will be used to check script url for blackbox state.</pre>
                    ///
                    ///
                    #[serde(rename = "patterns")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub patterns: Vec<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setBlackboxPatterns/parameters/skipAnonymous。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, also ignore scripts with no source url.</pre>
                    ///
                    ///
                    #[serde(rename = "skipAnonymous")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub skip_anonymous: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetBlackboxPatternsParams {
                    patterns: fields.patterns,
                    skip_anonymous: fields.skip_anonymous,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.setBlackboxPatterns。
///
///协议原文：
///
///<pre>Replace previous blackbox patterns with passed ones. Forces backend to skip stepping/pausing in
///scripts with url matching one of the patterns. VM will try to leave blackboxed script by
///performing 'step in' several times, finally resorting to 'step out' if unsuccessful.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetBlackboxPatterns;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.setBlackboxPatterns/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetBlackboxPatternsResponse {}
impl<'de> serde::Deserialize<'de> for SetBlackboxPatternsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetBlackboxPatternsResponse;
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
                Ok(SetBlackboxPatternsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetBlackboxPatterns {
    type Params = SetBlackboxPatternsParams;
    type Response = SetBlackboxPatternsResponse;
    const METHOD: &'static str = "Debugger.setBlackboxPatterns";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.setBlackboxedRanges/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetBlackboxedRangesParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setBlackboxedRanges/parameters/scriptId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the script.</pre>
    ///
    ///
    #[serde(rename = "scriptId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub script_id: crate::runtime::types::ScriptId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setBlackboxedRanges/parameters/positions。
    #[serde(rename = "positions")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub positions: Vec<crate::debugger::types::ScriptPosition>,
}
impl<'de> serde::Deserialize<'de> for SetBlackboxedRangesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetBlackboxedRangesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setBlackboxedRanges/parameters/scriptId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the script.</pre>
                    ///
                    ///
                    #[serde(rename = "scriptId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub script_id: crate::runtime::types::ScriptId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setBlackboxedRanges/parameters/positions。
                    #[serde(rename = "positions")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub positions: Vec<crate::debugger::types::ScriptPosition>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetBlackboxedRangesParams {
                    script_id: fields.script_id,
                    positions: fields.positions,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.setBlackboxedRanges。
///
///协议原文：
///
///<pre>Makes backend skip steps in the script in blackboxed ranges. VM will try leave blacklisted
///scripts by performing 'step in' several times, finally resorting to 'step out' if unsuccessful.
///Positions array contains positions where blackbox state is changed. First interval isn't
///blackboxed. Array should be sorted.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetBlackboxedRanges;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.setBlackboxedRanges/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetBlackboxedRangesResponse {}
impl<'de> serde::Deserialize<'de> for SetBlackboxedRangesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetBlackboxedRangesResponse;
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
                Ok(SetBlackboxedRangesResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetBlackboxedRanges {
    type Params = SetBlackboxedRangesParams;
    type Response = SetBlackboxedRangesResponse;
    const METHOD: &'static str = "Debugger.setBlackboxedRanges";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.setBreakpoint/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetBreakpointParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setBreakpoint/parameters/location。
    ///
    ///协议原文：
    ///
    ///<pre>Location to set breakpoint in.</pre>
    ///
    ///
    #[serde(rename = "location")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub location: crate::debugger::types::Location,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setBreakpoint/parameters/condition。
    ///
    ///协议原文：
    ///
    ///<pre>Expression to use as a breakpoint condition. When specified, debugger will only stop on the
    ///breakpoint if this expression evaluates to true.</pre>
    ///
    ///
    #[serde(rename = "condition")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub condition: Option<String>,
}
impl<'de> serde::Deserialize<'de> for SetBreakpointParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetBreakpointParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setBreakpoint/parameters/location。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Location to set breakpoint in.</pre>
                    ///
                    ///
                    #[serde(rename = "location")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub location: crate::debugger::types::Location,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setBreakpoint/parameters/condition。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Expression to use as a breakpoint condition. When specified, debugger will only stop on the
                    ///breakpoint if this expression evaluates to true.</pre>
                    ///
                    ///
                    #[serde(rename = "condition")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub condition: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetBreakpointParams {
                    location: fields.location,
                    condition: fields.condition,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.setBreakpoint。
///
///协议原文：
///
///<pre>Sets JavaScript breakpoint at a given location.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetBreakpoint;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.setBreakpoint/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetBreakpointResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setBreakpoint/returns/breakpointId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the created breakpoint for further reference.</pre>
    ///
    ///
    #[serde(rename = "breakpointId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub breakpoint_id: crate::debugger::types::BreakpointId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setBreakpoint/returns/actualLocation。
    ///
    ///协议原文：
    ///
    ///<pre>Location this breakpoint resolved into.</pre>
    ///
    ///
    #[serde(rename = "actualLocation")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub actual_location: crate::debugger::types::Location,
}
impl<'de> serde::Deserialize<'de> for SetBreakpointResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetBreakpointResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setBreakpoint/returns/breakpointId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the created breakpoint for further reference.</pre>
                    ///
                    ///
                    #[serde(rename = "breakpointId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub breakpoint_id: crate::debugger::types::BreakpointId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setBreakpoint/returns/actualLocation。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Location this breakpoint resolved into.</pre>
                    ///
                    ///
                    #[serde(rename = "actualLocation")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub actual_location: crate::debugger::types::Location,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetBreakpointResponse {
                    breakpoint_id: fields.breakpoint_id,
                    actual_location: fields.actual_location,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetBreakpoint {
    type Params = SetBreakpointParams;
    type Response = SetBreakpointResponse;
    const METHOD: &'static str = "Debugger.setBreakpoint";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.setBreakpointByUrl/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetBreakpointByUrlParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setBreakpointByUrl/parameters/lineNumber。
    ///
    ///协议原文：
    ///
    ///<pre>Line number to set breakpoint at.</pre>
    ///
    ///
    #[serde(rename = "lineNumber")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub line_number: i64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setBreakpointByUrl/parameters/url。
    ///
    ///协议原文：
    ///
    ///<pre>URL of the resources to set breakpoint on.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setBreakpointByUrl/parameters/urlRegex。
    ///
    ///协议原文：
    ///
    ///<pre>Regex pattern for the URLs of the resources to set breakpoints on. Either `url` or
    ///`urlRegex` must be specified.</pre>
    ///
    ///
    #[serde(rename = "urlRegex")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub url_regex: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setBreakpointByUrl/parameters/scriptHash。
    ///
    ///协议原文：
    ///
    ///<pre>Script hash of the resources to set breakpoint on.</pre>
    ///
    ///
    #[serde(rename = "scriptHash")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub script_hash: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setBreakpointByUrl/parameters/columnNumber。
    ///
    ///协议原文：
    ///
    ///<pre>Offset in the line to set breakpoint at.</pre>
    ///
    ///
    #[serde(rename = "columnNumber")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub column_number: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setBreakpointByUrl/parameters/condition。
    ///
    ///协议原文：
    ///
    ///<pre>Expression to use as a breakpoint condition. When specified, debugger will only stop on the
    ///breakpoint if this expression evaluates to true.</pre>
    ///
    ///
    #[serde(rename = "condition")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub condition: Option<String>,
}
impl<'de> serde::Deserialize<'de> for SetBreakpointByUrlParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetBreakpointByUrlParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setBreakpointByUrl/parameters/lineNumber。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Line number to set breakpoint at.</pre>
                    ///
                    ///
                    #[serde(rename = "lineNumber")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub line_number: i64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setBreakpointByUrl/parameters/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>URL of the resources to set breakpoint on.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub url: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setBreakpointByUrl/parameters/urlRegex。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Regex pattern for the URLs of the resources to set breakpoints on. Either `url` or
                    ///`urlRegex` must be specified.</pre>
                    ///
                    ///
                    #[serde(rename = "urlRegex")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub url_regex: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setBreakpointByUrl/parameters/scriptHash。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Script hash of the resources to set breakpoint on.</pre>
                    ///
                    ///
                    #[serde(rename = "scriptHash")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub script_hash: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setBreakpointByUrl/parameters/columnNumber。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Offset in the line to set breakpoint at.</pre>
                    ///
                    ///
                    #[serde(rename = "columnNumber")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub column_number: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setBreakpointByUrl/parameters/condition。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Expression to use as a breakpoint condition. When specified, debugger will only stop on the
                    ///breakpoint if this expression evaluates to true.</pre>
                    ///
                    ///
                    #[serde(rename = "condition")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub condition: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetBreakpointByUrlParams {
                    line_number: fields.line_number,
                    url: fields.url,
                    url_regex: fields.url_regex,
                    script_hash: fields.script_hash,
                    column_number: fields.column_number,
                    condition: fields.condition,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.setBreakpointByUrl。
///
///协议原文：
///
///<pre>Sets JavaScript breakpoint at given location specified either by URL or URL regex. Once this
///command is issued, all existing parsed scripts will have breakpoints resolved and returned in
///`locations` property. Further matching script parsing will result in subsequent
///`breakpointResolved` events issued. This logical breakpoint will survive page reloads.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetBreakpointByUrl;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.setBreakpointByUrl/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetBreakpointByUrlResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setBreakpointByUrl/returns/breakpointId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the created breakpoint for further reference.</pre>
    ///
    ///
    #[serde(rename = "breakpointId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub breakpoint_id: crate::debugger::types::BreakpointId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setBreakpointByUrl/returns/locations。
    ///
    ///协议原文：
    ///
    ///<pre>List of the locations this breakpoint resolved into upon addition.</pre>
    ///
    ///
    #[serde(rename = "locations")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub locations: Vec<crate::debugger::types::Location>,
}
impl<'de> serde::Deserialize<'de> for SetBreakpointByUrlResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetBreakpointByUrlResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setBreakpointByUrl/returns/breakpointId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the created breakpoint for further reference.</pre>
                    ///
                    ///
                    #[serde(rename = "breakpointId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub breakpoint_id: crate::debugger::types::BreakpointId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setBreakpointByUrl/returns/locations。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>List of the locations this breakpoint resolved into upon addition.</pre>
                    ///
                    ///
                    #[serde(rename = "locations")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub locations: Vec<crate::debugger::types::Location>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetBreakpointByUrlResponse {
                    breakpoint_id: fields.breakpoint_id,
                    locations: fields.locations,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetBreakpointByUrl {
    type Params = SetBreakpointByUrlParams;
    type Response = SetBreakpointByUrlResponse;
    const METHOD: &'static str = "Debugger.setBreakpointByUrl";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.setBreakpointOnFunctionCall/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetBreakpointOnFunctionCallParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setBreakpointOnFunctionCall/parameters/objectId。
    ///
    ///协议原文：
    ///
    ///<pre>Function object id.</pre>
    ///
    ///
    #[serde(rename = "objectId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub object_id: crate::runtime::types::RemoteObjectId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setBreakpointOnFunctionCall/parameters/condition。
    ///
    ///协议原文：
    ///
    ///<pre>Expression to use as a breakpoint condition. When specified, debugger will
    ///stop on the breakpoint if this expression evaluates to true.</pre>
    ///
    ///
    #[serde(rename = "condition")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub condition: Option<String>,
}
impl<'de> serde::Deserialize<'de> for SetBreakpointOnFunctionCallParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetBreakpointOnFunctionCallParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setBreakpointOnFunctionCall/parameters/objectId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Function object id.</pre>
                    ///
                    ///
                    #[serde(rename = "objectId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub object_id: crate::runtime::types::RemoteObjectId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setBreakpointOnFunctionCall/parameters/condition。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Expression to use as a breakpoint condition. When specified, debugger will
                    ///stop on the breakpoint if this expression evaluates to true.</pre>
                    ///
                    ///
                    #[serde(rename = "condition")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub condition: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetBreakpointOnFunctionCallParams {
                    object_id: fields.object_id,
                    condition: fields.condition,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.setBreakpointOnFunctionCall。
///
///协议原文：
///
///<pre>Sets JavaScript breakpoint before each call to the given function.
///If another function was created from the same source as a given one,
///calling it will also trigger the breakpoint.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetBreakpointOnFunctionCall;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.setBreakpointOnFunctionCall/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetBreakpointOnFunctionCallResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setBreakpointOnFunctionCall/returns/breakpointId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the created breakpoint for further reference.</pre>
    ///
    ///
    #[serde(rename = "breakpointId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub breakpoint_id: crate::debugger::types::BreakpointId,
}
impl<'de> serde::Deserialize<'de> for SetBreakpointOnFunctionCallResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetBreakpointOnFunctionCallResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setBreakpointOnFunctionCall/returns/breakpointId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the created breakpoint for further reference.</pre>
                    ///
                    ///
                    #[serde(rename = "breakpointId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub breakpoint_id: crate::debugger::types::BreakpointId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetBreakpointOnFunctionCallResponse {
                    breakpoint_id: fields.breakpoint_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetBreakpointOnFunctionCall {
    type Params = SetBreakpointOnFunctionCallParams;
    type Response = SetBreakpointOnFunctionCallResponse;
    const METHOD: &'static str = "Debugger.setBreakpointOnFunctionCall";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.setBreakpointsActive/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetBreakpointsActiveParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setBreakpointsActive/parameters/active。
    ///
    ///协议原文：
    ///
    ///<pre>New value for breakpoints active state.</pre>
    ///
    ///
    #[serde(rename = "active")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub active: bool,
}
impl<'de> serde::Deserialize<'de> for SetBreakpointsActiveParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetBreakpointsActiveParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setBreakpointsActive/parameters/active。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>New value for breakpoints active state.</pre>
                    ///
                    ///
                    #[serde(rename = "active")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub active: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetBreakpointsActiveParams {
                    active: fields.active,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.setBreakpointsActive。
///
///协议原文：
///
///<pre>Activates / deactivates all breakpoints on the page.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetBreakpointsActive;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.setBreakpointsActive/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetBreakpointsActiveResponse {}
impl<'de> serde::Deserialize<'de> for SetBreakpointsActiveResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetBreakpointsActiveResponse;
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
                Ok(SetBreakpointsActiveResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetBreakpointsActive {
    type Params = SetBreakpointsActiveParams;
    type Response = SetBreakpointsActiveResponse;
    const METHOD: &'static str = "Debugger.setBreakpointsActive";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Debugger.command.setInstrumentationBreakpoint/parameters/instrumentation。
///
///协议原文：
///
///<pre>Instrumentation name.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SetInstrumentationBreakpointParamsInstrumentation {
    ///协议值 "beforeScriptExecution"。
    BeforeScriptExecution,
    ///协议值 "beforeScriptWithSourceMapExecution"。
    BeforeScriptWithSourceMapExecution,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl SetInstrumentationBreakpointParamsInstrumentation {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::debugger::commands::SetInstrumentationBreakpointParamsInstrumentation::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::BeforeScriptExecution => "beforeScriptExecution",
            Self::BeforeScriptWithSourceMapExecution => "beforeScriptWithSourceMapExecution",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for SetInstrumentationBreakpointParamsInstrumentation {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for SetInstrumentationBreakpointParamsInstrumentation {
    fn from(value: String) -> Self {
        match value.as_str() {
            "beforeScriptExecution" => {
                SetInstrumentationBreakpointParamsInstrumentation::BeforeScriptExecution
            }
            "beforeScriptWithSourceMapExecution" => {
                SetInstrumentationBreakpointParamsInstrumentation::BeforeScriptWithSourceMapExecution
            }
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for SetInstrumentationBreakpointParamsInstrumentation {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetInstrumentationBreakpointParamsInstrumentation {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetInstrumentationBreakpointParamsInstrumentation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(
                    match v {
                        "beforeScriptExecution" => {
                            SetInstrumentationBreakpointParamsInstrumentation::BeforeScriptExecution
                        }
                        "beforeScriptWithSourceMapExecution" => {
                            SetInstrumentationBreakpointParamsInstrumentation::BeforeScriptWithSourceMapExecution
                        }
                        _ => {
                            SetInstrumentationBreakpointParamsInstrumentation::Unknown(
                                v.to_owned(),
                            )
                        }
                    },
                )
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(SetInstrumentationBreakpointParamsInstrumentation::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.setInstrumentationBreakpoint/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetInstrumentationBreakpointParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setInstrumentationBreakpoint/parameters/instrumentation。
    ///
    ///协议原文：
    ///
    ///<pre>Instrumentation name.</pre>
    ///
    ///
    #[serde(rename = "instrumentation")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub instrumentation: SetInstrumentationBreakpointParamsInstrumentation,
}
impl<'de> serde::Deserialize<'de> for SetInstrumentationBreakpointParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetInstrumentationBreakpointParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setInstrumentationBreakpoint/parameters/instrumentation。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Instrumentation name.</pre>
                    ///
                    ///
                    #[serde(rename = "instrumentation")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub instrumentation: SetInstrumentationBreakpointParamsInstrumentation,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetInstrumentationBreakpointParams {
                    instrumentation: fields.instrumentation,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.setInstrumentationBreakpoint。
///
///协议原文：
///
///<pre>Sets instrumentation breakpoint.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetInstrumentationBreakpoint;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.setInstrumentationBreakpoint/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetInstrumentationBreakpointResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setInstrumentationBreakpoint/returns/breakpointId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the created breakpoint for further reference.</pre>
    ///
    ///
    #[serde(rename = "breakpointId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub breakpoint_id: crate::debugger::types::BreakpointId,
}
impl<'de> serde::Deserialize<'de> for SetInstrumentationBreakpointResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetInstrumentationBreakpointResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setInstrumentationBreakpoint/returns/breakpointId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the created breakpoint for further reference.</pre>
                    ///
                    ///
                    #[serde(rename = "breakpointId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub breakpoint_id: crate::debugger::types::BreakpointId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetInstrumentationBreakpointResponse {
                    breakpoint_id: fields.breakpoint_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetInstrumentationBreakpoint {
    type Params = SetInstrumentationBreakpointParams;
    type Response = SetInstrumentationBreakpointResponse;
    const METHOD: &'static str = "Debugger.setInstrumentationBreakpoint";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Debugger.command.setPauseOnExceptions/parameters/state。
///
///协议原文：
///
///<pre>Pause on exceptions mode.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SetPauseOnExceptionsParamsState {
    ///协议值 "none"。
    None,
    ///协议值 "caught"。
    Caught,
    ///协议值 "uncaught"。
    Uncaught,
    ///协议值 "all"。
    All,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl SetPauseOnExceptionsParamsState {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::debugger::commands::SetPauseOnExceptionsParamsState::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::None => "none",
            Self::Caught => "caught",
            Self::Uncaught => "uncaught",
            Self::All => "all",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for SetPauseOnExceptionsParamsState {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for SetPauseOnExceptionsParamsState {
    fn from(value: String) -> Self {
        match value.as_str() {
            "none" => SetPauseOnExceptionsParamsState::None,
            "caught" => SetPauseOnExceptionsParamsState::Caught,
            "uncaught" => SetPauseOnExceptionsParamsState::Uncaught,
            "all" => SetPauseOnExceptionsParamsState::All,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for SetPauseOnExceptionsParamsState {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetPauseOnExceptionsParamsState {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetPauseOnExceptionsParamsState;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "none" => SetPauseOnExceptionsParamsState::None,
                    "caught" => SetPauseOnExceptionsParamsState::Caught,
                    "uncaught" => SetPauseOnExceptionsParamsState::Uncaught,
                    "all" => SetPauseOnExceptionsParamsState::All,
                    _ => SetPauseOnExceptionsParamsState::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(SetPauseOnExceptionsParamsState::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.setPauseOnExceptions/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetPauseOnExceptionsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setPauseOnExceptions/parameters/state。
    ///
    ///协议原文：
    ///
    ///<pre>Pause on exceptions mode.</pre>
    ///
    ///
    #[serde(rename = "state")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub state: SetPauseOnExceptionsParamsState,
}
impl<'de> serde::Deserialize<'de> for SetPauseOnExceptionsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetPauseOnExceptionsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setPauseOnExceptions/parameters/state。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Pause on exceptions mode.</pre>
                    ///
                    ///
                    #[serde(rename = "state")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub state: SetPauseOnExceptionsParamsState,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetPauseOnExceptionsParams {
                    state: fields.state,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.setPauseOnExceptions。
///
///协议原文：
///
///<pre>Defines pause on exceptions state. Can be set to stop on all exceptions, uncaught exceptions,
///or caught exceptions, no exceptions. Initial pause on exceptions state is `none`.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetPauseOnExceptions;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.setPauseOnExceptions/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetPauseOnExceptionsResponse {}
impl<'de> serde::Deserialize<'de> for SetPauseOnExceptionsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetPauseOnExceptionsResponse;
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
                Ok(SetPauseOnExceptionsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetPauseOnExceptions {
    type Params = SetPauseOnExceptionsParams;
    type Response = SetPauseOnExceptionsResponse;
    const METHOD: &'static str = "Debugger.setPauseOnExceptions";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.setReturnValue/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetReturnValueParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setReturnValue/parameters/newValue。
    ///
    ///协议原文：
    ///
    ///<pre>New return value.</pre>
    ///
    ///
    #[serde(rename = "newValue")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub new_value: crate::runtime::types::CallArgument,
}
impl<'de> serde::Deserialize<'de> for SetReturnValueParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetReturnValueParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setReturnValue/parameters/newValue。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>New return value.</pre>
                    ///
                    ///
                    #[serde(rename = "newValue")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub new_value: crate::runtime::types::CallArgument,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetReturnValueParams {
                    new_value: fields.new_value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.setReturnValue。
///
///协议原文：
///
///<pre>Changes return value in top frame. Available only at return break position.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetReturnValue;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.setReturnValue/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetReturnValueResponse {}
impl<'de> serde::Deserialize<'de> for SetReturnValueResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetReturnValueResponse;
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
                Ok(SetReturnValueResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetReturnValue {
    type Params = SetReturnValueParams;
    type Response = SetReturnValueResponse;
    const METHOD: &'static str = "Debugger.setReturnValue";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.setScriptSource/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetScriptSourceParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setScriptSource/parameters/scriptId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the script to edit.</pre>
    ///
    ///
    #[serde(rename = "scriptId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub script_id: crate::runtime::types::ScriptId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setScriptSource/parameters/scriptSource。
    ///
    ///协议原文：
    ///
    ///<pre>New content of the script.</pre>
    ///
    ///
    #[serde(rename = "scriptSource")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub script_source: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setScriptSource/parameters/dryRun。
    ///
    ///协议原文：
    ///
    ///<pre>If true the change will not actually be applied. Dry run may be used to get result
    ///description without actually modifying the code.</pre>
    ///
    ///
    #[serde(rename = "dryRun")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub dry_run: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setScriptSource/parameters/allowTopFrameEditing。
    ///
    ///协议原文：
    ///
    ///<pre>If true, then `scriptSource` is allowed to change the function on top of the stack
    ///as long as the top-most stack frame is the only activation of that function.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "allowTopFrameEditing")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_top_frame_editing: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for SetScriptSourceParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetScriptSourceParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setScriptSource/parameters/scriptId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the script to edit.</pre>
                    ///
                    ///
                    #[serde(rename = "scriptId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub script_id: crate::runtime::types::ScriptId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setScriptSource/parameters/scriptSource。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>New content of the script.</pre>
                    ///
                    ///
                    #[serde(rename = "scriptSource")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub script_source: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setScriptSource/parameters/dryRun。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true the change will not actually be applied. Dry run may be used to get result
                    ///description without actually modifying the code.</pre>
                    ///
                    ///
                    #[serde(rename = "dryRun")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub dry_run: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setScriptSource/parameters/allowTopFrameEditing。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, then `scriptSource` is allowed to change the function on top of the stack
                    ///as long as the top-most stack frame is the only activation of that function.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "allowTopFrameEditing")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub allow_top_frame_editing: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetScriptSourceParams {
                    script_id: fields.script_id,
                    script_source: fields.script_source,
                    dry_run: fields.dry_run,
                    allow_top_frame_editing: fields.allow_top_frame_editing,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.setScriptSource。
///
///协议原文：
///
///<pre>Live edit is no longer supported and this command always fails with a "no longer available" error.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct SetScriptSource;
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Debugger.command.setScriptSource/returns/status。
///
///协议原文：
///
///<pre>Whether the operation was successful or not. Only `Ok` denotes a
///successful live edit while the other enum variants denote why
///the live edit failed.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SetScriptSourceResponseStatus {
    ///协议值 "Ok"。
    Ok,
    ///协议值 "CompileError"。
    CompileError,
    ///协议值 "BlockedByActiveGenerator"。
    BlockedByActiveGenerator,
    ///协议值 "BlockedByActiveFunction"。
    BlockedByActiveFunction,
    ///协议值 "BlockedByTopLevelEsModuleChange"。
    BlockedByTopLevelEsModuleChange,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl SetScriptSourceResponseStatus {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::debugger::commands::SetScriptSourceResponseStatus::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::CompileError => "CompileError",
            Self::BlockedByActiveGenerator => "BlockedByActiveGenerator",
            Self::BlockedByActiveFunction => "BlockedByActiveFunction",
            Self::BlockedByTopLevelEsModuleChange => "BlockedByTopLevelEsModuleChange",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for SetScriptSourceResponseStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for SetScriptSourceResponseStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Ok" => SetScriptSourceResponseStatus::Ok,
            "CompileError" => SetScriptSourceResponseStatus::CompileError,
            "BlockedByActiveGenerator" => SetScriptSourceResponseStatus::BlockedByActiveGenerator,
            "BlockedByActiveFunction" => SetScriptSourceResponseStatus::BlockedByActiveFunction,
            "BlockedByTopLevelEsModuleChange" => {
                SetScriptSourceResponseStatus::BlockedByTopLevelEsModuleChange
            }
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for SetScriptSourceResponseStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetScriptSourceResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetScriptSourceResponseStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Ok" => SetScriptSourceResponseStatus::Ok,
                    "CompileError" => SetScriptSourceResponseStatus::CompileError,
                    "BlockedByActiveGenerator" => {
                        SetScriptSourceResponseStatus::BlockedByActiveGenerator
                    }
                    "BlockedByActiveFunction" => {
                        SetScriptSourceResponseStatus::BlockedByActiveFunction
                    }
                    "BlockedByTopLevelEsModuleChange" => {
                        SetScriptSourceResponseStatus::BlockedByTopLevelEsModuleChange
                    }
                    _ => SetScriptSourceResponseStatus::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(SetScriptSourceResponseStatus::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.setScriptSource/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetScriptSourceResponse {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setScriptSource/returns/callFrames。
    ///
    ///协议原文：
    ///
    ///<pre>New stack trace in case editing has happened while VM was stopped.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "callFrames")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub call_frames: Option<Vec<crate::debugger::types::CallFrame>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setScriptSource/returns/stackChanged。
    ///
    ///协议原文：
    ///
    ///<pre>Whether current call stack  was modified after applying the changes.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "stackChanged")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub stack_changed: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setScriptSource/returns/asyncStackTrace。
    ///
    ///协议原文：
    ///
    ///<pre>Async stack trace, if any.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "asyncStackTrace")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub async_stack_trace: Option<crate::runtime::types::StackTrace>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setScriptSource/returns/asyncStackTraceId。
    ///
    ///协议原文：
    ///
    ///<pre>Async stack trace, if any.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "asyncStackTraceId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub async_stack_trace_id: Option<crate::runtime::types::StackTraceId>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setScriptSource/returns/status。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the operation was successful or not. Only `Ok` denotes a
    ///successful live edit while the other enum variants denote why
    ///the live edit failed.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "status")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub status: SetScriptSourceResponseStatus,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setScriptSource/returns/exceptionDetails。
    ///
    ///协议原文：
    ///
    ///<pre>Exception details if any. Only present when `status` is `CompileError`.</pre>
    ///
    ///
    #[serde(rename = "exceptionDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub exception_details: Option<crate::runtime::types::ExceptionDetails>,
}
impl<'de> serde::Deserialize<'de> for SetScriptSourceResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetScriptSourceResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setScriptSource/returns/callFrames。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>New stack trace in case editing has happened while VM was stopped.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "callFrames")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub call_frames: Option<Vec<crate::debugger::types::CallFrame>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setScriptSource/returns/stackChanged。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether current call stack  was modified after applying the changes.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "stackChanged")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub stack_changed: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setScriptSource/returns/asyncStackTrace。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Async stack trace, if any.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "asyncStackTrace")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub async_stack_trace: Option<crate::runtime::types::StackTrace>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setScriptSource/returns/asyncStackTraceId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Async stack trace, if any.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "asyncStackTraceId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub async_stack_trace_id: Option<crate::runtime::types::StackTraceId>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setScriptSource/returns/status。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the operation was successful or not. Only `Ok` denotes a
                    ///successful live edit while the other enum variants denote why
                    ///the live edit failed.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "status")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub status: SetScriptSourceResponseStatus,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.setScriptSource/returns/exceptionDetails。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Exception details if any. Only present when `status` is `CompileError`.</pre>
                    ///
                    ///
                    #[serde(rename = "exceptionDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub exception_details: Option<crate::runtime::types::ExceptionDetails>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetScriptSourceResponse {
                    call_frames: fields.call_frames,
                    stack_changed: fields.stack_changed,
                    async_stack_trace: fields.async_stack_trace,
                    async_stack_trace_id: fields.async_stack_trace_id,
                    status: fields.status,
                    exception_details: fields.exception_details,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetScriptSource {
    type Params = SetScriptSourceParams;
    type Response = SetScriptSourceResponse;
    const METHOD: &'static str = "Debugger.setScriptSource";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.setSkipAllPauses/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetSkipAllPausesParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setSkipAllPauses/parameters/skip。
    ///
    ///协议原文：
    ///
    ///<pre>New value for skip pauses state.</pre>
    ///
    ///
    #[serde(rename = "skip")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub skip: bool,
}
impl<'de> serde::Deserialize<'de> for SetSkipAllPausesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetSkipAllPausesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setSkipAllPauses/parameters/skip。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>New value for skip pauses state.</pre>
                    ///
                    ///
                    #[serde(rename = "skip")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub skip: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetSkipAllPausesParams { skip: fields.skip })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.setSkipAllPauses。
///
///协议原文：
///
///<pre>Makes page not interrupt on any pauses (breakpoint, exception, dom exception etc).</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetSkipAllPauses;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.setSkipAllPauses/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetSkipAllPausesResponse {}
impl<'de> serde::Deserialize<'de> for SetSkipAllPausesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetSkipAllPausesResponse;
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
                Ok(SetSkipAllPausesResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetSkipAllPauses {
    type Params = SetSkipAllPausesParams;
    type Response = SetSkipAllPausesResponse;
    const METHOD: &'static str = "Debugger.setSkipAllPauses";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.setVariableValue/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetVariableValueParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setVariableValue/parameters/scopeNumber。
    ///
    ///协议原文：
    ///
    ///<pre>0-based number of scope as was listed in scope chain. Only 'local', 'closure' and 'catch'
    ///scope types are allowed. Other scopes could be manipulated manually.</pre>
    ///
    ///
    #[serde(rename = "scopeNumber")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub scope_number: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setVariableValue/parameters/variableName。
    ///
    ///协议原文：
    ///
    ///<pre>Variable name.</pre>
    ///
    ///
    #[serde(rename = "variableName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub variable_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setVariableValue/parameters/newValue。
    ///
    ///协议原文：
    ///
    ///<pre>New variable value.</pre>
    ///
    ///
    #[serde(rename = "newValue")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub new_value: crate::runtime::types::CallArgument,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setVariableValue/parameters/callFrameId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of callframe that holds variable.</pre>
    ///
    ///
    #[serde(rename = "callFrameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub call_frame_id: crate::debugger::types::CallFrameId,
}
impl<'de> serde::Deserialize<'de> for SetVariableValueParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetVariableValueParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setVariableValue/parameters/scopeNumber。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>0-based number of scope as was listed in scope chain. Only 'local', 'closure' and 'catch'
                    ///scope types are allowed. Other scopes could be manipulated manually.</pre>
                    ///
                    ///
                    #[serde(rename = "scopeNumber")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub scope_number: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setVariableValue/parameters/variableName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Variable name.</pre>
                    ///
                    ///
                    #[serde(rename = "variableName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub variable_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setVariableValue/parameters/newValue。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>New variable value.</pre>
                    ///
                    ///
                    #[serde(rename = "newValue")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub new_value: crate::runtime::types::CallArgument,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Debugger.command.setVariableValue/parameters/callFrameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of callframe that holds variable.</pre>
                    ///
                    ///
                    #[serde(rename = "callFrameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub call_frame_id: crate::debugger::types::CallFrameId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetVariableValueParams {
                    scope_number: fields.scope_number,
                    variable_name: fields.variable_name,
                    new_value: fields.new_value,
                    call_frame_id: fields.call_frame_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.setVariableValue。
///
///协议原文：
///
///<pre>Changes value of variable in a callframe. Object-based scopes are not supported and must be
///mutated manually.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetVariableValue;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.setVariableValue/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetVariableValueResponse {}
impl<'de> serde::Deserialize<'de> for SetVariableValueResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetVariableValueResponse;
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
                Ok(SetVariableValueResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetVariableValue {
    type Params = SetVariableValueParams;
    type Response = SetVariableValueResponse;
    const METHOD: &'static str = "Debugger.setVariableValue";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.stepInto/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct StepIntoParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.stepInto/parameters/breakOnAsyncCall。
    ///
    ///协议原文：
    ///
    ///<pre>Debugger will pause on the execution of the first async task which was scheduled
    ///before next pause.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "breakOnAsyncCall")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub break_on_async_call: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.stepInto/parameters/skipList。
    ///
    ///协议原文：
    ///
    ///<pre>The skipList specifies location ranges that should be skipped on step into.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "skipList")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub skip_list: Option<Vec<crate::debugger::types::LocationRange>>,
}
impl<'de> serde::Deserialize<'de> for StepIntoParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StepIntoParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.stepInto/parameters/breakOnAsyncCall。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Debugger will pause on the execution of the first async task which was scheduled
                    ///before next pause.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "breakOnAsyncCall")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub break_on_async_call: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.stepInto/parameters/skipList。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The skipList specifies location ranges that should be skipped on step into.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "skipList")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub skip_list: Option<Vec<crate::debugger::types::LocationRange>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StepIntoParams {
                    break_on_async_call: fields.break_on_async_call,
                    skip_list: fields.skip_list,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.stepInto。
///
///协议原文：
///
///<pre>Steps into the function call.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct StepInto;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.stepInto/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StepIntoResponse {}
impl<'de> serde::Deserialize<'de> for StepIntoResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StepIntoResponse;
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
                Ok(StepIntoResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for StepInto {
    type Params = StepIntoParams;
    type Response = StepIntoResponse;
    const METHOD: &'static str = "Debugger.stepInto";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.stepOut/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct StepOutParams {}
impl<'de> serde::Deserialize<'de> for StepOutParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StepOutParams;
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
                Ok(StepOutParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.stepOut。
///
///协议原文：
///
///<pre>Steps out of the function call.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct StepOut;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.stepOut/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StepOutResponse {}
impl<'de> serde::Deserialize<'de> for StepOutResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StepOutResponse;
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
                Ok(StepOutResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for StepOut {
    type Params = StepOutParams;
    type Response = StepOutResponse;
    const METHOD: &'static str = "Debugger.stepOut";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.stepOver/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct StepOverParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.stepOver/parameters/skipList。
    ///
    ///协议原文：
    ///
    ///<pre>The skipList specifies location ranges that should be skipped on step over.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "skipList")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub skip_list: Option<Vec<crate::debugger::types::LocationRange>>,
}
impl<'de> serde::Deserialize<'de> for StepOverParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StepOverParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Debugger.command.stepOver/parameters/skipList。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The skipList specifies location ranges that should be skipped on step over.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "skipList")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub skip_list: Option<Vec<crate::debugger::types::LocationRange>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StepOverParams {
                    skip_list: fields.skip_list,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Debugger.command.stepOver。
///
///协议原文：
///
///<pre>Steps over the statement.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct StepOver;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Debugger.command.stepOver/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StepOverResponse {}
impl<'de> serde::Deserialize<'de> for StepOverResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StepOverResponse;
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
                Ok(StepOverResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for StepOver {
    type Params = StepOverParams;
    type Response = StepOverResponse;
    const METHOD: &'static str = "Debugger.stepOver";
}
