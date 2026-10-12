///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMDebugger.command.getEventListeners/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetEventListenersParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.command.getEventListeners/parameters/objectId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the object to return listeners for.</pre>
    ///
    ///
    #[serde(rename = "objectId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub object_id: crate::runtime::types::RemoteObjectId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMDebugger.command.getEventListeners/parameters/depth。
    ///
    ///协议原文：
    ///
    ///<pre>The maximum depth at which Node children should be retrieved, defaults to 1. Use -1 for the
    ///entire subtree or provide an integer larger than 0.</pre>
    ///
    ///
    #[serde(rename = "depth")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub depth: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMDebugger.command.getEventListeners/parameters/pierce。
    ///
    ///协议原文：
    ///
    ///<pre>Whether or not iframes and shadow roots should be traversed when returning the subtree
    ///(default is false). Reports listeners for all contexts if pierce is enabled.</pre>
    ///
    ///
    #[serde(rename = "pierce")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub pierce: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for GetEventListenersParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetEventListenersParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.command.getEventListeners/parameters/objectId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the object to return listeners for.</pre>
                    ///
                    ///
                    #[serde(rename = "objectId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub object_id: crate::runtime::types::RemoteObjectId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMDebugger.command.getEventListeners/parameters/depth。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The maximum depth at which Node children should be retrieved, defaults to 1. Use -1 for the
                    ///entire subtree or provide an integer larger than 0.</pre>
                    ///
                    ///
                    #[serde(rename = "depth")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub depth: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMDebugger.command.getEventListeners/parameters/pierce。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether or not iframes and shadow roots should be traversed when returning the subtree
                    ///(default is false). Reports listeners for all contexts if pierce is enabled.</pre>
                    ///
                    ///
                    #[serde(rename = "pierce")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub pierce: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetEventListenersParams {
                    object_id: fields.object_id,
                    depth: fields.depth,
                    pierce: fields.pierce,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOMDebugger.command.getEventListeners。
///
///协议原文：
///
///<pre>Returns event listeners of the given object.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetEventListeners;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMDebugger.command.getEventListeners/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetEventListenersResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.command.getEventListeners/returns/listeners。
    ///
    ///协议原文：
    ///
    ///<pre>Array of relevant listeners.</pre>
    ///
    ///
    #[serde(rename = "listeners")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub listeners: Vec<crate::dom_debugger::types::EventListener>,
}
impl<'de> serde::Deserialize<'de> for GetEventListenersResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetEventListenersResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.command.getEventListeners/returns/listeners。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Array of relevant listeners.</pre>
                    ///
                    ///
                    #[serde(rename = "listeners")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub listeners: Vec<crate::dom_debugger::types::EventListener>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetEventListenersResponse {
                    listeners: fields.listeners,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetEventListeners {
    type Params = GetEventListenersParams;
    type Response = GetEventListenersResponse;
    const METHOD: &'static str = "DOMDebugger.getEventListeners";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMDebugger.command.removeDOMBreakpoint/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveDomBreakpointParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.command.removeDOMBreakpoint/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the node to remove breakpoint from.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.command.removeDOMBreakpoint/parameters/type。
    ///
    ///协议原文：
    ///
    ///<pre>Type of the breakpoint to remove.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: crate::dom_debugger::types::DomBreakpointType,
}
impl<'de> serde::Deserialize<'de> for RemoveDomBreakpointParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveDomBreakpointParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.command.removeDOMBreakpoint/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the node to remove breakpoint from.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.command.removeDOMBreakpoint/parameters/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Type of the breakpoint to remove.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: crate::dom_debugger::types::DomBreakpointType,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RemoveDomBreakpointParams {
                    node_id: fields.node_id,
                    r#type: fields.r#type,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOMDebugger.command.removeDOMBreakpoint。
///
///协议原文：
///
///<pre>Removes DOM breakpoint that was set using `setDOMBreakpoint`.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RemoveDomBreakpoint;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMDebugger.command.removeDOMBreakpoint/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveDomBreakpointResponse {}
impl<'de> serde::Deserialize<'de> for RemoveDomBreakpointResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveDomBreakpointResponse;
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
                Ok(RemoveDomBreakpointResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RemoveDomBreakpoint {
    type Params = RemoveDomBreakpointParams;
    type Response = RemoveDomBreakpointResponse;
    const METHOD: &'static str = "DOMDebugger.removeDOMBreakpoint";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMDebugger.command.removeEventListenerBreakpoint/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveEventListenerBreakpointParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.command.removeEventListenerBreakpoint/parameters/eventName。
    ///
    ///协议原文：
    ///
    ///<pre>Event name.</pre>
    ///
    ///
    #[serde(rename = "eventName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub event_name: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMDebugger.command.removeEventListenerBreakpoint/parameters/targetName。
    ///
    ///协议原文：
    ///
    ///<pre>EventTarget interface name.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "targetName")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_name: Option<String>,
}
impl<'de> serde::Deserialize<'de> for RemoveEventListenerBreakpointParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveEventListenerBreakpointParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.command.removeEventListenerBreakpoint/parameters/eventName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Event name.</pre>
                    ///
                    ///
                    #[serde(rename = "eventName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub event_name: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMDebugger.command.removeEventListenerBreakpoint/parameters/targetName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>EventTarget interface name.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "targetName")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub target_name: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RemoveEventListenerBreakpointParams {
                    event_name: fields.event_name,
                    target_name: fields.target_name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOMDebugger.command.removeEventListenerBreakpoint。
///
///协议原文：
///
///<pre>Removes breakpoint on particular DOM event.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RemoveEventListenerBreakpoint;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMDebugger.command.removeEventListenerBreakpoint/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveEventListenerBreakpointResponse {}
impl<'de> serde::Deserialize<'de> for RemoveEventListenerBreakpointResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveEventListenerBreakpointResponse;
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
                Ok(RemoveEventListenerBreakpointResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RemoveEventListenerBreakpoint {
    type Params = RemoveEventListenerBreakpointParams;
    type Response = RemoveEventListenerBreakpointResponse;
    const METHOD: &'static str = "DOMDebugger.removeEventListenerBreakpoint";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMDebugger.command.removeInstrumentationBreakpoint/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveInstrumentationBreakpointParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.command.removeInstrumentationBreakpoint/parameters/eventName。
    ///
    ///协议原文：
    ///
    ///<pre>Instrumentation name to stop on.</pre>
    ///
    ///
    #[serde(rename = "eventName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub event_name: String,
}
impl<'de> serde::Deserialize<'de> for RemoveInstrumentationBreakpointParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveInstrumentationBreakpointParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.command.removeInstrumentationBreakpoint/parameters/eventName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Instrumentation name to stop on.</pre>
                    ///
                    ///
                    #[serde(rename = "eventName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub event_name: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RemoveInstrumentationBreakpointParams {
                    event_name: fields.event_name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOMDebugger.command.removeInstrumentationBreakpoint。
///
///协议原文：
///
///<pre>Removes breakpoint on particular native event.</pre>
///
///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct RemoveInstrumentationBreakpoint;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMDebugger.command.removeInstrumentationBreakpoint/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveInstrumentationBreakpointResponse {}
impl<'de> serde::Deserialize<'de> for RemoveInstrumentationBreakpointResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveInstrumentationBreakpointResponse;
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
                Ok(RemoveInstrumentationBreakpointResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RemoveInstrumentationBreakpoint {
    type Params = RemoveInstrumentationBreakpointParams;
    type Response = RemoveInstrumentationBreakpointResponse;
    const METHOD: &'static str = "DOMDebugger.removeInstrumentationBreakpoint";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMDebugger.command.removeXHRBreakpoint/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveXhrBreakpointParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.command.removeXHRBreakpoint/parameters/url。
    ///
    ///协议原文：
    ///
    ///<pre>Resource URL substring.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
}
impl<'de> serde::Deserialize<'de> for RemoveXhrBreakpointParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveXhrBreakpointParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.command.removeXHRBreakpoint/parameters/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Resource URL substring.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RemoveXhrBreakpointParams { url: fields.url })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOMDebugger.command.removeXHRBreakpoint。
///
///协议原文：
///
///<pre>Removes breakpoint from XMLHttpRequest.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RemoveXhrBreakpoint;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMDebugger.command.removeXHRBreakpoint/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveXhrBreakpointResponse {}
impl<'de> serde::Deserialize<'de> for RemoveXhrBreakpointResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveXhrBreakpointResponse;
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
                Ok(RemoveXhrBreakpointResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RemoveXhrBreakpoint {
    type Params = RemoveXhrBreakpointParams;
    type Response = RemoveXhrBreakpointResponse;
    const METHOD: &'static str = "DOMDebugger.removeXHRBreakpoint";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMDebugger.command.setBreakOnCSPViolation/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetBreakOnCspViolationParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.command.setBreakOnCSPViolation/parameters/violationTypes。
    ///
    ///协议原文：
    ///
    ///<pre>CSP Violations to stop upon.</pre>
    ///
    ///
    #[serde(rename = "violationTypes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub violation_types: Vec<crate::dom_debugger::types::CspViolationType>,
}
impl<'de> serde::Deserialize<'de> for SetBreakOnCspViolationParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetBreakOnCspViolationParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.command.setBreakOnCSPViolation/parameters/violationTypes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>CSP Violations to stop upon.</pre>
                    ///
                    ///
                    #[serde(rename = "violationTypes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub violation_types: Vec<crate::dom_debugger::types::CspViolationType>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetBreakOnCspViolationParams {
                    violation_types: fields.violation_types,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOMDebugger.command.setBreakOnCSPViolation。
///
///协议原文：
///
///<pre>Sets breakpoint on particular CSP violations.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetBreakOnCspViolation;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMDebugger.command.setBreakOnCSPViolation/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetBreakOnCspViolationResponse {}
impl<'de> serde::Deserialize<'de> for SetBreakOnCspViolationResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetBreakOnCspViolationResponse;
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
                Ok(SetBreakOnCspViolationResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetBreakOnCspViolation {
    type Params = SetBreakOnCspViolationParams;
    type Response = SetBreakOnCspViolationResponse;
    const METHOD: &'static str = "DOMDebugger.setBreakOnCSPViolation";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMDebugger.command.setDOMBreakpoint/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDomBreakpointParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.command.setDOMBreakpoint/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the node to set breakpoint on.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.command.setDOMBreakpoint/parameters/type。
    ///
    ///协议原文：
    ///
    ///<pre>Type of the operation to stop upon.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: crate::dom_debugger::types::DomBreakpointType,
}
impl<'de> serde::Deserialize<'de> for SetDomBreakpointParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDomBreakpointParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.command.setDOMBreakpoint/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the node to set breakpoint on.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.command.setDOMBreakpoint/parameters/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Type of the operation to stop upon.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: crate::dom_debugger::types::DomBreakpointType,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetDomBreakpointParams {
                    node_id: fields.node_id,
                    r#type: fields.r#type,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOMDebugger.command.setDOMBreakpoint。
///
///协议原文：
///
///<pre>Sets breakpoint on particular operation with DOM.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetDomBreakpoint;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMDebugger.command.setDOMBreakpoint/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDomBreakpointResponse {}
impl<'de> serde::Deserialize<'de> for SetDomBreakpointResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDomBreakpointResponse;
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
                Ok(SetDomBreakpointResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetDomBreakpoint {
    type Params = SetDomBreakpointParams;
    type Response = SetDomBreakpointResponse;
    const METHOD: &'static str = "DOMDebugger.setDOMBreakpoint";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMDebugger.command.setEventListenerBreakpoint/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetEventListenerBreakpointParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.command.setEventListenerBreakpoint/parameters/eventName。
    ///
    ///协议原文：
    ///
    ///<pre>DOM Event name to stop on (any DOM event will do).</pre>
    ///
    ///
    #[serde(rename = "eventName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub event_name: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMDebugger.command.setEventListenerBreakpoint/parameters/targetName。
    ///
    ///协议原文：
    ///
    ///<pre>EventTarget interface name to stop on. If equal to `"*"` or not provided, will stop on any
    ///EventTarget.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "targetName")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_name: Option<String>,
}
impl<'de> serde::Deserialize<'de> for SetEventListenerBreakpointParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetEventListenerBreakpointParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.command.setEventListenerBreakpoint/parameters/eventName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>DOM Event name to stop on (any DOM event will do).</pre>
                    ///
                    ///
                    #[serde(rename = "eventName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub event_name: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMDebugger.command.setEventListenerBreakpoint/parameters/targetName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>EventTarget interface name to stop on. If equal to `"*"` or not provided, will stop on any
                    ///EventTarget.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "targetName")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub target_name: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetEventListenerBreakpointParams {
                    event_name: fields.event_name,
                    target_name: fields.target_name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOMDebugger.command.setEventListenerBreakpoint。
///
///协议原文：
///
///<pre>Sets breakpoint on particular DOM event.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetEventListenerBreakpoint;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMDebugger.command.setEventListenerBreakpoint/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetEventListenerBreakpointResponse {}
impl<'de> serde::Deserialize<'de> for SetEventListenerBreakpointResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetEventListenerBreakpointResponse;
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
                Ok(SetEventListenerBreakpointResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetEventListenerBreakpoint {
    type Params = SetEventListenerBreakpointParams;
    type Response = SetEventListenerBreakpointResponse;
    const METHOD: &'static str = "DOMDebugger.setEventListenerBreakpoint";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMDebugger.command.setInstrumentationBreakpoint/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetInstrumentationBreakpointParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.command.setInstrumentationBreakpoint/parameters/eventName。
    ///
    ///协议原文：
    ///
    ///<pre>Instrumentation name to stop on.</pre>
    ///
    ///
    #[serde(rename = "eventName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub event_name: String,
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
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.command.setInstrumentationBreakpoint/parameters/eventName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Instrumentation name to stop on.</pre>
                    ///
                    ///
                    #[serde(rename = "eventName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub event_name: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetInstrumentationBreakpointParams {
                    event_name: fields.event_name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOMDebugger.command.setInstrumentationBreakpoint。
///
///协议原文：
///
///<pre>Sets breakpoint on particular native event.</pre>
///
///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct SetInstrumentationBreakpoint;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMDebugger.command.setInstrumentationBreakpoint/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetInstrumentationBreakpointResponse {}
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
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetInstrumentationBreakpointResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetInstrumentationBreakpoint {
    type Params = SetInstrumentationBreakpointParams;
    type Response = SetInstrumentationBreakpointResponse;
    const METHOD: &'static str = "DOMDebugger.setInstrumentationBreakpoint";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMDebugger.command.setXHRBreakpoint/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetXhrBreakpointParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.command.setXHRBreakpoint/parameters/url。
    ///
    ///协议原文：
    ///
    ///<pre>Resource URL substring. All XHRs having this substring in the URL will get stopped upon.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
}
impl<'de> serde::Deserialize<'de> for SetXhrBreakpointParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetXhrBreakpointParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.command.setXHRBreakpoint/parameters/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Resource URL substring. All XHRs having this substring in the URL will get stopped upon.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetXhrBreakpointParams { url: fields.url })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOMDebugger.command.setXHRBreakpoint。
///
///协议原文：
///
///<pre>Sets breakpoint on XMLHttpRequest.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetXhrBreakpoint;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMDebugger.command.setXHRBreakpoint/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetXhrBreakpointResponse {}
impl<'de> serde::Deserialize<'de> for SetXhrBreakpointResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetXhrBreakpointResponse;
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
                Ok(SetXhrBreakpointResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetXhrBreakpoint {
    type Params = SetXhrBreakpointParams;
    type Response = SetXhrBreakpointResponse;
    const METHOD: &'static str = "DOMDebugger.setXHRBreakpoint";
}
