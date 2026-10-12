///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Accessibility.type.AXNode。
///
///协议原文：
///
///<pre>A node in the accessibility tree.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct AxNode {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.type.AXNode/properties/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Unique identifier for this node.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::accessibility::types::AxNodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.type.AXNode/properties/ignored。
    ///
    ///协议原文：
    ///
    ///<pre>Whether this node is ignored for accessibility</pre>
    ///
    ///
    #[serde(rename = "ignored")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub ignored: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXNode/properties/ignoredReasons。
    ///
    ///协议原文：
    ///
    ///<pre>Collection of reasons why this node is hidden.</pre>
    ///
    ///
    #[serde(rename = "ignoredReasons")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub ignored_reasons: Option<Vec<crate::accessibility::types::AxProperty>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXNode/properties/role。
    ///
    ///协议原文：
    ///
    ///<pre>This `Node`'s role, whether explicit or implicit.</pre>
    ///
    ///
    #[serde(rename = "role")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub role: Option<crate::accessibility::types::AxValue>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXNode/properties/chromeRole。
    ///
    ///协议原文：
    ///
    ///<pre>This `Node`'s Chrome raw role.</pre>
    ///
    ///
    #[serde(rename = "chromeRole")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub chrome_role: Option<crate::accessibility::types::AxValue>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXNode/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>The accessible name for this `Node`.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<crate::accessibility::types::AxValue>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXNode/properties/description。
    ///
    ///协议原文：
    ///
    ///<pre>The accessible description for this `Node`.</pre>
    ///
    ///
    #[serde(rename = "description")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<crate::accessibility::types::AxValue>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXNode/properties/value。
    ///
    ///协议原文：
    ///
    ///<pre>The value for this `Node`.</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<crate::accessibility::types::AxValue>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXNode/properties/properties。
    ///
    ///协议原文：
    ///
    ///<pre>All other properties</pre>
    ///
    ///
    #[serde(rename = "properties")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub properties: Option<Vec<crate::accessibility::types::AxProperty>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXNode/properties/parentId。
    ///
    ///协议原文：
    ///
    ///<pre>ID for this node's parent.</pre>
    ///
    ///
    #[serde(rename = "parentId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_id: Option<crate::accessibility::types::AxNodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXNode/properties/childIds。
    ///
    ///协议原文：
    ///
    ///<pre>IDs for each of this node's child nodes.</pre>
    ///
    ///
    #[serde(rename = "childIds")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub child_ids: Option<Vec<crate::accessibility::types::AxNodeId>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXNode/properties/backendDOMNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>The backend ID for the associated DOM node, if any.</pre>
    ///
    ///
    #[serde(rename = "backendDOMNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_dom_node_id: Option<crate::dom::types::BackendNodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXNode/properties/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>The frame ID for the frame associated with this nodes document.</pre>
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
impl<'de> serde::Deserialize<'de> for AxNode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AxNode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.type.AXNode/properties/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Unique identifier for this node.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::accessibility::types::AxNodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.type.AXNode/properties/ignored。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether this node is ignored for accessibility</pre>
                    ///
                    ///
                    #[serde(rename = "ignored")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub ignored: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXNode/properties/ignoredReasons。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Collection of reasons why this node is hidden.</pre>
                    ///
                    ///
                    #[serde(rename = "ignoredReasons")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub ignored_reasons: Option<Vec<crate::accessibility::types::AxProperty>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXNode/properties/role。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>This `Node`'s role, whether explicit or implicit.</pre>
                    ///
                    ///
                    #[serde(rename = "role")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub role: Option<crate::accessibility::types::AxValue>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXNode/properties/chromeRole。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>This `Node`'s Chrome raw role.</pre>
                    ///
                    ///
                    #[serde(rename = "chromeRole")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub chrome_role: Option<crate::accessibility::types::AxValue>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXNode/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The accessible name for this `Node`.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub name: Option<crate::accessibility::types::AxValue>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXNode/properties/description。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The accessible description for this `Node`.</pre>
                    ///
                    ///
                    #[serde(rename = "description")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub description: Option<crate::accessibility::types::AxValue>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXNode/properties/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The value for this `Node`.</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub value: Option<crate::accessibility::types::AxValue>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXNode/properties/properties。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>All other properties</pre>
                    ///
                    ///
                    #[serde(rename = "properties")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub properties: Option<Vec<crate::accessibility::types::AxProperty>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXNode/properties/parentId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>ID for this node's parent.</pre>
                    ///
                    ///
                    #[serde(rename = "parentId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub parent_id: Option<crate::accessibility::types::AxNodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXNode/properties/childIds。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>IDs for each of this node's child nodes.</pre>
                    ///
                    ///
                    #[serde(rename = "childIds")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub child_ids: Option<Vec<crate::accessibility::types::AxNodeId>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXNode/properties/backendDOMNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The backend ID for the associated DOM node, if any.</pre>
                    ///
                    ///
                    #[serde(rename = "backendDOMNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub backend_dom_node_id: Option<crate::dom::types::BackendNodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXNode/properties/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The frame ID for the frame associated with this nodes document.</pre>
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
                Ok(AxNode {
                    node_id: fields.node_id,
                    ignored: fields.ignored,
                    ignored_reasons: fields.ignored_reasons,
                    role: fields.role,
                    chrome_role: fields.chrome_role,
                    name: fields.name,
                    description: fields.description,
                    value: fields.value,
                    properties: fields.properties,
                    parent_id: fields.parent_id,
                    child_ids: fields.child_ids,
                    backend_dom_node_id: fields.backend_dom_node_id,
                    frame_id: fields.frame_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Accessibility.type.AXNodeId。
///
///协议原文：
///
///<pre>Unique accessibility node identifier.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct AxNodeId(String);
impl AxNodeId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::accessibility::types::AxNodeId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::accessibility::types::AxNodeId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::accessibility::types::AxNodeId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::accessibility::types::AxNodeId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::accessibility::types::AxNodeId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Accessibility.type.AXProperty。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AxProperty {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.type.AXProperty/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>The name of this property.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: crate::accessibility::types::AxPropertyName,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.type.AXProperty/properties/value。
    ///
    ///协议原文：
    ///
    ///<pre>The value of this property.</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: crate::accessibility::types::AxValue,
}
impl<'de> serde::Deserialize<'de> for AxProperty {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AxProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.type.AXProperty/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The name of this property.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: crate::accessibility::types::AxPropertyName,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.type.AXProperty/properties/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The value of this property.</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: crate::accessibility::types::AxValue,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AxProperty {
                    name: fields.name,
                    value: fields.value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Accessibility.type.AXPropertyName。
///
///协议原文：
///
///<pre>Values of AXProperty name:
///- from 'busy' to 'roledescription': states which apply to every AX node
///- from 'live' to 'root': attributes which apply to nodes in live regions
///- from 'autocomplete' to 'valuetext': attributes which apply to widgets
///- from 'checked' to 'selected': states which apply to widgets
///- from 'activedescendant' to 'owns': relationships between elements other than parent/child/sibling
///- from 'activeFullscreenElement' to 'uninteresting': reasons why this noode is hidden</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AxPropertyName {
    ///协议值 "actions"。
    Actions,
    ///协议值 "busy"。
    Busy,
    ///协议值 "disabled"。
    Disabled,
    ///协议值 "editable"。
    Editable,
    ///协议值 "focusable"。
    Focusable,
    ///协议值 "focused"。
    Focused,
    ///协议值 "hidden"。
    Hidden,
    ///协议值 "hiddenRoot"。
    HiddenRoot,
    ///协议值 "invalid"。
    Invalid,
    ///协议值 "keyshortcuts"。
    Keyshortcuts,
    ///协议值 "settable"。
    Settable,
    ///协议值 "roledescription"。
    Roledescription,
    ///协议值 "live"。
    Live,
    ///协议值 "atomic"。
    Atomic,
    ///协议值 "relevant"。
    Relevant,
    ///协议值 "root"。
    Root,
    ///协议值 "autocomplete"。
    Autocomplete,
    ///协议值 "hasPopup"。
    HasPopup,
    ///协议值 "level"。
    Level,
    ///协议值 "multiselectable"。
    Multiselectable,
    ///协议值 "orientation"。
    Orientation,
    ///协议值 "multiline"。
    Multiline,
    ///协议值 "readonly"。
    Readonly,
    ///协议值 "required"。
    Required,
    ///协议值 "valuemin"。
    Valuemin,
    ///协议值 "valuemax"。
    Valuemax,
    ///协议值 "valuetext"。
    Valuetext,
    ///协议值 "checked"。
    Checked,
    ///协议值 "expanded"。
    Expanded,
    ///协议值 "modal"。
    Modal,
    ///协议值 "pressed"。
    Pressed,
    ///协议值 "selected"。
    Selected,
    ///协议值 "activedescendant"。
    Activedescendant,
    ///协议值 "controls"。
    Controls,
    ///协议值 "describedby"。
    Describedby,
    ///协议值 "details"。
    Details,
    ///协议值 "errormessage"。
    Errormessage,
    ///协议值 "flowto"。
    Flowto,
    ///协议值 "labelledby"。
    Labelledby,
    ///协议值 "owns"。
    Owns,
    ///协议值 "url"。
    Url,
    ///协议值 "activeFullscreenElement"。
    ActiveFullscreenElement,
    ///协议值 "activeModalDialog"。
    ActiveModalDialog,
    ///协议值 "activeAriaModalDialog"。
    ActiveAriaModalDialog,
    ///协议值 "ariaHiddenElement"。
    AriaHiddenElement,
    ///协议值 "ariaHiddenSubtree"。
    AriaHiddenSubtree,
    ///协议值 "emptyAlt"。
    EmptyAlt,
    ///协议值 "emptyText"。
    EmptyText,
    ///协议值 "inertElement"。
    InertElement,
    ///协议值 "inertSubtree"。
    InertSubtree,
    ///协议值 "labelContainer"。
    LabelContainer,
    ///协议值 "labelFor"。
    LabelFor,
    ///协议值 "notRendered"。
    NotRendered,
    ///协议值 "notVisible"。
    NotVisible,
    ///协议值 "presentationalRole"。
    PresentationalRole,
    ///协议值 "probablyPresentational"。
    ProbablyPresentational,
    ///协议值 "inactiveCarouselTabContent"。
    InactiveCarouselTabContent,
    ///协议值 "uninteresting"。
    Uninteresting,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl AxPropertyName {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::accessibility::types::AxPropertyName::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Actions => "actions",
            Self::Busy => "busy",
            Self::Disabled => "disabled",
            Self::Editable => "editable",
            Self::Focusable => "focusable",
            Self::Focused => "focused",
            Self::Hidden => "hidden",
            Self::HiddenRoot => "hiddenRoot",
            Self::Invalid => "invalid",
            Self::Keyshortcuts => "keyshortcuts",
            Self::Settable => "settable",
            Self::Roledescription => "roledescription",
            Self::Live => "live",
            Self::Atomic => "atomic",
            Self::Relevant => "relevant",
            Self::Root => "root",
            Self::Autocomplete => "autocomplete",
            Self::HasPopup => "hasPopup",
            Self::Level => "level",
            Self::Multiselectable => "multiselectable",
            Self::Orientation => "orientation",
            Self::Multiline => "multiline",
            Self::Readonly => "readonly",
            Self::Required => "required",
            Self::Valuemin => "valuemin",
            Self::Valuemax => "valuemax",
            Self::Valuetext => "valuetext",
            Self::Checked => "checked",
            Self::Expanded => "expanded",
            Self::Modal => "modal",
            Self::Pressed => "pressed",
            Self::Selected => "selected",
            Self::Activedescendant => "activedescendant",
            Self::Controls => "controls",
            Self::Describedby => "describedby",
            Self::Details => "details",
            Self::Errormessage => "errormessage",
            Self::Flowto => "flowto",
            Self::Labelledby => "labelledby",
            Self::Owns => "owns",
            Self::Url => "url",
            Self::ActiveFullscreenElement => "activeFullscreenElement",
            Self::ActiveModalDialog => "activeModalDialog",
            Self::ActiveAriaModalDialog => "activeAriaModalDialog",
            Self::AriaHiddenElement => "ariaHiddenElement",
            Self::AriaHiddenSubtree => "ariaHiddenSubtree",
            Self::EmptyAlt => "emptyAlt",
            Self::EmptyText => "emptyText",
            Self::InertElement => "inertElement",
            Self::InertSubtree => "inertSubtree",
            Self::LabelContainer => "labelContainer",
            Self::LabelFor => "labelFor",
            Self::NotRendered => "notRendered",
            Self::NotVisible => "notVisible",
            Self::PresentationalRole => "presentationalRole",
            Self::ProbablyPresentational => "probablyPresentational",
            Self::InactiveCarouselTabContent => "inactiveCarouselTabContent",
            Self::Uninteresting => "uninteresting",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for AxPropertyName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for AxPropertyName {
    fn from(value: String) -> Self {
        match value.as_str() {
            "actions" => AxPropertyName::Actions,
            "busy" => AxPropertyName::Busy,
            "disabled" => AxPropertyName::Disabled,
            "editable" => AxPropertyName::Editable,
            "focusable" => AxPropertyName::Focusable,
            "focused" => AxPropertyName::Focused,
            "hidden" => AxPropertyName::Hidden,
            "hiddenRoot" => AxPropertyName::HiddenRoot,
            "invalid" => AxPropertyName::Invalid,
            "keyshortcuts" => AxPropertyName::Keyshortcuts,
            "settable" => AxPropertyName::Settable,
            "roledescription" => AxPropertyName::Roledescription,
            "live" => AxPropertyName::Live,
            "atomic" => AxPropertyName::Atomic,
            "relevant" => AxPropertyName::Relevant,
            "root" => AxPropertyName::Root,
            "autocomplete" => AxPropertyName::Autocomplete,
            "hasPopup" => AxPropertyName::HasPopup,
            "level" => AxPropertyName::Level,
            "multiselectable" => AxPropertyName::Multiselectable,
            "orientation" => AxPropertyName::Orientation,
            "multiline" => AxPropertyName::Multiline,
            "readonly" => AxPropertyName::Readonly,
            "required" => AxPropertyName::Required,
            "valuemin" => AxPropertyName::Valuemin,
            "valuemax" => AxPropertyName::Valuemax,
            "valuetext" => AxPropertyName::Valuetext,
            "checked" => AxPropertyName::Checked,
            "expanded" => AxPropertyName::Expanded,
            "modal" => AxPropertyName::Modal,
            "pressed" => AxPropertyName::Pressed,
            "selected" => AxPropertyName::Selected,
            "activedescendant" => AxPropertyName::Activedescendant,
            "controls" => AxPropertyName::Controls,
            "describedby" => AxPropertyName::Describedby,
            "details" => AxPropertyName::Details,
            "errormessage" => AxPropertyName::Errormessage,
            "flowto" => AxPropertyName::Flowto,
            "labelledby" => AxPropertyName::Labelledby,
            "owns" => AxPropertyName::Owns,
            "url" => AxPropertyName::Url,
            "activeFullscreenElement" => AxPropertyName::ActiveFullscreenElement,
            "activeModalDialog" => AxPropertyName::ActiveModalDialog,
            "activeAriaModalDialog" => AxPropertyName::ActiveAriaModalDialog,
            "ariaHiddenElement" => AxPropertyName::AriaHiddenElement,
            "ariaHiddenSubtree" => AxPropertyName::AriaHiddenSubtree,
            "emptyAlt" => AxPropertyName::EmptyAlt,
            "emptyText" => AxPropertyName::EmptyText,
            "inertElement" => AxPropertyName::InertElement,
            "inertSubtree" => AxPropertyName::InertSubtree,
            "labelContainer" => AxPropertyName::LabelContainer,
            "labelFor" => AxPropertyName::LabelFor,
            "notRendered" => AxPropertyName::NotRendered,
            "notVisible" => AxPropertyName::NotVisible,
            "presentationalRole" => AxPropertyName::PresentationalRole,
            "probablyPresentational" => AxPropertyName::ProbablyPresentational,
            "inactiveCarouselTabContent" => AxPropertyName::InactiveCarouselTabContent,
            "uninteresting" => AxPropertyName::Uninteresting,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for AxPropertyName {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AxPropertyName {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AxPropertyName;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "actions" => AxPropertyName::Actions,
                    "busy" => AxPropertyName::Busy,
                    "disabled" => AxPropertyName::Disabled,
                    "editable" => AxPropertyName::Editable,
                    "focusable" => AxPropertyName::Focusable,
                    "focused" => AxPropertyName::Focused,
                    "hidden" => AxPropertyName::Hidden,
                    "hiddenRoot" => AxPropertyName::HiddenRoot,
                    "invalid" => AxPropertyName::Invalid,
                    "keyshortcuts" => AxPropertyName::Keyshortcuts,
                    "settable" => AxPropertyName::Settable,
                    "roledescription" => AxPropertyName::Roledescription,
                    "live" => AxPropertyName::Live,
                    "atomic" => AxPropertyName::Atomic,
                    "relevant" => AxPropertyName::Relevant,
                    "root" => AxPropertyName::Root,
                    "autocomplete" => AxPropertyName::Autocomplete,
                    "hasPopup" => AxPropertyName::HasPopup,
                    "level" => AxPropertyName::Level,
                    "multiselectable" => AxPropertyName::Multiselectable,
                    "orientation" => AxPropertyName::Orientation,
                    "multiline" => AxPropertyName::Multiline,
                    "readonly" => AxPropertyName::Readonly,
                    "required" => AxPropertyName::Required,
                    "valuemin" => AxPropertyName::Valuemin,
                    "valuemax" => AxPropertyName::Valuemax,
                    "valuetext" => AxPropertyName::Valuetext,
                    "checked" => AxPropertyName::Checked,
                    "expanded" => AxPropertyName::Expanded,
                    "modal" => AxPropertyName::Modal,
                    "pressed" => AxPropertyName::Pressed,
                    "selected" => AxPropertyName::Selected,
                    "activedescendant" => AxPropertyName::Activedescendant,
                    "controls" => AxPropertyName::Controls,
                    "describedby" => AxPropertyName::Describedby,
                    "details" => AxPropertyName::Details,
                    "errormessage" => AxPropertyName::Errormessage,
                    "flowto" => AxPropertyName::Flowto,
                    "labelledby" => AxPropertyName::Labelledby,
                    "owns" => AxPropertyName::Owns,
                    "url" => AxPropertyName::Url,
                    "activeFullscreenElement" => AxPropertyName::ActiveFullscreenElement,
                    "activeModalDialog" => AxPropertyName::ActiveModalDialog,
                    "activeAriaModalDialog" => AxPropertyName::ActiveAriaModalDialog,
                    "ariaHiddenElement" => AxPropertyName::AriaHiddenElement,
                    "ariaHiddenSubtree" => AxPropertyName::AriaHiddenSubtree,
                    "emptyAlt" => AxPropertyName::EmptyAlt,
                    "emptyText" => AxPropertyName::EmptyText,
                    "inertElement" => AxPropertyName::InertElement,
                    "inertSubtree" => AxPropertyName::InertSubtree,
                    "labelContainer" => AxPropertyName::LabelContainer,
                    "labelFor" => AxPropertyName::LabelFor,
                    "notRendered" => AxPropertyName::NotRendered,
                    "notVisible" => AxPropertyName::NotVisible,
                    "presentationalRole" => AxPropertyName::PresentationalRole,
                    "probablyPresentational" => AxPropertyName::ProbablyPresentational,
                    "inactiveCarouselTabContent" => AxPropertyName::InactiveCarouselTabContent,
                    "uninteresting" => AxPropertyName::Uninteresting,
                    _ => AxPropertyName::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(AxPropertyName::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Accessibility.type.AXRelatedNode。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AxRelatedNode {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.type.AXRelatedNode/properties/backendDOMNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>The BackendNodeId of the related DOM node.</pre>
    ///
    ///
    #[serde(rename = "backendDOMNodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub backend_dom_node_id: crate::dom::types::BackendNodeId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXRelatedNode/properties/idref。
    ///
    ///协议原文：
    ///
    ///<pre>The IDRef value provided, if any.</pre>
    ///
    ///
    #[serde(rename = "idref")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub idref: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXRelatedNode/properties/text。
    ///
    ///协议原文：
    ///
    ///<pre>The text alternative of this node in the current context.</pre>
    ///
    ///
    #[serde(rename = "text")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub text: Option<String>,
}
impl<'de> serde::Deserialize<'de> for AxRelatedNode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AxRelatedNode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.type.AXRelatedNode/properties/backendDOMNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The BackendNodeId of the related DOM node.</pre>
                    ///
                    ///
                    #[serde(rename = "backendDOMNodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub backend_dom_node_id: crate::dom::types::BackendNodeId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXRelatedNode/properties/idref。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The IDRef value provided, if any.</pre>
                    ///
                    ///
                    #[serde(rename = "idref")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub idref: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXRelatedNode/properties/text。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The text alternative of this node in the current context.</pre>
                    ///
                    ///
                    #[serde(rename = "text")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub text: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AxRelatedNode {
                    backend_dom_node_id: fields.backend_dom_node_id,
                    idref: fields.idref,
                    text: fields.text,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Accessibility.type.AXValue。
///
///协议原文：
///
///<pre>A single computed AX property.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct AxValue {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.type.AXValue/properties/type。
    ///
    ///协议原文：
    ///
    ///<pre>The type of this value.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: crate::accessibility::types::AxValueType,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXValue/properties/value。
    ///
    ///协议原文：
    ///
    ///<pre>The computed value of this property.</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<serde_json::Value>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXValue/properties/relatedNodes。
    ///
    ///协议原文：
    ///
    ///<pre>One or more related nodes, if applicable.</pre>
    ///
    ///
    #[serde(rename = "relatedNodes")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub related_nodes: Option<Vec<crate::accessibility::types::AxRelatedNode>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXValue/properties/sources。
    ///
    ///协议原文：
    ///
    ///<pre>The sources which contributed to the computation of this property.</pre>
    ///
    ///
    #[serde(rename = "sources")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub sources: Option<Vec<crate::accessibility::types::AxValueSource>>,
}
impl<'de> serde::Deserialize<'de> for AxValue {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AxValue;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.type.AXValue/properties/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The type of this value.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: crate::accessibility::types::AxValueType,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXValue/properties/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The computed value of this property.</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub value: Option<serde_json::Value>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXValue/properties/relatedNodes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>One or more related nodes, if applicable.</pre>
                    ///
                    ///
                    #[serde(rename = "relatedNodes")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub related_nodes: Option<Vec<crate::accessibility::types::AxRelatedNode>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXValue/properties/sources。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The sources which contributed to the computation of this property.</pre>
                    ///
                    ///
                    #[serde(rename = "sources")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub sources: Option<Vec<crate::accessibility::types::AxValueSource>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AxValue {
                    r#type: fields.r#type,
                    value: fields.value,
                    related_nodes: fields.related_nodes,
                    sources: fields.sources,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Accessibility.type.AXValueNativeSourceType。
///
///协议原文：
///
///<pre>Enum of possible native property sources (as a subtype of a particular AXValueSourceType).</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AxValueNativeSourceType {
    ///协议值 "description"。
    Description,
    ///协议值 "figcaption"。
    Figcaption,
    ///协议值 "label"。
    Label,
    ///协议值 "labelfor"。
    Labelfor,
    ///协议值 "labelwrapped"。
    Labelwrapped,
    ///协议值 "legend"。
    Legend,
    ///协议值 "rubyannotation"。
    Rubyannotation,
    ///协议值 "tablecaption"。
    Tablecaption,
    ///协议值 "title"。
    Title,
    ///协议值 "other"。
    Other,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl AxValueNativeSourceType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::accessibility::types::AxValueNativeSourceType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Description => "description",
            Self::Figcaption => "figcaption",
            Self::Label => "label",
            Self::Labelfor => "labelfor",
            Self::Labelwrapped => "labelwrapped",
            Self::Legend => "legend",
            Self::Rubyannotation => "rubyannotation",
            Self::Tablecaption => "tablecaption",
            Self::Title => "title",
            Self::Other => "other",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for AxValueNativeSourceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for AxValueNativeSourceType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "description" => AxValueNativeSourceType::Description,
            "figcaption" => AxValueNativeSourceType::Figcaption,
            "label" => AxValueNativeSourceType::Label,
            "labelfor" => AxValueNativeSourceType::Labelfor,
            "labelwrapped" => AxValueNativeSourceType::Labelwrapped,
            "legend" => AxValueNativeSourceType::Legend,
            "rubyannotation" => AxValueNativeSourceType::Rubyannotation,
            "tablecaption" => AxValueNativeSourceType::Tablecaption,
            "title" => AxValueNativeSourceType::Title,
            "other" => AxValueNativeSourceType::Other,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for AxValueNativeSourceType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AxValueNativeSourceType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AxValueNativeSourceType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "description" => AxValueNativeSourceType::Description,
                    "figcaption" => AxValueNativeSourceType::Figcaption,
                    "label" => AxValueNativeSourceType::Label,
                    "labelfor" => AxValueNativeSourceType::Labelfor,
                    "labelwrapped" => AxValueNativeSourceType::Labelwrapped,
                    "legend" => AxValueNativeSourceType::Legend,
                    "rubyannotation" => AxValueNativeSourceType::Rubyannotation,
                    "tablecaption" => AxValueNativeSourceType::Tablecaption,
                    "title" => AxValueNativeSourceType::Title,
                    "other" => AxValueNativeSourceType::Other,
                    _ => AxValueNativeSourceType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(AxValueNativeSourceType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Accessibility.type.AXValueSource。
///
///协议原文：
///
///<pre>A single source for a computed AX property.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct AxValueSource {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.type.AXValueSource/properties/type。
    ///
    ///协议原文：
    ///
    ///<pre>What type of source this is.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: crate::accessibility::types::AxValueSourceType,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXValueSource/properties/value。
    ///
    ///协议原文：
    ///
    ///<pre>The value of this property source.</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<crate::accessibility::types::AxValue>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXValueSource/properties/attribute。
    ///
    ///协议原文：
    ///
    ///<pre>The name of the relevant attribute, if any.</pre>
    ///
    ///
    #[serde(rename = "attribute")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub attribute: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXValueSource/properties/attributeValue。
    ///
    ///协议原文：
    ///
    ///<pre>The value of the relevant attribute, if any.</pre>
    ///
    ///
    #[serde(rename = "attributeValue")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub attribute_value: Option<crate::accessibility::types::AxValue>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXValueSource/properties/superseded。
    ///
    ///协议原文：
    ///
    ///<pre>Whether this source is superseded by a higher priority source.</pre>
    ///
    ///
    #[serde(rename = "superseded")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub superseded: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXValueSource/properties/nativeSource。
    ///
    ///协议原文：
    ///
    ///<pre>The native markup source for this value, e.g. a `&lt;label&gt;` element.</pre>
    ///
    ///
    #[serde(rename = "nativeSource")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub native_source: Option<crate::accessibility::types::AxValueNativeSourceType>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXValueSource/properties/nativeSourceValue。
    ///
    ///协议原文：
    ///
    ///<pre>The value, such as a node or node list, of the native source.</pre>
    ///
    ///
    #[serde(rename = "nativeSourceValue")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub native_source_value: Option<crate::accessibility::types::AxValue>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXValueSource/properties/invalid。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the value for this property is invalid.</pre>
    ///
    ///
    #[serde(rename = "invalid")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub invalid: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXValueSource/properties/invalidReason。
    ///
    ///协议原文：
    ///
    ///<pre>Reason for the value being invalid, if it is.</pre>
    ///
    ///
    #[serde(rename = "invalidReason")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub invalid_reason: Option<String>,
}
impl<'de> serde::Deserialize<'de> for AxValueSource {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AxValueSource;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.type.AXValueSource/properties/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>What type of source this is.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: crate::accessibility::types::AxValueSourceType,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXValueSource/properties/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The value of this property source.</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub value: Option<crate::accessibility::types::AxValue>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXValueSource/properties/attribute。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The name of the relevant attribute, if any.</pre>
                    ///
                    ///
                    #[serde(rename = "attribute")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub attribute: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXValueSource/properties/attributeValue。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The value of the relevant attribute, if any.</pre>
                    ///
                    ///
                    #[serde(rename = "attributeValue")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub attribute_value: Option<crate::accessibility::types::AxValue>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXValueSource/properties/superseded。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether this source is superseded by a higher priority source.</pre>
                    ///
                    ///
                    #[serde(rename = "superseded")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub superseded: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXValueSource/properties/nativeSource。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The native markup source for this value, e.g. a `&lt;label&gt;` element.</pre>
                    ///
                    ///
                    #[serde(rename = "nativeSource")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub native_source: Option<crate::accessibility::types::AxValueNativeSourceType>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXValueSource/properties/nativeSourceValue。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The value, such as a node or node list, of the native source.</pre>
                    ///
                    ///
                    #[serde(rename = "nativeSourceValue")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub native_source_value: Option<crate::accessibility::types::AxValue>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXValueSource/properties/invalid。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the value for this property is invalid.</pre>
                    ///
                    ///
                    #[serde(rename = "invalid")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub invalid: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Accessibility.type.AXValueSource/properties/invalidReason。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Reason for the value being invalid, if it is.</pre>
                    ///
                    ///
                    #[serde(rename = "invalidReason")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub invalid_reason: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AxValueSource {
                    r#type: fields.r#type,
                    value: fields.value,
                    attribute: fields.attribute,
                    attribute_value: fields.attribute_value,
                    superseded: fields.superseded,
                    native_source: fields.native_source,
                    native_source_value: fields.native_source_value,
                    invalid: fields.invalid,
                    invalid_reason: fields.invalid_reason,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Accessibility.type.AXValueSourceType。
///
///协议原文：
///
///<pre>Enum of possible property sources.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AxValueSourceType {
    ///协议值 "attribute"。
    Attribute,
    ///协议值 "implicit"。
    Implicit,
    ///协议值 "style"。
    Style,
    ///协议值 "contents"。
    Contents,
    ///协议值 "placeholder"。
    Placeholder,
    ///协议值 "relatedElement"。
    RelatedElement,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl AxValueSourceType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::accessibility::types::AxValueSourceType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Attribute => "attribute",
            Self::Implicit => "implicit",
            Self::Style => "style",
            Self::Contents => "contents",
            Self::Placeholder => "placeholder",
            Self::RelatedElement => "relatedElement",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for AxValueSourceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for AxValueSourceType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "attribute" => AxValueSourceType::Attribute,
            "implicit" => AxValueSourceType::Implicit,
            "style" => AxValueSourceType::Style,
            "contents" => AxValueSourceType::Contents,
            "placeholder" => AxValueSourceType::Placeholder,
            "relatedElement" => AxValueSourceType::RelatedElement,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for AxValueSourceType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AxValueSourceType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AxValueSourceType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "attribute" => AxValueSourceType::Attribute,
                    "implicit" => AxValueSourceType::Implicit,
                    "style" => AxValueSourceType::Style,
                    "contents" => AxValueSourceType::Contents,
                    "placeholder" => AxValueSourceType::Placeholder,
                    "relatedElement" => AxValueSourceType::RelatedElement,
                    _ => AxValueSourceType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(AxValueSourceType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Accessibility.type.AXValueType。
///
///协议原文：
///
///<pre>Enum of possible property types.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AxValueType {
    ///协议值 "boolean"。
    Boolean,
    ///协议值 "tristate"。
    Tristate,
    ///协议值 "booleanOrUndefined"。
    BooleanOrUndefined,
    ///协议值 "idref"。
    Idref,
    ///协议值 "idrefList"。
    IdrefList,
    ///协议值 "integer"。
    Integer,
    ///协议值 "node"。
    Node,
    ///协议值 "nodeList"。
    NodeList,
    ///协议值 "number"。
    Number,
    ///协议值 "string"。
    String,
    ///协议值 "computedString"。
    ComputedString,
    ///协议值 "token"。
    Token,
    ///协议值 "tokenList"。
    TokenList,
    ///协议值 "domRelation"。
    DomRelation,
    ///协议值 "role"。
    Role,
    ///协议值 "internalRole"。
    InternalRole,
    ///协议值 "valueUndefined"。
    ValueUndefined,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl AxValueType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::accessibility::types::AxValueType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Boolean => "boolean",
            Self::Tristate => "tristate",
            Self::BooleanOrUndefined => "booleanOrUndefined",
            Self::Idref => "idref",
            Self::IdrefList => "idrefList",
            Self::Integer => "integer",
            Self::Node => "node",
            Self::NodeList => "nodeList",
            Self::Number => "number",
            Self::String => "string",
            Self::ComputedString => "computedString",
            Self::Token => "token",
            Self::TokenList => "tokenList",
            Self::DomRelation => "domRelation",
            Self::Role => "role",
            Self::InternalRole => "internalRole",
            Self::ValueUndefined => "valueUndefined",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for AxValueType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for AxValueType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "boolean" => AxValueType::Boolean,
            "tristate" => AxValueType::Tristate,
            "booleanOrUndefined" => AxValueType::BooleanOrUndefined,
            "idref" => AxValueType::Idref,
            "idrefList" => AxValueType::IdrefList,
            "integer" => AxValueType::Integer,
            "node" => AxValueType::Node,
            "nodeList" => AxValueType::NodeList,
            "number" => AxValueType::Number,
            "string" => AxValueType::String,
            "computedString" => AxValueType::ComputedString,
            "token" => AxValueType::Token,
            "tokenList" => AxValueType::TokenList,
            "domRelation" => AxValueType::DomRelation,
            "role" => AxValueType::Role,
            "internalRole" => AxValueType::InternalRole,
            "valueUndefined" => AxValueType::ValueUndefined,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for AxValueType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AxValueType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AxValueType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "boolean" => AxValueType::Boolean,
                    "tristate" => AxValueType::Tristate,
                    "booleanOrUndefined" => AxValueType::BooleanOrUndefined,
                    "idref" => AxValueType::Idref,
                    "idrefList" => AxValueType::IdrefList,
                    "integer" => AxValueType::Integer,
                    "node" => AxValueType::Node,
                    "nodeList" => AxValueType::NodeList,
                    "number" => AxValueType::Number,
                    "string" => AxValueType::String,
                    "computedString" => AxValueType::ComputedString,
                    "token" => AxValueType::Token,
                    "tokenList" => AxValueType::TokenList,
                    "domRelation" => AxValueType::DomRelation,
                    "role" => AxValueType::Role,
                    "internalRole" => AxValueType::InternalRole,
                    "valueUndefined" => AxValueType::ValueUndefined,
                    _ => AxValueType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(AxValueType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
