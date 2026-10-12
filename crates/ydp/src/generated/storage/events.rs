///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.event.cacheStorageContentUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CacheStorageContentUpdatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.cacheStorageContentUpdated/parameters/origin。
    ///
    ///协议原文：
    ///
    ///<pre>Origin to update.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.cacheStorageContentUpdated/parameters/storageKey。
    ///
    ///协议原文：
    ///
    ///<pre>Storage key to update.</pre>
    ///
    ///
    #[serde(rename = "storageKey")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_key: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.cacheStorageContentUpdated/parameters/bucketId。
    ///
    ///协议原文：
    ///
    ///<pre>Storage bucket to update.</pre>
    ///
    ///
    #[serde(rename = "bucketId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub bucket_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.cacheStorageContentUpdated/parameters/cacheName。
    ///
    ///协议原文：
    ///
    ///<pre>Name of cache in origin.</pre>
    ///
    ///
    #[serde(rename = "cacheName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cache_name: String,
}
impl<'de> serde::Deserialize<'de> for CacheStorageContentUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CacheStorageContentUpdatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.cacheStorageContentUpdated/parameters/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Origin to update.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.cacheStorageContentUpdated/parameters/storageKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage key to update.</pre>
                    ///
                    ///
                    #[serde(rename = "storageKey")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_key: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.cacheStorageContentUpdated/parameters/bucketId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage bucket to update.</pre>
                    ///
                    ///
                    #[serde(rename = "bucketId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub bucket_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.cacheStorageContentUpdated/parameters/cacheName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Name of cache in origin.</pre>
                    ///
                    ///
                    #[serde(rename = "cacheName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cache_name: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CacheStorageContentUpdatedParams {
                    origin: fields.origin,
                    storage_key: fields.storage_key,
                    bucket_id: fields.bucket_id,
                    cache_name: fields.cache_name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.event.cacheStorageContentUpdated。
///
///协议原文：
///
///<pre>A cache's contents have been modified.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct CacheStorageContentUpdated;
impl ydp_core::Event for CacheStorageContentUpdated {
    type Params = CacheStorageContentUpdatedParams;
    const METHOD: &'static str = "Storage.cacheStorageContentUpdated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.event.cacheStorageListUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CacheStorageListUpdatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.cacheStorageListUpdated/parameters/origin。
    ///
    ///协议原文：
    ///
    ///<pre>Origin to update.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.cacheStorageListUpdated/parameters/storageKey。
    ///
    ///协议原文：
    ///
    ///<pre>Storage key to update.</pre>
    ///
    ///
    #[serde(rename = "storageKey")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_key: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.cacheStorageListUpdated/parameters/bucketId。
    ///
    ///协议原文：
    ///
    ///<pre>Storage bucket to update.</pre>
    ///
    ///
    #[serde(rename = "bucketId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub bucket_id: String,
}
impl<'de> serde::Deserialize<'de> for CacheStorageListUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CacheStorageListUpdatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.cacheStorageListUpdated/parameters/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Origin to update.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.cacheStorageListUpdated/parameters/storageKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage key to update.</pre>
                    ///
                    ///
                    #[serde(rename = "storageKey")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_key: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.cacheStorageListUpdated/parameters/bucketId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage bucket to update.</pre>
                    ///
                    ///
                    #[serde(rename = "bucketId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub bucket_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CacheStorageListUpdatedParams {
                    origin: fields.origin,
                    storage_key: fields.storage_key,
                    bucket_id: fields.bucket_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.event.cacheStorageListUpdated。
///
///协议原文：
///
///<pre>A cache has been added/deleted.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct CacheStorageListUpdated;
impl ydp_core::Event for CacheStorageListUpdated {
    type Params = CacheStorageListUpdatedParams;
    const METHOD: &'static str = "Storage.cacheStorageListUpdated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.event.indexedDBContentUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct IndexedDbContentUpdatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.indexedDBContentUpdated/parameters/origin。
    ///
    ///协议原文：
    ///
    ///<pre>Origin to update.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.indexedDBContentUpdated/parameters/storageKey。
    ///
    ///协议原文：
    ///
    ///<pre>Storage key to update.</pre>
    ///
    ///
    #[serde(rename = "storageKey")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_key: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.indexedDBContentUpdated/parameters/bucketId。
    ///
    ///协议原文：
    ///
    ///<pre>Storage bucket to update.</pre>
    ///
    ///
    #[serde(rename = "bucketId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub bucket_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.indexedDBContentUpdated/parameters/databaseName。
    ///
    ///协议原文：
    ///
    ///<pre>Database to update.</pre>
    ///
    ///
    #[serde(rename = "databaseName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub database_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.indexedDBContentUpdated/parameters/objectStoreName。
    ///
    ///协议原文：
    ///
    ///<pre>ObjectStore to update.</pre>
    ///
    ///
    #[serde(rename = "objectStoreName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub object_store_name: String,
}
impl<'de> serde::Deserialize<'de> for IndexedDbContentUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = IndexedDbContentUpdatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.indexedDBContentUpdated/parameters/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Origin to update.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.indexedDBContentUpdated/parameters/storageKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage key to update.</pre>
                    ///
                    ///
                    #[serde(rename = "storageKey")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_key: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.indexedDBContentUpdated/parameters/bucketId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage bucket to update.</pre>
                    ///
                    ///
                    #[serde(rename = "bucketId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub bucket_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.indexedDBContentUpdated/parameters/databaseName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Database to update.</pre>
                    ///
                    ///
                    #[serde(rename = "databaseName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub database_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.indexedDBContentUpdated/parameters/objectStoreName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>ObjectStore to update.</pre>
                    ///
                    ///
                    #[serde(rename = "objectStoreName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub object_store_name: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(IndexedDbContentUpdatedParams {
                    origin: fields.origin,
                    storage_key: fields.storage_key,
                    bucket_id: fields.bucket_id,
                    database_name: fields.database_name,
                    object_store_name: fields.object_store_name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.event.indexedDBContentUpdated。
///
///协议原文：
///
///<pre>The origin's IndexedDB object store has been modified.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct IndexedDbContentUpdated;
impl ydp_core::Event for IndexedDbContentUpdated {
    type Params = IndexedDbContentUpdatedParams;
    const METHOD: &'static str = "Storage.indexedDBContentUpdated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.event.indexedDBListUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct IndexedDbListUpdatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.indexedDBListUpdated/parameters/origin。
    ///
    ///协议原文：
    ///
    ///<pre>Origin to update.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.indexedDBListUpdated/parameters/storageKey。
    ///
    ///协议原文：
    ///
    ///<pre>Storage key to update.</pre>
    ///
    ///
    #[serde(rename = "storageKey")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_key: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.indexedDBListUpdated/parameters/bucketId。
    ///
    ///协议原文：
    ///
    ///<pre>Storage bucket to update.</pre>
    ///
    ///
    #[serde(rename = "bucketId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub bucket_id: String,
}
impl<'de> serde::Deserialize<'de> for IndexedDbListUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = IndexedDbListUpdatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.indexedDBListUpdated/parameters/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Origin to update.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.indexedDBListUpdated/parameters/storageKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage key to update.</pre>
                    ///
                    ///
                    #[serde(rename = "storageKey")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_key: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.indexedDBListUpdated/parameters/bucketId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage bucket to update.</pre>
                    ///
                    ///
                    #[serde(rename = "bucketId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub bucket_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(IndexedDbListUpdatedParams {
                    origin: fields.origin,
                    storage_key: fields.storage_key,
                    bucket_id: fields.bucket_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.event.indexedDBListUpdated。
///
///协议原文：
///
///<pre>The origin's IndexedDB database list has been modified.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct IndexedDbListUpdated;
impl ydp_core::Event for IndexedDbListUpdated {
    type Params = IndexedDbListUpdatedParams;
    const METHOD: &'static str = "Storage.indexedDBListUpdated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.event.storageBucketCreatedOrUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StorageBucketCreatedOrUpdatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.storageBucketCreatedOrUpdated/parameters/bucketInfo。
    #[serde(rename = "bucketInfo")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub bucket_info: crate::storage::types::StorageBucketInfo,
}
impl<'de> serde::Deserialize<'de> for StorageBucketCreatedOrUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StorageBucketCreatedOrUpdatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.storageBucketCreatedOrUpdated/parameters/bucketInfo。
                    #[serde(rename = "bucketInfo")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub bucket_info: crate::storage::types::StorageBucketInfo,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StorageBucketCreatedOrUpdatedParams {
                    bucket_info: fields.bucket_info,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.event.storageBucketCreatedOrUpdated。
#[derive(Debug, Clone, Copy)]
pub struct StorageBucketCreatedOrUpdated;
impl ydp_core::Event for StorageBucketCreatedOrUpdated {
    type Params = StorageBucketCreatedOrUpdatedParams;
    const METHOD: &'static str = "Storage.storageBucketCreatedOrUpdated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.event.storageBucketDeleted/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StorageBucketDeletedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.storageBucketDeleted/parameters/bucketId。
    #[serde(rename = "bucketId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub bucket_id: String,
}
impl<'de> serde::Deserialize<'de> for StorageBucketDeletedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StorageBucketDeletedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.event.storageBucketDeleted/parameters/bucketId。
                    #[serde(rename = "bucketId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub bucket_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StorageBucketDeletedParams {
                    bucket_id: fields.bucket_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.event.storageBucketDeleted。
#[derive(Debug, Clone, Copy)]
pub struct StorageBucketDeleted;
impl ydp_core::Event for StorageBucketDeleted {
    type Params = StorageBucketDeletedParams;
    const METHOD: &'static str = "Storage.storageBucketDeleted";
}
