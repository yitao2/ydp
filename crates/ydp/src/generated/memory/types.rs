///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Memory.type.DOMCounter。
///
///协议原文：
///
///<pre>DOM object counter data.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct DomCounter {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.type.DOMCounter/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Object name. Note: object names should be presumed volatile and clients should not expect
    ///the returned names to be consistent across runs.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.type.DOMCounter/properties/count。
    ///
    ///协议原文：
    ///
    ///<pre>Object count.</pre>
    ///
    ///
    #[serde(rename = "count")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub count: i64,
}
impl<'de> serde::Deserialize<'de> for DomCounter {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DomCounter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.type.DOMCounter/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Object name. Note: object names should be presumed volatile and clients should not expect
                    ///the returned names to be consistent across runs.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.type.DOMCounter/properties/count。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Object count.</pre>
                    ///
                    ///
                    #[serde(rename = "count")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub count: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DomCounter {
                    name: fields.name,
                    count: fields.count,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Memory.type.Module。
///
///协议原文：
///
///<pre>Executable module information</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct Module {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.type.Module/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Name of the module.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.type.Module/properties/uuid。
    ///
    ///协议原文：
    ///
    ///<pre>UUID of the module.</pre>
    ///
    ///
    #[serde(rename = "uuid")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub uuid: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.type.Module/properties/baseAddress。
    ///
    ///协议原文：
    ///
    ///<pre>Base address where the module is loaded into memory. Encoded as a decimal
    ///or hexadecimal (0x prefixed) string.</pre>
    ///
    ///
    #[serde(rename = "baseAddress")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub base_address: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.type.Module/properties/size。
    ///
    ///协议原文：
    ///
    ///<pre>Size of the module in bytes.</pre>
    ///
    ///
    #[serde(rename = "size")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub size: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for Module {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Module;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.type.Module/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Name of the module.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.type.Module/properties/uuid。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>UUID of the module.</pre>
                    ///
                    ///
                    #[serde(rename = "uuid")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub uuid: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.type.Module/properties/baseAddress。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Base address where the module is loaded into memory. Encoded as a decimal
                    ///or hexadecimal (0x prefixed) string.</pre>
                    ///
                    ///
                    #[serde(rename = "baseAddress")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub base_address: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.type.Module/properties/size。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Size of the module in bytes.</pre>
                    ///
                    ///
                    #[serde(rename = "size")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub size: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Module {
                    name: fields.name,
                    uuid: fields.uuid,
                    base_address: fields.base_address,
                    size: fields.size,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Memory.type.PressureLevel。
///
///协议原文：
///
///<pre>Memory pressure level.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PressureLevel {
    ///协议值 "moderate"。
    Moderate,
    ///协议值 "critical"。
    Critical,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl PressureLevel {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::memory::types::PressureLevel::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Moderate => "moderate",
            Self::Critical => "critical",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for PressureLevel {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for PressureLevel {
    fn from(value: String) -> Self {
        match value.as_str() {
            "moderate" => PressureLevel::Moderate,
            "critical" => PressureLevel::Critical,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for PressureLevel {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PressureLevel {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PressureLevel;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "moderate" => PressureLevel::Moderate,
                    "critical" => PressureLevel::Critical,
                    _ => PressureLevel::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(PressureLevel::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Memory.type.SamplingProfile。
///
///协议原文：
///
///<pre>Array of heap profile samples.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct SamplingProfile {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.type.SamplingProfile/properties/samples。
    #[serde(rename = "samples")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub samples: Vec<crate::memory::types::SamplingProfileNode>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.type.SamplingProfile/properties/modules。
    #[serde(rename = "modules")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub modules: Vec<crate::memory::types::Module>,
}
impl<'de> serde::Deserialize<'de> for SamplingProfile {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SamplingProfile;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.type.SamplingProfile/properties/samples。
                    #[serde(rename = "samples")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub samples: Vec<crate::memory::types::SamplingProfileNode>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.type.SamplingProfile/properties/modules。
                    #[serde(rename = "modules")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub modules: Vec<crate::memory::types::Module>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SamplingProfile {
                    samples: fields.samples,
                    modules: fields.modules,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Memory.type.SamplingProfileNode。
///
///协议原文：
///
///<pre>Heap profile sample.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct SamplingProfileNode {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.type.SamplingProfileNode/properties/size。
    ///
    ///协议原文：
    ///
    ///<pre>Size of the sampled allocation.</pre>
    ///
    ///
    #[serde(rename = "size")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub size: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.type.SamplingProfileNode/properties/total。
    ///
    ///协议原文：
    ///
    ///<pre>Total bytes attributed to this sample.</pre>
    ///
    ///
    #[serde(rename = "total")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub total: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.type.SamplingProfileNode/properties/stack。
    ///
    ///协议原文：
    ///
    ///<pre>Execution stack at the point of allocation.</pre>
    ///
    ///
    #[serde(rename = "stack")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub stack: Vec<String>,
}
impl<'de> serde::Deserialize<'de> for SamplingProfileNode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SamplingProfileNode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.type.SamplingProfileNode/properties/size。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Size of the sampled allocation.</pre>
                    ///
                    ///
                    #[serde(rename = "size")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub size: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.type.SamplingProfileNode/properties/total。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Total bytes attributed to this sample.</pre>
                    ///
                    ///
                    #[serde(rename = "total")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub total: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Memory.type.SamplingProfileNode/properties/stack。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Execution stack at the point of allocation.</pre>
                    ///
                    ///
                    #[serde(rename = "stack")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub stack: Vec<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SamplingProfileNode {
                    size: fields.size,
                    total: fields.total,
                    stack: fields.stack,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
