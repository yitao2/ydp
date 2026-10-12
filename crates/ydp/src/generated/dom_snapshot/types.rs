///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：DOMSnapshot.type.ArrayOfStrings。
///
///协议原文：
///
///<pre>Index of the string in the strings table.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ArrayOfStrings(Vec<crate::dom_snapshot::types::StringIndex>);
impl ArrayOfStrings {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::dom_snapshot::types::ArrayOfStrings = serde_json::from_str("[]").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::dom_snapshot::types::ArrayOfStrings::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("[]").unwrap());
    ///```
    pub fn new(value: Vec<crate::dom_snapshot::types::StringIndex>) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::dom_snapshot::types::ArrayOfStrings = serde_json::from_str("[]").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::dom_snapshot::types::ArrayOfStrings::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("[]").unwrap());
    ///```
    pub fn into_inner(self) -> Vec<crate::dom_snapshot::types::StringIndex> {
        self.0
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMSnapshot.type.ComputedStyle。
///
///协议原文：
///
///<pre>A subset of the full ComputedStyle as defined by the request whitelist.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ComputedStyle {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.ComputedStyle/properties/properties。
    ///
    ///协议原文：
    ///
    ///<pre>Name/value pairs of computed style properties.</pre>
    ///
    ///
    #[serde(rename = "properties")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub properties: Vec<crate::dom_snapshot::types::NameValue>,
}
impl<'de> serde::Deserialize<'de> for ComputedStyle {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ComputedStyle;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.ComputedStyle/properties/properties。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Name/value pairs of computed style properties.</pre>
                    ///
                    ///
                    #[serde(rename = "properties")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub properties: Vec<crate::dom_snapshot::types::NameValue>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ComputedStyle {
                    properties: fields.properties,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMSnapshot.type.DOMNode。
///
///协议原文：
///
///<pre>A Node in the DOM tree.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct DomNode {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DOMNode/properties/nodeType。
    ///
    ///协议原文：
    ///
    ///<pre>`Node`'s nodeType.</pre>
    ///
    ///
    #[serde(rename = "nodeType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_type: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DOMNode/properties/nodeName。
    ///
    ///协议原文：
    ///
    ///<pre>`Node`'s nodeName.</pre>
    ///
    ///
    #[serde(rename = "nodeName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DOMNode/properties/nodeValue。
    ///
    ///协议原文：
    ///
    ///<pre>`Node`'s nodeValue.</pre>
    ///
    ///
    #[serde(rename = "nodeValue")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_value: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/textValue。
    ///
    ///协议原文：
    ///
    ///<pre>Only set for textarea elements, contains the text value.</pre>
    ///
    ///
    #[serde(rename = "textValue")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub text_value: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/inputValue。
    ///
    ///协议原文：
    ///
    ///<pre>Only set for input elements, contains the input's associated text value.</pre>
    ///
    ///
    #[serde(rename = "inputValue")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub input_value: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/inputChecked。
    ///
    ///协议原文：
    ///
    ///<pre>Only set for radio and checkbox input elements, indicates if the element has been checked</pre>
    ///
    ///
    #[serde(rename = "inputChecked")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub input_checked: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/optionSelected。
    ///
    ///协议原文：
    ///
    ///<pre>Only set for option elements, indicates if the element has been selected</pre>
    ///
    ///
    #[serde(rename = "optionSelected")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub option_selected: Option<bool>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DOMNode/properties/backendNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>`Node`'s id, corresponds to DOM.Node.backendNodeId.</pre>
    ///
    ///
    #[serde(rename = "backendNodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub backend_node_id: crate::dom::types::BackendNodeId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/childNodeIndexes。
    ///
    ///协议原文：
    ///
    ///<pre>The indexes of the node's child nodes in the `domNodes` array returned by `getSnapshot`, if
    ///any.</pre>
    ///
    ///
    #[serde(rename = "childNodeIndexes")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub child_node_indexes: Option<Vec<i64>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/attributes。
    ///
    ///协议原文：
    ///
    ///<pre>Attributes of an `Element` node.</pre>
    ///
    ///
    #[serde(rename = "attributes")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub attributes: Option<Vec<crate::dom_snapshot::types::NameValue>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/pseudoElementIndexes。
    ///
    ///协议原文：
    ///
    ///<pre>Indexes of pseudo elements associated with this node in the `domNodes` array returned by
    ///`getSnapshot`, if any.</pre>
    ///
    ///
    #[serde(rename = "pseudoElementIndexes")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub pseudo_element_indexes: Option<Vec<i64>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/layoutNodeIndex。
    ///
    ///协议原文：
    ///
    ///<pre>The index of the node's related layout tree node in the `layoutTreeNodes` array returned by
    ///`getSnapshot`, if any.</pre>
    ///
    ///
    #[serde(rename = "layoutNodeIndex")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub layout_node_index: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/documentURL。
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
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/baseURL。
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
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/contentLanguage。
    ///
    ///协议原文：
    ///
    ///<pre>Only set for documents, contains the document's content language.</pre>
    ///
    ///
    #[serde(rename = "contentLanguage")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub content_language: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/documentEncoding。
    ///
    ///协议原文：
    ///
    ///<pre>Only set for documents, contains the document's character set encoding.</pre>
    ///
    ///
    #[serde(rename = "documentEncoding")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub document_encoding: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/publicId。
    ///
    ///协议原文：
    ///
    ///<pre>`DocumentType` node's publicId.</pre>
    ///
    ///
    #[serde(rename = "publicId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub public_id: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/systemId。
    ///
    ///协议原文：
    ///
    ///<pre>`DocumentType` node's systemId.</pre>
    ///
    ///
    #[serde(rename = "systemId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub system_id: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Frame ID for frame owner elements and also for the document node.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub frame_id: Option<crate::page::types::FrameId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/contentDocumentIndex。
    ///
    ///协议原文：
    ///
    ///<pre>The index of a frame owner element's content document in the `domNodes` array returned by
    ///`getSnapshot`, if any.</pre>
    ///
    ///
    #[serde(rename = "contentDocumentIndex")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub content_document_index: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/pseudoType。
    ///
    ///协议原文：
    ///
    ///<pre>Type of a pseudo element node.</pre>
    ///
    ///
    #[serde(rename = "pseudoType")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub pseudo_type: Option<crate::dom::types::PseudoType>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/shadowRootType。
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
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/isClickable。
    ///
    ///协议原文：
    ///
    ///<pre>Whether this DOM node responds to mouse clicks. This includes nodes that have had click
    ///event listeners attached via JavaScript as well as anchor tags that naturally navigate when
    ///clicked.</pre>
    ///
    ///
    #[serde(rename = "isClickable")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_clickable: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/eventListeners。
    ///
    ///协议原文：
    ///
    ///<pre>Details of the node's event listeners, if any.</pre>
    ///
    ///
    #[serde(rename = "eventListeners")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub event_listeners: Option<Vec<crate::dom_debugger::types::EventListener>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/currentSourceURL。
    ///
    ///协议原文：
    ///
    ///<pre>The selected url for nodes with a srcset attribute.</pre>
    ///
    ///
    #[serde(rename = "currentSourceURL")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub current_source_url: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/originURL。
    ///
    ///协议原文：
    ///
    ///<pre>The url of the script (if any) that generates this node.</pre>
    ///
    ///
    #[serde(rename = "originURL")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub origin_url: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/scrollOffsetX。
    ///
    ///协议原文：
    ///
    ///<pre>Scroll offsets, set when this node is a Document.</pre>
    ///
    ///
    #[serde(rename = "scrollOffsetX")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub scroll_offset_x: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/scrollOffsetY。
    #[serde(rename = "scrollOffsetY")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub scroll_offset_y: Option<ydp_core::FiniteF64>,
}
impl<'de> serde::Deserialize<'de> for DomNode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DomNode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DOMNode/properties/nodeType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`Node`'s nodeType.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_type: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DOMNode/properties/nodeName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`Node`'s nodeName.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DOMNode/properties/nodeValue。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`Node`'s nodeValue.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeValue")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_value: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/textValue。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Only set for textarea elements, contains the text value.</pre>
                    ///
                    ///
                    #[serde(rename = "textValue")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub text_value: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/inputValue。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Only set for input elements, contains the input's associated text value.</pre>
                    ///
                    ///
                    #[serde(rename = "inputValue")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub input_value: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/inputChecked。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Only set for radio and checkbox input elements, indicates if the element has been checked</pre>
                    ///
                    ///
                    #[serde(rename = "inputChecked")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub input_checked: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/optionSelected。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Only set for option elements, indicates if the element has been selected</pre>
                    ///
                    ///
                    #[serde(rename = "optionSelected")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub option_selected: Option<bool>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DOMNode/properties/backendNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`Node`'s id, corresponds to DOM.Node.backendNodeId.</pre>
                    ///
                    ///
                    #[serde(rename = "backendNodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub backend_node_id: crate::dom::types::BackendNodeId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/childNodeIndexes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The indexes of the node's child nodes in the `domNodes` array returned by `getSnapshot`, if
                    ///any.</pre>
                    ///
                    ///
                    #[serde(rename = "childNodeIndexes")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub child_node_indexes: Option<Vec<i64>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/attributes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Attributes of an `Element` node.</pre>
                    ///
                    ///
                    #[serde(rename = "attributes")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub attributes: Option<Vec<crate::dom_snapshot::types::NameValue>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/pseudoElementIndexes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Indexes of pseudo elements associated with this node in the `domNodes` array returned by
                    ///`getSnapshot`, if any.</pre>
                    ///
                    ///
                    #[serde(rename = "pseudoElementIndexes")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub pseudo_element_indexes: Option<Vec<i64>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/layoutNodeIndex。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The index of the node's related layout tree node in the `layoutTreeNodes` array returned by
                    ///`getSnapshot`, if any.</pre>
                    ///
                    ///
                    #[serde(rename = "layoutNodeIndex")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub layout_node_index: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/documentURL。
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
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/baseURL。
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
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/contentLanguage。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Only set for documents, contains the document's content language.</pre>
                    ///
                    ///
                    #[serde(rename = "contentLanguage")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub content_language: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/documentEncoding。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Only set for documents, contains the document's character set encoding.</pre>
                    ///
                    ///
                    #[serde(rename = "documentEncoding")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub document_encoding: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/publicId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`DocumentType` node's publicId.</pre>
                    ///
                    ///
                    #[serde(rename = "publicId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub public_id: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/systemId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`DocumentType` node's systemId.</pre>
                    ///
                    ///
                    #[serde(rename = "systemId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub system_id: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame ID for frame owner elements and also for the document node.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub frame_id: Option<crate::page::types::FrameId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/contentDocumentIndex。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The index of a frame owner element's content document in the `domNodes` array returned by
                    ///`getSnapshot`, if any.</pre>
                    ///
                    ///
                    #[serde(rename = "contentDocumentIndex")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub content_document_index: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/pseudoType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Type of a pseudo element node.</pre>
                    ///
                    ///
                    #[serde(rename = "pseudoType")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub pseudo_type: Option<crate::dom::types::PseudoType>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/shadowRootType。
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
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/isClickable。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether this DOM node responds to mouse clicks. This includes nodes that have had click
                    ///event listeners attached via JavaScript as well as anchor tags that naturally navigate when
                    ///clicked.</pre>
                    ///
                    ///
                    #[serde(rename = "isClickable")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub is_clickable: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/eventListeners。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Details of the node's event listeners, if any.</pre>
                    ///
                    ///
                    #[serde(rename = "eventListeners")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub event_listeners: Option<Vec<crate::dom_debugger::types::EventListener>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/currentSourceURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The selected url for nodes with a srcset attribute.</pre>
                    ///
                    ///
                    #[serde(rename = "currentSourceURL")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub current_source_url: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/originURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The url of the script (if any) that generates this node.</pre>
                    ///
                    ///
                    #[serde(rename = "originURL")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub origin_url: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/scrollOffsetX。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Scroll offsets, set when this node is a Document.</pre>
                    ///
                    ///
                    #[serde(rename = "scrollOffsetX")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub scroll_offset_x: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DOMNode/properties/scrollOffsetY。
                    #[serde(rename = "scrollOffsetY")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub scroll_offset_y: Option<ydp_core::FiniteF64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DomNode {
                    node_type: fields.node_type,
                    node_name: fields.node_name,
                    node_value: fields.node_value,
                    text_value: fields.text_value,
                    input_value: fields.input_value,
                    input_checked: fields.input_checked,
                    option_selected: fields.option_selected,
                    backend_node_id: fields.backend_node_id,
                    child_node_indexes: fields.child_node_indexes,
                    attributes: fields.attributes,
                    pseudo_element_indexes: fields.pseudo_element_indexes,
                    layout_node_index: fields.layout_node_index,
                    document_url: fields.document_url,
                    base_url: fields.base_url,
                    content_language: fields.content_language,
                    document_encoding: fields.document_encoding,
                    public_id: fields.public_id,
                    system_id: fields.system_id,
                    frame_id: fields.frame_id,
                    content_document_index: fields.content_document_index,
                    pseudo_type: fields.pseudo_type,
                    shadow_root_type: fields.shadow_root_type,
                    is_clickable: fields.is_clickable,
                    event_listeners: fields.event_listeners,
                    current_source_url: fields.current_source_url,
                    origin_url: fields.origin_url,
                    scroll_offset_x: fields.scroll_offset_x,
                    scroll_offset_y: fields.scroll_offset_y,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMSnapshot.type.DocumentSnapshot。
///
///协议原文：
///
///<pre>Document snapshot.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct DocumentSnapshot {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/documentURL。
    ///
    ///协议原文：
    ///
    ///<pre>Document URL that `Document` or `FrameOwner` node points to.</pre>
    ///
    ///
    #[serde(rename = "documentURL")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub document_url: crate::dom_snapshot::types::StringIndex,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/title。
    ///
    ///协议原文：
    ///
    ///<pre>Document title.</pre>
    ///
    ///
    #[serde(rename = "title")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub title: crate::dom_snapshot::types::StringIndex,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/baseURL。
    ///
    ///协议原文：
    ///
    ///<pre>Base URL that `Document` or `FrameOwner` node uses for URL completion.</pre>
    ///
    ///
    #[serde(rename = "baseURL")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub base_url: crate::dom_snapshot::types::StringIndex,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/contentLanguage。
    ///
    ///协议原文：
    ///
    ///<pre>Contains the document's content language.</pre>
    ///
    ///
    #[serde(rename = "contentLanguage")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub content_language: crate::dom_snapshot::types::StringIndex,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/encodingName。
    ///
    ///协议原文：
    ///
    ///<pre>Contains the document's character set encoding.</pre>
    ///
    ///
    #[serde(rename = "encodingName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub encoding_name: crate::dom_snapshot::types::StringIndex,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/publicId。
    ///
    ///协议原文：
    ///
    ///<pre>`DocumentType` node's publicId.</pre>
    ///
    ///
    #[serde(rename = "publicId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub public_id: crate::dom_snapshot::types::StringIndex,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/systemId。
    ///
    ///协议原文：
    ///
    ///<pre>`DocumentType` node's systemId.</pre>
    ///
    ///
    #[serde(rename = "systemId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub system_id: crate::dom_snapshot::types::StringIndex,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Frame ID for frame owner elements and also for the document node.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::dom_snapshot::types::StringIndex,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/nodes。
    ///
    ///协议原文：
    ///
    ///<pre>A table with dom nodes.</pre>
    ///
    ///
    #[serde(rename = "nodes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub nodes: crate::dom_snapshot::types::NodeTreeSnapshot,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/layout。
    ///
    ///协议原文：
    ///
    ///<pre>The nodes in the layout tree.</pre>
    ///
    ///
    #[serde(rename = "layout")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub layout: crate::dom_snapshot::types::LayoutTreeSnapshot,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/textBoxes。
    ///
    ///协议原文：
    ///
    ///<pre>The post-layout inline text nodes.</pre>
    ///
    ///
    #[serde(rename = "textBoxes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text_boxes: crate::dom_snapshot::types::TextBoxSnapshot,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/scrollOffsetX。
    ///
    ///协议原文：
    ///
    ///<pre>Horizontal scroll offset.</pre>
    ///
    ///
    #[serde(rename = "scrollOffsetX")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub scroll_offset_x: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/scrollOffsetY。
    ///
    ///协议原文：
    ///
    ///<pre>Vertical scroll offset.</pre>
    ///
    ///
    #[serde(rename = "scrollOffsetY")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub scroll_offset_y: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/contentWidth。
    ///
    ///协议原文：
    ///
    ///<pre>Document content width.</pre>
    ///
    ///
    #[serde(rename = "contentWidth")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub content_width: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/contentHeight。
    ///
    ///协议原文：
    ///
    ///<pre>Document content height.</pre>
    ///
    ///
    #[serde(rename = "contentHeight")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub content_height: Option<ydp_core::FiniteF64>,
}
impl<'de> serde::Deserialize<'de> for DocumentSnapshot {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DocumentSnapshot;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/documentURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Document URL that `Document` or `FrameOwner` node points to.</pre>
                    ///
                    ///
                    #[serde(rename = "documentURL")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub document_url: crate::dom_snapshot::types::StringIndex,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/title。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Document title.</pre>
                    ///
                    ///
                    #[serde(rename = "title")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub title: crate::dom_snapshot::types::StringIndex,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/baseURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Base URL that `Document` or `FrameOwner` node uses for URL completion.</pre>
                    ///
                    ///
                    #[serde(rename = "baseURL")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub base_url: crate::dom_snapshot::types::StringIndex,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/contentLanguage。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Contains the document's content language.</pre>
                    ///
                    ///
                    #[serde(rename = "contentLanguage")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub content_language: crate::dom_snapshot::types::StringIndex,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/encodingName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Contains the document's character set encoding.</pre>
                    ///
                    ///
                    #[serde(rename = "encodingName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub encoding_name: crate::dom_snapshot::types::StringIndex,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/publicId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`DocumentType` node's publicId.</pre>
                    ///
                    ///
                    #[serde(rename = "publicId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub public_id: crate::dom_snapshot::types::StringIndex,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/systemId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`DocumentType` node's systemId.</pre>
                    ///
                    ///
                    #[serde(rename = "systemId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub system_id: crate::dom_snapshot::types::StringIndex,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame ID for frame owner elements and also for the document node.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::dom_snapshot::types::StringIndex,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/nodes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A table with dom nodes.</pre>
                    ///
                    ///
                    #[serde(rename = "nodes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub nodes: crate::dom_snapshot::types::NodeTreeSnapshot,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/layout。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The nodes in the layout tree.</pre>
                    ///
                    ///
                    #[serde(rename = "layout")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub layout: crate::dom_snapshot::types::LayoutTreeSnapshot,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/textBoxes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The post-layout inline text nodes.</pre>
                    ///
                    ///
                    #[serde(rename = "textBoxes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text_boxes: crate::dom_snapshot::types::TextBoxSnapshot,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/scrollOffsetX。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Horizontal scroll offset.</pre>
                    ///
                    ///
                    #[serde(rename = "scrollOffsetX")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub scroll_offset_x: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/scrollOffsetY。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Vertical scroll offset.</pre>
                    ///
                    ///
                    #[serde(rename = "scrollOffsetY")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub scroll_offset_y: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/contentWidth。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Document content width.</pre>
                    ///
                    ///
                    #[serde(rename = "contentWidth")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub content_width: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.DocumentSnapshot/properties/contentHeight。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Document content height.</pre>
                    ///
                    ///
                    #[serde(rename = "contentHeight")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub content_height: Option<ydp_core::FiniteF64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DocumentSnapshot {
                    document_url: fields.document_url,
                    title: fields.title,
                    base_url: fields.base_url,
                    content_language: fields.content_language,
                    encoding_name: fields.encoding_name,
                    public_id: fields.public_id,
                    system_id: fields.system_id,
                    frame_id: fields.frame_id,
                    nodes: fields.nodes,
                    layout: fields.layout,
                    text_boxes: fields.text_boxes,
                    scroll_offset_x: fields.scroll_offset_x,
                    scroll_offset_y: fields.scroll_offset_y,
                    content_width: fields.content_width,
                    content_height: fields.content_height,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMSnapshot.type.InlineTextBox。
///
///协议原文：
///
///<pre>Details of post layout rendered text positions. The exact layout should not be regarded as
///stable and may change between versions.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct InlineTextBox {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.InlineTextBox/properties/boundingBox。
    ///
    ///协议原文：
    ///
    ///<pre>The bounding box in document coordinates. Note that scroll offset of the document is ignored.</pre>
    ///
    ///
    #[serde(rename = "boundingBox")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub bounding_box: crate::dom::types::Rect,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.InlineTextBox/properties/startCharacterIndex。
    ///
    ///协议原文：
    ///
    ///<pre>The starting index in characters, for this post layout textbox substring. Characters that
    ///would be represented as a surrogate pair in UTF-16 have length 2.</pre>
    ///
    ///
    #[serde(rename = "startCharacterIndex")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub start_character_index: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.InlineTextBox/properties/numCharacters。
    ///
    ///协议原文：
    ///
    ///<pre>The number of characters in this post layout textbox substring. Characters that would be
    ///represented as a surrogate pair in UTF-16 have length 2.</pre>
    ///
    ///
    #[serde(rename = "numCharacters")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub num_characters: i64,
}
impl<'de> serde::Deserialize<'de> for InlineTextBox {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InlineTextBox;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.InlineTextBox/properties/boundingBox。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The bounding box in document coordinates. Note that scroll offset of the document is ignored.</pre>
                    ///
                    ///
                    #[serde(rename = "boundingBox")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub bounding_box: crate::dom::types::Rect,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.InlineTextBox/properties/startCharacterIndex。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The starting index in characters, for this post layout textbox substring. Characters that
                    ///would be represented as a surrogate pair in UTF-16 have length 2.</pre>
                    ///
                    ///
                    #[serde(rename = "startCharacterIndex")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub start_character_index: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.InlineTextBox/properties/numCharacters。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The number of characters in this post layout textbox substring. Characters that would be
                    ///represented as a surrogate pair in UTF-16 have length 2.</pre>
                    ///
                    ///
                    #[serde(rename = "numCharacters")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub num_characters: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(InlineTextBox {
                    bounding_box: fields.bounding_box,
                    start_character_index: fields.start_character_index,
                    num_characters: fields.num_characters,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMSnapshot.type.LayoutTreeNode。
///
///协议原文：
///
///<pre>Details of an element in the DOM tree with a LayoutObject.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct LayoutTreeNode {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.LayoutTreeNode/properties/domNodeIndex。
    ///
    ///协议原文：
    ///
    ///<pre>The index of the related DOM node in the `domNodes` array returned by `getSnapshot`.</pre>
    ///
    ///
    #[serde(rename = "domNodeIndex")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub dom_node_index: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.LayoutTreeNode/properties/boundingBox。
    ///
    ///协议原文：
    ///
    ///<pre>The bounding box in document coordinates. Note that scroll offset of the document is ignored.</pre>
    ///
    ///
    #[serde(rename = "boundingBox")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub bounding_box: crate::dom::types::Rect,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.LayoutTreeNode/properties/layoutText。
    ///
    ///协议原文：
    ///
    ///<pre>Contents of the LayoutText, if any.</pre>
    ///
    ///
    #[serde(rename = "layoutText")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub layout_text: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.LayoutTreeNode/properties/inlineTextNodes。
    ///
    ///协议原文：
    ///
    ///<pre>The post-layout inline text nodes, if any.</pre>
    ///
    ///
    #[serde(rename = "inlineTextNodes")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub inline_text_nodes: Option<Vec<crate::dom_snapshot::types::InlineTextBox>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.LayoutTreeNode/properties/styleIndex。
    ///
    ///协议原文：
    ///
    ///<pre>Index into the `computedStyles` array returned by `getSnapshot`.</pre>
    ///
    ///
    #[serde(rename = "styleIndex")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub style_index: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.LayoutTreeNode/properties/paintOrder。
    ///
    ///协议原文：
    ///
    ///<pre>Global paint order index, which is determined by the stacking order of the nodes. Nodes
    ///that are painted together will have the same index. Only provided if includePaintOrder in
    ///getSnapshot was true.</pre>
    ///
    ///
    #[serde(rename = "paintOrder")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub paint_order: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.LayoutTreeNode/properties/isStackingContext。
    ///
    ///协议原文：
    ///
    ///<pre>Set to true to indicate the element begins a new stacking context.</pre>
    ///
    ///
    #[serde(rename = "isStackingContext")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_stacking_context: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for LayoutTreeNode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LayoutTreeNode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.LayoutTreeNode/properties/domNodeIndex。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The index of the related DOM node in the `domNodes` array returned by `getSnapshot`.</pre>
                    ///
                    ///
                    #[serde(rename = "domNodeIndex")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub dom_node_index: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.LayoutTreeNode/properties/boundingBox。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The bounding box in document coordinates. Note that scroll offset of the document is ignored.</pre>
                    ///
                    ///
                    #[serde(rename = "boundingBox")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub bounding_box: crate::dom::types::Rect,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.LayoutTreeNode/properties/layoutText。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Contents of the LayoutText, if any.</pre>
                    ///
                    ///
                    #[serde(rename = "layoutText")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub layout_text: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.LayoutTreeNode/properties/inlineTextNodes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The post-layout inline text nodes, if any.</pre>
                    ///
                    ///
                    #[serde(rename = "inlineTextNodes")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub inline_text_nodes: Option<Vec<crate::dom_snapshot::types::InlineTextBox>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.LayoutTreeNode/properties/styleIndex。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Index into the `computedStyles` array returned by `getSnapshot`.</pre>
                    ///
                    ///
                    #[serde(rename = "styleIndex")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub style_index: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.LayoutTreeNode/properties/paintOrder。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Global paint order index, which is determined by the stacking order of the nodes. Nodes
                    ///that are painted together will have the same index. Only provided if includePaintOrder in
                    ///getSnapshot was true.</pre>
                    ///
                    ///
                    #[serde(rename = "paintOrder")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub paint_order: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.LayoutTreeNode/properties/isStackingContext。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Set to true to indicate the element begins a new stacking context.</pre>
                    ///
                    ///
                    #[serde(rename = "isStackingContext")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub is_stacking_context: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LayoutTreeNode {
                    dom_node_index: fields.dom_node_index,
                    bounding_box: fields.bounding_box,
                    layout_text: fields.layout_text,
                    inline_text_nodes: fields.inline_text_nodes,
                    style_index: fields.style_index,
                    paint_order: fields.paint_order,
                    is_stacking_context: fields.is_stacking_context,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMSnapshot.type.LayoutTreeSnapshot。
///
///协议原文：
///
///<pre>Table of details of an element in the DOM tree with a LayoutObject.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct LayoutTreeSnapshot {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.LayoutTreeSnapshot/properties/nodeIndex。
    ///
    ///协议原文：
    ///
    ///<pre>Index of the corresponding node in the `NodeTreeSnapshot` array returned by `captureSnapshot`.</pre>
    ///
    ///
    #[serde(rename = "nodeIndex")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_index: Vec<i64>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.LayoutTreeSnapshot/properties/styles。
    ///
    ///协议原文：
    ///
    ///<pre>Array of indexes specifying computed style strings, filtered according to the `computedStyles` parameter passed to `captureSnapshot`.</pre>
    ///
    ///
    #[serde(rename = "styles")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub styles: Vec<crate::dom_snapshot::types::ArrayOfStrings>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.LayoutTreeSnapshot/properties/bounds。
    ///
    ///协议原文：
    ///
    ///<pre>The absolute position bounding box.</pre>
    ///
    ///
    #[serde(rename = "bounds")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub bounds: Vec<crate::dom_snapshot::types::Rectangle>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.LayoutTreeSnapshot/properties/text。
    ///
    ///协议原文：
    ///
    ///<pre>Contents of the LayoutText, if any.</pre>
    ///
    ///
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: Vec<crate::dom_snapshot::types::StringIndex>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.LayoutTreeSnapshot/properties/stackingContexts。
    ///
    ///协议原文：
    ///
    ///<pre>Stacking context information.</pre>
    ///
    ///
    #[serde(rename = "stackingContexts")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub stacking_contexts: crate::dom_snapshot::types::RareBooleanData,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.LayoutTreeSnapshot/properties/paintOrders。
    ///
    ///协议原文：
    ///
    ///<pre>Global paint order index, which is determined by the stacking order of the nodes. Nodes
    ///that are painted together will have the same index. Only provided if includePaintOrder in
    ///captureSnapshot was true.</pre>
    ///
    ///
    #[serde(rename = "paintOrders")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub paint_orders: Option<Vec<i64>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.LayoutTreeSnapshot/properties/offsetRects。
    ///
    ///协议原文：
    ///
    ///<pre>The offset rect of nodes. Only available when includeDOMRects is set to true</pre>
    ///
    ///
    #[serde(rename = "offsetRects")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub offset_rects: Option<Vec<crate::dom_snapshot::types::Rectangle>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.LayoutTreeSnapshot/properties/scrollRects。
    ///
    ///协议原文：
    ///
    ///<pre>The scroll rect of nodes. Only available when includeDOMRects is set to true</pre>
    ///
    ///
    #[serde(rename = "scrollRects")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub scroll_rects: Option<Vec<crate::dom_snapshot::types::Rectangle>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.LayoutTreeSnapshot/properties/clientRects。
    ///
    ///协议原文：
    ///
    ///<pre>The client rect of nodes. Only available when includeDOMRects is set to true</pre>
    ///
    ///
    #[serde(rename = "clientRects")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub client_rects: Option<Vec<crate::dom_snapshot::types::Rectangle>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.LayoutTreeSnapshot/properties/blendedBackgroundColors。
    ///
    ///协议原文：
    ///
    ///<pre>The list of background colors that are blended with colors of overlapping elements.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "blendedBackgroundColors")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub blended_background_colors: Option<Vec<crate::dom_snapshot::types::StringIndex>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.LayoutTreeSnapshot/properties/textColorOpacities。
    ///
    ///协议原文：
    ///
    ///<pre>The list of computed text opacities.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "textColorOpacities")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub text_color_opacities: Option<Vec<ydp_core::FiniteF64>>,
}
impl<'de> serde::Deserialize<'de> for LayoutTreeSnapshot {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LayoutTreeSnapshot;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.LayoutTreeSnapshot/properties/nodeIndex。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Index of the corresponding node in the `NodeTreeSnapshot` array returned by `captureSnapshot`.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeIndex")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_index: Vec<i64>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.LayoutTreeSnapshot/properties/styles。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Array of indexes specifying computed style strings, filtered according to the `computedStyles` parameter passed to `captureSnapshot`.</pre>
                    ///
                    ///
                    #[serde(rename = "styles")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub styles: Vec<crate::dom_snapshot::types::ArrayOfStrings>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.LayoutTreeSnapshot/properties/bounds。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The absolute position bounding box.</pre>
                    ///
                    ///
                    #[serde(rename = "bounds")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub bounds: Vec<crate::dom_snapshot::types::Rectangle>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.LayoutTreeSnapshot/properties/text。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Contents of the LayoutText, if any.</pre>
                    ///
                    ///
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: Vec<crate::dom_snapshot::types::StringIndex>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.LayoutTreeSnapshot/properties/stackingContexts。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Stacking context information.</pre>
                    ///
                    ///
                    #[serde(rename = "stackingContexts")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub stacking_contexts: crate::dom_snapshot::types::RareBooleanData,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.LayoutTreeSnapshot/properties/paintOrders。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Global paint order index, which is determined by the stacking order of the nodes. Nodes
                    ///that are painted together will have the same index. Only provided if includePaintOrder in
                    ///captureSnapshot was true.</pre>
                    ///
                    ///
                    #[serde(rename = "paintOrders")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub paint_orders: Option<Vec<i64>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.LayoutTreeSnapshot/properties/offsetRects。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The offset rect of nodes. Only available when includeDOMRects is set to true</pre>
                    ///
                    ///
                    #[serde(rename = "offsetRects")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub offset_rects: Option<Vec<crate::dom_snapshot::types::Rectangle>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.LayoutTreeSnapshot/properties/scrollRects。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The scroll rect of nodes. Only available when includeDOMRects is set to true</pre>
                    ///
                    ///
                    #[serde(rename = "scrollRects")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub scroll_rects: Option<Vec<crate::dom_snapshot::types::Rectangle>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.LayoutTreeSnapshot/properties/clientRects。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The client rect of nodes. Only available when includeDOMRects is set to true</pre>
                    ///
                    ///
                    #[serde(rename = "clientRects")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub client_rects: Option<Vec<crate::dom_snapshot::types::Rectangle>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.LayoutTreeSnapshot/properties/blendedBackgroundColors。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The list of background colors that are blended with colors of overlapping elements.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "blendedBackgroundColors")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub blended_background_colors:
                        Option<Vec<crate::dom_snapshot::types::StringIndex>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.LayoutTreeSnapshot/properties/textColorOpacities。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The list of computed text opacities.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "textColorOpacities")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub text_color_opacities: Option<Vec<ydp_core::FiniteF64>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LayoutTreeSnapshot {
                    node_index: fields.node_index,
                    styles: fields.styles,
                    bounds: fields.bounds,
                    text: fields.text,
                    stacking_contexts: fields.stacking_contexts,
                    paint_orders: fields.paint_orders,
                    offset_rects: fields.offset_rects,
                    scroll_rects: fields.scroll_rects,
                    client_rects: fields.client_rects,
                    blended_background_colors: fields.blended_background_colors,
                    text_color_opacities: fields.text_color_opacities,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMSnapshot.type.NameValue。
///
///协议原文：
///
///<pre>A name/value pair.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct NameValue {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.NameValue/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Attribute/property name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.NameValue/properties/value。
    ///
    ///协议原文：
    ///
    ///<pre>Attribute/property value.</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: String,
}
impl<'de> serde::Deserialize<'de> for NameValue {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NameValue;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.NameValue/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Attribute/property name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.NameValue/properties/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Attribute/property value.</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(NameValue {
                    name: fields.name,
                    value: fields.value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMSnapshot.type.NodeTreeSnapshot。
///
///协议原文：
///
///<pre>Table containing nodes.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeTreeSnapshot {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/parentIndex。
    ///
    ///协议原文：
    ///
    ///<pre>Parent node index.</pre>
    ///
    ///
    #[serde(rename = "parentIndex")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_index: Option<Vec<i64>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/nodeType。
    ///
    ///协议原文：
    ///
    ///<pre>`Node`'s nodeType.</pre>
    ///
    ///
    #[serde(rename = "nodeType")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub node_type: Option<Vec<i64>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/shadowRootType。
    ///
    ///协议原文：
    ///
    ///<pre>Type of the shadow root the `Node` is in. String values are equal to the `ShadowRootType` enum.</pre>
    ///
    ///
    #[serde(rename = "shadowRootType")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub shadow_root_type: Option<crate::dom_snapshot::types::RareStringData>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/nodeName。
    ///
    ///协议原文：
    ///
    ///<pre>`Node`'s nodeName.</pre>
    ///
    ///
    #[serde(rename = "nodeName")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub node_name: Option<Vec<crate::dom_snapshot::types::StringIndex>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/nodeValue。
    ///
    ///协议原文：
    ///
    ///<pre>`Node`'s nodeValue.</pre>
    ///
    ///
    #[serde(rename = "nodeValue")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub node_value: Option<Vec<crate::dom_snapshot::types::StringIndex>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/backendNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>`Node`'s id, corresponds to DOM.Node.backendNodeId.</pre>
    ///
    ///
    #[serde(rename = "backendNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_node_id: Option<Vec<crate::dom::types::BackendNodeId>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/attributes。
    ///
    ///协议原文：
    ///
    ///<pre>Attributes of an `Element` node. Flatten name, value pairs.</pre>
    ///
    ///
    #[serde(rename = "attributes")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub attributes: Option<Vec<crate::dom_snapshot::types::ArrayOfStrings>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/textValue。
    ///
    ///协议原文：
    ///
    ///<pre>Only set for textarea elements, contains the text value.</pre>
    ///
    ///
    #[serde(rename = "textValue")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub text_value: Option<crate::dom_snapshot::types::RareStringData>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/inputValue。
    ///
    ///协议原文：
    ///
    ///<pre>Only set for input elements, contains the input's associated text value.</pre>
    ///
    ///
    #[serde(rename = "inputValue")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub input_value: Option<crate::dom_snapshot::types::RareStringData>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/inputChecked。
    ///
    ///协议原文：
    ///
    ///<pre>Only set for radio and checkbox input elements, indicates if the element has been checked</pre>
    ///
    ///
    #[serde(rename = "inputChecked")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub input_checked: Option<crate::dom_snapshot::types::RareBooleanData>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/optionSelected。
    ///
    ///协议原文：
    ///
    ///<pre>Only set for option elements, indicates if the element has been selected</pre>
    ///
    ///
    #[serde(rename = "optionSelected")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub option_selected: Option<crate::dom_snapshot::types::RareBooleanData>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/contentDocumentIndex。
    ///
    ///协议原文：
    ///
    ///<pre>The index of the document in the list of the snapshot documents.</pre>
    ///
    ///
    #[serde(rename = "contentDocumentIndex")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub content_document_index: Option<crate::dom_snapshot::types::RareIntegerData>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/pseudoType。
    ///
    ///协议原文：
    ///
    ///<pre>Type of a pseudo element node.</pre>
    ///
    ///
    #[serde(rename = "pseudoType")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub pseudo_type: Option<crate::dom_snapshot::types::RareStringData>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/pseudoIdentifier。
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
    pub pseudo_identifier: Option<crate::dom_snapshot::types::RareStringData>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/isClickable。
    ///
    ///协议原文：
    ///
    ///<pre>Whether this DOM node responds to mouse clicks. This includes nodes that have had click
    ///event listeners attached via JavaScript as well as anchor tags that naturally navigate when
    ///clicked.</pre>
    ///
    ///
    #[serde(rename = "isClickable")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_clickable: Option<crate::dom_snapshot::types::RareBooleanData>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/currentSourceURL。
    ///
    ///协议原文：
    ///
    ///<pre>The selected url for nodes with a srcset attribute.</pre>
    ///
    ///
    #[serde(rename = "currentSourceURL")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub current_source_url: Option<crate::dom_snapshot::types::RareStringData>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/originURL。
    ///
    ///协议原文：
    ///
    ///<pre>The url of the script (if any) that generates this node.</pre>
    ///
    ///
    #[serde(rename = "originURL")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub origin_url: Option<crate::dom_snapshot::types::RareStringData>,
}
impl<'de> serde::Deserialize<'de> for NodeTreeSnapshot {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NodeTreeSnapshot;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/parentIndex。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Parent node index.</pre>
                    ///
                    ///
                    #[serde(rename = "parentIndex")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub parent_index: Option<Vec<i64>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/nodeType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`Node`'s nodeType.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeType")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub node_type: Option<Vec<i64>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/shadowRootType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Type of the shadow root the `Node` is in. String values are equal to the `ShadowRootType` enum.</pre>
                    ///
                    ///
                    #[serde(rename = "shadowRootType")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub shadow_root_type: Option<crate::dom_snapshot::types::RareStringData>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/nodeName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`Node`'s nodeName.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeName")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub node_name: Option<Vec<crate::dom_snapshot::types::StringIndex>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/nodeValue。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`Node`'s nodeValue.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeValue")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub node_value: Option<Vec<crate::dom_snapshot::types::StringIndex>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/backendNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`Node`'s id, corresponds to DOM.Node.backendNodeId.</pre>
                    ///
                    ///
                    #[serde(rename = "backendNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub backend_node_id: Option<Vec<crate::dom::types::BackendNodeId>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/attributes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Attributes of an `Element` node. Flatten name, value pairs.</pre>
                    ///
                    ///
                    #[serde(rename = "attributes")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub attributes: Option<Vec<crate::dom_snapshot::types::ArrayOfStrings>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/textValue。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Only set for textarea elements, contains the text value.</pre>
                    ///
                    ///
                    #[serde(rename = "textValue")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub text_value: Option<crate::dom_snapshot::types::RareStringData>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/inputValue。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Only set for input elements, contains the input's associated text value.</pre>
                    ///
                    ///
                    #[serde(rename = "inputValue")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub input_value: Option<crate::dom_snapshot::types::RareStringData>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/inputChecked。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Only set for radio and checkbox input elements, indicates if the element has been checked</pre>
                    ///
                    ///
                    #[serde(rename = "inputChecked")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub input_checked: Option<crate::dom_snapshot::types::RareBooleanData>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/optionSelected。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Only set for option elements, indicates if the element has been selected</pre>
                    ///
                    ///
                    #[serde(rename = "optionSelected")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub option_selected: Option<crate::dom_snapshot::types::RareBooleanData>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/contentDocumentIndex。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The index of the document in the list of the snapshot documents.</pre>
                    ///
                    ///
                    #[serde(rename = "contentDocumentIndex")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub content_document_index: Option<crate::dom_snapshot::types::RareIntegerData>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/pseudoType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Type of a pseudo element node.</pre>
                    ///
                    ///
                    #[serde(rename = "pseudoType")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub pseudo_type: Option<crate::dom_snapshot::types::RareStringData>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/pseudoIdentifier。
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
                    pub pseudo_identifier: Option<crate::dom_snapshot::types::RareStringData>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/isClickable。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether this DOM node responds to mouse clicks. This includes nodes that have had click
                    ///event listeners attached via JavaScript as well as anchor tags that naturally navigate when
                    ///clicked.</pre>
                    ///
                    ///
                    #[serde(rename = "isClickable")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub is_clickable: Option<crate::dom_snapshot::types::RareBooleanData>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/currentSourceURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The selected url for nodes with a srcset attribute.</pre>
                    ///
                    ///
                    #[serde(rename = "currentSourceURL")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub current_source_url: Option<crate::dom_snapshot::types::RareStringData>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.type.NodeTreeSnapshot/properties/originURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The url of the script (if any) that generates this node.</pre>
                    ///
                    ///
                    #[serde(rename = "originURL")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub origin_url: Option<crate::dom_snapshot::types::RareStringData>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(NodeTreeSnapshot {
                    parent_index: fields.parent_index,
                    node_type: fields.node_type,
                    shadow_root_type: fields.shadow_root_type,
                    node_name: fields.node_name,
                    node_value: fields.node_value,
                    backend_node_id: fields.backend_node_id,
                    attributes: fields.attributes,
                    text_value: fields.text_value,
                    input_value: fields.input_value,
                    input_checked: fields.input_checked,
                    option_selected: fields.option_selected,
                    content_document_index: fields.content_document_index,
                    pseudo_type: fields.pseudo_type,
                    pseudo_identifier: fields.pseudo_identifier,
                    is_clickable: fields.is_clickable,
                    current_source_url: fields.current_source_url,
                    origin_url: fields.origin_url,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMSnapshot.type.RareBooleanData。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RareBooleanData {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.RareBooleanData/properties/index。
    #[serde(rename = "index")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub index: Vec<i64>,
}
impl<'de> serde::Deserialize<'de> for RareBooleanData {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RareBooleanData;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.RareBooleanData/properties/index。
                    #[serde(rename = "index")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub index: Vec<i64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RareBooleanData {
                    index: fields.index,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMSnapshot.type.RareIntegerData。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RareIntegerData {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.RareIntegerData/properties/index。
    #[serde(rename = "index")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub index: Vec<i64>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.RareIntegerData/properties/value。
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: Vec<i64>,
}
impl<'de> serde::Deserialize<'de> for RareIntegerData {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RareIntegerData;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.RareIntegerData/properties/index。
                    #[serde(rename = "index")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub index: Vec<i64>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.RareIntegerData/properties/value。
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: Vec<i64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RareIntegerData {
                    index: fields.index,
                    value: fields.value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMSnapshot.type.RareStringData。
///
///协议原文：
///
///<pre>Data that is only present on rare nodes.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct RareStringData {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.RareStringData/properties/index。
    #[serde(rename = "index")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub index: Vec<i64>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.RareStringData/properties/value。
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: Vec<crate::dom_snapshot::types::StringIndex>,
}
impl<'de> serde::Deserialize<'de> for RareStringData {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RareStringData;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.RareStringData/properties/index。
                    #[serde(rename = "index")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub index: Vec<i64>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.RareStringData/properties/value。
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: Vec<crate::dom_snapshot::types::StringIndex>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RareStringData {
                    index: fields.index,
                    value: fields.value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：DOMSnapshot.type.Rectangle。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Rectangle(Vec<ydp_core::FiniteF64>);
impl Rectangle {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::dom_snapshot::types::Rectangle = serde_json::from_str("[]").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::dom_snapshot::types::Rectangle::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("[]").unwrap());
    ///```
    pub fn new(value: Vec<ydp_core::FiniteF64>) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::dom_snapshot::types::Rectangle = serde_json::from_str("[]").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::dom_snapshot::types::Rectangle::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("[]").unwrap());
    ///```
    pub fn into_inner(self) -> Vec<ydp_core::FiniteF64> {
        self.0
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：DOMSnapshot.type.StringIndex。
///
///协议原文：
///
///<pre>Index of the string in the strings table.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct StringIndex(i64);
impl StringIndex {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::dom_snapshot::types::StringIndex = serde_json::from_str("0").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::dom_snapshot::types::StringIndex::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("0").unwrap());
    ///```
    pub fn new(value: i64) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::dom_snapshot::types::StringIndex = serde_json::from_str("0").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::dom_snapshot::types::StringIndex::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("0").unwrap());
    ///```
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMSnapshot.type.TextBoxSnapshot。
///
///协议原文：
///
///<pre>Table of details of the post layout rendered text positions. The exact layout should not be regarded as
///stable and may change between versions.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct TextBoxSnapshot {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.TextBoxSnapshot/properties/layoutIndex。
    ///
    ///协议原文：
    ///
    ///<pre>Index of the layout tree node that owns this box collection.</pre>
    ///
    ///
    #[serde(rename = "layoutIndex")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub layout_index: Vec<i64>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.TextBoxSnapshot/properties/bounds。
    ///
    ///协议原文：
    ///
    ///<pre>The absolute position bounding box.</pre>
    ///
    ///
    #[serde(rename = "bounds")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub bounds: Vec<crate::dom_snapshot::types::Rectangle>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.TextBoxSnapshot/properties/start。
    ///
    ///协议原文：
    ///
    ///<pre>The starting index in characters, for this post layout textbox substring. Characters that
    ///would be represented as a surrogate pair in UTF-16 have length 2.</pre>
    ///
    ///
    #[serde(rename = "start")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub start: Vec<i64>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.TextBoxSnapshot/properties/length。
    ///
    ///协议原文：
    ///
    ///<pre>The number of characters in this post layout textbox substring. Characters that would be
    ///represented as a surrogate pair in UTF-16 have length 2.</pre>
    ///
    ///
    #[serde(rename = "length")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub length: Vec<i64>,
}
impl<'de> serde::Deserialize<'de> for TextBoxSnapshot {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TextBoxSnapshot;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.TextBoxSnapshot/properties/layoutIndex。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Index of the layout tree node that owns this box collection.</pre>
                    ///
                    ///
                    #[serde(rename = "layoutIndex")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub layout_index: Vec<i64>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.TextBoxSnapshot/properties/bounds。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The absolute position bounding box.</pre>
                    ///
                    ///
                    #[serde(rename = "bounds")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub bounds: Vec<crate::dom_snapshot::types::Rectangle>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.TextBoxSnapshot/properties/start。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The starting index in characters, for this post layout textbox substring. Characters that
                    ///would be represented as a surrogate pair in UTF-16 have length 2.</pre>
                    ///
                    ///
                    #[serde(rename = "start")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub start: Vec<i64>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.type.TextBoxSnapshot/properties/length。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The number of characters in this post layout textbox substring. Characters that would be
                    ///represented as a surrogate pair in UTF-16 have length 2.</pre>
                    ///
                    ///
                    #[serde(rename = "length")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub length: Vec<i64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TextBoxSnapshot {
                    layout_index: fields.layout_index,
                    bounds: fields.bounds,
                    start: fields.start,
                    length: fields.length,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
