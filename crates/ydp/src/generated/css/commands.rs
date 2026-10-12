///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.addRule/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddRuleParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.addRule/parameters/styleSheetId。
    ///
    ///协议原文：
    ///
    ///<pre>The css style sheet identifier where a new rule should be inserted.</pre>
    ///
    ///
    #[serde(rename = "styleSheetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub style_sheet_id: crate::dom::types::StyleSheetId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.addRule/parameters/ruleText。
    ///
    ///协议原文：
    ///
    ///<pre>The text of a new rule.</pre>
    ///
    ///
    #[serde(rename = "ruleText")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub rule_text: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.addRule/parameters/location。
    ///
    ///协议原文：
    ///
    ///<pre>Text position of a new rule in the target style sheet.</pre>
    ///
    ///
    #[serde(rename = "location")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub location: crate::css::types::SourceRange,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.addRule/parameters/nodeForPropertySyntaxValidation。
    ///
    ///协议原文：
    ///
    ///<pre>NodeId for the DOM node in whose context custom property declarations for registered properties should be
    ///validated. If omitted, declarations in the new rule text can only be validated statically, which may produce
    ///incorrect results if the declaration contains a var() for example.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "nodeForPropertySyntaxValidation")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub node_for_property_syntax_validation: Option<crate::dom::types::NodeId>,
}
impl<'de> serde::Deserialize<'de> for AddRuleParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddRuleParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.addRule/parameters/styleSheetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The css style sheet identifier where a new rule should be inserted.</pre>
                    ///
                    ///
                    #[serde(rename = "styleSheetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub style_sheet_id: crate::dom::types::StyleSheetId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.addRule/parameters/ruleText。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The text of a new rule.</pre>
                    ///
                    ///
                    #[serde(rename = "ruleText")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub rule_text: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.addRule/parameters/location。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Text position of a new rule in the target style sheet.</pre>
                    ///
                    ///
                    #[serde(rename = "location")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub location: crate::css::types::SourceRange,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.addRule/parameters/nodeForPropertySyntaxValidation。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>NodeId for the DOM node in whose context custom property declarations for registered properties should be
                    ///validated. If omitted, declarations in the new rule text can only be validated statically, which may produce
                    ///incorrect results if the declaration contains a var() for example.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "nodeForPropertySyntaxValidation")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub node_for_property_syntax_validation: Option<crate::dom::types::NodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AddRuleParams {
                    style_sheet_id: fields.style_sheet_id,
                    rule_text: fields.rule_text,
                    location: fields.location,
                    node_for_property_syntax_validation: fields.node_for_property_syntax_validation,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.addRule。
///
///协议原文：
///
///<pre>Inserts a new rule with the given `ruleText` in a stylesheet with given `styleSheetId`, at the
///position specified by `location`.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct AddRule;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.addRule/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddRuleResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.addRule/returns/rule。
    ///
    ///协议原文：
    ///
    ///<pre>The newly created rule.</pre>
    ///
    ///
    #[serde(rename = "rule")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub rule: crate::css::types::CssRule,
}
impl<'de> serde::Deserialize<'de> for AddRuleResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddRuleResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.addRule/returns/rule。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The newly created rule.</pre>
                    ///
                    ///
                    #[serde(rename = "rule")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub rule: crate::css::types::CssRule,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AddRuleResponse { rule: fields.rule })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for AddRule {
    type Params = AddRuleParams;
    type Response = AddRuleResponse;
    const METHOD: &'static str = "CSS.addRule";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.collectClassNames/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CollectClassNamesParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.collectClassNames/parameters/styleSheetId。
    #[serde(rename = "styleSheetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub style_sheet_id: crate::dom::types::StyleSheetId,
}
impl<'de> serde::Deserialize<'de> for CollectClassNamesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CollectClassNamesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.collectClassNames/parameters/styleSheetId。
                    #[serde(rename = "styleSheetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub style_sheet_id: crate::dom::types::StyleSheetId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CollectClassNamesParams {
                    style_sheet_id: fields.style_sheet_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.collectClassNames。
///
///协议原文：
///
///<pre>Returns all class names from specified stylesheet.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct CollectClassNames;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.collectClassNames/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CollectClassNamesResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.collectClassNames/returns/classNames。
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
impl<'de> serde::Deserialize<'de> for CollectClassNamesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CollectClassNamesResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.collectClassNames/returns/classNames。
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
                Ok(CollectClassNamesResponse {
                    class_names: fields.class_names,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for CollectClassNames {
    type Params = CollectClassNamesParams;
    type Response = CollectClassNamesResponse;
    const METHOD: &'static str = "CSS.collectClassNames";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.createStyleSheet/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CreateStyleSheetParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.createStyleSheet/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the frame where "via-inspector" stylesheet should be created.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.createStyleSheet/parameters/force。
    ///
    ///协议原文：
    ///
    ///<pre>If true, creates a new stylesheet for every call. If false,
    ///returns a stylesheet previously created by a call with force=false
    ///for the frame's document if it exists or creates a new stylesheet
    ///(default: false).</pre>
    ///
    ///
    #[serde(rename = "force")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub force: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for CreateStyleSheetParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CreateStyleSheetParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.createStyleSheet/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the frame where "via-inspector" stylesheet should be created.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.createStyleSheet/parameters/force。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, creates a new stylesheet for every call. If false,
                    ///returns a stylesheet previously created by a call with force=false
                    ///for the frame's document if it exists or creates a new stylesheet
                    ///(default: false).</pre>
                    ///
                    ///
                    #[serde(rename = "force")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub force: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CreateStyleSheetParams {
                    frame_id: fields.frame_id,
                    force: fields.force,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.createStyleSheet。
///
///协议原文：
///
///<pre>Creates a new special "via-inspector" stylesheet in the frame with given `frameId`.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct CreateStyleSheet;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.createStyleSheet/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CreateStyleSheetResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.createStyleSheet/returns/styleSheetId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the created "via-inspector" stylesheet.</pre>
    ///
    ///
    #[serde(rename = "styleSheetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub style_sheet_id: crate::dom::types::StyleSheetId,
}
impl<'de> serde::Deserialize<'de> for CreateStyleSheetResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CreateStyleSheetResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.createStyleSheet/returns/styleSheetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the created "via-inspector" stylesheet.</pre>
                    ///
                    ///
                    #[serde(rename = "styleSheetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub style_sheet_id: crate::dom::types::StyleSheetId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CreateStyleSheetResponse {
                    style_sheet_id: fields.style_sheet_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for CreateStyleSheet {
    type Params = CreateStyleSheetParams;
    type Response = CreateStyleSheetResponse;
    const METHOD: &'static str = "CSS.createStyleSheet";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.disable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.disable。
///
///协议原文：
///
///<pre>Disables the CSS agent for the given page.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.disable/returns。
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
    const METHOD: &'static str = "CSS.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.enable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.enable。
///
///协议原文：
///
///<pre>Enables the CSS agent for the given page. Clients should not assume that the CSS agent has been
///enabled until the result of this command is received.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.enable/returns。
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
    const METHOD: &'static str = "CSS.enable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.forcePseudoState/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ForcePseudoStateParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.forcePseudoState/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>The element id for which to force the pseudo state.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.forcePseudoState/parameters/forcedPseudoClasses。
    ///
    ///协议原文：
    ///
    ///<pre>Element pseudo classes to force when computing the element's style.</pre>
    ///
    ///
    #[serde(rename = "forcedPseudoClasses")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub forced_pseudo_classes: Vec<String>,
}
impl<'de> serde::Deserialize<'de> for ForcePseudoStateParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ForcePseudoStateParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.forcePseudoState/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The element id for which to force the pseudo state.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.forcePseudoState/parameters/forcedPseudoClasses。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Element pseudo classes to force when computing the element's style.</pre>
                    ///
                    ///
                    #[serde(rename = "forcedPseudoClasses")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub forced_pseudo_classes: Vec<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ForcePseudoStateParams {
                    node_id: fields.node_id,
                    forced_pseudo_classes: fields.forced_pseudo_classes,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.forcePseudoState。
///
///协议原文：
///
///<pre>Ensures that the given node will have specified pseudo-classes whenever its style is computed by
///the browser.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ForcePseudoState;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.forcePseudoState/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ForcePseudoStateResponse {}
impl<'de> serde::Deserialize<'de> for ForcePseudoStateResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ForcePseudoStateResponse;
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
                Ok(ForcePseudoStateResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ForcePseudoState {
    type Params = ForcePseudoStateParams;
    type Response = ForcePseudoStateResponse;
    const METHOD: &'static str = "CSS.forcePseudoState";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.forceStartingStyle/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ForceStartingStyleParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.forceStartingStyle/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>The element id for which to force the starting-style state.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.forceStartingStyle/parameters/forced。
    ///
    ///协议原文：
    ///
    ///<pre>Boolean indicating if this is on or off.</pre>
    ///
    ///
    #[serde(rename = "forced")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub forced: bool,
}
impl<'de> serde::Deserialize<'de> for ForceStartingStyleParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ForceStartingStyleParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.forceStartingStyle/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The element id for which to force the starting-style state.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.forceStartingStyle/parameters/forced。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Boolean indicating if this is on or off.</pre>
                    ///
                    ///
                    #[serde(rename = "forced")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub forced: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ForceStartingStyleParams {
                    node_id: fields.node_id,
                    forced: fields.forced,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.forceStartingStyle。
///
///协议原文：
///
///<pre>Ensures that the given node is in its starting-style state.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ForceStartingStyle;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.forceStartingStyle/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ForceStartingStyleResponse {}
impl<'de> serde::Deserialize<'de> for ForceStartingStyleResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ForceStartingStyleResponse;
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
                Ok(ForceStartingStyleResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ForceStartingStyle {
    type Params = ForceStartingStyleParams;
    type Response = ForceStartingStyleResponse;
    const METHOD: &'static str = "CSS.forceStartingStyle";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.getAnimatedStylesForNode/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetAnimatedStylesForNodeParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getAnimatedStylesForNode/parameters/nodeId。
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for GetAnimatedStylesForNodeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetAnimatedStylesForNodeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getAnimatedStylesForNode/parameters/nodeId。
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetAnimatedStylesForNodeParams {
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.getAnimatedStylesForNode。
///
///协议原文：
///
///<pre>Returns the styles coming from animations &amp; transitions
///including the animation &amp; transition styles coming from inheritance chain.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetAnimatedStylesForNode;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.getAnimatedStylesForNode/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetAnimatedStylesForNodeResponse {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getAnimatedStylesForNode/returns/animationStyles。
    ///
    ///协议原文：
    ///
    ///<pre>Styles coming from animations.</pre>
    ///
    ///
    #[serde(rename = "animationStyles")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub animation_styles: Option<Vec<crate::css::types::CssAnimationStyle>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getAnimatedStylesForNode/returns/transitionsStyle。
    ///
    ///协议原文：
    ///
    ///<pre>Style coming from transitions.</pre>
    ///
    ///
    #[serde(rename = "transitionsStyle")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub transitions_style: Option<crate::css::types::CssStyle>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getAnimatedStylesForNode/returns/inherited。
    ///
    ///协议原文：
    ///
    ///<pre>Inherited style entries for animationsStyle and transitionsStyle from
    ///the inheritance chain of the element.</pre>
    ///
    ///
    #[serde(rename = "inherited")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub inherited: Option<Vec<crate::css::types::InheritedAnimatedStyleEntry>>,
}
impl<'de> serde::Deserialize<'de> for GetAnimatedStylesForNodeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetAnimatedStylesForNodeResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getAnimatedStylesForNode/returns/animationStyles。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Styles coming from animations.</pre>
                    ///
                    ///
                    #[serde(rename = "animationStyles")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub animation_styles: Option<Vec<crate::css::types::CssAnimationStyle>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getAnimatedStylesForNode/returns/transitionsStyle。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Style coming from transitions.</pre>
                    ///
                    ///
                    #[serde(rename = "transitionsStyle")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub transitions_style: Option<crate::css::types::CssStyle>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getAnimatedStylesForNode/returns/inherited。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Inherited style entries for animationsStyle and transitionsStyle from
                    ///the inheritance chain of the element.</pre>
                    ///
                    ///
                    #[serde(rename = "inherited")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub inherited: Option<Vec<crate::css::types::InheritedAnimatedStyleEntry>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetAnimatedStylesForNodeResponse {
                    animation_styles: fields.animation_styles,
                    transitions_style: fields.transitions_style,
                    inherited: fields.inherited,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetAnimatedStylesForNode {
    type Params = GetAnimatedStylesForNodeParams;
    type Response = GetAnimatedStylesForNodeResponse;
    const METHOD: &'static str = "CSS.getAnimatedStylesForNode";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.getBackgroundColors/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetBackgroundColorsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getBackgroundColors/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node to get background colors for.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for GetBackgroundColorsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetBackgroundColorsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getBackgroundColors/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node to get background colors for.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetBackgroundColorsParams {
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.getBackgroundColors。
#[derive(Debug, Clone, Copy)]
pub struct GetBackgroundColors;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.getBackgroundColors/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetBackgroundColorsResponse {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getBackgroundColors/returns/backgroundColors。
    ///
    ///协议原文：
    ///
    ///<pre>The range of background colors behind this element, if it contains any visible text. If no
    ///visible text is present, this will be undefined. In the case of a flat background color,
    ///this will consist of simply that color. In the case of a gradient, this will consist of each
    ///of the color stops. For anything more complicated, this will be an empty array. Images will
    ///be ignored (as if the image had failed to load).</pre>
    ///
    ///
    #[serde(rename = "backgroundColors")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub background_colors: Option<Vec<String>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getBackgroundColors/returns/computedFontSize。
    ///
    ///协议原文：
    ///
    ///<pre>The computed font size for this node, as a CSS computed value string (e.g. '12px').</pre>
    ///
    ///
    #[serde(rename = "computedFontSize")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub computed_font_size: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getBackgroundColors/returns/computedFontWeight。
    ///
    ///协议原文：
    ///
    ///<pre>The computed font weight for this node, as a CSS computed value string (e.g. 'normal' or
    ///'100').</pre>
    ///
    ///
    #[serde(rename = "computedFontWeight")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub computed_font_weight: Option<String>,
}
impl<'de> serde::Deserialize<'de> for GetBackgroundColorsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetBackgroundColorsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getBackgroundColors/returns/backgroundColors。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The range of background colors behind this element, if it contains any visible text. If no
                    ///visible text is present, this will be undefined. In the case of a flat background color,
                    ///this will consist of simply that color. In the case of a gradient, this will consist of each
                    ///of the color stops. For anything more complicated, this will be an empty array. Images will
                    ///be ignored (as if the image had failed to load).</pre>
                    ///
                    ///
                    #[serde(rename = "backgroundColors")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub background_colors: Option<Vec<String>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getBackgroundColors/returns/computedFontSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The computed font size for this node, as a CSS computed value string (e.g. '12px').</pre>
                    ///
                    ///
                    #[serde(rename = "computedFontSize")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub computed_font_size: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getBackgroundColors/returns/computedFontWeight。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The computed font weight for this node, as a CSS computed value string (e.g. 'normal' or
                    ///'100').</pre>
                    ///
                    ///
                    #[serde(rename = "computedFontWeight")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub computed_font_weight: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetBackgroundColorsResponse {
                    background_colors: fields.background_colors,
                    computed_font_size: fields.computed_font_size,
                    computed_font_weight: fields.computed_font_weight,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetBackgroundColors {
    type Params = GetBackgroundColorsParams;
    type Response = GetBackgroundColorsResponse;
    const METHOD: &'static str = "CSS.getBackgroundColors";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.getComputedStyleForNode/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetComputedStyleForNodeParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getComputedStyleForNode/parameters/nodeId。
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for GetComputedStyleForNodeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetComputedStyleForNodeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getComputedStyleForNode/parameters/nodeId。
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetComputedStyleForNodeParams {
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.getComputedStyleForNode。
///
///协议原文：
///
///<pre>Returns the computed style for a DOM node identified by `nodeId`.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetComputedStyleForNode;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.getComputedStyleForNode/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetComputedStyleForNodeResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getComputedStyleForNode/returns/computedStyle。
    ///
    ///协议原文：
    ///
    ///<pre>Computed style for the specified DOM node.</pre>
    ///
    ///
    #[serde(rename = "computedStyle")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub computed_style: Vec<crate::css::types::CssComputedStyleProperty>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getComputedStyleForNode/returns/extraFields。
    ///
    ///协议原文：
    ///
    ///<pre>A list of non-standard "extra fields" which blink stores alongside each
    ///computed style.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "extraFields")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub extra_fields: crate::css::types::ComputedStyleExtraFields,
}
impl<'de> serde::Deserialize<'de> for GetComputedStyleForNodeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetComputedStyleForNodeResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getComputedStyleForNode/returns/computedStyle。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Computed style for the specified DOM node.</pre>
                    ///
                    ///
                    #[serde(rename = "computedStyle")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub computed_style: Vec<crate::css::types::CssComputedStyleProperty>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getComputedStyleForNode/returns/extraFields。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A list of non-standard "extra fields" which blink stores alongside each
                    ///computed style.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "extraFields")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub extra_fields: crate::css::types::ComputedStyleExtraFields,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetComputedStyleForNodeResponse {
                    computed_style: fields.computed_style,
                    extra_fields: fields.extra_fields,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetComputedStyleForNode {
    type Params = GetComputedStyleForNodeParams;
    type Response = GetComputedStyleForNodeResponse;
    const METHOD: &'static str = "CSS.getComputedStyleForNode";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.getEnvironmentVariables/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetEnvironmentVariablesParams {}
impl<'de> serde::Deserialize<'de> for GetEnvironmentVariablesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetEnvironmentVariablesParams;
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
                Ok(GetEnvironmentVariablesParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.getEnvironmentVariables。
///
///协议原文：
///
///<pre>Returns the values of the default UA-defined environment variables used in env()</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetEnvironmentVariables;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.getEnvironmentVariables/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetEnvironmentVariablesResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getEnvironmentVariables/returns/environmentVariables。
    #[serde(rename = "environmentVariables")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub environment_variables: std::collections::BTreeMap<String, serde_json::Value>,
}
impl<'de> serde::Deserialize<'de> for GetEnvironmentVariablesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetEnvironmentVariablesResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getEnvironmentVariables/returns/environmentVariables。
                    #[serde(rename = "environmentVariables")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub environment_variables:
                        std::collections::BTreeMap<String, serde_json::Value>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetEnvironmentVariablesResponse {
                    environment_variables: fields.environment_variables,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetEnvironmentVariables {
    type Params = GetEnvironmentVariablesParams;
    type Response = GetEnvironmentVariablesResponse;
    const METHOD: &'static str = "CSS.getEnvironmentVariables";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.getInlineStylesForNode/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetInlineStylesForNodeParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getInlineStylesForNode/parameters/nodeId。
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for GetInlineStylesForNodeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetInlineStylesForNodeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getInlineStylesForNode/parameters/nodeId。
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetInlineStylesForNodeParams {
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.getInlineStylesForNode。
///
///协议原文：
///
///<pre>Returns the styles defined inline (explicitly in the "style" attribute and implicitly, using DOM
///attributes) for a DOM node identified by `nodeId`.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetInlineStylesForNode;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.getInlineStylesForNode/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetInlineStylesForNodeResponse {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getInlineStylesForNode/returns/inlineStyle。
    ///
    ///协议原文：
    ///
    ///<pre>Inline style for the specified DOM node.</pre>
    ///
    ///
    #[serde(rename = "inlineStyle")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub inline_style: Option<crate::css::types::CssStyle>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getInlineStylesForNode/returns/attributesStyle。
    ///
    ///协议原文：
    ///
    ///<pre>Attribute-defined element style (e.g. resulting from "width=20 height=100%").</pre>
    ///
    ///
    #[serde(rename = "attributesStyle")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub attributes_style: Option<crate::css::types::CssStyle>,
}
impl<'de> serde::Deserialize<'de> for GetInlineStylesForNodeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetInlineStylesForNodeResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getInlineStylesForNode/returns/inlineStyle。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Inline style for the specified DOM node.</pre>
                    ///
                    ///
                    #[serde(rename = "inlineStyle")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub inline_style: Option<crate::css::types::CssStyle>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getInlineStylesForNode/returns/attributesStyle。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Attribute-defined element style (e.g. resulting from "width=20 height=100%").</pre>
                    ///
                    ///
                    #[serde(rename = "attributesStyle")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub attributes_style: Option<crate::css::types::CssStyle>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetInlineStylesForNodeResponse {
                    inline_style: fields.inline_style,
                    attributes_style: fields.attributes_style,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetInlineStylesForNode {
    type Params = GetInlineStylesForNodeParams;
    type Response = GetInlineStylesForNodeResponse;
    const METHOD: &'static str = "CSS.getInlineStylesForNode";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.getLayersForNode/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetLayersForNodeParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getLayersForNode/parameters/nodeId。
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for GetLayersForNodeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetLayersForNodeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getLayersForNode/parameters/nodeId。
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetLayersForNodeParams {
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.getLayersForNode。
///
///协议原文：
///
///<pre>Returns all layers parsed by the rendering engine for the tree scope of a node.
///Given a DOM element identified by nodeId, getLayersForNode returns the root
///layer for the nearest ancestor document or shadow root. The layer root contains
///the full layer tree for the tree scope and their ordering.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetLayersForNode;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.getLayersForNode/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetLayersForNodeResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getLayersForNode/returns/rootLayer。
    #[serde(rename = "rootLayer")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub root_layer: crate::css::types::CssLayerData,
}
impl<'de> serde::Deserialize<'de> for GetLayersForNodeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetLayersForNodeResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getLayersForNode/returns/rootLayer。
                    #[serde(rename = "rootLayer")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub root_layer: crate::css::types::CssLayerData,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetLayersForNodeResponse {
                    root_layer: fields.root_layer,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetLayersForNode {
    type Params = GetLayersForNodeParams;
    type Response = GetLayersForNodeResponse;
    const METHOD: &'static str = "CSS.getLayersForNode";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.getLocationForSelector/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetLocationForSelectorParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getLocationForSelector/parameters/styleSheetId。
    #[serde(rename = "styleSheetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub style_sheet_id: crate::dom::types::StyleSheetId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getLocationForSelector/parameters/selectorText。
    #[serde(rename = "selectorText")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub selector_text: String,
}
impl<'de> serde::Deserialize<'de> for GetLocationForSelectorParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetLocationForSelectorParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getLocationForSelector/parameters/styleSheetId。
                    #[serde(rename = "styleSheetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub style_sheet_id: crate::dom::types::StyleSheetId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getLocationForSelector/parameters/selectorText。
                    #[serde(rename = "selectorText")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub selector_text: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetLocationForSelectorParams {
                    style_sheet_id: fields.style_sheet_id,
                    selector_text: fields.selector_text,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.getLocationForSelector。
///
///协议原文：
///
///<pre>Given a CSS selector text and a style sheet ID, getLocationForSelector
///returns an array of locations of the CSS selector in the style sheet.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetLocationForSelector;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.getLocationForSelector/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetLocationForSelectorResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getLocationForSelector/returns/ranges。
    #[serde(rename = "ranges")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub ranges: Vec<crate::css::types::SourceRange>,
}
impl<'de> serde::Deserialize<'de> for GetLocationForSelectorResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetLocationForSelectorResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getLocationForSelector/returns/ranges。
                    #[serde(rename = "ranges")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub ranges: Vec<crate::css::types::SourceRange>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetLocationForSelectorResponse {
                    ranges: fields.ranges,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetLocationForSelector {
    type Params = GetLocationForSelectorParams;
    type Response = GetLocationForSelectorResponse;
    const METHOD: &'static str = "CSS.getLocationForSelector";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.getLonghandProperties/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetLonghandPropertiesParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getLonghandProperties/parameters/shorthandName。
    #[serde(rename = "shorthandName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub shorthand_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getLonghandProperties/parameters/value。
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: String,
}
impl<'de> serde::Deserialize<'de> for GetLonghandPropertiesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetLonghandPropertiesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getLonghandProperties/parameters/shorthandName。
                    #[serde(rename = "shorthandName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub shorthand_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getLonghandProperties/parameters/value。
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetLonghandPropertiesParams {
                    shorthand_name: fields.shorthand_name,
                    value: fields.value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.getLonghandProperties。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetLonghandProperties;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.getLonghandProperties/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetLonghandPropertiesResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getLonghandProperties/returns/longhandProperties。
    #[serde(rename = "longhandProperties")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub longhand_properties: Vec<crate::css::types::CssProperty>,
}
impl<'de> serde::Deserialize<'de> for GetLonghandPropertiesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetLonghandPropertiesResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getLonghandProperties/returns/longhandProperties。
                    #[serde(rename = "longhandProperties")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub longhand_properties: Vec<crate::css::types::CssProperty>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetLonghandPropertiesResponse {
                    longhand_properties: fields.longhand_properties,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetLonghandProperties {
    type Params = GetLonghandPropertiesParams;
    type Response = GetLonghandPropertiesResponse;
    const METHOD: &'static str = "CSS.getLonghandProperties";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.getMatchedStylesForNode/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetMatchedStylesForNodeParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getMatchedStylesForNode/parameters/nodeId。
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for GetMatchedStylesForNodeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetMatchedStylesForNodeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getMatchedStylesForNode/parameters/nodeId。
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetMatchedStylesForNodeParams {
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.getMatchedStylesForNode。
///
///协议原文：
///
///<pre>Returns requested styles for a DOM node identified by `nodeId`.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetMatchedStylesForNode;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.getMatchedStylesForNode/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetMatchedStylesForNodeResponse {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getMatchedStylesForNode/returns/inlineStyle。
    ///
    ///协议原文：
    ///
    ///<pre>Inline style for the specified DOM node.</pre>
    ///
    ///
    #[serde(rename = "inlineStyle")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub inline_style: Option<crate::css::types::CssStyle>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getMatchedStylesForNode/returns/attributesStyle。
    ///
    ///协议原文：
    ///
    ///<pre>Attribute-defined element style (e.g. resulting from "width=20 height=100%").</pre>
    ///
    ///
    #[serde(rename = "attributesStyle")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub attributes_style: Option<crate::css::types::CssStyle>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getMatchedStylesForNode/returns/matchedCSSRules。
    ///
    ///协议原文：
    ///
    ///<pre>CSS rules matching this node, from all applicable stylesheets.</pre>
    ///
    ///
    #[serde(rename = "matchedCSSRules")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub matched_css_rules: Option<Vec<crate::css::types::RuleMatch>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getMatchedStylesForNode/returns/pseudoElements。
    ///
    ///协议原文：
    ///
    ///<pre>Pseudo style matches for this node.</pre>
    ///
    ///
    #[serde(rename = "pseudoElements")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub pseudo_elements: Option<Vec<crate::css::types::PseudoElementMatches>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getMatchedStylesForNode/returns/inherited。
    ///
    ///协议原文：
    ///
    ///<pre>A chain of inherited styles (from the immediate node parent up to the DOM tree root).</pre>
    ///
    ///
    #[serde(rename = "inherited")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub inherited: Option<Vec<crate::css::types::InheritedStyleEntry>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getMatchedStylesForNode/returns/inheritedPseudoElements。
    ///
    ///协议原文：
    ///
    ///<pre>A chain of inherited pseudo element styles (from the immediate node parent up to the DOM tree root).</pre>
    ///
    ///
    #[serde(rename = "inheritedPseudoElements")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub inherited_pseudo_elements: Option<Vec<crate::css::types::InheritedPseudoElementMatches>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getMatchedStylesForNode/returns/cssKeyframesRules。
    ///
    ///协议原文：
    ///
    ///<pre>A list of CSS keyframed animations matching this node.</pre>
    ///
    ///
    #[serde(rename = "cssKeyframesRules")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub css_keyframes_rules: Option<Vec<crate::css::types::CssKeyframesRule>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getMatchedStylesForNode/returns/cssPositionTryRules。
    ///
    ///协议原文：
    ///
    ///<pre>A list of CSS @position-try rules matching this node, based on the position-try-fallbacks property.</pre>
    ///
    ///
    #[serde(rename = "cssPositionTryRules")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub css_position_try_rules: Option<Vec<crate::css::types::CssPositionTryRule>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getMatchedStylesForNode/returns/activePositionFallbackIndex。
    ///
    ///协议原文：
    ///
    ///<pre>Index of the active fallback in the applied position-try-fallback property,
    ///will not be set if there is no active position-try fallback.</pre>
    ///
    ///
    #[serde(rename = "activePositionFallbackIndex")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub active_position_fallback_index: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getMatchedStylesForNode/returns/cssPropertyRules。
    ///
    ///协议原文：
    ///
    ///<pre>A list of CSS at-property rules matching this node.</pre>
    ///
    ///
    #[serde(rename = "cssPropertyRules")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub css_property_rules: Option<Vec<crate::css::types::CssPropertyRule>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getMatchedStylesForNode/returns/cssPropertyRegistrations。
    ///
    ///协议原文：
    ///
    ///<pre>A list of CSS property registrations matching this node.</pre>
    ///
    ///
    #[serde(rename = "cssPropertyRegistrations")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub css_property_registrations: Option<Vec<crate::css::types::CssPropertyRegistration>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getMatchedStylesForNode/returns/cssAtRules。
    ///
    ///协议原文：
    ///
    ///<pre>A list of simple @rules matching this node or its pseudo-elements.</pre>
    ///
    ///
    #[serde(rename = "cssAtRules")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub css_at_rules: Option<Vec<crate::css::types::CssAtRule>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getMatchedStylesForNode/returns/parentLayoutNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the first parent element that does not have display: contents.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "parentLayoutNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_layout_node_id: Option<crate::dom::types::NodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getMatchedStylesForNode/returns/cssFunctionRules。
    ///
    ///协议原文：
    ///
    ///<pre>A list of CSS at-function rules referenced by styles of this node.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "cssFunctionRules")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub css_function_rules: Option<Vec<crate::css::types::CssFunctionRule>>,
}
impl<'de> serde::Deserialize<'de> for GetMatchedStylesForNodeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetMatchedStylesForNodeResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getMatchedStylesForNode/returns/inlineStyle。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Inline style for the specified DOM node.</pre>
                    ///
                    ///
                    #[serde(rename = "inlineStyle")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub inline_style: Option<crate::css::types::CssStyle>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getMatchedStylesForNode/returns/attributesStyle。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Attribute-defined element style (e.g. resulting from "width=20 height=100%").</pre>
                    ///
                    ///
                    #[serde(rename = "attributesStyle")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub attributes_style: Option<crate::css::types::CssStyle>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getMatchedStylesForNode/returns/matchedCSSRules。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>CSS rules matching this node, from all applicable stylesheets.</pre>
                    ///
                    ///
                    #[serde(rename = "matchedCSSRules")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub matched_css_rules: Option<Vec<crate::css::types::RuleMatch>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getMatchedStylesForNode/returns/pseudoElements。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Pseudo style matches for this node.</pre>
                    ///
                    ///
                    #[serde(rename = "pseudoElements")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub pseudo_elements: Option<Vec<crate::css::types::PseudoElementMatches>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getMatchedStylesForNode/returns/inherited。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A chain of inherited styles (from the immediate node parent up to the DOM tree root).</pre>
                    ///
                    ///
                    #[serde(rename = "inherited")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub inherited: Option<Vec<crate::css::types::InheritedStyleEntry>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getMatchedStylesForNode/returns/inheritedPseudoElements。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A chain of inherited pseudo element styles (from the immediate node parent up to the DOM tree root).</pre>
                    ///
                    ///
                    #[serde(rename = "inheritedPseudoElements")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub inherited_pseudo_elements:
                        Option<Vec<crate::css::types::InheritedPseudoElementMatches>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getMatchedStylesForNode/returns/cssKeyframesRules。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A list of CSS keyframed animations matching this node.</pre>
                    ///
                    ///
                    #[serde(rename = "cssKeyframesRules")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub css_keyframes_rules: Option<Vec<crate::css::types::CssKeyframesRule>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getMatchedStylesForNode/returns/cssPositionTryRules。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A list of CSS @position-try rules matching this node, based on the position-try-fallbacks property.</pre>
                    ///
                    ///
                    #[serde(rename = "cssPositionTryRules")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub css_position_try_rules: Option<Vec<crate::css::types::CssPositionTryRule>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getMatchedStylesForNode/returns/activePositionFallbackIndex。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Index of the active fallback in the applied position-try-fallback property,
                    ///will not be set if there is no active position-try fallback.</pre>
                    ///
                    ///
                    #[serde(rename = "activePositionFallbackIndex")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub active_position_fallback_index: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getMatchedStylesForNode/returns/cssPropertyRules。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A list of CSS at-property rules matching this node.</pre>
                    ///
                    ///
                    #[serde(rename = "cssPropertyRules")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub css_property_rules: Option<Vec<crate::css::types::CssPropertyRule>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getMatchedStylesForNode/returns/cssPropertyRegistrations。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A list of CSS property registrations matching this node.</pre>
                    ///
                    ///
                    #[serde(rename = "cssPropertyRegistrations")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub css_property_registrations:
                        Option<Vec<crate::css::types::CssPropertyRegistration>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getMatchedStylesForNode/returns/cssAtRules。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A list of simple @rules matching this node or its pseudo-elements.</pre>
                    ///
                    ///
                    #[serde(rename = "cssAtRules")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub css_at_rules: Option<Vec<crate::css::types::CssAtRule>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getMatchedStylesForNode/returns/parentLayoutNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the first parent element that does not have display: contents.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "parentLayoutNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub parent_layout_node_id: Option<crate::dom::types::NodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.getMatchedStylesForNode/returns/cssFunctionRules。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A list of CSS at-function rules referenced by styles of this node.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "cssFunctionRules")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub css_function_rules: Option<Vec<crate::css::types::CssFunctionRule>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetMatchedStylesForNodeResponse {
                    inline_style: fields.inline_style,
                    attributes_style: fields.attributes_style,
                    matched_css_rules: fields.matched_css_rules,
                    pseudo_elements: fields.pseudo_elements,
                    inherited: fields.inherited,
                    inherited_pseudo_elements: fields.inherited_pseudo_elements,
                    css_keyframes_rules: fields.css_keyframes_rules,
                    css_position_try_rules: fields.css_position_try_rules,
                    active_position_fallback_index: fields.active_position_fallback_index,
                    css_property_rules: fields.css_property_rules,
                    css_property_registrations: fields.css_property_registrations,
                    css_at_rules: fields.css_at_rules,
                    parent_layout_node_id: fields.parent_layout_node_id,
                    css_function_rules: fields.css_function_rules,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetMatchedStylesForNode {
    type Params = GetMatchedStylesForNodeParams;
    type Response = GetMatchedStylesForNodeResponse;
    const METHOD: &'static str = "CSS.getMatchedStylesForNode";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.getMediaQueries/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetMediaQueriesParams {}
impl<'de> serde::Deserialize<'de> for GetMediaQueriesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetMediaQueriesParams;
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
                Ok(GetMediaQueriesParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.getMediaQueries。
///
///协议原文：
///
///<pre>Returns all media queries parsed by the rendering engine.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetMediaQueries;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.getMediaQueries/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetMediaQueriesResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getMediaQueries/returns/medias。
    #[serde(rename = "medias")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub medias: Vec<crate::css::types::CssMedia>,
}
impl<'de> serde::Deserialize<'de> for GetMediaQueriesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetMediaQueriesResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getMediaQueries/returns/medias。
                    #[serde(rename = "medias")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub medias: Vec<crate::css::types::CssMedia>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetMediaQueriesResponse {
                    medias: fields.medias,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetMediaQueries {
    type Params = GetMediaQueriesParams;
    type Response = GetMediaQueriesResponse;
    const METHOD: &'static str = "CSS.getMediaQueries";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.getPlatformFontsForNode/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetPlatformFontsForNodeParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getPlatformFontsForNode/parameters/nodeId。
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for GetPlatformFontsForNodeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetPlatformFontsForNodeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getPlatformFontsForNode/parameters/nodeId。
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetPlatformFontsForNodeParams {
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.getPlatformFontsForNode。
///
///协议原文：
///
///<pre>Requests information about platform fonts which we used to render child TextNodes in the given
///node.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetPlatformFontsForNode;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.getPlatformFontsForNode/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetPlatformFontsForNodeResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getPlatformFontsForNode/returns/fonts。
    ///
    ///协议原文：
    ///
    ///<pre>Usage statistics for every employed platform font.</pre>
    ///
    ///
    #[serde(rename = "fonts")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub fonts: Vec<crate::css::types::PlatformFontUsage>,
}
impl<'de> serde::Deserialize<'de> for GetPlatformFontsForNodeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetPlatformFontsForNodeResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getPlatformFontsForNode/returns/fonts。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Usage statistics for every employed platform font.</pre>
                    ///
                    ///
                    #[serde(rename = "fonts")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub fonts: Vec<crate::css::types::PlatformFontUsage>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetPlatformFontsForNodeResponse {
                    fonts: fields.fonts,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetPlatformFontsForNode {
    type Params = GetPlatformFontsForNodeParams;
    type Response = GetPlatformFontsForNodeResponse;
    const METHOD: &'static str = "CSS.getPlatformFontsForNode";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.getStyleSheetText/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetStyleSheetTextParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getStyleSheetText/parameters/styleSheetId。
    #[serde(rename = "styleSheetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub style_sheet_id: crate::dom::types::StyleSheetId,
}
impl<'de> serde::Deserialize<'de> for GetStyleSheetTextParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetStyleSheetTextParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getStyleSheetText/parameters/styleSheetId。
                    #[serde(rename = "styleSheetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub style_sheet_id: crate::dom::types::StyleSheetId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetStyleSheetTextParams {
                    style_sheet_id: fields.style_sheet_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.getStyleSheetText。
///
///协议原文：
///
///<pre>Returns the current textual content for a stylesheet.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetStyleSheetText;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.getStyleSheetText/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetStyleSheetTextResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getStyleSheetText/returns/text。
    ///
    ///协议原文：
    ///
    ///<pre>The stylesheet text.</pre>
    ///
    ///
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: String,
}
impl<'de> serde::Deserialize<'de> for GetStyleSheetTextResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetStyleSheetTextResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.getStyleSheetText/returns/text。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The stylesheet text.</pre>
                    ///
                    ///
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetStyleSheetTextResponse { text: fields.text })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetStyleSheetText {
    type Params = GetStyleSheetTextParams;
    type Response = GetStyleSheetTextResponse;
    const METHOD: &'static str = "CSS.getStyleSheetText";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.resolveValues/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ResolveValuesParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.resolveValues/parameters/values。
    ///
    ///协议原文：
    ///
    ///<pre>Cascade-dependent keywords (revert/revert-layer) do not work.</pre>
    ///
    ///
    #[serde(rename = "values")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub values: Vec<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.resolveValues/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node in whose context the expression is evaluated</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.resolveValues/parameters/propertyName。
    ///
    ///协议原文：
    ///
    ///<pre>Only longhands and custom property names are accepted.</pre>
    ///
    ///
    #[serde(rename = "propertyName")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub property_name: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.resolveValues/parameters/pseudoType。
    ///
    ///协议原文：
    ///
    ///<pre>Pseudo element type, only works for pseudo elements that generate
    ///elements in the tree, such as ::before and ::after.</pre>
    ///
    ///
    #[serde(rename = "pseudoType")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub pseudo_type: Option<crate::dom::types::PseudoType>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.resolveValues/parameters/pseudoIdentifier。
    ///
    ///协议原文：
    ///
    ///<pre>Pseudo element custom ident.</pre>
    ///
    ///
    #[serde(rename = "pseudoIdentifier")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub pseudo_identifier: Option<String>,
}
impl<'de> serde::Deserialize<'de> for ResolveValuesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResolveValuesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.resolveValues/parameters/values。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cascade-dependent keywords (revert/revert-layer) do not work.</pre>
                    ///
                    ///
                    #[serde(rename = "values")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub values: Vec<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.resolveValues/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node in whose context the expression is evaluated</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.resolveValues/parameters/propertyName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Only longhands and custom property names are accepted.</pre>
                    ///
                    ///
                    #[serde(rename = "propertyName")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub property_name: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.resolveValues/parameters/pseudoType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Pseudo element type, only works for pseudo elements that generate
                    ///elements in the tree, such as ::before and ::after.</pre>
                    ///
                    ///
                    #[serde(rename = "pseudoType")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub pseudo_type: Option<crate::dom::types::PseudoType>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.resolveValues/parameters/pseudoIdentifier。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Pseudo element custom ident.</pre>
                    ///
                    ///
                    #[serde(rename = "pseudoIdentifier")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub pseudo_identifier: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ResolveValuesParams {
                    values: fields.values,
                    node_id: fields.node_id,
                    property_name: fields.property_name,
                    pseudo_type: fields.pseudo_type,
                    pseudo_identifier: fields.pseudo_identifier,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.resolveValues。
///
///协议原文：
///
///<pre>Resolve the specified values in the context of the provided element.
///For example, a value of '1em' is evaluated according to the computed
///'font-size' of the element and a value 'calc(1px + 2px)' will be
///resolved to '3px'.
///If the `propertyName` was specified the `values` are resolved as if
///they were property's declaration. If a value cannot be parsed according
///to the provided property syntax, the value is parsed using combined
///syntax as if null `propertyName` was provided. If the value cannot be
///resolved even then, return the provided value without any changes.
///Note: this function currently does not resolve CSS random() function,
///it returns unmodified random() function parts.`</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ResolveValues;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.resolveValues/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ResolveValuesResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.resolveValues/returns/results。
    #[serde(rename = "results")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub results: Vec<String>,
}
impl<'de> serde::Deserialize<'de> for ResolveValuesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResolveValuesResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.resolveValues/returns/results。
                    #[serde(rename = "results")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub results: Vec<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ResolveValuesResponse {
                    results: fields.results,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ResolveValues {
    type Params = ResolveValuesParams;
    type Response = ResolveValuesResponse;
    const METHOD: &'static str = "CSS.resolveValues";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.setContainerQueryConditionText/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetContainerQueryConditionTextParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setContainerQueryConditionText/parameters/styleSheetId。
    #[serde(rename = "styleSheetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub style_sheet_id: crate::dom::types::StyleSheetId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setContainerQueryConditionText/parameters/range。
    #[serde(rename = "range")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub range: crate::css::types::SourceRange,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setContainerQueryConditionText/parameters/text。
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: String,
}
impl<'de> serde::Deserialize<'de> for SetContainerQueryConditionTextParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetContainerQueryConditionTextParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setContainerQueryConditionText/parameters/styleSheetId。
                    #[serde(rename = "styleSheetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub style_sheet_id: crate::dom::types::StyleSheetId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setContainerQueryConditionText/parameters/range。
                    #[serde(rename = "range")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub range: crate::css::types::SourceRange,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setContainerQueryConditionText/parameters/text。
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetContainerQueryConditionTextParams {
                    style_sheet_id: fields.style_sheet_id,
                    range: fields.range,
                    text: fields.text,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.setContainerQueryConditionText。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetContainerQueryConditionText;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.setContainerQueryConditionText/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetContainerQueryConditionTextResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setContainerQueryConditionText/returns/containerQuery。
    ///
    ///协议原文：
    ///
    ///<pre>The resulting CSS container query rule after modification.</pre>
    ///
    ///
    #[serde(rename = "containerQuery")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub container_query: crate::css::types::CssContainerQuery,
}
impl<'de> serde::Deserialize<'de> for SetContainerQueryConditionTextResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetContainerQueryConditionTextResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setContainerQueryConditionText/returns/containerQuery。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The resulting CSS container query rule after modification.</pre>
                    ///
                    ///
                    #[serde(rename = "containerQuery")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub container_query: crate::css::types::CssContainerQuery,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetContainerQueryConditionTextResponse {
                    container_query: fields.container_query,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetContainerQueryConditionText {
    type Params = SetContainerQueryConditionTextParams;
    type Response = SetContainerQueryConditionTextResponse;
    const METHOD: &'static str = "CSS.setContainerQueryConditionText";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.setContainerQueryText/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetContainerQueryTextParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setContainerQueryText/parameters/styleSheetId。
    #[serde(rename = "styleSheetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub style_sheet_id: crate::dom::types::StyleSheetId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setContainerQueryText/parameters/range。
    #[serde(rename = "range")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub range: crate::css::types::SourceRange,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setContainerQueryText/parameters/text。
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: String,
}
impl<'de> serde::Deserialize<'de> for SetContainerQueryTextParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetContainerQueryTextParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setContainerQueryText/parameters/styleSheetId。
                    #[serde(rename = "styleSheetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub style_sheet_id: crate::dom::types::StyleSheetId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setContainerQueryText/parameters/range。
                    #[serde(rename = "range")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub range: crate::css::types::SourceRange,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setContainerQueryText/parameters/text。
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetContainerQueryTextParams {
                    style_sheet_id: fields.style_sheet_id,
                    range: fields.range,
                    text: fields.text,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.setContainerQueryText。
///
///协议原文：
///
///<pre>Modifies the expression of a container query.
///Deprecated. Use setContainerQueryConditionText instead.</pre>
///
///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct SetContainerQueryText;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.setContainerQueryText/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetContainerQueryTextResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setContainerQueryText/returns/containerQuery。
    ///
    ///协议原文：
    ///
    ///<pre>The resulting CSS container query rule after modification.</pre>
    ///
    ///
    #[serde(rename = "containerQuery")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub container_query: crate::css::types::CssContainerQuery,
}
impl<'de> serde::Deserialize<'de> for SetContainerQueryTextResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetContainerQueryTextResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setContainerQueryText/returns/containerQuery。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The resulting CSS container query rule after modification.</pre>
                    ///
                    ///
                    #[serde(rename = "containerQuery")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub container_query: crate::css::types::CssContainerQuery,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetContainerQueryTextResponse {
                    container_query: fields.container_query,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetContainerQueryText {
    type Params = SetContainerQueryTextParams;
    type Response = SetContainerQueryTextResponse;
    const METHOD: &'static str = "CSS.setContainerQueryText";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.setEffectivePropertyValueForNode/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetEffectivePropertyValueForNodeParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setEffectivePropertyValueForNode/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>The element id for which to set property.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setEffectivePropertyValueForNode/parameters/propertyName。
    #[serde(rename = "propertyName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub property_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setEffectivePropertyValueForNode/parameters/value。
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: String,
}
impl<'de> serde::Deserialize<'de> for SetEffectivePropertyValueForNodeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetEffectivePropertyValueForNodeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setEffectivePropertyValueForNode/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The element id for which to set property.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setEffectivePropertyValueForNode/parameters/propertyName。
                    #[serde(rename = "propertyName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub property_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setEffectivePropertyValueForNode/parameters/value。
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetEffectivePropertyValueForNodeParams {
                    node_id: fields.node_id,
                    property_name: fields.property_name,
                    value: fields.value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.setEffectivePropertyValueForNode。
///
///协议原文：
///
///<pre>Find a rule with the given active property for the given node and set the new value for this
///property</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetEffectivePropertyValueForNode;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.setEffectivePropertyValueForNode/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetEffectivePropertyValueForNodeResponse {}
impl<'de> serde::Deserialize<'de> for SetEffectivePropertyValueForNodeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetEffectivePropertyValueForNodeResponse;
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
                Ok(SetEffectivePropertyValueForNodeResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetEffectivePropertyValueForNode {
    type Params = SetEffectivePropertyValueForNodeParams;
    type Response = SetEffectivePropertyValueForNodeResponse;
    const METHOD: &'static str = "CSS.setEffectivePropertyValueForNode";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.setKeyframeKey/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetKeyframeKeyParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setKeyframeKey/parameters/styleSheetId。
    #[serde(rename = "styleSheetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub style_sheet_id: crate::dom::types::StyleSheetId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setKeyframeKey/parameters/range。
    #[serde(rename = "range")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub range: crate::css::types::SourceRange,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setKeyframeKey/parameters/keyText。
    #[serde(rename = "keyText")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub key_text: String,
}
impl<'de> serde::Deserialize<'de> for SetKeyframeKeyParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetKeyframeKeyParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setKeyframeKey/parameters/styleSheetId。
                    #[serde(rename = "styleSheetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub style_sheet_id: crate::dom::types::StyleSheetId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setKeyframeKey/parameters/range。
                    #[serde(rename = "range")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub range: crate::css::types::SourceRange,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setKeyframeKey/parameters/keyText。
                    #[serde(rename = "keyText")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub key_text: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetKeyframeKeyParams {
                    style_sheet_id: fields.style_sheet_id,
                    range: fields.range,
                    key_text: fields.key_text,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.setKeyframeKey。
///
///协议原文：
///
///<pre>Modifies the keyframe rule key text.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetKeyframeKey;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.setKeyframeKey/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetKeyframeKeyResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setKeyframeKey/returns/keyText。
    ///
    ///协议原文：
    ///
    ///<pre>The resulting key text after modification.</pre>
    ///
    ///
    #[serde(rename = "keyText")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub key_text: crate::css::types::Value,
}
impl<'de> serde::Deserialize<'de> for SetKeyframeKeyResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetKeyframeKeyResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setKeyframeKey/returns/keyText。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The resulting key text after modification.</pre>
                    ///
                    ///
                    #[serde(rename = "keyText")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub key_text: crate::css::types::Value,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetKeyframeKeyResponse {
                    key_text: fields.key_text,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetKeyframeKey {
    type Params = SetKeyframeKeyParams;
    type Response = SetKeyframeKeyResponse;
    const METHOD: &'static str = "CSS.setKeyframeKey";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.setLocalFontsEnabled/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetLocalFontsEnabledParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setLocalFontsEnabled/parameters/enabled。
    ///
    ///协议原文：
    ///
    ///<pre>Whether rendering of local fonts is enabled.</pre>
    ///
    ///
    #[serde(rename = "enabled")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub enabled: bool,
}
impl<'de> serde::Deserialize<'de> for SetLocalFontsEnabledParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetLocalFontsEnabledParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setLocalFontsEnabled/parameters/enabled。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether rendering of local fonts is enabled.</pre>
                    ///
                    ///
                    #[serde(rename = "enabled")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub enabled: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetLocalFontsEnabledParams {
                    enabled: fields.enabled,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.setLocalFontsEnabled。
///
///协议原文：
///
///<pre>Enables/disables rendering of local CSS fonts (enabled by default).</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetLocalFontsEnabled;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.setLocalFontsEnabled/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetLocalFontsEnabledResponse {}
impl<'de> serde::Deserialize<'de> for SetLocalFontsEnabledResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetLocalFontsEnabledResponse;
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
                Ok(SetLocalFontsEnabledResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetLocalFontsEnabled {
    type Params = SetLocalFontsEnabledParams;
    type Response = SetLocalFontsEnabledResponse;
    const METHOD: &'static str = "CSS.setLocalFontsEnabled";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.setMediaText/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetMediaTextParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setMediaText/parameters/styleSheetId。
    #[serde(rename = "styleSheetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub style_sheet_id: crate::dom::types::StyleSheetId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setMediaText/parameters/range。
    #[serde(rename = "range")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub range: crate::css::types::SourceRange,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setMediaText/parameters/text。
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: String,
}
impl<'de> serde::Deserialize<'de> for SetMediaTextParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetMediaTextParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setMediaText/parameters/styleSheetId。
                    #[serde(rename = "styleSheetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub style_sheet_id: crate::dom::types::StyleSheetId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setMediaText/parameters/range。
                    #[serde(rename = "range")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub range: crate::css::types::SourceRange,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setMediaText/parameters/text。
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetMediaTextParams {
                    style_sheet_id: fields.style_sheet_id,
                    range: fields.range,
                    text: fields.text,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.setMediaText。
///
///协议原文：
///
///<pre>Modifies the rule selector.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetMediaText;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.setMediaText/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetMediaTextResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setMediaText/returns/media。
    ///
    ///协议原文：
    ///
    ///<pre>The resulting CSS media rule after modification.</pre>
    ///
    ///
    #[serde(rename = "media")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub media: crate::css::types::CssMedia,
}
impl<'de> serde::Deserialize<'de> for SetMediaTextResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetMediaTextResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setMediaText/returns/media。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The resulting CSS media rule after modification.</pre>
                    ///
                    ///
                    #[serde(rename = "media")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub media: crate::css::types::CssMedia,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetMediaTextResponse {
                    media: fields.media,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetMediaText {
    type Params = SetMediaTextParams;
    type Response = SetMediaTextResponse;
    const METHOD: &'static str = "CSS.setMediaText";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.setNavigationText/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetNavigationTextParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setNavigationText/parameters/styleSheetId。
    #[serde(rename = "styleSheetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub style_sheet_id: crate::dom::types::StyleSheetId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setNavigationText/parameters/range。
    #[serde(rename = "range")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub range: crate::css::types::SourceRange,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setNavigationText/parameters/text。
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: String,
}
impl<'de> serde::Deserialize<'de> for SetNavigationTextParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetNavigationTextParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setNavigationText/parameters/styleSheetId。
                    #[serde(rename = "styleSheetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub style_sheet_id: crate::dom::types::StyleSheetId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setNavigationText/parameters/range。
                    #[serde(rename = "range")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub range: crate::css::types::SourceRange,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setNavigationText/parameters/text。
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetNavigationTextParams {
                    style_sheet_id: fields.style_sheet_id,
                    range: fields.range,
                    text: fields.text,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.setNavigationText。
///
///协议原文：
///
///<pre>Modifies the expression of a navigation at-rule.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetNavigationText;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.setNavigationText/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetNavigationTextResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setNavigationText/returns/navigation。
    ///
    ///协议原文：
    ///
    ///<pre>The resulting CSS Navigation rule after modification.</pre>
    ///
    ///
    #[serde(rename = "navigation")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub navigation: crate::css::types::CssNavigation,
}
impl<'de> serde::Deserialize<'de> for SetNavigationTextResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetNavigationTextResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setNavigationText/returns/navigation。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The resulting CSS Navigation rule after modification.</pre>
                    ///
                    ///
                    #[serde(rename = "navigation")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub navigation: crate::css::types::CssNavigation,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetNavigationTextResponse {
                    navigation: fields.navigation,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetNavigationText {
    type Params = SetNavigationTextParams;
    type Response = SetNavigationTextResponse;
    const METHOD: &'static str = "CSS.setNavigationText";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.setPropertyRulePropertyName/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetPropertyRulePropertyNameParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setPropertyRulePropertyName/parameters/styleSheetId。
    #[serde(rename = "styleSheetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub style_sheet_id: crate::dom::types::StyleSheetId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setPropertyRulePropertyName/parameters/range。
    #[serde(rename = "range")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub range: crate::css::types::SourceRange,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setPropertyRulePropertyName/parameters/propertyName。
    #[serde(rename = "propertyName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub property_name: String,
}
impl<'de> serde::Deserialize<'de> for SetPropertyRulePropertyNameParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetPropertyRulePropertyNameParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setPropertyRulePropertyName/parameters/styleSheetId。
                    #[serde(rename = "styleSheetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub style_sheet_id: crate::dom::types::StyleSheetId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setPropertyRulePropertyName/parameters/range。
                    #[serde(rename = "range")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub range: crate::css::types::SourceRange,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setPropertyRulePropertyName/parameters/propertyName。
                    #[serde(rename = "propertyName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub property_name: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetPropertyRulePropertyNameParams {
                    style_sheet_id: fields.style_sheet_id,
                    range: fields.range,
                    property_name: fields.property_name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.setPropertyRulePropertyName。
///
///协议原文：
///
///<pre>Modifies the property rule property name.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetPropertyRulePropertyName;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.setPropertyRulePropertyName/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetPropertyRulePropertyNameResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setPropertyRulePropertyName/returns/propertyName。
    ///
    ///协议原文：
    ///
    ///<pre>The resulting key text after modification.</pre>
    ///
    ///
    #[serde(rename = "propertyName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub property_name: crate::css::types::Value,
}
impl<'de> serde::Deserialize<'de> for SetPropertyRulePropertyNameResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetPropertyRulePropertyNameResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setPropertyRulePropertyName/returns/propertyName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The resulting key text after modification.</pre>
                    ///
                    ///
                    #[serde(rename = "propertyName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub property_name: crate::css::types::Value,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetPropertyRulePropertyNameResponse {
                    property_name: fields.property_name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetPropertyRulePropertyName {
    type Params = SetPropertyRulePropertyNameParams;
    type Response = SetPropertyRulePropertyNameResponse;
    const METHOD: &'static str = "CSS.setPropertyRulePropertyName";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.setRuleSelector/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetRuleSelectorParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setRuleSelector/parameters/styleSheetId。
    #[serde(rename = "styleSheetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub style_sheet_id: crate::dom::types::StyleSheetId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setRuleSelector/parameters/range。
    #[serde(rename = "range")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub range: crate::css::types::SourceRange,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setRuleSelector/parameters/selector。
    #[serde(rename = "selector")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub selector: String,
}
impl<'de> serde::Deserialize<'de> for SetRuleSelectorParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetRuleSelectorParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setRuleSelector/parameters/styleSheetId。
                    #[serde(rename = "styleSheetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub style_sheet_id: crate::dom::types::StyleSheetId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setRuleSelector/parameters/range。
                    #[serde(rename = "range")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub range: crate::css::types::SourceRange,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setRuleSelector/parameters/selector。
                    #[serde(rename = "selector")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub selector: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetRuleSelectorParams {
                    style_sheet_id: fields.style_sheet_id,
                    range: fields.range,
                    selector: fields.selector,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.setRuleSelector。
///
///协议原文：
///
///<pre>Modifies the rule selector.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetRuleSelector;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.setRuleSelector/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetRuleSelectorResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setRuleSelector/returns/selectorList。
    ///
    ///协议原文：
    ///
    ///<pre>The resulting selector list after modification.</pre>
    ///
    ///
    #[serde(rename = "selectorList")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub selector_list: crate::css::types::SelectorList,
}
impl<'de> serde::Deserialize<'de> for SetRuleSelectorResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetRuleSelectorResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setRuleSelector/returns/selectorList。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The resulting selector list after modification.</pre>
                    ///
                    ///
                    #[serde(rename = "selectorList")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub selector_list: crate::css::types::SelectorList,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetRuleSelectorResponse {
                    selector_list: fields.selector_list,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetRuleSelector {
    type Params = SetRuleSelectorParams;
    type Response = SetRuleSelectorResponse;
    const METHOD: &'static str = "CSS.setRuleSelector";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.setScopeText/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetScopeTextParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setScopeText/parameters/styleSheetId。
    #[serde(rename = "styleSheetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub style_sheet_id: crate::dom::types::StyleSheetId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setScopeText/parameters/range。
    #[serde(rename = "range")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub range: crate::css::types::SourceRange,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setScopeText/parameters/text。
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: String,
}
impl<'de> serde::Deserialize<'de> for SetScopeTextParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetScopeTextParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setScopeText/parameters/styleSheetId。
                    #[serde(rename = "styleSheetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub style_sheet_id: crate::dom::types::StyleSheetId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setScopeText/parameters/range。
                    #[serde(rename = "range")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub range: crate::css::types::SourceRange,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setScopeText/parameters/text。
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetScopeTextParams {
                    style_sheet_id: fields.style_sheet_id,
                    range: fields.range,
                    text: fields.text,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.setScopeText。
///
///协议原文：
///
///<pre>Modifies the expression of a scope at-rule.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetScopeText;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.setScopeText/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetScopeTextResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setScopeText/returns/scope。
    ///
    ///协议原文：
    ///
    ///<pre>The resulting CSS Scope rule after modification.</pre>
    ///
    ///
    #[serde(rename = "scope")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub scope: crate::css::types::CssScope,
}
impl<'de> serde::Deserialize<'de> for SetScopeTextResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetScopeTextResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setScopeText/returns/scope。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The resulting CSS Scope rule after modification.</pre>
                    ///
                    ///
                    #[serde(rename = "scope")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub scope: crate::css::types::CssScope,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetScopeTextResponse {
                    scope: fields.scope,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetScopeText {
    type Params = SetScopeTextParams;
    type Response = SetScopeTextResponse;
    const METHOD: &'static str = "CSS.setScopeText";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.setStyleSheetText/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetStyleSheetTextParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setStyleSheetText/parameters/styleSheetId。
    #[serde(rename = "styleSheetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub style_sheet_id: crate::dom::types::StyleSheetId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setStyleSheetText/parameters/text。
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: String,
}
impl<'de> serde::Deserialize<'de> for SetStyleSheetTextParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetStyleSheetTextParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setStyleSheetText/parameters/styleSheetId。
                    #[serde(rename = "styleSheetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub style_sheet_id: crate::dom::types::StyleSheetId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setStyleSheetText/parameters/text。
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetStyleSheetTextParams {
                    style_sheet_id: fields.style_sheet_id,
                    text: fields.text,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.setStyleSheetText。
///
///协议原文：
///
///<pre>Sets the new stylesheet text.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetStyleSheetText;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.setStyleSheetText/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetStyleSheetTextResponse {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.setStyleSheetText/returns/sourceMapURL。
    ///
    ///协议原文：
    ///
    ///<pre>URL of source map associated with script (if any).</pre>
    ///
    ///
    #[serde(rename = "sourceMapURL")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_map_url: Option<String>,
}
impl<'de> serde::Deserialize<'de> for SetStyleSheetTextResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetStyleSheetTextResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.setStyleSheetText/returns/sourceMapURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>URL of source map associated with script (if any).</pre>
                    ///
                    ///
                    #[serde(rename = "sourceMapURL")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub source_map_url: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetStyleSheetTextResponse {
                    source_map_url: fields.source_map_url,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetStyleSheetText {
    type Params = SetStyleSheetTextParams;
    type Response = SetStyleSheetTextResponse;
    const METHOD: &'static str = "CSS.setStyleSheetText";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.setStyleTexts/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetStyleTextsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setStyleTexts/parameters/edits。
    #[serde(rename = "edits")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub edits: Vec<crate::css::types::StyleDeclarationEdit>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.setStyleTexts/parameters/nodeForPropertySyntaxValidation。
    ///
    ///协议原文：
    ///
    ///<pre>NodeId for the DOM node in whose context custom property declarations for registered properties should be
    ///validated. If omitted, declarations in the new rule text can only be validated statically, which may produce
    ///incorrect results if the declaration contains a var() for example.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "nodeForPropertySyntaxValidation")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub node_for_property_syntax_validation: Option<crate::dom::types::NodeId>,
}
impl<'de> serde::Deserialize<'de> for SetStyleTextsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetStyleTextsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setStyleTexts/parameters/edits。
                    #[serde(rename = "edits")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub edits: Vec<crate::css::types::StyleDeclarationEdit>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.setStyleTexts/parameters/nodeForPropertySyntaxValidation。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>NodeId for the DOM node in whose context custom property declarations for registered properties should be
                    ///validated. If omitted, declarations in the new rule text can only be validated statically, which may produce
                    ///incorrect results if the declaration contains a var() for example.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "nodeForPropertySyntaxValidation")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub node_for_property_syntax_validation: Option<crate::dom::types::NodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetStyleTextsParams {
                    edits: fields.edits,
                    node_for_property_syntax_validation: fields.node_for_property_syntax_validation,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.setStyleTexts。
///
///协议原文：
///
///<pre>Applies specified style edits one after another in the given order.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetStyleTexts;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.setStyleTexts/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetStyleTextsResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setStyleTexts/returns/styles。
    ///
    ///协议原文：
    ///
    ///<pre>The resulting styles after modification.</pre>
    ///
    ///
    #[serde(rename = "styles")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub styles: Vec<crate::css::types::CssStyle>,
}
impl<'de> serde::Deserialize<'de> for SetStyleTextsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetStyleTextsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setStyleTexts/returns/styles。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The resulting styles after modification.</pre>
                    ///
                    ///
                    #[serde(rename = "styles")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub styles: Vec<crate::css::types::CssStyle>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetStyleTextsResponse {
                    styles: fields.styles,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetStyleTexts {
    type Params = SetStyleTextsParams;
    type Response = SetStyleTextsResponse;
    const METHOD: &'static str = "CSS.setStyleTexts";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.setSupportsText/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetSupportsTextParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setSupportsText/parameters/styleSheetId。
    #[serde(rename = "styleSheetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub style_sheet_id: crate::dom::types::StyleSheetId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setSupportsText/parameters/range。
    #[serde(rename = "range")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub range: crate::css::types::SourceRange,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setSupportsText/parameters/text。
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: String,
}
impl<'de> serde::Deserialize<'de> for SetSupportsTextParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetSupportsTextParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setSupportsText/parameters/styleSheetId。
                    #[serde(rename = "styleSheetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub style_sheet_id: crate::dom::types::StyleSheetId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setSupportsText/parameters/range。
                    #[serde(rename = "range")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub range: crate::css::types::SourceRange,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setSupportsText/parameters/text。
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetSupportsTextParams {
                    style_sheet_id: fields.style_sheet_id,
                    range: fields.range,
                    text: fields.text,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.setSupportsText。
///
///协议原文：
///
///<pre>Modifies the expression of a supports at-rule.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetSupportsText;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.setSupportsText/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetSupportsTextResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setSupportsText/returns/supports。
    ///
    ///协议原文：
    ///
    ///<pre>The resulting CSS Supports rule after modification.</pre>
    ///
    ///
    #[serde(rename = "supports")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub supports: crate::css::types::CssSupports,
}
impl<'de> serde::Deserialize<'de> for SetSupportsTextResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetSupportsTextResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.setSupportsText/returns/supports。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The resulting CSS Supports rule after modification.</pre>
                    ///
                    ///
                    #[serde(rename = "supports")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub supports: crate::css::types::CssSupports,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetSupportsTextResponse {
                    supports: fields.supports,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetSupportsText {
    type Params = SetSupportsTextParams;
    type Response = SetSupportsTextResponse;
    const METHOD: &'static str = "CSS.setSupportsText";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.startRuleUsageTracking/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct StartRuleUsageTrackingParams {}
impl<'de> serde::Deserialize<'de> for StartRuleUsageTrackingParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StartRuleUsageTrackingParams;
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
                Ok(StartRuleUsageTrackingParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.startRuleUsageTracking。
///
///协议原文：
///
///<pre>Enables the selector recording.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct StartRuleUsageTracking;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.startRuleUsageTracking/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StartRuleUsageTrackingResponse {}
impl<'de> serde::Deserialize<'de> for StartRuleUsageTrackingResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StartRuleUsageTrackingResponse;
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
                Ok(StartRuleUsageTrackingResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for StartRuleUsageTracking {
    type Params = StartRuleUsageTrackingParams;
    type Response = StartRuleUsageTrackingResponse;
    const METHOD: &'static str = "CSS.startRuleUsageTracking";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.stopRuleUsageTracking/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct StopRuleUsageTrackingParams {}
impl<'de> serde::Deserialize<'de> for StopRuleUsageTrackingParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StopRuleUsageTrackingParams;
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
                Ok(StopRuleUsageTrackingParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.stopRuleUsageTracking。
///
///协议原文：
///
///<pre>Stop tracking rule usage and return the list of rules that were used since last call to
///`takeCoverageDelta` (or since start of coverage instrumentation).</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct StopRuleUsageTracking;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.stopRuleUsageTracking/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StopRuleUsageTrackingResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.stopRuleUsageTracking/returns/ruleUsage。
    #[serde(rename = "ruleUsage")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub rule_usage: Vec<crate::css::types::RuleUsage>,
}
impl<'de> serde::Deserialize<'de> for StopRuleUsageTrackingResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StopRuleUsageTrackingResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.stopRuleUsageTracking/returns/ruleUsage。
                    #[serde(rename = "ruleUsage")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub rule_usage: Vec<crate::css::types::RuleUsage>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StopRuleUsageTrackingResponse {
                    rule_usage: fields.rule_usage,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for StopRuleUsageTracking {
    type Params = StopRuleUsageTrackingParams;
    type Response = StopRuleUsageTrackingResponse;
    const METHOD: &'static str = "CSS.stopRuleUsageTracking";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.takeComputedStyleUpdates/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct TakeComputedStyleUpdatesParams {}
impl<'de> serde::Deserialize<'de> for TakeComputedStyleUpdatesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TakeComputedStyleUpdatesParams;
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
                Ok(TakeComputedStyleUpdatesParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.takeComputedStyleUpdates。
///
///协议原文：
///
///<pre>Polls the next batch of computed style updates.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct TakeComputedStyleUpdates;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.takeComputedStyleUpdates/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TakeComputedStyleUpdatesResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.takeComputedStyleUpdates/returns/nodeIds。
    ///
    ///协议原文：
    ///
    ///<pre>The list of node Ids that have their tracked computed styles updated.</pre>
    ///
    ///
    #[serde(rename = "nodeIds")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_ids: Vec<crate::dom::types::NodeId>,
}
impl<'de> serde::Deserialize<'de> for TakeComputedStyleUpdatesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TakeComputedStyleUpdatesResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.takeComputedStyleUpdates/returns/nodeIds。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The list of node Ids that have their tracked computed styles updated.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeIds")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_ids: Vec<crate::dom::types::NodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TakeComputedStyleUpdatesResponse {
                    node_ids: fields.node_ids,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for TakeComputedStyleUpdates {
    type Params = TakeComputedStyleUpdatesParams;
    type Response = TakeComputedStyleUpdatesResponse;
    const METHOD: &'static str = "CSS.takeComputedStyleUpdates";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.takeCoverageDelta/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct TakeCoverageDeltaParams {}
impl<'de> serde::Deserialize<'de> for TakeCoverageDeltaParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TakeCoverageDeltaParams;
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
                Ok(TakeCoverageDeltaParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.takeCoverageDelta。
///
///协议原文：
///
///<pre>Obtain list of rules that became used since last call to this method (or since start of coverage
///instrumentation).</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct TakeCoverageDelta;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.takeCoverageDelta/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TakeCoverageDeltaResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.takeCoverageDelta/returns/coverage。
    #[serde(rename = "coverage")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub coverage: Vec<crate::css::types::RuleUsage>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.takeCoverageDelta/returns/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Monotonically increasing time, in seconds.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for TakeCoverageDeltaResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TakeCoverageDeltaResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.takeCoverageDelta/returns/coverage。
                    #[serde(rename = "coverage")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub coverage: Vec<crate::css::types::RuleUsage>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.takeCoverageDelta/returns/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Monotonically increasing time, in seconds.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TakeCoverageDeltaResponse {
                    coverage: fields.coverage,
                    timestamp: fields.timestamp,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for TakeCoverageDelta {
    type Params = TakeCoverageDeltaParams;
    type Response = TakeCoverageDeltaResponse;
    const METHOD: &'static str = "CSS.takeCoverageDelta";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.trackComputedStyleUpdates/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TrackComputedStyleUpdatesParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.trackComputedStyleUpdates/parameters/propertiesToTrack。
    #[serde(rename = "propertiesToTrack")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub properties_to_track: Vec<crate::css::types::CssComputedStyleProperty>,
}
impl<'de> serde::Deserialize<'de> for TrackComputedStyleUpdatesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TrackComputedStyleUpdatesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.command.trackComputedStyleUpdates/parameters/propertiesToTrack。
                    #[serde(rename = "propertiesToTrack")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub properties_to_track: Vec<crate::css::types::CssComputedStyleProperty>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TrackComputedStyleUpdatesParams {
                    properties_to_track: fields.properties_to_track,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.trackComputedStyleUpdates。
///
///协议原文：
///
///<pre>Starts tracking the given computed styles for updates. The specified array of properties
///replaces the one previously specified. Pass empty array to disable tracking.
///Use takeComputedStyleUpdates to retrieve the list of nodes that had properties modified.
///The changes to computed style properties are only tracked for nodes pushed to the front-end
///by the DOM agent. If no changes to the tracked properties occur after the node has been pushed
///to the front-end, no updates will be issued for the node.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct TrackComputedStyleUpdates;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.trackComputedStyleUpdates/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TrackComputedStyleUpdatesResponse {}
impl<'de> serde::Deserialize<'de> for TrackComputedStyleUpdatesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TrackComputedStyleUpdatesResponse;
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
                Ok(TrackComputedStyleUpdatesResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for TrackComputedStyleUpdates {
    type Params = TrackComputedStyleUpdatesParams;
    type Response = TrackComputedStyleUpdatesResponse;
    const METHOD: &'static str = "CSS.trackComputedStyleUpdates";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.trackComputedStyleUpdatesForNode/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct TrackComputedStyleUpdatesForNodeParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.trackComputedStyleUpdatesForNode/parameters/nodeId。
    #[serde(rename = "nodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub node_id: Option<crate::dom::types::NodeId>,
}
impl<'de> serde::Deserialize<'de> for TrackComputedStyleUpdatesForNodeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TrackComputedStyleUpdatesForNodeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.command.trackComputedStyleUpdatesForNode/parameters/nodeId。
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
                Ok(TrackComputedStyleUpdatesForNodeParams {
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CSS.command.trackComputedStyleUpdatesForNode。
///
///协议原文：
///
///<pre>Starts tracking the given node for the computed style updates
///and whenever the computed style is updated for node, it queues
///a `computedStyleUpdated` event with throttling.
///There can only be 1 node tracked for computed style updates
///so passing a new node id removes tracking from the previous node.
///Pass `undefined` to disable tracking.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct TrackComputedStyleUpdatesForNode;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.command.trackComputedStyleUpdatesForNode/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TrackComputedStyleUpdatesForNodeResponse {}
impl<'de> serde::Deserialize<'de> for TrackComputedStyleUpdatesForNodeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TrackComputedStyleUpdatesForNodeResponse;
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
                Ok(TrackComputedStyleUpdatesForNodeResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for TrackComputedStyleUpdatesForNode {
    type Params = TrackComputedStyleUpdatesForNodeParams;
    type Response = TrackComputedStyleUpdatesForNodeResponse;
    const METHOD: &'static str = "CSS.trackComputedStyleUpdatesForNode";
}
