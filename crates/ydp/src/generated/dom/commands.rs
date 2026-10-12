///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.collectClassNamesFromSubtree/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CollectClassNamesFromSubtreeParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.collectClassNamesFromSubtree/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node to collect class names.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for CollectClassNamesFromSubtreeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CollectClassNamesFromSubtreeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.collectClassNamesFromSubtree/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node to collect class names.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CollectClassNamesFromSubtreeParams {
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.collectClassNamesFromSubtree。
///
///协议原文：
///
///<pre>Collects class names for the node with given id and all of it's child nodes.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct CollectClassNamesFromSubtree;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.collectClassNamesFromSubtree/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CollectClassNamesFromSubtreeResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.collectClassNamesFromSubtree/returns/classNames。
    ///
    ///协议原文：
    ///
    ///<pre>Class name list.</pre>
    ///
    ///
    #[serde(rename = "classNames")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub class_names: Vec<String>,
}
impl<'de> serde::Deserialize<'de> for CollectClassNamesFromSubtreeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CollectClassNamesFromSubtreeResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.collectClassNamesFromSubtree/returns/classNames。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Class name list.</pre>
                    ///
                    ///
                    #[serde(rename = "classNames")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub class_names: Vec<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CollectClassNamesFromSubtreeResponse {
                    class_names: fields.class_names,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for CollectClassNamesFromSubtree {
    type Params = CollectClassNamesFromSubtreeParams;
    type Response = CollectClassNamesFromSubtreeResponse;
    const METHOD: &'static str = "DOM.collectClassNamesFromSubtree";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.copyTo/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CopyToParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.copyTo/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node to copy.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.copyTo/parameters/targetNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the element to drop the copy into.</pre>
    ///
    ///
    #[serde(rename = "targetNodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub target_node_id: crate::dom::types::NodeId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.copyTo/parameters/insertBeforeNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Drop the copy before this node (if absent, the copy becomes the last child of
    ///`targetNodeId`).</pre>
    ///
    ///
    #[serde(rename = "insertBeforeNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub insert_before_node_id: Option<crate::dom::types::NodeId>,
}
impl<'de> serde::Deserialize<'de> for CopyToParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CopyToParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.copyTo/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node to copy.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.copyTo/parameters/targetNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the element to drop the copy into.</pre>
                    ///
                    ///
                    #[serde(rename = "targetNodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub target_node_id: crate::dom::types::NodeId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.copyTo/parameters/insertBeforeNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Drop the copy before this node (if absent, the copy becomes the last child of
                    ///`targetNodeId`).</pre>
                    ///
                    ///
                    #[serde(rename = "insertBeforeNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub insert_before_node_id: Option<crate::dom::types::NodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CopyToParams {
                    node_id: fields.node_id,
                    target_node_id: fields.target_node_id,
                    insert_before_node_id: fields.insert_before_node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.copyTo。
///
///协议原文：
///
///<pre>Creates a deep copy of the specified node and places it into the target container before the
///given anchor.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct CopyTo;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.copyTo/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CopyToResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.copyTo/returns/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node clone.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for CopyToResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CopyToResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.copyTo/returns/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node clone.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CopyToResponse {
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for CopyTo {
    type Params = CopyToParams;
    type Response = CopyToResponse;
    const METHOD: &'static str = "DOM.copyTo";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.describeNode/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct DescribeNodeParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.describeNode/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the node.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub node_id: Option<crate::dom::types::NodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.describeNode/parameters/backendNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the backend node.</pre>
    ///
    ///
    #[serde(rename = "backendNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.describeNode/parameters/objectId。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript object id of the node wrapper.</pre>
    ///
    ///
    #[serde(rename = "objectId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub object_id: Option<crate::runtime::types::RemoteObjectId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.describeNode/parameters/depth。
    ///
    ///协议原文：
    ///
    ///<pre>The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
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
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.describeNode/parameters/pierce。
    ///
    ///协议原文：
    ///
    ///<pre>Whether or not iframes and shadow roots should be traversed when returning the subtree
    ///(default is false).</pre>
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
impl<'de> serde::Deserialize<'de> for DescribeNodeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DescribeNodeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.describeNode/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the node.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub node_id: Option<crate::dom::types::NodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.describeNode/parameters/backendNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the backend node.</pre>
                    ///
                    ///
                    #[serde(rename = "backendNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.describeNode/parameters/objectId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript object id of the node wrapper.</pre>
                    ///
                    ///
                    #[serde(rename = "objectId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub object_id: Option<crate::runtime::types::RemoteObjectId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.describeNode/parameters/depth。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
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
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.describeNode/parameters/pierce。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether or not iframes and shadow roots should be traversed when returning the subtree
                    ///(default is false).</pre>
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
                Ok(DescribeNodeParams {
                    node_id: fields.node_id,
                    backend_node_id: fields.backend_node_id,
                    object_id: fields.object_id,
                    depth: fields.depth,
                    pierce: fields.pierce,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.describeNode。
///
///协议原文：
///
///<pre>Describes node given its id, does not require domain to be enabled. Does not start tracking any
///objects, can be used for automation.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct DescribeNode;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.describeNode/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DescribeNodeResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.describeNode/returns/node。
    ///
    ///协议原文：
    ///
    ///<pre>Node description.</pre>
    ///
    ///
    #[serde(rename = "node")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node: crate::dom::types::Node,
}
impl<'de> serde::Deserialize<'de> for DescribeNodeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DescribeNodeResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.describeNode/returns/node。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Node description.</pre>
                    ///
                    ///
                    #[serde(rename = "node")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node: crate::dom::types::Node,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DescribeNodeResponse { node: fields.node })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for DescribeNode {
    type Params = DescribeNodeParams;
    type Response = DescribeNodeResponse;
    const METHOD: &'static str = "DOM.describeNode";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.disable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.disable。
///
///协议原文：
///
///<pre>Disables DOM agent for the given page.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.disable/returns。
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
    const METHOD: &'static str = "DOM.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.discardSearchResults/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DiscardSearchResultsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.discardSearchResults/parameters/searchId。
    ///
    ///协议原文：
    ///
    ///<pre>Unique search session identifier.</pre>
    ///
    ///
    #[serde(rename = "searchId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub search_id: String,
}
impl<'de> serde::Deserialize<'de> for DiscardSearchResultsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DiscardSearchResultsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.discardSearchResults/parameters/searchId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Unique search session identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "searchId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub search_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DiscardSearchResultsParams {
                    search_id: fields.search_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.discardSearchResults。
///
///协议原文：
///
///<pre>Discards search results from the session with the given id. `getSearchResults` should no longer
///be called for that search.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct DiscardSearchResults;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.discardSearchResults/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DiscardSearchResultsResponse {}
impl<'de> serde::Deserialize<'de> for DiscardSearchResultsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DiscardSearchResultsResponse;
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
                Ok(DiscardSearchResultsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for DiscardSearchResults {
    type Params = DiscardSearchResultsParams;
    type Response = DiscardSearchResultsResponse;
    const METHOD: &'static str = "DOM.discardSearchResults";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：DOM.command.enable/parameters/includeWhitespace。
///
///协议原文：
///
///<pre>Whether to include whitespaces in the children array of returned Nodes.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum EnableParamsIncludeWhitespace {
    ///协议值 "none"。
    None,
    ///协议值 "all"。
    All,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl EnableParamsIncludeWhitespace {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::dom::commands::EnableParamsIncludeWhitespace::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::None => "none",
            Self::All => "all",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for EnableParamsIncludeWhitespace {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for EnableParamsIncludeWhitespace {
    fn from(value: String) -> Self {
        match value.as_str() {
            "none" => EnableParamsIncludeWhitespace::None,
            "all" => EnableParamsIncludeWhitespace::All,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for EnableParamsIncludeWhitespace {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for EnableParamsIncludeWhitespace {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EnableParamsIncludeWhitespace;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "none" => EnableParamsIncludeWhitespace::None,
                    "all" => EnableParamsIncludeWhitespace::All,
                    _ => EnableParamsIncludeWhitespace::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(EnableParamsIncludeWhitespace::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.enable/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct EnableParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.enable/parameters/includeWhitespace。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to include whitespaces in the children array of returned Nodes.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "includeWhitespace")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_whitespace: Option<EnableParamsIncludeWhitespace>,
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
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.enable/parameters/includeWhitespace。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to include whitespaces in the children array of returned Nodes.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "includeWhitespace")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub include_whitespace: Option<EnableParamsIncludeWhitespace>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EnableParams {
                    include_whitespace: fields.include_whitespace,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.enable。
///
///协议原文：
///
///<pre>Enables DOM agent for the given page.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.enable/returns。
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
    const METHOD: &'static str = "DOM.enable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.focus/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct FocusParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.focus/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the node.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub node_id: Option<crate::dom::types::NodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.focus/parameters/backendNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the backend node.</pre>
    ///
    ///
    #[serde(rename = "backendNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.focus/parameters/objectId。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript object id of the node wrapper.</pre>
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
impl<'de> serde::Deserialize<'de> for FocusParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FocusParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.focus/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the node.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub node_id: Option<crate::dom::types::NodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.focus/parameters/backendNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the backend node.</pre>
                    ///
                    ///
                    #[serde(rename = "backendNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.focus/parameters/objectId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript object id of the node wrapper.</pre>
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
                Ok(FocusParams {
                    node_id: fields.node_id,
                    backend_node_id: fields.backend_node_id,
                    object_id: fields.object_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.focus。
///
///协议原文：
///
///<pre>Focuses the given element.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Focus;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.focus/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FocusResponse {}
impl<'de> serde::Deserialize<'de> for FocusResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FocusResponse;
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
                Ok(FocusResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Focus {
    type Params = FocusParams;
    type Response = FocusResponse;
    const METHOD: &'static str = "DOM.focus";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.forceShowInterest/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ForceShowInterestParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.forceShowInterest/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the interest invoker HTMLElement.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.forceShowInterest/parameters/enable。
    ///
    ///协议原文：
    ///
    ///<pre>If true, opens and holds interest. If false, releases forced interest.</pre>
    ///
    ///
    #[serde(rename = "enable")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub enable: bool,
}
impl<'de> serde::Deserialize<'de> for ForceShowInterestParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ForceShowInterestParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.forceShowInterest/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the interest invoker HTMLElement.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.forceShowInterest/parameters/enable。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, opens and holds interest. If false, releases forced interest.</pre>
                    ///
                    ///
                    #[serde(rename = "enable")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub enable: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ForceShowInterestParams {
                    node_id: fields.node_id,
                    enable: fields.enable,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.forceShowInterest。
///
///协议原文：
///
///<pre>When enabling, this API forces an element to gain interest in its target,
///keeping interest active until disabled.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ForceShowInterest;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.forceShowInterest/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ForceShowInterestResponse {}
impl<'de> serde::Deserialize<'de> for ForceShowInterestResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ForceShowInterestResponse;
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
                Ok(ForceShowInterestResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ForceShowInterest {
    type Params = ForceShowInterestParams;
    type Response = ForceShowInterestResponse;
    const METHOD: &'static str = "DOM.forceShowInterest";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.forceShowPopover/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ForceShowPopoverParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.forceShowPopover/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the popover HTMLElement</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.forceShowPopover/parameters/enable。
    ///
    ///协议原文：
    ///
    ///<pre>If true, opens the popover and keeps it open. If false, closes the
    ///popover if it was previously force-opened.</pre>
    ///
    ///
    #[serde(rename = "enable")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub enable: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.forceShowPopover/parameters/invokerNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Optional ID of the element invoking this popover, used to establish the implicit anchor.
    ///If not provided, it will fall back to the first invoker in the document, preferring
    ///elements with a popovertarget attribute over those with a commandfor attribute. Note that
    ///if there are multiple invokers, this is just an estimate.</pre>
    ///
    ///
    #[serde(rename = "invokerNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub invoker_node_id: Option<crate::dom::types::BackendNodeId>,
}
impl<'de> serde::Deserialize<'de> for ForceShowPopoverParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ForceShowPopoverParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.forceShowPopover/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the popover HTMLElement</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.forceShowPopover/parameters/enable。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, opens the popover and keeps it open. If false, closes the
                    ///popover if it was previously force-opened.</pre>
                    ///
                    ///
                    #[serde(rename = "enable")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub enable: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.forceShowPopover/parameters/invokerNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Optional ID of the element invoking this popover, used to establish the implicit anchor.
                    ///If not provided, it will fall back to the first invoker in the document, preferring
                    ///elements with a popovertarget attribute over those with a commandfor attribute. Note that
                    ///if there are multiple invokers, this is just an estimate.</pre>
                    ///
                    ///
                    #[serde(rename = "invokerNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub invoker_node_id: Option<crate::dom::types::BackendNodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ForceShowPopoverParams {
                    node_id: fields.node_id,
                    enable: fields.enable,
                    invoker_node_id: fields.invoker_node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.forceShowPopover。
///
///协议原文：
///
///<pre>When enabling, this API force-opens the popover identified by nodeId
///and keeps it open until disabled.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ForceShowPopover;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.forceShowPopover/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ForceShowPopoverResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.forceShowPopover/returns/nodeIds。
    ///
    ///协议原文：
    ///
    ///<pre>List of popovers that were closed in order to respect popover stacking order.</pre>
    ///
    ///
    #[serde(rename = "nodeIds")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_ids: Vec<crate::dom::types::NodeId>,
}
impl<'de> serde::Deserialize<'de> for ForceShowPopoverResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ForceShowPopoverResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.forceShowPopover/returns/nodeIds。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>List of popovers that were closed in order to respect popover stacking order.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeIds")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_ids: Vec<crate::dom::types::NodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ForceShowPopoverResponse {
                    node_ids: fields.node_ids,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ForceShowPopover {
    type Params = ForceShowPopoverParams;
    type Response = ForceShowPopoverResponse;
    const METHOD: &'static str = "DOM.forceShowPopover";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getAnchorElement/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetAnchorElementParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getAnchorElement/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the positioned element from which to find the anchor.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getAnchorElement/parameters/anchorSpecifier。
    ///
    ///协议原文：
    ///
    ///<pre>An optional anchor specifier, as defined in
    ///https://www.w3.org/TR/css-anchor-position-1/#anchor-specifier.
    ///If not provided, it will return the implicit anchor element for
    ///the given positioned element.</pre>
    ///
    ///
    #[serde(rename = "anchorSpecifier")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub anchor_specifier: Option<String>,
}
impl<'de> serde::Deserialize<'de> for GetAnchorElementParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetAnchorElementParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getAnchorElement/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the positioned element from which to find the anchor.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getAnchorElement/parameters/anchorSpecifier。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An optional anchor specifier, as defined in
                    ///https://www.w3.org/TR/css-anchor-position-1/#anchor-specifier.
                    ///If not provided, it will return the implicit anchor element for
                    ///the given positioned element.</pre>
                    ///
                    ///
                    #[serde(rename = "anchorSpecifier")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub anchor_specifier: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetAnchorElementParams {
                    node_id: fields.node_id,
                    anchor_specifier: fields.anchor_specifier,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.getAnchorElement。
///
///协议原文：
///
///<pre>Returns the target anchor element of the given anchor query according to
///https://www.w3.org/TR/css-anchor-position-1/#target.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetAnchorElement;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getAnchorElement/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetAnchorElementResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getAnchorElement/returns/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>The anchor element of the given anchor query.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for GetAnchorElementResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetAnchorElementResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getAnchorElement/returns/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The anchor element of the given anchor query.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetAnchorElementResponse {
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetAnchorElement {
    type Params = GetAnchorElementParams;
    type Response = GetAnchorElementResponse;
    const METHOD: &'static str = "DOM.getAnchorElement";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getAttributes/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetAttributesParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getAttributes/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node to retrieve attributes for.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for GetAttributesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetAttributesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getAttributes/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node to retrieve attributes for.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetAttributesParams {
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.getAttributes。
///
///协议原文：
///
///<pre>Returns attributes for the specified node.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetAttributes;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getAttributes/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetAttributesResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getAttributes/returns/attributes。
    ///
    ///协议原文：
    ///
    ///<pre>An interleaved array of node attribute names and values.</pre>
    ///
    ///
    #[serde(rename = "attributes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub attributes: Vec<String>,
}
impl<'de> serde::Deserialize<'de> for GetAttributesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetAttributesResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getAttributes/returns/attributes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An interleaved array of node attribute names and values.</pre>
                    ///
                    ///
                    #[serde(rename = "attributes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub attributes: Vec<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetAttributesResponse {
                    attributes: fields.attributes,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetAttributes {
    type Params = GetAttributesParams;
    type Response = GetAttributesResponse;
    const METHOD: &'static str = "DOM.getAttributes";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getBoxModel/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetBoxModelParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getBoxModel/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the node.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub node_id: Option<crate::dom::types::NodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getBoxModel/parameters/backendNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the backend node.</pre>
    ///
    ///
    #[serde(rename = "backendNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getBoxModel/parameters/objectId。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript object id of the node wrapper.</pre>
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
impl<'de> serde::Deserialize<'de> for GetBoxModelParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetBoxModelParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getBoxModel/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the node.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub node_id: Option<crate::dom::types::NodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getBoxModel/parameters/backendNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the backend node.</pre>
                    ///
                    ///
                    #[serde(rename = "backendNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getBoxModel/parameters/objectId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript object id of the node wrapper.</pre>
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
                Ok(GetBoxModelParams {
                    node_id: fields.node_id,
                    backend_node_id: fields.backend_node_id,
                    object_id: fields.object_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.getBoxModel。
///
///协议原文：
///
///<pre>Returns boxes for the given node.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetBoxModel;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getBoxModel/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetBoxModelResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getBoxModel/returns/model。
    ///
    ///协议原文：
    ///
    ///<pre>Box model for the node.</pre>
    ///
    ///
    #[serde(rename = "model")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub model: crate::dom::types::BoxModel,
}
impl<'de> serde::Deserialize<'de> for GetBoxModelResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetBoxModelResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getBoxModel/returns/model。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Box model for the node.</pre>
                    ///
                    ///
                    #[serde(rename = "model")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub model: crate::dom::types::BoxModel,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetBoxModelResponse {
                    model: fields.model,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetBoxModel {
    type Params = GetBoxModelParams;
    type Response = GetBoxModelResponse;
    const METHOD: &'static str = "DOM.getBoxModel";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getContainerForNode/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetContainerForNodeParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getContainerForNode/parameters/nodeId。
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getContainerForNode/parameters/containerName。
    #[serde(rename = "containerName")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub container_name: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getContainerForNode/parameters/physicalAxes。
    #[serde(rename = "physicalAxes")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub physical_axes: Option<crate::dom::types::PhysicalAxes>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getContainerForNode/parameters/logicalAxes。
    #[serde(rename = "logicalAxes")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub logical_axes: Option<crate::dom::types::LogicalAxes>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getContainerForNode/parameters/queriesScrollState。
    #[serde(rename = "queriesScrollState")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub queries_scroll_state: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getContainerForNode/parameters/queriesAnchored。
    #[serde(rename = "queriesAnchored")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub queries_anchored: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for GetContainerForNodeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetContainerForNodeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getContainerForNode/parameters/nodeId。
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getContainerForNode/parameters/containerName。
                    #[serde(rename = "containerName")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub container_name: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getContainerForNode/parameters/physicalAxes。
                    #[serde(rename = "physicalAxes")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub physical_axes: Option<crate::dom::types::PhysicalAxes>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getContainerForNode/parameters/logicalAxes。
                    #[serde(rename = "logicalAxes")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub logical_axes: Option<crate::dom::types::LogicalAxes>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getContainerForNode/parameters/queriesScrollState。
                    #[serde(rename = "queriesScrollState")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub queries_scroll_state: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getContainerForNode/parameters/queriesAnchored。
                    #[serde(rename = "queriesAnchored")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub queries_anchored: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetContainerForNodeParams {
                    node_id: fields.node_id,
                    container_name: fields.container_name,
                    physical_axes: fields.physical_axes,
                    logical_axes: fields.logical_axes,
                    queries_scroll_state: fields.queries_scroll_state,
                    queries_anchored: fields.queries_anchored,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.getContainerForNode。
///
///协议原文：
///
///<pre>Returns the query container of the given node based on container query
///conditions: containerName, physical and logical axes, and whether it queries
///scroll-state or anchored elements. If no axes are provided and
///queriesScrollState is false, the style container is returned, which is the
///direct parent or the closest element with a matching container-name.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetContainerForNode;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getContainerForNode/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetContainerForNodeResponse {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getContainerForNode/returns/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>The container node for the given node, or null if not found.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub node_id: Option<crate::dom::types::NodeId>,
}
impl<'de> serde::Deserialize<'de> for GetContainerForNodeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetContainerForNodeResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getContainerForNode/returns/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The container node for the given node, or null if not found.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub node_id: Option<crate::dom::types::NodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetContainerForNodeResponse {
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetContainerForNode {
    type Params = GetContainerForNodeParams;
    type Response = GetContainerForNodeResponse;
    const METHOD: &'static str = "DOM.getContainerForNode";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getContentQuads/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetContentQuadsParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getContentQuads/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the node.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub node_id: Option<crate::dom::types::NodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getContentQuads/parameters/backendNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the backend node.</pre>
    ///
    ///
    #[serde(rename = "backendNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getContentQuads/parameters/objectId。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript object id of the node wrapper.</pre>
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
impl<'de> serde::Deserialize<'de> for GetContentQuadsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetContentQuadsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getContentQuads/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the node.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub node_id: Option<crate::dom::types::NodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getContentQuads/parameters/backendNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the backend node.</pre>
                    ///
                    ///
                    #[serde(rename = "backendNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getContentQuads/parameters/objectId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript object id of the node wrapper.</pre>
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
                Ok(GetContentQuadsParams {
                    node_id: fields.node_id,
                    backend_node_id: fields.backend_node_id,
                    object_id: fields.object_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.getContentQuads。
///
///协议原文：
///
///<pre>Returns quads that describe node position on the page. This method
///might return multiple quads for inline nodes.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetContentQuads;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getContentQuads/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetContentQuadsResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getContentQuads/returns/quads。
    ///
    ///协议原文：
    ///
    ///<pre>Quads that describe node layout relative to viewport.</pre>
    ///
    ///
    #[serde(rename = "quads")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub quads: Vec<crate::dom::types::Quad>,
}
impl<'de> serde::Deserialize<'de> for GetContentQuadsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetContentQuadsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getContentQuads/returns/quads。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Quads that describe node layout relative to viewport.</pre>
                    ///
                    ///
                    #[serde(rename = "quads")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub quads: Vec<crate::dom::types::Quad>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetContentQuadsResponse {
                    quads: fields.quads,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetContentQuads {
    type Params = GetContentQuadsParams;
    type Response = GetContentQuadsResponse;
    const METHOD: &'static str = "DOM.getContentQuads";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getDetachedDomNodes/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetDetachedDomNodesParams {}
impl<'de> serde::Deserialize<'de> for GetDetachedDomNodesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetDetachedDomNodesParams;
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
                Ok(GetDetachedDomNodesParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.getDetachedDomNodes。
///
///协议原文：
///
///<pre>Returns list of detached nodes</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetDetachedDomNodes;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getDetachedDomNodes/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetDetachedDomNodesResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getDetachedDomNodes/returns/detachedNodes。
    ///
    ///协议原文：
    ///
    ///<pre>The list of detached nodes</pre>
    ///
    ///
    #[serde(rename = "detachedNodes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub detached_nodes: Vec<crate::dom::types::DetachedElementInfo>,
}
impl<'de> serde::Deserialize<'de> for GetDetachedDomNodesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetDetachedDomNodesResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getDetachedDomNodes/returns/detachedNodes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The list of detached nodes</pre>
                    ///
                    ///
                    #[serde(rename = "detachedNodes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub detached_nodes: Vec<crate::dom::types::DetachedElementInfo>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetDetachedDomNodesResponse {
                    detached_nodes: fields.detached_nodes,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetDetachedDomNodes {
    type Params = GetDetachedDomNodesParams;
    type Response = GetDetachedDomNodesResponse;
    const METHOD: &'static str = "DOM.getDetachedDomNodes";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getDocument/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetDocumentParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getDocument/parameters/depth。
    ///
    ///协议原文：
    ///
    ///<pre>The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
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
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getDocument/parameters/pierce。
    ///
    ///协议原文：
    ///
    ///<pre>Whether or not iframes and shadow roots should be traversed when returning the subtree
    ///(default is false).</pre>
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
impl<'de> serde::Deserialize<'de> for GetDocumentParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetDocumentParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getDocument/parameters/depth。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
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
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getDocument/parameters/pierce。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether or not iframes and shadow roots should be traversed when returning the subtree
                    ///(default is false).</pre>
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
                Ok(GetDocumentParams {
                    depth: fields.depth,
                    pierce: fields.pierce,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.getDocument。
///
///协议原文：
///
///<pre>Returns the root DOM node (and optionally the subtree) to the caller.
///Implicitly enables the DOM domain events for the current target.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetDocument;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getDocument/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetDocumentResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getDocument/returns/root。
    ///
    ///协议原文：
    ///
    ///<pre>Resulting node.</pre>
    ///
    ///
    #[serde(rename = "root")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub root: crate::dom::types::Node,
}
impl<'de> serde::Deserialize<'de> for GetDocumentResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetDocumentResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getDocument/returns/root。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Resulting node.</pre>
                    ///
                    ///
                    #[serde(rename = "root")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub root: crate::dom::types::Node,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetDocumentResponse { root: fields.root })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetDocument {
    type Params = GetDocumentParams;
    type Response = GetDocumentResponse;
    const METHOD: &'static str = "DOM.getDocument";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：DOM.command.getElementByRelation/parameters/relation。
///
///协议原文：
///
///<pre>Type of relation to get.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum GetElementByRelationParamsRelation {
    ///协议值 "PopoverTarget"。
    PopoverTarget,
    ///协议值 "InterestTarget"。
    InterestTarget,
    ///协议值 "CommandFor"。
    CommandFor,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl GetElementByRelationParamsRelation {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::dom::commands::GetElementByRelationParamsRelation::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::PopoverTarget => "PopoverTarget",
            Self::InterestTarget => "InterestTarget",
            Self::CommandFor => "CommandFor",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for GetElementByRelationParamsRelation {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for GetElementByRelationParamsRelation {
    fn from(value: String) -> Self {
        match value.as_str() {
            "PopoverTarget" => GetElementByRelationParamsRelation::PopoverTarget,
            "InterestTarget" => GetElementByRelationParamsRelation::InterestTarget,
            "CommandFor" => GetElementByRelationParamsRelation::CommandFor,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for GetElementByRelationParamsRelation {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GetElementByRelationParamsRelation {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetElementByRelationParamsRelation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "PopoverTarget" => GetElementByRelationParamsRelation::PopoverTarget,
                    "InterestTarget" => GetElementByRelationParamsRelation::InterestTarget,
                    "CommandFor" => GetElementByRelationParamsRelation::CommandFor,
                    _ => GetElementByRelationParamsRelation::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(GetElementByRelationParamsRelation::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getElementByRelation/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetElementByRelationParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getElementByRelation/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node from which to query the relation.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getElementByRelation/parameters/relation。
    ///
    ///协议原文：
    ///
    ///<pre>Type of relation to get.</pre>
    ///
    ///
    #[serde(rename = "relation")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub relation: GetElementByRelationParamsRelation,
}
impl<'de> serde::Deserialize<'de> for GetElementByRelationParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetElementByRelationParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getElementByRelation/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node from which to query the relation.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getElementByRelation/parameters/relation。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Type of relation to get.</pre>
                    ///
                    ///
                    #[serde(rename = "relation")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub relation: GetElementByRelationParamsRelation,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetElementByRelationParams {
                    node_id: fields.node_id,
                    relation: fields.relation,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.getElementByRelation。
///
///协议原文：
///
///<pre>Returns the NodeId of the matched element according to certain relations.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetElementByRelation;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getElementByRelation/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetElementByRelationResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getElementByRelation/returns/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>NodeId of the element matching the queried relation.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for GetElementByRelationResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetElementByRelationResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getElementByRelation/returns/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>NodeId of the element matching the queried relation.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetElementByRelationResponse {
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetElementByRelation {
    type Params = GetElementByRelationParams;
    type Response = GetElementByRelationResponse;
    const METHOD: &'static str = "DOM.getElementByRelation";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getFileInfo/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetFileInfoParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getFileInfo/parameters/objectId。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript object id of the node wrapper.</pre>
    ///
    ///
    #[serde(rename = "objectId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub object_id: crate::runtime::types::RemoteObjectId,
}
impl<'de> serde::Deserialize<'de> for GetFileInfoParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetFileInfoParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getFileInfo/parameters/objectId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript object id of the node wrapper.</pre>
                    ///
                    ///
                    #[serde(rename = "objectId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub object_id: crate::runtime::types::RemoteObjectId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetFileInfoParams {
                    object_id: fields.object_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.getFileInfo。
///
///协议原文：
///
///<pre>Returns file information for the given
///File wrapper.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetFileInfo;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getFileInfo/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetFileInfoResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getFileInfo/returns/path。
    #[serde(rename = "path")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub path: String,
}
impl<'de> serde::Deserialize<'de> for GetFileInfoResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetFileInfoResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getFileInfo/returns/path。
                    #[serde(rename = "path")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub path: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetFileInfoResponse { path: fields.path })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetFileInfo {
    type Params = GetFileInfoParams;
    type Response = GetFileInfoResponse;
    const METHOD: &'static str = "DOM.getFileInfo";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getFlattenedDocument/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetFlattenedDocumentParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getFlattenedDocument/parameters/depth。
    ///
    ///协议原文：
    ///
    ///<pre>The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
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
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getFlattenedDocument/parameters/pierce。
    ///
    ///协议原文：
    ///
    ///<pre>Whether or not iframes and shadow roots should be traversed when returning the subtree
    ///(default is false).</pre>
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
impl<'de> serde::Deserialize<'de> for GetFlattenedDocumentParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetFlattenedDocumentParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getFlattenedDocument/parameters/depth。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
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
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getFlattenedDocument/parameters/pierce。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether or not iframes and shadow roots should be traversed when returning the subtree
                    ///(default is false).</pre>
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
                Ok(GetFlattenedDocumentParams {
                    depth: fields.depth,
                    pierce: fields.pierce,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.getFlattenedDocument。
///
///协议原文：
///
///<pre>Returns the root DOM node (and optionally the subtree) to the caller.
///Deprecated, as it is not designed to work well with the rest of the DOM agent.
///Use DOMSnapshot.captureSnapshot instead.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct GetFlattenedDocument;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getFlattenedDocument/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetFlattenedDocumentResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getFlattenedDocument/returns/nodes。
    ///
    ///协议原文：
    ///
    ///<pre>Resulting node.</pre>
    ///
    ///
    #[serde(rename = "nodes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub nodes: Vec<crate::dom::types::Node>,
}
impl<'de> serde::Deserialize<'de> for GetFlattenedDocumentResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetFlattenedDocumentResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getFlattenedDocument/returns/nodes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Resulting node.</pre>
                    ///
                    ///
                    #[serde(rename = "nodes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub nodes: Vec<crate::dom::types::Node>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetFlattenedDocumentResponse {
                    nodes: fields.nodes,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetFlattenedDocument {
    type Params = GetFlattenedDocumentParams;
    type Response = GetFlattenedDocumentResponse;
    const METHOD: &'static str = "DOM.getFlattenedDocument";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getFrameOwner/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetFrameOwnerParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getFrameOwner/parameters/frameId。
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
}
impl<'de> serde::Deserialize<'de> for GetFrameOwnerParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetFrameOwnerParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getFrameOwner/parameters/frameId。
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetFrameOwnerParams {
                    frame_id: fields.frame_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.getFrameOwner。
///
///协议原文：
///
///<pre>Returns iframe node that owns iframe with the given domain.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetFrameOwner;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getFrameOwner/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetFrameOwnerResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getFrameOwner/returns/backendNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Resulting node.</pre>
    ///
    ///
    #[serde(rename = "backendNodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub backend_node_id: crate::dom::types::BackendNodeId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getFrameOwner/returns/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node at given coordinates, only when enabled and requested document.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub node_id: Option<crate::dom::types::NodeId>,
}
impl<'de> serde::Deserialize<'de> for GetFrameOwnerResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetFrameOwnerResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getFrameOwner/returns/backendNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Resulting node.</pre>
                    ///
                    ///
                    #[serde(rename = "backendNodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub backend_node_id: crate::dom::types::BackendNodeId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getFrameOwner/returns/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node at given coordinates, only when enabled and requested document.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub node_id: Option<crate::dom::types::NodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetFrameOwnerResponse {
                    backend_node_id: fields.backend_node_id,
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetFrameOwner {
    type Params = GetFrameOwnerParams;
    type Response = GetFrameOwnerResponse;
    const METHOD: &'static str = "DOM.getFrameOwner";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getNodeForLocation/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetNodeForLocationParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getNodeForLocation/parameters/x。
    ///
    ///协议原文：
    ///
    ///<pre>X coordinate.</pre>
    ///
    ///
    #[serde(rename = "x")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub x: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getNodeForLocation/parameters/y。
    ///
    ///协议原文：
    ///
    ///<pre>Y coordinate.</pre>
    ///
    ///
    #[serde(rename = "y")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub y: i64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getNodeForLocation/parameters/includeUserAgentShadowDOM。
    ///
    ///协议原文：
    ///
    ///<pre>False to skip to the nearest non-UA shadow root ancestor (default: false).</pre>
    ///
    ///
    #[serde(rename = "includeUserAgentShadowDOM")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_user_agent_shadow_dom: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getNodeForLocation/parameters/ignorePointerEventsNone。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to ignore pointer-events: none on elements and hit test them.</pre>
    ///
    ///
    #[serde(rename = "ignorePointerEventsNone")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_pointer_events_none: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for GetNodeForLocationParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetNodeForLocationParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getNodeForLocation/parameters/x。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>X coordinate.</pre>
                    ///
                    ///
                    #[serde(rename = "x")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub x: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getNodeForLocation/parameters/y。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Y coordinate.</pre>
                    ///
                    ///
                    #[serde(rename = "y")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub y: i64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getNodeForLocation/parameters/includeUserAgentShadowDOM。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>False to skip to the nearest non-UA shadow root ancestor (default: false).</pre>
                    ///
                    ///
                    #[serde(rename = "includeUserAgentShadowDOM")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub include_user_agent_shadow_dom: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getNodeForLocation/parameters/ignorePointerEventsNone。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to ignore pointer-events: none on elements and hit test them.</pre>
                    ///
                    ///
                    #[serde(rename = "ignorePointerEventsNone")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub ignore_pointer_events_none: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetNodeForLocationParams {
                    x: fields.x,
                    y: fields.y,
                    include_user_agent_shadow_dom: fields.include_user_agent_shadow_dom,
                    ignore_pointer_events_none: fields.ignore_pointer_events_none,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.getNodeForLocation。
///
///协议原文：
///
///<pre>Returns node id at given location. Depending on whether DOM domain is enabled, nodeId is
///either returned or not.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetNodeForLocation;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getNodeForLocation/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetNodeForLocationResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getNodeForLocation/returns/backendNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Resulting node.</pre>
    ///
    ///
    #[serde(rename = "backendNodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub backend_node_id: crate::dom::types::BackendNodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getNodeForLocation/returns/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Frame this node belongs to.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getNodeForLocation/returns/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node at given coordinates, only when enabled and requested document.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub node_id: Option<crate::dom::types::NodeId>,
}
impl<'de> serde::Deserialize<'de> for GetNodeForLocationResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetNodeForLocationResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getNodeForLocation/returns/backendNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Resulting node.</pre>
                    ///
                    ///
                    #[serde(rename = "backendNodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub backend_node_id: crate::dom::types::BackendNodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getNodeForLocation/returns/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame this node belongs to.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getNodeForLocation/returns/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node at given coordinates, only when enabled and requested document.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub node_id: Option<crate::dom::types::NodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetNodeForLocationResponse {
                    backend_node_id: fields.backend_node_id,
                    frame_id: fields.frame_id,
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetNodeForLocation {
    type Params = GetNodeForLocationParams;
    type Response = GetNodeForLocationResponse;
    const METHOD: &'static str = "DOM.getNodeForLocation";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getNodeStackTraces/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetNodeStackTracesParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getNodeStackTraces/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node to get stack traces for.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for GetNodeStackTracesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetNodeStackTracesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getNodeStackTraces/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node to get stack traces for.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetNodeStackTracesParams {
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.getNodeStackTraces。
///
///协议原文：
///
///<pre>Gets stack traces associated with a Node. As of now, only provides stack trace for Node creation.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetNodeStackTraces;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getNodeStackTraces/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetNodeStackTracesResponse {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getNodeStackTraces/returns/creation。
    ///
    ///协议原文：
    ///
    ///<pre>Creation stack trace, if available.</pre>
    ///
    ///
    #[serde(rename = "creation")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub creation: Option<crate::runtime::types::StackTrace>,
}
impl<'de> serde::Deserialize<'de> for GetNodeStackTracesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetNodeStackTracesResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getNodeStackTraces/returns/creation。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Creation stack trace, if available.</pre>
                    ///
                    ///
                    #[serde(rename = "creation")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub creation: Option<crate::runtime::types::StackTrace>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetNodeStackTracesResponse {
                    creation: fields.creation,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetNodeStackTraces {
    type Params = GetNodeStackTracesParams;
    type Response = GetNodeStackTracesResponse;
    const METHOD: &'static str = "DOM.getNodeStackTraces";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getNodesForSubtreeByStyle/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetNodesForSubtreeByStyleParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getNodesForSubtreeByStyle/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Node ID pointing to the root of a subtree.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getNodesForSubtreeByStyle/parameters/computedStyles。
    ///
    ///协议原文：
    ///
    ///<pre>The style to filter nodes by (includes nodes if any of properties matches).</pre>
    ///
    ///
    #[serde(rename = "computedStyles")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub computed_styles: Vec<crate::dom::types::CssComputedStyleProperty>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getNodesForSubtreeByStyle/parameters/pierce。
    ///
    ///协议原文：
    ///
    ///<pre>Whether or not iframes and shadow roots in the same target should be traversed when returning the
    ///results (default is false).</pre>
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
impl<'de> serde::Deserialize<'de> for GetNodesForSubtreeByStyleParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetNodesForSubtreeByStyleParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getNodesForSubtreeByStyle/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Node ID pointing to the root of a subtree.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getNodesForSubtreeByStyle/parameters/computedStyles。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The style to filter nodes by (includes nodes if any of properties matches).</pre>
                    ///
                    ///
                    #[serde(rename = "computedStyles")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub computed_styles: Vec<crate::dom::types::CssComputedStyleProperty>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getNodesForSubtreeByStyle/parameters/pierce。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether or not iframes and shadow roots in the same target should be traversed when returning the
                    ///results (default is false).</pre>
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
                Ok(GetNodesForSubtreeByStyleParams {
                    node_id: fields.node_id,
                    computed_styles: fields.computed_styles,
                    pierce: fields.pierce,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.getNodesForSubtreeByStyle。
///
///协议原文：
///
///<pre>Finds nodes with a given computed style in a subtree.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetNodesForSubtreeByStyle;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getNodesForSubtreeByStyle/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetNodesForSubtreeByStyleResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getNodesForSubtreeByStyle/returns/nodeIds。
    ///
    ///协议原文：
    ///
    ///<pre>Resulting nodes.</pre>
    ///
    ///
    #[serde(rename = "nodeIds")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_ids: Vec<crate::dom::types::NodeId>,
}
impl<'de> serde::Deserialize<'de> for GetNodesForSubtreeByStyleResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetNodesForSubtreeByStyleResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getNodesForSubtreeByStyle/returns/nodeIds。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Resulting nodes.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeIds")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_ids: Vec<crate::dom::types::NodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetNodesForSubtreeByStyleResponse {
                    node_ids: fields.node_ids,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetNodesForSubtreeByStyle {
    type Params = GetNodesForSubtreeByStyleParams;
    type Response = GetNodesForSubtreeByStyleResponse;
    const METHOD: &'static str = "DOM.getNodesForSubtreeByStyle";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getOuterHTML/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetOuterHtmlParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getOuterHTML/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the node.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub node_id: Option<crate::dom::types::NodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getOuterHTML/parameters/backendNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the backend node.</pre>
    ///
    ///
    #[serde(rename = "backendNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getOuterHTML/parameters/objectId。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript object id of the node wrapper.</pre>
    ///
    ///
    #[serde(rename = "objectId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub object_id: Option<crate::runtime::types::RemoteObjectId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getOuterHTML/parameters/includeShadowDOM。
    ///
    ///协议原文：
    ///
    ///<pre>Include all shadow roots. Equals to false if not specified.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "includeShadowDOM")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_shadow_dom: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for GetOuterHtmlParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetOuterHtmlParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getOuterHTML/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the node.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub node_id: Option<crate::dom::types::NodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getOuterHTML/parameters/backendNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the backend node.</pre>
                    ///
                    ///
                    #[serde(rename = "backendNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getOuterHTML/parameters/objectId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript object id of the node wrapper.</pre>
                    ///
                    ///
                    #[serde(rename = "objectId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub object_id: Option<crate::runtime::types::RemoteObjectId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.getOuterHTML/parameters/includeShadowDOM。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Include all shadow roots. Equals to false if not specified.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "includeShadowDOM")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub include_shadow_dom: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetOuterHtmlParams {
                    node_id: fields.node_id,
                    backend_node_id: fields.backend_node_id,
                    object_id: fields.object_id,
                    include_shadow_dom: fields.include_shadow_dom,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.getOuterHTML。
///
///协议原文：
///
///<pre>Returns node's HTML markup.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetOuterHtml;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getOuterHTML/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetOuterHtmlResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getOuterHTML/returns/outerHTML。
    ///
    ///协议原文：
    ///
    ///<pre>Outer HTML markup.</pre>
    ///
    ///
    #[serde(rename = "outerHTML")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub outer_html: String,
}
impl<'de> serde::Deserialize<'de> for GetOuterHtmlResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetOuterHtmlResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getOuterHTML/returns/outerHTML。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Outer HTML markup.</pre>
                    ///
                    ///
                    #[serde(rename = "outerHTML")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub outer_html: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetOuterHtmlResponse {
                    outer_html: fields.outer_html,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetOuterHtml {
    type Params = GetOuterHtmlParams;
    type Response = GetOuterHtmlResponse;
    const METHOD: &'static str = "DOM.getOuterHTML";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getQueryingDescendantsForContainer/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetQueryingDescendantsForContainerParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getQueryingDescendantsForContainer/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the container node to find querying descendants from.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for GetQueryingDescendantsForContainerParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetQueryingDescendantsForContainerParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getQueryingDescendantsForContainer/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the container node to find querying descendants from.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetQueryingDescendantsForContainerParams {
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.getQueryingDescendantsForContainer。
///
///协议原文：
///
///<pre>Returns the descendants of a container query container that have
///container queries against this container.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetQueryingDescendantsForContainer;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getQueryingDescendantsForContainer/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetQueryingDescendantsForContainerResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getQueryingDescendantsForContainer/returns/nodeIds。
    ///
    ///协议原文：
    ///
    ///<pre>Descendant nodes with container queries against the given container.</pre>
    ///
    ///
    #[serde(rename = "nodeIds")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_ids: Vec<crate::dom::types::NodeId>,
}
impl<'de> serde::Deserialize<'de> for GetQueryingDescendantsForContainerResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetQueryingDescendantsForContainerResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getQueryingDescendantsForContainer/returns/nodeIds。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Descendant nodes with container queries against the given container.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeIds")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_ids: Vec<crate::dom::types::NodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetQueryingDescendantsForContainerResponse {
                    node_ids: fields.node_ids,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetQueryingDescendantsForContainer {
    type Params = GetQueryingDescendantsForContainerParams;
    type Response = GetQueryingDescendantsForContainerResponse;
    const METHOD: &'static str = "DOM.getQueryingDescendantsForContainer";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getRelayoutBoundary/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetRelayoutBoundaryParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getRelayoutBoundary/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for GetRelayoutBoundaryParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetRelayoutBoundaryParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getRelayoutBoundary/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetRelayoutBoundaryParams {
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.getRelayoutBoundary。
///
///协议原文：
///
///<pre>Returns the id of the nearest ancestor that is a relayout boundary.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetRelayoutBoundary;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getRelayoutBoundary/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetRelayoutBoundaryResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getRelayoutBoundary/returns/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Relayout boundary node id for the given node.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for GetRelayoutBoundaryResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetRelayoutBoundaryResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getRelayoutBoundary/returns/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Relayout boundary node id for the given node.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetRelayoutBoundaryResponse {
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetRelayoutBoundary {
    type Params = GetRelayoutBoundaryParams;
    type Response = GetRelayoutBoundaryResponse;
    const METHOD: &'static str = "DOM.getRelayoutBoundary";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getSearchResults/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetSearchResultsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getSearchResults/parameters/searchId。
    ///
    ///协议原文：
    ///
    ///<pre>Unique search session identifier.</pre>
    ///
    ///
    #[serde(rename = "searchId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub search_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getSearchResults/parameters/fromIndex。
    ///
    ///协议原文：
    ///
    ///<pre>Start index of the search result to be returned.</pre>
    ///
    ///
    #[serde(rename = "fromIndex")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub from_index: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getSearchResults/parameters/toIndex。
    ///
    ///协议原文：
    ///
    ///<pre>End index of the search result to be returned.</pre>
    ///
    ///
    #[serde(rename = "toIndex")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub to_index: i64,
}
impl<'de> serde::Deserialize<'de> for GetSearchResultsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetSearchResultsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getSearchResults/parameters/searchId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Unique search session identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "searchId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub search_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getSearchResults/parameters/fromIndex。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Start index of the search result to be returned.</pre>
                    ///
                    ///
                    #[serde(rename = "fromIndex")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub from_index: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getSearchResults/parameters/toIndex。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>End index of the search result to be returned.</pre>
                    ///
                    ///
                    #[serde(rename = "toIndex")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub to_index: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetSearchResultsParams {
                    search_id: fields.search_id,
                    from_index: fields.from_index,
                    to_index: fields.to_index,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.getSearchResults。
///
///协议原文：
///
///<pre>Returns search results from given `fromIndex` to given `toIndex` from the search with the given
///identifier.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetSearchResults;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getSearchResults/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetSearchResultsResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getSearchResults/returns/nodeIds。
    ///
    ///协议原文：
    ///
    ///<pre>Ids of the search result nodes.</pre>
    ///
    ///
    #[serde(rename = "nodeIds")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_ids: Vec<crate::dom::types::NodeId>,
}
impl<'de> serde::Deserialize<'de> for GetSearchResultsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetSearchResultsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getSearchResults/returns/nodeIds。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Ids of the search result nodes.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeIds")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_ids: Vec<crate::dom::types::NodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetSearchResultsResponse {
                    node_ids: fields.node_ids,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetSearchResults {
    type Params = GetSearchResultsParams;
    type Response = GetSearchResultsResponse;
    const METHOD: &'static str = "DOM.getSearchResults";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getTopLayerElements/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetTopLayerElementsParams {}
impl<'de> serde::Deserialize<'de> for GetTopLayerElementsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetTopLayerElementsParams;
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
                Ok(GetTopLayerElementsParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.getTopLayerElements。
///
///协议原文：
///
///<pre>Returns NodeIds of current top layer elements.
///Top layer is rendered closest to the user within a viewport, therefore its elements always
///appear on top of all other content.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetTopLayerElements;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.getTopLayerElements/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetTopLayerElementsResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getTopLayerElements/returns/nodeIds。
    ///
    ///协议原文：
    ///
    ///<pre>NodeIds of top layer elements</pre>
    ///
    ///
    #[serde(rename = "nodeIds")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_ids: Vec<crate::dom::types::NodeId>,
}
impl<'de> serde::Deserialize<'de> for GetTopLayerElementsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetTopLayerElementsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.getTopLayerElements/returns/nodeIds。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>NodeIds of top layer elements</pre>
                    ///
                    ///
                    #[serde(rename = "nodeIds")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_ids: Vec<crate::dom::types::NodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetTopLayerElementsResponse {
                    node_ids: fields.node_ids,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetTopLayerElements {
    type Params = GetTopLayerElementsParams;
    type Response = GetTopLayerElementsResponse;
    const METHOD: &'static str = "DOM.getTopLayerElements";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.hideHighlight/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct HideHighlightParams {}
impl<'de> serde::Deserialize<'de> for HideHighlightParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HideHighlightParams;
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
                Ok(HideHighlightParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.hideHighlight。
///
///协议原文：
///
///<pre>Hides any highlight.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct HideHighlight;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.hideHighlight/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct HideHighlightResponse {}
impl<'de> serde::Deserialize<'de> for HideHighlightResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HideHighlightResponse;
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
                Ok(HideHighlightResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for HideHighlight {
    type Params = HideHighlightParams;
    type Response = HideHighlightResponse;
    const METHOD: &'static str = "DOM.hideHighlight";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.highlightNode/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct HighlightNodeParams {}
impl<'de> serde::Deserialize<'de> for HighlightNodeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HighlightNodeParams;
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
                Ok(HighlightNodeParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.highlightNode。
///
///协议原文：
///
///<pre>Highlights DOM node.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct HighlightNode;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.highlightNode/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct HighlightNodeResponse {}
impl<'de> serde::Deserialize<'de> for HighlightNodeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HighlightNodeResponse;
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
                Ok(HighlightNodeResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for HighlightNode {
    type Params = HighlightNodeParams;
    type Response = HighlightNodeResponse;
    const METHOD: &'static str = "DOM.highlightNode";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.highlightRect/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct HighlightRectParams {}
impl<'de> serde::Deserialize<'de> for HighlightRectParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HighlightRectParams;
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
                Ok(HighlightRectParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.highlightRect。
///
///协议原文：
///
///<pre>Highlights given rectangle.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct HighlightRect;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.highlightRect/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct HighlightRectResponse {}
impl<'de> serde::Deserialize<'de> for HighlightRectResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HighlightRectResponse;
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
                Ok(HighlightRectResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for HighlightRect {
    type Params = HighlightRectParams;
    type Response = HighlightRectResponse;
    const METHOD: &'static str = "DOM.highlightRect";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.markUndoableState/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct MarkUndoableStateParams {}
impl<'de> serde::Deserialize<'de> for MarkUndoableStateParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MarkUndoableStateParams;
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
                Ok(MarkUndoableStateParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.markUndoableState。
///
///协议原文：
///
///<pre>Marks last undoable state.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct MarkUndoableState;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.markUndoableState/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct MarkUndoableStateResponse {}
impl<'de> serde::Deserialize<'de> for MarkUndoableStateResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MarkUndoableStateResponse;
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
                Ok(MarkUndoableStateResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for MarkUndoableState {
    type Params = MarkUndoableStateParams;
    type Response = MarkUndoableStateResponse;
    const METHOD: &'static str = "DOM.markUndoableState";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.moveTo/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct MoveToParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.moveTo/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node to move.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.moveTo/parameters/targetNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the element to drop the moved node into.</pre>
    ///
    ///
    #[serde(rename = "targetNodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub target_node_id: crate::dom::types::NodeId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.moveTo/parameters/insertBeforeNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Drop node before this one (if absent, the moved node becomes the last child of
    ///`targetNodeId`).</pre>
    ///
    ///
    #[serde(rename = "insertBeforeNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub insert_before_node_id: Option<crate::dom::types::NodeId>,
}
impl<'de> serde::Deserialize<'de> for MoveToParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MoveToParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.moveTo/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node to move.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.moveTo/parameters/targetNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the element to drop the moved node into.</pre>
                    ///
                    ///
                    #[serde(rename = "targetNodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub target_node_id: crate::dom::types::NodeId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.moveTo/parameters/insertBeforeNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Drop node before this one (if absent, the moved node becomes the last child of
                    ///`targetNodeId`).</pre>
                    ///
                    ///
                    #[serde(rename = "insertBeforeNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub insert_before_node_id: Option<crate::dom::types::NodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(MoveToParams {
                    node_id: fields.node_id,
                    target_node_id: fields.target_node_id,
                    insert_before_node_id: fields.insert_before_node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.moveTo。
///
///协议原文：
///
///<pre>Moves node into the new container, places it before the given anchor.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct MoveTo;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.moveTo/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct MoveToResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.moveTo/returns/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>New id of the moved node.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for MoveToResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MoveToResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.moveTo/returns/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>New id of the moved node.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(MoveToResponse {
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for MoveTo {
    type Params = MoveToParams;
    type Response = MoveToResponse;
    const METHOD: &'static str = "DOM.moveTo";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.performSearch/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PerformSearchParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.performSearch/parameters/query。
    ///
    ///协议原文：
    ///
    ///<pre>Plain text or query selector or XPath search query.</pre>
    ///
    ///
    #[serde(rename = "query")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub query: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.performSearch/parameters/includeUserAgentShadowDOM。
    ///
    ///协议原文：
    ///
    ///<pre>True to search in user agent shadow DOM.</pre>
    ///
    ///
    #[serde(rename = "includeUserAgentShadowDOM")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_user_agent_shadow_dom: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for PerformSearchParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PerformSearchParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.performSearch/parameters/query。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Plain text or query selector or XPath search query.</pre>
                    ///
                    ///
                    #[serde(rename = "query")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub query: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.performSearch/parameters/includeUserAgentShadowDOM。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True to search in user agent shadow DOM.</pre>
                    ///
                    ///
                    #[serde(rename = "includeUserAgentShadowDOM")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub include_user_agent_shadow_dom: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PerformSearchParams {
                    query: fields.query,
                    include_user_agent_shadow_dom: fields.include_user_agent_shadow_dom,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.performSearch。
///
///协议原文：
///
///<pre>Searches for a given string in the DOM tree. Use `getSearchResults` to access search results or
///`cancelSearch` to end this search session.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct PerformSearch;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.performSearch/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PerformSearchResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.performSearch/returns/searchId。
    ///
    ///协议原文：
    ///
    ///<pre>Unique search session identifier.</pre>
    ///
    ///
    #[serde(rename = "searchId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub search_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.performSearch/returns/resultCount。
    ///
    ///协议原文：
    ///
    ///<pre>Number of search results.</pre>
    ///
    ///
    #[serde(rename = "resultCount")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub result_count: i64,
}
impl<'de> serde::Deserialize<'de> for PerformSearchResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PerformSearchResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.performSearch/returns/searchId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Unique search session identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "searchId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub search_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.performSearch/returns/resultCount。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Number of search results.</pre>
                    ///
                    ///
                    #[serde(rename = "resultCount")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub result_count: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PerformSearchResponse {
                    search_id: fields.search_id,
                    result_count: fields.result_count,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for PerformSearch {
    type Params = PerformSearchParams;
    type Response = PerformSearchResponse;
    const METHOD: &'static str = "DOM.performSearch";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.pushNodeByPathToFrontend/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PushNodeByPathToFrontendParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.pushNodeByPathToFrontend/parameters/path。
    ///
    ///协议原文：
    ///
    ///<pre>Path to node in the proprietary format.</pre>
    ///
    ///
    #[serde(rename = "path")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub path: String,
}
impl<'de> serde::Deserialize<'de> for PushNodeByPathToFrontendParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PushNodeByPathToFrontendParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.pushNodeByPathToFrontend/parameters/path。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Path to node in the proprietary format.</pre>
                    ///
                    ///
                    #[serde(rename = "path")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub path: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PushNodeByPathToFrontendParams { path: fields.path })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.pushNodeByPathToFrontend。
///
///协议原文：
///
///<pre>Requests that the node is sent to the caller given its path. // FIXME, use XPath</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct PushNodeByPathToFrontend;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.pushNodeByPathToFrontend/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PushNodeByPathToFrontendResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.pushNodeByPathToFrontend/returns/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node for given path.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for PushNodeByPathToFrontendResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PushNodeByPathToFrontendResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.pushNodeByPathToFrontend/returns/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node for given path.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PushNodeByPathToFrontendResponse {
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for PushNodeByPathToFrontend {
    type Params = PushNodeByPathToFrontendParams;
    type Response = PushNodeByPathToFrontendResponse;
    const METHOD: &'static str = "DOM.pushNodeByPathToFrontend";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.pushNodesByBackendIdsToFrontend/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PushNodesByBackendIdsToFrontendParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.pushNodesByBackendIdsToFrontend/parameters/backendNodeIds。
    ///
    ///协议原文：
    ///
    ///<pre>The array of backend node ids.</pre>
    ///
    ///
    #[serde(rename = "backendNodeIds")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub backend_node_ids: Vec<crate::dom::types::BackendNodeId>,
}
impl<'de> serde::Deserialize<'de> for PushNodesByBackendIdsToFrontendParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PushNodesByBackendIdsToFrontendParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.pushNodesByBackendIdsToFrontend/parameters/backendNodeIds。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The array of backend node ids.</pre>
                    ///
                    ///
                    #[serde(rename = "backendNodeIds")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub backend_node_ids: Vec<crate::dom::types::BackendNodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PushNodesByBackendIdsToFrontendParams {
                    backend_node_ids: fields.backend_node_ids,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.pushNodesByBackendIdsToFrontend。
///
///协议原文：
///
///<pre>Requests that a batch of nodes is sent to the caller given their backend node ids.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct PushNodesByBackendIdsToFrontend;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.pushNodesByBackendIdsToFrontend/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PushNodesByBackendIdsToFrontendResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.pushNodesByBackendIdsToFrontend/returns/nodeIds。
    ///
    ///协议原文：
    ///
    ///<pre>The array of ids of pushed nodes that correspond to the backend ids specified in
    ///backendNodeIds.</pre>
    ///
    ///
    #[serde(rename = "nodeIds")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_ids: Vec<crate::dom::types::NodeId>,
}
impl<'de> serde::Deserialize<'de> for PushNodesByBackendIdsToFrontendResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PushNodesByBackendIdsToFrontendResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.pushNodesByBackendIdsToFrontend/returns/nodeIds。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The array of ids of pushed nodes that correspond to the backend ids specified in
                    ///backendNodeIds.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeIds")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_ids: Vec<crate::dom::types::NodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PushNodesByBackendIdsToFrontendResponse {
                    node_ids: fields.node_ids,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for PushNodesByBackendIdsToFrontend {
    type Params = PushNodesByBackendIdsToFrontendParams;
    type Response = PushNodesByBackendIdsToFrontendResponse;
    const METHOD: &'static str = "DOM.pushNodesByBackendIdsToFrontend";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.querySelector/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct QuerySelectorParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.querySelector/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node to query upon.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.querySelector/parameters/selector。
    ///
    ///协议原文：
    ///
    ///<pre>Selector string.</pre>
    ///
    ///
    #[serde(rename = "selector")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub selector: String,
}
impl<'de> serde::Deserialize<'de> for QuerySelectorParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = QuerySelectorParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.querySelector/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node to query upon.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.querySelector/parameters/selector。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Selector string.</pre>
                    ///
                    ///
                    #[serde(rename = "selector")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub selector: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(QuerySelectorParams {
                    node_id: fields.node_id,
                    selector: fields.selector,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.querySelector。
///
///协议原文：
///
///<pre>Executes `querySelector` on a given node.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct QuerySelector;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.querySelector/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct QuerySelectorResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.querySelector/returns/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Query selector result.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for QuerySelectorResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = QuerySelectorResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.querySelector/returns/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Query selector result.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(QuerySelectorResponse {
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for QuerySelector {
    type Params = QuerySelectorParams;
    type Response = QuerySelectorResponse;
    const METHOD: &'static str = "DOM.querySelector";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.querySelectorAll/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct QuerySelectorAllParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.querySelectorAll/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node to query upon.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.querySelectorAll/parameters/selector。
    ///
    ///协议原文：
    ///
    ///<pre>Selector string.</pre>
    ///
    ///
    #[serde(rename = "selector")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub selector: String,
}
impl<'de> serde::Deserialize<'de> for QuerySelectorAllParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = QuerySelectorAllParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.querySelectorAll/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node to query upon.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.querySelectorAll/parameters/selector。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Selector string.</pre>
                    ///
                    ///
                    #[serde(rename = "selector")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub selector: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(QuerySelectorAllParams {
                    node_id: fields.node_id,
                    selector: fields.selector,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.querySelectorAll。
///
///协议原文：
///
///<pre>Executes `querySelectorAll` on a given node.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct QuerySelectorAll;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.querySelectorAll/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct QuerySelectorAllResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.querySelectorAll/returns/nodeIds。
    ///
    ///协议原文：
    ///
    ///<pre>Query selector result.</pre>
    ///
    ///
    #[serde(rename = "nodeIds")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_ids: Vec<crate::dom::types::NodeId>,
}
impl<'de> serde::Deserialize<'de> for QuerySelectorAllResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = QuerySelectorAllResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.querySelectorAll/returns/nodeIds。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Query selector result.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeIds")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_ids: Vec<crate::dom::types::NodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(QuerySelectorAllResponse {
                    node_ids: fields.node_ids,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for QuerySelectorAll {
    type Params = QuerySelectorAllParams;
    type Response = QuerySelectorAllResponse;
    const METHOD: &'static str = "DOM.querySelectorAll";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.redo/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct RedoParams {}
impl<'de> serde::Deserialize<'de> for RedoParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RedoParams;
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
                Ok(RedoParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.redo。
///
///协议原文：
///
///<pre>Re-does the last undone action.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct Redo;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.redo/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RedoResponse {}
impl<'de> serde::Deserialize<'de> for RedoResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RedoResponse;
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
                Ok(RedoResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Redo {
    type Params = RedoParams;
    type Response = RedoResponse;
    const METHOD: &'static str = "DOM.redo";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.removeAttribute/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveAttributeParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.removeAttribute/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the element to remove attribute from.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.removeAttribute/parameters/name。
    ///
    ///协议原文：
    ///
    ///<pre>Name of the attribute to remove.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
}
impl<'de> serde::Deserialize<'de> for RemoveAttributeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveAttributeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.removeAttribute/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the element to remove attribute from.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.removeAttribute/parameters/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Name of the attribute to remove.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RemoveAttributeParams {
                    node_id: fields.node_id,
                    name: fields.name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.removeAttribute。
///
///协议原文：
///
///<pre>Removes attribute with given name from an element with given id.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RemoveAttribute;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.removeAttribute/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveAttributeResponse {}
impl<'de> serde::Deserialize<'de> for RemoveAttributeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveAttributeResponse;
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
                Ok(RemoveAttributeResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RemoveAttribute {
    type Params = RemoveAttributeParams;
    type Response = RemoveAttributeResponse;
    const METHOD: &'static str = "DOM.removeAttribute";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.removeNode/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveNodeParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.removeNode/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node to remove.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for RemoveNodeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveNodeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.removeNode/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node to remove.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RemoveNodeParams {
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.removeNode。
///
///协议原文：
///
///<pre>Removes node with given id.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RemoveNode;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.removeNode/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveNodeResponse {}
impl<'de> serde::Deserialize<'de> for RemoveNodeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveNodeResponse;
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
                Ok(RemoveNodeResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RemoveNode {
    type Params = RemoveNodeParams;
    type Response = RemoveNodeResponse;
    const METHOD: &'static str = "DOM.removeNode";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.requestChildNodes/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequestChildNodesParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.requestChildNodes/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node to get children for.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.requestChildNodes/parameters/depth。
    ///
    ///协议原文：
    ///
    ///<pre>The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
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
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.requestChildNodes/parameters/pierce。
    ///
    ///协议原文：
    ///
    ///<pre>Whether or not iframes and shadow roots should be traversed when returning the sub-tree
    ///(default is false).</pre>
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
impl<'de> serde::Deserialize<'de> for RequestChildNodesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RequestChildNodesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.requestChildNodes/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node to get children for.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.requestChildNodes/parameters/depth。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the
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
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.requestChildNodes/parameters/pierce。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether or not iframes and shadow roots should be traversed when returning the sub-tree
                    ///(default is false).</pre>
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
                Ok(RequestChildNodesParams {
                    node_id: fields.node_id,
                    depth: fields.depth,
                    pierce: fields.pierce,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.requestChildNodes。
///
///协议原文：
///
///<pre>Requests that children of the node with given id are returned to the caller in form of
///`setChildNodes` events where not only immediate children are retrieved, but all children down to
///the specified depth.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RequestChildNodes;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.requestChildNodes/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequestChildNodesResponse {}
impl<'de> serde::Deserialize<'de> for RequestChildNodesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RequestChildNodesResponse;
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
                Ok(RequestChildNodesResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RequestChildNodes {
    type Params = RequestChildNodesParams;
    type Response = RequestChildNodesResponse;
    const METHOD: &'static str = "DOM.requestChildNodes";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.requestNode/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequestNodeParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.requestNode/parameters/objectId。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript object id to convert into node.</pre>
    ///
    ///
    #[serde(rename = "objectId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub object_id: crate::runtime::types::RemoteObjectId,
}
impl<'de> serde::Deserialize<'de> for RequestNodeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RequestNodeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.requestNode/parameters/objectId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript object id to convert into node.</pre>
                    ///
                    ///
                    #[serde(rename = "objectId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub object_id: crate::runtime::types::RemoteObjectId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RequestNodeParams {
                    object_id: fields.object_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.requestNode。
///
///协议原文：
///
///<pre>Requests that the node is sent to the caller given the JavaScript node object reference. All
///nodes that form the path from the node to the root are also sent to the client as a series of
///`setChildNodes` notifications.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RequestNode;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.requestNode/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequestNodeResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.requestNode/returns/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Node id for given object.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for RequestNodeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RequestNodeResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.requestNode/returns/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Node id for given object.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RequestNodeResponse {
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RequestNode {
    type Params = RequestNodeParams;
    type Response = RequestNodeResponse;
    const METHOD: &'static str = "DOM.requestNode";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.resolveNode/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ResolveNodeParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.resolveNode/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node to resolve.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub node_id: Option<crate::dom::types::NodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.resolveNode/parameters/backendNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Backend identifier of the node to resolve.</pre>
    ///
    ///
    #[serde(rename = "backendNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.resolveNode/parameters/objectGroup。
    ///
    ///协议原文：
    ///
    ///<pre>Symbolic group name that can be used to release multiple objects.</pre>
    ///
    ///
    #[serde(rename = "objectGroup")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub object_group: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.resolveNode/parameters/executionContextId。
    ///
    ///协议原文：
    ///
    ///<pre>Execution context in which to resolve the node.</pre>
    ///
    ///
    #[serde(rename = "executionContextId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_context_id: Option<crate::runtime::types::ExecutionContextId>,
}
impl<'de> serde::Deserialize<'de> for ResolveNodeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResolveNodeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.resolveNode/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node to resolve.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub node_id: Option<crate::dom::types::NodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.resolveNode/parameters/backendNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Backend identifier of the node to resolve.</pre>
                    ///
                    ///
                    #[serde(rename = "backendNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.resolveNode/parameters/objectGroup。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Symbolic group name that can be used to release multiple objects.</pre>
                    ///
                    ///
                    #[serde(rename = "objectGroup")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub object_group: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.resolveNode/parameters/executionContextId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Execution context in which to resolve the node.</pre>
                    ///
                    ///
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
                Ok(ResolveNodeParams {
                    node_id: fields.node_id,
                    backend_node_id: fields.backend_node_id,
                    object_group: fields.object_group,
                    execution_context_id: fields.execution_context_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.resolveNode。
///
///协议原文：
///
///<pre>Resolves the JavaScript node object for a given NodeId or BackendNodeId.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ResolveNode;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.resolveNode/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ResolveNodeResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.resolveNode/returns/object。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript object wrapper for given node.</pre>
    ///
    ///
    #[serde(rename = "object")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub object: crate::runtime::types::RemoteObject,
}
impl<'de> serde::Deserialize<'de> for ResolveNodeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResolveNodeResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.resolveNode/returns/object。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript object wrapper for given node.</pre>
                    ///
                    ///
                    #[serde(rename = "object")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub object: crate::runtime::types::RemoteObject,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ResolveNodeResponse {
                    object: fields.object,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ResolveNode {
    type Params = ResolveNodeParams;
    type Response = ResolveNodeResponse;
    const METHOD: &'static str = "DOM.resolveNode";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.scrollIntoViewIfNeeded/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ScrollIntoViewIfNeededParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.scrollIntoViewIfNeeded/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the node.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub node_id: Option<crate::dom::types::NodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.scrollIntoViewIfNeeded/parameters/backendNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the backend node.</pre>
    ///
    ///
    #[serde(rename = "backendNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.scrollIntoViewIfNeeded/parameters/objectId。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript object id of the node wrapper.</pre>
    ///
    ///
    #[serde(rename = "objectId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub object_id: Option<crate::runtime::types::RemoteObjectId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.scrollIntoViewIfNeeded/parameters/rect。
    ///
    ///协议原文：
    ///
    ///<pre>The rect to be scrolled into view, relative to the node's border box, in CSS pixels.
    ///When omitted, center of the node will be used, similar to Element.scrollIntoView.</pre>
    ///
    ///
    #[serde(rename = "rect")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub rect: Option<crate::dom::types::Rect>,
}
impl<'de> serde::Deserialize<'de> for ScrollIntoViewIfNeededParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScrollIntoViewIfNeededParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.scrollIntoViewIfNeeded/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the node.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub node_id: Option<crate::dom::types::NodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.scrollIntoViewIfNeeded/parameters/backendNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the backend node.</pre>
                    ///
                    ///
                    #[serde(rename = "backendNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.scrollIntoViewIfNeeded/parameters/objectId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript object id of the node wrapper.</pre>
                    ///
                    ///
                    #[serde(rename = "objectId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub object_id: Option<crate::runtime::types::RemoteObjectId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.scrollIntoViewIfNeeded/parameters/rect。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The rect to be scrolled into view, relative to the node's border box, in CSS pixels.
                    ///When omitted, center of the node will be used, similar to Element.scrollIntoView.</pre>
                    ///
                    ///
                    #[serde(rename = "rect")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub rect: Option<crate::dom::types::Rect>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ScrollIntoViewIfNeededParams {
                    node_id: fields.node_id,
                    backend_node_id: fields.backend_node_id,
                    object_id: fields.object_id,
                    rect: fields.rect,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.scrollIntoViewIfNeeded。
///
///协议原文：
///
///<pre>Scrolls the specified rect of the given node into view if not already visible.
///Note: exactly one between nodeId, backendNodeId and objectId should be passed
///to identify the node.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ScrollIntoViewIfNeeded;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.scrollIntoViewIfNeeded/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ScrollIntoViewIfNeededResponse {}
impl<'de> serde::Deserialize<'de> for ScrollIntoViewIfNeededResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScrollIntoViewIfNeededResponse;
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
                Ok(ScrollIntoViewIfNeededResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ScrollIntoViewIfNeeded {
    type Params = ScrollIntoViewIfNeededParams;
    type Response = ScrollIntoViewIfNeededResponse;
    const METHOD: &'static str = "DOM.scrollIntoViewIfNeeded";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.setAttributeValue/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetAttributeValueParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setAttributeValue/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the element to set attribute for.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setAttributeValue/parameters/name。
    ///
    ///协议原文：
    ///
    ///<pre>Attribute name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setAttributeValue/parameters/value。
    ///
    ///协议原文：
    ///
    ///<pre>Attribute value.</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: String,
}
impl<'de> serde::Deserialize<'de> for SetAttributeValueParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetAttributeValueParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setAttributeValue/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the element to set attribute for.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setAttributeValue/parameters/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Attribute name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setAttributeValue/parameters/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Attribute value.</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetAttributeValueParams {
                    node_id: fields.node_id,
                    name: fields.name,
                    value: fields.value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.setAttributeValue。
///
///协议原文：
///
///<pre>Sets attribute for an element with given id.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetAttributeValue;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.setAttributeValue/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetAttributeValueResponse {}
impl<'de> serde::Deserialize<'de> for SetAttributeValueResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetAttributeValueResponse;
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
                Ok(SetAttributeValueResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetAttributeValue {
    type Params = SetAttributeValueParams;
    type Response = SetAttributeValueResponse;
    const METHOD: &'static str = "DOM.setAttributeValue";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.setAttributesAsText/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetAttributesAsTextParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setAttributesAsText/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the element to set attributes for.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setAttributesAsText/parameters/text。
    ///
    ///协议原文：
    ///
    ///<pre>Text with a number of attributes. Will parse this text using HTML parser.</pre>
    ///
    ///
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.setAttributesAsText/parameters/name。
    ///
    ///协议原文：
    ///
    ///<pre>Attribute name to replace with new attributes derived from text in case text parsed
    ///successfully.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
}
impl<'de> serde::Deserialize<'de> for SetAttributesAsTextParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetAttributesAsTextParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setAttributesAsText/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the element to set attributes for.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setAttributesAsText/parameters/text。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Text with a number of attributes. Will parse this text using HTML parser.</pre>
                    ///
                    ///
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.setAttributesAsText/parameters/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Attribute name to replace with new attributes derived from text in case text parsed
                    ///successfully.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub name: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetAttributesAsTextParams {
                    node_id: fields.node_id,
                    text: fields.text,
                    name: fields.name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.setAttributesAsText。
///
///协议原文：
///
///<pre>Sets attributes on element with given id. This method is useful when user edits some existing
///attribute value and types in several attribute name/value pairs.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetAttributesAsText;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.setAttributesAsText/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetAttributesAsTextResponse {}
impl<'de> serde::Deserialize<'de> for SetAttributesAsTextResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetAttributesAsTextResponse;
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
                Ok(SetAttributesAsTextResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetAttributesAsText {
    type Params = SetAttributesAsTextParams;
    type Response = SetAttributesAsTextResponse;
    const METHOD: &'static str = "DOM.setAttributesAsText";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.setFileInputFiles/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetFileInputFilesParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setFileInputFiles/parameters/files。
    ///
    ///协议原文：
    ///
    ///<pre>Array of file paths to set.</pre>
    ///
    ///
    #[serde(rename = "files")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub files: Vec<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.setFileInputFiles/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the node.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub node_id: Option<crate::dom::types::NodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.setFileInputFiles/parameters/backendNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the backend node.</pre>
    ///
    ///
    #[serde(rename = "backendNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.setFileInputFiles/parameters/objectId。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript object id of the node wrapper.</pre>
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
impl<'de> serde::Deserialize<'de> for SetFileInputFilesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetFileInputFilesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setFileInputFiles/parameters/files。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Array of file paths to set.</pre>
                    ///
                    ///
                    #[serde(rename = "files")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub files: Vec<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.setFileInputFiles/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the node.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub node_id: Option<crate::dom::types::NodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.setFileInputFiles/parameters/backendNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the backend node.</pre>
                    ///
                    ///
                    #[serde(rename = "backendNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.command.setFileInputFiles/parameters/objectId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript object id of the node wrapper.</pre>
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
                Ok(SetFileInputFilesParams {
                    files: fields.files,
                    node_id: fields.node_id,
                    backend_node_id: fields.backend_node_id,
                    object_id: fields.object_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.setFileInputFiles。
///
///协议原文：
///
///<pre>Sets files for the given file input element.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetFileInputFiles;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.setFileInputFiles/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetFileInputFilesResponse {}
impl<'de> serde::Deserialize<'de> for SetFileInputFilesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetFileInputFilesResponse;
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
                Ok(SetFileInputFilesResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetFileInputFiles {
    type Params = SetFileInputFilesParams;
    type Response = SetFileInputFilesResponse;
    const METHOD: &'static str = "DOM.setFileInputFiles";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.setInspectedNode/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetInspectedNodeParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setInspectedNode/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>DOM node id to be accessible by means of $x command line API.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for SetInspectedNodeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetInspectedNodeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setInspectedNode/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>DOM node id to be accessible by means of $x command line API.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetInspectedNodeParams {
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.setInspectedNode。
///
///协议原文：
///
///<pre>Enables console to refer to the node with given id via $x (see Command Line API for more details
///$x functions).</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetInspectedNode;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.setInspectedNode/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetInspectedNodeResponse {}
impl<'de> serde::Deserialize<'de> for SetInspectedNodeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetInspectedNodeResponse;
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
                Ok(SetInspectedNodeResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetInspectedNode {
    type Params = SetInspectedNodeParams;
    type Response = SetInspectedNodeResponse;
    const METHOD: &'static str = "DOM.setInspectedNode";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.setNodeName/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetNodeNameParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setNodeName/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node to set name for.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setNodeName/parameters/name。
    ///
    ///协议原文：
    ///
    ///<pre>New node's name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
}
impl<'de> serde::Deserialize<'de> for SetNodeNameParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetNodeNameParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setNodeName/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node to set name for.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setNodeName/parameters/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>New node's name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetNodeNameParams {
                    node_id: fields.node_id,
                    name: fields.name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.setNodeName。
///
///协议原文：
///
///<pre>Sets node name for a node with given id.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetNodeName;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.setNodeName/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetNodeNameResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setNodeName/returns/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>New node's id.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for SetNodeNameResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetNodeNameResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setNodeName/returns/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>New node's id.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetNodeNameResponse {
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetNodeName {
    type Params = SetNodeNameParams;
    type Response = SetNodeNameResponse;
    const METHOD: &'static str = "DOM.setNodeName";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.setNodeStackTracesEnabled/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetNodeStackTracesEnabledParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setNodeStackTracesEnabled/parameters/enable。
    ///
    ///协议原文：
    ///
    ///<pre>Enable or disable.</pre>
    ///
    ///
    #[serde(rename = "enable")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub enable: bool,
}
impl<'de> serde::Deserialize<'de> for SetNodeStackTracesEnabledParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetNodeStackTracesEnabledParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setNodeStackTracesEnabled/parameters/enable。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Enable or disable.</pre>
                    ///
                    ///
                    #[serde(rename = "enable")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub enable: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetNodeStackTracesEnabledParams {
                    enable: fields.enable,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.setNodeStackTracesEnabled。
///
///协议原文：
///
///<pre>Sets if stack traces should be captured for Nodes. See `Node.getNodeStackTraces`. Default is disabled.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetNodeStackTracesEnabled;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.setNodeStackTracesEnabled/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetNodeStackTracesEnabledResponse {}
impl<'de> serde::Deserialize<'de> for SetNodeStackTracesEnabledResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetNodeStackTracesEnabledResponse;
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
                Ok(SetNodeStackTracesEnabledResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetNodeStackTracesEnabled {
    type Params = SetNodeStackTracesEnabledParams;
    type Response = SetNodeStackTracesEnabledResponse;
    const METHOD: &'static str = "DOM.setNodeStackTracesEnabled";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.setNodeValue/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetNodeValueParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setNodeValue/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node to set value for.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setNodeValue/parameters/value。
    ///
    ///协议原文：
    ///
    ///<pre>New node's value.</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: String,
}
impl<'de> serde::Deserialize<'de> for SetNodeValueParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetNodeValueParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setNodeValue/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node to set value for.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setNodeValue/parameters/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>New node's value.</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetNodeValueParams {
                    node_id: fields.node_id,
                    value: fields.value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.setNodeValue。
///
///协议原文：
///
///<pre>Sets node value for a node with given id.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetNodeValue;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.setNodeValue/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetNodeValueResponse {}
impl<'de> serde::Deserialize<'de> for SetNodeValueResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetNodeValueResponse;
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
                Ok(SetNodeValueResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetNodeValue {
    type Params = SetNodeValueParams;
    type Response = SetNodeValueResponse;
    const METHOD: &'static str = "DOM.setNodeValue";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.setOuterHTML/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetOuterHtmlParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setOuterHTML/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node to set markup for.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setOuterHTML/parameters/outerHTML。
    ///
    ///协议原文：
    ///
    ///<pre>Outer HTML markup to set.</pre>
    ///
    ///
    #[serde(rename = "outerHTML")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub outer_html: String,
}
impl<'de> serde::Deserialize<'de> for SetOuterHtmlParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetOuterHtmlParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setOuterHTML/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node to set markup for.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.command.setOuterHTML/parameters/outerHTML。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Outer HTML markup to set.</pre>
                    ///
                    ///
                    #[serde(rename = "outerHTML")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub outer_html: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetOuterHtmlParams {
                    node_id: fields.node_id,
                    outer_html: fields.outer_html,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.setOuterHTML。
///
///协议原文：
///
///<pre>Sets node HTML markup, returns new node id.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetOuterHtml;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.setOuterHTML/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetOuterHtmlResponse {}
impl<'de> serde::Deserialize<'de> for SetOuterHtmlResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetOuterHtmlResponse;
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
                Ok(SetOuterHtmlResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetOuterHtml {
    type Params = SetOuterHtmlParams;
    type Response = SetOuterHtmlResponse;
    const METHOD: &'static str = "DOM.setOuterHTML";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.undo/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct UndoParams {}
impl<'de> serde::Deserialize<'de> for UndoParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UndoParams;
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
                Ok(UndoParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.command.undo。
///
///协议原文：
///
///<pre>Undoes the last performed action.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct Undo;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.command.undo/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct UndoResponse {}
impl<'de> serde::Deserialize<'de> for UndoResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UndoResponse;
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
                Ok(UndoResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Undo {
    type Params = UndoParams;
    type Response = UndoResponse;
    const METHOD: &'static str = "DOM.undo";
}
