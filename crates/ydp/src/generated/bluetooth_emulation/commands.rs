///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.addCharacteristic/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddCharacteristicParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.addCharacteristic/parameters/serviceId。
    #[serde(rename = "serviceId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub service_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.addCharacteristic/parameters/characteristicUuid。
    #[serde(rename = "characteristicUuid")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub characteristic_uuid: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.addCharacteristic/parameters/properties。
    #[serde(rename = "properties")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub properties: crate::bluetooth_emulation::types::CharacteristicProperties,
}
impl<'de> serde::Deserialize<'de> for AddCharacteristicParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddCharacteristicParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.addCharacteristic/parameters/serviceId。
                    #[serde(rename = "serviceId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub service_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.addCharacteristic/parameters/characteristicUuid。
                    #[serde(rename = "characteristicUuid")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub characteristic_uuid: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.addCharacteristic/parameters/properties。
                    #[serde(rename = "properties")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub properties: crate::bluetooth_emulation::types::CharacteristicProperties,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AddCharacteristicParams {
                    service_id: fields.service_id,
                    characteristic_uuid: fields.characteristic_uuid,
                    properties: fields.properties,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：BluetoothEmulation.command.addCharacteristic。
///
///协议原文：
///
///<pre>Adds a characteristic with |characteristicUuid| and |properties| to the
///service represented by |serviceId|.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct AddCharacteristic;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.addCharacteristic/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddCharacteristicResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.addCharacteristic/returns/characteristicId。
    ///
    ///协议原文：
    ///
    ///<pre>An identifier that uniquely represents this characteristic.</pre>
    ///
    ///
    #[serde(rename = "characteristicId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub characteristic_id: String,
}
impl<'de> serde::Deserialize<'de> for AddCharacteristicResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddCharacteristicResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.addCharacteristic/returns/characteristicId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An identifier that uniquely represents this characteristic.</pre>
                    ///
                    ///
                    #[serde(rename = "characteristicId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub characteristic_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AddCharacteristicResponse {
                    characteristic_id: fields.characteristic_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for AddCharacteristic {
    type Params = AddCharacteristicParams;
    type Response = AddCharacteristicResponse;
    const METHOD: &'static str = "BluetoothEmulation.addCharacteristic";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.addDescriptor/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddDescriptorParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.addDescriptor/parameters/characteristicId。
    #[serde(rename = "characteristicId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub characteristic_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.addDescriptor/parameters/descriptorUuid。
    #[serde(rename = "descriptorUuid")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub descriptor_uuid: String,
}
impl<'de> serde::Deserialize<'de> for AddDescriptorParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddDescriptorParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.addDescriptor/parameters/characteristicId。
                    #[serde(rename = "characteristicId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub characteristic_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.addDescriptor/parameters/descriptorUuid。
                    #[serde(rename = "descriptorUuid")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub descriptor_uuid: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AddDescriptorParams {
                    characteristic_id: fields.characteristic_id,
                    descriptor_uuid: fields.descriptor_uuid,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：BluetoothEmulation.command.addDescriptor。
///
///协议原文：
///
///<pre>Adds a descriptor with |descriptorUuid| to the characteristic respresented
///by |characteristicId|.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct AddDescriptor;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.addDescriptor/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddDescriptorResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.addDescriptor/returns/descriptorId。
    ///
    ///协议原文：
    ///
    ///<pre>An identifier that uniquely represents this descriptor.</pre>
    ///
    ///
    #[serde(rename = "descriptorId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub descriptor_id: String,
}
impl<'de> serde::Deserialize<'de> for AddDescriptorResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddDescriptorResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.addDescriptor/returns/descriptorId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An identifier that uniquely represents this descriptor.</pre>
                    ///
                    ///
                    #[serde(rename = "descriptorId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub descriptor_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AddDescriptorResponse {
                    descriptor_id: fields.descriptor_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for AddDescriptor {
    type Params = AddDescriptorParams;
    type Response = AddDescriptorResponse;
    const METHOD: &'static str = "BluetoothEmulation.addDescriptor";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.addService/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddServiceParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.addService/parameters/address。
    #[serde(rename = "address")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub address: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.addService/parameters/serviceUuid。
    #[serde(rename = "serviceUuid")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub service_uuid: String,
}
impl<'de> serde::Deserialize<'de> for AddServiceParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddServiceParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.addService/parameters/address。
                    #[serde(rename = "address")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub address: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.addService/parameters/serviceUuid。
                    #[serde(rename = "serviceUuid")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub service_uuid: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AddServiceParams {
                    address: fields.address,
                    service_uuid: fields.service_uuid,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：BluetoothEmulation.command.addService。
///
///协议原文：
///
///<pre>Adds a service with |serviceUuid| to the peripheral with |address|.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct AddService;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.addService/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddServiceResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.addService/returns/serviceId。
    ///
    ///协议原文：
    ///
    ///<pre>An identifier that uniquely represents this service.</pre>
    ///
    ///
    #[serde(rename = "serviceId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub service_id: String,
}
impl<'de> serde::Deserialize<'de> for AddServiceResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddServiceResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.addService/returns/serviceId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An identifier that uniquely represents this service.</pre>
                    ///
                    ///
                    #[serde(rename = "serviceId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub service_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AddServiceResponse {
                    service_id: fields.service_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for AddService {
    type Params = AddServiceParams;
    type Response = AddServiceResponse;
    const METHOD: &'static str = "BluetoothEmulation.addService";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.disable/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct DisableParams {}
impl<'de> serde::Deserialize<'de> for DisableParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DisableParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DisableParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：BluetoothEmulation.command.disable。
///
///协议原文：
///
///<pre>Disable the BluetoothEmulation domain.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.disable/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DisableResponse {}
impl<'de> serde::Deserialize<'de> for DisableResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DisableResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DisableResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Disable {
    type Params = DisableParams;
    type Response = DisableResponse;
    const METHOD: &'static str = "BluetoothEmulation.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.enable/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EnableParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.enable/parameters/state。
    ///
    ///协议原文：
    ///
    ///<pre>State of the simulated central.</pre>
    ///
    ///
    #[serde(rename = "state")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub state: crate::bluetooth_emulation::types::CentralState,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.enable/parameters/leSupported。
    ///
    ///协议原文：
    ///
    ///<pre>If the simulated central supports low-energy.</pre>
    ///
    ///
    #[serde(rename = "leSupported")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub le_supported: bool,
}
impl<'de> serde::Deserialize<'de> for EnableParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EnableParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.enable/parameters/state。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>State of the simulated central.</pre>
                    ///
                    ///
                    #[serde(rename = "state")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub state: crate::bluetooth_emulation::types::CentralState,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.enable/parameters/leSupported。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If the simulated central supports low-energy.</pre>
                    ///
                    ///
                    #[serde(rename = "leSupported")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub le_supported: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EnableParams {
                    state: fields.state,
                    le_supported: fields.le_supported,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：BluetoothEmulation.command.enable。
///
///协议原文：
///
///<pre>Enable the BluetoothEmulation domain.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.enable/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EnableResponse {}
impl<'de> serde::Deserialize<'de> for EnableResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EnableResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EnableResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Enable {
    type Params = EnableParams;
    type Response = EnableResponse;
    const METHOD: &'static str = "BluetoothEmulation.enable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.removeCharacteristic/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveCharacteristicParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.removeCharacteristic/parameters/characteristicId。
    #[serde(rename = "characteristicId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub characteristic_id: String,
}
impl<'de> serde::Deserialize<'de> for RemoveCharacteristicParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveCharacteristicParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.removeCharacteristic/parameters/characteristicId。
                    #[serde(rename = "characteristicId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub characteristic_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RemoveCharacteristicParams {
                    characteristic_id: fields.characteristic_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：BluetoothEmulation.command.removeCharacteristic。
///
///协议原文：
///
///<pre>Removes the characteristic respresented by |characteristicId| from the
///simulated central.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RemoveCharacteristic;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.removeCharacteristic/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveCharacteristicResponse {}
impl<'de> serde::Deserialize<'de> for RemoveCharacteristicResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveCharacteristicResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RemoveCharacteristicResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RemoveCharacteristic {
    type Params = RemoveCharacteristicParams;
    type Response = RemoveCharacteristicResponse;
    const METHOD: &'static str = "BluetoothEmulation.removeCharacteristic";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.removeDescriptor/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveDescriptorParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.removeDescriptor/parameters/descriptorId。
    #[serde(rename = "descriptorId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub descriptor_id: String,
}
impl<'de> serde::Deserialize<'de> for RemoveDescriptorParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveDescriptorParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.removeDescriptor/parameters/descriptorId。
                    #[serde(rename = "descriptorId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub descriptor_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RemoveDescriptorParams {
                    descriptor_id: fields.descriptor_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：BluetoothEmulation.command.removeDescriptor。
///
///协议原文：
///
///<pre>Removes the descriptor with |descriptorId| from the simulated central.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RemoveDescriptor;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.removeDescriptor/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveDescriptorResponse {}
impl<'de> serde::Deserialize<'de> for RemoveDescriptorResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveDescriptorResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RemoveDescriptorResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RemoveDescriptor {
    type Params = RemoveDescriptorParams;
    type Response = RemoveDescriptorResponse;
    const METHOD: &'static str = "BluetoothEmulation.removeDescriptor";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.removeService/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveServiceParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.removeService/parameters/serviceId。
    #[serde(rename = "serviceId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub service_id: String,
}
impl<'de> serde::Deserialize<'de> for RemoveServiceParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveServiceParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.removeService/parameters/serviceId。
                    #[serde(rename = "serviceId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub service_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RemoveServiceParams {
                    service_id: fields.service_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：BluetoothEmulation.command.removeService。
///
///协议原文：
///
///<pre>Removes the service respresented by |serviceId| from the simulated central.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RemoveService;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.removeService/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveServiceResponse {}
impl<'de> serde::Deserialize<'de> for RemoveServiceResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveServiceResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RemoveServiceResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RemoveService {
    type Params = RemoveServiceParams;
    type Response = RemoveServiceResponse;
    const METHOD: &'static str = "BluetoothEmulation.removeService";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.setSimulatedCentralState/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetSimulatedCentralStateParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.setSimulatedCentralState/parameters/state。
    ///
    ///协议原文：
    ///
    ///<pre>State of the simulated central.</pre>
    ///
    ///
    #[serde(rename = "state")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub state: crate::bluetooth_emulation::types::CentralState,
}
impl<'de> serde::Deserialize<'de> for SetSimulatedCentralStateParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetSimulatedCentralStateParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.setSimulatedCentralState/parameters/state。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>State of the simulated central.</pre>
                    ///
                    ///
                    #[serde(rename = "state")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub state: crate::bluetooth_emulation::types::CentralState,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetSimulatedCentralStateParams {
                    state: fields.state,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：BluetoothEmulation.command.setSimulatedCentralState。
///
///协议原文：
///
///<pre>Set the state of the simulated central.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetSimulatedCentralState;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.setSimulatedCentralState/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetSimulatedCentralStateResponse {}
impl<'de> serde::Deserialize<'de> for SetSimulatedCentralStateResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetSimulatedCentralStateResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetSimulatedCentralStateResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetSimulatedCentralState {
    type Params = SetSimulatedCentralStateParams;
    type Response = SetSimulatedCentralStateResponse;
    const METHOD: &'static str = "BluetoothEmulation.setSimulatedCentralState";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.simulateAdvertisement/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SimulateAdvertisementParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulateAdvertisement/parameters/entry。
    #[serde(rename = "entry")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub entry: crate::bluetooth_emulation::types::ScanEntry,
}
impl<'de> serde::Deserialize<'de> for SimulateAdvertisementParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SimulateAdvertisementParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulateAdvertisement/parameters/entry。
                    #[serde(rename = "entry")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub entry: crate::bluetooth_emulation::types::ScanEntry,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SimulateAdvertisementParams {
                    entry: fields.entry,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：BluetoothEmulation.command.simulateAdvertisement。
///
///协议原文：
///
///<pre>Simulates an advertisement packet described in |entry| being received by
///the central.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SimulateAdvertisement;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.simulateAdvertisement/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SimulateAdvertisementResponse {}
impl<'de> serde::Deserialize<'de> for SimulateAdvertisementResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SimulateAdvertisementResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SimulateAdvertisementResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SimulateAdvertisement {
    type Params = SimulateAdvertisementParams;
    type Response = SimulateAdvertisementResponse;
    const METHOD: &'static str = "BluetoothEmulation.simulateAdvertisement";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.simulateCharacteristicOperationResponse/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SimulateCharacteristicOperationResponseParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulateCharacteristicOperationResponse/parameters/characteristicId。
    #[serde(rename = "characteristicId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub characteristic_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulateCharacteristicOperationResponse/parameters/type。
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: crate::bluetooth_emulation::types::CharacteristicOperationType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulateCharacteristicOperationResponse/parameters/code。
    #[serde(rename = "code")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub code: i64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.command.simulateCharacteristicOperationResponse/parameters/data。
    #[serde(rename = "data")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub data: Option<String>,
}
impl<'de> serde::Deserialize<'de> for SimulateCharacteristicOperationResponseParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SimulateCharacteristicOperationResponseParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulateCharacteristicOperationResponse/parameters/characteristicId。
                    #[serde(rename = "characteristicId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub characteristic_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulateCharacteristicOperationResponse/parameters/type。
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: crate::bluetooth_emulation::types::CharacteristicOperationType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulateCharacteristicOperationResponse/parameters/code。
                    #[serde(rename = "code")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub code: i64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.command.simulateCharacteristicOperationResponse/parameters/data。
                    #[serde(rename = "data")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub data: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SimulateCharacteristicOperationResponseParams {
                    characteristic_id: fields.characteristic_id,
                    r#type: fields.r#type,
                    code: fields.code,
                    data: fields.data,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：BluetoothEmulation.command.simulateCharacteristicOperationResponse。
///
///协议原文：
///
///<pre>Simulates the response from the characteristic with |characteristicId| for a
///characteristic operation of |type|. The |code| value follows the Error
///Codes from Bluetooth Core Specification Vol 3 Part F 3.4.1.1 Error Response.
///The |data| is expected to exist when simulating a successful read operation
///response.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SimulateCharacteristicOperationResponse;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.simulateCharacteristicOperationResponse/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SimulateCharacteristicOperationResponseResponse {}
impl<'de> serde::Deserialize<'de> for SimulateCharacteristicOperationResponseResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SimulateCharacteristicOperationResponseResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SimulateCharacteristicOperationResponseResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SimulateCharacteristicOperationResponse {
    type Params = SimulateCharacteristicOperationResponseParams;
    type Response = SimulateCharacteristicOperationResponseResponse;
    const METHOD: &'static str = "BluetoothEmulation.simulateCharacteristicOperationResponse";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.simulateDescriptorOperationResponse/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SimulateDescriptorOperationResponseParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulateDescriptorOperationResponse/parameters/descriptorId。
    #[serde(rename = "descriptorId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub descriptor_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulateDescriptorOperationResponse/parameters/type。
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: crate::bluetooth_emulation::types::DescriptorOperationType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulateDescriptorOperationResponse/parameters/code。
    #[serde(rename = "code")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub code: i64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.command.simulateDescriptorOperationResponse/parameters/data。
    #[serde(rename = "data")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub data: Option<String>,
}
impl<'de> serde::Deserialize<'de> for SimulateDescriptorOperationResponseParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SimulateDescriptorOperationResponseParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulateDescriptorOperationResponse/parameters/descriptorId。
                    #[serde(rename = "descriptorId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub descriptor_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulateDescriptorOperationResponse/parameters/type。
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: crate::bluetooth_emulation::types::DescriptorOperationType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulateDescriptorOperationResponse/parameters/code。
                    #[serde(rename = "code")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub code: i64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.command.simulateDescriptorOperationResponse/parameters/data。
                    #[serde(rename = "data")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub data: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SimulateDescriptorOperationResponseParams {
                    descriptor_id: fields.descriptor_id,
                    r#type: fields.r#type,
                    code: fields.code,
                    data: fields.data,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：BluetoothEmulation.command.simulateDescriptorOperationResponse。
///
///协议原文：
///
///<pre>Simulates the response from the descriptor with |descriptorId| for a
///descriptor operation of |type|. The |code| value follows the Error
///Codes from Bluetooth Core Specification Vol 3 Part F 3.4.1.1 Error Response.
///The |data| is expected to exist when simulating a successful read operation
///response.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SimulateDescriptorOperationResponse;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.simulateDescriptorOperationResponse/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SimulateDescriptorOperationResponseResponse {}
impl<'de> serde::Deserialize<'de> for SimulateDescriptorOperationResponseResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SimulateDescriptorOperationResponseResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SimulateDescriptorOperationResponseResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SimulateDescriptorOperationResponse {
    type Params = SimulateDescriptorOperationResponseParams;
    type Response = SimulateDescriptorOperationResponseResponse;
    const METHOD: &'static str = "BluetoothEmulation.simulateDescriptorOperationResponse";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.simulateGATTDisconnection/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SimulateGattDisconnectionParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulateGATTDisconnection/parameters/address。
    #[serde(rename = "address")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub address: String,
}
impl<'de> serde::Deserialize<'de> for SimulateGattDisconnectionParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SimulateGattDisconnectionParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulateGATTDisconnection/parameters/address。
                    #[serde(rename = "address")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub address: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SimulateGattDisconnectionParams {
                    address: fields.address,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：BluetoothEmulation.command.simulateGATTDisconnection。
///
///协议原文：
///
///<pre>Simulates a GATT disconnection from the peripheral with |address|.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SimulateGattDisconnection;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.simulateGATTDisconnection/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SimulateGattDisconnectionResponse {}
impl<'de> serde::Deserialize<'de> for SimulateGattDisconnectionResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SimulateGattDisconnectionResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SimulateGattDisconnectionResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SimulateGattDisconnection {
    type Params = SimulateGattDisconnectionParams;
    type Response = SimulateGattDisconnectionResponse;
    const METHOD: &'static str = "BluetoothEmulation.simulateGATTDisconnection";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.simulateGATTOperationResponse/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SimulateGattOperationResponseParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulateGATTOperationResponse/parameters/address。
    #[serde(rename = "address")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub address: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulateGATTOperationResponse/parameters/type。
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: crate::bluetooth_emulation::types::GattOperationType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulateGATTOperationResponse/parameters/code。
    #[serde(rename = "code")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub code: i64,
}
impl<'de> serde::Deserialize<'de> for SimulateGattOperationResponseParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SimulateGattOperationResponseParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulateGATTOperationResponse/parameters/address。
                    #[serde(rename = "address")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub address: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulateGATTOperationResponse/parameters/type。
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: crate::bluetooth_emulation::types::GattOperationType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulateGATTOperationResponse/parameters/code。
                    #[serde(rename = "code")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub code: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SimulateGattOperationResponseParams {
                    address: fields.address,
                    r#type: fields.r#type,
                    code: fields.code,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：BluetoothEmulation.command.simulateGATTOperationResponse。
///
///协议原文：
///
///<pre>Simulates the response code from the peripheral with |address| for a
///GATT operation of |type|. The |code| value follows the HCI Error Codes from
///Bluetooth Core Specification Vol 2 Part D 1.3 List Of Error Codes.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SimulateGattOperationResponse;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.simulateGATTOperationResponse/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SimulateGattOperationResponseResponse {}
impl<'de> serde::Deserialize<'de> for SimulateGattOperationResponseResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SimulateGattOperationResponseResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SimulateGattOperationResponseResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SimulateGattOperationResponse {
    type Params = SimulateGattOperationResponseParams;
    type Response = SimulateGattOperationResponseResponse;
    const METHOD: &'static str = "BluetoothEmulation.simulateGATTOperationResponse";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.simulatePreconnectedPeripheral/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SimulatePreconnectedPeripheralParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulatePreconnectedPeripheral/parameters/address。
    #[serde(rename = "address")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub address: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulatePreconnectedPeripheral/parameters/name。
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulatePreconnectedPeripheral/parameters/manufacturerData。
    #[serde(rename = "manufacturerData")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub manufacturer_data: Vec<crate::bluetooth_emulation::types::ManufacturerData>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulatePreconnectedPeripheral/parameters/knownServiceUuids。
    #[serde(rename = "knownServiceUuids")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub known_service_uuids: Vec<String>,
}
impl<'de> serde::Deserialize<'de> for SimulatePreconnectedPeripheralParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SimulatePreconnectedPeripheralParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulatePreconnectedPeripheral/parameters/address。
                    #[serde(rename = "address")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub address: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulatePreconnectedPeripheral/parameters/name。
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulatePreconnectedPeripheral/parameters/manufacturerData。
                    #[serde(rename = "manufacturerData")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub manufacturer_data: Vec<crate::bluetooth_emulation::types::ManufacturerData>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.command.simulatePreconnectedPeripheral/parameters/knownServiceUuids。
                    #[serde(rename = "knownServiceUuids")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub known_service_uuids: Vec<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SimulatePreconnectedPeripheralParams {
                    address: fields.address,
                    name: fields.name,
                    manufacturer_data: fields.manufacturer_data,
                    known_service_uuids: fields.known_service_uuids,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：BluetoothEmulation.command.simulatePreconnectedPeripheral。
///
///协议原文：
///
///<pre>Simulates a peripheral with |address|, |name| and |knownServiceUuids|
///that has already been connected to the system.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SimulatePreconnectedPeripheral;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.command.simulatePreconnectedPeripheral/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SimulatePreconnectedPeripheralResponse {}
impl<'de> serde::Deserialize<'de> for SimulatePreconnectedPeripheralResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SimulatePreconnectedPeripheralResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SimulatePreconnectedPeripheralResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SimulatePreconnectedPeripheral {
    type Params = SimulatePreconnectedPeripheralParams;
    type Response = SimulatePreconnectedPeripheralResponse;
    const METHOD: &'static str = "BluetoothEmulation.simulatePreconnectedPeripheral";
}
