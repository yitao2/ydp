///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：DeviceAccess.type.DeviceId。
///
///协议原文：
///
///<pre>A device id.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct DeviceId(String);
impl DeviceId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::device_access::types::DeviceId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::device_access::types::DeviceId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::device_access::types::DeviceId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::device_access::types::DeviceId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::device_access::types::DeviceId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DeviceAccess.type.PromptDevice。
///
///协议原文：
///
///<pre>Device information displayed in a user prompt to select a device.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct PromptDevice {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DeviceAccess.type.PromptDevice/properties/id。
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: crate::device_access::types::DeviceId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DeviceAccess.type.PromptDevice/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Display name as it appears in a device request user prompt.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
}
impl<'de> serde::Deserialize<'de> for PromptDevice {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PromptDevice;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DeviceAccess.type.PromptDevice/properties/id。
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: crate::device_access::types::DeviceId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DeviceAccess.type.PromptDevice/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Display name as it appears in a device request user prompt.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PromptDevice {
                    id: fields.id,
                    name: fields.name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：DeviceAccess.type.RequestId。
///
///协议原文：
///
///<pre>Device request id.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct RequestId(String);
impl RequestId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::device_access::types::RequestId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::device_access::types::RequestId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::device_access::types::RequestId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::device_access::types::RequestId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::device_access::types::RequestId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
