///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SystemInfo.type.GPUDevice。
///
///协议原文：
///
///<pre>Describes a single graphics processor (GPU).</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct GpuDevice {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.GPUDevice/properties/vendorId。
    ///
    ///协议原文：
    ///
    ///<pre>PCI ID of the GPU vendor, if available; 0 otherwise.</pre>
    ///
    ///
    #[serde(rename = "vendorId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub vendor_id: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.GPUDevice/properties/deviceId。
    ///
    ///协议原文：
    ///
    ///<pre>PCI ID of the GPU device, if available; 0 otherwise.</pre>
    ///
    ///
    #[serde(rename = "deviceId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub device_id: ydp_core::FiniteF64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SystemInfo.type.GPUDevice/properties/subSysId。
    ///
    ///协议原文：
    ///
    ///<pre>Sub sys ID of the GPU, only available on Windows.</pre>
    ///
    ///
    #[serde(rename = "subSysId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub sub_sys_id: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SystemInfo.type.GPUDevice/properties/revision。
    ///
    ///协议原文：
    ///
    ///<pre>Revision of the GPU, only available on Windows.</pre>
    ///
    ///
    #[serde(rename = "revision")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub revision: Option<ydp_core::FiniteF64>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.GPUDevice/properties/vendorString。
    ///
    ///协议原文：
    ///
    ///<pre>String description of the GPU vendor, if the PCI ID is not available.</pre>
    ///
    ///
    #[serde(rename = "vendorString")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub vendor_string: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.GPUDevice/properties/deviceString。
    ///
    ///协议原文：
    ///
    ///<pre>String description of the GPU device, if the PCI ID is not available.</pre>
    ///
    ///
    #[serde(rename = "deviceString")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub device_string: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.GPUDevice/properties/driverVendor。
    ///
    ///协议原文：
    ///
    ///<pre>String description of the GPU driver vendor.</pre>
    ///
    ///
    #[serde(rename = "driverVendor")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub driver_vendor: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.GPUDevice/properties/driverVersion。
    ///
    ///协议原文：
    ///
    ///<pre>String description of the GPU driver version.</pre>
    ///
    ///
    #[serde(rename = "driverVersion")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub driver_version: String,
}
impl<'de> serde::Deserialize<'de> for GpuDevice {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GpuDevice;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.GPUDevice/properties/vendorId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>PCI ID of the GPU vendor, if available; 0 otherwise.</pre>
                    ///
                    ///
                    #[serde(rename = "vendorId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub vendor_id: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.GPUDevice/properties/deviceId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>PCI ID of the GPU device, if available; 0 otherwise.</pre>
                    ///
                    ///
                    #[serde(rename = "deviceId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub device_id: ydp_core::FiniteF64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SystemInfo.type.GPUDevice/properties/subSysId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Sub sys ID of the GPU, only available on Windows.</pre>
                    ///
                    ///
                    #[serde(rename = "subSysId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub sub_sys_id: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SystemInfo.type.GPUDevice/properties/revision。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Revision of the GPU, only available on Windows.</pre>
                    ///
                    ///
                    #[serde(rename = "revision")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub revision: Option<ydp_core::FiniteF64>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.GPUDevice/properties/vendorString。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>String description of the GPU vendor, if the PCI ID is not available.</pre>
                    ///
                    ///
                    #[serde(rename = "vendorString")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub vendor_string: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.GPUDevice/properties/deviceString。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>String description of the GPU device, if the PCI ID is not available.</pre>
                    ///
                    ///
                    #[serde(rename = "deviceString")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub device_string: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.GPUDevice/properties/driverVendor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>String description of the GPU driver vendor.</pre>
                    ///
                    ///
                    #[serde(rename = "driverVendor")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub driver_vendor: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.GPUDevice/properties/driverVersion。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>String description of the GPU driver version.</pre>
                    ///
                    ///
                    #[serde(rename = "driverVersion")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub driver_version: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GpuDevice {
                    vendor_id: fields.vendor_id,
                    device_id: fields.device_id,
                    sub_sys_id: fields.sub_sys_id,
                    revision: fields.revision,
                    vendor_string: fields.vendor_string,
                    device_string: fields.device_string,
                    driver_vendor: fields.driver_vendor,
                    driver_version: fields.driver_version,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SystemInfo.type.GPUInfo。
///
///协议原文：
///
///<pre>Provides information about the GPU(s) on the system.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct GpuInfo {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.GPUInfo/properties/devices。
    ///
    ///协议原文：
    ///
    ///<pre>The graphics devices on the system. Element 0 is the primary GPU.</pre>
    ///
    ///
    #[serde(rename = "devices")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub devices: Vec<crate::system_info::types::GpuDevice>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SystemInfo.type.GPUInfo/properties/auxAttributes。
    ///
    ///协议原文：
    ///
    ///<pre>An optional dictionary of additional GPU related attributes.</pre>
    ///
    ///
    #[serde(rename = "auxAttributes")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub aux_attributes: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SystemInfo.type.GPUInfo/properties/featureStatus。
    ///
    ///协议原文：
    ///
    ///<pre>An optional dictionary of graphics features and their status.</pre>
    ///
    ///
    #[serde(rename = "featureStatus")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub feature_status: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.GPUInfo/properties/driverBugWorkarounds。
    ///
    ///协议原文：
    ///
    ///<pre>An optional array of GPU driver bug workarounds.</pre>
    ///
    ///
    #[serde(rename = "driverBugWorkarounds")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub driver_bug_workarounds: Vec<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.GPUInfo/properties/videoDecoding。
    ///
    ///协议原文：
    ///
    ///<pre>Supported accelerated video decoding capabilities.</pre>
    ///
    ///
    #[serde(rename = "videoDecoding")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub video_decoding: Vec<crate::system_info::types::VideoDecodeAcceleratorCapability>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.GPUInfo/properties/videoEncoding。
    ///
    ///协议原文：
    ///
    ///<pre>Supported accelerated video encoding capabilities.</pre>
    ///
    ///
    #[serde(rename = "videoEncoding")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub video_encoding: Vec<crate::system_info::types::VideoEncodeAcceleratorCapability>,
}
impl<'de> serde::Deserialize<'de> for GpuInfo {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GpuInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.GPUInfo/properties/devices。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The graphics devices on the system. Element 0 is the primary GPU.</pre>
                    ///
                    ///
                    #[serde(rename = "devices")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub devices: Vec<crate::system_info::types::GpuDevice>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SystemInfo.type.GPUInfo/properties/auxAttributes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An optional dictionary of additional GPU related attributes.</pre>
                    ///
                    ///
                    #[serde(rename = "auxAttributes")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub aux_attributes:
                        Option<std::collections::BTreeMap<String, serde_json::Value>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SystemInfo.type.GPUInfo/properties/featureStatus。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An optional dictionary of graphics features and their status.</pre>
                    ///
                    ///
                    #[serde(rename = "featureStatus")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub feature_status:
                        Option<std::collections::BTreeMap<String, serde_json::Value>>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.GPUInfo/properties/driverBugWorkarounds。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An optional array of GPU driver bug workarounds.</pre>
                    ///
                    ///
                    #[serde(rename = "driverBugWorkarounds")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub driver_bug_workarounds: Vec<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.GPUInfo/properties/videoDecoding。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Supported accelerated video decoding capabilities.</pre>
                    ///
                    ///
                    #[serde(rename = "videoDecoding")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub video_decoding:
                        Vec<crate::system_info::types::VideoDecodeAcceleratorCapability>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.GPUInfo/properties/videoEncoding。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Supported accelerated video encoding capabilities.</pre>
                    ///
                    ///
                    #[serde(rename = "videoEncoding")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub video_encoding:
                        Vec<crate::system_info::types::VideoEncodeAcceleratorCapability>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GpuInfo {
                    devices: fields.devices,
                    aux_attributes: fields.aux_attributes,
                    feature_status: fields.feature_status,
                    driver_bug_workarounds: fields.driver_bug_workarounds,
                    video_decoding: fields.video_decoding,
                    video_encoding: fields.video_encoding,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：SystemInfo.type.ImageType。
///
///协议原文：
///
///<pre>Image format of a given image.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ImageType {
    ///协议值 "jpeg"。
    Jpeg,
    ///协议值 "webp"。
    Webp,
    ///协议值 "unknown"。
    KnownUnknown,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ImageType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::system_info::types::ImageType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Jpeg => "jpeg",
            Self::Webp => "webp",
            Self::KnownUnknown => "unknown",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ImageType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ImageType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "jpeg" => ImageType::Jpeg,
            "webp" => ImageType::Webp,
            "unknown" => ImageType::KnownUnknown,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ImageType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ImageType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImageType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "jpeg" => ImageType::Jpeg,
                    "webp" => ImageType::Webp,
                    "unknown" => ImageType::KnownUnknown,
                    _ => ImageType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ImageType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SystemInfo.type.ProcessInfo。
///
///协议原文：
///
///<pre>Represents process info.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProcessInfo {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.ProcessInfo/properties/type。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies process type.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.ProcessInfo/properties/id。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies process id.</pre>
    ///
    ///
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.ProcessInfo/properties/cpuTime。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies cumulative CPU usage in seconds across all threads of the
    ///process since the process start.</pre>
    ///
    ///
    #[serde(rename = "cpuTime")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cpu_time: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for ProcessInfo {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ProcessInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.ProcessInfo/properties/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies process type.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.ProcessInfo/properties/id。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies process id.</pre>
                    ///
                    ///
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.ProcessInfo/properties/cpuTime。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies cumulative CPU usage in seconds across all threads of the
                    ///process since the process start.</pre>
                    ///
                    ///
                    #[serde(rename = "cpuTime")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cpu_time: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ProcessInfo {
                    r#type: fields.r#type,
                    id: fields.id,
                    cpu_time: fields.cpu_time,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SystemInfo.type.Size。
///
///协议原文：
///
///<pre>Describes the width and height dimensions of an entity.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct Size {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.Size/properties/width。
    ///
    ///协议原文：
    ///
    ///<pre>Width in pixels.</pre>
    ///
    ///
    #[serde(rename = "width")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub width: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.Size/properties/height。
    ///
    ///协议原文：
    ///
    ///<pre>Height in pixels.</pre>
    ///
    ///
    #[serde(rename = "height")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub height: i64,
}
impl<'de> serde::Deserialize<'de> for Size {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Size;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.Size/properties/width。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Width in pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "width")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub width: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.Size/properties/height。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Height in pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "height")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub height: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Size {
                    width: fields.width,
                    height: fields.height,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：SystemInfo.type.SubsamplingFormat。
///
///协议原文：
///
///<pre>YUV subsampling type of the pixels of a given image.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SubsamplingFormat {
    ///协议值 "yuv420"。
    Yuv420,
    ///协议值 "yuv422"。
    Yuv422,
    ///协议值 "yuv444"。
    Yuv444,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl SubsamplingFormat {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::system_info::types::SubsamplingFormat::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Yuv420 => "yuv420",
            Self::Yuv422 => "yuv422",
            Self::Yuv444 => "yuv444",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for SubsamplingFormat {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for SubsamplingFormat {
    fn from(value: String) -> Self {
        match value.as_str() {
            "yuv420" => SubsamplingFormat::Yuv420,
            "yuv422" => SubsamplingFormat::Yuv422,
            "yuv444" => SubsamplingFormat::Yuv444,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for SubsamplingFormat {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SubsamplingFormat {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubsamplingFormat;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "yuv420" => SubsamplingFormat::Yuv420,
                    "yuv422" => SubsamplingFormat::Yuv422,
                    "yuv444" => SubsamplingFormat::Yuv444,
                    _ => SubsamplingFormat::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(SubsamplingFormat::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SystemInfo.type.VideoDecodeAcceleratorCapability。
///
///协议原文：
///
///<pre>Describes a supported video decoding profile with its associated minimum and
///maximum resolutions.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct VideoDecodeAcceleratorCapability {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.VideoDecodeAcceleratorCapability/properties/profile。
    ///
    ///协议原文：
    ///
    ///<pre>Video codec profile that is supported, e.g. VP9 Profile 2.</pre>
    ///
    ///
    #[serde(rename = "profile")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub profile: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.VideoDecodeAcceleratorCapability/properties/maxResolution。
    ///
    ///协议原文：
    ///
    ///<pre>Maximum video dimensions in pixels supported for this |profile|.</pre>
    ///
    ///
    #[serde(rename = "maxResolution")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub max_resolution: crate::system_info::types::Size,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.VideoDecodeAcceleratorCapability/properties/minResolution。
    ///
    ///协议原文：
    ///
    ///<pre>Minimum video dimensions in pixels supported for this |profile|.</pre>
    ///
    ///
    #[serde(rename = "minResolution")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub min_resolution: crate::system_info::types::Size,
}
impl<'de> serde::Deserialize<'de> for VideoDecodeAcceleratorCapability {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = VideoDecodeAcceleratorCapability;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.VideoDecodeAcceleratorCapability/properties/profile。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Video codec profile that is supported, e.g. VP9 Profile 2.</pre>
                    ///
                    ///
                    #[serde(rename = "profile")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub profile: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.VideoDecodeAcceleratorCapability/properties/maxResolution。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Maximum video dimensions in pixels supported for this |profile|.</pre>
                    ///
                    ///
                    #[serde(rename = "maxResolution")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub max_resolution: crate::system_info::types::Size,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.VideoDecodeAcceleratorCapability/properties/minResolution。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Minimum video dimensions in pixels supported for this |profile|.</pre>
                    ///
                    ///
                    #[serde(rename = "minResolution")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub min_resolution: crate::system_info::types::Size,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(VideoDecodeAcceleratorCapability {
                    profile: fields.profile,
                    max_resolution: fields.max_resolution,
                    min_resolution: fields.min_resolution,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SystemInfo.type.VideoEncodeAcceleratorCapability。
///
///协议原文：
///
///<pre>Describes a supported video encoding profile with its associated maximum
///resolution and maximum framerate.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct VideoEncodeAcceleratorCapability {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.VideoEncodeAcceleratorCapability/properties/profile。
    ///
    ///协议原文：
    ///
    ///<pre>Video codec profile that is supported, e.g H264 Main.</pre>
    ///
    ///
    #[serde(rename = "profile")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub profile: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.VideoEncodeAcceleratorCapability/properties/maxResolution。
    ///
    ///协议原文：
    ///
    ///<pre>Maximum video dimensions in pixels supported for this |profile|.</pre>
    ///
    ///
    #[serde(rename = "maxResolution")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub max_resolution: crate::system_info::types::Size,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.VideoEncodeAcceleratorCapability/properties/maxFramerateNumerator。
    ///
    ///协议原文：
    ///
    ///<pre>Maximum encoding framerate in frames per second supported for this
    ///|profile|, as fraction's numerator and denominator, e.g. 24/1 fps,
    ///24000/1001 fps, etc.</pre>
    ///
    ///
    #[serde(rename = "maxFramerateNumerator")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub max_framerate_numerator: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.VideoEncodeAcceleratorCapability/properties/maxFramerateDenominator。
    #[serde(rename = "maxFramerateDenominator")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub max_framerate_denominator: i64,
}
impl<'de> serde::Deserialize<'de> for VideoEncodeAcceleratorCapability {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = VideoEncodeAcceleratorCapability;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.VideoEncodeAcceleratorCapability/properties/profile。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Video codec profile that is supported, e.g H264 Main.</pre>
                    ///
                    ///
                    #[serde(rename = "profile")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub profile: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.VideoEncodeAcceleratorCapability/properties/maxResolution。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Maximum video dimensions in pixels supported for this |profile|.</pre>
                    ///
                    ///
                    #[serde(rename = "maxResolution")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub max_resolution: crate::system_info::types::Size,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.VideoEncodeAcceleratorCapability/properties/maxFramerateNumerator。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Maximum encoding framerate in frames per second supported for this
                    ///|profile|, as fraction's numerator and denominator, e.g. 24/1 fps,
                    ///24000/1001 fps, etc.</pre>
                    ///
                    ///
                    #[serde(rename = "maxFramerateNumerator")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub max_framerate_numerator: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.type.VideoEncodeAcceleratorCapability/properties/maxFramerateDenominator。
                    #[serde(rename = "maxFramerateDenominator")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub max_framerate_denominator: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(VideoEncodeAcceleratorCapability {
                    profile: fields.profile,
                    max_resolution: fields.max_resolution,
                    max_framerate_numerator: fields.max_framerate_numerator,
                    max_framerate_denominator: fields.max_framerate_denominator,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
