///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IndexedDB.command.clearObjectStore/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearObjectStoreParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.clearObjectStore/parameters/securityOrigin。
    ///
    ///协议原文：
    ///
    ///<pre>At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    ///Security origin.</pre>
    ///
    ///
    #[serde(rename = "securityOrigin")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub security_origin: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.clearObjectStore/parameters/storageKey。
    ///
    ///协议原文：
    ///
    ///<pre>Storage key.</pre>
    ///
    ///
    #[serde(rename = "storageKey")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_key: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.clearObjectStore/parameters/storageBucket。
    ///
    ///协议原文：
    ///
    ///<pre>Storage bucket. If not specified, it uses the default bucket.</pre>
    ///
    ///
    #[serde(rename = "storageBucket")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_bucket: Option<crate::storage::types::StorageBucket>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.clearObjectStore/parameters/databaseName。
    ///
    ///协议原文：
    ///
    ///<pre>Database name.</pre>
    ///
    ///
    #[serde(rename = "databaseName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub database_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.clearObjectStore/parameters/objectStoreName。
    ///
    ///协议原文：
    ///
    ///<pre>Object store name.</pre>
    ///
    ///
    #[serde(rename = "objectStoreName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub object_store_name: String,
}
impl<'de> serde::Deserialize<'de> for ClearObjectStoreParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearObjectStoreParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.clearObjectStore/parameters/securityOrigin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
                    ///Security origin.</pre>
                    ///
                    ///
                    #[serde(rename = "securityOrigin")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub security_origin: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.clearObjectStore/parameters/storageKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage key.</pre>
                    ///
                    ///
                    #[serde(rename = "storageKey")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub storage_key: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.clearObjectStore/parameters/storageBucket。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage bucket. If not specified, it uses the default bucket.</pre>
                    ///
                    ///
                    #[serde(rename = "storageBucket")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub storage_bucket: Option<crate::storage::types::StorageBucket>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.clearObjectStore/parameters/databaseName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Database name.</pre>
                    ///
                    ///
                    #[serde(rename = "databaseName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub database_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.clearObjectStore/parameters/objectStoreName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Object store name.</pre>
                    ///
                    ///
                    #[serde(rename = "objectStoreName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub object_store_name: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ClearObjectStoreParams {
                    security_origin: fields.security_origin,
                    storage_key: fields.storage_key,
                    storage_bucket: fields.storage_bucket,
                    database_name: fields.database_name,
                    object_store_name: fields.object_store_name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：IndexedDB.command.clearObjectStore。
///
///协议原文：
///
///<pre>Clears all entries from an object store.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ClearObjectStore;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IndexedDB.command.clearObjectStore/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearObjectStoreResponse {}
impl<'de> serde::Deserialize<'de> for ClearObjectStoreResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearObjectStoreResponse;
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
                Ok(ClearObjectStoreResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ClearObjectStore {
    type Params = ClearObjectStoreParams;
    type Response = ClearObjectStoreResponse;
    const METHOD: &'static str = "IndexedDB.clearObjectStore";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IndexedDB.command.deleteDatabase/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeleteDatabaseParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.deleteDatabase/parameters/securityOrigin。
    ///
    ///协议原文：
    ///
    ///<pre>At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    ///Security origin.</pre>
    ///
    ///
    #[serde(rename = "securityOrigin")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub security_origin: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.deleteDatabase/parameters/storageKey。
    ///
    ///协议原文：
    ///
    ///<pre>Storage key.</pre>
    ///
    ///
    #[serde(rename = "storageKey")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_key: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.deleteDatabase/parameters/storageBucket。
    ///
    ///协议原文：
    ///
    ///<pre>Storage bucket. If not specified, it uses the default bucket.</pre>
    ///
    ///
    #[serde(rename = "storageBucket")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_bucket: Option<crate::storage::types::StorageBucket>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.deleteDatabase/parameters/databaseName。
    ///
    ///协议原文：
    ///
    ///<pre>Database name.</pre>
    ///
    ///
    #[serde(rename = "databaseName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub database_name: String,
}
impl<'de> serde::Deserialize<'de> for DeleteDatabaseParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeleteDatabaseParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.deleteDatabase/parameters/securityOrigin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
                    ///Security origin.</pre>
                    ///
                    ///
                    #[serde(rename = "securityOrigin")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub security_origin: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.deleteDatabase/parameters/storageKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage key.</pre>
                    ///
                    ///
                    #[serde(rename = "storageKey")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub storage_key: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.deleteDatabase/parameters/storageBucket。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage bucket. If not specified, it uses the default bucket.</pre>
                    ///
                    ///
                    #[serde(rename = "storageBucket")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub storage_bucket: Option<crate::storage::types::StorageBucket>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.deleteDatabase/parameters/databaseName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Database name.</pre>
                    ///
                    ///
                    #[serde(rename = "databaseName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub database_name: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DeleteDatabaseParams {
                    security_origin: fields.security_origin,
                    storage_key: fields.storage_key,
                    storage_bucket: fields.storage_bucket,
                    database_name: fields.database_name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：IndexedDB.command.deleteDatabase。
///
///协议原文：
///
///<pre>Deletes a database.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct DeleteDatabase;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IndexedDB.command.deleteDatabase/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeleteDatabaseResponse {}
impl<'de> serde::Deserialize<'de> for DeleteDatabaseResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeleteDatabaseResponse;
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
                Ok(DeleteDatabaseResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for DeleteDatabase {
    type Params = DeleteDatabaseParams;
    type Response = DeleteDatabaseResponse;
    const METHOD: &'static str = "IndexedDB.deleteDatabase";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IndexedDB.command.deleteObjectStoreEntries/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeleteObjectStoreEntriesParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.deleteObjectStoreEntries/parameters/securityOrigin。
    ///
    ///协议原文：
    ///
    ///<pre>At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    ///Security origin.</pre>
    ///
    ///
    #[serde(rename = "securityOrigin")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub security_origin: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.deleteObjectStoreEntries/parameters/storageKey。
    ///
    ///协议原文：
    ///
    ///<pre>Storage key.</pre>
    ///
    ///
    #[serde(rename = "storageKey")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_key: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.deleteObjectStoreEntries/parameters/storageBucket。
    ///
    ///协议原文：
    ///
    ///<pre>Storage bucket. If not specified, it uses the default bucket.</pre>
    ///
    ///
    #[serde(rename = "storageBucket")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_bucket: Option<crate::storage::types::StorageBucket>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.deleteObjectStoreEntries/parameters/databaseName。
    #[serde(rename = "databaseName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub database_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.deleteObjectStoreEntries/parameters/objectStoreName。
    #[serde(rename = "objectStoreName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub object_store_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.deleteObjectStoreEntries/parameters/keyRange。
    ///
    ///协议原文：
    ///
    ///<pre>Range of entry keys to delete</pre>
    ///
    ///
    #[serde(rename = "keyRange")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub key_range: crate::indexed_db::types::KeyRange,
}
impl<'de> serde::Deserialize<'de> for DeleteObjectStoreEntriesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeleteObjectStoreEntriesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.deleteObjectStoreEntries/parameters/securityOrigin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
                    ///Security origin.</pre>
                    ///
                    ///
                    #[serde(rename = "securityOrigin")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub security_origin: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.deleteObjectStoreEntries/parameters/storageKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage key.</pre>
                    ///
                    ///
                    #[serde(rename = "storageKey")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub storage_key: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.deleteObjectStoreEntries/parameters/storageBucket。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage bucket. If not specified, it uses the default bucket.</pre>
                    ///
                    ///
                    #[serde(rename = "storageBucket")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub storage_bucket: Option<crate::storage::types::StorageBucket>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.deleteObjectStoreEntries/parameters/databaseName。
                    #[serde(rename = "databaseName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub database_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.deleteObjectStoreEntries/parameters/objectStoreName。
                    #[serde(rename = "objectStoreName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub object_store_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.deleteObjectStoreEntries/parameters/keyRange。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Range of entry keys to delete</pre>
                    ///
                    ///
                    #[serde(rename = "keyRange")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub key_range: crate::indexed_db::types::KeyRange,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DeleteObjectStoreEntriesParams {
                    security_origin: fields.security_origin,
                    storage_key: fields.storage_key,
                    storage_bucket: fields.storage_bucket,
                    database_name: fields.database_name,
                    object_store_name: fields.object_store_name,
                    key_range: fields.key_range,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：IndexedDB.command.deleteObjectStoreEntries。
///
///协议原文：
///
///<pre>Delete a range of entries from an object store</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct DeleteObjectStoreEntries;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IndexedDB.command.deleteObjectStoreEntries/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeleteObjectStoreEntriesResponse {}
impl<'de> serde::Deserialize<'de> for DeleteObjectStoreEntriesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeleteObjectStoreEntriesResponse;
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
                Ok(DeleteObjectStoreEntriesResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for DeleteObjectStoreEntries {
    type Params = DeleteObjectStoreEntriesParams;
    type Response = DeleteObjectStoreEntriesResponse;
    const METHOD: &'static str = "IndexedDB.deleteObjectStoreEntries";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IndexedDB.command.disable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：IndexedDB.command.disable。
///
///协议原文：
///
///<pre>Disables events from backend.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IndexedDB.command.disable/returns。
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
    const METHOD: &'static str = "IndexedDB.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IndexedDB.command.enable/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct EnableParams {}
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
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EnableParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：IndexedDB.command.enable。
///
///协议原文：
///
///<pre>Enables events from backend.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IndexedDB.command.enable/returns。
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
    const METHOD: &'static str = "IndexedDB.enable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IndexedDB.command.getMetadata/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetMetadataParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.getMetadata/parameters/securityOrigin。
    ///
    ///协议原文：
    ///
    ///<pre>At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    ///Security origin.</pre>
    ///
    ///
    #[serde(rename = "securityOrigin")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub security_origin: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.getMetadata/parameters/storageKey。
    ///
    ///协议原文：
    ///
    ///<pre>Storage key.</pre>
    ///
    ///
    #[serde(rename = "storageKey")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_key: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.getMetadata/parameters/storageBucket。
    ///
    ///协议原文：
    ///
    ///<pre>Storage bucket. If not specified, it uses the default bucket.</pre>
    ///
    ///
    #[serde(rename = "storageBucket")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_bucket: Option<crate::storage::types::StorageBucket>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.getMetadata/parameters/databaseName。
    ///
    ///协议原文：
    ///
    ///<pre>Database name.</pre>
    ///
    ///
    #[serde(rename = "databaseName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub database_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.getMetadata/parameters/objectStoreName。
    ///
    ///协议原文：
    ///
    ///<pre>Object store name.</pre>
    ///
    ///
    #[serde(rename = "objectStoreName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub object_store_name: String,
}
impl<'de> serde::Deserialize<'de> for GetMetadataParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetMetadataParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.getMetadata/parameters/securityOrigin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
                    ///Security origin.</pre>
                    ///
                    ///
                    #[serde(rename = "securityOrigin")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub security_origin: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.getMetadata/parameters/storageKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage key.</pre>
                    ///
                    ///
                    #[serde(rename = "storageKey")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub storage_key: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.getMetadata/parameters/storageBucket。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage bucket. If not specified, it uses the default bucket.</pre>
                    ///
                    ///
                    #[serde(rename = "storageBucket")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub storage_bucket: Option<crate::storage::types::StorageBucket>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.getMetadata/parameters/databaseName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Database name.</pre>
                    ///
                    ///
                    #[serde(rename = "databaseName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub database_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.getMetadata/parameters/objectStoreName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Object store name.</pre>
                    ///
                    ///
                    #[serde(rename = "objectStoreName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub object_store_name: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetMetadataParams {
                    security_origin: fields.security_origin,
                    storage_key: fields.storage_key,
                    storage_bucket: fields.storage_bucket,
                    database_name: fields.database_name,
                    object_store_name: fields.object_store_name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：IndexedDB.command.getMetadata。
///
///协议原文：
///
///<pre>Gets metadata of an object store.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetMetadata;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IndexedDB.command.getMetadata/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetMetadataResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.getMetadata/returns/entriesCount。
    ///
    ///协议原文：
    ///
    ///<pre>the entries count</pre>
    ///
    ///
    #[serde(rename = "entriesCount")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub entries_count: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.getMetadata/returns/keyGeneratorValue。
    ///
    ///协议原文：
    ///
    ///<pre>the current value of key generator, to become the next inserted
    ///key into the object store. Valid if objectStore.autoIncrement
    ///is true.</pre>
    ///
    ///
    #[serde(rename = "keyGeneratorValue")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub key_generator_value: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for GetMetadataResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetMetadataResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.getMetadata/returns/entriesCount。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>the entries count</pre>
                    ///
                    ///
                    #[serde(rename = "entriesCount")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub entries_count: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.getMetadata/returns/keyGeneratorValue。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>the current value of key generator, to become the next inserted
                    ///key into the object store. Valid if objectStore.autoIncrement
                    ///is true.</pre>
                    ///
                    ///
                    #[serde(rename = "keyGeneratorValue")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub key_generator_value: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetMetadataResponse {
                    entries_count: fields.entries_count,
                    key_generator_value: fields.key_generator_value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetMetadata {
    type Params = GetMetadataParams;
    type Response = GetMetadataResponse;
    const METHOD: &'static str = "IndexedDB.getMetadata";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IndexedDB.command.requestData/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequestDataParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.requestData/parameters/securityOrigin。
    ///
    ///协议原文：
    ///
    ///<pre>At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    ///Security origin.</pre>
    ///
    ///
    #[serde(rename = "securityOrigin")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub security_origin: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.requestData/parameters/storageKey。
    ///
    ///协议原文：
    ///
    ///<pre>Storage key.</pre>
    ///
    ///
    #[serde(rename = "storageKey")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_key: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.requestData/parameters/storageBucket。
    ///
    ///协议原文：
    ///
    ///<pre>Storage bucket. If not specified, it uses the default bucket.</pre>
    ///
    ///
    #[serde(rename = "storageBucket")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_bucket: Option<crate::storage::types::StorageBucket>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.requestData/parameters/databaseName。
    ///
    ///协议原文：
    ///
    ///<pre>Database name.</pre>
    ///
    ///
    #[serde(rename = "databaseName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub database_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.requestData/parameters/objectStoreName。
    ///
    ///协议原文：
    ///
    ///<pre>Object store name.</pre>
    ///
    ///
    #[serde(rename = "objectStoreName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub object_store_name: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.requestData/parameters/indexName。
    ///
    ///协议原文：
    ///
    ///<pre>Index name. If not specified, it performs an object store data request.</pre>
    ///
    ///
    #[serde(rename = "indexName")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub index_name: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.requestData/parameters/skipCount。
    ///
    ///协议原文：
    ///
    ///<pre>Number of records to skip.</pre>
    ///
    ///
    #[serde(rename = "skipCount")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub skip_count: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.requestData/parameters/pageSize。
    ///
    ///协议原文：
    ///
    ///<pre>Number of records to fetch.</pre>
    ///
    ///
    #[serde(rename = "pageSize")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub page_size: i64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.requestData/parameters/keyRange。
    ///
    ///协议原文：
    ///
    ///<pre>Key range.</pre>
    ///
    ///
    #[serde(rename = "keyRange")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub key_range: Option<crate::indexed_db::types::KeyRange>,
}
impl<'de> serde::Deserialize<'de> for RequestDataParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RequestDataParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.requestData/parameters/securityOrigin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
                    ///Security origin.</pre>
                    ///
                    ///
                    #[serde(rename = "securityOrigin")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub security_origin: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.requestData/parameters/storageKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage key.</pre>
                    ///
                    ///
                    #[serde(rename = "storageKey")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub storage_key: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.requestData/parameters/storageBucket。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage bucket. If not specified, it uses the default bucket.</pre>
                    ///
                    ///
                    #[serde(rename = "storageBucket")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub storage_bucket: Option<crate::storage::types::StorageBucket>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.requestData/parameters/databaseName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Database name.</pre>
                    ///
                    ///
                    #[serde(rename = "databaseName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub database_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.requestData/parameters/objectStoreName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Object store name.</pre>
                    ///
                    ///
                    #[serde(rename = "objectStoreName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub object_store_name: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.requestData/parameters/indexName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Index name. If not specified, it performs an object store data request.</pre>
                    ///
                    ///
                    #[serde(rename = "indexName")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub index_name: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.requestData/parameters/skipCount。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Number of records to skip.</pre>
                    ///
                    ///
                    #[serde(rename = "skipCount")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub skip_count: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.requestData/parameters/pageSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Number of records to fetch.</pre>
                    ///
                    ///
                    #[serde(rename = "pageSize")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub page_size: i64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.requestData/parameters/keyRange。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Key range.</pre>
                    ///
                    ///
                    #[serde(rename = "keyRange")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub key_range: Option<crate::indexed_db::types::KeyRange>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RequestDataParams {
                    security_origin: fields.security_origin,
                    storage_key: fields.storage_key,
                    storage_bucket: fields.storage_bucket,
                    database_name: fields.database_name,
                    object_store_name: fields.object_store_name,
                    index_name: fields.index_name,
                    skip_count: fields.skip_count,
                    page_size: fields.page_size,
                    key_range: fields.key_range,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：IndexedDB.command.requestData。
///
///协议原文：
///
///<pre>Requests data from object store or index.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RequestData;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IndexedDB.command.requestData/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequestDataResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.requestData/returns/objectStoreDataEntries。
    ///
    ///协议原文：
    ///
    ///<pre>Array of object store data entries.</pre>
    ///
    ///
    #[serde(rename = "objectStoreDataEntries")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub object_store_data_entries: Vec<crate::indexed_db::types::DataEntry>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.requestData/returns/hasMore。
    ///
    ///协议原文：
    ///
    ///<pre>If true, there are more entries to fetch in the given range.</pre>
    ///
    ///
    #[serde(rename = "hasMore")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub has_more: bool,
}
impl<'de> serde::Deserialize<'de> for RequestDataResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RequestDataResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.requestData/returns/objectStoreDataEntries。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Array of object store data entries.</pre>
                    ///
                    ///
                    #[serde(rename = "objectStoreDataEntries")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub object_store_data_entries: Vec<crate::indexed_db::types::DataEntry>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.requestData/returns/hasMore。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, there are more entries to fetch in the given range.</pre>
                    ///
                    ///
                    #[serde(rename = "hasMore")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub has_more: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RequestDataResponse {
                    object_store_data_entries: fields.object_store_data_entries,
                    has_more: fields.has_more,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RequestData {
    type Params = RequestDataParams;
    type Response = RequestDataResponse;
    const METHOD: &'static str = "IndexedDB.requestData";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IndexedDB.command.requestDatabase/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequestDatabaseParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.requestDatabase/parameters/securityOrigin。
    ///
    ///协议原文：
    ///
    ///<pre>At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    ///Security origin.</pre>
    ///
    ///
    #[serde(rename = "securityOrigin")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub security_origin: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.requestDatabase/parameters/storageKey。
    ///
    ///协议原文：
    ///
    ///<pre>Storage key.</pre>
    ///
    ///
    #[serde(rename = "storageKey")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_key: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.requestDatabase/parameters/storageBucket。
    ///
    ///协议原文：
    ///
    ///<pre>Storage bucket. If not specified, it uses the default bucket.</pre>
    ///
    ///
    #[serde(rename = "storageBucket")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_bucket: Option<crate::storage::types::StorageBucket>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.requestDatabase/parameters/databaseName。
    ///
    ///协议原文：
    ///
    ///<pre>Database name.</pre>
    ///
    ///
    #[serde(rename = "databaseName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub database_name: String,
}
impl<'de> serde::Deserialize<'de> for RequestDatabaseParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RequestDatabaseParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.requestDatabase/parameters/securityOrigin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
                    ///Security origin.</pre>
                    ///
                    ///
                    #[serde(rename = "securityOrigin")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub security_origin: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.requestDatabase/parameters/storageKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage key.</pre>
                    ///
                    ///
                    #[serde(rename = "storageKey")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub storage_key: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.requestDatabase/parameters/storageBucket。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage bucket. If not specified, it uses the default bucket.</pre>
                    ///
                    ///
                    #[serde(rename = "storageBucket")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub storage_bucket: Option<crate::storage::types::StorageBucket>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.requestDatabase/parameters/databaseName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Database name.</pre>
                    ///
                    ///
                    #[serde(rename = "databaseName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub database_name: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RequestDatabaseParams {
                    security_origin: fields.security_origin,
                    storage_key: fields.storage_key,
                    storage_bucket: fields.storage_bucket,
                    database_name: fields.database_name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：IndexedDB.command.requestDatabase。
///
///协议原文：
///
///<pre>Requests database with given name in given frame.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RequestDatabase;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IndexedDB.command.requestDatabase/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequestDatabaseResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.requestDatabase/returns/databaseWithObjectStores。
    ///
    ///协议原文：
    ///
    ///<pre>Database with an array of object stores.</pre>
    ///
    ///
    #[serde(rename = "databaseWithObjectStores")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub database_with_object_stores: crate::indexed_db::types::DatabaseWithObjectStores,
}
impl<'de> serde::Deserialize<'de> for RequestDatabaseResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RequestDatabaseResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.requestDatabase/returns/databaseWithObjectStores。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Database with an array of object stores.</pre>
                    ///
                    ///
                    #[serde(rename = "databaseWithObjectStores")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub database_with_object_stores:
                        crate::indexed_db::types::DatabaseWithObjectStores,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RequestDatabaseResponse {
                    database_with_object_stores: fields.database_with_object_stores,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RequestDatabase {
    type Params = RequestDatabaseParams;
    type Response = RequestDatabaseResponse;
    const METHOD: &'static str = "IndexedDB.requestDatabase";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IndexedDB.command.requestDatabaseNames/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct RequestDatabaseNamesParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.requestDatabaseNames/parameters/securityOrigin。
    ///
    ///协议原文：
    ///
    ///<pre>At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
    ///Security origin.</pre>
    ///
    ///
    #[serde(rename = "securityOrigin")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub security_origin: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.requestDatabaseNames/parameters/storageKey。
    ///
    ///协议原文：
    ///
    ///<pre>Storage key.</pre>
    ///
    ///
    #[serde(rename = "storageKey")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_key: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.requestDatabaseNames/parameters/storageBucket。
    ///
    ///协议原文：
    ///
    ///<pre>Storage bucket. If not specified, it uses the default bucket.</pre>
    ///
    ///
    #[serde(rename = "storageBucket")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_bucket: Option<crate::storage::types::StorageBucket>,
}
impl<'de> serde::Deserialize<'de> for RequestDatabaseNamesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RequestDatabaseNamesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.requestDatabaseNames/parameters/securityOrigin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.
                    ///Security origin.</pre>
                    ///
                    ///
                    #[serde(rename = "securityOrigin")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub security_origin: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.requestDatabaseNames/parameters/storageKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage key.</pre>
                    ///
                    ///
                    #[serde(rename = "storageKey")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub storage_key: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IndexedDB.command.requestDatabaseNames/parameters/storageBucket。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage bucket. If not specified, it uses the default bucket.</pre>
                    ///
                    ///
                    #[serde(rename = "storageBucket")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub storage_bucket: Option<crate::storage::types::StorageBucket>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RequestDatabaseNamesParams {
                    security_origin: fields.security_origin,
                    storage_key: fields.storage_key,
                    storage_bucket: fields.storage_bucket,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：IndexedDB.command.requestDatabaseNames。
///
///协议原文：
///
///<pre>Requests database names for given security origin.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RequestDatabaseNames;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IndexedDB.command.requestDatabaseNames/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequestDatabaseNamesResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.requestDatabaseNames/returns/databaseNames。
    ///
    ///协议原文：
    ///
    ///<pre>Database names for origin.</pre>
    ///
    ///
    #[serde(rename = "databaseNames")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub database_names: Vec<String>,
}
impl<'de> serde::Deserialize<'de> for RequestDatabaseNamesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RequestDatabaseNamesResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IndexedDB.command.requestDatabaseNames/returns/databaseNames。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Database names for origin.</pre>
                    ///
                    ///
                    #[serde(rename = "databaseNames")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub database_names: Vec<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RequestDatabaseNamesResponse {
                    database_names: fields.database_names,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RequestDatabaseNames {
    type Params = RequestDatabaseNamesParams;
    type Response = RequestDatabaseNamesResponse;
    const METHOD: &'static str = "IndexedDB.requestDatabaseNames";
}
