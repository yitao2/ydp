///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CacheStorage.type.Cache。
///
///协议原文：
///
///<pre>Cache identifier.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct Cache {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.Cache/properties/cacheId。
    ///
    ///协议原文：
    ///
    ///<pre>An opaque unique id of the cache.</pre>
    ///
    ///
    #[serde(rename = "cacheId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cache_id: crate::cache_storage::types::CacheId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.Cache/properties/securityOrigin。
    ///
    ///协议原文：
    ///
    ///<pre>Security origin of the cache.</pre>
    ///
    ///
    #[serde(rename = "securityOrigin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub security_origin: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.Cache/properties/storageKey。
    ///
    ///协议原文：
    ///
    ///<pre>Storage key of the cache.</pre>
    ///
    ///
    #[serde(rename = "storageKey")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_key: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CacheStorage.type.Cache/properties/storageBucket。
    ///
    ///协议原文：
    ///
    ///<pre>Storage bucket of the cache.</pre>
    ///
    ///
    #[serde(rename = "storageBucket")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_bucket: Option<crate::storage::types::StorageBucket>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.Cache/properties/cacheName。
    ///
    ///协议原文：
    ///
    ///<pre>The name of the cache.</pre>
    ///
    ///
    #[serde(rename = "cacheName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cache_name: String,
}
impl<'de> serde::Deserialize<'de> for Cache {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Cache;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.Cache/properties/cacheId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An opaque unique id of the cache.</pre>
                    ///
                    ///
                    #[serde(rename = "cacheId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cache_id: crate::cache_storage::types::CacheId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.Cache/properties/securityOrigin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Security origin of the cache.</pre>
                    ///
                    ///
                    #[serde(rename = "securityOrigin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub security_origin: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.Cache/properties/storageKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage key of the cache.</pre>
                    ///
                    ///
                    #[serde(rename = "storageKey")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_key: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：CacheStorage.type.Cache/properties/storageBucket。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage bucket of the cache.</pre>
                    ///
                    ///
                    #[serde(rename = "storageBucket")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub storage_bucket: Option<crate::storage::types::StorageBucket>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.Cache/properties/cacheName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The name of the cache.</pre>
                    ///
                    ///
                    #[serde(rename = "cacheName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cache_name: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Cache {
                    cache_id: fields.cache_id,
                    security_origin: fields.security_origin,
                    storage_key: fields.storage_key,
                    storage_bucket: fields.storage_bucket,
                    cache_name: fields.cache_name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：CacheStorage.type.CacheId。
///
///协议原文：
///
///<pre>Unique identifier of the Cache object.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct CacheId(String);
impl CacheId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::cache_storage::types::CacheId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::cache_storage::types::CacheId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::cache_storage::types::CacheId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::cache_storage::types::CacheId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::cache_storage::types::CacheId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CacheStorage.type.CachedResponse。
///
///协议原文：
///
///<pre>Cached response</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CachedResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.CachedResponse/properties/body。
    ///
    ///协议原文：
    ///
    ///<pre>Entry content, base64-encoded. (Encoded as a base64 string when passed over JSON)</pre>
    ///
    ///
    #[serde(rename = "body")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub body: String,
}
impl<'de> serde::Deserialize<'de> for CachedResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CachedResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.CachedResponse/properties/body。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Entry content, base64-encoded. (Encoded as a base64 string when passed over JSON)</pre>
                    ///
                    ///
                    #[serde(rename = "body")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub body: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CachedResponse { body: fields.body })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：CacheStorage.type.CachedResponseType。
///
///协议原文：
///
///<pre>type of HTTP response cached</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CachedResponseType {
    ///协议值 "basic"。
    Basic,
    ///协议值 "cors"。
    Cors,
    ///协议值 "default"。
    Default,
    ///协议值 "error"。
    Error,
    ///协议值 "opaqueResponse"。
    OpaqueResponse,
    ///协议值 "opaqueRedirect"。
    OpaqueRedirect,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl CachedResponseType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::cache_storage::types::CachedResponseType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Basic => "basic",
            Self::Cors => "cors",
            Self::Default => "default",
            Self::Error => "error",
            Self::OpaqueResponse => "opaqueResponse",
            Self::OpaqueRedirect => "opaqueRedirect",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for CachedResponseType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for CachedResponseType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "basic" => CachedResponseType::Basic,
            "cors" => CachedResponseType::Cors,
            "default" => CachedResponseType::Default,
            "error" => CachedResponseType::Error,
            "opaqueResponse" => CachedResponseType::OpaqueResponse,
            "opaqueRedirect" => CachedResponseType::OpaqueRedirect,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for CachedResponseType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CachedResponseType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CachedResponseType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "basic" => CachedResponseType::Basic,
                    "cors" => CachedResponseType::Cors,
                    "default" => CachedResponseType::Default,
                    "error" => CachedResponseType::Error,
                    "opaqueResponse" => CachedResponseType::OpaqueResponse,
                    "opaqueRedirect" => CachedResponseType::OpaqueRedirect,
                    _ => CachedResponseType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(CachedResponseType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CacheStorage.type.DataEntry。
///
///协议原文：
///
///<pre>Data entry.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct DataEntry {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.DataEntry/properties/requestURL。
    ///
    ///协议原文：
    ///
    ///<pre>Request URL.</pre>
    ///
    ///
    #[serde(rename = "requestURL")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.DataEntry/properties/requestMethod。
    ///
    ///协议原文：
    ///
    ///<pre>Request method.</pre>
    ///
    ///
    #[serde(rename = "requestMethod")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_method: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.DataEntry/properties/requestHeaders。
    ///
    ///协议原文：
    ///
    ///<pre>Request headers</pre>
    ///
    ///
    #[serde(rename = "requestHeaders")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_headers: Vec<crate::cache_storage::types::Header>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.DataEntry/properties/responseTime。
    ///
    ///协议原文：
    ///
    ///<pre>Number of seconds since epoch.</pre>
    ///
    ///
    #[serde(rename = "responseTime")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub response_time: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.DataEntry/properties/responseStatus。
    ///
    ///协议原文：
    ///
    ///<pre>HTTP response status code.</pre>
    ///
    ///
    #[serde(rename = "responseStatus")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub response_status: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.DataEntry/properties/responseStatusText。
    ///
    ///协议原文：
    ///
    ///<pre>HTTP response status text.</pre>
    ///
    ///
    #[serde(rename = "responseStatusText")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub response_status_text: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.DataEntry/properties/responseType。
    ///
    ///协议原文：
    ///
    ///<pre>HTTP response type</pre>
    ///
    ///
    #[serde(rename = "responseType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub response_type: crate::cache_storage::types::CachedResponseType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.DataEntry/properties/responseHeaders。
    ///
    ///协议原文：
    ///
    ///<pre>Response headers</pre>
    ///
    ///
    #[serde(rename = "responseHeaders")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub response_headers: Vec<crate::cache_storage::types::Header>,
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
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.DataEntry/properties/requestURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request URL.</pre>
                    ///
                    ///
                    #[serde(rename = "requestURL")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.DataEntry/properties/requestMethod。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request method.</pre>
                    ///
                    ///
                    #[serde(rename = "requestMethod")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_method: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.DataEntry/properties/requestHeaders。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request headers</pre>
                    ///
                    ///
                    #[serde(rename = "requestHeaders")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_headers: Vec<crate::cache_storage::types::Header>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.DataEntry/properties/responseTime。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Number of seconds since epoch.</pre>
                    ///
                    ///
                    #[serde(rename = "responseTime")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub response_time: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.DataEntry/properties/responseStatus。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>HTTP response status code.</pre>
                    ///
                    ///
                    #[serde(rename = "responseStatus")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub response_status: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.DataEntry/properties/responseStatusText。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>HTTP response status text.</pre>
                    ///
                    ///
                    #[serde(rename = "responseStatusText")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub response_status_text: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.DataEntry/properties/responseType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>HTTP response type</pre>
                    ///
                    ///
                    #[serde(rename = "responseType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub response_type: crate::cache_storage::types::CachedResponseType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.DataEntry/properties/responseHeaders。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Response headers</pre>
                    ///
                    ///
                    #[serde(rename = "responseHeaders")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub response_headers: Vec<crate::cache_storage::types::Header>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DataEntry {
                    request_url: fields.request_url,
                    request_method: fields.request_method,
                    request_headers: fields.request_headers,
                    response_time: fields.response_time,
                    response_status: fields.response_status,
                    response_status_text: fields.response_status_text,
                    response_type: fields.response_type,
                    response_headers: fields.response_headers,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：CacheStorage.type.Header。
#[derive(Debug, Clone, serde::Serialize)]
pub struct Header {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.Header/properties/name。
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.Header/properties/value。
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: String,
}
impl<'de> serde::Deserialize<'de> for Header {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Header;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.Header/properties/name。
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：CacheStorage.type.Header/properties/value。
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Header {
                    name: fields.name,
                    value: fields.value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
