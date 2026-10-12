///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebMCP.type.Annotation。
///
///协议原文：
///
///<pre>Tool annotations</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct Annotation {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebMCP.type.Annotation/properties/readOnly。
    ///
    ///协议原文：
    ///
    ///<pre>A hint indicating that the tool does not modify any state.</pre>
    ///
    ///
    #[serde(rename = "readOnly")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub read_only: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebMCP.type.Annotation/properties/untrustedContent。
    ///
    ///协议原文：
    ///
    ///<pre>A hint indicating that the tool output may contain untrusted content, ex: UGC, 3rd party data.</pre>
    ///
    ///
    #[serde(rename = "untrustedContent")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub untrusted_content: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebMCP.type.Annotation/properties/consequential。
    ///
    ///协议原文：
    ///
    ///<pre>A hint indicating that executing the tool will result in consequential actions, ex: booking a flight, transferring money.</pre>
    ///
    ///
    #[serde(rename = "consequential")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub consequential: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebMCP.type.Annotation/properties/autosubmit。
    ///
    ///协议原文：
    ///
    ///<pre>If the declarative tool was declared with the autosubmit attribute.</pre>
    ///
    ///
    #[serde(rename = "autosubmit")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub autosubmit: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for Annotation {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Annotation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebMCP.type.Annotation/properties/readOnly。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A hint indicating that the tool does not modify any state.</pre>
                    ///
                    ///
                    #[serde(rename = "readOnly")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub read_only: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebMCP.type.Annotation/properties/untrustedContent。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A hint indicating that the tool output may contain untrusted content, ex: UGC, 3rd party data.</pre>
                    ///
                    ///
                    #[serde(rename = "untrustedContent")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub untrusted_content: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebMCP.type.Annotation/properties/consequential。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A hint indicating that executing the tool will result in consequential actions, ex: booking a flight, transferring money.</pre>
                    ///
                    ///
                    #[serde(rename = "consequential")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub consequential: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebMCP.type.Annotation/properties/autosubmit。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If the declarative tool was declared with the autosubmit attribute.</pre>
                    ///
                    ///
                    #[serde(rename = "autosubmit")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub autosubmit: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Annotation {
                    read_only: fields.read_only,
                    untrusted_content: fields.untrusted_content,
                    consequential: fields.consequential,
                    autosubmit: fields.autosubmit,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：WebMCP.type.InvocationStatus。
///
///协议原文：
///
///<pre>Represents the status of a tool invocation.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum InvocationStatus {
    ///协议值 "Completed"。
    Completed,
    ///协议值 "Canceled"。
    Canceled,
    ///协议值 "Error"。
    Error,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl InvocationStatus {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::web_mcp::types::InvocationStatus::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Completed => "Completed",
            Self::Canceled => "Canceled",
            Self::Error => "Error",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for InvocationStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for InvocationStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Completed" => InvocationStatus::Completed,
            "Canceled" => InvocationStatus::Canceled,
            "Error" => InvocationStatus::Error,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for InvocationStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InvocationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InvocationStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Completed" => InvocationStatus::Completed,
                    "Canceled" => InvocationStatus::Canceled,
                    "Error" => InvocationStatus::Error,
                    _ => InvocationStatus::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(InvocationStatus::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebMCP.type.RemovedTool。
///
///协议原文：
///
///<pre>Definition of a tool that was removed.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemovedTool {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.type.RemovedTool/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Tool name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.type.RemovedTool/properties/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Frame identifier associated with the tool registration.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
}
impl<'de> serde::Deserialize<'de> for RemovedTool {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemovedTool;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.type.RemovedTool/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Tool name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.type.RemovedTool/properties/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame identifier associated with the tool registration.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RemovedTool {
                    name: fields.name,
                    frame_id: fields.frame_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebMCP.type.Tool。
///
///协议原文：
///
///<pre>Definition of a tool that can be invoked.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct Tool {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.type.Tool/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Tool name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.type.Tool/properties/description。
    ///
    ///协议原文：
    ///
    ///<pre>Tool description.</pre>
    ///
    ///
    #[serde(rename = "description")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub description: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebMCP.type.Tool/properties/inputSchema。
    ///
    ///协议原文：
    ///
    ///<pre>Schema for the tool's input parameters.</pre>
    ///
    ///
    #[serde(rename = "inputSchema")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub input_schema: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebMCP.type.Tool/properties/annotations。
    ///
    ///协议原文：
    ///
    ///<pre>Optional annotations for the tool.</pre>
    ///
    ///
    #[serde(rename = "annotations")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub annotations: Option<crate::web_mcp::types::Annotation>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.type.Tool/properties/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Frame identifier associated with the tool registration.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebMCP.type.Tool/properties/backendNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Optional node ID for declarative tools.</pre>
    ///
    ///
    #[serde(rename = "backendNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebMCP.type.Tool/properties/stackTrace。
    ///
    ///协议原文：
    ///
    ///<pre>The stack trace at the time of the registration.</pre>
    ///
    ///
    #[serde(rename = "stackTrace")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub stack_trace: Option<crate::runtime::types::StackTrace>,
}
impl<'de> serde::Deserialize<'de> for Tool {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Tool;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.type.Tool/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Tool name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.type.Tool/properties/description。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Tool description.</pre>
                    ///
                    ///
                    #[serde(rename = "description")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub description: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebMCP.type.Tool/properties/inputSchema。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Schema for the tool's input parameters.</pre>
                    ///
                    ///
                    #[serde(rename = "inputSchema")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub input_schema: Option<std::collections::BTreeMap<String, serde_json::Value>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebMCP.type.Tool/properties/annotations。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Optional annotations for the tool.</pre>
                    ///
                    ///
                    #[serde(rename = "annotations")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub annotations: Option<crate::web_mcp::types::Annotation>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.type.Tool/properties/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame identifier associated with the tool registration.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebMCP.type.Tool/properties/backendNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Optional node ID for declarative tools.</pre>
                    ///
                    ///
                    #[serde(rename = "backendNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebMCP.type.Tool/properties/stackTrace。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The stack trace at the time of the registration.</pre>
                    ///
                    ///
                    #[serde(rename = "stackTrace")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub stack_trace: Option<crate::runtime::types::StackTrace>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Tool {
                    name: fields.name,
                    description: fields.description,
                    input_schema: fields.input_schema,
                    annotations: fields.annotations,
                    frame_id: fields.frame_id,
                    backend_node_id: fields.backend_node_id,
                    stack_trace: fields.stack_trace,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
