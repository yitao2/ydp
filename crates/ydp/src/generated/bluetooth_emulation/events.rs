///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.event.characteristicOperationReceived/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CharacteristicOperationReceivedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.event.characteristicOperationReceived/parameters/characteristicId。
    #[serde(rename = "characteristicId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub characteristic_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.event.characteristicOperationReceived/parameters/type。
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: crate::bluetooth_emulation::types::CharacteristicOperationType,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.event.characteristicOperationReceived/parameters/data。
    #[serde(rename = "data")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub data: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.event.characteristicOperationReceived/parameters/writeType。
    #[serde(rename = "writeType")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub write_type: Option<crate::bluetooth_emulation::types::CharacteristicWriteType>,
}
impl<'de> serde::Deserialize<'de> for CharacteristicOperationReceivedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CharacteristicOperationReceivedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.event.characteristicOperationReceived/parameters/characteristicId。
                    #[serde(rename = "characteristicId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub characteristic_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.event.characteristicOperationReceived/parameters/type。
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: crate::bluetooth_emulation::types::CharacteristicOperationType,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.event.characteristicOperationReceived/parameters/data。
                    #[serde(rename = "data")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub data: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.event.characteristicOperationReceived/parameters/writeType。
                    #[serde(rename = "writeType")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub write_type:
                        Option<crate::bluetooth_emulation::types::CharacteristicWriteType>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CharacteristicOperationReceivedParams {
                    characteristic_id: fields.characteristic_id,
                    r#type: fields.r#type,
                    data: fields.data,
                    write_type: fields.write_type,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：BluetoothEmulation.event.characteristicOperationReceived。
///
///协议原文：
///
///<pre>Event for when a characteristic operation of |type| to the characteristic
///respresented by |characteristicId| happened. |data| and |writeType| is
///expected to exist when |type| is write.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct CharacteristicOperationReceived;
impl ydp_core::Event for CharacteristicOperationReceived {
    type Params = CharacteristicOperationReceivedParams;
    const METHOD: &'static str = "BluetoothEmulation.characteristicOperationReceived";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.event.descriptorOperationReceived/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DescriptorOperationReceivedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.event.descriptorOperationReceived/parameters/descriptorId。
    #[serde(rename = "descriptorId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub descriptor_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.event.descriptorOperationReceived/parameters/type。
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: crate::bluetooth_emulation::types::DescriptorOperationType,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.event.descriptorOperationReceived/parameters/data。
    #[serde(rename = "data")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub data: Option<String>,
}
impl<'de> serde::Deserialize<'de> for DescriptorOperationReceivedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DescriptorOperationReceivedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.event.descriptorOperationReceived/parameters/descriptorId。
                    #[serde(rename = "descriptorId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub descriptor_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.event.descriptorOperationReceived/parameters/type。
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: crate::bluetooth_emulation::types::DescriptorOperationType,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.event.descriptorOperationReceived/parameters/data。
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
                Ok(DescriptorOperationReceivedParams {
                    descriptor_id: fields.descriptor_id,
                    r#type: fields.r#type,
                    data: fields.data,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：BluetoothEmulation.event.descriptorOperationReceived。
///
///协议原文：
///
///<pre>Event for when a descriptor operation of |type| to the descriptor
///respresented by |descriptorId| happened. |data| is expected to exist when
///|type| is write.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct DescriptorOperationReceived;
impl ydp_core::Event for DescriptorOperationReceived {
    type Params = DescriptorOperationReceivedParams;
    const METHOD: &'static str = "BluetoothEmulation.descriptorOperationReceived";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.event.gattOperationReceived/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GattOperationReceivedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.event.gattOperationReceived/parameters/address。
    #[serde(rename = "address")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub address: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.event.gattOperationReceived/parameters/type。
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: crate::bluetooth_emulation::types::GattOperationType,
}
impl<'de> serde::Deserialize<'de> for GattOperationReceivedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GattOperationReceivedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.event.gattOperationReceived/parameters/address。
                    #[serde(rename = "address")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub address: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.event.gattOperationReceived/parameters/type。
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: crate::bluetooth_emulation::types::GattOperationType,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GattOperationReceivedParams {
                    address: fields.address,
                    r#type: fields.r#type,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：BluetoothEmulation.event.gattOperationReceived。
///
///协议原文：
///
///<pre>Event for when a GATT operation of |type| to the peripheral with |address|
///happened.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GattOperationReceived;
impl ydp_core::Event for GattOperationReceived {
    type Params = GattOperationReceivedParams;
    const METHOD: &'static str = "BluetoothEmulation.gattOperationReceived";
}
