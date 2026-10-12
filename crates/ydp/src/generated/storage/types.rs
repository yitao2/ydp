///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.type.RelatedWebsiteSet。
///
///协议原文：
///
///<pre>A single Related Website Set object.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RelatedWebsiteSet {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.type.RelatedWebsiteSet/properties/primarySites。
    ///
    ///协议原文：
    ///
    ///<pre>The primary site of this set, along with the ccTLDs if there is any.</pre>
    ///
    ///
    #[serde(rename = "primarySites")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub primary_sites: Vec<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.type.RelatedWebsiteSet/properties/associatedSites。
    ///
    ///协议原文：
    ///
    ///<pre>The associated sites of this set, along with the ccTLDs if there is any.</pre>
    ///
    ///
    #[serde(rename = "associatedSites")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub associated_sites: Vec<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.type.RelatedWebsiteSet/properties/serviceSites。
    ///
    ///协议原文：
    ///
    ///<pre>The service sites of this set, along with the ccTLDs if there is any.</pre>
    ///
    ///
    #[serde(rename = "serviceSites")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub service_sites: Vec<String>,
}
impl<'de> serde::Deserialize<'de> for RelatedWebsiteSet {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RelatedWebsiteSet;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.type.RelatedWebsiteSet/properties/primarySites。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The primary site of this set, along with the ccTLDs if there is any.</pre>
                    ///
                    ///
                    #[serde(rename = "primarySites")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub primary_sites: Vec<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.type.RelatedWebsiteSet/properties/associatedSites。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The associated sites of this set, along with the ccTLDs if there is any.</pre>
                    ///
                    ///
                    #[serde(rename = "associatedSites")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub associated_sites: Vec<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.type.RelatedWebsiteSet/properties/serviceSites。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The service sites of this set, along with the ccTLDs if there is any.</pre>
                    ///
                    ///
                    #[serde(rename = "serviceSites")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub service_sites: Vec<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RelatedWebsiteSet {
                    primary_sites: fields.primary_sites,
                    associated_sites: fields.associated_sites,
                    service_sites: fields.service_sites,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Storage.type.SerializedStorageKey。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SerializedStorageKey(String);
impl SerializedStorageKey {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::storage::types::SerializedStorageKey = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::storage::types::SerializedStorageKey::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::storage::types::SerializedStorageKey = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::storage::types::SerializedStorageKey::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::storage::types::SerializedStorageKey::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.type.StorageBucket。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StorageBucket {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.type.StorageBucket/properties/storageKey。
    #[serde(rename = "storageKey")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_key: crate::storage::types::SerializedStorageKey,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Storage.type.StorageBucket/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>If not specified, it is the default bucket of the storageKey.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
}
impl<'de> serde::Deserialize<'de> for StorageBucket {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StorageBucket;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.type.StorageBucket/properties/storageKey。
                    #[serde(rename = "storageKey")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_key: crate::storage::types::SerializedStorageKey,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Storage.type.StorageBucket/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If not specified, it is the default bucket of the storageKey.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub name: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StorageBucket {
                    storage_key: fields.storage_key,
                    name: fields.name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.type.StorageBucketInfo。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StorageBucketInfo {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.type.StorageBucketInfo/properties/bucket。
    #[serde(rename = "bucket")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub bucket: crate::storage::types::StorageBucket,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.type.StorageBucketInfo/properties/id。
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.type.StorageBucketInfo/properties/expiration。
    #[serde(rename = "expiration")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub expiration: crate::network::types::TimeSinceEpoch,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.type.StorageBucketInfo/properties/quota。
    ///
    ///协议原文：
    ///
    ///<pre>Storage quota (bytes).</pre>
    ///
    ///
    #[serde(rename = "quota")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub quota: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.type.StorageBucketInfo/properties/persistent。
    #[serde(rename = "persistent")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub persistent: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.type.StorageBucketInfo/properties/durability。
    #[serde(rename = "durability")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub durability: crate::storage::types::StorageBucketsDurability,
}
impl<'de> serde::Deserialize<'de> for StorageBucketInfo {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StorageBucketInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.type.StorageBucketInfo/properties/bucket。
                    #[serde(rename = "bucket")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub bucket: crate::storage::types::StorageBucket,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.type.StorageBucketInfo/properties/id。
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.type.StorageBucketInfo/properties/expiration。
                    #[serde(rename = "expiration")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub expiration: crate::network::types::TimeSinceEpoch,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.type.StorageBucketInfo/properties/quota。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage quota (bytes).</pre>
                    ///
                    ///
                    #[serde(rename = "quota")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub quota: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.type.StorageBucketInfo/properties/persistent。
                    #[serde(rename = "persistent")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub persistent: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.type.StorageBucketInfo/properties/durability。
                    #[serde(rename = "durability")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub durability: crate::storage::types::StorageBucketsDurability,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StorageBucketInfo {
                    bucket: fields.bucket,
                    id: fields.id,
                    expiration: fields.expiration,
                    quota: fields.quota,
                    persistent: fields.persistent,
                    durability: fields.durability,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Storage.type.StorageBucketsDurability。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StorageBucketsDurability {
    ///协议值 "relaxed"。
    Relaxed,
    ///协议值 "strict"。
    Strict,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl StorageBucketsDurability {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::storage::types::StorageBucketsDurability::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Relaxed => "relaxed",
            Self::Strict => "strict",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for StorageBucketsDurability {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for StorageBucketsDurability {
    fn from(value: String) -> Self {
        match value.as_str() {
            "relaxed" => StorageBucketsDurability::Relaxed,
            "strict" => StorageBucketsDurability::Strict,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for StorageBucketsDurability {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StorageBucketsDurability {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StorageBucketsDurability;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "relaxed" => StorageBucketsDurability::Relaxed,
                    "strict" => StorageBucketsDurability::Strict,
                    _ => StorageBucketsDurability::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(StorageBucketsDurability::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Storage.type.StorageType。
///
///协议原文：
///
///<pre>Enum of possible storage types.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StorageType {
    ///协议值 "cookies"。
    Cookies,
    ///协议值 "file_systems"。
    FileSystems,
    ///协议值 "indexeddb"。
    Indexeddb,
    ///协议值 "local_storage"。
    LocalStorage,
    ///协议值 "shader_cache"。
    ShaderCache,
    ///协议值 "websql"。
    Websql,
    ///协议值 "service_workers"。
    ServiceWorkers,
    ///协议值 "cache_storage"。
    CacheStorage,
    ///协议值 "storage_buckets"。
    StorageBuckets,
    ///协议值 "all"。
    All,
    ///协议值 "other"。
    Other,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl StorageType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::storage::types::StorageType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Cookies => "cookies",
            Self::FileSystems => "file_systems",
            Self::Indexeddb => "indexeddb",
            Self::LocalStorage => "local_storage",
            Self::ShaderCache => "shader_cache",
            Self::Websql => "websql",
            Self::ServiceWorkers => "service_workers",
            Self::CacheStorage => "cache_storage",
            Self::StorageBuckets => "storage_buckets",
            Self::All => "all",
            Self::Other => "other",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for StorageType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for StorageType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "cookies" => StorageType::Cookies,
            "file_systems" => StorageType::FileSystems,
            "indexeddb" => StorageType::Indexeddb,
            "local_storage" => StorageType::LocalStorage,
            "shader_cache" => StorageType::ShaderCache,
            "websql" => StorageType::Websql,
            "service_workers" => StorageType::ServiceWorkers,
            "cache_storage" => StorageType::CacheStorage,
            "storage_buckets" => StorageType::StorageBuckets,
            "all" => StorageType::All,
            "other" => StorageType::Other,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for StorageType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StorageType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StorageType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "cookies" => StorageType::Cookies,
                    "file_systems" => StorageType::FileSystems,
                    "indexeddb" => StorageType::Indexeddb,
                    "local_storage" => StorageType::LocalStorage,
                    "shader_cache" => StorageType::ShaderCache,
                    "websql" => StorageType::Websql,
                    "service_workers" => StorageType::ServiceWorkers,
                    "cache_storage" => StorageType::CacheStorage,
                    "storage_buckets" => StorageType::StorageBuckets,
                    "all" => StorageType::All,
                    "other" => StorageType::Other,
                    _ => StorageType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(StorageType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.type.TrustTokens。
///
///协议原文：
///
///<pre>Pair of issuer origin and number of available (signed, but not used) Trust
///Tokens from that issuer.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TrustTokens {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.type.TrustTokens/properties/issuerOrigin。
    #[serde(rename = "issuerOrigin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub issuer_origin: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.type.TrustTokens/properties/count。
    #[serde(rename = "count")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub count: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for TrustTokens {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TrustTokens;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.type.TrustTokens/properties/issuerOrigin。
                    #[serde(rename = "issuerOrigin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub issuer_origin: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.type.TrustTokens/properties/count。
                    #[serde(rename = "count")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub count: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TrustTokens {
                    issuer_origin: fields.issuer_origin,
                    count: fields.count,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.type.UsageForType。
///
///协议原文：
///
///<pre>Usage for a storage type.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct UsageForType {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.type.UsageForType/properties/storageType。
    ///
    ///协议原文：
    ///
    ///<pre>Name of storage type.</pre>
    ///
    ///
    #[serde(rename = "storageType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_type: crate::storage::types::StorageType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.type.UsageForType/properties/usage。
    ///
    ///协议原文：
    ///
    ///<pre>Storage usage (bytes).</pre>
    ///
    ///
    #[serde(rename = "usage")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub usage: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for UsageForType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UsageForType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.type.UsageForType/properties/storageType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Name of storage type.</pre>
                    ///
                    ///
                    #[serde(rename = "storageType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_type: crate::storage::types::StorageType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.type.UsageForType/properties/usage。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage usage (bytes).</pre>
                    ///
                    ///
                    #[serde(rename = "usage")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub usage: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(UsageForType {
                    storage_type: fields.storage_type,
                    usage: fields.usage,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
