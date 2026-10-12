///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Extensions.type.ExtensionInfo。
///
///协议原文：
///
///<pre>Detailed information about an extension.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ExtensionInfo {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.type.ExtensionInfo/properties/id。
    ///
    ///协议原文：
    ///
    ///<pre>Extension id.</pre>
    ///
    ///
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.type.ExtensionInfo/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Extension name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.type.ExtensionInfo/properties/version。
    ///
    ///协议原文：
    ///
    ///<pre>Extension version.</pre>
    ///
    ///
    #[serde(rename = "version")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub version: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.type.ExtensionInfo/properties/path。
    ///
    ///协议原文：
    ///
    ///<pre>The path from which the extension was loaded.</pre>
    ///
    ///
    #[serde(rename = "path")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub path: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.type.ExtensionInfo/properties/enabled。
    ///
    ///协议原文：
    ///
    ///<pre>Extension enabled status.</pre>
    ///
    ///
    #[serde(rename = "enabled")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub enabled: bool,
}
impl<'de> serde::Deserialize<'de> for ExtensionInfo {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExtensionInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.type.ExtensionInfo/properties/id。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Extension id.</pre>
                    ///
                    ///
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.type.ExtensionInfo/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Extension name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.type.ExtensionInfo/properties/version。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Extension version.</pre>
                    ///
                    ///
                    #[serde(rename = "version")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub version: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.type.ExtensionInfo/properties/path。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The path from which the extension was loaded.</pre>
                    ///
                    ///
                    #[serde(rename = "path")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub path: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.type.ExtensionInfo/properties/enabled。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Extension enabled status.</pre>
                    ///
                    ///
                    #[serde(rename = "enabled")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub enabled: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ExtensionInfo {
                    id: fields.id,
                    name: fields.name,
                    version: fields.version,
                    path: fields.path,
                    enabled: fields.enabled,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Extensions.type.StorageArea。
///
///协议原文：
///
///<pre>Storage areas.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StorageArea {
    ///协议值 "session"。
    Session,
    ///协议值 "local"。
    Local,
    ///协议值 "sync"。
    Sync,
    ///协议值 "managed"。
    Managed,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl StorageArea {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::extensions::types::StorageArea::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Session => "session",
            Self::Local => "local",
            Self::Sync => "sync",
            Self::Managed => "managed",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for StorageArea {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for StorageArea {
    fn from(value: String) -> Self {
        match value.as_str() {
            "session" => StorageArea::Session,
            "local" => StorageArea::Local,
            "sync" => StorageArea::Sync,
            "managed" => StorageArea::Managed,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for StorageArea {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StorageArea {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StorageArea;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "session" => StorageArea::Session,
                    "local" => StorageArea::Local,
                    "sync" => StorageArea::Sync,
                    "managed" => StorageArea::Managed,
                    _ => StorageArea::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(StorageArea::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
