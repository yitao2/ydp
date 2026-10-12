///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Accessibility.command.disable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Accessibility.command.disable。
///
///协议原文：
///
///<pre>Disables the accessibility domain.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Accessibility.command.disable/returns。
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
    const METHOD: &'static str = "Accessibility.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Accessibility.command.enable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Accessibility.command.enable。
///
///协议原文：
///
///<pre>Enables the accessibility domain which causes `AXNodeId`s to remain consistent between method calls.
///This turns on accessibility for the page, which can impact performance until accessibility is disabled.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Accessibility.command.enable/returns。
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
    const METHOD: &'static str = "Accessibility.enable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Accessibility.command.getAXNodeAndAncestors/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetAxNodeAndAncestorsParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.getAXNodeAndAncestors/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the node to get.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub node_id: Option<crate::dom::types::NodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.getAXNodeAndAncestors/parameters/backendNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the backend node to get.</pre>
    ///
    ///
    #[serde(rename = "backendNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.getAXNodeAndAncestors/parameters/objectId。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript object id of the node wrapper to get.</pre>
    ///
    ///
    #[serde(rename = "objectId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub object_id: Option<crate::runtime::types::RemoteObjectId>,
}
impl<'de> serde::Deserialize<'de> for GetAxNodeAndAncestorsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetAxNodeAndAncestorsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.getAXNodeAndAncestors/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the node to get.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub node_id: Option<crate::dom::types::NodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.getAXNodeAndAncestors/parameters/backendNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the backend node to get.</pre>
                    ///
                    ///
                    #[serde(rename = "backendNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.getAXNodeAndAncestors/parameters/objectId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript object id of the node wrapper to get.</pre>
                    ///
                    ///
                    #[serde(rename = "objectId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub object_id: Option<crate::runtime::types::RemoteObjectId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetAxNodeAndAncestorsParams {
                    node_id: fields.node_id,
                    backend_node_id: fields.backend_node_id,
                    object_id: fields.object_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Accessibility.command.getAXNodeAndAncestors。
///
///协议原文：
///
///<pre>Fetches a node and all ancestors up to and including the root.
///Requires `enable()` to have been called previously.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetAxNodeAndAncestors;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Accessibility.command.getAXNodeAndAncestors/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetAxNodeAndAncestorsResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.command.getAXNodeAndAncestors/returns/nodes。
    #[serde(rename = "nodes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub nodes: Vec<crate::accessibility::types::AxNode>,
}
impl<'de> serde::Deserialize<'de> for GetAxNodeAndAncestorsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetAxNodeAndAncestorsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.command.getAXNodeAndAncestors/returns/nodes。
                    #[serde(rename = "nodes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub nodes: Vec<crate::accessibility::types::AxNode>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetAxNodeAndAncestorsResponse {
                    nodes: fields.nodes,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetAxNodeAndAncestors {
    type Params = GetAxNodeAndAncestorsParams;
    type Response = GetAxNodeAndAncestorsResponse;
    const METHOD: &'static str = "Accessibility.getAXNodeAndAncestors";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Accessibility.command.getChildAXNodes/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetChildAxNodesParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.command.getChildAXNodes/parameters/id。
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: crate::accessibility::types::AxNodeId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.getChildAXNodes/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>The frame in whose document the node resides.
    ///If omitted, the root frame is used.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub frame_id: Option<crate::page::types::FrameId>,
}
impl<'de> serde::Deserialize<'de> for GetChildAxNodesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetChildAxNodesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.command.getChildAXNodes/parameters/id。
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: crate::accessibility::types::AxNodeId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.getChildAXNodes/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The frame in whose document the node resides.
                    ///If omitted, the root frame is used.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub frame_id: Option<crate::page::types::FrameId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetChildAxNodesParams {
                    id: fields.id,
                    frame_id: fields.frame_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Accessibility.command.getChildAXNodes。
///
///协议原文：
///
///<pre>Fetches a particular accessibility node by AXNodeId.
///Requires `enable()` to have been called previously.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetChildAxNodes;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Accessibility.command.getChildAXNodes/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetChildAxNodesResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.command.getChildAXNodes/returns/nodes。
    #[serde(rename = "nodes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub nodes: Vec<crate::accessibility::types::AxNode>,
}
impl<'de> serde::Deserialize<'de> for GetChildAxNodesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetChildAxNodesResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.command.getChildAXNodes/returns/nodes。
                    #[serde(rename = "nodes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub nodes: Vec<crate::accessibility::types::AxNode>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetChildAxNodesResponse {
                    nodes: fields.nodes,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetChildAxNodes {
    type Params = GetChildAxNodesParams;
    type Response = GetChildAxNodesResponse;
    const METHOD: &'static str = "Accessibility.getChildAXNodes";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Accessibility.command.getFullAXTree/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetFullAxTreeParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.getFullAXTree/parameters/depth。
    ///
    ///协议原文：
    ///
    ///<pre>The maximum depth at which descendants of the root node should be retrieved.
    ///If omitted, the full tree is returned.</pre>
    ///
    ///
    #[serde(rename = "depth")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub depth: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.getFullAXTree/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>The frame for whose document the AX tree should be retrieved.
    ///If omitted, the root frame is used.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub frame_id: Option<crate::page::types::FrameId>,
}
impl<'de> serde::Deserialize<'de> for GetFullAxTreeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetFullAxTreeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.getFullAXTree/parameters/depth。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The maximum depth at which descendants of the root node should be retrieved.
                    ///If omitted, the full tree is returned.</pre>
                    ///
                    ///
                    #[serde(rename = "depth")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub depth: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.getFullAXTree/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The frame for whose document the AX tree should be retrieved.
                    ///If omitted, the root frame is used.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub frame_id: Option<crate::page::types::FrameId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetFullAxTreeParams {
                    depth: fields.depth,
                    frame_id: fields.frame_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Accessibility.command.getFullAXTree。
///
///协议原文：
///
///<pre>Fetches the entire accessibility tree for the root Document</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetFullAxTree;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Accessibility.command.getFullAXTree/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetFullAxTreeResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.command.getFullAXTree/returns/nodes。
    #[serde(rename = "nodes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub nodes: Vec<crate::accessibility::types::AxNode>,
}
impl<'de> serde::Deserialize<'de> for GetFullAxTreeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetFullAxTreeResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.command.getFullAXTree/returns/nodes。
                    #[serde(rename = "nodes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub nodes: Vec<crate::accessibility::types::AxNode>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetFullAxTreeResponse {
                    nodes: fields.nodes,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetFullAxTree {
    type Params = GetFullAxTreeParams;
    type Response = GetFullAxTreeResponse;
    const METHOD: &'static str = "Accessibility.getFullAXTree";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Accessibility.command.getPartialAXTree/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetPartialAxTreeParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.getPartialAXTree/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the node to get the partial accessibility tree for.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub node_id: Option<crate::dom::types::NodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.getPartialAXTree/parameters/backendNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the backend node to get the partial accessibility tree for.</pre>
    ///
    ///
    #[serde(rename = "backendNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.getPartialAXTree/parameters/objectId。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript object id of the node wrapper to get the partial accessibility tree for.</pre>
    ///
    ///
    #[serde(rename = "objectId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub object_id: Option<crate::runtime::types::RemoteObjectId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.getPartialAXTree/parameters/fetchRelatives。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to fetch this node's ancestors, siblings and children. Defaults to true.</pre>
    ///
    ///
    #[serde(rename = "fetchRelatives")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub fetch_relatives: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for GetPartialAxTreeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetPartialAxTreeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.getPartialAXTree/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the node to get the partial accessibility tree for.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub node_id: Option<crate::dom::types::NodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.getPartialAXTree/parameters/backendNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the backend node to get the partial accessibility tree for.</pre>
                    ///
                    ///
                    #[serde(rename = "backendNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.getPartialAXTree/parameters/objectId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript object id of the node wrapper to get the partial accessibility tree for.</pre>
                    ///
                    ///
                    #[serde(rename = "objectId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub object_id: Option<crate::runtime::types::RemoteObjectId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.getPartialAXTree/parameters/fetchRelatives。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to fetch this node's ancestors, siblings and children. Defaults to true.</pre>
                    ///
                    ///
                    #[serde(rename = "fetchRelatives")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub fetch_relatives: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetPartialAxTreeParams {
                    node_id: fields.node_id,
                    backend_node_id: fields.backend_node_id,
                    object_id: fields.object_id,
                    fetch_relatives: fields.fetch_relatives,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Accessibility.command.getPartialAXTree。
///
///协议原文：
///
///<pre>Fetches the accessibility node and partial accessibility tree for this DOM node, if it exists.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetPartialAxTree;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Accessibility.command.getPartialAXTree/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetPartialAxTreeResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.command.getPartialAXTree/returns/nodes。
    ///
    ///协议原文：
    ///
    ///<pre>The `Accessibility.AXNode` for this DOM node, if it exists, plus its ancestors, siblings and
    ///children, if requested.</pre>
    ///
    ///
    #[serde(rename = "nodes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub nodes: Vec<crate::accessibility::types::AxNode>,
}
impl<'de> serde::Deserialize<'de> for GetPartialAxTreeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetPartialAxTreeResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.command.getPartialAXTree/returns/nodes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The `Accessibility.AXNode` for this DOM node, if it exists, plus its ancestors, siblings and
                    ///children, if requested.</pre>
                    ///
                    ///
                    #[serde(rename = "nodes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub nodes: Vec<crate::accessibility::types::AxNode>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetPartialAxTreeResponse {
                    nodes: fields.nodes,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetPartialAxTree {
    type Params = GetPartialAxTreeParams;
    type Response = GetPartialAxTreeResponse;
    const METHOD: &'static str = "Accessibility.getPartialAXTree";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Accessibility.command.getRootAXNode/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetRootAxNodeParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.getRootAXNode/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>The frame in whose document the node resides.
    ///If omitted, the root frame is used.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub frame_id: Option<crate::page::types::FrameId>,
}
impl<'de> serde::Deserialize<'de> for GetRootAxNodeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetRootAxNodeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.getRootAXNode/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The frame in whose document the node resides.
                    ///If omitted, the root frame is used.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub frame_id: Option<crate::page::types::FrameId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetRootAxNodeParams {
                    frame_id: fields.frame_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Accessibility.command.getRootAXNode。
///
///协议原文：
///
///<pre>Fetches the root node.
///Requires `enable()` to have been called previously.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetRootAxNode;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Accessibility.command.getRootAXNode/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetRootAxNodeResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.command.getRootAXNode/returns/node。
    #[serde(rename = "node")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node: crate::accessibility::types::AxNode,
}
impl<'de> serde::Deserialize<'de> for GetRootAxNodeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetRootAxNodeResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.command.getRootAXNode/returns/node。
                    #[serde(rename = "node")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node: crate::accessibility::types::AxNode,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetRootAxNodeResponse { node: fields.node })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetRootAxNode {
    type Params = GetRootAxNodeParams;
    type Response = GetRootAxNodeResponse;
    const METHOD: &'static str = "Accessibility.getRootAXNode";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Accessibility.command.queryAXTree/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct QueryAxTreeParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.queryAXTree/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the node for the root to query.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub node_id: Option<crate::dom::types::NodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.queryAXTree/parameters/backendNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the backend node for the root to query.</pre>
    ///
    ///
    #[serde(rename = "backendNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.queryAXTree/parameters/objectId。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript object id of the node wrapper for the root to query.</pre>
    ///
    ///
    #[serde(rename = "objectId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub object_id: Option<crate::runtime::types::RemoteObjectId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.queryAXTree/parameters/accessibleName。
    ///
    ///协议原文：
    ///
    ///<pre>Find nodes with this computed name.</pre>
    ///
    ///
    #[serde(rename = "accessibleName")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub accessible_name: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.queryAXTree/parameters/role。
    ///
    ///协议原文：
    ///
    ///<pre>Find nodes with this computed role.</pre>
    ///
    ///
    #[serde(rename = "role")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub role: Option<String>,
}
impl<'de> serde::Deserialize<'de> for QueryAxTreeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = QueryAxTreeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.queryAXTree/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the node for the root to query.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub node_id: Option<crate::dom::types::NodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.queryAXTree/parameters/backendNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the backend node for the root to query.</pre>
                    ///
                    ///
                    #[serde(rename = "backendNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.queryAXTree/parameters/objectId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript object id of the node wrapper for the root to query.</pre>
                    ///
                    ///
                    #[serde(rename = "objectId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub object_id: Option<crate::runtime::types::RemoteObjectId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.queryAXTree/parameters/accessibleName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Find nodes with this computed name.</pre>
                    ///
                    ///
                    #[serde(rename = "accessibleName")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub accessible_name: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.command.queryAXTree/parameters/role。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Find nodes with this computed role.</pre>
                    ///
                    ///
                    #[serde(rename = "role")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub role: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(QueryAxTreeParams {
                    node_id: fields.node_id,
                    backend_node_id: fields.backend_node_id,
                    object_id: fields.object_id,
                    accessible_name: fields.accessible_name,
                    role: fields.role,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Accessibility.command.queryAXTree。
///
///协议原文：
///
///<pre>Query a DOM node's accessibility subtree for accessible name and role.
///This command computes the name and role for all nodes in the subtree, including those that are
///ignored for accessibility, and returns those that match the specified name and role. If no DOM
///node is specified, or the DOM node does not exist, the command returns an error. If neither
///`accessibleName` or `role` is specified, it returns all the accessibility nodes in the subtree.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct QueryAxTree;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Accessibility.command.queryAXTree/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct QueryAxTreeResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.command.queryAXTree/returns/nodes。
    ///
    ///协议原文：
    ///
    ///<pre>A list of `Accessibility.AXNode` matching the specified attributes,
    ///including nodes that are ignored for accessibility.</pre>
    ///
    ///
    #[serde(rename = "nodes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub nodes: Vec<crate::accessibility::types::AxNode>,
}
impl<'de> serde::Deserialize<'de> for QueryAxTreeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = QueryAxTreeResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.command.queryAXTree/returns/nodes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A list of `Accessibility.AXNode` matching the specified attributes,
                    ///including nodes that are ignored for accessibility.</pre>
                    ///
                    ///
                    #[serde(rename = "nodes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub nodes: Vec<crate::accessibility::types::AxNode>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(QueryAxTreeResponse {
                    nodes: fields.nodes,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for QueryAxTree {
    type Params = QueryAxTreeParams;
    type Response = QueryAxTreeResponse;
    const METHOD: &'static str = "Accessibility.queryAXTree";
}
