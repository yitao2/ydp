///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：BluetoothEmulation.type.CentralState。
///
///协议原文：
///
///<pre>Indicates the various states of Central.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CentralState {
    ///协议值 "absent"。
    Absent,
    ///协议值 "powered-off"。
    PoweredOff,
    ///协议值 "powered-on"。
    PoweredOn,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl CentralState {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::bluetooth_emulation::types::CentralState::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Absent => "absent",
            Self::PoweredOff => "powered-off",
            Self::PoweredOn => "powered-on",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for CentralState {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for CentralState {
    fn from(value: String) -> Self {
        match value.as_str() {
            "absent" => CentralState::Absent,
            "powered-off" => CentralState::PoweredOff,
            "powered-on" => CentralState::PoweredOn,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for CentralState {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CentralState {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CentralState;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "absent" => CentralState::Absent,
                    "powered-off" => CentralState::PoweredOff,
                    "powered-on" => CentralState::PoweredOn,
                    _ => CentralState::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(CentralState::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：BluetoothEmulation.type.CharacteristicOperationType。
///
///协议原文：
///
///<pre>Indicates the various types of characteristic operation.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CharacteristicOperationType {
    ///协议值 "read"。
    Read,
    ///协议值 "write"。
    Write,
    ///协议值 "subscribe-to-notifications"。
    SubscribeToNotifications,
    ///协议值 "unsubscribe-from-notifications"。
    UnsubscribeFromNotifications,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl CharacteristicOperationType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::bluetooth_emulation::types::CharacteristicOperationType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Read => "read",
            Self::Write => "write",
            Self::SubscribeToNotifications => "subscribe-to-notifications",
            Self::UnsubscribeFromNotifications => "unsubscribe-from-notifications",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for CharacteristicOperationType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for CharacteristicOperationType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "read" => CharacteristicOperationType::Read,
            "write" => CharacteristicOperationType::Write,
            "subscribe-to-notifications" => CharacteristicOperationType::SubscribeToNotifications,
            "unsubscribe-from-notifications" => {
                CharacteristicOperationType::UnsubscribeFromNotifications
            }
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for CharacteristicOperationType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CharacteristicOperationType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CharacteristicOperationType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "read" => CharacteristicOperationType::Read,
                    "write" => CharacteristicOperationType::Write,
                    "subscribe-to-notifications" => {
                        CharacteristicOperationType::SubscribeToNotifications
                    }
                    "unsubscribe-from-notifications" => {
                        CharacteristicOperationType::UnsubscribeFromNotifications
                    }
                    _ => CharacteristicOperationType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(CharacteristicOperationType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.type.CharacteristicProperties。
///
///协议原文：
///
///<pre>Describes the properties of a characteristic. This follows Bluetooth Core
///Specification BT 4.2 Vol 3 Part G 3.3.1. Characteristic Properties.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CharacteristicProperties {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.type.CharacteristicProperties/properties/broadcast。
    #[serde(rename = "broadcast")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub broadcast: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.type.CharacteristicProperties/properties/read。
    #[serde(rename = "read")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub read: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.type.CharacteristicProperties/properties/writeWithoutResponse。
    #[serde(rename = "writeWithoutResponse")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub write_without_response: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.type.CharacteristicProperties/properties/write。
    #[serde(rename = "write")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub write: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.type.CharacteristicProperties/properties/notify。
    #[serde(rename = "notify")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub notify: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.type.CharacteristicProperties/properties/indicate。
    #[serde(rename = "indicate")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub indicate: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.type.CharacteristicProperties/properties/authenticatedSignedWrites。
    #[serde(rename = "authenticatedSignedWrites")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub authenticated_signed_writes: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.type.CharacteristicProperties/properties/extendedProperties。
    #[serde(rename = "extendedProperties")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub extended_properties: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for CharacteristicProperties {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CharacteristicProperties;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.type.CharacteristicProperties/properties/broadcast。
                    #[serde(rename = "broadcast")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub broadcast: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.type.CharacteristicProperties/properties/read。
                    #[serde(rename = "read")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub read: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.type.CharacteristicProperties/properties/writeWithoutResponse。
                    #[serde(rename = "writeWithoutResponse")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub write_without_response: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.type.CharacteristicProperties/properties/write。
                    #[serde(rename = "write")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub write: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.type.CharacteristicProperties/properties/notify。
                    #[serde(rename = "notify")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub notify: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.type.CharacteristicProperties/properties/indicate。
                    #[serde(rename = "indicate")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub indicate: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.type.CharacteristicProperties/properties/authenticatedSignedWrites。
                    #[serde(rename = "authenticatedSignedWrites")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub authenticated_signed_writes: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.type.CharacteristicProperties/properties/extendedProperties。
                    #[serde(rename = "extendedProperties")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub extended_properties: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CharacteristicProperties {
                    broadcast: fields.broadcast,
                    read: fields.read,
                    write_without_response: fields.write_without_response,
                    write: fields.write,
                    notify: fields.notify,
                    indicate: fields.indicate,
                    authenticated_signed_writes: fields.authenticated_signed_writes,
                    extended_properties: fields.extended_properties,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：BluetoothEmulation.type.CharacteristicWriteType。
///
///协议原文：
///
///<pre>Indicates the various types of characteristic write.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CharacteristicWriteType {
    ///协议值 "write-default-deprecated"。
    WriteDefaultDeprecated,
    ///协议值 "write-with-response"。
    WriteWithResponse,
    ///协议值 "write-without-response"。
    WriteWithoutResponse,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl CharacteristicWriteType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::bluetooth_emulation::types::CharacteristicWriteType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::WriteDefaultDeprecated => "write-default-deprecated",
            Self::WriteWithResponse => "write-with-response",
            Self::WriteWithoutResponse => "write-without-response",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for CharacteristicWriteType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for CharacteristicWriteType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "write-default-deprecated" => CharacteristicWriteType::WriteDefaultDeprecated,
            "write-with-response" => CharacteristicWriteType::WriteWithResponse,
            "write-without-response" => CharacteristicWriteType::WriteWithoutResponse,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for CharacteristicWriteType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CharacteristicWriteType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CharacteristicWriteType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "write-default-deprecated" => CharacteristicWriteType::WriteDefaultDeprecated,
                    "write-with-response" => CharacteristicWriteType::WriteWithResponse,
                    "write-without-response" => CharacteristicWriteType::WriteWithoutResponse,
                    _ => CharacteristicWriteType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(CharacteristicWriteType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：BluetoothEmulation.type.DescriptorOperationType。
///
///协议原文：
///
///<pre>Indicates the various types of descriptor operation.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DescriptorOperationType {
    ///协议值 "read"。
    Read,
    ///协议值 "write"。
    Write,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl DescriptorOperationType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::bluetooth_emulation::types::DescriptorOperationType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Read => "read",
            Self::Write => "write",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for DescriptorOperationType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for DescriptorOperationType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "read" => DescriptorOperationType::Read,
            "write" => DescriptorOperationType::Write,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for DescriptorOperationType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DescriptorOperationType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DescriptorOperationType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "read" => DescriptorOperationType::Read,
                    "write" => DescriptorOperationType::Write,
                    _ => DescriptorOperationType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(DescriptorOperationType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：BluetoothEmulation.type.GATTOperationType。
///
///协议原文：
///
///<pre>Indicates the various types of GATT event.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum GattOperationType {
    ///协议值 "connection"。
    Connection,
    ///协议值 "discovery"。
    Discovery,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl GattOperationType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::bluetooth_emulation::types::GattOperationType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Connection => "connection",
            Self::Discovery => "discovery",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for GattOperationType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for GattOperationType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "connection" => GattOperationType::Connection,
            "discovery" => GattOperationType::Discovery,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for GattOperationType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GattOperationType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GattOperationType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "connection" => GattOperationType::Connection,
                    "discovery" => GattOperationType::Discovery,
                    _ => GattOperationType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(GattOperationType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.type.ManufacturerData。
///
///协议原文：
///
///<pre>Stores the manufacturer data</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ManufacturerData {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.type.ManufacturerData/properties/key。
    ///
    ///协议原文：
    ///
    ///<pre>Company identifier
    ///https://bitbucket.org/bluetooth-SIG/public/src/main/assigned_numbers/company_identifiers/company_identifiers.yaml
    ///https://usb.org/developers</pre>
    ///
    ///
    #[serde(rename = "key")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub key: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.type.ManufacturerData/properties/data。
    ///
    ///协议原文：
    ///
    ///<pre>Manufacturer-specific data (Encoded as a base64 string when passed over JSON)</pre>
    ///
    ///
    #[serde(rename = "data")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub data: String,
}
impl<'de> serde::Deserialize<'de> for ManufacturerData {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ManufacturerData;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.type.ManufacturerData/properties/key。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Company identifier
                    ///https://bitbucket.org/bluetooth-SIG/public/src/main/assigned_numbers/company_identifiers/company_identifiers.yaml
                    ///https://usb.org/developers</pre>
                    ///
                    ///
                    #[serde(rename = "key")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub key: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.type.ManufacturerData/properties/data。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Manufacturer-specific data (Encoded as a base64 string when passed over JSON)</pre>
                    ///
                    ///
                    #[serde(rename = "data")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub data: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ManufacturerData {
                    key: fields.key,
                    data: fields.data,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.type.ScanEntry。
///
///协议原文：
///
///<pre>Stores the advertisement packet information that is sent by a Bluetooth device.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ScanEntry {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.type.ScanEntry/properties/deviceAddress。
    #[serde(rename = "deviceAddress")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub device_address: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.type.ScanEntry/properties/rssi。
    #[serde(rename = "rssi")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub rssi: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.type.ScanEntry/properties/scanRecord。
    #[serde(rename = "scanRecord")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub scan_record: crate::bluetooth_emulation::types::ScanRecord,
}
impl<'de> serde::Deserialize<'de> for ScanEntry {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScanEntry;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.type.ScanEntry/properties/deviceAddress。
                    #[serde(rename = "deviceAddress")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub device_address: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.type.ScanEntry/properties/rssi。
                    #[serde(rename = "rssi")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub rssi: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BluetoothEmulation.type.ScanEntry/properties/scanRecord。
                    #[serde(rename = "scanRecord")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub scan_record: crate::bluetooth_emulation::types::ScanRecord,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ScanEntry {
                    device_address: fields.device_address,
                    rssi: fields.rssi,
                    scan_record: fields.scan_record,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BluetoothEmulation.type.ScanRecord。
///
///协议原文：
///
///<pre>Stores the byte data of the advertisement packet sent by a Bluetooth device.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ScanRecord {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.type.ScanRecord/properties/name。
    #[serde(rename = "name")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.type.ScanRecord/properties/uuids。
    #[serde(rename = "uuids")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub uuids: Option<Vec<String>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.type.ScanRecord/properties/appearance。
    ///
    ///协议原文：
    ///
    ///<pre>Stores the external appearance description of the device.</pre>
    ///
    ///
    #[serde(rename = "appearance")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub appearance: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.type.ScanRecord/properties/txPower。
    ///
    ///协议原文：
    ///
    ///<pre>Stores the transmission power of a broadcasting device.</pre>
    ///
    ///
    #[serde(rename = "txPower")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub tx_power: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.type.ScanRecord/properties/manufacturerData。
    ///
    ///协议原文：
    ///
    ///<pre>Key is the company identifier and the value is an array of bytes of
    ///manufacturer specific data.</pre>
    ///
    ///
    #[serde(rename = "manufacturerData")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub manufacturer_data: Option<Vec<crate::bluetooth_emulation::types::ManufacturerData>>,
}
impl<'de> serde::Deserialize<'de> for ScanRecord {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScanRecord;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.type.ScanRecord/properties/name。
                    #[serde(rename = "name")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub name: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.type.ScanRecord/properties/uuids。
                    #[serde(rename = "uuids")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub uuids: Option<Vec<String>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.type.ScanRecord/properties/appearance。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Stores the external appearance description of the device.</pre>
                    ///
                    ///
                    #[serde(rename = "appearance")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub appearance: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.type.ScanRecord/properties/txPower。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Stores the transmission power of a broadcasting device.</pre>
                    ///
                    ///
                    #[serde(rename = "txPower")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub tx_power: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：BluetoothEmulation.type.ScanRecord/properties/manufacturerData。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Key is the company identifier and the value is an array of bytes of
                    ///manufacturer specific data.</pre>
                    ///
                    ///
                    #[serde(rename = "manufacturerData")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub manufacturer_data:
                        Option<Vec<crate::bluetooth_emulation::types::ManufacturerData>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ScanRecord {
                    name: fields.name,
                    uuids: fields.uuids,
                    appearance: fields.appearance,
                    tx_power: fields.tx_power,
                    manufacturer_data: fields.manufacturer_data,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
