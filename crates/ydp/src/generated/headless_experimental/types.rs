///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：HeadlessExperimental.type.ScreenshotParams/properties/format。
///
///协议原文：
///
///<pre>Image compression format (defaults to png).</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ScreenshotParamsFormat {
    ///协议值 "jpeg"。
    Jpeg,
    ///协议值 "png"。
    Png,
    ///协议值 "webp"。
    Webp,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ScreenshotParamsFormat {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::headless_experimental::types::ScreenshotParamsFormat::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Jpeg => "jpeg",
            Self::Png => "png",
            Self::Webp => "webp",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ScreenshotParamsFormat {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ScreenshotParamsFormat {
    fn from(value: String) -> Self {
        match value.as_str() {
            "jpeg" => ScreenshotParamsFormat::Jpeg,
            "png" => ScreenshotParamsFormat::Png,
            "webp" => ScreenshotParamsFormat::Webp,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ScreenshotParamsFormat {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ScreenshotParamsFormat {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScreenshotParamsFormat;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "jpeg" => ScreenshotParamsFormat::Jpeg,
                    "png" => ScreenshotParamsFormat::Png,
                    "webp" => ScreenshotParamsFormat::Webp,
                    _ => ScreenshotParamsFormat::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ScreenshotParamsFormat::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeadlessExperimental.type.ScreenshotParams。
///
///协议原文：
///
///<pre>Encoding options for a screenshot.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ScreenshotParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeadlessExperimental.type.ScreenshotParams/properties/format。
    ///
    ///协议原文：
    ///
    ///<pre>Image compression format (defaults to png).</pre>
    ///
    ///
    #[serde(rename = "format")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub format: Option<ScreenshotParamsFormat>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeadlessExperimental.type.ScreenshotParams/properties/quality。
    ///
    ///协议原文：
    ///
    ///<pre>Compression quality from range [0..100] (jpeg and webp only).</pre>
    ///
    ///
    #[serde(rename = "quality")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub quality: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeadlessExperimental.type.ScreenshotParams/properties/optimizeForSpeed。
    ///
    ///协议原文：
    ///
    ///<pre>Optimize image encoding for speed, not for resulting size (defaults to false)</pre>
    ///
    ///
    #[serde(rename = "optimizeForSpeed")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub optimize_for_speed: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for ScreenshotParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScreenshotParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeadlessExperimental.type.ScreenshotParams/properties/format。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Image compression format (defaults to png).</pre>
                    ///
                    ///
                    #[serde(rename = "format")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub format: Option<ScreenshotParamsFormat>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeadlessExperimental.type.ScreenshotParams/properties/quality。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Compression quality from range [0..100] (jpeg and webp only).</pre>
                    ///
                    ///
                    #[serde(rename = "quality")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub quality: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeadlessExperimental.type.ScreenshotParams/properties/optimizeForSpeed。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Optimize image encoding for speed, not for resulting size (defaults to false)</pre>
                    ///
                    ///
                    #[serde(rename = "optimizeForSpeed")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub optimize_for_speed: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ScreenshotParams {
                    format: fields.format,
                    quality: fields.quality,
                    optimize_for_speed: fields.optimize_for_speed,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
