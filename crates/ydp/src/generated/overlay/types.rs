///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.type.BoxStyle。
///
///协议原文：
///
///<pre>Style information for drawing a box.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct BoxStyle {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.BoxStyle/properties/fillColor。
    ///
    ///协议原文：
    ///
    ///<pre>The background color for the box (default: transparent)</pre>
    ///
    ///
    #[serde(rename = "fillColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub fill_color: Option<crate::dom::types::Rgba>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.BoxStyle/properties/hatchColor。
    ///
    ///协议原文：
    ///
    ///<pre>The hatching color for the box (default: transparent)</pre>
    ///
    ///
    #[serde(rename = "hatchColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub hatch_color: Option<crate::dom::types::Rgba>,
}
impl<'de> serde::Deserialize<'de> for BoxStyle {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BoxStyle;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.BoxStyle/properties/fillColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The background color for the box (default: transparent)</pre>
                    ///
                    ///
                    #[serde(rename = "fillColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub fill_color: Option<crate::dom::types::Rgba>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.BoxStyle/properties/hatchColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The hatching color for the box (default: transparent)</pre>
                    ///
                    ///
                    #[serde(rename = "hatchColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub hatch_color: Option<crate::dom::types::Rgba>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(BoxStyle {
                    fill_color: fields.fill_color,
                    hatch_color: fields.hatch_color,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Overlay.type.ColorFormat。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ColorFormat {
    ///协议值 "rgb"。
    Rgb,
    ///协议值 "hsl"。
    Hsl,
    ///协议值 "hwb"。
    Hwb,
    ///协议值 "hex"。
    Hex,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ColorFormat {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::overlay::types::ColorFormat::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Rgb => "rgb",
            Self::Hsl => "hsl",
            Self::Hwb => "hwb",
            Self::Hex => "hex",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ColorFormat {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ColorFormat {
    fn from(value: String) -> Self {
        match value.as_str() {
            "rgb" => ColorFormat::Rgb,
            "hsl" => ColorFormat::Hsl,
            "hwb" => ColorFormat::Hwb,
            "hex" => ColorFormat::Hex,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ColorFormat {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ColorFormat {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ColorFormat;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "rgb" => ColorFormat::Rgb,
                    "hsl" => ColorFormat::Hsl,
                    "hwb" => ColorFormat::Hwb,
                    "hex" => ColorFormat::Hex,
                    _ => ColorFormat::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ColorFormat::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.type.ContainerQueryContainerHighlightConfig。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ContainerQueryContainerHighlightConfig {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.ContainerQueryContainerHighlightConfig/properties/containerBorder。
    ///
    ///协议原文：
    ///
    ///<pre>The style of the container border.</pre>
    ///
    ///
    #[serde(rename = "containerBorder")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub container_border: Option<crate::overlay::types::LineStyle>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.ContainerQueryContainerHighlightConfig/properties/descendantBorder。
    ///
    ///协议原文：
    ///
    ///<pre>The style of the descendants' borders.</pre>
    ///
    ///
    #[serde(rename = "descendantBorder")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub descendant_border: Option<crate::overlay::types::LineStyle>,
}
impl<'de> serde::Deserialize<'de> for ContainerQueryContainerHighlightConfig {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContainerQueryContainerHighlightConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.ContainerQueryContainerHighlightConfig/properties/containerBorder。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The style of the container border.</pre>
                    ///
                    ///
                    #[serde(rename = "containerBorder")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub container_border: Option<crate::overlay::types::LineStyle>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.ContainerQueryContainerHighlightConfig/properties/descendantBorder。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The style of the descendants' borders.</pre>
                    ///
                    ///
                    #[serde(rename = "descendantBorder")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub descendant_border: Option<crate::overlay::types::LineStyle>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ContainerQueryContainerHighlightConfig {
                    container_border: fields.container_border,
                    descendant_border: fields.descendant_border,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.type.ContainerQueryHighlightConfig。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ContainerQueryHighlightConfig {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.ContainerQueryHighlightConfig/properties/containerQueryContainerHighlightConfig。
    ///
    ///协议原文：
    ///
    ///<pre>A descriptor for the highlight appearance of container query containers.</pre>
    ///
    ///
    #[serde(rename = "containerQueryContainerHighlightConfig")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub container_query_container_highlight_config:
        crate::overlay::types::ContainerQueryContainerHighlightConfig,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.ContainerQueryHighlightConfig/properties/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the container node to highlight.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for ContainerQueryHighlightConfig {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContainerQueryHighlightConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.ContainerQueryHighlightConfig/properties/containerQueryContainerHighlightConfig。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A descriptor for the highlight appearance of container query containers.</pre>
                    ///
                    ///
                    #[serde(rename = "containerQueryContainerHighlightConfig")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub container_query_container_highlight_config:
                        crate::overlay::types::ContainerQueryContainerHighlightConfig,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.ContainerQueryHighlightConfig/properties/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the container node to highlight.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ContainerQueryHighlightConfig {
                    container_query_container_highlight_config: fields
                        .container_query_container_highlight_config,
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Overlay.type.ContrastAlgorithm。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ContrastAlgorithm {
    ///协议值 "aa"。
    Aa,
    ///协议值 "aaa"。
    Aaa,
    ///协议值 "apca"。
    Apca,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ContrastAlgorithm {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::overlay::types::ContrastAlgorithm::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Aa => "aa",
            Self::Aaa => "aaa",
            Self::Apca => "apca",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ContrastAlgorithm {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ContrastAlgorithm {
    fn from(value: String) -> Self {
        match value.as_str() {
            "aa" => ContrastAlgorithm::Aa,
            "aaa" => ContrastAlgorithm::Aaa,
            "apca" => ContrastAlgorithm::Apca,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ContrastAlgorithm {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ContrastAlgorithm {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContrastAlgorithm;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "aa" => ContrastAlgorithm::Aa,
                    "aaa" => ContrastAlgorithm::Aaa,
                    "apca" => ContrastAlgorithm::Apca,
                    _ => ContrastAlgorithm::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ContrastAlgorithm::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.type.DisplayCutoutConfig。
///
///协议原文：
///
///<pre>Configuration for a display cutout.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct DisplayCutoutConfig {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.DisplayCutoutConfig/properties/rect。
    ///
    ///协议原文：
    ///
    ///<pre>A rectangle representing the cutout bounds.</pre>
    ///
    ///
    #[serde(rename = "rect")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub rect: crate::dom::types::Rect,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.DisplayCutoutConfig/properties/shape。
    ///
    ///协议原文：
    ///
    ///<pre>Shape used to draw the cutout.</pre>
    ///
    ///
    #[serde(rename = "shape")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub shape: crate::overlay::types::DisplayCutoutShape,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.DisplayCutoutConfig/properties/borderRadius。
    ///
    ///协议原文：
    ///
    ///<pre>Border radius for rounded cutout shapes.</pre>
    ///
    ///
    #[serde(rename = "borderRadius")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub border_radius: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.DisplayCutoutConfig/properties/upperRadius。
    ///
    ///协议原文：
    ///
    ///<pre>Upper shoulder radius for notch cutout shapes.</pre>
    ///
    ///
    #[serde(rename = "upperRadius")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_radius: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.DisplayCutoutConfig/properties/lowerRadius。
    ///
    ///协议原文：
    ///
    ///<pre>Lower transition radius for notch cutout shapes.</pre>
    ///
    ///
    #[serde(rename = "lowerRadius")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub lower_radius: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.DisplayCutoutConfig/properties/cx。
    ///
    ///协议原文：
    ///
    ///<pre>Center x coordinate for circle cutout shapes.</pre>
    ///
    ///
    #[serde(rename = "cx")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub cx: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.DisplayCutoutConfig/properties/cy。
    ///
    ///协议原文：
    ///
    ///<pre>Center y coordinate for circle cutout shapes.</pre>
    ///
    ///
    #[serde(rename = "cy")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub cy: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.DisplayCutoutConfig/properties/radius。
    ///
    ///协议原文：
    ///
    ///<pre>Radius for circle cutout shapes.</pre>
    ///
    ///
    #[serde(rename = "radius")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub radius: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.DisplayCutoutConfig/properties/contentColor。
    ///
    ///协议原文：
    ///
    ///<pre>The cutout fill color (default: black).</pre>
    ///
    ///
    #[serde(rename = "contentColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub content_color: Option<crate::dom::types::Rgba>,
}
impl<'de> serde::Deserialize<'de> for DisplayCutoutConfig {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DisplayCutoutConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.DisplayCutoutConfig/properties/rect。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A rectangle representing the cutout bounds.</pre>
                    ///
                    ///
                    #[serde(rename = "rect")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub rect: crate::dom::types::Rect,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.DisplayCutoutConfig/properties/shape。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Shape used to draw the cutout.</pre>
                    ///
                    ///
                    #[serde(rename = "shape")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub shape: crate::overlay::types::DisplayCutoutShape,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.DisplayCutoutConfig/properties/borderRadius。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Border radius for rounded cutout shapes.</pre>
                    ///
                    ///
                    #[serde(rename = "borderRadius")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub border_radius: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.DisplayCutoutConfig/properties/upperRadius。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Upper shoulder radius for notch cutout shapes.</pre>
                    ///
                    ///
                    #[serde(rename = "upperRadius")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub upper_radius: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.DisplayCutoutConfig/properties/lowerRadius。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Lower transition radius for notch cutout shapes.</pre>
                    ///
                    ///
                    #[serde(rename = "lowerRadius")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub lower_radius: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.DisplayCutoutConfig/properties/cx。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Center x coordinate for circle cutout shapes.</pre>
                    ///
                    ///
                    #[serde(rename = "cx")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub cx: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.DisplayCutoutConfig/properties/cy。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Center y coordinate for circle cutout shapes.</pre>
                    ///
                    ///
                    #[serde(rename = "cy")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub cy: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.DisplayCutoutConfig/properties/radius。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Radius for circle cutout shapes.</pre>
                    ///
                    ///
                    #[serde(rename = "radius")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub radius: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.DisplayCutoutConfig/properties/contentColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The cutout fill color (default: black).</pre>
                    ///
                    ///
                    #[serde(rename = "contentColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub content_color: Option<crate::dom::types::Rgba>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DisplayCutoutConfig {
                    rect: fields.rect,
                    shape: fields.shape,
                    border_radius: fields.border_radius,
                    upper_radius: fields.upper_radius,
                    lower_radius: fields.lower_radius,
                    cx: fields.cx,
                    cy: fields.cy,
                    radius: fields.radius,
                    content_color: fields.content_color,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Overlay.type.DisplayCutoutShape。
///
///协议原文：
///
///<pre>Supported display cutout shapes.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DisplayCutoutShape {
    ///协议值 "pill"。
    Pill,
    ///协议值 "notch"。
    Notch,
    ///协议值 "circle"。
    Circle,
    ///协议值 "rectangle"。
    Rectangle,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl DisplayCutoutShape {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::overlay::types::DisplayCutoutShape::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pill => "pill",
            Self::Notch => "notch",
            Self::Circle => "circle",
            Self::Rectangle => "rectangle",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for DisplayCutoutShape {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for DisplayCutoutShape {
    fn from(value: String) -> Self {
        match value.as_str() {
            "pill" => DisplayCutoutShape::Pill,
            "notch" => DisplayCutoutShape::Notch,
            "circle" => DisplayCutoutShape::Circle,
            "rectangle" => DisplayCutoutShape::Rectangle,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for DisplayCutoutShape {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DisplayCutoutShape {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DisplayCutoutShape;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "pill" => DisplayCutoutShape::Pill,
                    "notch" => DisplayCutoutShape::Notch,
                    "circle" => DisplayCutoutShape::Circle,
                    "rectangle" => DisplayCutoutShape::Rectangle,
                    _ => DisplayCutoutShape::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(DisplayCutoutShape::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.type.FlexContainerHighlightConfig。
///
///协议原文：
///
///<pre>Configuration data for the highlighting of Flex container elements.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct FlexContainerHighlightConfig {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.FlexContainerHighlightConfig/properties/containerBorder。
    ///
    ///协议原文：
    ///
    ///<pre>The style of the container border</pre>
    ///
    ///
    #[serde(rename = "containerBorder")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub container_border: Option<crate::overlay::types::LineStyle>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.FlexContainerHighlightConfig/properties/lineSeparator。
    ///
    ///协议原文：
    ///
    ///<pre>The style of the separator between lines</pre>
    ///
    ///
    #[serde(rename = "lineSeparator")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub line_separator: Option<crate::overlay::types::LineStyle>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.FlexContainerHighlightConfig/properties/itemSeparator。
    ///
    ///协议原文：
    ///
    ///<pre>The style of the separator between items</pre>
    ///
    ///
    #[serde(rename = "itemSeparator")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub item_separator: Option<crate::overlay::types::LineStyle>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.FlexContainerHighlightConfig/properties/mainDistributedSpace。
    ///
    ///协议原文：
    ///
    ///<pre>Style of content-distribution space on the main axis (justify-content).</pre>
    ///
    ///
    #[serde(rename = "mainDistributedSpace")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub main_distributed_space: Option<crate::overlay::types::BoxStyle>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.FlexContainerHighlightConfig/properties/crossDistributedSpace。
    ///
    ///协议原文：
    ///
    ///<pre>Style of content-distribution space on the cross axis (align-content).</pre>
    ///
    ///
    #[serde(rename = "crossDistributedSpace")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub cross_distributed_space: Option<crate::overlay::types::BoxStyle>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.FlexContainerHighlightConfig/properties/rowGapSpace。
    ///
    ///协议原文：
    ///
    ///<pre>Style of empty space caused by row gaps (gap/row-gap).</pre>
    ///
    ///
    #[serde(rename = "rowGapSpace")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub row_gap_space: Option<crate::overlay::types::BoxStyle>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.FlexContainerHighlightConfig/properties/columnGapSpace。
    ///
    ///协议原文：
    ///
    ///<pre>Style of empty space caused by columns gaps (gap/column-gap).</pre>
    ///
    ///
    #[serde(rename = "columnGapSpace")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub column_gap_space: Option<crate::overlay::types::BoxStyle>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.FlexContainerHighlightConfig/properties/crossAlignment。
    ///
    ///协议原文：
    ///
    ///<pre>Style of the self-alignment line (align-items).</pre>
    ///
    ///
    #[serde(rename = "crossAlignment")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub cross_alignment: Option<crate::overlay::types::LineStyle>,
}
impl<'de> serde::Deserialize<'de> for FlexContainerHighlightConfig {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FlexContainerHighlightConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.FlexContainerHighlightConfig/properties/containerBorder。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The style of the container border</pre>
                    ///
                    ///
                    #[serde(rename = "containerBorder")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub container_border: Option<crate::overlay::types::LineStyle>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.FlexContainerHighlightConfig/properties/lineSeparator。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The style of the separator between lines</pre>
                    ///
                    ///
                    #[serde(rename = "lineSeparator")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub line_separator: Option<crate::overlay::types::LineStyle>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.FlexContainerHighlightConfig/properties/itemSeparator。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The style of the separator between items</pre>
                    ///
                    ///
                    #[serde(rename = "itemSeparator")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub item_separator: Option<crate::overlay::types::LineStyle>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.FlexContainerHighlightConfig/properties/mainDistributedSpace。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Style of content-distribution space on the main axis (justify-content).</pre>
                    ///
                    ///
                    #[serde(rename = "mainDistributedSpace")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub main_distributed_space: Option<crate::overlay::types::BoxStyle>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.FlexContainerHighlightConfig/properties/crossDistributedSpace。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Style of content-distribution space on the cross axis (align-content).</pre>
                    ///
                    ///
                    #[serde(rename = "crossDistributedSpace")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub cross_distributed_space: Option<crate::overlay::types::BoxStyle>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.FlexContainerHighlightConfig/properties/rowGapSpace。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Style of empty space caused by row gaps (gap/row-gap).</pre>
                    ///
                    ///
                    #[serde(rename = "rowGapSpace")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub row_gap_space: Option<crate::overlay::types::BoxStyle>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.FlexContainerHighlightConfig/properties/columnGapSpace。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Style of empty space caused by columns gaps (gap/column-gap).</pre>
                    ///
                    ///
                    #[serde(rename = "columnGapSpace")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub column_gap_space: Option<crate::overlay::types::BoxStyle>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.FlexContainerHighlightConfig/properties/crossAlignment。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Style of the self-alignment line (align-items).</pre>
                    ///
                    ///
                    #[serde(rename = "crossAlignment")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub cross_alignment: Option<crate::overlay::types::LineStyle>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FlexContainerHighlightConfig {
                    container_border: fields.container_border,
                    line_separator: fields.line_separator,
                    item_separator: fields.item_separator,
                    main_distributed_space: fields.main_distributed_space,
                    cross_distributed_space: fields.cross_distributed_space,
                    row_gap_space: fields.row_gap_space,
                    column_gap_space: fields.column_gap_space,
                    cross_alignment: fields.cross_alignment,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.type.FlexItemHighlightConfig。
///
///协议原文：
///
///<pre>Configuration data for the highlighting of Flex item elements.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct FlexItemHighlightConfig {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.FlexItemHighlightConfig/properties/baseSizeBox。
    ///
    ///协议原文：
    ///
    ///<pre>Style of the box representing the item's base size</pre>
    ///
    ///
    #[serde(rename = "baseSizeBox")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub base_size_box: Option<crate::overlay::types::BoxStyle>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.FlexItemHighlightConfig/properties/baseSizeBorder。
    ///
    ///协议原文：
    ///
    ///<pre>Style of the border around the box representing the item's base size</pre>
    ///
    ///
    #[serde(rename = "baseSizeBorder")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub base_size_border: Option<crate::overlay::types::LineStyle>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.FlexItemHighlightConfig/properties/flexibilityArrow。
    ///
    ///协议原文：
    ///
    ///<pre>Style of the arrow representing if the item grew or shrank</pre>
    ///
    ///
    #[serde(rename = "flexibilityArrow")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub flexibility_arrow: Option<crate::overlay::types::LineStyle>,
}
impl<'de> serde::Deserialize<'de> for FlexItemHighlightConfig {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FlexItemHighlightConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.FlexItemHighlightConfig/properties/baseSizeBox。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Style of the box representing the item's base size</pre>
                    ///
                    ///
                    #[serde(rename = "baseSizeBox")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub base_size_box: Option<crate::overlay::types::BoxStyle>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.FlexItemHighlightConfig/properties/baseSizeBorder。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Style of the border around the box representing the item's base size</pre>
                    ///
                    ///
                    #[serde(rename = "baseSizeBorder")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub base_size_border: Option<crate::overlay::types::LineStyle>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.FlexItemHighlightConfig/properties/flexibilityArrow。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Style of the arrow representing if the item grew or shrank</pre>
                    ///
                    ///
                    #[serde(rename = "flexibilityArrow")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub flexibility_arrow: Option<crate::overlay::types::LineStyle>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FlexItemHighlightConfig {
                    base_size_box: fields.base_size_box,
                    base_size_border: fields.base_size_border,
                    flexibility_arrow: fields.flexibility_arrow,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.type.FlexNodeHighlightConfig。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FlexNodeHighlightConfig {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.FlexNodeHighlightConfig/properties/flexContainerHighlightConfig。
    ///
    ///协议原文：
    ///
    ///<pre>A descriptor for the highlight appearance of flex containers.</pre>
    ///
    ///
    #[serde(rename = "flexContainerHighlightConfig")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub flex_container_highlight_config: crate::overlay::types::FlexContainerHighlightConfig,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.FlexNodeHighlightConfig/properties/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the node to highlight.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for FlexNodeHighlightConfig {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FlexNodeHighlightConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.FlexNodeHighlightConfig/properties/flexContainerHighlightConfig。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A descriptor for the highlight appearance of flex containers.</pre>
                    ///
                    ///
                    #[serde(rename = "flexContainerHighlightConfig")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub flex_container_highlight_config:
                        crate::overlay::types::FlexContainerHighlightConfig,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.FlexNodeHighlightConfig/properties/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the node to highlight.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FlexNodeHighlightConfig {
                    flex_container_highlight_config: fields.flex_container_highlight_config,
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.type.GridHighlightConfig。
///
///协议原文：
///
///<pre>Configuration data for the highlighting of Grid elements.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct GridHighlightConfig {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/showGridExtensionLines。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the extension lines from grid cells to the rulers should be shown (default: false).</pre>
    ///
    ///
    #[serde(rename = "showGridExtensionLines")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub show_grid_extension_lines: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/showPositiveLineNumbers。
    ///
    ///协议原文：
    ///
    ///<pre>Show Positive line number labels (default: false).</pre>
    ///
    ///
    #[serde(rename = "showPositiveLineNumbers")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub show_positive_line_numbers: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/showNegativeLineNumbers。
    ///
    ///协议原文：
    ///
    ///<pre>Show Negative line number labels (default: false).</pre>
    ///
    ///
    #[serde(rename = "showNegativeLineNumbers")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub show_negative_line_numbers: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/showAreaNames。
    ///
    ///协议原文：
    ///
    ///<pre>Show area name labels (default: false).</pre>
    ///
    ///
    #[serde(rename = "showAreaNames")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub show_area_names: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/showLineNames。
    ///
    ///协议原文：
    ///
    ///<pre>Show line name labels (default: false).</pre>
    ///
    ///
    #[serde(rename = "showLineNames")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub show_line_names: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/showTrackSizes。
    ///
    ///协议原文：
    ///
    ///<pre>Show track size labels (default: false).</pre>
    ///
    ///
    #[serde(rename = "showTrackSizes")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub show_track_sizes: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/gridBorderColor。
    ///
    ///协议原文：
    ///
    ///<pre>The grid container border highlight color (default: transparent).</pre>
    ///
    ///
    #[serde(rename = "gridBorderColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub grid_border_color: Option<crate::dom::types::Rgba>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/cellBorderColor。
    ///
    ///协议原文：
    ///
    ///<pre>The cell border color (default: transparent). Deprecated, please use rowLineColor and columnLineColor instead.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "cellBorderColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub cell_border_color: Option<crate::dom::types::Rgba>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/rowLineColor。
    ///
    ///协议原文：
    ///
    ///<pre>The row line color (default: transparent).</pre>
    ///
    ///
    #[serde(rename = "rowLineColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub row_line_color: Option<crate::dom::types::Rgba>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/columnLineColor。
    ///
    ///协议原文：
    ///
    ///<pre>The column line color (default: transparent).</pre>
    ///
    ///
    #[serde(rename = "columnLineColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub column_line_color: Option<crate::dom::types::Rgba>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/gridBorderDash。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the grid border is dashed (default: false).</pre>
    ///
    ///
    #[serde(rename = "gridBorderDash")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub grid_border_dash: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/cellBorderDash。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the cell border is dashed (default: false). Deprecated, please us rowLineDash and columnLineDash instead.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "cellBorderDash")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub cell_border_dash: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/rowLineDash。
    ///
    ///协议原文：
    ///
    ///<pre>Whether row lines are dashed (default: false).</pre>
    ///
    ///
    #[serde(rename = "rowLineDash")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub row_line_dash: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/columnLineDash。
    ///
    ///协议原文：
    ///
    ///<pre>Whether column lines are dashed (default: false).</pre>
    ///
    ///
    #[serde(rename = "columnLineDash")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub column_line_dash: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/rowGapColor。
    ///
    ///协议原文：
    ///
    ///<pre>The row gap highlight fill color (default: transparent).</pre>
    ///
    ///
    #[serde(rename = "rowGapColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub row_gap_color: Option<crate::dom::types::Rgba>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/rowHatchColor。
    ///
    ///协议原文：
    ///
    ///<pre>The row gap hatching fill color (default: transparent).</pre>
    ///
    ///
    #[serde(rename = "rowHatchColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub row_hatch_color: Option<crate::dom::types::Rgba>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/columnGapColor。
    ///
    ///协议原文：
    ///
    ///<pre>The column gap highlight fill color (default: transparent).</pre>
    ///
    ///
    #[serde(rename = "columnGapColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub column_gap_color: Option<crate::dom::types::Rgba>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/columnHatchColor。
    ///
    ///协议原文：
    ///
    ///<pre>The column gap hatching fill color (default: transparent).</pre>
    ///
    ///
    #[serde(rename = "columnHatchColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub column_hatch_color: Option<crate::dom::types::Rgba>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/areaBorderColor。
    ///
    ///协议原文：
    ///
    ///<pre>The named grid areas border color (Default: transparent).</pre>
    ///
    ///
    #[serde(rename = "areaBorderColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub area_border_color: Option<crate::dom::types::Rgba>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/gridBackgroundColor。
    ///
    ///协议原文：
    ///
    ///<pre>The grid container background color (Default: transparent).</pre>
    ///
    ///
    #[serde(rename = "gridBackgroundColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub grid_background_color: Option<crate::dom::types::Rgba>,
}
impl<'de> serde::Deserialize<'de> for GridHighlightConfig {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GridHighlightConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/showGridExtensionLines。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the extension lines from grid cells to the rulers should be shown (default: false).</pre>
                    ///
                    ///
                    #[serde(rename = "showGridExtensionLines")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub show_grid_extension_lines: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/showPositiveLineNumbers。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Show Positive line number labels (default: false).</pre>
                    ///
                    ///
                    #[serde(rename = "showPositiveLineNumbers")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub show_positive_line_numbers: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/showNegativeLineNumbers。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Show Negative line number labels (default: false).</pre>
                    ///
                    ///
                    #[serde(rename = "showNegativeLineNumbers")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub show_negative_line_numbers: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/showAreaNames。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Show area name labels (default: false).</pre>
                    ///
                    ///
                    #[serde(rename = "showAreaNames")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub show_area_names: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/showLineNames。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Show line name labels (default: false).</pre>
                    ///
                    ///
                    #[serde(rename = "showLineNames")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub show_line_names: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/showTrackSizes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Show track size labels (default: false).</pre>
                    ///
                    ///
                    #[serde(rename = "showTrackSizes")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub show_track_sizes: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/gridBorderColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The grid container border highlight color (default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "gridBorderColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub grid_border_color: Option<crate::dom::types::Rgba>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/cellBorderColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The cell border color (default: transparent). Deprecated, please use rowLineColor and columnLineColor instead.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "cellBorderColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub cell_border_color: Option<crate::dom::types::Rgba>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/rowLineColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The row line color (default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "rowLineColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub row_line_color: Option<crate::dom::types::Rgba>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/columnLineColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The column line color (default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "columnLineColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub column_line_color: Option<crate::dom::types::Rgba>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/gridBorderDash。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the grid border is dashed (default: false).</pre>
                    ///
                    ///
                    #[serde(rename = "gridBorderDash")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub grid_border_dash: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/cellBorderDash。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the cell border is dashed (default: false). Deprecated, please us rowLineDash and columnLineDash instead.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "cellBorderDash")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub cell_border_dash: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/rowLineDash。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether row lines are dashed (default: false).</pre>
                    ///
                    ///
                    #[serde(rename = "rowLineDash")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub row_line_dash: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/columnLineDash。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether column lines are dashed (default: false).</pre>
                    ///
                    ///
                    #[serde(rename = "columnLineDash")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub column_line_dash: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/rowGapColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The row gap highlight fill color (default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "rowGapColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub row_gap_color: Option<crate::dom::types::Rgba>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/rowHatchColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The row gap hatching fill color (default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "rowHatchColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub row_hatch_color: Option<crate::dom::types::Rgba>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/columnGapColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The column gap highlight fill color (default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "columnGapColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub column_gap_color: Option<crate::dom::types::Rgba>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/columnHatchColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The column gap hatching fill color (default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "columnHatchColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub column_hatch_color: Option<crate::dom::types::Rgba>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/areaBorderColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The named grid areas border color (Default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "areaBorderColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub area_border_color: Option<crate::dom::types::Rgba>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.GridHighlightConfig/properties/gridBackgroundColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The grid container background color (Default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "gridBackgroundColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub grid_background_color: Option<crate::dom::types::Rgba>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GridHighlightConfig {
                    show_grid_extension_lines: fields.show_grid_extension_lines,
                    show_positive_line_numbers: fields.show_positive_line_numbers,
                    show_negative_line_numbers: fields.show_negative_line_numbers,
                    show_area_names: fields.show_area_names,
                    show_line_names: fields.show_line_names,
                    show_track_sizes: fields.show_track_sizes,
                    grid_border_color: fields.grid_border_color,
                    cell_border_color: fields.cell_border_color,
                    row_line_color: fields.row_line_color,
                    column_line_color: fields.column_line_color,
                    grid_border_dash: fields.grid_border_dash,
                    cell_border_dash: fields.cell_border_dash,
                    row_line_dash: fields.row_line_dash,
                    column_line_dash: fields.column_line_dash,
                    row_gap_color: fields.row_gap_color,
                    row_hatch_color: fields.row_hatch_color,
                    column_gap_color: fields.column_gap_color,
                    column_hatch_color: fields.column_hatch_color,
                    area_border_color: fields.area_border_color,
                    grid_background_color: fields.grid_background_color,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.type.GridNodeHighlightConfig。
///
///协议原文：
///
///<pre>Configurations for Persistent Grid Highlight</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct GridNodeHighlightConfig {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.GridNodeHighlightConfig/properties/gridHighlightConfig。
    ///
    ///协议原文：
    ///
    ///<pre>A descriptor for the highlight appearance.</pre>
    ///
    ///
    #[serde(rename = "gridHighlightConfig")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub grid_highlight_config: crate::overlay::types::GridHighlightConfig,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.GridNodeHighlightConfig/properties/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the node to highlight.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for GridNodeHighlightConfig {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GridNodeHighlightConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.GridNodeHighlightConfig/properties/gridHighlightConfig。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A descriptor for the highlight appearance.</pre>
                    ///
                    ///
                    #[serde(rename = "gridHighlightConfig")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub grid_highlight_config: crate::overlay::types::GridHighlightConfig,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.GridNodeHighlightConfig/properties/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the node to highlight.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GridNodeHighlightConfig {
                    grid_highlight_config: fields.grid_highlight_config,
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.type.HighlightConfig。
///
///协议原文：
///
///<pre>Configuration data for the highlighting of page elements.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct HighlightConfig {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/showInfo。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the node info tooltip should be shown (default: false).</pre>
    ///
    ///
    #[serde(rename = "showInfo")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub show_info: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/showStyles。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the node styles in the tooltip (default: false).</pre>
    ///
    ///
    #[serde(rename = "showStyles")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub show_styles: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/showRulers。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the rulers should be shown (default: false).</pre>
    ///
    ///
    #[serde(rename = "showRulers")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub show_rulers: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/showAccessibilityInfo。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the a11y info should be shown (default: true).</pre>
    ///
    ///
    #[serde(rename = "showAccessibilityInfo")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub show_accessibility_info: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/showExtensionLines。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the extension lines from node to the rulers should be shown (default: false).</pre>
    ///
    ///
    #[serde(rename = "showExtensionLines")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub show_extension_lines: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/contentColor。
    ///
    ///协议原文：
    ///
    ///<pre>The content box highlight fill color (default: transparent).</pre>
    ///
    ///
    #[serde(rename = "contentColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub content_color: Option<crate::dom::types::Rgba>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/paddingColor。
    ///
    ///协议原文：
    ///
    ///<pre>The padding highlight fill color (default: transparent).</pre>
    ///
    ///
    #[serde(rename = "paddingColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub padding_color: Option<crate::dom::types::Rgba>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/borderColor。
    ///
    ///协议原文：
    ///
    ///<pre>The border highlight fill color (default: transparent).</pre>
    ///
    ///
    #[serde(rename = "borderColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub border_color: Option<crate::dom::types::Rgba>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/marginColor。
    ///
    ///协议原文：
    ///
    ///<pre>The margin highlight fill color (default: transparent).</pre>
    ///
    ///
    #[serde(rename = "marginColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_color: Option<crate::dom::types::Rgba>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/eventTargetColor。
    ///
    ///协议原文：
    ///
    ///<pre>The event target element highlight fill color (default: transparent).</pre>
    ///
    ///
    #[serde(rename = "eventTargetColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub event_target_color: Option<crate::dom::types::Rgba>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/shapeColor。
    ///
    ///协议原文：
    ///
    ///<pre>The shape outside fill color (default: transparent).</pre>
    ///
    ///
    #[serde(rename = "shapeColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub shape_color: Option<crate::dom::types::Rgba>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/shapeMarginColor。
    ///
    ///协议原文：
    ///
    ///<pre>The shape margin fill color (default: transparent).</pre>
    ///
    ///
    #[serde(rename = "shapeMarginColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub shape_margin_color: Option<crate::dom::types::Rgba>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/cssGridColor。
    ///
    ///协议原文：
    ///
    ///<pre>The grid layout color (default: transparent).</pre>
    ///
    ///
    #[serde(rename = "cssGridColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub css_grid_color: Option<crate::dom::types::Rgba>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/colorFormat。
    ///
    ///协议原文：
    ///
    ///<pre>The color format used to format color styles (default: hex).</pre>
    ///
    ///
    #[serde(rename = "colorFormat")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub color_format: Option<crate::overlay::types::ColorFormat>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/gridHighlightConfig。
    ///
    ///协议原文：
    ///
    ///<pre>The grid layout highlight configuration (default: all transparent).</pre>
    ///
    ///
    #[serde(rename = "gridHighlightConfig")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub grid_highlight_config: Option<crate::overlay::types::GridHighlightConfig>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/flexContainerHighlightConfig。
    ///
    ///协议原文：
    ///
    ///<pre>The flex container highlight configuration (default: all transparent).</pre>
    ///
    ///
    #[serde(rename = "flexContainerHighlightConfig")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub flex_container_highlight_config:
        Option<crate::overlay::types::FlexContainerHighlightConfig>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/flexItemHighlightConfig。
    ///
    ///协议原文：
    ///
    ///<pre>The flex item highlight configuration (default: all transparent).</pre>
    ///
    ///
    #[serde(rename = "flexItemHighlightConfig")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub flex_item_highlight_config: Option<crate::overlay::types::FlexItemHighlightConfig>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/contrastAlgorithm。
    ///
    ///协议原文：
    ///
    ///<pre>The contrast algorithm to use for the contrast ratio (default: aa).</pre>
    ///
    ///
    #[serde(rename = "contrastAlgorithm")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub contrast_algorithm: Option<crate::overlay::types::ContrastAlgorithm>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/containerQueryContainerHighlightConfig。
    ///
    ///协议原文：
    ///
    ///<pre>The container query container highlight configuration (default: all transparent).</pre>
    ///
    ///
    #[serde(rename = "containerQueryContainerHighlightConfig")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub container_query_container_highlight_config:
        Option<crate::overlay::types::ContainerQueryContainerHighlightConfig>,
}
impl<'de> serde::Deserialize<'de> for HighlightConfig {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HighlightConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/showInfo。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the node info tooltip should be shown (default: false).</pre>
                    ///
                    ///
                    #[serde(rename = "showInfo")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub show_info: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/showStyles。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the node styles in the tooltip (default: false).</pre>
                    ///
                    ///
                    #[serde(rename = "showStyles")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub show_styles: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/showRulers。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the rulers should be shown (default: false).</pre>
                    ///
                    ///
                    #[serde(rename = "showRulers")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub show_rulers: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/showAccessibilityInfo。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the a11y info should be shown (default: true).</pre>
                    ///
                    ///
                    #[serde(rename = "showAccessibilityInfo")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub show_accessibility_info: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/showExtensionLines。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the extension lines from node to the rulers should be shown (default: false).</pre>
                    ///
                    ///
                    #[serde(rename = "showExtensionLines")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub show_extension_lines: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/contentColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The content box highlight fill color (default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "contentColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub content_color: Option<crate::dom::types::Rgba>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/paddingColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The padding highlight fill color (default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "paddingColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub padding_color: Option<crate::dom::types::Rgba>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/borderColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The border highlight fill color (default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "borderColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub border_color: Option<crate::dom::types::Rgba>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/marginColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The margin highlight fill color (default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "marginColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub margin_color: Option<crate::dom::types::Rgba>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/eventTargetColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The event target element highlight fill color (default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "eventTargetColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub event_target_color: Option<crate::dom::types::Rgba>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/shapeColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The shape outside fill color (default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "shapeColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub shape_color: Option<crate::dom::types::Rgba>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/shapeMarginColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The shape margin fill color (default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "shapeMarginColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub shape_margin_color: Option<crate::dom::types::Rgba>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/cssGridColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The grid layout color (default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "cssGridColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub css_grid_color: Option<crate::dom::types::Rgba>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/colorFormat。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The color format used to format color styles (default: hex).</pre>
                    ///
                    ///
                    #[serde(rename = "colorFormat")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub color_format: Option<crate::overlay::types::ColorFormat>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/gridHighlightConfig。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The grid layout highlight configuration (default: all transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "gridHighlightConfig")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub grid_highlight_config: Option<crate::overlay::types::GridHighlightConfig>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/flexContainerHighlightConfig。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The flex container highlight configuration (default: all transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "flexContainerHighlightConfig")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub flex_container_highlight_config:
                        Option<crate::overlay::types::FlexContainerHighlightConfig>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/flexItemHighlightConfig。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The flex item highlight configuration (default: all transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "flexItemHighlightConfig")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub flex_item_highlight_config:
                        Option<crate::overlay::types::FlexItemHighlightConfig>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/contrastAlgorithm。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The contrast algorithm to use for the contrast ratio (default: aa).</pre>
                    ///
                    ///
                    #[serde(rename = "contrastAlgorithm")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub contrast_algorithm: Option<crate::overlay::types::ContrastAlgorithm>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HighlightConfig/properties/containerQueryContainerHighlightConfig。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The container query container highlight configuration (default: all transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "containerQueryContainerHighlightConfig")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub container_query_container_highlight_config:
                        Option<crate::overlay::types::ContainerQueryContainerHighlightConfig>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(HighlightConfig {
                    show_info: fields.show_info,
                    show_styles: fields.show_styles,
                    show_rulers: fields.show_rulers,
                    show_accessibility_info: fields.show_accessibility_info,
                    show_extension_lines: fields.show_extension_lines,
                    content_color: fields.content_color,
                    padding_color: fields.padding_color,
                    border_color: fields.border_color,
                    margin_color: fields.margin_color,
                    event_target_color: fields.event_target_color,
                    shape_color: fields.shape_color,
                    shape_margin_color: fields.shape_margin_color,
                    css_grid_color: fields.css_grid_color,
                    color_format: fields.color_format,
                    grid_highlight_config: fields.grid_highlight_config,
                    flex_container_highlight_config: fields.flex_container_highlight_config,
                    flex_item_highlight_config: fields.flex_item_highlight_config,
                    contrast_algorithm: fields.contrast_algorithm,
                    container_query_container_highlight_config: fields
                        .container_query_container_highlight_config,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.type.HingeConfig。
///
///协议原文：
///
///<pre>Configuration for dual screen hinge</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct HingeConfig {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.HingeConfig/properties/rect。
    ///
    ///协议原文：
    ///
    ///<pre>A rectangle represent hinge</pre>
    ///
    ///
    #[serde(rename = "rect")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub rect: crate::dom::types::Rect,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HingeConfig/properties/contentColor。
    ///
    ///协议原文：
    ///
    ///<pre>The content box highlight fill color (default: a dark color).</pre>
    ///
    ///
    #[serde(rename = "contentColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub content_color: Option<crate::dom::types::Rgba>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HingeConfig/properties/outlineColor。
    ///
    ///协议原文：
    ///
    ///<pre>The content box highlight outline color (default: transparent).</pre>
    ///
    ///
    #[serde(rename = "outlineColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub outline_color: Option<crate::dom::types::Rgba>,
}
impl<'de> serde::Deserialize<'de> for HingeConfig {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HingeConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.HingeConfig/properties/rect。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A rectangle represent hinge</pre>
                    ///
                    ///
                    #[serde(rename = "rect")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub rect: crate::dom::types::Rect,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HingeConfig/properties/contentColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The content box highlight fill color (default: a dark color).</pre>
                    ///
                    ///
                    #[serde(rename = "contentColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub content_color: Option<crate::dom::types::Rgba>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.HingeConfig/properties/outlineColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The content box highlight outline color (default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "outlineColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub outline_color: Option<crate::dom::types::Rgba>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(HingeConfig {
                    rect: fields.rect,
                    content_color: fields.content_color,
                    outline_color: fields.outline_color,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Overlay.type.InspectMode。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum InspectMode {
    ///协议值 "searchForNode"。
    SearchForNode,
    ///协议值 "searchForUAShadowDOM"。
    SearchForUaShadowDom,
    ///协议值 "captureAreaScreenshot"。
    CaptureAreaScreenshot,
    ///协议值 "none"。
    None,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl InspectMode {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::overlay::types::InspectMode::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::SearchForNode => "searchForNode",
            Self::SearchForUaShadowDom => "searchForUAShadowDOM",
            Self::CaptureAreaScreenshot => "captureAreaScreenshot",
            Self::None => "none",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for InspectMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for InspectMode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "searchForNode" => InspectMode::SearchForNode,
            "searchForUAShadowDOM" => InspectMode::SearchForUaShadowDom,
            "captureAreaScreenshot" => InspectMode::CaptureAreaScreenshot,
            "none" => InspectMode::None,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for InspectMode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InspectMode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InspectMode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "searchForNode" => InspectMode::SearchForNode,
                    "searchForUAShadowDOM" => InspectMode::SearchForUaShadowDom,
                    "captureAreaScreenshot" => InspectMode::CaptureAreaScreenshot,
                    "none" => InspectMode::None,
                    _ => InspectMode::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(InspectMode::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.type.InspectedElementAnchorConfig。
#[derive(Debug, Clone, serde::Serialize)]
pub struct InspectedElementAnchorConfig {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.InspectedElementAnchorConfig/properties/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the node to highlight.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub node_id: Option<crate::dom::types::NodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.InspectedElementAnchorConfig/properties/backendNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the backend node to highlight.</pre>
    ///
    ///
    #[serde(rename = "backendNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
}
impl<'de> serde::Deserialize<'de> for InspectedElementAnchorConfig {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InspectedElementAnchorConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.InspectedElementAnchorConfig/properties/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the node to highlight.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub node_id: Option<crate::dom::types::NodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.InspectedElementAnchorConfig/properties/backendNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the backend node to highlight.</pre>
                    ///
                    ///
                    #[serde(rename = "backendNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(InspectedElementAnchorConfig {
                    node_id: fields.node_id,
                    backend_node_id: fields.backend_node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.type.IsolatedElementHighlightConfig。
#[derive(Debug, Clone, serde::Serialize)]
pub struct IsolatedElementHighlightConfig {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.IsolatedElementHighlightConfig/properties/isolationModeHighlightConfig。
    ///
    ///协议原文：
    ///
    ///<pre>A descriptor for the highlight appearance of an element in isolation mode.</pre>
    ///
    ///
    #[serde(rename = "isolationModeHighlightConfig")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub isolation_mode_highlight_config: crate::overlay::types::IsolationModeHighlightConfig,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.IsolatedElementHighlightConfig/properties/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the isolated element to highlight.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for IsolatedElementHighlightConfig {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = IsolatedElementHighlightConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.IsolatedElementHighlightConfig/properties/isolationModeHighlightConfig。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A descriptor for the highlight appearance of an element in isolation mode.</pre>
                    ///
                    ///
                    #[serde(rename = "isolationModeHighlightConfig")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub isolation_mode_highlight_config:
                        crate::overlay::types::IsolationModeHighlightConfig,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.IsolatedElementHighlightConfig/properties/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the isolated element to highlight.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(IsolatedElementHighlightConfig {
                    isolation_mode_highlight_config: fields.isolation_mode_highlight_config,
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.type.IsolationModeHighlightConfig。
#[derive(Debug, Clone, serde::Serialize)]
pub struct IsolationModeHighlightConfig {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.IsolationModeHighlightConfig/properties/resizerColor。
    ///
    ///协议原文：
    ///
    ///<pre>The fill color of the resizers (default: transparent).</pre>
    ///
    ///
    #[serde(rename = "resizerColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub resizer_color: Option<crate::dom::types::Rgba>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.IsolationModeHighlightConfig/properties/resizerHandleColor。
    ///
    ///协议原文：
    ///
    ///<pre>The fill color for resizer handles (default: transparent).</pre>
    ///
    ///
    #[serde(rename = "resizerHandleColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub resizer_handle_color: Option<crate::dom::types::Rgba>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.IsolationModeHighlightConfig/properties/maskColor。
    ///
    ///协议原文：
    ///
    ///<pre>The fill color for the mask covering non-isolated elements (default: transparent).</pre>
    ///
    ///
    #[serde(rename = "maskColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub mask_color: Option<crate::dom::types::Rgba>,
}
impl<'de> serde::Deserialize<'de> for IsolationModeHighlightConfig {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = IsolationModeHighlightConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.IsolationModeHighlightConfig/properties/resizerColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The fill color of the resizers (default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "resizerColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub resizer_color: Option<crate::dom::types::Rgba>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.IsolationModeHighlightConfig/properties/resizerHandleColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The fill color for resizer handles (default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "resizerHandleColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub resizer_handle_color: Option<crate::dom::types::Rgba>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.IsolationModeHighlightConfig/properties/maskColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The fill color for the mask covering non-isolated elements (default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "maskColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub mask_color: Option<crate::dom::types::Rgba>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(IsolationModeHighlightConfig {
                    resizer_color: fields.resizer_color,
                    resizer_handle_color: fields.resizer_handle_color,
                    mask_color: fields.mask_color,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Overlay.type.LineStyle/properties/pattern。
///
///协议原文：
///
///<pre>The line pattern (default: solid)</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum LineStylePattern {
    ///协议值 "dashed"。
    Dashed,
    ///协议值 "dotted"。
    Dotted,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl LineStylePattern {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::overlay::types::LineStylePattern::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Dashed => "dashed",
            Self::Dotted => "dotted",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for LineStylePattern {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for LineStylePattern {
    fn from(value: String) -> Self {
        match value.as_str() {
            "dashed" => LineStylePattern::Dashed,
            "dotted" => LineStylePattern::Dotted,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for LineStylePattern {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LineStylePattern {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LineStylePattern;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "dashed" => LineStylePattern::Dashed,
                    "dotted" => LineStylePattern::Dotted,
                    _ => LineStylePattern::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(LineStylePattern::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.type.LineStyle。
///
///协议原文：
///
///<pre>Style information for drawing a line.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct LineStyle {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.LineStyle/properties/color。
    ///
    ///协议原文：
    ///
    ///<pre>The color of the line (default: transparent)</pre>
    ///
    ///
    #[serde(rename = "color")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub color: Option<crate::dom::types::Rgba>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.LineStyle/properties/pattern。
    ///
    ///协议原文：
    ///
    ///<pre>The line pattern (default: solid)</pre>
    ///
    ///
    #[serde(rename = "pattern")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub pattern: Option<LineStylePattern>,
}
impl<'de> serde::Deserialize<'de> for LineStyle {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LineStyle;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.LineStyle/properties/color。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The color of the line (default: transparent)</pre>
                    ///
                    ///
                    #[serde(rename = "color")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub color: Option<crate::dom::types::Rgba>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.LineStyle/properties/pattern。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The line pattern (default: solid)</pre>
                    ///
                    ///
                    #[serde(rename = "pattern")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub pattern: Option<LineStylePattern>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LineStyle {
                    color: fields.color,
                    pattern: fields.pattern,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.type.ScrollSnapContainerHighlightConfig。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ScrollSnapContainerHighlightConfig {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.ScrollSnapContainerHighlightConfig/properties/snapportBorder。
    ///
    ///协议原文：
    ///
    ///<pre>The style of the snapport border (default: transparent)</pre>
    ///
    ///
    #[serde(rename = "snapportBorder")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub snapport_border: Option<crate::overlay::types::LineStyle>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.ScrollSnapContainerHighlightConfig/properties/snapAreaBorder。
    ///
    ///协议原文：
    ///
    ///<pre>The style of the snap area border (default: transparent)</pre>
    ///
    ///
    #[serde(rename = "snapAreaBorder")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub snap_area_border: Option<crate::overlay::types::LineStyle>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.ScrollSnapContainerHighlightConfig/properties/scrollMarginColor。
    ///
    ///协议原文：
    ///
    ///<pre>The margin highlight fill color (default: transparent).</pre>
    ///
    ///
    #[serde(rename = "scrollMarginColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub scroll_margin_color: Option<crate::dom::types::Rgba>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.ScrollSnapContainerHighlightConfig/properties/scrollPaddingColor。
    ///
    ///协议原文：
    ///
    ///<pre>The padding highlight fill color (default: transparent).</pre>
    ///
    ///
    #[serde(rename = "scrollPaddingColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub scroll_padding_color: Option<crate::dom::types::Rgba>,
}
impl<'de> serde::Deserialize<'de> for ScrollSnapContainerHighlightConfig {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScrollSnapContainerHighlightConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.ScrollSnapContainerHighlightConfig/properties/snapportBorder。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The style of the snapport border (default: transparent)</pre>
                    ///
                    ///
                    #[serde(rename = "snapportBorder")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub snapport_border: Option<crate::overlay::types::LineStyle>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.ScrollSnapContainerHighlightConfig/properties/snapAreaBorder。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The style of the snap area border (default: transparent)</pre>
                    ///
                    ///
                    #[serde(rename = "snapAreaBorder")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub snap_area_border: Option<crate::overlay::types::LineStyle>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.ScrollSnapContainerHighlightConfig/properties/scrollMarginColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The margin highlight fill color (default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "scrollMarginColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub scroll_margin_color: Option<crate::dom::types::Rgba>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.type.ScrollSnapContainerHighlightConfig/properties/scrollPaddingColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The padding highlight fill color (default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "scrollPaddingColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub scroll_padding_color: Option<crate::dom::types::Rgba>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ScrollSnapContainerHighlightConfig {
                    snapport_border: fields.snapport_border,
                    snap_area_border: fields.snap_area_border,
                    scroll_margin_color: fields.scroll_margin_color,
                    scroll_padding_color: fields.scroll_padding_color,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.type.ScrollSnapHighlightConfig。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ScrollSnapHighlightConfig {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.ScrollSnapHighlightConfig/properties/scrollSnapContainerHighlightConfig。
    ///
    ///协议原文：
    ///
    ///<pre>A descriptor for the highlight appearance of scroll snap containers.</pre>
    ///
    ///
    #[serde(rename = "scrollSnapContainerHighlightConfig")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub scroll_snap_container_highlight_config:
        crate::overlay::types::ScrollSnapContainerHighlightConfig,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.ScrollSnapHighlightConfig/properties/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the node to highlight.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for ScrollSnapHighlightConfig {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScrollSnapHighlightConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.ScrollSnapHighlightConfig/properties/scrollSnapContainerHighlightConfig。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A descriptor for the highlight appearance of scroll snap containers.</pre>
                    ///
                    ///
                    #[serde(rename = "scrollSnapContainerHighlightConfig")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub scroll_snap_container_highlight_config:
                        crate::overlay::types::ScrollSnapContainerHighlightConfig,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.ScrollSnapHighlightConfig/properties/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the node to highlight.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ScrollSnapHighlightConfig {
                    scroll_snap_container_highlight_config: fields
                        .scroll_snap_container_highlight_config,
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.type.SourceOrderConfig。
///
///协议原文：
///
///<pre>Configuration data for drawing the source order of an elements children.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct SourceOrderConfig {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.SourceOrderConfig/properties/parentOutlineColor。
    ///
    ///协议原文：
    ///
    ///<pre>the color to outline the given element in.</pre>
    ///
    ///
    #[serde(rename = "parentOutlineColor")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub parent_outline_color: crate::dom::types::Rgba,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.SourceOrderConfig/properties/childOutlineColor。
    ///
    ///协议原文：
    ///
    ///<pre>the color to outline the child elements in.</pre>
    ///
    ///
    #[serde(rename = "childOutlineColor")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub child_outline_color: crate::dom::types::Rgba,
}
impl<'de> serde::Deserialize<'de> for SourceOrderConfig {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SourceOrderConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.SourceOrderConfig/properties/parentOutlineColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>the color to outline the given element in.</pre>
                    ///
                    ///
                    #[serde(rename = "parentOutlineColor")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub parent_outline_color: crate::dom::types::Rgba,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.SourceOrderConfig/properties/childOutlineColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>the color to outline the child elements in.</pre>
                    ///
                    ///
                    #[serde(rename = "childOutlineColor")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub child_outline_color: crate::dom::types::Rgba,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SourceOrderConfig {
                    parent_outline_color: fields.parent_outline_color,
                    child_outline_color: fields.child_outline_color,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.type.WindowControlsOverlayConfig。
///
///协议原文：
///
///<pre>Configuration for Window Controls Overlay</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct WindowControlsOverlayConfig {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.WindowControlsOverlayConfig/properties/showCSS。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the title bar CSS should be shown when emulating the Window Controls Overlay.</pre>
    ///
    ///
    #[serde(rename = "showCSS")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub show_css: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.WindowControlsOverlayConfig/properties/selectedPlatform。
    ///
    ///协议原文：
    ///
    ///<pre>Selected platforms to show the overlay.</pre>
    ///
    ///
    #[serde(rename = "selectedPlatform")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub selected_platform: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.WindowControlsOverlayConfig/properties/themeColor。
    ///
    ///协议原文：
    ///
    ///<pre>The theme color defined in app manifest.</pre>
    ///
    ///
    #[serde(rename = "themeColor")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub theme_color: String,
}
impl<'de> serde::Deserialize<'de> for WindowControlsOverlayConfig {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WindowControlsOverlayConfig;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.WindowControlsOverlayConfig/properties/showCSS。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the title bar CSS should be shown when emulating the Window Controls Overlay.</pre>
                    ///
                    ///
                    #[serde(rename = "showCSS")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub show_css: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.WindowControlsOverlayConfig/properties/selectedPlatform。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Selected platforms to show the overlay.</pre>
                    ///
                    ///
                    #[serde(rename = "selectedPlatform")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub selected_platform: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.type.WindowControlsOverlayConfig/properties/themeColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The theme color defined in app manifest.</pre>
                    ///
                    ///
                    #[serde(rename = "themeColor")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub theme_color: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(WindowControlsOverlayConfig {
                    show_css: fields.show_css,
                    selected_platform: fields.selected_platform,
                    theme_color: fields.theme_color,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
