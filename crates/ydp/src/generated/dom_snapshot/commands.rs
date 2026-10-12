///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMSnapshot.command.captureSnapshot/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CaptureSnapshotParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.command.captureSnapshot/parameters/computedStyles。
    ///
    ///协议原文：
    ///
    ///<pre>Whitelist of computed styles to return.</pre>
    ///
    ///
    #[serde(rename = "computedStyles")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub computed_styles: Vec<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.command.captureSnapshot/parameters/includePaintOrder。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to include layout object paint orders into the snapshot.</pre>
    ///
    ///
    #[serde(rename = "includePaintOrder")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_paint_order: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.command.captureSnapshot/parameters/includeDOMRects。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to include DOM rectangles (offsetRects, clientRects, scrollRects) into the snapshot</pre>
    ///
    ///
    #[serde(rename = "includeDOMRects")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_dom_rects: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.command.captureSnapshot/parameters/includeBlendedBackgroundColors。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to include blended background colors in the snapshot (default: false).
    ///Blended background color is achieved by blending background colors of all elements
    ///that overlap with the current element.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "includeBlendedBackgroundColors")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_blended_background_colors: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.command.captureSnapshot/parameters/includeTextColorOpacities。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to include text color opacity in the snapshot (default: false).
    ///An element might have the opacity property set that affects the text color of the element.
    ///The final text color opacity is computed based on the opacity of all overlapping elements.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "includeTextColorOpacities")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_text_color_opacities: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for CaptureSnapshotParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CaptureSnapshotParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.command.captureSnapshot/parameters/computedStyles。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whitelist of computed styles to return.</pre>
                    ///
                    ///
                    #[serde(rename = "computedStyles")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub computed_styles: Vec<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.command.captureSnapshot/parameters/includePaintOrder。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to include layout object paint orders into the snapshot.</pre>
                    ///
                    ///
                    #[serde(rename = "includePaintOrder")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub include_paint_order: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.command.captureSnapshot/parameters/includeDOMRects。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to include DOM rectangles (offsetRects, clientRects, scrollRects) into the snapshot</pre>
                    ///
                    ///
                    #[serde(rename = "includeDOMRects")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub include_dom_rects: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.command.captureSnapshot/parameters/includeBlendedBackgroundColors。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to include blended background colors in the snapshot (default: false).
                    ///Blended background color is achieved by blending background colors of all elements
                    ///that overlap with the current element.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "includeBlendedBackgroundColors")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub include_blended_background_colors: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.command.captureSnapshot/parameters/includeTextColorOpacities。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to include text color opacity in the snapshot (default: false).
                    ///An element might have the opacity property set that affects the text color of the element.
                    ///The final text color opacity is computed based on the opacity of all overlapping elements.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "includeTextColorOpacities")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub include_text_color_opacities: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CaptureSnapshotParams {
                    computed_styles: fields.computed_styles,
                    include_paint_order: fields.include_paint_order,
                    include_dom_rects: fields.include_dom_rects,
                    include_blended_background_colors: fields.include_blended_background_colors,
                    include_text_color_opacities: fields.include_text_color_opacities,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOMSnapshot.command.captureSnapshot。
///
///协议原文：
///
///<pre>Returns a document snapshot, including the full DOM tree of the root node (including iframes,
///template contents, and imported documents) in a flattened array, as well as layout and
///white-listed computed style information for the nodes. Shadow DOM in the returned DOM tree is
///flattened.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct CaptureSnapshot;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMSnapshot.command.captureSnapshot/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CaptureSnapshotResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.command.captureSnapshot/returns/documents。
    ///
    ///协议原文：
    ///
    ///<pre>The nodes in the DOM tree. The DOMNode at index 0 corresponds to the root document.</pre>
    ///
    ///
    #[serde(rename = "documents")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub documents: Vec<crate::dom_snapshot::types::DocumentSnapshot>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.command.captureSnapshot/returns/strings。
    ///
    ///协议原文：
    ///
    ///<pre>Shared string table that all string properties refer to with indexes.</pre>
    ///
    ///
    #[serde(rename = "strings")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub strings: Vec<String>,
}
impl<'de> serde::Deserialize<'de> for CaptureSnapshotResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CaptureSnapshotResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.command.captureSnapshot/returns/documents。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The nodes in the DOM tree. The DOMNode at index 0 corresponds to the root document.</pre>
                    ///
                    ///
                    #[serde(rename = "documents")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub documents: Vec<crate::dom_snapshot::types::DocumentSnapshot>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.command.captureSnapshot/returns/strings。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Shared string table that all string properties refer to with indexes.</pre>
                    ///
                    ///
                    #[serde(rename = "strings")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub strings: Vec<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CaptureSnapshotResponse {
                    documents: fields.documents,
                    strings: fields.strings,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for CaptureSnapshot {
    type Params = CaptureSnapshotParams;
    type Response = CaptureSnapshotResponse;
    const METHOD: &'static str = "DOMSnapshot.captureSnapshot";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMSnapshot.command.disable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOMSnapshot.command.disable。
///
///协议原文：
///
///<pre>Disables DOM snapshot agent for the given page.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMSnapshot.command.disable/returns。
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
    const METHOD: &'static str = "DOMSnapshot.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMSnapshot.command.enable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOMSnapshot.command.enable。
///
///协议原文：
///
///<pre>Enables DOM snapshot agent for the given page.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMSnapshot.command.enable/returns。
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
    const METHOD: &'static str = "DOMSnapshot.enable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMSnapshot.command.getSnapshot/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetSnapshotParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.command.getSnapshot/parameters/computedStyleWhitelist。
    ///
    ///协议原文：
    ///
    ///<pre>Whitelist of computed styles to return.</pre>
    ///
    ///
    #[serde(rename = "computedStyleWhitelist")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub computed_style_whitelist: Vec<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.command.getSnapshot/parameters/includeEventListeners。
    ///
    ///协议原文：
    ///
    ///<pre>Whether or not to retrieve details of DOM listeners (default false).</pre>
    ///
    ///
    #[serde(rename = "includeEventListeners")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_event_listeners: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.command.getSnapshot/parameters/includePaintOrder。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to determine and include the paint order index of LayoutTreeNodes (default false).</pre>
    ///
    ///
    #[serde(rename = "includePaintOrder")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_paint_order: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.command.getSnapshot/parameters/includeUserAgentShadowTree。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to include UA shadow tree in the snapshot (default false).</pre>
    ///
    ///
    #[serde(rename = "includeUserAgentShadowTree")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_user_agent_shadow_tree: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for GetSnapshotParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetSnapshotParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.command.getSnapshot/parameters/computedStyleWhitelist。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whitelist of computed styles to return.</pre>
                    ///
                    ///
                    #[serde(rename = "computedStyleWhitelist")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub computed_style_whitelist: Vec<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.command.getSnapshot/parameters/includeEventListeners。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether or not to retrieve details of DOM listeners (default false).</pre>
                    ///
                    ///
                    #[serde(rename = "includeEventListeners")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub include_event_listeners: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.command.getSnapshot/parameters/includePaintOrder。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to determine and include the paint order index of LayoutTreeNodes (default false).</pre>
                    ///
                    ///
                    #[serde(rename = "includePaintOrder")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub include_paint_order: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMSnapshot.command.getSnapshot/parameters/includeUserAgentShadowTree。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to include UA shadow tree in the snapshot (default false).</pre>
                    ///
                    ///
                    #[serde(rename = "includeUserAgentShadowTree")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub include_user_agent_shadow_tree: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetSnapshotParams {
                    computed_style_whitelist: fields.computed_style_whitelist,
                    include_event_listeners: fields.include_event_listeners,
                    include_paint_order: fields.include_paint_order,
                    include_user_agent_shadow_tree: fields.include_user_agent_shadow_tree,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOMSnapshot.command.getSnapshot。
///
///协议原文：
///
///<pre>Returns a document snapshot, including the full DOM tree of the root node (including iframes,
///template contents, and imported documents) in a flattened array, as well as layout and
///white-listed computed style information for the nodes. Shadow DOM in the returned DOM tree is
///flattened.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct GetSnapshot;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMSnapshot.command.getSnapshot/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetSnapshotResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.command.getSnapshot/returns/domNodes。
    ///
    ///协议原文：
    ///
    ///<pre>The nodes in the DOM tree. The DOMNode at index 0 corresponds to the root document.</pre>
    ///
    ///
    #[serde(rename = "domNodes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub dom_nodes: Vec<crate::dom_snapshot::types::DomNode>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.command.getSnapshot/returns/layoutTreeNodes。
    ///
    ///协议原文：
    ///
    ///<pre>The nodes in the layout tree.</pre>
    ///
    ///
    #[serde(rename = "layoutTreeNodes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub layout_tree_nodes: Vec<crate::dom_snapshot::types::LayoutTreeNode>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.command.getSnapshot/returns/computedStyles。
    ///
    ///协议原文：
    ///
    ///<pre>Whitelisted ComputedStyle properties for each node in the layout tree.</pre>
    ///
    ///
    #[serde(rename = "computedStyles")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub computed_styles: Vec<crate::dom_snapshot::types::ComputedStyle>,
}
impl<'de> serde::Deserialize<'de> for GetSnapshotResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetSnapshotResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.command.getSnapshot/returns/domNodes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The nodes in the DOM tree. The DOMNode at index 0 corresponds to the root document.</pre>
                    ///
                    ///
                    #[serde(rename = "domNodes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub dom_nodes: Vec<crate::dom_snapshot::types::DomNode>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.command.getSnapshot/returns/layoutTreeNodes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The nodes in the layout tree.</pre>
                    ///
                    ///
                    #[serde(rename = "layoutTreeNodes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub layout_tree_nodes: Vec<crate::dom_snapshot::types::LayoutTreeNode>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMSnapshot.command.getSnapshot/returns/computedStyles。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whitelisted ComputedStyle properties for each node in the layout tree.</pre>
                    ///
                    ///
                    #[serde(rename = "computedStyles")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub computed_styles: Vec<crate::dom_snapshot::types::ComputedStyle>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetSnapshotResponse {
                    dom_nodes: fields.dom_nodes,
                    layout_tree_nodes: fields.layout_tree_nodes,
                    computed_styles: fields.computed_styles,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetSnapshot {
    type Params = GetSnapshotParams;
    type Response = GetSnapshotResponse;
    const METHOD: &'static str = "DOMSnapshot.getSnapshot";
}
