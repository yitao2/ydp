///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IndexedDB.type.DataEntry。
///
///协议原文：
///
///<pre>Data entry.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct DataEntry {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.DataEntry/properties/key。
    ///
    ///协议原文：
    ///
    ///<pre>Key object.</pre>
    ///
    ///
    #[serde(rename = "key")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub key: crate::runtime::types::RemoteObject,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.DataEntry/properties/primaryKey。
    ///
    ///协议原文：
    ///
    ///<pre>Primary key object.</pre>
    ///
    ///
    #[serde(rename = "primaryKey")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub primary_key: crate::runtime::types::RemoteObject,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.DataEntry/properties/value。
    ///
    ///协议原文：
    ///
    ///<pre>Value object.</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: crate::runtime::types::RemoteObject,
}
impl<'de> serde::Deserialize<'de> for DataEntry {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DataEntry;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.DataEntry/properties/key。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Key object.</pre>
                    ///
                    ///
                    #[serde(rename = "key")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub key: crate::runtime::types::RemoteObject,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.DataEntry/properties/primaryKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Primary key object.</pre>
                    ///
                    ///
                    #[serde(rename = "primaryKey")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub primary_key: crate::runtime::types::RemoteObject,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.DataEntry/properties/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Value object.</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: crate::runtime::types::RemoteObject,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DataEntry {
                    key: fields.key,
                    primary_key: fields.primary_key,
                    value: fields.value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IndexedDB.type.DatabaseWithObjectStores。
///
///协议原文：
///
///<pre>Database with an array of object stores.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct DatabaseWithObjectStores {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.DatabaseWithObjectStores/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Database name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.DatabaseWithObjectStores/properties/version。
    ///
    ///协议原文：
    ///
    ///<pre>Database version (type is not 'integer', as the standard
    ///requires the version number to be 'unsigned long long')</pre>
    ///
    ///
    #[serde(rename = "version")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub version: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.DatabaseWithObjectStores/properties/objectStores。
    ///
    ///协议原文：
    ///
    ///<pre>Object stores in this database.</pre>
    ///
    ///
    #[serde(rename = "objectStores")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub object_stores: Vec<crate::indexed_db::types::ObjectStore>,
}
impl<'de> serde::Deserialize<'de> for DatabaseWithObjectStores {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DatabaseWithObjectStores;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.DatabaseWithObjectStores/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Database name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.DatabaseWithObjectStores/properties/version。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Database version (type is not 'integer', as the standard
                    ///requires the version number to be 'unsigned long long')</pre>
                    ///
                    ///
                    #[serde(rename = "version")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub version: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.DatabaseWithObjectStores/properties/objectStores。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Object stores in this database.</pre>
                    ///
                    ///
                    #[serde(rename = "objectStores")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub object_stores: Vec<crate::indexed_db::types::ObjectStore>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DatabaseWithObjectStores {
                    name: fields.name,
                    version: fields.version,
                    object_stores: fields.object_stores,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：IndexedDB.type.Key/properties/type。
///
///协议原文：
///
///<pre>Key type.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum KeyType {
    ///协议值 "number"。
    Number,
    ///协议值 "string"。
    String,
    ///协议值 "date"。
    Date,
    ///协议值 "array"。
    Array,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl KeyType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::indexed_db::types::KeyType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Number => "number",
            Self::String => "string",
            Self::Date => "date",
            Self::Array => "array",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for KeyType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for KeyType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "number" => KeyType::Number,
            "string" => KeyType::String,
            "date" => KeyType::Date,
            "array" => KeyType::Array,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for KeyType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KeyType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = KeyType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "number" => KeyType::Number,
                    "string" => KeyType::String,
                    "date" => KeyType::Date,
                    "array" => KeyType::Array,
                    _ => KeyType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(KeyType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IndexedDB.type.Key。
///
///协议原文：
///
///<pre>Key.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct Key {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.Key/properties/type。
    ///
    ///协议原文：
    ///
    ///<pre>Key type.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: KeyType,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.type.Key/properties/number。
    ///
    ///协议原文：
    ///
    ///<pre>Number value.</pre>
    ///
    ///
    #[serde(rename = "number")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub number: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.type.Key/properties/string。
    ///
    ///协议原文：
    ///
    ///<pre>String value.</pre>
    ///
    ///
    #[serde(rename = "string")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub string: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.type.Key/properties/date。
    ///
    ///协议原文：
    ///
    ///<pre>Date value.</pre>
    ///
    ///
    #[serde(rename = "date")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub date: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.type.Key/properties/array。
    ///
    ///协议原文：
    ///
    ///<pre>Array value.</pre>
    ///
    ///
    #[serde(rename = "array")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub array: Option<Vec<crate::indexed_db::types::Key>>,
}
impl<'de> serde::Deserialize<'de> for Key {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Key;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.Key/properties/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Key type.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: KeyType,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.type.Key/properties/number。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Number value.</pre>
                    ///
                    ///
                    #[serde(rename = "number")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub number: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.type.Key/properties/string。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>String value.</pre>
                    ///
                    ///
                    #[serde(rename = "string")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub string: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.type.Key/properties/date。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Date value.</pre>
                    ///
                    ///
                    #[serde(rename = "date")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub date: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.type.Key/properties/array。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Array value.</pre>
                    ///
                    ///
                    #[serde(rename = "array")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub array: Option<Vec<crate::indexed_db::types::Key>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Key {
                    r#type: fields.r#type,
                    number: fields.number,
                    string: fields.string,
                    date: fields.date,
                    array: fields.array,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：IndexedDB.type.KeyPath/properties/type。
///
///协议原文：
///
///<pre>Key path type.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum KeyPathType {
    ///协议值 "null"。
    Null,
    ///协议值 "string"。
    String,
    ///协议值 "array"。
    Array,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl KeyPathType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::indexed_db::types::KeyPathType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Null => "null",
            Self::String => "string",
            Self::Array => "array",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for KeyPathType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for KeyPathType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "null" => KeyPathType::Null,
            "string" => KeyPathType::String,
            "array" => KeyPathType::Array,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for KeyPathType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KeyPathType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = KeyPathType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "null" => KeyPathType::Null,
                    "string" => KeyPathType::String,
                    "array" => KeyPathType::Array,
                    _ => KeyPathType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(KeyPathType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IndexedDB.type.KeyPath。
///
///协议原文：
///
///<pre>Key path.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct KeyPath {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.KeyPath/properties/type。
    ///
    ///协议原文：
    ///
    ///<pre>Key path type.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: KeyPathType,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.type.KeyPath/properties/string。
    ///
    ///协议原文：
    ///
    ///<pre>String value.</pre>
    ///
    ///
    #[serde(rename = "string")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub string: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.type.KeyPath/properties/array。
    ///
    ///协议原文：
    ///
    ///<pre>Array value.</pre>
    ///
    ///
    #[serde(rename = "array")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub array: Option<Vec<String>>,
}
impl<'de> serde::Deserialize<'de> for KeyPath {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = KeyPath;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.KeyPath/properties/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Key path type.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: KeyPathType,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.type.KeyPath/properties/string。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>String value.</pre>
                    ///
                    ///
                    #[serde(rename = "string")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub string: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.type.KeyPath/properties/array。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Array value.</pre>
                    ///
                    ///
                    #[serde(rename = "array")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub array: Option<Vec<String>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(KeyPath {
                    r#type: fields.r#type,
                    string: fields.string,
                    array: fields.array,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IndexedDB.type.KeyRange。
///
///协议原文：
///
///<pre>Key range.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct KeyRange {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.type.KeyRange/properties/lower。
    ///
    ///协议原文：
    ///
    ///<pre>Lower bound.</pre>
    ///
    ///
    #[serde(rename = "lower")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub lower: Option<crate::indexed_db::types::Key>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.type.KeyRange/properties/upper。
    ///
    ///协议原文：
    ///
    ///<pre>Upper bound.</pre>
    ///
    ///
    #[serde(rename = "upper")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub upper: Option<crate::indexed_db::types::Key>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.KeyRange/properties/lowerOpen。
    ///
    ///协议原文：
    ///
    ///<pre>If true lower bound is open.</pre>
    ///
    ///
    #[serde(rename = "lowerOpen")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub lower_open: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.KeyRange/properties/upperOpen。
    ///
    ///协议原文：
    ///
    ///<pre>If true upper bound is open.</pre>
    ///
    ///
    #[serde(rename = "upperOpen")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub upper_open: bool,
}
impl<'de> serde::Deserialize<'de> for KeyRange {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = KeyRange;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.type.KeyRange/properties/lower。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Lower bound.</pre>
                    ///
                    ///
                    #[serde(rename = "lower")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub lower: Option<crate::indexed_db::types::Key>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.type.KeyRange/properties/upper。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Upper bound.</pre>
                    ///
                    ///
                    #[serde(rename = "upper")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub upper: Option<crate::indexed_db::types::Key>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.KeyRange/properties/lowerOpen。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true lower bound is open.</pre>
                    ///
                    ///
                    #[serde(rename = "lowerOpen")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub lower_open: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.KeyRange/properties/upperOpen。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true upper bound is open.</pre>
                    ///
                    ///
                    #[serde(rename = "upperOpen")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub upper_open: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(KeyRange {
                    lower: fields.lower,
                    upper: fields.upper,
                    lower_open: fields.lower_open,
                    upper_open: fields.upper_open,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IndexedDB.type.ObjectStore。
///
///协议原文：
///
///<pre>Object store.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ObjectStore {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.ObjectStore/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Object store name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.ObjectStore/properties/keyPath。
    ///
    ///协议原文：
    ///
    ///<pre>Object store key path.</pre>
    ///
    ///
    #[serde(rename = "keyPath")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub key_path: crate::indexed_db::types::KeyPath,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.ObjectStore/properties/autoIncrement。
    ///
    ///协议原文：
    ///
    ///<pre>If true, object store has auto increment flag set.</pre>
    ///
    ///
    #[serde(rename = "autoIncrement")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub auto_increment: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.ObjectStore/properties/indexes。
    ///
    ///协议原文：
    ///
    ///<pre>Indexes in this object store.</pre>
    ///
    ///
    #[serde(rename = "indexes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub indexes: Vec<crate::indexed_db::types::ObjectStoreIndex>,
}
impl<'de> serde::Deserialize<'de> for ObjectStore {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ObjectStore;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.ObjectStore/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Object store name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.ObjectStore/properties/keyPath。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Object store key path.</pre>
                    ///
                    ///
                    #[serde(rename = "keyPath")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub key_path: crate::indexed_db::types::KeyPath,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.ObjectStore/properties/autoIncrement。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, object store has auto increment flag set.</pre>
                    ///
                    ///
                    #[serde(rename = "autoIncrement")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub auto_increment: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.ObjectStore/properties/indexes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Indexes in this object store.</pre>
                    ///
                    ///
                    #[serde(rename = "indexes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub indexes: Vec<crate::indexed_db::types::ObjectStoreIndex>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ObjectStore {
                    name: fields.name,
                    key_path: fields.key_path,
                    auto_increment: fields.auto_increment,
                    indexes: fields.indexes,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IndexedDB.type.ObjectStoreIndex。
///
///协议原文：
///
///<pre>Object store index.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ObjectStoreIndex {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.ObjectStoreIndex/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Index name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.ObjectStoreIndex/properties/keyPath。
    ///
    ///协议原文：
    ///
    ///<pre>Index key path.</pre>
    ///
    ///
    #[serde(rename = "keyPath")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub key_path: crate::indexed_db::types::KeyPath,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.ObjectStoreIndex/properties/unique。
    ///
    ///协议原文：
    ///
    ///<pre>If true, index is unique.</pre>
    ///
    ///
    #[serde(rename = "unique")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub unique: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.ObjectStoreIndex/properties/multiEntry。
    ///
    ///协议原文：
    ///
    ///<pre>If true, index allows multiple entries for a key.</pre>
    ///
    ///
    #[serde(rename = "multiEntry")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub multi_entry: bool,
}
impl<'de> serde::Deserialize<'de> for ObjectStoreIndex {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ObjectStoreIndex;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.ObjectStoreIndex/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Index name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.ObjectStoreIndex/properties/keyPath。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Index key path.</pre>
                    ///
                    ///
                    #[serde(rename = "keyPath")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub key_path: crate::indexed_db::types::KeyPath,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.ObjectStoreIndex/properties/unique。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, index is unique.</pre>
                    ///
                    ///
                    #[serde(rename = "unique")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub unique: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.type.ObjectStoreIndex/properties/multiEntry。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, index allows multiple entries for a key.</pre>
                    ///
                    ///
                    #[serde(rename = "multiEntry")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub multi_entry: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ObjectStoreIndex {
                    name: fields.name,
                    key_path: fields.key_path,
                    unique: fields.unique,
                    multi_entry: fields.multi_entry,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
