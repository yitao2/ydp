///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.event.computedStyleUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ComputedStyleUpdatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.event.computedStyleUpdated/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>The node id that has updated computed styles.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for ComputedStyleUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ComputedStyleUpdatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.event.computedStyleUpdated/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The node id that has updated computed styles.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ComputedStyleUpdatedParams {
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.event.computedStyleUpdated。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ComputedStyleUpdated;
impl ydp_core::Event for ComputedStyleUpdated {
    type Params = ComputedStyleUpdatedParams;
    const METHOD: &'static str = "CSS.computedStyleUpdated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.event.fontsUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct FontsUpdatedParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.event.fontsUpdated/parameters/font。
    ///
    ///协议原文：
    ///
    ///<pre>The web font that has loaded.</pre>
    ///
    ///
    #[serde(rename = "font")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub font: Option<crate::css::types::FontFace>,
}
impl<'de> serde::Deserialize<'de> for FontsUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FontsUpdatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.event.fontsUpdated/parameters/font。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The web font that has loaded.</pre>
                    ///
                    ///
                    #[serde(rename = "font")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub font: Option<crate::css::types::FontFace>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FontsUpdatedParams { font: fields.font })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.event.fontsUpdated。
///
///协议原文：
///
///<pre>Fires whenever a web font is updated.  A non-empty font parameter indicates a successfully loaded
///web font.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct FontsUpdated;
impl ydp_core::Event for FontsUpdated {
    type Params = FontsUpdatedParams;
    const METHOD: &'static str = "CSS.fontsUpdated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.event.mediaQueryResultChanged/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct MediaQueryResultChangedParams {}
impl<'de> serde::Deserialize<'de> for MediaQueryResultChangedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MediaQueryResultChangedParams;
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
                Ok(MediaQueryResultChangedParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.event.mediaQueryResultChanged。
///
///协议原文：
///
///<pre>Fires whenever a MediaQuery result changes (for example, after a browser window has been
///resized.) The current implementation considers only viewport-dependent media features.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct MediaQueryResultChanged;
impl ydp_core::Event for MediaQueryResultChanged {
    type Params = MediaQueryResultChangedParams;
    const METHOD: &'static str = "CSS.mediaQueryResultChanged";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.event.styleSheetAdded/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StyleSheetAddedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.event.styleSheetAdded/parameters/header。
    ///
    ///协议原文：
    ///
    ///<pre>Added stylesheet metainfo.</pre>
    ///
    ///
    #[serde(rename = "header")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub header: crate::css::types::CssStyleSheetHeader,
}
impl<'de> serde::Deserialize<'de> for StyleSheetAddedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StyleSheetAddedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.event.styleSheetAdded/parameters/header。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Added stylesheet metainfo.</pre>
                    ///
                    ///
                    #[serde(rename = "header")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub header: crate::css::types::CssStyleSheetHeader,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StyleSheetAddedParams {
                    header: fields.header,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.event.styleSheetAdded。
///
///协议原文：
///
///<pre>Fired whenever an active document stylesheet is added.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct StyleSheetAdded;
impl ydp_core::Event for StyleSheetAdded {
    type Params = StyleSheetAddedParams;
    const METHOD: &'static str = "CSS.styleSheetAdded";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.event.styleSheetChanged/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StyleSheetChangedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.event.styleSheetChanged/parameters/styleSheetId。
    #[serde(rename = "styleSheetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub style_sheet_id: crate::dom::types::StyleSheetId,
}
impl<'de> serde::Deserialize<'de> for StyleSheetChangedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StyleSheetChangedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.event.styleSheetChanged/parameters/styleSheetId。
                    #[serde(rename = "styleSheetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub style_sheet_id: crate::dom::types::StyleSheetId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StyleSheetChangedParams {
                    style_sheet_id: fields.style_sheet_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.event.styleSheetChanged。
///
///协议原文：
///
///<pre>Fired whenever a stylesheet is changed as a result of the client operation.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct StyleSheetChanged;
impl ydp_core::Event for StyleSheetChanged {
    type Params = StyleSheetChangedParams;
    const METHOD: &'static str = "CSS.styleSheetChanged";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.event.styleSheetRemoved/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StyleSheetRemovedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.event.styleSheetRemoved/parameters/styleSheetId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the removed stylesheet.</pre>
    ///
    ///
    #[serde(rename = "styleSheetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub style_sheet_id: crate::dom::types::StyleSheetId,
}
impl<'de> serde::Deserialize<'de> for StyleSheetRemovedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StyleSheetRemovedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.event.styleSheetRemoved/parameters/styleSheetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the removed stylesheet.</pre>
                    ///
                    ///
                    #[serde(rename = "styleSheetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub style_sheet_id: crate::dom::types::StyleSheetId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StyleSheetRemovedParams {
                    style_sheet_id: fields.style_sheet_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.event.styleSheetRemoved。
///
///协议原文：
///
///<pre>Fired whenever an active document stylesheet is removed.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct StyleSheetRemoved;
impl ydp_core::Event for StyleSheetRemoved {
    type Params = StyleSheetRemovedParams;
    const METHOD: &'static str = "CSS.styleSheetRemoved";
}
