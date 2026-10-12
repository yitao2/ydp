///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DeviceAccess.event.deviceRequestPrompted/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeviceRequestPromptedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DeviceAccess.event.deviceRequestPrompted/parameters/id。
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: crate::device_access::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DeviceAccess.event.deviceRequestPrompted/parameters/devices。
    #[serde(rename = "devices")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub devices: Vec<crate::device_access::types::PromptDevice>,
}
impl<'de> serde::Deserialize<'de> for DeviceRequestPromptedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceRequestPromptedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DeviceAccess.event.deviceRequestPrompted/parameters/id。
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: crate::device_access::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DeviceAccess.event.deviceRequestPrompted/parameters/devices。
                    #[serde(rename = "devices")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub devices: Vec<crate::device_access::types::PromptDevice>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DeviceRequestPromptedParams {
                    id: fields.id,
                    devices: fields.devices,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DeviceAccess.event.deviceRequestPrompted。
///
///协议原文：
///
///<pre>A device request opened a user prompt to select a device. Respond with the
///selectPrompt or cancelPrompt command.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct DeviceRequestPrompted;
impl ydp_core::Event for DeviceRequestPrompted {
    type Params = DeviceRequestPromptedParams;
    const METHOD: &'static str = "DeviceAccess.deviceRequestPrompted";
}
