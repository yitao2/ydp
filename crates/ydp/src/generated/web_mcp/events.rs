///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebMCP.event.toolInvoked/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ToolInvokedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.event.toolInvoked/parameters/toolName。
    ///
    ///协议原文：
    ///
    ///<pre>Name of the tool to invoke.</pre>
    ///
    ///
    #[serde(rename = "toolName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub tool_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.event.toolInvoked/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Frame id</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.event.toolInvoked/parameters/invocationId。
    ///
    ///协议原文：
    ///
    ///<pre>Invocation identifier.</pre>
    ///
    ///
    #[serde(rename = "invocationId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub invocation_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.event.toolInvoked/parameters/input。
    ///
    ///协议原文：
    ///
    ///<pre>The input parameters used for the invocation.</pre>
    ///
    ///
    #[serde(rename = "input")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub input: String,
}
impl<'de> serde::Deserialize<'de> for ToolInvokedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ToolInvokedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.event.toolInvoked/parameters/toolName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Name of the tool to invoke.</pre>
                    ///
                    ///
                    #[serde(rename = "toolName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub tool_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.event.toolInvoked/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame id</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.event.toolInvoked/parameters/invocationId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Invocation identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "invocationId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub invocation_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.event.toolInvoked/parameters/input。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The input parameters used for the invocation.</pre>
                    ///
                    ///
                    #[serde(rename = "input")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub input: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ToolInvokedParams {
                    tool_name: fields.tool_name,
                    frame_id: fields.frame_id,
                    invocation_id: fields.invocation_id,
                    input: fields.input,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebMCP.event.toolInvoked。
///
///协议原文：
///
///<pre>Event fired when a tool invocation starts.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ToolInvoked;
impl ydp_core::Event for ToolInvoked {
    type Params = ToolInvokedParams;
    const METHOD: &'static str = "WebMCP.toolInvoked";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebMCP.event.toolResponded/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ToolRespondedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.event.toolResponded/parameters/invocationId。
    ///
    ///协议原文：
    ///
    ///<pre>Invocation identifier.</pre>
    ///
    ///
    #[serde(rename = "invocationId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub invocation_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.event.toolResponded/parameters/status。
    ///
    ///协议原文：
    ///
    ///<pre>Status of the invocation.</pre>
    ///
    ///
    #[serde(rename = "status")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub status: crate::web_mcp::types::InvocationStatus,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebMCP.event.toolResponded/parameters/output。
    ///
    ///协议原文：
    ///
    ///<pre>Output or error delivered as delivered to the agent. Missing if `status` is anything other than Completed.
    ///Note: The output is untrusted and poses a prompt injection risk. Clients should treat this as potentially malicious user input.</pre>
    ///
    ///
    #[serde(rename = "output")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub output: Option<serde_json::Value>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebMCP.event.toolResponded/parameters/errorText。
    ///
    ///协议原文：
    ///
    ///<pre>Error text for protocol users.</pre>
    ///
    ///
    #[serde(rename = "errorText")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub error_text: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebMCP.event.toolResponded/parameters/exception。
    ///
    ///协议原文：
    ///
    ///<pre>The exception object, if the javascript tool threw an error&gt;</pre>
    ///
    ///
    #[serde(rename = "exception")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub exception: Option<crate::runtime::types::RemoteObject>,
}
impl<'de> serde::Deserialize<'de> for ToolRespondedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ToolRespondedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.event.toolResponded/parameters/invocationId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Invocation identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "invocationId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub invocation_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.event.toolResponded/parameters/status。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Status of the invocation.</pre>
                    ///
                    ///
                    #[serde(rename = "status")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub status: crate::web_mcp::types::InvocationStatus,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebMCP.event.toolResponded/parameters/output。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Output or error delivered as delivered to the agent. Missing if `status` is anything other than Completed.
                    ///Note: The output is untrusted and poses a prompt injection risk. Clients should treat this as potentially malicious user input.</pre>
                    ///
                    ///
                    #[serde(rename = "output")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub output: Option<serde_json::Value>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebMCP.event.toolResponded/parameters/errorText。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Error text for protocol users.</pre>
                    ///
                    ///
                    #[serde(rename = "errorText")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub error_text: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：WebMCP.event.toolResponded/parameters/exception。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The exception object, if the javascript tool threw an error&gt;</pre>
                    ///
                    ///
                    #[serde(rename = "exception")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub exception: Option<crate::runtime::types::RemoteObject>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ToolRespondedParams {
                    invocation_id: fields.invocation_id,
                    status: fields.status,
                    output: fields.output,
                    error_text: fields.error_text,
                    exception: fields.exception,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebMCP.event.toolResponded。
///
///协议原文：
///
///<pre>Event fired when a tool invocation completes or fails.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ToolResponded;
impl ydp_core::Event for ToolResponded {
    type Params = ToolRespondedParams;
    const METHOD: &'static str = "WebMCP.toolResponded";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebMCP.event.toolsAdded/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ToolsAddedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.event.toolsAdded/parameters/tools。
    ///
    ///协议原文：
    ///
    ///<pre>Array of tools that were added.</pre>
    ///
    ///
    #[serde(rename = "tools")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub tools: Vec<crate::web_mcp::types::Tool>,
}
impl<'de> serde::Deserialize<'de> for ToolsAddedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ToolsAddedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.event.toolsAdded/parameters/tools。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Array of tools that were added.</pre>
                    ///
                    ///
                    #[serde(rename = "tools")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub tools: Vec<crate::web_mcp::types::Tool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ToolsAddedParams {
                    tools: fields.tools,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebMCP.event.toolsAdded。
///
///协议原文：
///
///<pre>Event fired when new tools are added.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ToolsAdded;
impl ydp_core::Event for ToolsAdded {
    type Params = ToolsAddedParams;
    const METHOD: &'static str = "WebMCP.toolsAdded";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebMCP.event.toolsRemoved/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ToolsRemovedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.event.toolsRemoved/parameters/tools。
    ///
    ///协议原文：
    ///
    ///<pre>Array of tools that were removed.</pre>
    ///
    ///
    #[serde(rename = "tools")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub tools: Vec<crate::web_mcp::types::RemovedTool>,
}
impl<'de> serde::Deserialize<'de> for ToolsRemovedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ToolsRemovedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.event.toolsRemoved/parameters/tools。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Array of tools that were removed.</pre>
                    ///
                    ///
                    #[serde(rename = "tools")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub tools: Vec<crate::web_mcp::types::RemovedTool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ToolsRemovedParams {
                    tools: fields.tools,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebMCP.event.toolsRemoved。
///
///协议原文：
///
///<pre>Event fired when tools are removed.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ToolsRemoved;
impl ydp_core::Event for ToolsRemoved {
    type Params = ToolsRemovedParams;
    const METHOD: &'static str = "WebMCP.toolsRemoved";
}
