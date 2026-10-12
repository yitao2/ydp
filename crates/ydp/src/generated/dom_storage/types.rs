///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：DOMStorage.type.Item。
///
///协议原文：
///
///<pre>DOM Storage item.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Item(Vec<String>);
impl Item {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::dom_storage::types::Item = serde_json::from_str("[]").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::dom_storage::types::Item::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("[]").unwrap());
    ///```
    pub fn new(value: Vec<String>) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::dom_storage::types::Item = serde_json::from_str("[]").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::dom_storage::types::Item::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("[]").unwrap());
    ///```
    pub fn into_inner(self) -> Vec<String> {
        self.0
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：DOMStorage.type.SerializedStorageKey。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SerializedStorageKey(String);
impl SerializedStorageKey {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::dom_storage::types::SerializedStorageKey = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::dom_storage::types::SerializedStorageKey::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::dom_storage::types::SerializedStorageKey = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::dom_storage::types::SerializedStorageKey::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::dom_storage::types::SerializedStorageKey::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMStorage.type.StorageId。
///
///协议原文：
///
///<pre>DOM Storage identifier.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct StorageId {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMStorage.type.StorageId/properties/securityOrigin。
    ///
    ///协议原文：
    ///
    ///<pre>Security origin for the storage.</pre>
    ///
    ///
    #[serde(rename = "securityOrigin")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub security_origin: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMStorage.type.StorageId/properties/storageKey。
    ///
    ///协议原文：
    ///
    ///<pre>Represents a key by which DOM Storage keys its CachedStorageAreas</pre>
    ///
    ///
    #[serde(rename = "storageKey")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_key: Option<crate::dom_storage::types::SerializedStorageKey>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.type.StorageId/properties/isLocalStorage。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the storage is local storage (not session storage).</pre>
    ///
    ///
    #[serde(rename = "isLocalStorage")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub is_local_storage: bool,
}
impl<'de> serde::Deserialize<'de> for StorageId {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StorageId;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMStorage.type.StorageId/properties/securityOrigin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Security origin for the storage.</pre>
                    ///
                    ///
                    #[serde(rename = "securityOrigin")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub security_origin: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMStorage.type.StorageId/properties/storageKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Represents a key by which DOM Storage keys its CachedStorageAreas</pre>
                    ///
                    ///
                    #[serde(rename = "storageKey")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub storage_key: Option<crate::dom_storage::types::SerializedStorageKey>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.type.StorageId/properties/isLocalStorage。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the storage is local storage (not session storage).</pre>
                    ///
                    ///
                    #[serde(rename = "isLocalStorage")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub is_local_storage: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StorageId {
                    security_origin: fields.security_origin,
                    storage_key: fields.storage_key,
                    is_local_storage: fields.is_local_storage,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
