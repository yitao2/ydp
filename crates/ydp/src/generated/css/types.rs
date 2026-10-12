///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.CSSAnimationStyle。
///
///协议原文：
///
///<pre>CSS style coming from animations with the name of the animation.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CssAnimationStyle {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSAnimationStyle/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>The name of the animation.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSAnimationStyle/properties/style。
    ///
    ///协议原文：
    ///
    ///<pre>The style coming from the animation.</pre>
    ///
    ///
    #[serde(rename = "style")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub style: crate::css::types::CssStyle,
}
impl<'de> serde::Deserialize<'de> for CssAnimationStyle {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssAnimationStyle;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSAnimationStyle/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The name of the animation.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub name: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSAnimationStyle/properties/style。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The style coming from the animation.</pre>
                    ///
                    ///
                    #[serde(rename = "style")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub style: crate::css::types::CssStyle,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CssAnimationStyle {
                    name: fields.name,
                    style: fields.style,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：CSS.type.CSSAtRule/properties/type。
///
///协议原文：
///
///<pre>Type of at-rule.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CssAtRuleType {
    ///协议值 "font-face"。
    FontFace,
    ///协议值 "font-feature-values"。
    FontFeatureValues,
    ///协议值 "font-palette-values"。
    FontPaletteValues,
    ///协议值 "counter-style"。
    CounterStyle,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl CssAtRuleType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::css::types::CssAtRuleType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::FontFace => "font-face",
            Self::FontFeatureValues => "font-feature-values",
            Self::FontPaletteValues => "font-palette-values",
            Self::CounterStyle => "counter-style",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for CssAtRuleType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for CssAtRuleType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "font-face" => CssAtRuleType::FontFace,
            "font-feature-values" => CssAtRuleType::FontFeatureValues,
            "font-palette-values" => CssAtRuleType::FontPaletteValues,
            "counter-style" => CssAtRuleType::CounterStyle,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for CssAtRuleType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CssAtRuleType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssAtRuleType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "font-face" => CssAtRuleType::FontFace,
                    "font-feature-values" => CssAtRuleType::FontFeatureValues,
                    "font-palette-values" => CssAtRuleType::FontPaletteValues,
                    "counter-style" => CssAtRuleType::CounterStyle,
                    _ => CssAtRuleType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(CssAtRuleType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：CSS.type.CSSAtRule/properties/subsection。
///
///协议原文：
///
///<pre>Subsection of font-feature-values, if this is a subsection.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CssAtRuleSubsection {
    ///协议值 "swash"。
    Swash,
    ///协议值 "annotation"。
    Annotation,
    ///协议值 "ornaments"。
    Ornaments,
    ///协议值 "stylistic"。
    Stylistic,
    ///协议值 "styleset"。
    Styleset,
    ///协议值 "character-variant"。
    CharacterVariant,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl CssAtRuleSubsection {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::css::types::CssAtRuleSubsection::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Swash => "swash",
            Self::Annotation => "annotation",
            Self::Ornaments => "ornaments",
            Self::Stylistic => "stylistic",
            Self::Styleset => "styleset",
            Self::CharacterVariant => "character-variant",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for CssAtRuleSubsection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for CssAtRuleSubsection {
    fn from(value: String) -> Self {
        match value.as_str() {
            "swash" => CssAtRuleSubsection::Swash,
            "annotation" => CssAtRuleSubsection::Annotation,
            "ornaments" => CssAtRuleSubsection::Ornaments,
            "stylistic" => CssAtRuleSubsection::Stylistic,
            "styleset" => CssAtRuleSubsection::Styleset,
            "character-variant" => CssAtRuleSubsection::CharacterVariant,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for CssAtRuleSubsection {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CssAtRuleSubsection {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssAtRuleSubsection;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "swash" => CssAtRuleSubsection::Swash,
                    "annotation" => CssAtRuleSubsection::Annotation,
                    "ornaments" => CssAtRuleSubsection::Ornaments,
                    "stylistic" => CssAtRuleSubsection::Stylistic,
                    "styleset" => CssAtRuleSubsection::Styleset,
                    "character-variant" => CssAtRuleSubsection::CharacterVariant,
                    _ => CssAtRuleSubsection::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(CssAtRuleSubsection::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.CSSAtRule。
///
///协议原文：
///
///<pre>CSS generic @rule representation.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CssAtRule {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSAtRule/properties/type。
    ///
    ///协议原文：
    ///
    ///<pre>Type of at-rule.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: CssAtRuleType,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSAtRule/properties/subsection。
    ///
    ///协议原文：
    ///
    ///<pre>Subsection of font-feature-values, if this is a subsection.</pre>
    ///
    ///
    #[serde(rename = "subsection")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub subsection: Option<CssAtRuleSubsection>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSAtRule/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>LINT.ThenChange(//third_party/blink/renderer/core/inspector/inspector_style_sheet.cc:FontVariantAlternatesFeatureType,//third_party/blink/renderer/core/inspector/inspector_css_agent.cc:FontVariantAlternatesFeatureType)
    ///Associated name, if applicable.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<crate::css::types::Value>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSAtRule/properties/styleSheetId。
    ///
    ///协议原文：
    ///
    ///<pre>The css style sheet identifier (absent for user agent stylesheet and user-specified
    ///stylesheet rules) this rule came from.</pre>
    ///
    ///
    #[serde(rename = "styleSheetId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSAtRule/properties/origin。
    ///
    ///协议原文：
    ///
    ///<pre>Parent stylesheet's origin.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: crate::css::types::StyleSheetOrigin,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSAtRule/properties/style。
    ///
    ///协议原文：
    ///
    ///<pre>Associated style declaration.</pre>
    ///
    ///
    #[serde(rename = "style")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub style: crate::css::types::CssStyle,
}
impl<'de> serde::Deserialize<'de> for CssAtRule {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssAtRule;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSAtRule/properties/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Type of at-rule.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: CssAtRuleType,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSAtRule/properties/subsection。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Subsection of font-feature-values, if this is a subsection.</pre>
                    ///
                    ///
                    #[serde(rename = "subsection")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub subsection: Option<CssAtRuleSubsection>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSAtRule/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>LINT.ThenChange(//third_party/blink/renderer/core/inspector/inspector_style_sheet.cc:FontVariantAlternatesFeatureType,//third_party/blink/renderer/core/inspector/inspector_css_agent.cc:FontVariantAlternatesFeatureType)
                    ///Associated name, if applicable.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub name: Option<crate::css::types::Value>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSAtRule/properties/styleSheetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The css style sheet identifier (absent for user agent stylesheet and user-specified
                    ///stylesheet rules) this rule came from.</pre>
                    ///
                    ///
                    #[serde(rename = "styleSheetId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSAtRule/properties/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Parent stylesheet's origin.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: crate::css::types::StyleSheetOrigin,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSAtRule/properties/style。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Associated style declaration.</pre>
                    ///
                    ///
                    #[serde(rename = "style")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub style: crate::css::types::CssStyle,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CssAtRule {
                    r#type: fields.r#type,
                    subsection: fields.subsection,
                    name: fields.name,
                    style_sheet_id: fields.style_sheet_id,
                    origin: fields.origin,
                    style: fields.style,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.CSSComputedStyleProperty。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CssComputedStyleProperty {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSComputedStyleProperty/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Computed style property name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSComputedStyleProperty/properties/value。
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
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSComputedStyleProperty/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Computed style property name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSComputedStyleProperty/properties/value。
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
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.CSSContainerQuery。
///
///协议原文：
///
///<pre>CSS container query rule descriptor.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CssContainerQuery {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSContainerQuery/properties/text。
    ///
    ///协议原文：
    ///
    ///<pre>Container query text.
    ///Contains the query part without the container name for a single query.
    ///Deprecated in favor of conditionText which contains the full prelude
    ///after @container.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSContainerQuery/properties/range。
    ///
    ///协议原文：
    ///
    ///<pre>The associated rule header range in the enclosing stylesheet (if
    ///available).</pre>
    ///
    ///
    #[serde(rename = "range")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub range: Option<crate::css::types::SourceRange>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSContainerQuery/properties/styleSheetId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the stylesheet containing this object (if exists).</pre>
    ///
    ///
    #[serde(rename = "styleSheetId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSContainerQuery/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Optional name for the container.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSContainerQuery/properties/physicalAxes。
    ///
    ///协议原文：
    ///
    ///<pre>Optional physical axes queried for the container.</pre>
    ///
    ///
    #[serde(rename = "physicalAxes")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub physical_axes: Option<crate::dom::types::PhysicalAxes>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSContainerQuery/properties/logicalAxes。
    ///
    ///协议原文：
    ///
    ///<pre>Optional logical axes queried for the container.</pre>
    ///
    ///
    #[serde(rename = "logicalAxes")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub logical_axes: Option<crate::dom::types::LogicalAxes>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSContainerQuery/properties/queriesScrollState。
    ///
    ///协议原文：
    ///
    ///<pre>true if the query contains scroll-state() queries.</pre>
    ///
    ///
    #[serde(rename = "queriesScrollState")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub queries_scroll_state: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSContainerQuery/properties/queriesAnchored。
    ///
    ///协议原文：
    ///
    ///<pre>true if the query contains anchored() queries.</pre>
    ///
    ///
    #[serde(rename = "queriesAnchored")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub queries_anchored: Option<bool>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSContainerQuery/properties/conditionText。
    ///
    ///协议原文：
    ///
    ///<pre>CSSContainerRule.conditionText</pre>
    ///
    ///
    #[serde(rename = "conditionText")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub condition_text: String,
}
impl<'de> serde::Deserialize<'de> for CssContainerQuery {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssContainerQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSContainerQuery/properties/text。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Container query text.
                    ///Contains the query part without the container name for a single query.
                    ///Deprecated in favor of conditionText which contains the full prelude
                    ///after @container.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSContainerQuery/properties/range。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The associated rule header range in the enclosing stylesheet (if
                    ///available).</pre>
                    ///
                    ///
                    #[serde(rename = "range")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub range: Option<crate::css::types::SourceRange>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSContainerQuery/properties/styleSheetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the stylesheet containing this object (if exists).</pre>
                    ///
                    ///
                    #[serde(rename = "styleSheetId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSContainerQuery/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Optional name for the container.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub name: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSContainerQuery/properties/physicalAxes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Optional physical axes queried for the container.</pre>
                    ///
                    ///
                    #[serde(rename = "physicalAxes")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub physical_axes: Option<crate::dom::types::PhysicalAxes>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSContainerQuery/properties/logicalAxes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Optional logical axes queried for the container.</pre>
                    ///
                    ///
                    #[serde(rename = "logicalAxes")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub logical_axes: Option<crate::dom::types::LogicalAxes>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSContainerQuery/properties/queriesScrollState。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>true if the query contains scroll-state() queries.</pre>
                    ///
                    ///
                    #[serde(rename = "queriesScrollState")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub queries_scroll_state: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSContainerQuery/properties/queriesAnchored。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>true if the query contains anchored() queries.</pre>
                    ///
                    ///
                    #[serde(rename = "queriesAnchored")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub queries_anchored: Option<bool>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSContainerQuery/properties/conditionText。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>CSSContainerRule.conditionText</pre>
                    ///
                    ///
                    #[serde(rename = "conditionText")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub condition_text: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CssContainerQuery {
                    text: fields.text,
                    range: fields.range,
                    style_sheet_id: fields.style_sheet_id,
                    name: fields.name,
                    physical_axes: fields.physical_axes,
                    logical_axes: fields.logical_axes,
                    queries_scroll_state: fields.queries_scroll_state,
                    queries_anchored: fields.queries_anchored,
                    condition_text: fields.condition_text,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.CSSFunctionConditionNode。
///
///协议原文：
///
///<pre>CSS function conditional block representation.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CssFunctionConditionNode {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSFunctionConditionNode/properties/media。
    ///
    ///协议原文：
    ///
    ///<pre>Media query for this conditional block. Only one type of condition should be set.</pre>
    ///
    ///
    #[serde(rename = "media")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub media: Option<crate::css::types::CssMedia>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSFunctionConditionNode/properties/containerQueries。
    ///
    ///协议原文：
    ///
    ///<pre>Container query for this conditional block. Only one type of condition should be set.</pre>
    ///
    ///
    #[serde(rename = "containerQueries")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub container_queries: Option<crate::css::types::CssContainerQuery>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSFunctionConditionNode/properties/supports。
    ///
    ///协议原文：
    ///
    ///<pre>@supports CSS at-rule condition. Only one type of condition should be set.</pre>
    ///
    ///
    #[serde(rename = "supports")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub supports: Option<crate::css::types::CssSupports>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSFunctionConditionNode/properties/navigation。
    ///
    ///协议原文：
    ///
    ///<pre>@navigation condition. Only one type of condition should be set.</pre>
    ///
    ///
    #[serde(rename = "navigation")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub navigation: Option<crate::css::types::CssNavigation>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSFunctionConditionNode/properties/children。
    ///
    ///协议原文：
    ///
    ///<pre>Block body.</pre>
    ///
    ///
    #[serde(rename = "children")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub children: Vec<crate::css::types::CssFunctionNode>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSFunctionConditionNode/properties/conditionText。
    ///
    ///协议原文：
    ///
    ///<pre>The condition text.</pre>
    ///
    ///
    #[serde(rename = "conditionText")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub condition_text: String,
}
impl<'de> serde::Deserialize<'de> for CssFunctionConditionNode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssFunctionConditionNode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSFunctionConditionNode/properties/media。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Media query for this conditional block. Only one type of condition should be set.</pre>
                    ///
                    ///
                    #[serde(rename = "media")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub media: Option<crate::css::types::CssMedia>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSFunctionConditionNode/properties/containerQueries。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Container query for this conditional block. Only one type of condition should be set.</pre>
                    ///
                    ///
                    #[serde(rename = "containerQueries")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub container_queries: Option<crate::css::types::CssContainerQuery>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSFunctionConditionNode/properties/supports。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>@supports CSS at-rule condition. Only one type of condition should be set.</pre>
                    ///
                    ///
                    #[serde(rename = "supports")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub supports: Option<crate::css::types::CssSupports>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSFunctionConditionNode/properties/navigation。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>@navigation condition. Only one type of condition should be set.</pre>
                    ///
                    ///
                    #[serde(rename = "navigation")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub navigation: Option<crate::css::types::CssNavigation>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSFunctionConditionNode/properties/children。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Block body.</pre>
                    ///
                    ///
                    #[serde(rename = "children")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub children: Vec<crate::css::types::CssFunctionNode>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSFunctionConditionNode/properties/conditionText。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The condition text.</pre>
                    ///
                    ///
                    #[serde(rename = "conditionText")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub condition_text: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CssFunctionConditionNode {
                    media: fields.media,
                    container_queries: fields.container_queries,
                    supports: fields.supports,
                    navigation: fields.navigation,
                    children: fields.children,
                    condition_text: fields.condition_text,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.CSSFunctionNode。
///
///协议原文：
///
///<pre>Section of the body of a CSS function rule.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CssFunctionNode {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSFunctionNode/properties/condition。
    ///
    ///协议原文：
    ///
    ///<pre>A conditional block. If set, style should not be set.</pre>
    ///
    ///
    #[serde(rename = "condition")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub condition: Option<crate::css::types::CssFunctionConditionNode>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSFunctionNode/properties/style。
    ///
    ///协议原文：
    ///
    ///<pre>Values set by this node. If set, condition should not be set.</pre>
    ///
    ///
    #[serde(rename = "style")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub style: Option<crate::css::types::CssStyle>,
}
impl<'de> serde::Deserialize<'de> for CssFunctionNode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssFunctionNode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSFunctionNode/properties/condition。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A conditional block. If set, style should not be set.</pre>
                    ///
                    ///
                    #[serde(rename = "condition")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub condition: Option<crate::css::types::CssFunctionConditionNode>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSFunctionNode/properties/style。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Values set by this node. If set, condition should not be set.</pre>
                    ///
                    ///
                    #[serde(rename = "style")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub style: Option<crate::css::types::CssStyle>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CssFunctionNode {
                    condition: fields.condition,
                    style: fields.style,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.CSSFunctionParameter。
///
///协议原文：
///
///<pre>CSS function argument representation.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CssFunctionParameter {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSFunctionParameter/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>The parameter name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSFunctionParameter/properties/type。
    ///
    ///协议原文：
    ///
    ///<pre>The parameter type.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: String,
}
impl<'de> serde::Deserialize<'de> for CssFunctionParameter {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssFunctionParameter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSFunctionParameter/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The parameter name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSFunctionParameter/properties/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The parameter type.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CssFunctionParameter {
                    name: fields.name,
                    r#type: fields.r#type,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.CSSFunctionRule。
///
///协议原文：
///
///<pre>CSS function at-rule representation.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CssFunctionRule {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSFunctionRule/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Name of the function.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: crate::css::types::Value,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSFunctionRule/properties/styleSheetId。
    ///
    ///协议原文：
    ///
    ///<pre>The css style sheet identifier (absent for user agent stylesheet and user-specified
    ///stylesheet rules) this rule came from.</pre>
    ///
    ///
    #[serde(rename = "styleSheetId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSFunctionRule/properties/origin。
    ///
    ///协议原文：
    ///
    ///<pre>Parent stylesheet's origin.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: crate::css::types::StyleSheetOrigin,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSFunctionRule/properties/parameters。
    ///
    ///协议原文：
    ///
    ///<pre>List of parameters.</pre>
    ///
    ///
    #[serde(rename = "parameters")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub parameters: Vec<crate::css::types::CssFunctionParameter>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSFunctionRule/properties/children。
    ///
    ///协议原文：
    ///
    ///<pre>Function body.</pre>
    ///
    ///
    #[serde(rename = "children")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub children: Vec<crate::css::types::CssFunctionNode>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSFunctionRule/properties/originTreeScopeNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>The BackendNodeId of the DOM node that constitutes the origin tree scope of this rule.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "originTreeScopeNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub origin_tree_scope_node_id: Option<crate::dom::types::BackendNodeId>,
}
impl<'de> serde::Deserialize<'de> for CssFunctionRule {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssFunctionRule;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSFunctionRule/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Name of the function.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: crate::css::types::Value,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSFunctionRule/properties/styleSheetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The css style sheet identifier (absent for user agent stylesheet and user-specified
                    ///stylesheet rules) this rule came from.</pre>
                    ///
                    ///
                    #[serde(rename = "styleSheetId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSFunctionRule/properties/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Parent stylesheet's origin.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: crate::css::types::StyleSheetOrigin,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSFunctionRule/properties/parameters。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>List of parameters.</pre>
                    ///
                    ///
                    #[serde(rename = "parameters")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub parameters: Vec<crate::css::types::CssFunctionParameter>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSFunctionRule/properties/children。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Function body.</pre>
                    ///
                    ///
                    #[serde(rename = "children")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub children: Vec<crate::css::types::CssFunctionNode>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSFunctionRule/properties/originTreeScopeNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The BackendNodeId of the DOM node that constitutes the origin tree scope of this rule.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "originTreeScopeNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub origin_tree_scope_node_id: Option<crate::dom::types::BackendNodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CssFunctionRule {
                    name: fields.name,
                    style_sheet_id: fields.style_sheet_id,
                    origin: fields.origin,
                    parameters: fields.parameters,
                    children: fields.children,
                    origin_tree_scope_node_id: fields.origin_tree_scope_node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.CSSKeyframeRule。
///
///协议原文：
///
///<pre>CSS keyframe rule representation.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CssKeyframeRule {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSKeyframeRule/properties/styleSheetId。
    ///
    ///协议原文：
    ///
    ///<pre>The css style sheet identifier (absent for user agent stylesheet and user-specified
    ///stylesheet rules) this rule came from.</pre>
    ///
    ///
    #[serde(rename = "styleSheetId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSKeyframeRule/properties/origin。
    ///
    ///协议原文：
    ///
    ///<pre>Parent stylesheet's origin.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: crate::css::types::StyleSheetOrigin,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSKeyframeRule/properties/keyText。
    ///
    ///协议原文：
    ///
    ///<pre>Associated key text.</pre>
    ///
    ///
    #[serde(rename = "keyText")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub key_text: crate::css::types::Value,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSKeyframeRule/properties/style。
    ///
    ///协议原文：
    ///
    ///<pre>Associated style declaration.</pre>
    ///
    ///
    #[serde(rename = "style")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub style: crate::css::types::CssStyle,
}
impl<'de> serde::Deserialize<'de> for CssKeyframeRule {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssKeyframeRule;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSKeyframeRule/properties/styleSheetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The css style sheet identifier (absent for user agent stylesheet and user-specified
                    ///stylesheet rules) this rule came from.</pre>
                    ///
                    ///
                    #[serde(rename = "styleSheetId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSKeyframeRule/properties/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Parent stylesheet's origin.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: crate::css::types::StyleSheetOrigin,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSKeyframeRule/properties/keyText。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Associated key text.</pre>
                    ///
                    ///
                    #[serde(rename = "keyText")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub key_text: crate::css::types::Value,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSKeyframeRule/properties/style。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Associated style declaration.</pre>
                    ///
                    ///
                    #[serde(rename = "style")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub style: crate::css::types::CssStyle,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CssKeyframeRule {
                    style_sheet_id: fields.style_sheet_id,
                    origin: fields.origin,
                    key_text: fields.key_text,
                    style: fields.style,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.CSSKeyframesRule。
///
///协议原文：
///
///<pre>CSS keyframes rule representation.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CssKeyframesRule {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSKeyframesRule/properties/animationName。
    ///
    ///协议原文：
    ///
    ///<pre>Animation name.</pre>
    ///
    ///
    #[serde(rename = "animationName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub animation_name: crate::css::types::Value,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSKeyframesRule/properties/keyframes。
    ///
    ///协议原文：
    ///
    ///<pre>List of keyframes.</pre>
    ///
    ///
    #[serde(rename = "keyframes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub keyframes: Vec<crate::css::types::CssKeyframeRule>,
}
impl<'de> serde::Deserialize<'de> for CssKeyframesRule {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssKeyframesRule;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSKeyframesRule/properties/animationName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Animation name.</pre>
                    ///
                    ///
                    #[serde(rename = "animationName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub animation_name: crate::css::types::Value,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSKeyframesRule/properties/keyframes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>List of keyframes.</pre>
                    ///
                    ///
                    #[serde(rename = "keyframes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub keyframes: Vec<crate::css::types::CssKeyframeRule>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CssKeyframesRule {
                    animation_name: fields.animation_name,
                    keyframes: fields.keyframes,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.CSSLayer。
///
///协议原文：
///
///<pre>CSS Layer at-rule descriptor.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CssLayer {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSLayer/properties/text。
    ///
    ///协议原文：
    ///
    ///<pre>Layer name.</pre>
    ///
    ///
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSLayer/properties/range。
    ///
    ///协议原文：
    ///
    ///<pre>The associated rule header range in the enclosing stylesheet (if
    ///available).</pre>
    ///
    ///
    #[serde(rename = "range")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub range: Option<crate::css::types::SourceRange>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSLayer/properties/styleSheetId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the stylesheet containing this object (if exists).</pre>
    ///
    ///
    #[serde(rename = "styleSheetId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
}
impl<'de> serde::Deserialize<'de> for CssLayer {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssLayer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSLayer/properties/text。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Layer name.</pre>
                    ///
                    ///
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSLayer/properties/range。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The associated rule header range in the enclosing stylesheet (if
                    ///available).</pre>
                    ///
                    ///
                    #[serde(rename = "range")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub range: Option<crate::css::types::SourceRange>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSLayer/properties/styleSheetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the stylesheet containing this object (if exists).</pre>
                    ///
                    ///
                    #[serde(rename = "styleSheetId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CssLayer {
                    text: fields.text,
                    range: fields.range,
                    style_sheet_id: fields.style_sheet_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.CSSLayerData。
///
///协议原文：
///
///<pre>CSS Layer data.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CssLayerData {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSLayerData/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Layer name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSLayerData/properties/subLayers。
    ///
    ///协议原文：
    ///
    ///<pre>Direct sub-layers</pre>
    ///
    ///
    #[serde(rename = "subLayers")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub sub_layers: Option<Vec<crate::css::types::CssLayerData>>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSLayerData/properties/order。
    ///
    ///协议原文：
    ///
    ///<pre>Layer order. The order determines the order of the layer in the cascade order.
    ///A higher number has higher priority in the cascade order.</pre>
    ///
    ///
    #[serde(rename = "order")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub order: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for CssLayerData {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssLayerData;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSLayerData/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Layer name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSLayerData/properties/subLayers。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Direct sub-layers</pre>
                    ///
                    ///
                    #[serde(rename = "subLayers")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub sub_layers: Option<Vec<crate::css::types::CssLayerData>>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSLayerData/properties/order。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Layer order. The order determines the order of the layer in the cascade order.
                    ///A higher number has higher priority in the cascade order.</pre>
                    ///
                    ///
                    #[serde(rename = "order")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub order: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CssLayerData {
                    name: fields.name,
                    sub_layers: fields.sub_layers,
                    order: fields.order,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：CSS.type.CSSMedia/properties/source。
///
///协议原文：
///
///<pre>Source of the media query: "mediaRule" if specified by a @media rule, "importRule" if
///specified by an @import rule, "linkedSheet" if specified by a "media" attribute in a linked
///stylesheet's LINK tag, "inlineSheet" if specified by a "media" attribute in an inline
///stylesheet's STYLE tag.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CssMediaSource {
    ///协议值 "mediaRule"。
    MediaRule,
    ///协议值 "importRule"。
    ImportRule,
    ///协议值 "linkedSheet"。
    LinkedSheet,
    ///协议值 "inlineSheet"。
    InlineSheet,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl CssMediaSource {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::css::types::CssMediaSource::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::MediaRule => "mediaRule",
            Self::ImportRule => "importRule",
            Self::LinkedSheet => "linkedSheet",
            Self::InlineSheet => "inlineSheet",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for CssMediaSource {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for CssMediaSource {
    fn from(value: String) -> Self {
        match value.as_str() {
            "mediaRule" => CssMediaSource::MediaRule,
            "importRule" => CssMediaSource::ImportRule,
            "linkedSheet" => CssMediaSource::LinkedSheet,
            "inlineSheet" => CssMediaSource::InlineSheet,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for CssMediaSource {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CssMediaSource {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssMediaSource;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "mediaRule" => CssMediaSource::MediaRule,
                    "importRule" => CssMediaSource::ImportRule,
                    "linkedSheet" => CssMediaSource::LinkedSheet,
                    "inlineSheet" => CssMediaSource::InlineSheet,
                    _ => CssMediaSource::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(CssMediaSource::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.CSSMedia。
///
///协议原文：
///
///<pre>CSS media rule descriptor.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CssMedia {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSMedia/properties/text。
    ///
    ///协议原文：
    ///
    ///<pre>Media query text.</pre>
    ///
    ///
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSMedia/properties/source。
    ///
    ///协议原文：
    ///
    ///<pre>Source of the media query: "mediaRule" if specified by a @media rule, "importRule" if
    ///specified by an @import rule, "linkedSheet" if specified by a "media" attribute in a linked
    ///stylesheet's LINK tag, "inlineSheet" if specified by a "media" attribute in an inline
    ///stylesheet's STYLE tag.</pre>
    ///
    ///
    #[serde(rename = "source")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub source: CssMediaSource,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSMedia/properties/sourceURL。
    ///
    ///协议原文：
    ///
    ///<pre>URL of the document containing the media query description.</pre>
    ///
    ///
    #[serde(rename = "sourceURL")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_url: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSMedia/properties/range。
    ///
    ///协议原文：
    ///
    ///<pre>The associated rule (@media or @import) header range in the enclosing stylesheet (if
    ///available).</pre>
    ///
    ///
    #[serde(rename = "range")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub range: Option<crate::css::types::SourceRange>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSMedia/properties/styleSheetId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the stylesheet containing this object (if exists).</pre>
    ///
    ///
    #[serde(rename = "styleSheetId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSMedia/properties/mediaList。
    ///
    ///协议原文：
    ///
    ///<pre>Array of media queries.</pre>
    ///
    ///
    #[serde(rename = "mediaList")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub media_list: Option<Vec<crate::css::types::MediaQuery>>,
}
impl<'de> serde::Deserialize<'de> for CssMedia {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssMedia;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSMedia/properties/text。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Media query text.</pre>
                    ///
                    ///
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSMedia/properties/source。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Source of the media query: "mediaRule" if specified by a @media rule, "importRule" if
                    ///specified by an @import rule, "linkedSheet" if specified by a "media" attribute in a linked
                    ///stylesheet's LINK tag, "inlineSheet" if specified by a "media" attribute in an inline
                    ///stylesheet's STYLE tag.</pre>
                    ///
                    ///
                    #[serde(rename = "source")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub source: CssMediaSource,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSMedia/properties/sourceURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>URL of the document containing the media query description.</pre>
                    ///
                    ///
                    #[serde(rename = "sourceURL")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub source_url: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSMedia/properties/range。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The associated rule (@media or @import) header range in the enclosing stylesheet (if
                    ///available).</pre>
                    ///
                    ///
                    #[serde(rename = "range")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub range: Option<crate::css::types::SourceRange>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSMedia/properties/styleSheetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the stylesheet containing this object (if exists).</pre>
                    ///
                    ///
                    #[serde(rename = "styleSheetId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSMedia/properties/mediaList。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Array of media queries.</pre>
                    ///
                    ///
                    #[serde(rename = "mediaList")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub media_list: Option<Vec<crate::css::types::MediaQuery>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CssMedia {
                    text: fields.text,
                    source: fields.source,
                    source_url: fields.source_url,
                    range: fields.range,
                    style_sheet_id: fields.style_sheet_id,
                    media_list: fields.media_list,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.CSSNavigation。
///
///协议原文：
///
///<pre>CSS Navigation at-rule descriptor.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CssNavigation {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSNavigation/properties/text。
    ///
    ///协议原文：
    ///
    ///<pre>Navigation rule text.</pre>
    ///
    ///
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSNavigation/properties/active。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the navigation condition is satisfied.</pre>
    ///
    ///
    #[serde(rename = "active")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub active: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSNavigation/properties/range。
    ///
    ///协议原文：
    ///
    ///<pre>The associated rule header range in the enclosing stylesheet (if
    ///available).</pre>
    ///
    ///
    #[serde(rename = "range")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub range: Option<crate::css::types::SourceRange>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSNavigation/properties/styleSheetId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the stylesheet containing this object (if exists).</pre>
    ///
    ///
    #[serde(rename = "styleSheetId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
}
impl<'de> serde::Deserialize<'de> for CssNavigation {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssNavigation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSNavigation/properties/text。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Navigation rule text.</pre>
                    ///
                    ///
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSNavigation/properties/active。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the navigation condition is satisfied.</pre>
                    ///
                    ///
                    #[serde(rename = "active")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub active: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSNavigation/properties/range。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The associated rule header range in the enclosing stylesheet (if
                    ///available).</pre>
                    ///
                    ///
                    #[serde(rename = "range")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub range: Option<crate::css::types::SourceRange>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSNavigation/properties/styleSheetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the stylesheet containing this object (if exists).</pre>
                    ///
                    ///
                    #[serde(rename = "styleSheetId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CssNavigation {
                    text: fields.text,
                    active: fields.active,
                    range: fields.range,
                    style_sheet_id: fields.style_sheet_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.CSSPositionTryRule。
///
///协议原文：
///
///<pre>CSS @position-try rule representation.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CssPositionTryRule {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSPositionTryRule/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>The prelude dashed-ident name</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: crate::css::types::Value,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSPositionTryRule/properties/styleSheetId。
    ///
    ///协议原文：
    ///
    ///<pre>The css style sheet identifier (absent for user agent stylesheet and user-specified
    ///stylesheet rules) this rule came from.</pre>
    ///
    ///
    #[serde(rename = "styleSheetId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSPositionTryRule/properties/origin。
    ///
    ///协议原文：
    ///
    ///<pre>Parent stylesheet's origin.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: crate::css::types::StyleSheetOrigin,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSPositionTryRule/properties/style。
    ///
    ///协议原文：
    ///
    ///<pre>Associated style declaration.</pre>
    ///
    ///
    #[serde(rename = "style")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub style: crate::css::types::CssStyle,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSPositionTryRule/properties/active。
    #[serde(rename = "active")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub active: bool,
}
impl<'de> serde::Deserialize<'de> for CssPositionTryRule {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssPositionTryRule;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSPositionTryRule/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The prelude dashed-ident name</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: crate::css::types::Value,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSPositionTryRule/properties/styleSheetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The css style sheet identifier (absent for user agent stylesheet and user-specified
                    ///stylesheet rules) this rule came from.</pre>
                    ///
                    ///
                    #[serde(rename = "styleSheetId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSPositionTryRule/properties/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Parent stylesheet's origin.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: crate::css::types::StyleSheetOrigin,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSPositionTryRule/properties/style。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Associated style declaration.</pre>
                    ///
                    ///
                    #[serde(rename = "style")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub style: crate::css::types::CssStyle,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSPositionTryRule/properties/active。
                    #[serde(rename = "active")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub active: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CssPositionTryRule {
                    name: fields.name,
                    style_sheet_id: fields.style_sheet_id,
                    origin: fields.origin,
                    style: fields.style,
                    active: fields.active,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.CSSProperty。
///
///协议原文：
///
///<pre>CSS property declaration data.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CssProperty {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSProperty/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>The property name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSProperty/properties/value。
    ///
    ///协议原文：
    ///
    ///<pre>The property value.</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSProperty/properties/important。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the property has "!important" annotation (implies `false` if absent).</pre>
    ///
    ///
    #[serde(rename = "important")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub important: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSProperty/properties/implicit。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the property is implicit (implies `false` if absent).</pre>
    ///
    ///
    #[serde(rename = "implicit")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub implicit: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSProperty/properties/text。
    ///
    ///协议原文：
    ///
    ///<pre>The full property text as specified in the style.</pre>
    ///
    ///
    #[serde(rename = "text")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub text: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSProperty/properties/parsedOk。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the property is understood by the browser (implies `true` if absent).</pre>
    ///
    ///
    #[serde(rename = "parsedOk")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub parsed_ok: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSProperty/properties/disabled。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the property is disabled by the user (present for source-based properties only).</pre>
    ///
    ///
    #[serde(rename = "disabled")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSProperty/properties/range。
    ///
    ///协议原文：
    ///
    ///<pre>The entire property range in the enclosing style declaration (if available).</pre>
    ///
    ///
    #[serde(rename = "range")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub range: Option<crate::css::types::SourceRange>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSProperty/properties/longhandProperties。
    ///
    ///协议原文：
    ///
    ///<pre>Parsed longhand components of this property if it is a shorthand.
    ///This field will be empty if the given property is not a shorthand.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "longhandProperties")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub longhand_properties: Option<Vec<crate::css::types::CssProperty>>,
}
impl<'de> serde::Deserialize<'de> for CssProperty {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSProperty/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The property name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSProperty/properties/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The property value.</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSProperty/properties/important。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the property has "!important" annotation (implies `false` if absent).</pre>
                    ///
                    ///
                    #[serde(rename = "important")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub important: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSProperty/properties/implicit。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the property is implicit (implies `false` if absent).</pre>
                    ///
                    ///
                    #[serde(rename = "implicit")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub implicit: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSProperty/properties/text。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The full property text as specified in the style.</pre>
                    ///
                    ///
                    #[serde(rename = "text")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub text: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSProperty/properties/parsedOk。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the property is understood by the browser (implies `true` if absent).</pre>
                    ///
                    ///
                    #[serde(rename = "parsedOk")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub parsed_ok: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSProperty/properties/disabled。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the property is disabled by the user (present for source-based properties only).</pre>
                    ///
                    ///
                    #[serde(rename = "disabled")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub disabled: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSProperty/properties/range。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The entire property range in the enclosing style declaration (if available).</pre>
                    ///
                    ///
                    #[serde(rename = "range")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub range: Option<crate::css::types::SourceRange>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSProperty/properties/longhandProperties。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Parsed longhand components of this property if it is a shorthand.
                    ///This field will be empty if the given property is not a shorthand.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "longhandProperties")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub longhand_properties: Option<Vec<crate::css::types::CssProperty>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CssProperty {
                    name: fields.name,
                    value: fields.value,
                    important: fields.important,
                    implicit: fields.implicit,
                    text: fields.text,
                    parsed_ok: fields.parsed_ok,
                    disabled: fields.disabled,
                    range: fields.range,
                    longhand_properties: fields.longhand_properties,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.CSSPropertyRegistration。
///
///协议原文：
///
///<pre>Representation of a custom property registration through CSS.registerProperty</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CssPropertyRegistration {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSPropertyRegistration/properties/propertyName。
    #[serde(rename = "propertyName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub property_name: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSPropertyRegistration/properties/initialValue。
    #[serde(rename = "initialValue")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_value: Option<crate::css::types::Value>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSPropertyRegistration/properties/inherits。
    #[serde(rename = "inherits")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub inherits: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSPropertyRegistration/properties/syntax。
    #[serde(rename = "syntax")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub syntax: String,
}
impl<'de> serde::Deserialize<'de> for CssPropertyRegistration {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssPropertyRegistration;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSPropertyRegistration/properties/propertyName。
                    #[serde(rename = "propertyName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub property_name: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSPropertyRegistration/properties/initialValue。
                    #[serde(rename = "initialValue")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub initial_value: Option<crate::css::types::Value>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSPropertyRegistration/properties/inherits。
                    #[serde(rename = "inherits")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub inherits: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSPropertyRegistration/properties/syntax。
                    #[serde(rename = "syntax")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub syntax: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CssPropertyRegistration {
                    property_name: fields.property_name,
                    initial_value: fields.initial_value,
                    inherits: fields.inherits,
                    syntax: fields.syntax,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.CSSPropertyRule。
///
///协议原文：
///
///<pre>CSS property at-rule representation.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CssPropertyRule {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSPropertyRule/properties/styleSheetId。
    ///
    ///协议原文：
    ///
    ///<pre>The css style sheet identifier (absent for user agent stylesheet and user-specified
    ///stylesheet rules) this rule came from.</pre>
    ///
    ///
    #[serde(rename = "styleSheetId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSPropertyRule/properties/origin。
    ///
    ///协议原文：
    ///
    ///<pre>Parent stylesheet's origin.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: crate::css::types::StyleSheetOrigin,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSPropertyRule/properties/propertyName。
    ///
    ///协议原文：
    ///
    ///<pre>Associated property name.</pre>
    ///
    ///
    #[serde(rename = "propertyName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub property_name: crate::css::types::Value,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSPropertyRule/properties/style。
    ///
    ///协议原文：
    ///
    ///<pre>Associated style declaration.</pre>
    ///
    ///
    #[serde(rename = "style")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub style: crate::css::types::CssStyle,
}
impl<'de> serde::Deserialize<'de> for CssPropertyRule {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssPropertyRule;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSPropertyRule/properties/styleSheetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The css style sheet identifier (absent for user agent stylesheet and user-specified
                    ///stylesheet rules) this rule came from.</pre>
                    ///
                    ///
                    #[serde(rename = "styleSheetId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSPropertyRule/properties/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Parent stylesheet's origin.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: crate::css::types::StyleSheetOrigin,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSPropertyRule/properties/propertyName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Associated property name.</pre>
                    ///
                    ///
                    #[serde(rename = "propertyName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub property_name: crate::css::types::Value,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSPropertyRule/properties/style。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Associated style declaration.</pre>
                    ///
                    ///
                    #[serde(rename = "style")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub style: crate::css::types::CssStyle,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CssPropertyRule {
                    style_sheet_id: fields.style_sheet_id,
                    origin: fields.origin,
                    property_name: fields.property_name,
                    style: fields.style,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.CSSRule。
///
///协议原文：
///
///<pre>CSS rule representation.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CssRule {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSRule/properties/styleSheetId。
    ///
    ///协议原文：
    ///
    ///<pre>The css style sheet identifier (absent for user agent stylesheet and user-specified
    ///stylesheet rules) this rule came from.</pre>
    ///
    ///
    #[serde(rename = "styleSheetId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSRule/properties/selectorList。
    ///
    ///协议原文：
    ///
    ///<pre>Rule selector data.</pre>
    ///
    ///
    #[serde(rename = "selectorList")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub selector_list: crate::css::types::SelectorList,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSRule/properties/nestingSelectors。
    ///
    ///协议原文：
    ///
    ///<pre>Array of selectors from ancestor style rules, sorted by distance from the current rule.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "nestingSelectors")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub nesting_selectors: Option<Vec<String>>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSRule/properties/origin。
    ///
    ///协议原文：
    ///
    ///<pre>Parent stylesheet's origin.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: crate::css::types::StyleSheetOrigin,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSRule/properties/style。
    ///
    ///协议原文：
    ///
    ///<pre>Associated style declaration.</pre>
    ///
    ///
    #[serde(rename = "style")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub style: crate::css::types::CssStyle,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSRule/properties/originTreeScopeNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>The BackendNodeId of the DOM node that constitutes the origin tree scope of this rule.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "originTreeScopeNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub origin_tree_scope_node_id: Option<crate::dom::types::BackendNodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSRule/properties/media。
    ///
    ///协议原文：
    ///
    ///<pre>Media list array (for rules involving media queries). The array enumerates media queries
    ///starting with the innermost one, going outwards.</pre>
    ///
    ///
    #[serde(rename = "media")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub media: Option<Vec<crate::css::types::CssMedia>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSRule/properties/containerQueries。
    ///
    ///协议原文：
    ///
    ///<pre>Container query list array (for rules involving container queries).
    ///The array enumerates container queries starting with the innermost one, going outwards.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "containerQueries")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub container_queries: Option<Vec<crate::css::types::CssContainerQuery>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSRule/properties/supports。
    ///
    ///协议原文：
    ///
    ///<pre>@supports CSS at-rule array.
    ///The array enumerates @supports at-rules starting with the innermost one, going outwards.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "supports")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub supports: Option<Vec<crate::css::types::CssSupports>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSRule/properties/layers。
    ///
    ///协议原文：
    ///
    ///<pre>Cascade layer array. Contains the layer hierarchy that this rule belongs to starting
    ///with the innermost layer and going outwards.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "layers")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub layers: Option<Vec<crate::css::types::CssLayer>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSRule/properties/scopes。
    ///
    ///协议原文：
    ///
    ///<pre>@scope CSS at-rule array.
    ///The array enumerates @scope at-rules starting with the innermost one, going outwards.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "scopes")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub scopes: Option<Vec<crate::css::types::CssScope>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSRule/properties/ruleTypes。
    ///
    ///协议原文：
    ///
    ///<pre>The array keeps the types of ancestor CSSRules from the innermost going outwards.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "ruleTypes")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub rule_types: Option<Vec<crate::css::types::CssRuleType>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSRule/properties/startingStyles。
    ///
    ///协议原文：
    ///
    ///<pre>@starting-style CSS at-rule array.
    ///The array enumerates @starting-style at-rules starting with the innermost one, going outwards.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "startingStyles")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub starting_styles: Option<Vec<crate::css::types::CssStartingStyle>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSRule/properties/navigations。
    ///
    ///协议原文：
    ///
    ///<pre>@navigation CSS at-rule array.
    ///The array enumerates @navigation at-rules starting with the innermost one, going outwards.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "navigations")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub navigations: Option<Vec<crate::css::types::CssNavigation>>,
}
impl<'de> serde::Deserialize<'de> for CssRule {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssRule;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSRule/properties/styleSheetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The css style sheet identifier (absent for user agent stylesheet and user-specified
                    ///stylesheet rules) this rule came from.</pre>
                    ///
                    ///
                    #[serde(rename = "styleSheetId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSRule/properties/selectorList。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Rule selector data.</pre>
                    ///
                    ///
                    #[serde(rename = "selectorList")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub selector_list: crate::css::types::SelectorList,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSRule/properties/nestingSelectors。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Array of selectors from ancestor style rules, sorted by distance from the current rule.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "nestingSelectors")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub nesting_selectors: Option<Vec<String>>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSRule/properties/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Parent stylesheet's origin.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: crate::css::types::StyleSheetOrigin,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSRule/properties/style。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Associated style declaration.</pre>
                    ///
                    ///
                    #[serde(rename = "style")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub style: crate::css::types::CssStyle,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSRule/properties/originTreeScopeNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The BackendNodeId of the DOM node that constitutes the origin tree scope of this rule.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "originTreeScopeNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub origin_tree_scope_node_id: Option<crate::dom::types::BackendNodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSRule/properties/media。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Media list array (for rules involving media queries). The array enumerates media queries
                    ///starting with the innermost one, going outwards.</pre>
                    ///
                    ///
                    #[serde(rename = "media")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub media: Option<Vec<crate::css::types::CssMedia>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSRule/properties/containerQueries。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Container query list array (for rules involving container queries).
                    ///The array enumerates container queries starting with the innermost one, going outwards.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "containerQueries")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub container_queries: Option<Vec<crate::css::types::CssContainerQuery>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSRule/properties/supports。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>@supports CSS at-rule array.
                    ///The array enumerates @supports at-rules starting with the innermost one, going outwards.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "supports")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub supports: Option<Vec<crate::css::types::CssSupports>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSRule/properties/layers。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cascade layer array. Contains the layer hierarchy that this rule belongs to starting
                    ///with the innermost layer and going outwards.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "layers")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub layers: Option<Vec<crate::css::types::CssLayer>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSRule/properties/scopes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>@scope CSS at-rule array.
                    ///The array enumerates @scope at-rules starting with the innermost one, going outwards.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "scopes")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub scopes: Option<Vec<crate::css::types::CssScope>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSRule/properties/ruleTypes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The array keeps the types of ancestor CSSRules from the innermost going outwards.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "ruleTypes")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub rule_types: Option<Vec<crate::css::types::CssRuleType>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSRule/properties/startingStyles。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>@starting-style CSS at-rule array.
                    ///The array enumerates @starting-style at-rules starting with the innermost one, going outwards.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "startingStyles")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub starting_styles: Option<Vec<crate::css::types::CssStartingStyle>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSRule/properties/navigations。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>@navigation CSS at-rule array.
                    ///The array enumerates @navigation at-rules starting with the innermost one, going outwards.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "navigations")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub navigations: Option<Vec<crate::css::types::CssNavigation>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CssRule {
                    style_sheet_id: fields.style_sheet_id,
                    selector_list: fields.selector_list,
                    nesting_selectors: fields.nesting_selectors,
                    origin: fields.origin,
                    style: fields.style,
                    origin_tree_scope_node_id: fields.origin_tree_scope_node_id,
                    media: fields.media,
                    container_queries: fields.container_queries,
                    supports: fields.supports,
                    layers: fields.layers,
                    scopes: fields.scopes,
                    rule_types: fields.rule_types,
                    starting_styles: fields.starting_styles,
                    navigations: fields.navigations,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：CSS.type.CSSRuleType。
///
///协议原文：
///
///<pre>Enum indicating the type of a CSS rule, used to represent the order of a style rule's ancestors.
///This list only contains rule types that are collected during the ancestor rule collection.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CssRuleType {
    ///协议值 "MediaRule"。
    MediaRule,
    ///协议值 "SupportsRule"。
    SupportsRule,
    ///协议值 "ContainerRule"。
    ContainerRule,
    ///协议值 "LayerRule"。
    LayerRule,
    ///协议值 "ScopeRule"。
    ScopeRule,
    ///协议值 "StyleRule"。
    StyleRule,
    ///协议值 "StartingStyleRule"。
    StartingStyleRule,
    ///协议值 "NavigationRule"。
    NavigationRule,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl CssRuleType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::css::types::CssRuleType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::MediaRule => "MediaRule",
            Self::SupportsRule => "SupportsRule",
            Self::ContainerRule => "ContainerRule",
            Self::LayerRule => "LayerRule",
            Self::ScopeRule => "ScopeRule",
            Self::StyleRule => "StyleRule",
            Self::StartingStyleRule => "StartingStyleRule",
            Self::NavigationRule => "NavigationRule",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for CssRuleType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for CssRuleType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "MediaRule" => CssRuleType::MediaRule,
            "SupportsRule" => CssRuleType::SupportsRule,
            "ContainerRule" => CssRuleType::ContainerRule,
            "LayerRule" => CssRuleType::LayerRule,
            "ScopeRule" => CssRuleType::ScopeRule,
            "StyleRule" => CssRuleType::StyleRule,
            "StartingStyleRule" => CssRuleType::StartingStyleRule,
            "NavigationRule" => CssRuleType::NavigationRule,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for CssRuleType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CssRuleType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssRuleType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "MediaRule" => CssRuleType::MediaRule,
                    "SupportsRule" => CssRuleType::SupportsRule,
                    "ContainerRule" => CssRuleType::ContainerRule,
                    "LayerRule" => CssRuleType::LayerRule,
                    "ScopeRule" => CssRuleType::ScopeRule,
                    "StyleRule" => CssRuleType::StyleRule,
                    "StartingStyleRule" => CssRuleType::StartingStyleRule,
                    "NavigationRule" => CssRuleType::NavigationRule,
                    _ => CssRuleType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(CssRuleType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.CSSScope。
///
///协议原文：
///
///<pre>CSS Scope at-rule descriptor.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CssScope {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSScope/properties/text。
    ///
    ///协议原文：
    ///
    ///<pre>Scope rule text.</pre>
    ///
    ///
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSScope/properties/range。
    ///
    ///协议原文：
    ///
    ///<pre>The associated rule header range in the enclosing stylesheet (if
    ///available).</pre>
    ///
    ///
    #[serde(rename = "range")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub range: Option<crate::css::types::SourceRange>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSScope/properties/styleSheetId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the stylesheet containing this object (if exists).</pre>
    ///
    ///
    #[serde(rename = "styleSheetId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
}
impl<'de> serde::Deserialize<'de> for CssScope {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssScope;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSScope/properties/text。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Scope rule text.</pre>
                    ///
                    ///
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSScope/properties/range。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The associated rule header range in the enclosing stylesheet (if
                    ///available).</pre>
                    ///
                    ///
                    #[serde(rename = "range")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub range: Option<crate::css::types::SourceRange>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSScope/properties/styleSheetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the stylesheet containing this object (if exists).</pre>
                    ///
                    ///
                    #[serde(rename = "styleSheetId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CssScope {
                    text: fields.text,
                    range: fields.range,
                    style_sheet_id: fields.style_sheet_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.CSSStartingStyle。
///
///协议原文：
///
///<pre>CSS Starting Style at-rule descriptor.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CssStartingStyle {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSStartingStyle/properties/range。
    ///
    ///协议原文：
    ///
    ///<pre>The associated rule header range in the enclosing stylesheet (if
    ///available).</pre>
    ///
    ///
    #[serde(rename = "range")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub range: Option<crate::css::types::SourceRange>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSStartingStyle/properties/styleSheetId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the stylesheet containing this object (if exists).</pre>
    ///
    ///
    #[serde(rename = "styleSheetId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
}
impl<'de> serde::Deserialize<'de> for CssStartingStyle {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssStartingStyle;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSStartingStyle/properties/range。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The associated rule header range in the enclosing stylesheet (if
                    ///available).</pre>
                    ///
                    ///
                    #[serde(rename = "range")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub range: Option<crate::css::types::SourceRange>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSStartingStyle/properties/styleSheetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the stylesheet containing this object (if exists).</pre>
                    ///
                    ///
                    #[serde(rename = "styleSheetId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CssStartingStyle {
                    range: fields.range,
                    style_sheet_id: fields.style_sheet_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.CSSStyle。
///
///协议原文：
///
///<pre>CSS style representation.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CssStyle {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSStyle/properties/styleSheetId。
    ///
    ///协议原文：
    ///
    ///<pre>The css style sheet identifier (absent for user agent stylesheet and user-specified
    ///stylesheet rules) this rule came from.</pre>
    ///
    ///
    #[serde(rename = "styleSheetId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyle/properties/cssProperties。
    ///
    ///协议原文：
    ///
    ///<pre>CSS properties in the style.</pre>
    ///
    ///
    #[serde(rename = "cssProperties")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub css_properties: Vec<crate::css::types::CssProperty>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyle/properties/shorthandEntries。
    ///
    ///协议原文：
    ///
    ///<pre>Computed values for all shorthands found in the style.</pre>
    ///
    ///
    #[serde(rename = "shorthandEntries")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub shorthand_entries: Vec<crate::css::types::ShorthandEntry>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSStyle/properties/cssText。
    ///
    ///协议原文：
    ///
    ///<pre>Style declaration text (if available).</pre>
    ///
    ///
    #[serde(rename = "cssText")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub css_text: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSStyle/properties/range。
    ///
    ///协议原文：
    ///
    ///<pre>Style declaration range in the enclosing stylesheet (if available).</pre>
    ///
    ///
    #[serde(rename = "range")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub range: Option<crate::css::types::SourceRange>,
}
impl<'de> serde::Deserialize<'de> for CssStyle {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssStyle;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSStyle/properties/styleSheetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The css style sheet identifier (absent for user agent stylesheet and user-specified
                    ///stylesheet rules) this rule came from.</pre>
                    ///
                    ///
                    #[serde(rename = "styleSheetId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyle/properties/cssProperties。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>CSS properties in the style.</pre>
                    ///
                    ///
                    #[serde(rename = "cssProperties")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub css_properties: Vec<crate::css::types::CssProperty>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyle/properties/shorthandEntries。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Computed values for all shorthands found in the style.</pre>
                    ///
                    ///
                    #[serde(rename = "shorthandEntries")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub shorthand_entries: Vec<crate::css::types::ShorthandEntry>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSStyle/properties/cssText。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Style declaration text (if available).</pre>
                    ///
                    ///
                    #[serde(rename = "cssText")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub css_text: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSStyle/properties/range。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Style declaration range in the enclosing stylesheet (if available).</pre>
                    ///
                    ///
                    #[serde(rename = "range")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub range: Option<crate::css::types::SourceRange>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CssStyle {
                    style_sheet_id: fields.style_sheet_id,
                    css_properties: fields.css_properties,
                    shorthand_entries: fields.shorthand_entries,
                    css_text: fields.css_text,
                    range: fields.range,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.CSSStyleSheetHeader。
///
///协议原文：
///
///<pre>CSS stylesheet metainformation.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CssStyleSheetHeader {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyleSheetHeader/properties/styleSheetId。
    ///
    ///协议原文：
    ///
    ///<pre>The stylesheet identifier.</pre>
    ///
    ///
    #[serde(rename = "styleSheetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub style_sheet_id: crate::dom::types::StyleSheetId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyleSheetHeader/properties/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Owner frame identifier.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyleSheetHeader/properties/sourceURL。
    ///
    ///协议原文：
    ///
    ///<pre>Stylesheet resource URL. Empty if this is a constructed stylesheet created using
    ///new CSSStyleSheet() (but non-empty if this is a constructed stylesheet imported
    ///as a CSS module script).</pre>
    ///
    ///
    #[serde(rename = "sourceURL")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub source_url: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSStyleSheetHeader/properties/sourceMapURL。
    ///
    ///协议原文：
    ///
    ///<pre>URL of source map associated with the stylesheet (if any).</pre>
    ///
    ///
    #[serde(rename = "sourceMapURL")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_map_url: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyleSheetHeader/properties/origin。
    ///
    ///协议原文：
    ///
    ///<pre>Stylesheet origin.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: crate::css::types::StyleSheetOrigin,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyleSheetHeader/properties/title。
    ///
    ///协议原文：
    ///
    ///<pre>Stylesheet title.</pre>
    ///
    ///
    #[serde(rename = "title")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub title: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSStyleSheetHeader/properties/ownerNode。
    ///
    ///协议原文：
    ///
    ///<pre>The backend id for the owner node of the stylesheet.</pre>
    ///
    ///
    #[serde(rename = "ownerNode")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub owner_node: Option<crate::dom::types::BackendNodeId>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyleSheetHeader/properties/disabled。
    ///
    ///协议原文：
    ///
    ///<pre>Denotes whether the stylesheet is disabled.</pre>
    ///
    ///
    #[serde(rename = "disabled")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub disabled: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSStyleSheetHeader/properties/hasSourceURL。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the sourceURL field value comes from the sourceURL comment.</pre>
    ///
    ///
    #[serde(rename = "hasSourceURL")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_source_url: Option<bool>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyleSheetHeader/properties/isInline。
    ///
    ///协议原文：
    ///
    ///<pre>Whether this stylesheet is created for STYLE tag by parser. This flag is not set for
    ///document.written STYLE tags.</pre>
    ///
    ///
    #[serde(rename = "isInline")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub is_inline: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyleSheetHeader/properties/isMutable。
    ///
    ///协议原文：
    ///
    ///<pre>Whether this stylesheet is mutable. Inline stylesheets become mutable
    ///after they have been modified via CSSOM API.
    ///`&lt;link&gt;` element's stylesheets become mutable only if DevTools modifies them.
    ///Constructed stylesheets (new CSSStyleSheet()) are mutable immediately after creation.</pre>
    ///
    ///
    #[serde(rename = "isMutable")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub is_mutable: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyleSheetHeader/properties/isConstructed。
    ///
    ///协议原文：
    ///
    ///<pre>True if this stylesheet is created through new CSSStyleSheet() or imported as a
    ///CSS module script.</pre>
    ///
    ///
    #[serde(rename = "isConstructed")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub is_constructed: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyleSheetHeader/properties/startLine。
    ///
    ///协议原文：
    ///
    ///<pre>Line offset of the stylesheet within the resource (zero based).</pre>
    ///
    ///
    #[serde(rename = "startLine")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub start_line: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyleSheetHeader/properties/startColumn。
    ///
    ///协议原文：
    ///
    ///<pre>Column offset of the stylesheet within the resource (zero based).</pre>
    ///
    ///
    #[serde(rename = "startColumn")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub start_column: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyleSheetHeader/properties/length。
    ///
    ///协议原文：
    ///
    ///<pre>Size of the content (in characters).</pre>
    ///
    ///
    #[serde(rename = "length")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub length: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyleSheetHeader/properties/endLine。
    ///
    ///协议原文：
    ///
    ///<pre>Line offset of the end of the stylesheet within the resource (zero based).</pre>
    ///
    ///
    #[serde(rename = "endLine")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub end_line: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyleSheetHeader/properties/endColumn。
    ///
    ///协议原文：
    ///
    ///<pre>Column offset of the end of the stylesheet within the resource (zero based).</pre>
    ///
    ///
    #[serde(rename = "endColumn")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub end_column: ydp_core::FiniteF64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSStyleSheetHeader/properties/loadingFailed。
    ///
    ///协议原文：
    ///
    ///<pre>If the style sheet was loaded from a network resource, this indicates when the resource failed to load</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "loadingFailed")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub loading_failed: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for CssStyleSheetHeader {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssStyleSheetHeader;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyleSheetHeader/properties/styleSheetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The stylesheet identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "styleSheetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub style_sheet_id: crate::dom::types::StyleSheetId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyleSheetHeader/properties/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Owner frame identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyleSheetHeader/properties/sourceURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Stylesheet resource URL. Empty if this is a constructed stylesheet created using
                    ///new CSSStyleSheet() (but non-empty if this is a constructed stylesheet imported
                    ///as a CSS module script).</pre>
                    ///
                    ///
                    #[serde(rename = "sourceURL")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub source_url: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSStyleSheetHeader/properties/sourceMapURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>URL of source map associated with the stylesheet (if any).</pre>
                    ///
                    ///
                    #[serde(rename = "sourceMapURL")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub source_map_url: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyleSheetHeader/properties/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Stylesheet origin.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: crate::css::types::StyleSheetOrigin,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyleSheetHeader/properties/title。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Stylesheet title.</pre>
                    ///
                    ///
                    #[serde(rename = "title")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub title: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSStyleSheetHeader/properties/ownerNode。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The backend id for the owner node of the stylesheet.</pre>
                    ///
                    ///
                    #[serde(rename = "ownerNode")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub owner_node: Option<crate::dom::types::BackendNodeId>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyleSheetHeader/properties/disabled。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Denotes whether the stylesheet is disabled.</pre>
                    ///
                    ///
                    #[serde(rename = "disabled")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub disabled: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSStyleSheetHeader/properties/hasSourceURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the sourceURL field value comes from the sourceURL comment.</pre>
                    ///
                    ///
                    #[serde(rename = "hasSourceURL")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub has_source_url: Option<bool>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyleSheetHeader/properties/isInline。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether this stylesheet is created for STYLE tag by parser. This flag is not set for
                    ///document.written STYLE tags.</pre>
                    ///
                    ///
                    #[serde(rename = "isInline")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub is_inline: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyleSheetHeader/properties/isMutable。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether this stylesheet is mutable. Inline stylesheets become mutable
                    ///after they have been modified via CSSOM API.
                    ///`&lt;link&gt;` element's stylesheets become mutable only if DevTools modifies them.
                    ///Constructed stylesheets (new CSSStyleSheet()) are mutable immediately after creation.</pre>
                    ///
                    ///
                    #[serde(rename = "isMutable")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub is_mutable: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyleSheetHeader/properties/isConstructed。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if this stylesheet is created through new CSSStyleSheet() or imported as a
                    ///CSS module script.</pre>
                    ///
                    ///
                    #[serde(rename = "isConstructed")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub is_constructed: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyleSheetHeader/properties/startLine。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Line offset of the stylesheet within the resource (zero based).</pre>
                    ///
                    ///
                    #[serde(rename = "startLine")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub start_line: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyleSheetHeader/properties/startColumn。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Column offset of the stylesheet within the resource (zero based).</pre>
                    ///
                    ///
                    #[serde(rename = "startColumn")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub start_column: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyleSheetHeader/properties/length。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Size of the content (in characters).</pre>
                    ///
                    ///
                    #[serde(rename = "length")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub length: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyleSheetHeader/properties/endLine。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Line offset of the end of the stylesheet within the resource (zero based).</pre>
                    ///
                    ///
                    #[serde(rename = "endLine")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub end_line: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSStyleSheetHeader/properties/endColumn。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Column offset of the end of the stylesheet within the resource (zero based).</pre>
                    ///
                    ///
                    #[serde(rename = "endColumn")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub end_column: ydp_core::FiniteF64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSStyleSheetHeader/properties/loadingFailed。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If the style sheet was loaded from a network resource, this indicates when the resource failed to load</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "loadingFailed")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub loading_failed: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CssStyleSheetHeader {
                    style_sheet_id: fields.style_sheet_id,
                    frame_id: fields.frame_id,
                    source_url: fields.source_url,
                    source_map_url: fields.source_map_url,
                    origin: fields.origin,
                    title: fields.title,
                    owner_node: fields.owner_node,
                    disabled: fields.disabled,
                    has_source_url: fields.has_source_url,
                    is_inline: fields.is_inline,
                    is_mutable: fields.is_mutable,
                    is_constructed: fields.is_constructed,
                    start_line: fields.start_line,
                    start_column: fields.start_column,
                    length: fields.length,
                    end_line: fields.end_line,
                    end_column: fields.end_column,
                    loading_failed: fields.loading_failed,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.CSSSupports。
///
///协议原文：
///
///<pre>CSS Supports at-rule descriptor.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CssSupports {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSSupports/properties/text。
    ///
    ///协议原文：
    ///
    ///<pre>Supports rule text.</pre>
    ///
    ///
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSSupports/properties/active。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the supports condition is satisfied.</pre>
    ///
    ///
    #[serde(rename = "active")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub active: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSSupports/properties/range。
    ///
    ///协议原文：
    ///
    ///<pre>The associated rule header range in the enclosing stylesheet (if
    ///available).</pre>
    ///
    ///
    #[serde(rename = "range")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub range: Option<crate::css::types::SourceRange>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSSupports/properties/styleSheetId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the stylesheet containing this object (if exists).</pre>
    ///
    ///
    #[serde(rename = "styleSheetId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
}
impl<'de> serde::Deserialize<'de> for CssSupports {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssSupports;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSSupports/properties/text。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Supports rule text.</pre>
                    ///
                    ///
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSSupports/properties/active。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the supports condition is satisfied.</pre>
                    ///
                    ///
                    #[serde(rename = "active")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub active: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSSupports/properties/range。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The associated rule header range in the enclosing stylesheet (if
                    ///available).</pre>
                    ///
                    ///
                    #[serde(rename = "range")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub range: Option<crate::css::types::SourceRange>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSSupports/properties/styleSheetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the stylesheet containing this object (if exists).</pre>
                    ///
                    ///
                    #[serde(rename = "styleSheetId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CssSupports {
                    text: fields.text,
                    active: fields.active,
                    range: fields.range,
                    style_sheet_id: fields.style_sheet_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.CSSTryRule。
///
///协议原文：
///
///<pre>CSS try rule representation.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CssTryRule {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSTryRule/properties/styleSheetId。
    ///
    ///协议原文：
    ///
    ///<pre>The css style sheet identifier (absent for user agent stylesheet and user-specified
    ///stylesheet rules) this rule came from.</pre>
    ///
    ///
    #[serde(rename = "styleSheetId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSTryRule/properties/origin。
    ///
    ///协议原文：
    ///
    ///<pre>Parent stylesheet's origin.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: crate::css::types::StyleSheetOrigin,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSTryRule/properties/style。
    ///
    ///协议原文：
    ///
    ///<pre>Associated style declaration.</pre>
    ///
    ///
    #[serde(rename = "style")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub style: crate::css::types::CssStyle,
}
impl<'de> serde::Deserialize<'de> for CssTryRule {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CssTryRule;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.CSSTryRule/properties/styleSheetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The css style sheet identifier (absent for user agent stylesheet and user-specified
                    ///stylesheet rules) this rule came from.</pre>
                    ///
                    ///
                    #[serde(rename = "styleSheetId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub style_sheet_id: Option<crate::dom::types::StyleSheetId>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSTryRule/properties/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Parent stylesheet's origin.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: crate::css::types::StyleSheetOrigin,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.CSSTryRule/properties/style。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Associated style declaration.</pre>
                    ///
                    ///
                    #[serde(rename = "style")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub style: crate::css::types::CssStyle,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CssTryRule {
                    style_sheet_id: fields.style_sheet_id,
                    origin: fields.origin,
                    style: fields.style,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.ComputedStyleExtraFields。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ComputedStyleExtraFields {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.ComputedStyleExtraFields/properties/isAppearanceBase。
    ///
    ///协议原文：
    ///
    ///<pre>Returns whether or not this node is being rendered with base appearance,
    ///which happens when it has its appearance property set to base/base-select
    ///or it is in the subtree of an element being rendered with base appearance.</pre>
    ///
    ///
    #[serde(rename = "isAppearanceBase")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub is_appearance_base: bool,
}
impl<'de> serde::Deserialize<'de> for ComputedStyleExtraFields {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ComputedStyleExtraFields;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.ComputedStyleExtraFields/properties/isAppearanceBase。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Returns whether or not this node is being rendered with base appearance,
                    ///which happens when it has its appearance property set to base/base-select
                    ///or it is in the subtree of an element being rendered with base appearance.</pre>
                    ///
                    ///
                    #[serde(rename = "isAppearanceBase")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub is_appearance_base: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ComputedStyleExtraFields {
                    is_appearance_base: fields.is_appearance_base,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.FontFace。
///
///协议原文：
///
///<pre>Properties of a web font: https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#font-descriptions
///and additional information such as platformFontFamily and fontVariationAxes.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct FontFace {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.FontFace/properties/fontFamily。
    ///
    ///协议原文：
    ///
    ///<pre>The font-family.</pre>
    ///
    ///
    #[serde(rename = "fontFamily")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub font_family: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.FontFace/properties/fontStyle。
    ///
    ///协议原文：
    ///
    ///<pre>The font-style.</pre>
    ///
    ///
    #[serde(rename = "fontStyle")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub font_style: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.FontFace/properties/fontVariant。
    ///
    ///协议原文：
    ///
    ///<pre>The font-variant.</pre>
    ///
    ///
    #[serde(rename = "fontVariant")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub font_variant: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.FontFace/properties/fontWeight。
    ///
    ///协议原文：
    ///
    ///<pre>The font-weight.</pre>
    ///
    ///
    #[serde(rename = "fontWeight")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub font_weight: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.FontFace/properties/fontStretch。
    ///
    ///协议原文：
    ///
    ///<pre>The font-stretch.</pre>
    ///
    ///
    #[serde(rename = "fontStretch")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub font_stretch: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.FontFace/properties/fontDisplay。
    ///
    ///协议原文：
    ///
    ///<pre>The font-display.</pre>
    ///
    ///
    #[serde(rename = "fontDisplay")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub font_display: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.FontFace/properties/unicodeRange。
    ///
    ///协议原文：
    ///
    ///<pre>The unicode-range.</pre>
    ///
    ///
    #[serde(rename = "unicodeRange")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub unicode_range: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.FontFace/properties/src。
    ///
    ///协议原文：
    ///
    ///<pre>The src.</pre>
    ///
    ///
    #[serde(rename = "src")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub src: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.FontFace/properties/platformFontFamily。
    ///
    ///协议原文：
    ///
    ///<pre>The resolved platform font family</pre>
    ///
    ///
    #[serde(rename = "platformFontFamily")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub platform_font_family: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.FontFace/properties/fontVariationAxes。
    ///
    ///协议原文：
    ///
    ///<pre>Available variation settings (a.k.a. "axes").</pre>
    ///
    ///
    #[serde(rename = "fontVariationAxes")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub font_variation_axes: Option<Vec<crate::css::types::FontVariationAxis>>,
}
impl<'de> serde::Deserialize<'de> for FontFace {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FontFace;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.FontFace/properties/fontFamily。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The font-family.</pre>
                    ///
                    ///
                    #[serde(rename = "fontFamily")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub font_family: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.FontFace/properties/fontStyle。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The font-style.</pre>
                    ///
                    ///
                    #[serde(rename = "fontStyle")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub font_style: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.FontFace/properties/fontVariant。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The font-variant.</pre>
                    ///
                    ///
                    #[serde(rename = "fontVariant")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub font_variant: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.FontFace/properties/fontWeight。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The font-weight.</pre>
                    ///
                    ///
                    #[serde(rename = "fontWeight")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub font_weight: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.FontFace/properties/fontStretch。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The font-stretch.</pre>
                    ///
                    ///
                    #[serde(rename = "fontStretch")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub font_stretch: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.FontFace/properties/fontDisplay。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The font-display.</pre>
                    ///
                    ///
                    #[serde(rename = "fontDisplay")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub font_display: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.FontFace/properties/unicodeRange。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The unicode-range.</pre>
                    ///
                    ///
                    #[serde(rename = "unicodeRange")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub unicode_range: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.FontFace/properties/src。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The src.</pre>
                    ///
                    ///
                    #[serde(rename = "src")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub src: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.FontFace/properties/platformFontFamily。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The resolved platform font family</pre>
                    ///
                    ///
                    #[serde(rename = "platformFontFamily")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub platform_font_family: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.FontFace/properties/fontVariationAxes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Available variation settings (a.k.a. "axes").</pre>
                    ///
                    ///
                    #[serde(rename = "fontVariationAxes")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub font_variation_axes: Option<Vec<crate::css::types::FontVariationAxis>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FontFace {
                    font_family: fields.font_family,
                    font_style: fields.font_style,
                    font_variant: fields.font_variant,
                    font_weight: fields.font_weight,
                    font_stretch: fields.font_stretch,
                    font_display: fields.font_display,
                    unicode_range: fields.unicode_range,
                    src: fields.src,
                    platform_font_family: fields.platform_font_family,
                    font_variation_axes: fields.font_variation_axes,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.FontVariationAxis。
///
///协议原文：
///
///<pre>Information about font variation axes for variable fonts</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct FontVariationAxis {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.FontVariationAxis/properties/tag。
    ///
    ///协议原文：
    ///
    ///<pre>The font-variation-setting tag (a.k.a. "axis tag").</pre>
    ///
    ///
    #[serde(rename = "tag")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub tag: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.FontVariationAxis/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Human-readable variation name in the default language (normally, "en").</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.FontVariationAxis/properties/minValue。
    ///
    ///协议原文：
    ///
    ///<pre>The minimum value (inclusive) the font supports for this tag.</pre>
    ///
    ///
    #[serde(rename = "minValue")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub min_value: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.FontVariationAxis/properties/maxValue。
    ///
    ///协议原文：
    ///
    ///<pre>The maximum value (inclusive) the font supports for this tag.</pre>
    ///
    ///
    #[serde(rename = "maxValue")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub max_value: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.FontVariationAxis/properties/defaultValue。
    ///
    ///协议原文：
    ///
    ///<pre>The default value.</pre>
    ///
    ///
    #[serde(rename = "defaultValue")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub default_value: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for FontVariationAxis {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FontVariationAxis;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.FontVariationAxis/properties/tag。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The font-variation-setting tag (a.k.a. "axis tag").</pre>
                    ///
                    ///
                    #[serde(rename = "tag")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub tag: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.FontVariationAxis/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Human-readable variation name in the default language (normally, "en").</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.FontVariationAxis/properties/minValue。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The minimum value (inclusive) the font supports for this tag.</pre>
                    ///
                    ///
                    #[serde(rename = "minValue")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub min_value: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.FontVariationAxis/properties/maxValue。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The maximum value (inclusive) the font supports for this tag.</pre>
                    ///
                    ///
                    #[serde(rename = "maxValue")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub max_value: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.FontVariationAxis/properties/defaultValue。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The default value.</pre>
                    ///
                    ///
                    #[serde(rename = "defaultValue")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub default_value: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FontVariationAxis {
                    tag: fields.tag,
                    name: fields.name,
                    min_value: fields.min_value,
                    max_value: fields.max_value,
                    default_value: fields.default_value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.InheritedAnimatedStyleEntry。
///
///协议原文：
///
///<pre>Inherited CSS style collection for animated styles from ancestor node.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct InheritedAnimatedStyleEntry {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.InheritedAnimatedStyleEntry/properties/animationStyles。
    ///
    ///协议原文：
    ///
    ///<pre>Styles coming from the animations of the ancestor, if any, in the style inheritance chain.</pre>
    ///
    ///
    #[serde(rename = "animationStyles")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub animation_styles: Option<Vec<crate::css::types::CssAnimationStyle>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.InheritedAnimatedStyleEntry/properties/transitionsStyle。
    ///
    ///协议原文：
    ///
    ///<pre>The style coming from the transitions of the ancestor, if any, in the style inheritance chain.</pre>
    ///
    ///
    #[serde(rename = "transitionsStyle")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub transitions_style: Option<crate::css::types::CssStyle>,
}
impl<'de> serde::Deserialize<'de> for InheritedAnimatedStyleEntry {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InheritedAnimatedStyleEntry;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.InheritedAnimatedStyleEntry/properties/animationStyles。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Styles coming from the animations of the ancestor, if any, in the style inheritance chain.</pre>
                    ///
                    ///
                    #[serde(rename = "animationStyles")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub animation_styles: Option<Vec<crate::css::types::CssAnimationStyle>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.InheritedAnimatedStyleEntry/properties/transitionsStyle。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The style coming from the transitions of the ancestor, if any, in the style inheritance chain.</pre>
                    ///
                    ///
                    #[serde(rename = "transitionsStyle")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub transitions_style: Option<crate::css::types::CssStyle>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(InheritedAnimatedStyleEntry {
                    animation_styles: fields.animation_styles,
                    transitions_style: fields.transitions_style,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.InheritedPseudoElementMatches。
///
///协议原文：
///
///<pre>Inherited pseudo element matches from pseudos of an ancestor node.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct InheritedPseudoElementMatches {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.InheritedPseudoElementMatches/properties/pseudoElements。
    ///
    ///协议原文：
    ///
    ///<pre>Matches of pseudo styles from the pseudos of an ancestor node.</pre>
    ///
    ///
    #[serde(rename = "pseudoElements")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub pseudo_elements: Vec<crate::css::types::PseudoElementMatches>,
}
impl<'de> serde::Deserialize<'de> for InheritedPseudoElementMatches {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InheritedPseudoElementMatches;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.InheritedPseudoElementMatches/properties/pseudoElements。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Matches of pseudo styles from the pseudos of an ancestor node.</pre>
                    ///
                    ///
                    #[serde(rename = "pseudoElements")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub pseudo_elements: Vec<crate::css::types::PseudoElementMatches>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(InheritedPseudoElementMatches {
                    pseudo_elements: fields.pseudo_elements,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.InheritedStyleEntry。
///
///协议原文：
///
///<pre>Inherited CSS rule collection from ancestor node.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct InheritedStyleEntry {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.InheritedStyleEntry/properties/inlineStyle。
    ///
    ///协议原文：
    ///
    ///<pre>The ancestor node's inline style, if any, in the style inheritance chain.</pre>
    ///
    ///
    #[serde(rename = "inlineStyle")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub inline_style: Option<crate::css::types::CssStyle>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.InheritedStyleEntry/properties/matchedCSSRules。
    ///
    ///协议原文：
    ///
    ///<pre>Matches of CSS rules matching the ancestor node in the style inheritance chain.</pre>
    ///
    ///
    #[serde(rename = "matchedCSSRules")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub matched_css_rules: Vec<crate::css::types::RuleMatch>,
}
impl<'de> serde::Deserialize<'de> for InheritedStyleEntry {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InheritedStyleEntry;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.InheritedStyleEntry/properties/inlineStyle。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The ancestor node's inline style, if any, in the style inheritance chain.</pre>
                    ///
                    ///
                    #[serde(rename = "inlineStyle")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub inline_style: Option<crate::css::types::CssStyle>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.InheritedStyleEntry/properties/matchedCSSRules。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Matches of CSS rules matching the ancestor node in the style inheritance chain.</pre>
                    ///
                    ///
                    #[serde(rename = "matchedCSSRules")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub matched_css_rules: Vec<crate::css::types::RuleMatch>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(InheritedStyleEntry {
                    inline_style: fields.inline_style,
                    matched_css_rules: fields.matched_css_rules,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.MediaQuery。
///
///协议原文：
///
///<pre>Media query descriptor.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct MediaQuery {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.MediaQuery/properties/expressions。
    ///
    ///协议原文：
    ///
    ///<pre>Array of media query expressions.</pre>
    ///
    ///
    #[serde(rename = "expressions")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub expressions: Vec<crate::css::types::MediaQueryExpression>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.MediaQuery/properties/active。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the media query condition is satisfied.</pre>
    ///
    ///
    #[serde(rename = "active")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub active: bool,
}
impl<'de> serde::Deserialize<'de> for MediaQuery {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MediaQuery;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.MediaQuery/properties/expressions。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Array of media query expressions.</pre>
                    ///
                    ///
                    #[serde(rename = "expressions")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub expressions: Vec<crate::css::types::MediaQueryExpression>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.MediaQuery/properties/active。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the media query condition is satisfied.</pre>
                    ///
                    ///
                    #[serde(rename = "active")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub active: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(MediaQuery {
                    expressions: fields.expressions,
                    active: fields.active,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.MediaQueryExpression。
///
///协议原文：
///
///<pre>Media query expression descriptor.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct MediaQueryExpression {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.MediaQueryExpression/properties/value。
    ///
    ///协议原文：
    ///
    ///<pre>Media query expression value.</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.MediaQueryExpression/properties/unit。
    ///
    ///协议原文：
    ///
    ///<pre>Media query expression units.</pre>
    ///
    ///
    #[serde(rename = "unit")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub unit: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.MediaQueryExpression/properties/feature。
    ///
    ///协议原文：
    ///
    ///<pre>Media query expression feature.</pre>
    ///
    ///
    #[serde(rename = "feature")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub feature: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.MediaQueryExpression/properties/valueRange。
    ///
    ///协议原文：
    ///
    ///<pre>The associated range of the value text in the enclosing stylesheet (if available).</pre>
    ///
    ///
    #[serde(rename = "valueRange")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub value_range: Option<crate::css::types::SourceRange>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.MediaQueryExpression/properties/computedLength。
    ///
    ///协议原文：
    ///
    ///<pre>Computed length of media query expression (if applicable).</pre>
    ///
    ///
    #[serde(rename = "computedLength")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub computed_length: Option<ydp_core::FiniteF64>,
}
impl<'de> serde::Deserialize<'de> for MediaQueryExpression {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MediaQueryExpression;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.MediaQueryExpression/properties/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Media query expression value.</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.MediaQueryExpression/properties/unit。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Media query expression units.</pre>
                    ///
                    ///
                    #[serde(rename = "unit")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub unit: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.MediaQueryExpression/properties/feature。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Media query expression feature.</pre>
                    ///
                    ///
                    #[serde(rename = "feature")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub feature: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.MediaQueryExpression/properties/valueRange。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The associated range of the value text in the enclosing stylesheet (if available).</pre>
                    ///
                    ///
                    #[serde(rename = "valueRange")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub value_range: Option<crate::css::types::SourceRange>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.MediaQueryExpression/properties/computedLength。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Computed length of media query expression (if applicable).</pre>
                    ///
                    ///
                    #[serde(rename = "computedLength")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub computed_length: Option<ydp_core::FiniteF64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(MediaQueryExpression {
                    value: fields.value,
                    unit: fields.unit,
                    feature: fields.feature,
                    value_range: fields.value_range,
                    computed_length: fields.computed_length,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.PlatformFontUsage。
///
///协议原文：
///
///<pre>Information about amount of glyphs that were rendered with given font.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct PlatformFontUsage {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.PlatformFontUsage/properties/familyName。
    ///
    ///协议原文：
    ///
    ///<pre>Font's family name reported by platform.</pre>
    ///
    ///
    #[serde(rename = "familyName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub family_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.PlatformFontUsage/properties/postScriptName。
    ///
    ///协议原文：
    ///
    ///<pre>Font's PostScript name reported by platform.</pre>
    ///
    ///
    #[serde(rename = "postScriptName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub post_script_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.PlatformFontUsage/properties/isCustomFont。
    ///
    ///协议原文：
    ///
    ///<pre>Indicates if the font was downloaded or resolved locally.</pre>
    ///
    ///
    #[serde(rename = "isCustomFont")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub is_custom_font: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.PlatformFontUsage/properties/glyphCount。
    ///
    ///协议原文：
    ///
    ///<pre>Amount of glyphs that were rendered with this font.</pre>
    ///
    ///
    #[serde(rename = "glyphCount")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub glyph_count: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for PlatformFontUsage {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PlatformFontUsage;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.PlatformFontUsage/properties/familyName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Font's family name reported by platform.</pre>
                    ///
                    ///
                    #[serde(rename = "familyName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub family_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.PlatformFontUsage/properties/postScriptName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Font's PostScript name reported by platform.</pre>
                    ///
                    ///
                    #[serde(rename = "postScriptName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub post_script_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.PlatformFontUsage/properties/isCustomFont。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Indicates if the font was downloaded or resolved locally.</pre>
                    ///
                    ///
                    #[serde(rename = "isCustomFont")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub is_custom_font: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.PlatformFontUsage/properties/glyphCount。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Amount of glyphs that were rendered with this font.</pre>
                    ///
                    ///
                    #[serde(rename = "glyphCount")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub glyph_count: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PlatformFontUsage {
                    family_name: fields.family_name,
                    post_script_name: fields.post_script_name,
                    is_custom_font: fields.is_custom_font,
                    glyph_count: fields.glyph_count,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.PseudoElementMatches。
///
///协议原文：
///
///<pre>CSS rule collection for a single pseudo style.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct PseudoElementMatches {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.PseudoElementMatches/properties/pseudoType。
    ///
    ///协议原文：
    ///
    ///<pre>Pseudo element type.</pre>
    ///
    ///
    #[serde(rename = "pseudoType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub pseudo_type: crate::dom::types::PseudoType,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.PseudoElementMatches/properties/pseudoIdentifier。
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
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.PseudoElementMatches/properties/matches。
    ///
    ///协议原文：
    ///
    ///<pre>Matches of CSS rules applicable to the pseudo style.</pre>
    ///
    ///
    #[serde(rename = "matches")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub matches: Vec<crate::css::types::RuleMatch>,
}
impl<'de> serde::Deserialize<'de> for PseudoElementMatches {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PseudoElementMatches;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.PseudoElementMatches/properties/pseudoType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Pseudo element type.</pre>
                    ///
                    ///
                    #[serde(rename = "pseudoType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub pseudo_type: crate::dom::types::PseudoType,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.PseudoElementMatches/properties/pseudoIdentifier。
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
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.PseudoElementMatches/properties/matches。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Matches of CSS rules applicable to the pseudo style.</pre>
                    ///
                    ///
                    #[serde(rename = "matches")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub matches: Vec<crate::css::types::RuleMatch>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PseudoElementMatches {
                    pseudo_type: fields.pseudo_type,
                    pseudo_identifier: fields.pseudo_identifier,
                    matches: fields.matches,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.RuleMatch。
///
///协议原文：
///
///<pre>Match data for a CSS rule.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct RuleMatch {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.RuleMatch/properties/rule。
    ///
    ///协议原文：
    ///
    ///<pre>CSS rule in the match.</pre>
    ///
    ///
    #[serde(rename = "rule")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub rule: crate::css::types::CssRule,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.RuleMatch/properties/matchingSelectors。
    ///
    ///协议原文：
    ///
    ///<pre>Matching selector indices in the rule's selectorList selectors (0-based).</pre>
    ///
    ///
    #[serde(rename = "matchingSelectors")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub matching_selectors: Vec<i64>,
}
impl<'de> serde::Deserialize<'de> for RuleMatch {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RuleMatch;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.RuleMatch/properties/rule。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>CSS rule in the match.</pre>
                    ///
                    ///
                    #[serde(rename = "rule")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub rule: crate::css::types::CssRule,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.RuleMatch/properties/matchingSelectors。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Matching selector indices in the rule's selectorList selectors (0-based).</pre>
                    ///
                    ///
                    #[serde(rename = "matchingSelectors")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub matching_selectors: Vec<i64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RuleMatch {
                    rule: fields.rule,
                    matching_selectors: fields.matching_selectors,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.RuleUsage。
///
///协议原文：
///
///<pre>CSS coverage information.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct RuleUsage {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.RuleUsage/properties/styleSheetId。
    ///
    ///协议原文：
    ///
    ///<pre>The css style sheet identifier (absent for user agent stylesheet and user-specified
    ///stylesheet rules) this rule came from.</pre>
    ///
    ///
    #[serde(rename = "styleSheetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub style_sheet_id: crate::dom::types::StyleSheetId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.RuleUsage/properties/startOffset。
    ///
    ///协议原文：
    ///
    ///<pre>Offset of the start of the rule (including selector) from the beginning of the stylesheet.</pre>
    ///
    ///
    #[serde(rename = "startOffset")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub start_offset: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.RuleUsage/properties/endOffset。
    ///
    ///协议原文：
    ///
    ///<pre>Offset of the end of the rule body from the beginning of the stylesheet.</pre>
    ///
    ///
    #[serde(rename = "endOffset")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub end_offset: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.RuleUsage/properties/used。
    ///
    ///协议原文：
    ///
    ///<pre>Indicates whether the rule was actually used by some element in the page.</pre>
    ///
    ///
    #[serde(rename = "used")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub used: bool,
}
impl<'de> serde::Deserialize<'de> for RuleUsage {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RuleUsage;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.RuleUsage/properties/styleSheetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The css style sheet identifier (absent for user agent stylesheet and user-specified
                    ///stylesheet rules) this rule came from.</pre>
                    ///
                    ///
                    #[serde(rename = "styleSheetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub style_sheet_id: crate::dom::types::StyleSheetId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.RuleUsage/properties/startOffset。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Offset of the start of the rule (including selector) from the beginning of the stylesheet.</pre>
                    ///
                    ///
                    #[serde(rename = "startOffset")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub start_offset: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.RuleUsage/properties/endOffset。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Offset of the end of the rule body from the beginning of the stylesheet.</pre>
                    ///
                    ///
                    #[serde(rename = "endOffset")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub end_offset: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.RuleUsage/properties/used。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Indicates whether the rule was actually used by some element in the page.</pre>
                    ///
                    ///
                    #[serde(rename = "used")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub used: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RuleUsage {
                    style_sheet_id: fields.style_sheet_id,
                    start_offset: fields.start_offset,
                    end_offset: fields.end_offset,
                    used: fields.used,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.SelectorList。
///
///协议原文：
///
///<pre>Selector list data.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct SelectorList {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.SelectorList/properties/selectors。
    ///
    ///协议原文：
    ///
    ///<pre>Selectors in the list.</pre>
    ///
    ///
    #[serde(rename = "selectors")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub selectors: Vec<crate::css::types::Value>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.SelectorList/properties/text。
    ///
    ///协议原文：
    ///
    ///<pre>Rule selector text.</pre>
    ///
    ///
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: String,
}
impl<'de> serde::Deserialize<'de> for SelectorList {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SelectorList;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.SelectorList/properties/selectors。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Selectors in the list.</pre>
                    ///
                    ///
                    #[serde(rename = "selectors")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub selectors: Vec<crate::css::types::Value>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.SelectorList/properties/text。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Rule selector text.</pre>
                    ///
                    ///
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SelectorList {
                    selectors: fields.selectors,
                    text: fields.text,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.ShorthandEntry。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ShorthandEntry {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.ShorthandEntry/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Shorthand name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.ShorthandEntry/properties/value。
    ///
    ///协议原文：
    ///
    ///<pre>Shorthand value.</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.ShorthandEntry/properties/important。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the property has "!important" annotation (implies `false` if absent).</pre>
    ///
    ///
    #[serde(rename = "important")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub important: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for ShorthandEntry {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ShorthandEntry;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.ShorthandEntry/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Shorthand name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.ShorthandEntry/properties/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Shorthand value.</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.ShorthandEntry/properties/important。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the property has "!important" annotation (implies `false` if absent).</pre>
                    ///
                    ///
                    #[serde(rename = "important")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub important: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ShorthandEntry {
                    name: fields.name,
                    value: fields.value,
                    important: fields.important,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.SourceRange。
///
///协议原文：
///
///<pre>Text range within a resource. All numbers are zero-based.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct SourceRange {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.SourceRange/properties/startLine。
    ///
    ///协议原文：
    ///
    ///<pre>Start line of range.</pre>
    ///
    ///
    #[serde(rename = "startLine")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub start_line: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.SourceRange/properties/startColumn。
    ///
    ///协议原文：
    ///
    ///<pre>Start column of range (inclusive).</pre>
    ///
    ///
    #[serde(rename = "startColumn")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub start_column: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.SourceRange/properties/endLine。
    ///
    ///协议原文：
    ///
    ///<pre>End line of range</pre>
    ///
    ///
    #[serde(rename = "endLine")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub end_line: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.SourceRange/properties/endColumn。
    ///
    ///协议原文：
    ///
    ///<pre>End column of range (exclusive).</pre>
    ///
    ///
    #[serde(rename = "endColumn")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub end_column: i64,
}
impl<'de> serde::Deserialize<'de> for SourceRange {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SourceRange;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.SourceRange/properties/startLine。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Start line of range.</pre>
                    ///
                    ///
                    #[serde(rename = "startLine")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub start_line: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.SourceRange/properties/startColumn。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Start column of range (inclusive).</pre>
                    ///
                    ///
                    #[serde(rename = "startColumn")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub start_column: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.SourceRange/properties/endLine。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>End line of range</pre>
                    ///
                    ///
                    #[serde(rename = "endLine")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub end_line: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.SourceRange/properties/endColumn。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>End column of range (exclusive).</pre>
                    ///
                    ///
                    #[serde(rename = "endColumn")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub end_column: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SourceRange {
                    start_line: fields.start_line,
                    start_column: fields.start_column,
                    end_line: fields.end_line,
                    end_column: fields.end_column,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.Specificity。
///
///协议原文：
///
///<pre>Specificity:
///https://drafts.csswg.org/selectors/#specificity-rules</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct Specificity {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.Specificity/properties/a。
    ///
    ///协议原文：
    ///
    ///<pre>The a component, which represents the number of ID selectors.</pre>
    ///
    ///
    #[serde(rename = "a")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub a: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.Specificity/properties/b。
    ///
    ///协议原文：
    ///
    ///<pre>The b component, which represents the number of class selectors, attributes selectors, and
    ///pseudo-classes.</pre>
    ///
    ///
    #[serde(rename = "b")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub b: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.Specificity/properties/c。
    ///
    ///协议原文：
    ///
    ///<pre>The c component, which represents the number of type selectors and pseudo-elements.</pre>
    ///
    ///
    #[serde(rename = "c")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub c: i64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.Specificity/properties/components。
    ///
    ///协议原文：
    ///
    ///<pre>Per-simple-selector contributions used to explain this specificity.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "components")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub components: Option<Vec<crate::css::types::SpecificityComponent>>,
}
impl<'de> serde::Deserialize<'de> for Specificity {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Specificity;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.Specificity/properties/a。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The a component, which represents the number of ID selectors.</pre>
                    ///
                    ///
                    #[serde(rename = "a")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub a: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.Specificity/properties/b。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The b component, which represents the number of class selectors, attributes selectors, and
                    ///pseudo-classes.</pre>
                    ///
                    ///
                    #[serde(rename = "b")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub b: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.Specificity/properties/c。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The c component, which represents the number of type selectors and pseudo-elements.</pre>
                    ///
                    ///
                    #[serde(rename = "c")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub c: i64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.Specificity/properties/components。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Per-simple-selector contributions used to explain this specificity.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "components")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub components: Option<Vec<crate::css::types::SpecificityComponent>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Specificity {
                    a: fields.a,
                    b: fields.b,
                    c: fields.c,
                    components: fields.components,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.SpecificityComponent。
///
///协议原文：
///
///<pre>Contribution of an individual simple selector to specificity.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SpecificityComponent {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.SpecificityComponent/properties/text。
    ///
    ///协议原文：
    ///
    ///<pre>The simple selector text that contributes to specificity.</pre>
    ///
    ///
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.SpecificityComponent/properties/a。
    ///
    ///协议原文：
    ///
    ///<pre>The a component contribution.</pre>
    ///
    ///
    #[serde(rename = "a")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub a: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.SpecificityComponent/properties/b。
    ///
    ///协议原文：
    ///
    ///<pre>The b component contribution.</pre>
    ///
    ///
    #[serde(rename = "b")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub b: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.SpecificityComponent/properties/c。
    ///
    ///协议原文：
    ///
    ///<pre>The c component contribution.</pre>
    ///
    ///
    #[serde(rename = "c")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub c: i64,
}
impl<'de> serde::Deserialize<'de> for SpecificityComponent {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SpecificityComponent;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.SpecificityComponent/properties/text。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The simple selector text that contributes to specificity.</pre>
                    ///
                    ///
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.SpecificityComponent/properties/a。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The a component contribution.</pre>
                    ///
                    ///
                    #[serde(rename = "a")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub a: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.SpecificityComponent/properties/b。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The b component contribution.</pre>
                    ///
                    ///
                    #[serde(rename = "b")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub b: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.SpecificityComponent/properties/c。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The c component contribution.</pre>
                    ///
                    ///
                    #[serde(rename = "c")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub c: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SpecificityComponent {
                    text: fields.text,
                    a: fields.a,
                    b: fields.b,
                    c: fields.c,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.StyleDeclarationEdit。
///
///协议原文：
///
///<pre>A descriptor of operation to mutate style declaration text.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct StyleDeclarationEdit {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.StyleDeclarationEdit/properties/styleSheetId。
    ///
    ///协议原文：
    ///
    ///<pre>The css style sheet identifier.</pre>
    ///
    ///
    #[serde(rename = "styleSheetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub style_sheet_id: crate::dom::types::StyleSheetId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.StyleDeclarationEdit/properties/range。
    ///
    ///协议原文：
    ///
    ///<pre>The range of the style text in the enclosing stylesheet.</pre>
    ///
    ///
    #[serde(rename = "range")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub range: crate::css::types::SourceRange,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.StyleDeclarationEdit/properties/text。
    ///
    ///协议原文：
    ///
    ///<pre>New style text.</pre>
    ///
    ///
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: String,
}
impl<'de> serde::Deserialize<'de> for StyleDeclarationEdit {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StyleDeclarationEdit;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.StyleDeclarationEdit/properties/styleSheetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The css style sheet identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "styleSheetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub style_sheet_id: crate::dom::types::StyleSheetId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.StyleDeclarationEdit/properties/range。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The range of the style text in the enclosing stylesheet.</pre>
                    ///
                    ///
                    #[serde(rename = "range")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub range: crate::css::types::SourceRange,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.StyleDeclarationEdit/properties/text。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>New style text.</pre>
                    ///
                    ///
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StyleDeclarationEdit {
                    style_sheet_id: fields.style_sheet_id,
                    range: fields.range,
                    text: fields.text,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：CSS.type.StyleSheetOrigin。
///
///协议原文：
///
///<pre>Stylesheet type: "injected" for stylesheets injected via extension, "user-agent" for user-agent
///stylesheets, "inspector" for stylesheets created by the inspector (i.e. those holding the "via
///inspector" rules), "regular" for regular stylesheets.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StyleSheetOrigin {
    ///协议值 "injected"。
    Injected,
    ///协议值 "user-agent"。
    UserAgent,
    ///协议值 "inspector"。
    Inspector,
    ///协议值 "regular"。
    Regular,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl StyleSheetOrigin {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::css::types::StyleSheetOrigin::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Injected => "injected",
            Self::UserAgent => "user-agent",
            Self::Inspector => "inspector",
            Self::Regular => "regular",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for StyleSheetOrigin {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for StyleSheetOrigin {
    fn from(value: String) -> Self {
        match value.as_str() {
            "injected" => StyleSheetOrigin::Injected,
            "user-agent" => StyleSheetOrigin::UserAgent,
            "inspector" => StyleSheetOrigin::Inspector,
            "regular" => StyleSheetOrigin::Regular,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for StyleSheetOrigin {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StyleSheetOrigin {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StyleSheetOrigin;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "injected" => StyleSheetOrigin::Injected,
                    "user-agent" => StyleSheetOrigin::UserAgent,
                    "inspector" => StyleSheetOrigin::Inspector,
                    "regular" => StyleSheetOrigin::Regular,
                    _ => StyleSheetOrigin::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(StyleSheetOrigin::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CSS.type.Value。
///
///协议原文：
///
///<pre>Data for a simple selector (these are delimited by commas in a selector list).</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct Value {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.Value/properties/text。
    ///
    ///协议原文：
    ///
    ///<pre>Value text.</pre>
    ///
    ///
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.Value/properties/range。
    ///
    ///协议原文：
    ///
    ///<pre>Value range in the underlying resource (if available).</pre>
    ///
    ///
    #[serde(rename = "range")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub range: Option<crate::css::types::SourceRange>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.Value/properties/specificity。
    ///
    ///协议原文：
    ///
    ///<pre>Specificity of the selector.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "specificity")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub specificity: Option<crate::css::types::Specificity>,
}
impl<'de> serde::Deserialize<'de> for Value {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Value;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CSS.type.Value/properties/text。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Value text.</pre>
                    ///
                    ///
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.Value/properties/range。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Value range in the underlying resource (if available).</pre>
                    ///
                    ///
                    #[serde(rename = "range")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub range: Option<crate::css::types::SourceRange>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CSS.type.Value/properties/specificity。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specificity of the selector.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "specificity")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub specificity: Option<crate::css::types::Specificity>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Value {
                    text: fields.text,
                    range: fields.range,
                    specificity: fields.specificity,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
