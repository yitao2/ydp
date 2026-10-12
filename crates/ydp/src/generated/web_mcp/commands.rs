///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebMCP.command.cancelInvocation/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CancelInvocationParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.command.cancelInvocation/parameters/invocationId。
    ///
    ///协议原文：
    ///
    ///<pre>Invocation identifier to cancel.</pre>
    ///
    ///
    #[serde(rename = "invocationId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub invocation_id: String,
}
impl<'de> serde::Deserialize<'de> for CancelInvocationParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CancelInvocationParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.command.cancelInvocation/parameters/invocationId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Invocation identifier to cancel.</pre>
                    ///
                    ///
                    #[serde(rename = "invocationId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub invocation_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CancelInvocationParams {
                    invocation_id: fields.invocation_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebMCP.command.cancelInvocation。
///
///协议原文：
///
///<pre>Cancels a pending tool invocation.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct CancelInvocation;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebMCP.command.cancelInvocation/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CancelInvocationResponse {}
impl<'de> serde::Deserialize<'de> for CancelInvocationResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CancelInvocationResponse;
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
                Ok(CancelInvocationResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for CancelInvocation {
    type Params = CancelInvocationParams;
    type Response = CancelInvocationResponse;
    const METHOD: &'static str = "WebMCP.cancelInvocation";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebMCP.command.disable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebMCP.command.disable。
///
///协议原文：
///
///<pre>Disables the WebMCP domain.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebMCP.command.disable/returns。
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
    const METHOD: &'static str = "WebMCP.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebMCP.command.enable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebMCP.command.enable。
///
///协议原文：
///
///<pre>Enables the WebMCP domain, allowing events to be sent. Enabling the domain will trigger a toolsAdded event for
///all currently registered tools.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebMCP.command.enable/returns。
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
    const METHOD: &'static str = "WebMCP.enable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebMCP.command.invokeTool/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct InvokeToolParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.command.invokeTool/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Frame in which to invoke the tool.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.command.invokeTool/parameters/toolName。
    ///
    ///协议原文：
    ///
    ///<pre>Name of the tool to invoke.</pre>
    ///
    ///
    #[serde(rename = "toolName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub tool_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.command.invokeTool/parameters/input。
    ///
    ///协议原文：
    ///
    ///<pre>Input parameters for the tool, matching the tool's inputSchema.</pre>
    ///
    ///
    #[serde(rename = "input")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub input: std::collections::BTreeMap<String, serde_json::Value>,
}
impl<'de> serde::Deserialize<'de> for InvokeToolParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InvokeToolParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.command.invokeTool/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame in which to invoke the tool.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.command.invokeTool/parameters/toolName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Name of the tool to invoke.</pre>
                    ///
                    ///
                    #[serde(rename = "toolName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub tool_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.command.invokeTool/parameters/input。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Input parameters for the tool, matching the tool's inputSchema.</pre>
                    ///
                    ///
                    #[serde(rename = "input")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub input: std::collections::BTreeMap<String, serde_json::Value>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(InvokeToolParams {
                    frame_id: fields.frame_id,
                    tool_name: fields.tool_name,
                    input: fields.input,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：WebMCP.command.invokeTool。
///
///协议原文：
///
///<pre>Invokes a registered tool.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct InvokeTool;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：WebMCP.command.invokeTool/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct InvokeToolResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.command.invokeTool/returns/invocationId。
    ///
    ///协议原文：
    ///
    ///<pre>Unique identifier for this invocation. Response is sent before tool events.</pre>
    ///
    ///
    #[serde(rename = "invocationId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub invocation_id: String,
}
impl<'de> serde::Deserialize<'de> for InvokeToolResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InvokeToolResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：WebMCP.command.invokeTool/returns/invocationId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Unique identifier for this invocation. Response is sent before tool events.</pre>
                    ///
                    ///
                    #[serde(rename = "invocationId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub invocation_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(InvokeToolResponse {
                    invocation_id: fields.invocation_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for InvokeTool {
    type Params = InvokeToolParams;
    type Response = InvokeToolResponse;
    const METHOD: &'static str = "WebMCP.invokeTool";
}
