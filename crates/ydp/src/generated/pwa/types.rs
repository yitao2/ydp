///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：PWA.type.DisplayMode。
///
///协议原文：
///
///<pre>If user prefers opening the app in browser or an app window.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DisplayMode {
    ///协议值 "standalone"。
    Standalone,
    ///协议值 "browser"。
    Browser,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl DisplayMode {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::pwa::types::DisplayMode::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Standalone => "standalone",
            Self::Browser => "browser",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for DisplayMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for DisplayMode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "standalone" => DisplayMode::Standalone,
            "browser" => DisplayMode::Browser,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for DisplayMode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DisplayMode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DisplayMode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "standalone" => DisplayMode::Standalone,
                    "browser" => DisplayMode::Browser,
                    _ => DisplayMode::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(DisplayMode::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：PWA.type.FileHandler。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FileHandler {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.type.FileHandler/properties/action。
    #[serde(rename = "action")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub action: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.type.FileHandler/properties/accepts。
    #[serde(rename = "accepts")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub accepts: Vec<crate::pwa::types::FileHandlerAccept>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.type.FileHandler/properties/displayName。
    #[serde(rename = "displayName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub display_name: String,
}
impl<'de> serde::Deserialize<'de> for FileHandler {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FileHandler;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.type.FileHandler/properties/action。
                    #[serde(rename = "action")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub action: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.type.FileHandler/properties/accepts。
                    #[serde(rename = "accepts")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub accepts: Vec<crate::pwa::types::FileHandlerAccept>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.type.FileHandler/properties/displayName。
                    #[serde(rename = "displayName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub display_name: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FileHandler {
                    action: fields.action,
                    accepts: fields.accepts,
                    display_name: fields.display_name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：PWA.type.FileHandlerAccept。
///
///协议原文：
///
///<pre>The following types are the replica of
///https://crsrc.org/c/chrome/browser/web_applications/proto/web_app_os_integration_state.proto;drc=9910d3be894c8f142c977ba1023f30a656bc13fc;l=67</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct FileHandlerAccept {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.type.FileHandlerAccept/properties/mediaType。
    ///
    ///协议原文：
    ///
    ///<pre>New name of the mimetype according to
    ///https://www.iana.org/assignments/media-types/media-types.xhtml</pre>
    ///
    ///
    #[serde(rename = "mediaType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub media_type: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.type.FileHandlerAccept/properties/fileExtensions。
    #[serde(rename = "fileExtensions")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub file_extensions: Vec<String>,
}
impl<'de> serde::Deserialize<'de> for FileHandlerAccept {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FileHandlerAccept;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.type.FileHandlerAccept/properties/mediaType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>New name of the mimetype according to
                    ///https://www.iana.org/assignments/media-types/media-types.xhtml</pre>
                    ///
                    ///
                    #[serde(rename = "mediaType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub media_type: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：PWA.type.FileHandlerAccept/properties/fileExtensions。
                    #[serde(rename = "fileExtensions")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub file_extensions: Vec<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FileHandlerAccept {
                    media_type: fields.media_type,
                    file_extensions: fields.file_extensions,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
