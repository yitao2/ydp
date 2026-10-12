///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CacheStorage.command.deleteCache/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeleteCacheParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.command.deleteCache/parameters/cacheId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of cache for deletion.</pre>
    ///
    ///
    #[serde(rename = "cacheId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cache_id: crate::cache_storage::types::CacheId,
}
impl<'de> serde::Deserialize<'de> for DeleteCacheParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeleteCacheParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.command.deleteCache/parameters/cacheId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of cache for deletion.</pre>
                    ///
                    ///
                    #[serde(rename = "cacheId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cache_id: crate::cache_storage::types::CacheId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DeleteCacheParams {
                    cache_id: fields.cache_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CacheStorage.command.deleteCache。
///
///协议原文：
///
///<pre>Deletes a cache.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct DeleteCache;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CacheStorage.command.deleteCache/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeleteCacheResponse {}
impl<'de> serde::Deserialize<'de> for DeleteCacheResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeleteCacheResponse;
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
                Ok(DeleteCacheResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for DeleteCache {
    type Params = DeleteCacheParams;
    type Response = DeleteCacheResponse;
    const METHOD: &'static str = "CacheStorage.deleteCache";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CacheStorage.command.deleteEntry/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeleteEntryParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.command.deleteEntry/parameters/cacheId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of cache where the entry will be deleted.</pre>
    ///
    ///
    #[serde(rename = "cacheId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cache_id: crate::cache_storage::types::CacheId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.command.deleteEntry/parameters/request。
    ///
    ///协议原文：
    ///
    ///<pre>URL spec of the request.</pre>
    ///
    ///
    #[serde(rename = "request")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request: String,
}
impl<'de> serde::Deserialize<'de> for DeleteEntryParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeleteEntryParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.command.deleteEntry/parameters/cacheId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of cache where the entry will be deleted.</pre>
                    ///
                    ///
                    #[serde(rename = "cacheId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cache_id: crate::cache_storage::types::CacheId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.command.deleteEntry/parameters/request。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>URL spec of the request.</pre>
                    ///
                    ///
                    #[serde(rename = "request")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DeleteEntryParams {
                    cache_id: fields.cache_id,
                    request: fields.request,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CacheStorage.command.deleteEntry。
///
///协议原文：
///
///<pre>Deletes a cache entry.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct DeleteEntry;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CacheStorage.command.deleteEntry/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeleteEntryResponse {}
impl<'de> serde::Deserialize<'de> for DeleteEntryResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeleteEntryResponse;
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
                Ok(DeleteEntryResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for DeleteEntry {
    type Params = DeleteEntryParams;
    type Response = DeleteEntryResponse;
    const METHOD: &'static str = "CacheStorage.deleteEntry";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CacheStorage.command.requestCacheNames/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct RequestCacheNamesParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CacheStorage.command.requestCacheNames/parameters/securityOrigin。
    ///
    ///协议原文：
    ///
    ///<pre>At least and at most one of securityOrigin, storageKey, storageBucket must be specified.
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
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CacheStorage.command.requestCacheNames/parameters/storageKey。
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
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CacheStorage.command.requestCacheNames/parameters/storageBucket。
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
impl<'de> serde::Deserialize<'de> for RequestCacheNamesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RequestCacheNamesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CacheStorage.command.requestCacheNames/parameters/securityOrigin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>At least and at most one of securityOrigin, storageKey, storageBucket must be specified.
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
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CacheStorage.command.requestCacheNames/parameters/storageKey。
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
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CacheStorage.command.requestCacheNames/parameters/storageBucket。
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
                Ok(RequestCacheNamesParams {
                    security_origin: fields.security_origin,
                    storage_key: fields.storage_key,
                    storage_bucket: fields.storage_bucket,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CacheStorage.command.requestCacheNames。
///
///协议原文：
///
///<pre>Requests cache names.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RequestCacheNames;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CacheStorage.command.requestCacheNames/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequestCacheNamesResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.command.requestCacheNames/returns/caches。
    ///
    ///协议原文：
    ///
    ///<pre>Caches for the security origin.</pre>
    ///
    ///
    #[serde(rename = "caches")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub caches: Vec<crate::cache_storage::types::Cache>,
}
impl<'de> serde::Deserialize<'de> for RequestCacheNamesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RequestCacheNamesResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.command.requestCacheNames/returns/caches。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Caches for the security origin.</pre>
                    ///
                    ///
                    #[serde(rename = "caches")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub caches: Vec<crate::cache_storage::types::Cache>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RequestCacheNamesResponse {
                    caches: fields.caches,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RequestCacheNames {
    type Params = RequestCacheNamesParams;
    type Response = RequestCacheNamesResponse;
    const METHOD: &'static str = "CacheStorage.requestCacheNames";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CacheStorage.command.requestCachedResponse/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequestCachedResponseParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.command.requestCachedResponse/parameters/cacheId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of cache that contains the entry.</pre>
    ///
    ///
    #[serde(rename = "cacheId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cache_id: crate::cache_storage::types::CacheId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.command.requestCachedResponse/parameters/requestURL。
    ///
    ///协议原文：
    ///
    ///<pre>URL spec of the request.</pre>
    ///
    ///
    #[serde(rename = "requestURL")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.command.requestCachedResponse/parameters/requestHeaders。
    ///
    ///协议原文：
    ///
    ///<pre>headers of the request.</pre>
    ///
    ///
    #[serde(rename = "requestHeaders")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_headers: Vec<crate::cache_storage::types::Header>,
}
impl<'de> serde::Deserialize<'de> for RequestCachedResponseParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RequestCachedResponseParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.command.requestCachedResponse/parameters/cacheId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of cache that contains the entry.</pre>
                    ///
                    ///
                    #[serde(rename = "cacheId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cache_id: crate::cache_storage::types::CacheId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.command.requestCachedResponse/parameters/requestURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>URL spec of the request.</pre>
                    ///
                    ///
                    #[serde(rename = "requestURL")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.command.requestCachedResponse/parameters/requestHeaders。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>headers of the request.</pre>
                    ///
                    ///
                    #[serde(rename = "requestHeaders")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_headers: Vec<crate::cache_storage::types::Header>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RequestCachedResponseParams {
                    cache_id: fields.cache_id,
                    request_url: fields.request_url,
                    request_headers: fields.request_headers,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CacheStorage.command.requestCachedResponse。
///
///协议原文：
///
///<pre>Fetches cache entry.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RequestCachedResponse;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CacheStorage.command.requestCachedResponse/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequestCachedResponseResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.command.requestCachedResponse/returns/response。
    ///
    ///协议原文：
    ///
    ///<pre>Response read from the cache.</pre>
    ///
    ///
    #[serde(rename = "response")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub response: crate::cache_storage::types::CachedResponse,
}
impl<'de> serde::Deserialize<'de> for RequestCachedResponseResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RequestCachedResponseResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.command.requestCachedResponse/returns/response。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Response read from the cache.</pre>
                    ///
                    ///
                    #[serde(rename = "response")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub response: crate::cache_storage::types::CachedResponse,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RequestCachedResponseResponse {
                    response: fields.response,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RequestCachedResponse {
    type Params = RequestCachedResponseParams;
    type Response = RequestCachedResponseResponse;
    const METHOD: &'static str = "CacheStorage.requestCachedResponse";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CacheStorage.command.requestEntries/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequestEntriesParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.command.requestEntries/parameters/cacheId。
    ///
    ///协议原文：
    ///
    ///<pre>ID of cache to get entries from.</pre>
    ///
    ///
    #[serde(rename = "cacheId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cache_id: crate::cache_storage::types::CacheId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CacheStorage.command.requestEntries/parameters/skipCount。
    ///
    ///协议原文：
    ///
    ///<pre>Number of records to skip.</pre>
    ///
    ///
    #[serde(rename = "skipCount")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub skip_count: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CacheStorage.command.requestEntries/parameters/pageSize。
    ///
    ///协议原文：
    ///
    ///<pre>Number of records to fetch.</pre>
    ///
    ///
    #[serde(rename = "pageSize")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub page_size: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CacheStorage.command.requestEntries/parameters/pathFilter。
    ///
    ///协议原文：
    ///
    ///<pre>If present, only return the entries containing this substring in the path</pre>
    ///
    ///
    #[serde(rename = "pathFilter")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub path_filter: Option<String>,
}
impl<'de> serde::Deserialize<'de> for RequestEntriesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RequestEntriesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.command.requestEntries/parameters/cacheId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>ID of cache to get entries from.</pre>
                    ///
                    ///
                    #[serde(rename = "cacheId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cache_id: crate::cache_storage::types::CacheId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CacheStorage.command.requestEntries/parameters/skipCount。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Number of records to skip.</pre>
                    ///
                    ///
                    #[serde(rename = "skipCount")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub skip_count: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CacheStorage.command.requestEntries/parameters/pageSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Number of records to fetch.</pre>
                    ///
                    ///
                    #[serde(rename = "pageSize")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub page_size: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CacheStorage.command.requestEntries/parameters/pathFilter。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If present, only return the entries containing this substring in the path</pre>
                    ///
                    ///
                    #[serde(rename = "pathFilter")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub path_filter: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RequestEntriesParams {
                    cache_id: fields.cache_id,
                    skip_count: fields.skip_count,
                    page_size: fields.page_size,
                    path_filter: fields.path_filter,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：CacheStorage.command.requestEntries。
///
///协议原文：
///
///<pre>Requests data from cache.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RequestEntries;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CacheStorage.command.requestEntries/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequestEntriesResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.command.requestEntries/returns/cacheDataEntries。
    ///
    ///协议原文：
    ///
    ///<pre>Array of object store data entries.</pre>
    ///
    ///
    #[serde(rename = "cacheDataEntries")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cache_data_entries: Vec<crate::cache_storage::types::DataEntry>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.command.requestEntries/returns/returnCount。
    ///
    ///协议原文：
    ///
    ///<pre>Count of returned entries from this storage. If pathFilter is empty, it
    ///is the count of all entries from this storage.</pre>
    ///
    ///
    #[serde(rename = "returnCount")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub return_count: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for RequestEntriesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RequestEntriesResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.command.requestEntries/returns/cacheDataEntries。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Array of object store data entries.</pre>
                    ///
                    ///
                    #[serde(rename = "cacheDataEntries")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cache_data_entries: Vec<crate::cache_storage::types::DataEntry>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.command.requestEntries/returns/returnCount。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Count of returned entries from this storage. If pathFilter is empty, it
                    ///is the count of all entries from this storage.</pre>
                    ///
                    ///
                    #[serde(rename = "returnCount")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub return_count: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RequestEntriesResponse {
                    cache_data_entries: fields.cache_data_entries,
                    return_count: fields.return_count,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RequestEntries {
    type Params = RequestEntriesParams;
    type Response = RequestEntriesResponse;
    const METHOD: &'static str = "CacheStorage.requestEntries";
}
