///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.type.BackendNode。
///
///协议原文：
///
///<pre>Backend node with a friendly name.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct BackendNode {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.BackendNode/properties/nodeType。
    ///
    ///协议原文：
    ///
    ///<pre>`Node`'s nodeType.</pre>
    ///
    ///
    #[serde(rename = "nodeType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_type: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.BackendNode/properties/nodeName。
    ///
    ///协议原文：
    ///
    ///<pre>`Node`'s nodeName.</pre>
    ///
    ///
    #[serde(rename = "nodeName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.BackendNode/properties/backendNodeId。
    #[serde(rename = "backendNodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub backend_node_id: crate::dom::types::BackendNodeId,
}
impl<'de> serde::Deserialize<'de> for BackendNode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BackendNode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.BackendNode/properties/nodeType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`Node`'s nodeType.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_type: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.BackendNode/properties/nodeName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`Node`'s nodeName.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.BackendNode/properties/backendNodeId。
                    #[serde(rename = "backendNodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub backend_node_id: crate::dom::types::BackendNodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(BackendNode {
                    node_type: fields.node_type,
                    node_name: fields.node_name,
                    backend_node_id: fields.backend_node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：DOM.type.BackendNodeId。
///
///协议原文：
///
///<pre>Unique DOM node identifier used to reference a node that may not have been pushed to the
///front-end.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct BackendNodeId(i64);
impl BackendNodeId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::dom::types::BackendNodeId = serde_json::from_str("0").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::dom::types::BackendNodeId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("0").unwrap());
    ///```
    pub fn new(value: i64) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::dom::types::BackendNodeId = serde_json::from_str("0").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::dom::types::BackendNodeId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("0").unwrap());
    ///```
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.type.BoxModel。
///
///协议原文：
///
///<pre>Box model.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct BoxModel {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.BoxModel/properties/content。
    ///
    ///协议原文：
    ///
    ///<pre>Content box</pre>
    ///
    ///
    #[serde(rename = "content")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub content: crate::dom::types::Quad,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.BoxModel/properties/padding。
    ///
    ///协议原文：
    ///
    ///<pre>Padding box</pre>
    ///
    ///
    #[serde(rename = "padding")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub padding: crate::dom::types::Quad,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.BoxModel/properties/border。
    ///
    ///协议原文：
    ///
    ///<pre>Border box</pre>
    ///
    ///
    #[serde(rename = "border")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub border: crate::dom::types::Quad,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.BoxModel/properties/margin。
    ///
    ///协议原文：
    ///
    ///<pre>Margin box</pre>
    ///
    ///
    #[serde(rename = "margin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub margin: crate::dom::types::Quad,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.BoxModel/properties/width。
    ///
    ///协议原文：
    ///
    ///<pre>Node width</pre>
    ///
    ///
    #[serde(rename = "width")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub width: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.BoxModel/properties/height。
    ///
    ///协议原文：
    ///
    ///<pre>Node height</pre>
    ///
    ///
    #[serde(rename = "height")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub height: i64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.BoxModel/properties/shapeOutside。
    ///
    ///协议原文：
    ///
    ///<pre>Shape outside coordinates</pre>
    ///
    ///
    #[serde(rename = "shapeOutside")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub shape_outside: Option<crate::dom::types::ShapeOutsideInfo>,
}
impl<'de> serde::Deserialize<'de> for BoxModel {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BoxModel;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.BoxModel/properties/content。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Content box</pre>
                    ///
                    ///
                    #[serde(rename = "content")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub content: crate::dom::types::Quad,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.BoxModel/properties/padding。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Padding box</pre>
                    ///
                    ///
                    #[serde(rename = "padding")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub padding: crate::dom::types::Quad,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.BoxModel/properties/border。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Border box</pre>
                    ///
                    ///
                    #[serde(rename = "border")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub border: crate::dom::types::Quad,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.BoxModel/properties/margin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Margin box</pre>
                    ///
                    ///
                    #[serde(rename = "margin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub margin: crate::dom::types::Quad,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.BoxModel/properties/width。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Node width</pre>
                    ///
                    ///
                    #[serde(rename = "width")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub width: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.BoxModel/properties/height。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Node height</pre>
                    ///
                    ///
                    #[serde(rename = "height")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub height: i64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.BoxModel/properties/shapeOutside。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Shape outside coordinates</pre>
                    ///
                    ///
                    #[serde(rename = "shapeOutside")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub shape_outside: Option<crate::dom::types::ShapeOutsideInfo>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(BoxModel {
                    content: fields.content,
                    padding: fields.padding,
                    border: fields.border,
                    margin: fields.margin,
                    width: fields.width,
                    height: fields.height,
                    shape_outside: fields.shape_outside,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.type.CSSComputedStyleProperty。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CssComputedStyleProperty {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.CSSComputedStyleProperty/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Computed style property name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.CSSComputedStyleProperty/properties/value。
    ///
    ///协议原文：
    ///
    ///<pre>Computed style property value.</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: String,
}
impl<'de> serde::Deserialize<'de> for CssComputedStyleProperty {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssComputedStyleProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.CSSComputedStyleProperty/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Computed style property name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.CSSComputedStyleProperty/properties/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Computed style property value.</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CssComputedStyleProperty {
                    name: fields.name,
                    value: fields.value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：DOM.type.CompatibilityMode。
///
///协议原文：
///
///<pre>Document compatibility mode.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CompatibilityMode {
    ///协议值 "QuirksMode"。
    QuirksMode,
    ///协议值 "LimitedQuirksMode"。
    LimitedQuirksMode,
    ///协议值 "NoQuirksMode"。
    NoQuirksMode,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl CompatibilityMode {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::dom::types::CompatibilityMode::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::QuirksMode => "QuirksMode",
            Self::LimitedQuirksMode => "LimitedQuirksMode",
            Self::NoQuirksMode => "NoQuirksMode",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for CompatibilityMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for CompatibilityMode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "QuirksMode" => CompatibilityMode::QuirksMode,
            "LimitedQuirksMode" => CompatibilityMode::LimitedQuirksMode,
            "NoQuirksMode" => CompatibilityMode::NoQuirksMode,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for CompatibilityMode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CompatibilityMode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CompatibilityMode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "QuirksMode" => CompatibilityMode::QuirksMode,
                    "LimitedQuirksMode" => CompatibilityMode::LimitedQuirksMode,
                    "NoQuirksMode" => CompatibilityMode::NoQuirksMode,
                    _ => CompatibilityMode::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(CompatibilityMode::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.type.DetachedElementInfo。
///
///协议原文：
///
///<pre>A structure to hold the top-level node of a detached tree and an array of its retained descendants.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct DetachedElementInfo {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.DetachedElementInfo/properties/treeNode。
    #[serde(rename = "treeNode")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub tree_node: crate::dom::types::Node,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.DetachedElementInfo/properties/retainedNodeIds。
    #[serde(rename = "retainedNodeIds")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub retained_node_ids: Vec<crate::dom::types::NodeId>,
}
impl<'de> serde::Deserialize<'de> for DetachedElementInfo {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DetachedElementInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.DetachedElementInfo/properties/treeNode。
                    #[serde(rename = "treeNode")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub tree_node: crate::dom::types::Node,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.DetachedElementInfo/properties/retainedNodeIds。
                    #[serde(rename = "retainedNodeIds")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub retained_node_ids: Vec<crate::dom::types::NodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DetachedElementInfo {
                    tree_node: fields.tree_node,
                    retained_node_ids: fields.retained_node_ids,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：DOM.type.LogicalAxes。
///
///协议原文：
///
///<pre>ContainerSelector logical axes</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum LogicalAxes {
    ///协议值 "Inline"。
    Inline,
    ///协议值 "Block"。
    Block,
    ///协议值 "Both"。
    Both,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl LogicalAxes {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::dom::types::LogicalAxes::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Inline => "Inline",
            Self::Block => "Block",
            Self::Both => "Both",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for LogicalAxes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for LogicalAxes {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Inline" => LogicalAxes::Inline,
            "Block" => LogicalAxes::Block,
            "Both" => LogicalAxes::Both,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for LogicalAxes {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LogicalAxes {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LogicalAxes;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Inline" => LogicalAxes::Inline,
                    "Block" => LogicalAxes::Block,
                    "Both" => LogicalAxes::Both,
                    _ => LogicalAxes::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(LogicalAxes::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.type.Node。
///
///协议原文：
///
///<pre>DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
///DOMNode is a base node mirror type.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct Node {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.Node/properties/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Node identifier that is passed into the rest of the DOM messages as the `nodeId`. Backend
    ///will only push node with given `id` once. It is aware of all requested nodes and will only
    ///fire DOM events for nodes known to the client.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/parentId。
    ///
    ///协议原文：
    ///
    ///<pre>The id of the parent node if any.</pre>
    ///
    ///
    #[serde(rename = "parentId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_id: Option<crate::dom::types::NodeId>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.Node/properties/backendNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>The BackendNodeId for this node.</pre>
    ///
    ///
    #[serde(rename = "backendNodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub backend_node_id: crate::dom::types::BackendNodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.Node/properties/nodeType。
    ///
    ///协议原文：
    ///
    ///<pre>`Node`'s nodeType.</pre>
    ///
    ///
    #[serde(rename = "nodeType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_type: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.Node/properties/nodeName。
    ///
    ///协议原文：
    ///
    ///<pre>`Node`'s nodeName.</pre>
    ///
    ///
    #[serde(rename = "nodeName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.Node/properties/localName。
    ///
    ///协议原文：
    ///
    ///<pre>`Node`'s localName.</pre>
    ///
    ///
    #[serde(rename = "localName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub local_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.Node/properties/nodeValue。
    ///
    ///协议原文：
    ///
    ///<pre>`Node`'s nodeValue.</pre>
    ///
    ///
    #[serde(rename = "nodeValue")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_value: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/childNodeCount。
    ///
    ///协议原文：
    ///
    ///<pre>Child count for `Container` nodes.</pre>
    ///
    ///
    #[serde(rename = "childNodeCount")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub child_node_count: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/children。
    ///
    ///协议原文：
    ///
    ///<pre>Child nodes of this node when requested with children.</pre>
    ///
    ///
    #[serde(rename = "children")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub children: Option<Vec<crate::dom::types::Node>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/attributes。
    ///
    ///协议原文：
    ///
    ///<pre>Attributes of the `Element` node in the form of flat array `[name1, value1, name2, value2]`.</pre>
    ///
    ///
    #[serde(rename = "attributes")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub attributes: Option<Vec<String>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/documentURL。
    ///
    ///协议原文：
    ///
    ///<pre>Document URL that `Document` or `FrameOwner` node points to.</pre>
    ///
    ///
    #[serde(rename = "documentURL")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub document_url: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/baseURL。
    ///
    ///协议原文：
    ///
    ///<pre>Base URL that `Document` or `FrameOwner` node uses for URL completion.</pre>
    ///
    ///
    #[serde(rename = "baseURL")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub base_url: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/publicId。
    ///
    ///协议原文：
    ///
    ///<pre>`DocumentType`'s publicId.</pre>
    ///
    ///
    #[serde(rename = "publicId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub public_id: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/systemId。
    ///
    ///协议原文：
    ///
    ///<pre>`DocumentType`'s systemId.</pre>
    ///
    ///
    #[serde(rename = "systemId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub system_id: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/internalSubset。
    ///
    ///协议原文：
    ///
    ///<pre>`DocumentType`'s internalSubset.</pre>
    ///
    ///
    #[serde(rename = "internalSubset")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_subset: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/xmlVersion。
    ///
    ///协议原文：
    ///
    ///<pre>`Document`'s XML version in case of XML documents.</pre>
    ///
    ///
    #[serde(rename = "xmlVersion")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub xml_version: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>`Attr`'s name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/value。
    ///
    ///协议原文：
    ///
    ///<pre>`Attr`'s value.</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/pseudoType。
    ///
    ///协议原文：
    ///
    ///<pre>Pseudo element type for this node.</pre>
    ///
    ///
    #[serde(rename = "pseudoType")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub pseudo_type: Option<crate::dom::types::PseudoType>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/pseudoIdentifier。
    ///
    ///协议原文：
    ///
    ///<pre>Pseudo element identifier for this node. Only present if there is a
    ///valid pseudoType.</pre>
    ///
    ///
    #[serde(rename = "pseudoIdentifier")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub pseudo_identifier: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/shadowRootType。
    ///
    ///协议原文：
    ///
    ///<pre>Shadow root type.</pre>
    ///
    ///
    #[serde(rename = "shadowRootType")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub shadow_root_type: Option<crate::dom::types::ShadowRootType>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Frame ID for frame owner elements.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub frame_id: Option<crate::page::types::FrameId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/contentDocument。
    ///
    ///协议原文：
    ///
    ///<pre>Content document for frame owner elements.</pre>
    ///
    ///
    #[serde(rename = "contentDocument")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub content_document: Option<Box<crate::dom::types::Node>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/shadowRoots。
    ///
    ///协议原文：
    ///
    ///<pre>Shadow root list for given element host.</pre>
    ///
    ///
    #[serde(rename = "shadowRoots")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub shadow_roots: Option<Vec<crate::dom::types::Node>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/templateContent。
    ///
    ///协议原文：
    ///
    ///<pre>Content document fragment for template elements.</pre>
    ///
    ///
    #[serde(rename = "templateContent")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub template_content: Option<Box<crate::dom::types::Node>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/pseudoElements。
    ///
    ///协议原文：
    ///
    ///<pre>Pseudo elements associated with this node.</pre>
    ///
    ///
    #[serde(rename = "pseudoElements")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub pseudo_elements: Option<Vec<crate::dom::types::Node>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/importedDocument。
    ///
    ///协议原文：
    ///
    ///<pre>Deprecated, as the HTML Imports API has been removed (crbug.com/937746).
    ///This property used to return the imported document for the HTMLImport links.
    ///The property is always undefined now.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "importedDocument")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub imported_document: Option<Box<crate::dom::types::Node>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/distributedNodes。
    ///
    ///协议原文：
    ///
    ///<pre>Distributed nodes for given insertion point.</pre>
    ///
    ///
    #[serde(rename = "distributedNodes")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub distributed_nodes: Option<Vec<crate::dom::types::BackendNode>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/isSVG。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the node is SVG.</pre>
    ///
    ///
    #[serde(rename = "isSVG")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_svg: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/compatibilityMode。
    #[serde(rename = "compatibilityMode")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub compatibility_mode: Option<crate::dom::types::CompatibilityMode>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/assignedSlot。
    #[serde(rename = "assignedSlot")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub assigned_slot: Option<crate::dom::types::BackendNode>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/isScrollable。此项为 experimental，端点可能不支持。
    #[serde(rename = "isScrollable")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_scrollable: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/affectedByStartingStyles。此项为 experimental，端点可能不支持。
    #[serde(rename = "affectedByStartingStyles")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub affected_by_starting_styles: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/adoptedStyleSheets。此项为 experimental，端点可能不支持。
    #[serde(rename = "adoptedStyleSheets")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub adopted_style_sheets: Option<Vec<crate::dom::types::StyleSheetId>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/adProvenance。此项为 experimental，端点可能不支持。
    #[serde(rename = "adProvenance")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub ad_provenance: Option<crate::network::types::AdProvenance>,
}
impl<'de> serde::Deserialize<'de> for Node {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Node;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.Node/properties/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Node identifier that is passed into the rest of the DOM messages as the `nodeId`. Backend
                    ///will only push node with given `id` once. It is aware of all requested nodes and will only
                    ///fire DOM events for nodes known to the client.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/parentId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The id of the parent node if any.</pre>
                    ///
                    ///
                    #[serde(rename = "parentId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub parent_id: Option<crate::dom::types::NodeId>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.Node/properties/backendNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The BackendNodeId for this node.</pre>
                    ///
                    ///
                    #[serde(rename = "backendNodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub backend_node_id: crate::dom::types::BackendNodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.Node/properties/nodeType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`Node`'s nodeType.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_type: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.Node/properties/nodeName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`Node`'s nodeName.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.Node/properties/localName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`Node`'s localName.</pre>
                    ///
                    ///
                    #[serde(rename = "localName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub local_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.Node/properties/nodeValue。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`Node`'s nodeValue.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeValue")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_value: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/childNodeCount。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Child count for `Container` nodes.</pre>
                    ///
                    ///
                    #[serde(rename = "childNodeCount")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub child_node_count: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/children。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Child nodes of this node when requested with children.</pre>
                    ///
                    ///
                    #[serde(rename = "children")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub children: Option<Vec<crate::dom::types::Node>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/attributes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Attributes of the `Element` node in the form of flat array `[name1, value1, name2, value2]`.</pre>
                    ///
                    ///
                    #[serde(rename = "attributes")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub attributes: Option<Vec<String>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/documentURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Document URL that `Document` or `FrameOwner` node points to.</pre>
                    ///
                    ///
                    #[serde(rename = "documentURL")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub document_url: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/baseURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Base URL that `Document` or `FrameOwner` node uses for URL completion.</pre>
                    ///
                    ///
                    #[serde(rename = "baseURL")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub base_url: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/publicId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`DocumentType`'s publicId.</pre>
                    ///
                    ///
                    #[serde(rename = "publicId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub public_id: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/systemId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`DocumentType`'s systemId.</pre>
                    ///
                    ///
                    #[serde(rename = "systemId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub system_id: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/internalSubset。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`DocumentType`'s internalSubset.</pre>
                    ///
                    ///
                    #[serde(rename = "internalSubset")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub internal_subset: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/xmlVersion。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`Document`'s XML version in case of XML documents.</pre>
                    ///
                    ///
                    #[serde(rename = "xmlVersion")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub xml_version: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`Attr`'s name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub name: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`Attr`'s value.</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub value: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/pseudoType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Pseudo element type for this node.</pre>
                    ///
                    ///
                    #[serde(rename = "pseudoType")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub pseudo_type: Option<crate::dom::types::PseudoType>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/pseudoIdentifier。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Pseudo element identifier for this node. Only present if there is a
                    ///valid pseudoType.</pre>
                    ///
                    ///
                    #[serde(rename = "pseudoIdentifier")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub pseudo_identifier: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/shadowRootType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Shadow root type.</pre>
                    ///
                    ///
                    #[serde(rename = "shadowRootType")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub shadow_root_type: Option<crate::dom::types::ShadowRootType>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame ID for frame owner elements.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub frame_id: Option<crate::page::types::FrameId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/contentDocument。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Content document for frame owner elements.</pre>
                    ///
                    ///
                    #[serde(rename = "contentDocument")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub content_document: Option<Box<crate::dom::types::Node>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/shadowRoots。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Shadow root list for given element host.</pre>
                    ///
                    ///
                    #[serde(rename = "shadowRoots")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub shadow_roots: Option<Vec<crate::dom::types::Node>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/templateContent。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Content document fragment for template elements.</pre>
                    ///
                    ///
                    #[serde(rename = "templateContent")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub template_content: Option<Box<crate::dom::types::Node>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/pseudoElements。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Pseudo elements associated with this node.</pre>
                    ///
                    ///
                    #[serde(rename = "pseudoElements")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub pseudo_elements: Option<Vec<crate::dom::types::Node>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/importedDocument。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Deprecated, as the HTML Imports API has been removed (crbug.com/937746).
                    ///This property used to return the imported document for the HTMLImport links.
                    ///The property is always undefined now.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "importedDocument")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub imported_document: Option<Box<crate::dom::types::Node>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/distributedNodes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Distributed nodes for given insertion point.</pre>
                    ///
                    ///
                    #[serde(rename = "distributedNodes")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub distributed_nodes: Option<Vec<crate::dom::types::BackendNode>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/isSVG。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the node is SVG.</pre>
                    ///
                    ///
                    #[serde(rename = "isSVG")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub is_svg: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/compatibilityMode。
                    #[serde(rename = "compatibilityMode")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub compatibility_mode: Option<crate::dom::types::CompatibilityMode>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/assignedSlot。
                    #[serde(rename = "assignedSlot")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub assigned_slot: Option<crate::dom::types::BackendNode>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/isScrollable。此项为 experimental，端点可能不支持。
                    #[serde(rename = "isScrollable")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub is_scrollable: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/affectedByStartingStyles。此项为 experimental，端点可能不支持。
                    #[serde(rename = "affectedByStartingStyles")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub affected_by_starting_styles: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/adoptedStyleSheets。此项为 experimental，端点可能不支持。
                    #[serde(rename = "adoptedStyleSheets")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub adopted_style_sheets: Option<Vec<crate::dom::types::StyleSheetId>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.Node/properties/adProvenance。此项为 experimental，端点可能不支持。
                    #[serde(rename = "adProvenance")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub ad_provenance: Option<crate::network::types::AdProvenance>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Node {
                    node_id: fields.node_id,
                    parent_id: fields.parent_id,
                    backend_node_id: fields.backend_node_id,
                    node_type: fields.node_type,
                    node_name: fields.node_name,
                    local_name: fields.local_name,
                    node_value: fields.node_value,
                    child_node_count: fields.child_node_count,
                    children: fields.children,
                    attributes: fields.attributes,
                    document_url: fields.document_url,
                    base_url: fields.base_url,
                    public_id: fields.public_id,
                    system_id: fields.system_id,
                    internal_subset: fields.internal_subset,
                    xml_version: fields.xml_version,
                    name: fields.name,
                    value: fields.value,
                    pseudo_type: fields.pseudo_type,
                    pseudo_identifier: fields.pseudo_identifier,
                    shadow_root_type: fields.shadow_root_type,
                    frame_id: fields.frame_id,
                    content_document: fields.content_document,
                    shadow_roots: fields.shadow_roots,
                    template_content: fields.template_content,
                    pseudo_elements: fields.pseudo_elements,
                    imported_document: fields.imported_document,
                    distributed_nodes: fields.distributed_nodes,
                    is_svg: fields.is_svg,
                    compatibility_mode: fields.compatibility_mode,
                    assigned_slot: fields.assigned_slot,
                    is_scrollable: fields.is_scrollable,
                    affected_by_starting_styles: fields.affected_by_starting_styles,
                    adopted_style_sheets: fields.adopted_style_sheets,
                    ad_provenance: fields.ad_provenance,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：DOM.type.NodeId。
///
///协议原文：
///
///<pre>Unique DOM node identifier.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct NodeId(i64);
impl NodeId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::dom::types::NodeId = serde_json::from_str("0").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::dom::types::NodeId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("0").unwrap());
    ///```
    pub fn new(value: i64) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::dom::types::NodeId = serde_json::from_str("0").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::dom::types::NodeId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("0").unwrap());
    ///```
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：DOM.type.PhysicalAxes。
///
///协议原文：
///
///<pre>ContainerSelector physical axes</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PhysicalAxes {
    ///协议值 "Horizontal"。
    Horizontal,
    ///协议值 "Vertical"。
    Vertical,
    ///协议值 "Both"。
    Both,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl PhysicalAxes {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::dom::types::PhysicalAxes::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Horizontal => "Horizontal",
            Self::Vertical => "Vertical",
            Self::Both => "Both",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for PhysicalAxes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for PhysicalAxes {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Horizontal" => PhysicalAxes::Horizontal,
            "Vertical" => PhysicalAxes::Vertical,
            "Both" => PhysicalAxes::Both,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for PhysicalAxes {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PhysicalAxes {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PhysicalAxes;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Horizontal" => PhysicalAxes::Horizontal,
                    "Vertical" => PhysicalAxes::Vertical,
                    "Both" => PhysicalAxes::Both,
                    _ => PhysicalAxes::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(PhysicalAxes::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：DOM.type.PseudoType。
///
///协议原文：
///
///<pre>Pseudo element type.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PseudoType {
    ///协议值 "first-line"。
    FirstLine,
    ///协议值 "first-letter"。
    FirstLetter,
    ///协议值 "checkmark"。
    Checkmark,
    ///协议值 "before"。
    Before,
    ///协议值 "after"。
    After,
    ///协议值 "expand-icon"。
    ExpandIcon,
    ///协议值 "picker-icon"。
    PickerIcon,
    ///协议值 "interest-button"。
    InterestButton,
    ///协议值 "marker"。
    Marker,
    ///协议值 "backdrop"。
    Backdrop,
    ///协议值 "column"。
    Column,
    ///协议值 "selection"。
    Selection,
    ///协议值 "search-text"。
    SearchText,
    ///协议值 "target-text"。
    TargetText,
    ///协议值 "spelling-error"。
    SpellingError,
    ///协议值 "grammar-error"。
    GrammarError,
    ///协议值 "highlight"。
    Highlight,
    ///协议值 "first-line-inherited"。
    FirstLineInherited,
    ///协议值 "scroll-marker"。
    ScrollMarker,
    ///协议值 "scroll-marker-group"。
    ScrollMarkerGroup,
    ///协议值 "scroll-button"。
    ScrollButton,
    ///协议值 "scrollbar"。
    Scrollbar,
    ///协议值 "scrollbar-thumb"。
    ScrollbarThumb,
    ///协议值 "scrollbar-button"。
    ScrollbarButton,
    ///协议值 "scrollbar-track"。
    ScrollbarTrack,
    ///协议值 "scrollbar-track-piece"。
    ScrollbarTrackPiece,
    ///协议值 "scrollbar-corner"。
    ScrollbarCorner,
    ///协议值 "resizer"。
    Resizer,
    ///协议值 "input-list-button"。
    InputListButton,
    ///协议值 "view-transition"。
    ViewTransition,
    ///协议值 "view-transition-group"。
    ViewTransitionGroup,
    ///协议值 "view-transition-image-pair"。
    ViewTransitionImagePair,
    ///协议值 "view-transition-group-children"。
    ViewTransitionGroupChildren,
    ///协议值 "view-transition-old"。
    ViewTransitionOld,
    ///协议值 "view-transition-new"。
    ViewTransitionNew,
    ///协议值 "placeholder"。
    Placeholder,
    ///协议值 "file-selector-button"。
    FileSelectorButton,
    ///协议值 "details-content"。
    DetailsContent,
    ///协议值 "picker"。
    Picker,
    ///协议值 "select-listbox"。
    SelectListbox,
    ///协议值 "permission-icon"。
    PermissionIcon,
    ///协议值 "overscroll-area-parent"。
    OverscrollAreaParent,
    ///协议值 "overscroll-backdrop"。
    OverscrollBackdrop,
    ///协议值 "skeleton"。
    Skeleton,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl PseudoType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::dom::types::PseudoType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::FirstLine => "first-line",
            Self::FirstLetter => "first-letter",
            Self::Checkmark => "checkmark",
            Self::Before => "before",
            Self::After => "after",
            Self::ExpandIcon => "expand-icon",
            Self::PickerIcon => "picker-icon",
            Self::InterestButton => "interest-button",
            Self::Marker => "marker",
            Self::Backdrop => "backdrop",
            Self::Column => "column",
            Self::Selection => "selection",
            Self::SearchText => "search-text",
            Self::TargetText => "target-text",
            Self::SpellingError => "spelling-error",
            Self::GrammarError => "grammar-error",
            Self::Highlight => "highlight",
            Self::FirstLineInherited => "first-line-inherited",
            Self::ScrollMarker => "scroll-marker",
            Self::ScrollMarkerGroup => "scroll-marker-group",
            Self::ScrollButton => "scroll-button",
            Self::Scrollbar => "scrollbar",
            Self::ScrollbarThumb => "scrollbar-thumb",
            Self::ScrollbarButton => "scrollbar-button",
            Self::ScrollbarTrack => "scrollbar-track",
            Self::ScrollbarTrackPiece => "scrollbar-track-piece",
            Self::ScrollbarCorner => "scrollbar-corner",
            Self::Resizer => "resizer",
            Self::InputListButton => "input-list-button",
            Self::ViewTransition => "view-transition",
            Self::ViewTransitionGroup => "view-transition-group",
            Self::ViewTransitionImagePair => "view-transition-image-pair",
            Self::ViewTransitionGroupChildren => "view-transition-group-children",
            Self::ViewTransitionOld => "view-transition-old",
            Self::ViewTransitionNew => "view-transition-new",
            Self::Placeholder => "placeholder",
            Self::FileSelectorButton => "file-selector-button",
            Self::DetailsContent => "details-content",
            Self::Picker => "picker",
            Self::SelectListbox => "select-listbox",
            Self::PermissionIcon => "permission-icon",
            Self::OverscrollAreaParent => "overscroll-area-parent",
            Self::OverscrollBackdrop => "overscroll-backdrop",
            Self::Skeleton => "skeleton",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for PseudoType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for PseudoType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "first-line" => PseudoType::FirstLine,
            "first-letter" => PseudoType::FirstLetter,
            "checkmark" => PseudoType::Checkmark,
            "before" => PseudoType::Before,
            "after" => PseudoType::After,
            "expand-icon" => PseudoType::ExpandIcon,
            "picker-icon" => PseudoType::PickerIcon,
            "interest-button" => PseudoType::InterestButton,
            "marker" => PseudoType::Marker,
            "backdrop" => PseudoType::Backdrop,
            "column" => PseudoType::Column,
            "selection" => PseudoType::Selection,
            "search-text" => PseudoType::SearchText,
            "target-text" => PseudoType::TargetText,
            "spelling-error" => PseudoType::SpellingError,
            "grammar-error" => PseudoType::GrammarError,
            "highlight" => PseudoType::Highlight,
            "first-line-inherited" => PseudoType::FirstLineInherited,
            "scroll-marker" => PseudoType::ScrollMarker,
            "scroll-marker-group" => PseudoType::ScrollMarkerGroup,
            "scroll-button" => PseudoType::ScrollButton,
            "scrollbar" => PseudoType::Scrollbar,
            "scrollbar-thumb" => PseudoType::ScrollbarThumb,
            "scrollbar-button" => PseudoType::ScrollbarButton,
            "scrollbar-track" => PseudoType::ScrollbarTrack,
            "scrollbar-track-piece" => PseudoType::ScrollbarTrackPiece,
            "scrollbar-corner" => PseudoType::ScrollbarCorner,
            "resizer" => PseudoType::Resizer,
            "input-list-button" => PseudoType::InputListButton,
            "view-transition" => PseudoType::ViewTransition,
            "view-transition-group" => PseudoType::ViewTransitionGroup,
            "view-transition-image-pair" => PseudoType::ViewTransitionImagePair,
            "view-transition-group-children" => PseudoType::ViewTransitionGroupChildren,
            "view-transition-old" => PseudoType::ViewTransitionOld,
            "view-transition-new" => PseudoType::ViewTransitionNew,
            "placeholder" => PseudoType::Placeholder,
            "file-selector-button" => PseudoType::FileSelectorButton,
            "details-content" => PseudoType::DetailsContent,
            "picker" => PseudoType::Picker,
            "select-listbox" => PseudoType::SelectListbox,
            "permission-icon" => PseudoType::PermissionIcon,
            "overscroll-area-parent" => PseudoType::OverscrollAreaParent,
            "overscroll-backdrop" => PseudoType::OverscrollBackdrop,
            "skeleton" => PseudoType::Skeleton,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for PseudoType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PseudoType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PseudoType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "first-line" => PseudoType::FirstLine,
                    "first-letter" => PseudoType::FirstLetter,
                    "checkmark" => PseudoType::Checkmark,
                    "before" => PseudoType::Before,
                    "after" => PseudoType::After,
                    "expand-icon" => PseudoType::ExpandIcon,
                    "picker-icon" => PseudoType::PickerIcon,
                    "interest-button" => PseudoType::InterestButton,
                    "marker" => PseudoType::Marker,
                    "backdrop" => PseudoType::Backdrop,
                    "column" => PseudoType::Column,
                    "selection" => PseudoType::Selection,
                    "search-text" => PseudoType::SearchText,
                    "target-text" => PseudoType::TargetText,
                    "spelling-error" => PseudoType::SpellingError,
                    "grammar-error" => PseudoType::GrammarError,
                    "highlight" => PseudoType::Highlight,
                    "first-line-inherited" => PseudoType::FirstLineInherited,
                    "scroll-marker" => PseudoType::ScrollMarker,
                    "scroll-marker-group" => PseudoType::ScrollMarkerGroup,
                    "scroll-button" => PseudoType::ScrollButton,
                    "scrollbar" => PseudoType::Scrollbar,
                    "scrollbar-thumb" => PseudoType::ScrollbarThumb,
                    "scrollbar-button" => PseudoType::ScrollbarButton,
                    "scrollbar-track" => PseudoType::ScrollbarTrack,
                    "scrollbar-track-piece" => PseudoType::ScrollbarTrackPiece,
                    "scrollbar-corner" => PseudoType::ScrollbarCorner,
                    "resizer" => PseudoType::Resizer,
                    "input-list-button" => PseudoType::InputListButton,
                    "view-transition" => PseudoType::ViewTransition,
                    "view-transition-group" => PseudoType::ViewTransitionGroup,
                    "view-transition-image-pair" => PseudoType::ViewTransitionImagePair,
                    "view-transition-group-children" => PseudoType::ViewTransitionGroupChildren,
                    "view-transition-old" => PseudoType::ViewTransitionOld,
                    "view-transition-new" => PseudoType::ViewTransitionNew,
                    "placeholder" => PseudoType::Placeholder,
                    "file-selector-button" => PseudoType::FileSelectorButton,
                    "details-content" => PseudoType::DetailsContent,
                    "picker" => PseudoType::Picker,
                    "select-listbox" => PseudoType::SelectListbox,
                    "permission-icon" => PseudoType::PermissionIcon,
                    "overscroll-area-parent" => PseudoType::OverscrollAreaParent,
                    "overscroll-backdrop" => PseudoType::OverscrollBackdrop,
                    "skeleton" => PseudoType::Skeleton,
                    _ => PseudoType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(PseudoType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：DOM.type.Quad。
///
///协议原文：
///
///<pre>An array of quad vertices, x immediately followed by y for each point, points clock-wise.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Quad(Vec<ydp_core::FiniteF64>);
impl Quad {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::dom::types::Quad = serde_json::from_str("[]").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::dom::types::Quad::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("[]").unwrap());
    ///```
    pub fn new(value: Vec<ydp_core::FiniteF64>) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::dom::types::Quad = serde_json::from_str("[]").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::dom::types::Quad::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("[]").unwrap());
    ///```
    pub fn into_inner(self) -> Vec<ydp_core::FiniteF64> {
        self.0
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.type.RGBA。
///
///协议原文：
///
///<pre>A structure holding an RGBA color.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct Rgba {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.RGBA/properties/r。
    ///
    ///协议原文：
    ///
    ///<pre>The red component, in the [0-255] range.</pre>
    ///
    ///
    #[serde(rename = "r")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.RGBA/properties/g。
    ///
    ///协议原文：
    ///
    ///<pre>The green component, in the [0-255] range.</pre>
    ///
    ///
    #[serde(rename = "g")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub g: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.RGBA/properties/b。
    ///
    ///协议原文：
    ///
    ///<pre>The blue component, in the [0-255] range.</pre>
    ///
    ///
    #[serde(rename = "b")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub b: i64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.RGBA/properties/a。
    ///
    ///协议原文：
    ///
    ///<pre>The alpha component, in the [0-1] range (default: 1).</pre>
    ///
    ///
    #[serde(rename = "a")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub a: Option<ydp_core::FiniteF64>,
}
impl<'de> serde::Deserialize<'de> for Rgba {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Rgba;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.RGBA/properties/r。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The red component, in the [0-255] range.</pre>
                    ///
                    ///
                    #[serde(rename = "r")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.RGBA/properties/g。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The green component, in the [0-255] range.</pre>
                    ///
                    ///
                    #[serde(rename = "g")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub g: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.RGBA/properties/b。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The blue component, in the [0-255] range.</pre>
                    ///
                    ///
                    #[serde(rename = "b")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub b: i64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.type.RGBA/properties/a。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The alpha component, in the [0-1] range (default: 1).</pre>
                    ///
                    ///
                    #[serde(rename = "a")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub a: Option<ydp_core::FiniteF64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Rgba {
                    r: fields.r,
                    g: fields.g,
                    b: fields.b,
                    a: fields.a,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.type.Rect。
///
///协议原文：
///
///<pre>Rectangle.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct Rect {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.Rect/properties/x。
    ///
    ///协议原文：
    ///
    ///<pre>X coordinate</pre>
    ///
    ///
    #[serde(rename = "x")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub x: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.Rect/properties/y。
    ///
    ///协议原文：
    ///
    ///<pre>Y coordinate</pre>
    ///
    ///
    #[serde(rename = "y")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub y: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.Rect/properties/width。
    ///
    ///协议原文：
    ///
    ///<pre>Rectangle width</pre>
    ///
    ///
    #[serde(rename = "width")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub width: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.Rect/properties/height。
    ///
    ///协议原文：
    ///
    ///<pre>Rectangle height</pre>
    ///
    ///
    #[serde(rename = "height")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub height: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for Rect {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Rect;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.Rect/properties/x。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>X coordinate</pre>
                    ///
                    ///
                    #[serde(rename = "x")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub x: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.Rect/properties/y。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Y coordinate</pre>
                    ///
                    ///
                    #[serde(rename = "y")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub y: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.Rect/properties/width。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Rectangle width</pre>
                    ///
                    ///
                    #[serde(rename = "width")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub width: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.Rect/properties/height。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Rectangle height</pre>
                    ///
                    ///
                    #[serde(rename = "height")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub height: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Rect {
                    x: fields.x,
                    y: fields.y,
                    width: fields.width,
                    height: fields.height,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：DOM.type.ScrollOrientation。
///
///协议原文：
///
///<pre>Physical scroll orientation</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ScrollOrientation {
    ///协议值 "horizontal"。
    Horizontal,
    ///协议值 "vertical"。
    Vertical,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ScrollOrientation {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::dom::types::ScrollOrientation::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ScrollOrientation {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ScrollOrientation {
    fn from(value: String) -> Self {
        match value.as_str() {
            "horizontal" => ScrollOrientation::Horizontal,
            "vertical" => ScrollOrientation::Vertical,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ScrollOrientation {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ScrollOrientation {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScrollOrientation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "horizontal" => ScrollOrientation::Horizontal,
                    "vertical" => ScrollOrientation::Vertical,
                    _ => ScrollOrientation::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ScrollOrientation::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：DOM.type.ShadowRootType。
///
///协议原文：
///
///<pre>Shadow root type.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ShadowRootType {
    ///协议值 "user-agent"。
    UserAgent,
    ///协议值 "open"。
    Open,
    ///协议值 "closed"。
    Closed,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ShadowRootType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::dom::types::ShadowRootType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::UserAgent => "user-agent",
            Self::Open => "open",
            Self::Closed => "closed",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ShadowRootType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ShadowRootType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "user-agent" => ShadowRootType::UserAgent,
            "open" => ShadowRootType::Open,
            "closed" => ShadowRootType::Closed,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ShadowRootType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ShadowRootType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ShadowRootType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "user-agent" => ShadowRootType::UserAgent,
                    "open" => ShadowRootType::Open,
                    "closed" => ShadowRootType::Closed,
                    _ => ShadowRootType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ShadowRootType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.type.ShapeOutsideInfo。
///
///协议原文：
///
///<pre>CSS Shape Outside details.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ShapeOutsideInfo {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.ShapeOutsideInfo/properties/bounds。
    ///
    ///协议原文：
    ///
    ///<pre>Shape bounds</pre>
    ///
    ///
    #[serde(rename = "bounds")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub bounds: crate::dom::types::Quad,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.ShapeOutsideInfo/properties/shape。
    ///
    ///协议原文：
    ///
    ///<pre>Shape coordinate details</pre>
    ///
    ///
    #[serde(rename = "shape")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub shape: Vec<serde_json::Value>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.ShapeOutsideInfo/properties/marginShape。
    ///
    ///协议原文：
    ///
    ///<pre>Margin shape bounds</pre>
    ///
    ///
    #[serde(rename = "marginShape")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub margin_shape: Vec<serde_json::Value>,
}
impl<'de> serde::Deserialize<'de> for ShapeOutsideInfo {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ShapeOutsideInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.ShapeOutsideInfo/properties/bounds。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Shape bounds</pre>
                    ///
                    ///
                    #[serde(rename = "bounds")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub bounds: crate::dom::types::Quad,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.ShapeOutsideInfo/properties/shape。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Shape coordinate details</pre>
                    ///
                    ///
                    #[serde(rename = "shape")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub shape: Vec<serde_json::Value>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.type.ShapeOutsideInfo/properties/marginShape。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Margin shape bounds</pre>
                    ///
                    ///
                    #[serde(rename = "marginShape")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub margin_shape: Vec<serde_json::Value>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ShapeOutsideInfo {
                    bounds: fields.bounds,
                    shape: fields.shape,
                    margin_shape: fields.margin_shape,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：DOM.type.StyleSheetId。
///
///协议原文：
///
///<pre>Unique identifier for a CSS stylesheet.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct StyleSheetId(String);
impl StyleSheetId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::dom::types::StyleSheetId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::dom::types::StyleSheetId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::dom::types::StyleSheetId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::dom::types::StyleSheetId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::dom::types::StyleSheetId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
