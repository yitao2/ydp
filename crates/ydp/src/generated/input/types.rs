///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.type.DragData。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DragData {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.type.DragData/properties/items。
    #[serde(rename = "items")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub items: Vec<crate::input::types::DragDataItem>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.type.DragData/properties/files。
    ///
    ///协议原文：
    ///
    ///<pre>List of filenames that should be included when dropping</pre>
    ///
    ///
    #[serde(rename = "files")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub files: Option<Vec<String>>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.type.DragData/properties/dragOperationsMask。
    ///
    ///协议原文：
    ///
    ///<pre>Bit field representing allowed drag operations. Copy = 1, Link = 2, Move = 16</pre>
    ///
    ///
    #[serde(rename = "dragOperationsMask")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub drag_operations_mask: i64,
}
impl<'de> serde::Deserialize<'de> for DragData {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DragData;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.type.DragData/properties/items。
                    #[serde(rename = "items")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub items: Vec<crate::input::types::DragDataItem>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.type.DragData/properties/files。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>List of filenames that should be included when dropping</pre>
                    ///
                    ///
                    #[serde(rename = "files")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub files: Option<Vec<String>>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.type.DragData/properties/dragOperationsMask。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Bit field representing allowed drag operations. Copy = 1, Link = 2, Move = 16</pre>
                    ///
                    ///
                    #[serde(rename = "dragOperationsMask")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub drag_operations_mask: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DragData {
                    items: fields.items,
                    files: fields.files,
                    drag_operations_mask: fields.drag_operations_mask,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.type.DragDataItem。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DragDataItem {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.type.DragDataItem/properties/mimeType。
    ///
    ///协议原文：
    ///
    ///<pre>Mime type of the dragged data.</pre>
    ///
    ///
    #[serde(rename = "mimeType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub mime_type: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.type.DragDataItem/properties/data。
    ///
    ///协议原文：
    ///
    ///<pre>Depending of the value of `mimeType`, it contains the dragged link,
    ///text, HTML markup or any other data.</pre>
    ///
    ///
    #[serde(rename = "data")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub data: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.type.DragDataItem/properties/title。
    ///
    ///协议原文：
    ///
    ///<pre>Title associated with a link. Only valid when `mimeType` == "text/uri-list".</pre>
    ///
    ///
    #[serde(rename = "title")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.type.DragDataItem/properties/baseURL。
    ///
    ///协议原文：
    ///
    ///<pre>Stores the base URL for the contained markup. Only valid when `mimeType`
    ///== "text/html".</pre>
    ///
    ///
    #[serde(rename = "baseURL")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub base_url: Option<String>,
}
impl<'de> serde::Deserialize<'de> for DragDataItem {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DragDataItem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.type.DragDataItem/properties/mimeType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Mime type of the dragged data.</pre>
                    ///
                    ///
                    #[serde(rename = "mimeType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub mime_type: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.type.DragDataItem/properties/data。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Depending of the value of `mimeType`, it contains the dragged link,
                    ///text, HTML markup or any other data.</pre>
                    ///
                    ///
                    #[serde(rename = "data")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub data: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.type.DragDataItem/properties/title。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Title associated with a link. Only valid when `mimeType` == "text/uri-list".</pre>
                    ///
                    ///
                    #[serde(rename = "title")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub title: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.type.DragDataItem/properties/baseURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Stores the base URL for the contained markup. Only valid when `mimeType`
                    ///== "text/html".</pre>
                    ///
                    ///
                    #[serde(rename = "baseURL")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub base_url: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DragDataItem {
                    mime_type: fields.mime_type,
                    data: fields.data,
                    title: fields.title,
                    base_url: fields.base_url,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Input.type.GestureSourceType。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum GestureSourceType {
    ///协议值 "default"。
    Default,
    ///协议值 "touch"。
    Touch,
    ///协议值 "mouse"。
    Mouse,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl GestureSourceType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::input::types::GestureSourceType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Default => "default",
            Self::Touch => "touch",
            Self::Mouse => "mouse",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for GestureSourceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for GestureSourceType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "default" => GestureSourceType::Default,
            "touch" => GestureSourceType::Touch,
            "mouse" => GestureSourceType::Mouse,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for GestureSourceType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GestureSourceType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GestureSourceType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "default" => GestureSourceType::Default,
                    "touch" => GestureSourceType::Touch,
                    "mouse" => GestureSourceType::Mouse,
                    _ => GestureSourceType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(GestureSourceType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Input.type.MouseButton。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MouseButton {
    ///协议值 "none"。
    None,
    ///协议值 "left"。
    Left,
    ///协议值 "middle"。
    Middle,
    ///协议值 "right"。
    Right,
    ///协议值 "back"。
    Back,
    ///协议值 "forward"。
    Forward,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl MouseButton {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::input::types::MouseButton::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::None => "none",
            Self::Left => "left",
            Self::Middle => "middle",
            Self::Right => "right",
            Self::Back => "back",
            Self::Forward => "forward",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for MouseButton {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for MouseButton {
    fn from(value: String) -> Self {
        match value.as_str() {
            "none" => MouseButton::None,
            "left" => MouseButton::Left,
            "middle" => MouseButton::Middle,
            "right" => MouseButton::Right,
            "back" => MouseButton::Back,
            "forward" => MouseButton::Forward,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for MouseButton {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MouseButton {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MouseButton;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "none" => MouseButton::None,
                    "left" => MouseButton::Left,
                    "middle" => MouseButton::Middle,
                    "right" => MouseButton::Right,
                    "back" => MouseButton::Back,
                    "forward" => MouseButton::Forward,
                    _ => MouseButton::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(MouseButton::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Input.type.TimeSinceEpoch。
///
///协议原文：
///
///<pre>UTC time in seconds, counted from January 1, 1970.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct TimeSinceEpoch(ydp_core::FiniteF64);
impl TimeSinceEpoch {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::input::types::TimeSinceEpoch = serde_json::from_str("0.0").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::input::types::TimeSinceEpoch::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("0.0").unwrap());
    ///```
    pub fn new(value: ydp_core::FiniteF64) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::input::types::TimeSinceEpoch = serde_json::from_str("0.0").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::input::types::TimeSinceEpoch::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("0.0").unwrap());
    ///```
    pub fn into_inner(self) -> ydp_core::FiniteF64 {
        self.0
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.type.TouchPoint。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TouchPoint {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.type.TouchPoint/properties/x。
    ///
    ///协议原文：
    ///
    ///<pre>X coordinate of the event relative to the main frame's viewport in CSS pixels.</pre>
    ///
    ///
    #[serde(rename = "x")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub x: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.type.TouchPoint/properties/y。
    ///
    ///协议原文：
    ///
    ///<pre>Y coordinate of the event relative to the main frame's viewport in CSS pixels. 0 refers to
    ///the top of the viewport and Y increases as it proceeds towards the bottom of the viewport.</pre>
    ///
    ///
    #[serde(rename = "y")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub y: ydp_core::FiniteF64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.type.TouchPoint/properties/radiusX。
    ///
    ///协议原文：
    ///
    ///<pre>X radius of the touch area (default: 1.0).</pre>
    ///
    ///
    #[serde(rename = "radiusX")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub radius_x: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.type.TouchPoint/properties/radiusY。
    ///
    ///协议原文：
    ///
    ///<pre>Y radius of the touch area (default: 1.0).</pre>
    ///
    ///
    #[serde(rename = "radiusY")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub radius_y: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.type.TouchPoint/properties/rotationAngle。
    ///
    ///协议原文：
    ///
    ///<pre>Rotation angle (default: 0.0).</pre>
    ///
    ///
    #[serde(rename = "rotationAngle")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub rotation_angle: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.type.TouchPoint/properties/force。
    ///
    ///协议原文：
    ///
    ///<pre>Force (default: 1.0).</pre>
    ///
    ///
    #[serde(rename = "force")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub force: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.type.TouchPoint/properties/tangentialPressure。
    ///
    ///协议原文：
    ///
    ///<pre>The normalized tangential pressure, which has a range of [-1,1] (default: 0).</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "tangentialPressure")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub tangential_pressure: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.type.TouchPoint/properties/tiltX。
    ///
    ///协议原文：
    ///
    ///<pre>The plane angle between the Y-Z plane and the plane containing both the stylus axis and the Y axis, in degrees of the range [-90,90], a positive tiltX is to the right (default: 0)</pre>
    ///
    ///
    #[serde(rename = "tiltX")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub tilt_x: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.type.TouchPoint/properties/tiltY。
    ///
    ///协议原文：
    ///
    ///<pre>The plane angle between the X-Z plane and the plane containing both the stylus axis and the X axis, in degrees of the range [-90,90], a positive tiltY is towards the user (default: 0).</pre>
    ///
    ///
    #[serde(rename = "tiltY")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub tilt_y: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.type.TouchPoint/properties/twist。
    ///
    ///协议原文：
    ///
    ///<pre>The clockwise rotation of a pen stylus around its own major axis, in degrees in the range [0,359] (default: 0).</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "twist")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub twist: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.type.TouchPoint/properties/id。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier used to track touch sources between events, must be unique within an event.</pre>
    ///
    ///
    #[serde(rename = "id")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<ydp_core::FiniteF64>,
}
impl<'de> serde::Deserialize<'de> for TouchPoint {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TouchPoint;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.type.TouchPoint/properties/x。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>X coordinate of the event relative to the main frame's viewport in CSS pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "x")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub x: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.type.TouchPoint/properties/y。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Y coordinate of the event relative to the main frame's viewport in CSS pixels. 0 refers to
                    ///the top of the viewport and Y increases as it proceeds towards the bottom of the viewport.</pre>
                    ///
                    ///
                    #[serde(rename = "y")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub y: ydp_core::FiniteF64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.type.TouchPoint/properties/radiusX。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>X radius of the touch area (default: 1.0).</pre>
                    ///
                    ///
                    #[serde(rename = "radiusX")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub radius_x: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.type.TouchPoint/properties/radiusY。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Y radius of the touch area (default: 1.0).</pre>
                    ///
                    ///
                    #[serde(rename = "radiusY")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub radius_y: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.type.TouchPoint/properties/rotationAngle。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Rotation angle (default: 0.0).</pre>
                    ///
                    ///
                    #[serde(rename = "rotationAngle")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub rotation_angle: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.type.TouchPoint/properties/force。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Force (default: 1.0).</pre>
                    ///
                    ///
                    #[serde(rename = "force")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub force: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.type.TouchPoint/properties/tangentialPressure。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The normalized tangential pressure, which has a range of [-1,1] (default: 0).</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "tangentialPressure")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub tangential_pressure: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.type.TouchPoint/properties/tiltX。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The plane angle between the Y-Z plane and the plane containing both the stylus axis and the Y axis, in degrees of the range [-90,90], a positive tiltX is to the right (default: 0)</pre>
                    ///
                    ///
                    #[serde(rename = "tiltX")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub tilt_x: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.type.TouchPoint/properties/tiltY。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The plane angle between the X-Z plane and the plane containing both the stylus axis and the X axis, in degrees of the range [-90,90], a positive tiltY is towards the user (default: 0).</pre>
                    ///
                    ///
                    #[serde(rename = "tiltY")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub tilt_y: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.type.TouchPoint/properties/twist。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The clockwise rotation of a pen stylus around its own major axis, in degrees in the range [0,359] (default: 0).</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "twist")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub twist: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.type.TouchPoint/properties/id。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier used to track touch sources between events, must be unique within an event.</pre>
                    ///
                    ///
                    #[serde(rename = "id")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub id: Option<ydp_core::FiniteF64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TouchPoint {
                    x: fields.x,
                    y: fields.y,
                    radius_x: fields.radius_x,
                    radius_y: fields.radius_y,
                    rotation_angle: fields.rotation_angle,
                    force: fields.force,
                    tangential_pressure: fields.tangential_pressure,
                    tilt_x: fields.tilt_x,
                    tilt_y: fields.tilt_y,
                    twist: fields.twist,
                    id: fields.id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
