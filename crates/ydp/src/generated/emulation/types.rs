///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Emulation.type.DevicePosture/properties/type。
///
///协议原文：
///
///<pre>Current posture of the device</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DevicePostureType {
    ///协议值 "continuous"。
    Continuous,
    ///协议值 "folded"。
    Folded,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl DevicePostureType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::emulation::types::DevicePostureType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Continuous => "continuous",
            Self::Folded => "folded",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for DevicePostureType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for DevicePostureType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "continuous" => DevicePostureType::Continuous,
            "folded" => DevicePostureType::Folded,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for DevicePostureType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DevicePostureType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DevicePostureType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "continuous" => DevicePostureType::Continuous,
                    "folded" => DevicePostureType::Folded,
                    _ => DevicePostureType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(DevicePostureType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.type.DevicePosture。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DevicePosture {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.DevicePosture/properties/type。
    ///
    ///协议原文：
    ///
    ///<pre>Current posture of the device</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: DevicePostureType,
}
impl<'de> serde::Deserialize<'de> for DevicePosture {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DevicePosture;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.DevicePosture/properties/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Current posture of the device</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: DevicePostureType,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DevicePosture {
                    r#type: fields.r#type,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Emulation.type.DisabledImageType。
///
///协议原文：
///
///<pre>Enum of image types that can be disabled.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DisabledImageType {
    ///协议值 "avif"。
    Avif,
    ///协议值 "jxl"。
    Jxl,
    ///协议值 "webp"。
    Webp,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl DisabledImageType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::emulation::types::DisabledImageType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Avif => "avif",
            Self::Jxl => "jxl",
            Self::Webp => "webp",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for DisabledImageType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for DisabledImageType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "avif" => DisabledImageType::Avif,
            "jxl" => DisabledImageType::Jxl,
            "webp" => DisabledImageType::Webp,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for DisabledImageType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DisabledImageType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DisabledImageType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "avif" => DisabledImageType::Avif,
                    "jxl" => DisabledImageType::Jxl,
                    "webp" => DisabledImageType::Webp,
                    _ => DisabledImageType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(DisabledImageType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Emulation.type.DisplayFeature/properties/orientation。
///
///协议原文：
///
///<pre>Orientation of a display feature in relation to screen</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DisplayFeatureOrientation {
    ///协议值 "vertical"。
    Vertical,
    ///协议值 "horizontal"。
    Horizontal,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl DisplayFeatureOrientation {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::emulation::types::DisplayFeatureOrientation::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for DisplayFeatureOrientation {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for DisplayFeatureOrientation {
    fn from(value: String) -> Self {
        match value.as_str() {
            "vertical" => DisplayFeatureOrientation::Vertical,
            "horizontal" => DisplayFeatureOrientation::Horizontal,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for DisplayFeatureOrientation {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DisplayFeatureOrientation {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DisplayFeatureOrientation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "vertical" => DisplayFeatureOrientation::Vertical,
                    "horizontal" => DisplayFeatureOrientation::Horizontal,
                    _ => DisplayFeatureOrientation::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(DisplayFeatureOrientation::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.type.DisplayFeature。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DisplayFeature {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.DisplayFeature/properties/orientation。
    ///
    ///协议原文：
    ///
    ///<pre>Orientation of a display feature in relation to screen</pre>
    ///
    ///
    #[serde(rename = "orientation")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub orientation: DisplayFeatureOrientation,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.DisplayFeature/properties/offset。
    ///
    ///协议原文：
    ///
    ///<pre>The offset from the screen origin in either the x (for vertical
    ///orientation) or y (for horizontal orientation) direction.</pre>
    ///
    ///
    #[serde(rename = "offset")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub offset: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.DisplayFeature/properties/maskLength。
    ///
    ///协议原文：
    ///
    ///<pre>A display feature may mask content such that it is not physically
    ///displayed - this length along with the offset describes this area.
    ///A display feature that only splits content will have a 0 mask_length.</pre>
    ///
    ///
    #[serde(rename = "maskLength")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub mask_length: i64,
}
impl<'de> serde::Deserialize<'de> for DisplayFeature {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DisplayFeature;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.DisplayFeature/properties/orientation。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Orientation of a display feature in relation to screen</pre>
                    ///
                    ///
                    #[serde(rename = "orientation")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub orientation: DisplayFeatureOrientation,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.DisplayFeature/properties/offset。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The offset from the screen origin in either the x (for vertical
                    ///orientation) or y (for horizontal orientation) direction.</pre>
                    ///
                    ///
                    #[serde(rename = "offset")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub offset: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.DisplayFeature/properties/maskLength。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A display feature may mask content such that it is not physically
                    ///displayed - this length along with the offset describes this area.
                    ///A display feature that only splits content will have a 0 mask_length.</pre>
                    ///
                    ///
                    #[serde(rename = "maskLength")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub mask_length: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DisplayFeature {
                    orientation: fields.orientation,
                    offset: fields.offset,
                    mask_length: fields.mask_length,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.type.MediaFeature。
#[derive(Debug, Clone, serde::Serialize)]
pub struct MediaFeature {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.MediaFeature/properties/name。
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.MediaFeature/properties/value。
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: String,
}
impl<'de> serde::Deserialize<'de> for MediaFeature {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MediaFeature;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.MediaFeature/properties/name。
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.MediaFeature/properties/value。
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(MediaFeature {
                    name: fields.name,
                    value: fields.value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.type.PressureMetadata。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PressureMetadata {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.PressureMetadata/properties/available。
    #[serde(rename = "available")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub available: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for PressureMetadata {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PressureMetadata;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.PressureMetadata/properties/available。
                    #[serde(rename = "available")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub available: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PressureMetadata {
                    available: fields.available,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Emulation.type.PressureSource。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PressureSource {
    ///协议值 "cpu"。
    Cpu,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl PressureSource {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::emulation::types::PressureSource::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Cpu => "cpu",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for PressureSource {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for PressureSource {
    fn from(value: String) -> Self {
        match value.as_str() {
            "cpu" => PressureSource::Cpu,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for PressureSource {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PressureSource {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PressureSource;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "cpu" => PressureSource::Cpu,
                    _ => PressureSource::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(PressureSource::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Emulation.type.PressureState。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PressureState {
    ///协议值 "nominal"。
    Nominal,
    ///协议值 "fair"。
    Fair,
    ///协议值 "serious"。
    Serious,
    ///协议值 "critical"。
    Critical,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl PressureState {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::emulation::types::PressureState::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Nominal => "nominal",
            Self::Fair => "fair",
            Self::Serious => "serious",
            Self::Critical => "critical",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for PressureState {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for PressureState {
    fn from(value: String) -> Self {
        match value.as_str() {
            "nominal" => PressureState::Nominal,
            "fair" => PressureState::Fair,
            "serious" => PressureState::Serious,
            "critical" => PressureState::Critical,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for PressureState {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PressureState {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PressureState;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "nominal" => PressureState::Nominal,
                    "fair" => PressureState::Fair,
                    "serious" => PressureState::Serious,
                    "critical" => PressureState::Critical,
                    _ => PressureState::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(PressureState::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.type.SafeAreaInsets。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SafeAreaInsets {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.SafeAreaInsets/properties/top。
    ///
    ///协议原文：
    ///
    ///<pre>Overrides safe-area-inset-top.</pre>
    ///
    ///
    #[serde(rename = "top")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub top: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.SafeAreaInsets/properties/topMax。
    ///
    ///协议原文：
    ///
    ///<pre>Overrides safe-area-max-inset-top.</pre>
    ///
    ///
    #[serde(rename = "topMax")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub top_max: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.SafeAreaInsets/properties/left。
    ///
    ///协议原文：
    ///
    ///<pre>Overrides safe-area-inset-left.</pre>
    ///
    ///
    #[serde(rename = "left")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub left: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.SafeAreaInsets/properties/leftMax。
    ///
    ///协议原文：
    ///
    ///<pre>Overrides safe-area-max-inset-left.</pre>
    ///
    ///
    #[serde(rename = "leftMax")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub left_max: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.SafeAreaInsets/properties/bottom。
    ///
    ///协议原文：
    ///
    ///<pre>Overrides safe-area-inset-bottom.</pre>
    ///
    ///
    #[serde(rename = "bottom")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub bottom: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.SafeAreaInsets/properties/bottomMax。
    ///
    ///协议原文：
    ///
    ///<pre>Overrides safe-area-max-inset-bottom.</pre>
    ///
    ///
    #[serde(rename = "bottomMax")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub bottom_max: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.SafeAreaInsets/properties/right。
    ///
    ///协议原文：
    ///
    ///<pre>Overrides safe-area-inset-right.</pre>
    ///
    ///
    #[serde(rename = "right")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub right: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.SafeAreaInsets/properties/rightMax。
    ///
    ///协议原文：
    ///
    ///<pre>Overrides safe-area-max-inset-right.</pre>
    ///
    ///
    #[serde(rename = "rightMax")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub right_max: Option<i64>,
}
impl<'de> serde::Deserialize<'de> for SafeAreaInsets {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SafeAreaInsets;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.SafeAreaInsets/properties/top。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Overrides safe-area-inset-top.</pre>
                    ///
                    ///
                    #[serde(rename = "top")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub top: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.SafeAreaInsets/properties/topMax。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Overrides safe-area-max-inset-top.</pre>
                    ///
                    ///
                    #[serde(rename = "topMax")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub top_max: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.SafeAreaInsets/properties/left。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Overrides safe-area-inset-left.</pre>
                    ///
                    ///
                    #[serde(rename = "left")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub left: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.SafeAreaInsets/properties/leftMax。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Overrides safe-area-max-inset-left.</pre>
                    ///
                    ///
                    #[serde(rename = "leftMax")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub left_max: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.SafeAreaInsets/properties/bottom。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Overrides safe-area-inset-bottom.</pre>
                    ///
                    ///
                    #[serde(rename = "bottom")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub bottom: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.SafeAreaInsets/properties/bottomMax。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Overrides safe-area-max-inset-bottom.</pre>
                    ///
                    ///
                    #[serde(rename = "bottomMax")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub bottom_max: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.SafeAreaInsets/properties/right。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Overrides safe-area-inset-right.</pre>
                    ///
                    ///
                    #[serde(rename = "right")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub right: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.SafeAreaInsets/properties/rightMax。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Overrides safe-area-max-inset-right.</pre>
                    ///
                    ///
                    #[serde(rename = "rightMax")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub right_max: Option<i64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SafeAreaInsets {
                    top: fields.top,
                    top_max: fields.top_max,
                    left: fields.left,
                    left_max: fields.left_max,
                    bottom: fields.bottom,
                    bottom_max: fields.bottom_max,
                    right: fields.right,
                    right_max: fields.right_max,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Emulation.type.ScreenId。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ScreenId(String);
impl ScreenId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::emulation::types::ScreenId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::emulation::types::ScreenId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::emulation::types::ScreenId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::emulation::types::ScreenId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::emulation::types::ScreenId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.type.ScreenInfo。
///
///协议原文：
///
///<pre>Screen information similar to the one returned by window.getScreenDetails() method,
///see https://w3c.github.io/window-management/#screendetailed.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ScreenInfo {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/left。
    ///
    ///协议原文：
    ///
    ///<pre>Offset of the left edge of the screen.</pre>
    ///
    ///
    #[serde(rename = "left")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub left: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/top。
    ///
    ///协议原文：
    ///
    ///<pre>Offset of the top edge of the screen.</pre>
    ///
    ///
    #[serde(rename = "top")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub top: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/width。
    ///
    ///协议原文：
    ///
    ///<pre>Width of the screen.</pre>
    ///
    ///
    #[serde(rename = "width")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub width: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/height。
    ///
    ///协议原文：
    ///
    ///<pre>Height of the screen.</pre>
    ///
    ///
    #[serde(rename = "height")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub height: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/availLeft。
    ///
    ///协议原文：
    ///
    ///<pre>Offset of the left edge of the available screen area.</pre>
    ///
    ///
    #[serde(rename = "availLeft")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub avail_left: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/availTop。
    ///
    ///协议原文：
    ///
    ///<pre>Offset of the top edge of the available screen area.</pre>
    ///
    ///
    #[serde(rename = "availTop")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub avail_top: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/availWidth。
    ///
    ///协议原文：
    ///
    ///<pre>Width of the available screen area.</pre>
    ///
    ///
    #[serde(rename = "availWidth")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub avail_width: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/availHeight。
    ///
    ///协议原文：
    ///
    ///<pre>Height of the available screen area.</pre>
    ///
    ///
    #[serde(rename = "availHeight")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub avail_height: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/devicePixelRatio。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies the screen's device pixel ratio.</pre>
    ///
    ///
    #[serde(rename = "devicePixelRatio")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub device_pixel_ratio: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/orientation。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies the screen's orientation.</pre>
    ///
    ///
    #[serde(rename = "orientation")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub orientation: crate::emulation::types::ScreenOrientation,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/colorDepth。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies the screen's color depth in bits.</pre>
    ///
    ///
    #[serde(rename = "colorDepth")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub color_depth: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/isExtended。
    ///
    ///协议原文：
    ///
    ///<pre>Indicates whether the device has multiple screens.</pre>
    ///
    ///
    #[serde(rename = "isExtended")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub is_extended: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/isInternal。
    ///
    ///协议原文：
    ///
    ///<pre>Indicates whether the screen is internal to the device or external, attached to the device.</pre>
    ///
    ///
    #[serde(rename = "isInternal")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub is_internal: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/isPrimary。
    ///
    ///协议原文：
    ///
    ///<pre>Indicates whether the screen is set as the the operating system primary screen.</pre>
    ///
    ///
    #[serde(rename = "isPrimary")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub is_primary: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/label。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies the descriptive label for the screen.</pre>
    ///
    ///
    #[serde(rename = "label")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub label: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/id。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies the unique identifier of the screen.</pre>
    ///
    ///
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: crate::emulation::types::ScreenId,
}
impl<'de> serde::Deserialize<'de> for ScreenInfo {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScreenInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/left。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Offset of the left edge of the screen.</pre>
                    ///
                    ///
                    #[serde(rename = "left")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub left: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/top。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Offset of the top edge of the screen.</pre>
                    ///
                    ///
                    #[serde(rename = "top")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub top: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/width。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Width of the screen.</pre>
                    ///
                    ///
                    #[serde(rename = "width")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub width: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/height。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Height of the screen.</pre>
                    ///
                    ///
                    #[serde(rename = "height")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub height: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/availLeft。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Offset of the left edge of the available screen area.</pre>
                    ///
                    ///
                    #[serde(rename = "availLeft")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub avail_left: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/availTop。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Offset of the top edge of the available screen area.</pre>
                    ///
                    ///
                    #[serde(rename = "availTop")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub avail_top: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/availWidth。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Width of the available screen area.</pre>
                    ///
                    ///
                    #[serde(rename = "availWidth")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub avail_width: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/availHeight。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Height of the available screen area.</pre>
                    ///
                    ///
                    #[serde(rename = "availHeight")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub avail_height: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/devicePixelRatio。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies the screen's device pixel ratio.</pre>
                    ///
                    ///
                    #[serde(rename = "devicePixelRatio")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub device_pixel_ratio: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/orientation。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies the screen's orientation.</pre>
                    ///
                    ///
                    #[serde(rename = "orientation")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub orientation: crate::emulation::types::ScreenOrientation,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/colorDepth。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies the screen's color depth in bits.</pre>
                    ///
                    ///
                    #[serde(rename = "colorDepth")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub color_depth: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/isExtended。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Indicates whether the device has multiple screens.</pre>
                    ///
                    ///
                    #[serde(rename = "isExtended")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub is_extended: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/isInternal。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Indicates whether the screen is internal to the device or external, attached to the device.</pre>
                    ///
                    ///
                    #[serde(rename = "isInternal")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub is_internal: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/isPrimary。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Indicates whether the screen is set as the the operating system primary screen.</pre>
                    ///
                    ///
                    #[serde(rename = "isPrimary")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub is_primary: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/label。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies the descriptive label for the screen.</pre>
                    ///
                    ///
                    #[serde(rename = "label")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub label: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenInfo/properties/id。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies the unique identifier of the screen.</pre>
                    ///
                    ///
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: crate::emulation::types::ScreenId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ScreenInfo {
                    left: fields.left,
                    top: fields.top,
                    width: fields.width,
                    height: fields.height,
                    avail_left: fields.avail_left,
                    avail_top: fields.avail_top,
                    avail_width: fields.avail_width,
                    avail_height: fields.avail_height,
                    device_pixel_ratio: fields.device_pixel_ratio,
                    orientation: fields.orientation,
                    color_depth: fields.color_depth,
                    is_extended: fields.is_extended,
                    is_internal: fields.is_internal,
                    is_primary: fields.is_primary,
                    label: fields.label,
                    id: fields.id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Emulation.type.ScreenOrientation/properties/type。
///
///协议原文：
///
///<pre>Orientation type.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ScreenOrientationType {
    ///协议值 "portraitPrimary"。
    PortraitPrimary,
    ///协议值 "portraitSecondary"。
    PortraitSecondary,
    ///协议值 "landscapePrimary"。
    LandscapePrimary,
    ///协议值 "landscapeSecondary"。
    LandscapeSecondary,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ScreenOrientationType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::emulation::types::ScreenOrientationType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::PortraitPrimary => "portraitPrimary",
            Self::PortraitSecondary => "portraitSecondary",
            Self::LandscapePrimary => "landscapePrimary",
            Self::LandscapeSecondary => "landscapeSecondary",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ScreenOrientationType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ScreenOrientationType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "portraitPrimary" => ScreenOrientationType::PortraitPrimary,
            "portraitSecondary" => ScreenOrientationType::PortraitSecondary,
            "landscapePrimary" => ScreenOrientationType::LandscapePrimary,
            "landscapeSecondary" => ScreenOrientationType::LandscapeSecondary,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ScreenOrientationType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ScreenOrientationType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScreenOrientationType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "portraitPrimary" => ScreenOrientationType::PortraitPrimary,
                    "portraitSecondary" => ScreenOrientationType::PortraitSecondary,
                    "landscapePrimary" => ScreenOrientationType::LandscapePrimary,
                    "landscapeSecondary" => ScreenOrientationType::LandscapeSecondary,
                    _ => ScreenOrientationType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ScreenOrientationType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.type.ScreenOrientation。
///
///协议原文：
///
///<pre>Screen orientation.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ScreenOrientation {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenOrientation/properties/type。
    ///
    ///协议原文：
    ///
    ///<pre>Orientation type.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: ScreenOrientationType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenOrientation/properties/angle。
    ///
    ///协议原文：
    ///
    ///<pre>Orientation angle.</pre>
    ///
    ///
    #[serde(rename = "angle")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub angle: i64,
}
impl<'de> serde::Deserialize<'de> for ScreenOrientation {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScreenOrientation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenOrientation/properties/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Orientation type.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: ScreenOrientationType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.ScreenOrientation/properties/angle。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Orientation angle.</pre>
                    ///
                    ///
                    #[serde(rename = "angle")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub angle: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ScreenOrientation {
                    r#type: fields.r#type,
                    angle: fields.angle,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.type.SensorMetadata。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SensorMetadata {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.SensorMetadata/properties/available。
    #[serde(rename = "available")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub available: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.SensorMetadata/properties/minimumFrequency。
    #[serde(rename = "minimumFrequency")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_frequency: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.SensorMetadata/properties/maximumFrequency。
    #[serde(rename = "maximumFrequency")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub maximum_frequency: Option<ydp_core::FiniteF64>,
}
impl<'de> serde::Deserialize<'de> for SensorMetadata {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SensorMetadata;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.SensorMetadata/properties/available。
                    #[serde(rename = "available")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub available: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.SensorMetadata/properties/minimumFrequency。
                    #[serde(rename = "minimumFrequency")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub minimum_frequency: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.SensorMetadata/properties/maximumFrequency。
                    #[serde(rename = "maximumFrequency")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub maximum_frequency: Option<ydp_core::FiniteF64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SensorMetadata {
                    available: fields.available,
                    minimum_frequency: fields.minimum_frequency,
                    maximum_frequency: fields.maximum_frequency,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.type.SensorReading。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SensorReading {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.SensorReading/properties/single。
    #[serde(rename = "single")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub single: Option<crate::emulation::types::SensorReadingSingle>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.SensorReading/properties/xyz。
    #[serde(rename = "xyz")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub xyz: Option<crate::emulation::types::SensorReadingXyz>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.SensorReading/properties/quaternion。
    #[serde(rename = "quaternion")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub quaternion: Option<crate::emulation::types::SensorReadingQuaternion>,
}
impl<'de> serde::Deserialize<'de> for SensorReading {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SensorReading;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.SensorReading/properties/single。
                    #[serde(rename = "single")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub single: Option<crate::emulation::types::SensorReadingSingle>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.SensorReading/properties/xyz。
                    #[serde(rename = "xyz")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub xyz: Option<crate::emulation::types::SensorReadingXyz>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.SensorReading/properties/quaternion。
                    #[serde(rename = "quaternion")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub quaternion: Option<crate::emulation::types::SensorReadingQuaternion>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SensorReading {
                    single: fields.single,
                    xyz: fields.xyz,
                    quaternion: fields.quaternion,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.type.SensorReadingQuaternion。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SensorReadingQuaternion {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.SensorReadingQuaternion/properties/x。
    #[serde(rename = "x")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub x: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.SensorReadingQuaternion/properties/y。
    #[serde(rename = "y")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub y: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.SensorReadingQuaternion/properties/z。
    #[serde(rename = "z")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub z: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.SensorReadingQuaternion/properties/w。
    #[serde(rename = "w")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub w: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for SensorReadingQuaternion {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SensorReadingQuaternion;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.SensorReadingQuaternion/properties/x。
                    #[serde(rename = "x")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub x: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.SensorReadingQuaternion/properties/y。
                    #[serde(rename = "y")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub y: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.SensorReadingQuaternion/properties/z。
                    #[serde(rename = "z")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub z: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.SensorReadingQuaternion/properties/w。
                    #[serde(rename = "w")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub w: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SensorReadingQuaternion {
                    x: fields.x,
                    y: fields.y,
                    z: fields.z,
                    w: fields.w,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.type.SensorReadingSingle。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SensorReadingSingle {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.SensorReadingSingle/properties/value。
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for SensorReadingSingle {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SensorReadingSingle;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.SensorReadingSingle/properties/value。
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SensorReadingSingle {
                    value: fields.value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.type.SensorReadingXYZ。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SensorReadingXyz {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.SensorReadingXYZ/properties/x。
    #[serde(rename = "x")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub x: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.SensorReadingXYZ/properties/y。
    #[serde(rename = "y")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub y: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.SensorReadingXYZ/properties/z。
    #[serde(rename = "z")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub z: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for SensorReadingXyz {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SensorReadingXyz;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.SensorReadingXYZ/properties/x。
                    #[serde(rename = "x")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub x: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.SensorReadingXYZ/properties/y。
                    #[serde(rename = "y")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub y: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.SensorReadingXYZ/properties/z。
                    #[serde(rename = "z")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub z: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SensorReadingXyz {
                    x: fields.x,
                    y: fields.y,
                    z: fields.z,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Emulation.type.SensorType。
///
///协议原文：
///
///<pre>Used to specify sensor types to emulate.
///See https://w3c.github.io/sensors/#automation for more information.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SensorType {
    ///协议值 "absolute-orientation"。
    AbsoluteOrientation,
    ///协议值 "accelerometer"。
    Accelerometer,
    ///协议值 "ambient-light"。
    AmbientLight,
    ///协议值 "gravity"。
    Gravity,
    ///协议值 "gyroscope"。
    Gyroscope,
    ///协议值 "linear-acceleration"。
    LinearAcceleration,
    ///协议值 "magnetometer"。
    Magnetometer,
    ///协议值 "relative-orientation"。
    RelativeOrientation,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl SensorType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::emulation::types::SensorType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::AbsoluteOrientation => "absolute-orientation",
            Self::Accelerometer => "accelerometer",
            Self::AmbientLight => "ambient-light",
            Self::Gravity => "gravity",
            Self::Gyroscope => "gyroscope",
            Self::LinearAcceleration => "linear-acceleration",
            Self::Magnetometer => "magnetometer",
            Self::RelativeOrientation => "relative-orientation",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for SensorType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for SensorType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "absolute-orientation" => SensorType::AbsoluteOrientation,
            "accelerometer" => SensorType::Accelerometer,
            "ambient-light" => SensorType::AmbientLight,
            "gravity" => SensorType::Gravity,
            "gyroscope" => SensorType::Gyroscope,
            "linear-acceleration" => SensorType::LinearAcceleration,
            "magnetometer" => SensorType::Magnetometer,
            "relative-orientation" => SensorType::RelativeOrientation,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for SensorType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SensorType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SensorType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "absolute-orientation" => SensorType::AbsoluteOrientation,
                    "accelerometer" => SensorType::Accelerometer,
                    "ambient-light" => SensorType::AmbientLight,
                    "gravity" => SensorType::Gravity,
                    "gyroscope" => SensorType::Gyroscope,
                    "linear-acceleration" => SensorType::LinearAcceleration,
                    "magnetometer" => SensorType::Magnetometer,
                    "relative-orientation" => SensorType::RelativeOrientation,
                    _ => SensorType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(SensorType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.type.UserAgentBrandVersion。
///
///协议原文：
///
///<pre>Used to specify User Agent Client Hints to emulate. See https://wicg.github.io/ua-client-hints</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct UserAgentBrandVersion {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.UserAgentBrandVersion/properties/brand。
    #[serde(rename = "brand")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub brand: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.UserAgentBrandVersion/properties/version。
    #[serde(rename = "version")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub version: String,
}
impl<'de> serde::Deserialize<'de> for UserAgentBrandVersion {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UserAgentBrandVersion;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.UserAgentBrandVersion/properties/brand。
                    #[serde(rename = "brand")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub brand: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.UserAgentBrandVersion/properties/version。
                    #[serde(rename = "version")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub version: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(UserAgentBrandVersion {
                    brand: fields.brand,
                    version: fields.version,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.type.UserAgentMetadata。
///
///协议原文：
///
///<pre>Used to specify User Agent Client Hints to emulate. See https://wicg.github.io/ua-client-hints
///Missing optional values will be filled in by the target with what it would normally use.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct UserAgentMetadata {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.UserAgentMetadata/properties/brands。
    ///
    ///协议原文：
    ///
    ///<pre>Brands appearing in Sec-CH-UA.</pre>
    ///
    ///
    #[serde(rename = "brands")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub brands: Option<Vec<crate::emulation::types::UserAgentBrandVersion>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.UserAgentMetadata/properties/fullVersionList。
    ///
    ///协议原文：
    ///
    ///<pre>Brands appearing in Sec-CH-UA-Full-Version-List.</pre>
    ///
    ///
    #[serde(rename = "fullVersionList")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub full_version_list: Option<Vec<crate::emulation::types::UserAgentBrandVersion>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.UserAgentMetadata/properties/fullVersion。此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "fullVersion")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub full_version: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.UserAgentMetadata/properties/platform。
    #[serde(rename = "platform")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub platform: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.UserAgentMetadata/properties/platformVersion。
    #[serde(rename = "platformVersion")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub platform_version: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.UserAgentMetadata/properties/architecture。
    #[serde(rename = "architecture")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub architecture: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.UserAgentMetadata/properties/model。
    #[serde(rename = "model")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub model: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.UserAgentMetadata/properties/mobile。
    #[serde(rename = "mobile")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub mobile: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.UserAgentMetadata/properties/bitness。
    #[serde(rename = "bitness")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub bitness: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.UserAgentMetadata/properties/wow64。
    #[serde(rename = "wow64")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub wow64: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.UserAgentMetadata/properties/formFactors。
    ///
    ///协议原文：
    ///
    ///<pre>Used to specify User Agent form-factor values.
    ///See https://wicg.github.io/ua-client-hints/#sec-ch-ua-form-factors</pre>
    ///
    ///
    #[serde(rename = "formFactors")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub form_factors: Option<Vec<String>>,
}
impl<'de> serde::Deserialize<'de> for UserAgentMetadata {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UserAgentMetadata;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.UserAgentMetadata/properties/brands。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Brands appearing in Sec-CH-UA.</pre>
                    ///
                    ///
                    #[serde(rename = "brands")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub brands: Option<Vec<crate::emulation::types::UserAgentBrandVersion>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.UserAgentMetadata/properties/fullVersionList。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Brands appearing in Sec-CH-UA-Full-Version-List.</pre>
                    ///
                    ///
                    #[serde(rename = "fullVersionList")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub full_version_list:
                        Option<Vec<crate::emulation::types::UserAgentBrandVersion>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.UserAgentMetadata/properties/fullVersion。此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "fullVersion")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub full_version: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.UserAgentMetadata/properties/platform。
                    #[serde(rename = "platform")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub platform: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.UserAgentMetadata/properties/platformVersion。
                    #[serde(rename = "platformVersion")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub platform_version: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.UserAgentMetadata/properties/architecture。
                    #[serde(rename = "architecture")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub architecture: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.UserAgentMetadata/properties/model。
                    #[serde(rename = "model")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub model: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.type.UserAgentMetadata/properties/mobile。
                    #[serde(rename = "mobile")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub mobile: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.UserAgentMetadata/properties/bitness。
                    #[serde(rename = "bitness")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub bitness: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.UserAgentMetadata/properties/wow64。
                    #[serde(rename = "wow64")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub wow64: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.UserAgentMetadata/properties/formFactors。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Used to specify User Agent form-factor values.
                    ///See https://wicg.github.io/ua-client-hints/#sec-ch-ua-form-factors</pre>
                    ///
                    ///
                    #[serde(rename = "formFactors")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub form_factors: Option<Vec<String>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(UserAgentMetadata {
                    brands: fields.brands,
                    full_version_list: fields.full_version_list,
                    full_version: fields.full_version,
                    platform: fields.platform,
                    platform_version: fields.platform_version,
                    architecture: fields.architecture,
                    model: fields.model,
                    mobile: fields.mobile,
                    bitness: fields.bitness,
                    wow64: fields.wow64,
                    form_factors: fields.form_factors,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Emulation.type.VirtualTimePolicy。
///
///协议原文：
///
///<pre>advance: If the scheduler runs out of immediate work, the virtual time base may fast forward to
///allow the next delayed task (if any) to run; pause: The virtual time base may not advance;
///pauseIfNetworkFetchesPending: The virtual time base may not advance if there are any pending
///resource fetches.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum VirtualTimePolicy {
    ///协议值 "advance"。
    Advance,
    ///协议值 "pause"。
    Pause,
    ///协议值 "pauseIfNetworkFetchesPending"。
    PauseIfNetworkFetchesPending,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl VirtualTimePolicy {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::emulation::types::VirtualTimePolicy::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Advance => "advance",
            Self::Pause => "pause",
            Self::PauseIfNetworkFetchesPending => "pauseIfNetworkFetchesPending",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for VirtualTimePolicy {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for VirtualTimePolicy {
    fn from(value: String) -> Self {
        match value.as_str() {
            "advance" => VirtualTimePolicy::Advance,
            "pause" => VirtualTimePolicy::Pause,
            "pauseIfNetworkFetchesPending" => VirtualTimePolicy::PauseIfNetworkFetchesPending,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for VirtualTimePolicy {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for VirtualTimePolicy {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = VirtualTimePolicy;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "advance" => VirtualTimePolicy::Advance,
                    "pause" => VirtualTimePolicy::Pause,
                    "pauseIfNetworkFetchesPending" => {
                        VirtualTimePolicy::PauseIfNetworkFetchesPending
                    }
                    _ => VirtualTimePolicy::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(VirtualTimePolicy::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.type.WorkAreaInsets。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct WorkAreaInsets {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.WorkAreaInsets/properties/top。
    ///
    ///协议原文：
    ///
    ///<pre>Work area top inset in pixels. Default is 0;</pre>
    ///
    ///
    #[serde(rename = "top")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub top: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.WorkAreaInsets/properties/left。
    ///
    ///协议原文：
    ///
    ///<pre>Work area left inset in pixels. Default is 0;</pre>
    ///
    ///
    #[serde(rename = "left")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub left: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.WorkAreaInsets/properties/bottom。
    ///
    ///协议原文：
    ///
    ///<pre>Work area bottom inset in pixels. Default is 0;</pre>
    ///
    ///
    #[serde(rename = "bottom")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub bottom: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.WorkAreaInsets/properties/right。
    ///
    ///协议原文：
    ///
    ///<pre>Work area right inset in pixels. Default is 0;</pre>
    ///
    ///
    #[serde(rename = "right")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub right: Option<i64>,
}
impl<'de> serde::Deserialize<'de> for WorkAreaInsets {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WorkAreaInsets;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.WorkAreaInsets/properties/top。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Work area top inset in pixels. Default is 0;</pre>
                    ///
                    ///
                    #[serde(rename = "top")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub top: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.WorkAreaInsets/properties/left。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Work area left inset in pixels. Default is 0;</pre>
                    ///
                    ///
                    #[serde(rename = "left")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub left: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.WorkAreaInsets/properties/bottom。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Work area bottom inset in pixels. Default is 0;</pre>
                    ///
                    ///
                    #[serde(rename = "bottom")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub bottom: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.type.WorkAreaInsets/properties/right。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Work area right inset in pixels. Default is 0;</pre>
                    ///
                    ///
                    #[serde(rename = "right")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub right: Option<i64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(WorkAreaInsets {
                    top: fields.top,
                    left: fields.left,
                    bottom: fields.bottom,
                    right: fields.right,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
