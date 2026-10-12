///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.type.Bounds。
///
///协议原文：
///
///<pre>Browser window bounds information</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct Bounds {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.type.Bounds/properties/left。
    ///
    ///协议原文：
    ///
    ///<pre>The offset from the left edge of the screen to the window in pixels.</pre>
    ///
    ///
    #[serde(rename = "left")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub left: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.type.Bounds/properties/top。
    ///
    ///协议原文：
    ///
    ///<pre>The offset from the top edge of the screen to the window in pixels.</pre>
    ///
    ///
    #[serde(rename = "top")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub top: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.type.Bounds/properties/width。
    ///
    ///协议原文：
    ///
    ///<pre>The window width in pixels.</pre>
    ///
    ///
    #[serde(rename = "width")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub width: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.type.Bounds/properties/height。
    ///
    ///协议原文：
    ///
    ///<pre>The window height in pixels.</pre>
    ///
    ///
    #[serde(rename = "height")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub height: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.type.Bounds/properties/windowState。
    ///
    ///协议原文：
    ///
    ///<pre>The window state. Default to normal.</pre>
    ///
    ///
    #[serde(rename = "windowState")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub window_state: Option<crate::browser::types::WindowState>,
}
impl<'de> serde::Deserialize<'de> for Bounds {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Bounds;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.type.Bounds/properties/left。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The offset from the left edge of the screen to the window in pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "left")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub left: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.type.Bounds/properties/top。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The offset from the top edge of the screen to the window in pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "top")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub top: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.type.Bounds/properties/width。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The window width in pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "width")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub width: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.type.Bounds/properties/height。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The window height in pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "height")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub height: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.type.Bounds/properties/windowState。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The window state. Default to normal.</pre>
                    ///
                    ///
                    #[serde(rename = "windowState")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub window_state: Option<crate::browser::types::WindowState>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Bounds {
                    left: fields.left,
                    top: fields.top,
                    width: fields.width,
                    height: fields.height,
                    window_state: fields.window_state,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Browser.type.BrowserCommandId。
///
///协议原文：
///
///<pre>Browser command ids used by executeBrowserCommand.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum BrowserCommandId {
    ///协议值 "openTabSearch"。
    OpenTabSearch,
    ///协议值 "closeTabSearch"。
    CloseTabSearch,
    ///协议值 "openGlic"。
    OpenGlic,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl BrowserCommandId {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::browser::types::BrowserCommandId::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::OpenTabSearch => "openTabSearch",
            Self::CloseTabSearch => "closeTabSearch",
            Self::OpenGlic => "openGlic",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for BrowserCommandId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for BrowserCommandId {
    fn from(value: String) -> Self {
        match value.as_str() {
            "openTabSearch" => BrowserCommandId::OpenTabSearch,
            "closeTabSearch" => BrowserCommandId::CloseTabSearch,
            "openGlic" => BrowserCommandId::OpenGlic,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for BrowserCommandId {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BrowserCommandId {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BrowserCommandId;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "openTabSearch" => BrowserCommandId::OpenTabSearch,
                    "closeTabSearch" => BrowserCommandId::CloseTabSearch,
                    "openGlic" => BrowserCommandId::OpenGlic,
                    _ => BrowserCommandId::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(BrowserCommandId::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Browser.type.BrowserContextID。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct BrowserContextId(String);
impl BrowserContextId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::browser::types::BrowserContextId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::browser::types::BrowserContextId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::browser::types::BrowserContextId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::browser::types::BrowserContextId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::browser::types::BrowserContextId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.type.Bucket。
///
///协议原文：
///
///<pre>Chrome histogram bucket.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct Bucket {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.type.Bucket/properties/low。
    ///
    ///协议原文：
    ///
    ///<pre>Minimum value (inclusive).</pre>
    ///
    ///
    #[serde(rename = "low")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub low: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.type.Bucket/properties/high。
    ///
    ///协议原文：
    ///
    ///<pre>Maximum value (exclusive).</pre>
    ///
    ///
    #[serde(rename = "high")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub high: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.type.Bucket/properties/count。
    ///
    ///协议原文：
    ///
    ///<pre>Number of samples.</pre>
    ///
    ///
    #[serde(rename = "count")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub count: i64,
}
impl<'de> serde::Deserialize<'de> for Bucket {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Bucket;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.type.Bucket/properties/low。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Minimum value (inclusive).</pre>
                    ///
                    ///
                    #[serde(rename = "low")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub low: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.type.Bucket/properties/high。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Maximum value (exclusive).</pre>
                    ///
                    ///
                    #[serde(rename = "high")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub high: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.type.Bucket/properties/count。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Number of samples.</pre>
                    ///
                    ///
                    #[serde(rename = "count")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub count: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Bucket {
                    low: fields.low,
                    high: fields.high,
                    count: fields.count,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.type.Histogram。
///
///协议原文：
///
///<pre>Chrome histogram.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct Histogram {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.type.Histogram/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.type.Histogram/properties/sum。
    ///
    ///协议原文：
    ///
    ///<pre>Sum of sample values.</pre>
    ///
    ///
    #[serde(rename = "sum")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub sum: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.type.Histogram/properties/count。
    ///
    ///协议原文：
    ///
    ///<pre>Total number of samples.</pre>
    ///
    ///
    #[serde(rename = "count")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub count: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.type.Histogram/properties/buckets。
    ///
    ///协议原文：
    ///
    ///<pre>Buckets.</pre>
    ///
    ///
    #[serde(rename = "buckets")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub buckets: Vec<crate::browser::types::Bucket>,
}
impl<'de> serde::Deserialize<'de> for Histogram {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Histogram;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.type.Histogram/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.type.Histogram/properties/sum。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Sum of sample values.</pre>
                    ///
                    ///
                    #[serde(rename = "sum")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub sum: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.type.Histogram/properties/count。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Total number of samples.</pre>
                    ///
                    ///
                    #[serde(rename = "count")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub count: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.type.Histogram/properties/buckets。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Buckets.</pre>
                    ///
                    ///
                    #[serde(rename = "buckets")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub buckets: Vec<crate::browser::types::Bucket>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Histogram {
                    name: fields.name,
                    sum: fields.sum,
                    count: fields.count,
                    buckets: fields.buckets,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.type.PermissionDescriptor。
///
///协议原文：
///
///<pre>Definition of PermissionDescriptor defined in the Permissions API:
///https://w3c.github.io/permissions/#dom-permissiondescriptor.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PermissionDescriptor {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.type.PermissionDescriptor/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Name of permission.
    ///See https://cs.chromium.org/chromium/src/third_party/blink/renderer/modules/permissions/permission_descriptor.idl for valid permission names.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.type.PermissionDescriptor/properties/sysex。
    ///
    ///协议原文：
    ///
    ///<pre>For "midi" permission, may also specify sysex control.</pre>
    ///
    ///
    #[serde(rename = "sysex")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub sysex: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.type.PermissionDescriptor/properties/userVisibleOnly。
    ///
    ///协议原文：
    ///
    ///<pre>For "push" permission, may specify userVisibleOnly.
    ///Note that userVisibleOnly = true is the only currently supported type.</pre>
    ///
    ///
    #[serde(rename = "userVisibleOnly")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_visible_only: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.type.PermissionDescriptor/properties/allowWithoutSanitization。
    ///
    ///协议原文：
    ///
    ///<pre>For "clipboard" permission, may specify allowWithoutSanitization.</pre>
    ///
    ///
    #[serde(rename = "allowWithoutSanitization")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_without_sanitization: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.type.PermissionDescriptor/properties/allowWithoutGesture。
    ///
    ///协议原文：
    ///
    ///<pre>For "fullscreen" permission, must specify allowWithoutGesture:true.</pre>
    ///
    ///
    #[serde(rename = "allowWithoutGesture")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_without_gesture: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.type.PermissionDescriptor/properties/panTiltZoom。
    ///
    ///协议原文：
    ///
    ///<pre>For "camera" permission, may specify panTiltZoom.</pre>
    ///
    ///
    #[serde(rename = "panTiltZoom")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub pan_tilt_zoom: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for PermissionDescriptor {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PermissionDescriptor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.type.PermissionDescriptor/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Name of permission.
                    ///See https://cs.chromium.org/chromium/src/third_party/blink/renderer/modules/permissions/permission_descriptor.idl for valid permission names.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.type.PermissionDescriptor/properties/sysex。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>For "midi" permission, may also specify sysex control.</pre>
                    ///
                    ///
                    #[serde(rename = "sysex")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub sysex: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.type.PermissionDescriptor/properties/userVisibleOnly。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>For "push" permission, may specify userVisibleOnly.
                    ///Note that userVisibleOnly = true is the only currently supported type.</pre>
                    ///
                    ///
                    #[serde(rename = "userVisibleOnly")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub user_visible_only: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.type.PermissionDescriptor/properties/allowWithoutSanitization。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>For "clipboard" permission, may specify allowWithoutSanitization.</pre>
                    ///
                    ///
                    #[serde(rename = "allowWithoutSanitization")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub allow_without_sanitization: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.type.PermissionDescriptor/properties/allowWithoutGesture。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>For "fullscreen" permission, must specify allowWithoutGesture:true.</pre>
                    ///
                    ///
                    #[serde(rename = "allowWithoutGesture")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub allow_without_gesture: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.type.PermissionDescriptor/properties/panTiltZoom。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>For "camera" permission, may specify panTiltZoom.</pre>
                    ///
                    ///
                    #[serde(rename = "panTiltZoom")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub pan_tilt_zoom: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PermissionDescriptor {
                    name: fields.name,
                    sysex: fields.sysex,
                    user_visible_only: fields.user_visible_only,
                    allow_without_sanitization: fields.allow_without_sanitization,
                    allow_without_gesture: fields.allow_without_gesture,
                    pan_tilt_zoom: fields.pan_tilt_zoom,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Browser.type.PermissionSetting。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PermissionSetting {
    ///协议值 "granted"。
    Granted,
    ///协议值 "denied"。
    Denied,
    ///协议值 "prompt"。
    Prompt,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl PermissionSetting {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::browser::types::PermissionSetting::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Granted => "granted",
            Self::Denied => "denied",
            Self::Prompt => "prompt",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for PermissionSetting {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for PermissionSetting {
    fn from(value: String) -> Self {
        match value.as_str() {
            "granted" => PermissionSetting::Granted,
            "denied" => PermissionSetting::Denied,
            "prompt" => PermissionSetting::Prompt,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for PermissionSetting {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PermissionSetting {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PermissionSetting;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "granted" => PermissionSetting::Granted,
                    "denied" => PermissionSetting::Denied,
                    "prompt" => PermissionSetting::Prompt,
                    _ => PermissionSetting::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(PermissionSetting::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Browser.type.PermissionType。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PermissionType {
    ///协议值 "ar"。
    Ar,
    ///协议值 "audioCapture"。
    AudioCapture,
    ///协议值 "automaticFullscreen"。
    AutomaticFullscreen,
    ///协议值 "backgroundFetch"。
    BackgroundFetch,
    ///协议值 "backgroundSync"。
    BackgroundSync,
    ///协议值 "cameraPanTiltZoom"。
    CameraPanTiltZoom,
    ///协议值 "capturedSurfaceControl"。
    CapturedSurfaceControl,
    ///协议值 "clipboardReadWrite"。
    ClipboardReadWrite,
    ///协议值 "clipboardSanitizedWrite"。
    ClipboardSanitizedWrite,
    ///协议值 "displayCapture"。
    DisplayCapture,
    ///协议值 "durableStorage"。
    DurableStorage,
    ///协议值 "geolocation"。
    Geolocation,
    ///协议值 "handTracking"。
    HandTracking,
    ///协议值 "idleDetection"。
    IdleDetection,
    ///协议值 "keyboardLock"。
    KeyboardLock,
    ///协议值 "localFonts"。
    LocalFonts,
    ///协议值 "localNetwork"。
    LocalNetwork,
    ///协议值 "localNetworkAccess"。
    LocalNetworkAccess,
    ///协议值 "loopbackNetwork"。
    LoopbackNetwork,
    ///协议值 "midi"。
    Midi,
    ///协议值 "midiSysex"。
    MidiSysex,
    ///协议值 "nfc"。
    Nfc,
    ///协议值 "notifications"。
    Notifications,
    ///协议值 "paymentHandler"。
    PaymentHandler,
    ///协议值 "periodicBackgroundSync"。
    PeriodicBackgroundSync,
    ///协议值 "pointerLock"。
    PointerLock,
    ///协议值 "protectedMediaIdentifier"。
    ProtectedMediaIdentifier,
    ///协议值 "sensors"。
    Sensors,
    ///协议值 "smartCard"。
    SmartCard,
    ///协议值 "speakerSelection"。
    SpeakerSelection,
    ///协议值 "storageAccess"。
    StorageAccess,
    ///协议值 "topLevelStorageAccess"。
    TopLevelStorageAccess,
    ///协议值 "videoCapture"。
    VideoCapture,
    ///协议值 "vr"。
    Vr,
    ///协议值 "wakeLockScreen"。
    WakeLockScreen,
    ///协议值 "wakeLockSystem"。
    WakeLockSystem,
    ///协议值 "webAppInstallation"。
    WebAppInstallation,
    ///协议值 "webPrinting"。
    WebPrinting,
    ///协议值 "windowManagement"。
    WindowManagement,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl PermissionType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::browser::types::PermissionType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ar => "ar",
            Self::AudioCapture => "audioCapture",
            Self::AutomaticFullscreen => "automaticFullscreen",
            Self::BackgroundFetch => "backgroundFetch",
            Self::BackgroundSync => "backgroundSync",
            Self::CameraPanTiltZoom => "cameraPanTiltZoom",
            Self::CapturedSurfaceControl => "capturedSurfaceControl",
            Self::ClipboardReadWrite => "clipboardReadWrite",
            Self::ClipboardSanitizedWrite => "clipboardSanitizedWrite",
            Self::DisplayCapture => "displayCapture",
            Self::DurableStorage => "durableStorage",
            Self::Geolocation => "geolocation",
            Self::HandTracking => "handTracking",
            Self::IdleDetection => "idleDetection",
            Self::KeyboardLock => "keyboardLock",
            Self::LocalFonts => "localFonts",
            Self::LocalNetwork => "localNetwork",
            Self::LocalNetworkAccess => "localNetworkAccess",
            Self::LoopbackNetwork => "loopbackNetwork",
            Self::Midi => "midi",
            Self::MidiSysex => "midiSysex",
            Self::Nfc => "nfc",
            Self::Notifications => "notifications",
            Self::PaymentHandler => "paymentHandler",
            Self::PeriodicBackgroundSync => "periodicBackgroundSync",
            Self::PointerLock => "pointerLock",
            Self::ProtectedMediaIdentifier => "protectedMediaIdentifier",
            Self::Sensors => "sensors",
            Self::SmartCard => "smartCard",
            Self::SpeakerSelection => "speakerSelection",
            Self::StorageAccess => "storageAccess",
            Self::TopLevelStorageAccess => "topLevelStorageAccess",
            Self::VideoCapture => "videoCapture",
            Self::Vr => "vr",
            Self::WakeLockScreen => "wakeLockScreen",
            Self::WakeLockSystem => "wakeLockSystem",
            Self::WebAppInstallation => "webAppInstallation",
            Self::WebPrinting => "webPrinting",
            Self::WindowManagement => "windowManagement",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for PermissionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for PermissionType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ar" => PermissionType::Ar,
            "audioCapture" => PermissionType::AudioCapture,
            "automaticFullscreen" => PermissionType::AutomaticFullscreen,
            "backgroundFetch" => PermissionType::BackgroundFetch,
            "backgroundSync" => PermissionType::BackgroundSync,
            "cameraPanTiltZoom" => PermissionType::CameraPanTiltZoom,
            "capturedSurfaceControl" => PermissionType::CapturedSurfaceControl,
            "clipboardReadWrite" => PermissionType::ClipboardReadWrite,
            "clipboardSanitizedWrite" => PermissionType::ClipboardSanitizedWrite,
            "displayCapture" => PermissionType::DisplayCapture,
            "durableStorage" => PermissionType::DurableStorage,
            "geolocation" => PermissionType::Geolocation,
            "handTracking" => PermissionType::HandTracking,
            "idleDetection" => PermissionType::IdleDetection,
            "keyboardLock" => PermissionType::KeyboardLock,
            "localFonts" => PermissionType::LocalFonts,
            "localNetwork" => PermissionType::LocalNetwork,
            "localNetworkAccess" => PermissionType::LocalNetworkAccess,
            "loopbackNetwork" => PermissionType::LoopbackNetwork,
            "midi" => PermissionType::Midi,
            "midiSysex" => PermissionType::MidiSysex,
            "nfc" => PermissionType::Nfc,
            "notifications" => PermissionType::Notifications,
            "paymentHandler" => PermissionType::PaymentHandler,
            "periodicBackgroundSync" => PermissionType::PeriodicBackgroundSync,
            "pointerLock" => PermissionType::PointerLock,
            "protectedMediaIdentifier" => PermissionType::ProtectedMediaIdentifier,
            "sensors" => PermissionType::Sensors,
            "smartCard" => PermissionType::SmartCard,
            "speakerSelection" => PermissionType::SpeakerSelection,
            "storageAccess" => PermissionType::StorageAccess,
            "topLevelStorageAccess" => PermissionType::TopLevelStorageAccess,
            "videoCapture" => PermissionType::VideoCapture,
            "vr" => PermissionType::Vr,
            "wakeLockScreen" => PermissionType::WakeLockScreen,
            "wakeLockSystem" => PermissionType::WakeLockSystem,
            "webAppInstallation" => PermissionType::WebAppInstallation,
            "webPrinting" => PermissionType::WebPrinting,
            "windowManagement" => PermissionType::WindowManagement,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for PermissionType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PermissionType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PermissionType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "ar" => PermissionType::Ar,
                    "audioCapture" => PermissionType::AudioCapture,
                    "automaticFullscreen" => PermissionType::AutomaticFullscreen,
                    "backgroundFetch" => PermissionType::BackgroundFetch,
                    "backgroundSync" => PermissionType::BackgroundSync,
                    "cameraPanTiltZoom" => PermissionType::CameraPanTiltZoom,
                    "capturedSurfaceControl" => PermissionType::CapturedSurfaceControl,
                    "clipboardReadWrite" => PermissionType::ClipboardReadWrite,
                    "clipboardSanitizedWrite" => PermissionType::ClipboardSanitizedWrite,
                    "displayCapture" => PermissionType::DisplayCapture,
                    "durableStorage" => PermissionType::DurableStorage,
                    "geolocation" => PermissionType::Geolocation,
                    "handTracking" => PermissionType::HandTracking,
                    "idleDetection" => PermissionType::IdleDetection,
                    "keyboardLock" => PermissionType::KeyboardLock,
                    "localFonts" => PermissionType::LocalFonts,
                    "localNetwork" => PermissionType::LocalNetwork,
                    "localNetworkAccess" => PermissionType::LocalNetworkAccess,
                    "loopbackNetwork" => PermissionType::LoopbackNetwork,
                    "midi" => PermissionType::Midi,
                    "midiSysex" => PermissionType::MidiSysex,
                    "nfc" => PermissionType::Nfc,
                    "notifications" => PermissionType::Notifications,
                    "paymentHandler" => PermissionType::PaymentHandler,
                    "periodicBackgroundSync" => PermissionType::PeriodicBackgroundSync,
                    "pointerLock" => PermissionType::PointerLock,
                    "protectedMediaIdentifier" => PermissionType::ProtectedMediaIdentifier,
                    "sensors" => PermissionType::Sensors,
                    "smartCard" => PermissionType::SmartCard,
                    "speakerSelection" => PermissionType::SpeakerSelection,
                    "storageAccess" => PermissionType::StorageAccess,
                    "topLevelStorageAccess" => PermissionType::TopLevelStorageAccess,
                    "videoCapture" => PermissionType::VideoCapture,
                    "vr" => PermissionType::Vr,
                    "wakeLockScreen" => PermissionType::WakeLockScreen,
                    "wakeLockSystem" => PermissionType::WakeLockSystem,
                    "webAppInstallation" => PermissionType::WebAppInstallation,
                    "webPrinting" => PermissionType::WebPrinting,
                    "windowManagement" => PermissionType::WindowManagement,
                    _ => PermissionType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(PermissionType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Browser.type.WindowID。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct WindowId(i64);
impl WindowId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::browser::types::WindowId = serde_json::from_str("0").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::browser::types::WindowId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("0").unwrap());
    ///```
    pub fn new(value: i64) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::browser::types::WindowId = serde_json::from_str("0").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::browser::types::WindowId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("0").unwrap());
    ///```
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Browser.type.WindowState。
///
///协议原文：
///
///<pre>The state of the browser window.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum WindowState {
    ///协议值 "normal"。
    Normal,
    ///协议值 "minimized"。
    Minimized,
    ///协议值 "maximized"。
    Maximized,
    ///协议值 "fullscreen"。
    Fullscreen,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl WindowState {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::browser::types::WindowState::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Normal => "normal",
            Self::Minimized => "minimized",
            Self::Maximized => "maximized",
            Self::Fullscreen => "fullscreen",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for WindowState {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for WindowState {
    fn from(value: String) -> Self {
        match value.as_str() {
            "normal" => WindowState::Normal,
            "minimized" => WindowState::Minimized,
            "maximized" => WindowState::Maximized,
            "fullscreen" => WindowState::Fullscreen,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for WindowState {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for WindowState {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WindowState;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "normal" => WindowState::Normal,
                    "minimized" => WindowState::Minimized,
                    "maximized" => WindowState::Maximized,
                    "fullscreen" => WindowState::Fullscreen,
                    _ => WindowState::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(WindowState::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
