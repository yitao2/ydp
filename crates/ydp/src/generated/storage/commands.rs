///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.clearCookies/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ClearCookiesParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Storage.command.clearCookies/parameters/browserContextId。
    ///
    ///协议原文：
    ///
    ///<pre>Browser context to use when called on the browser endpoint.</pre>
    ///
    ///
    #[serde(rename = "browserContextId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub browser_context_id: Option<crate::browser::types::BrowserContextId>,
}
impl<'de> serde::Deserialize<'de> for ClearCookiesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearCookiesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Storage.command.clearCookies/parameters/browserContextId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Browser context to use when called on the browser endpoint.</pre>
                    ///
                    ///
                    #[serde(rename = "browserContextId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub browser_context_id: Option<crate::browser::types::BrowserContextId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ClearCookiesParams {
                    browser_context_id: fields.browser_context_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.command.clearCookies。
///
///协议原文：
///
///<pre>Clears cookies.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ClearCookies;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.clearCookies/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearCookiesResponse {}
impl<'de> serde::Deserialize<'de> for ClearCookiesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearCookiesResponse;
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
                Ok(ClearCookiesResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ClearCookies {
    type Params = ClearCookiesParams;
    type Response = ClearCookiesResponse;
    const METHOD: &'static str = "Storage.clearCookies";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.clearDataForOrigin/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearDataForOriginParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.clearDataForOrigin/parameters/origin。
    ///
    ///协议原文：
    ///
    ///<pre>Security origin.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.clearDataForOrigin/parameters/storageTypes。
    ///
    ///协议原文：
    ///
    ///<pre>Comma separated list of StorageType to clear.</pre>
    ///
    ///
    #[serde(rename = "storageTypes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_types: String,
}
impl<'de> serde::Deserialize<'de> for ClearDataForOriginParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearDataForOriginParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.clearDataForOrigin/parameters/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Security origin.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.clearDataForOrigin/parameters/storageTypes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Comma separated list of StorageType to clear.</pre>
                    ///
                    ///
                    #[serde(rename = "storageTypes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_types: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ClearDataForOriginParams {
                    origin: fields.origin,
                    storage_types: fields.storage_types,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.command.clearDataForOrigin。
///
///协议原文：
///
///<pre>Clears storage for origin.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ClearDataForOrigin;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.clearDataForOrigin/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearDataForOriginResponse {}
impl<'de> serde::Deserialize<'de> for ClearDataForOriginResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearDataForOriginResponse;
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
                Ok(ClearDataForOriginResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ClearDataForOrigin {
    type Params = ClearDataForOriginParams;
    type Response = ClearDataForOriginResponse;
    const METHOD: &'static str = "Storage.clearDataForOrigin";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.clearDataForStorageKey/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearDataForStorageKeyParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.clearDataForStorageKey/parameters/storageKey。
    ///
    ///协议原文：
    ///
    ///<pre>Storage key.</pre>
    ///
    ///
    #[serde(rename = "storageKey")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_key: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.clearDataForStorageKey/parameters/storageTypes。
    ///
    ///协议原文：
    ///
    ///<pre>Comma separated list of StorageType to clear.</pre>
    ///
    ///
    #[serde(rename = "storageTypes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_types: String,
}
impl<'de> serde::Deserialize<'de> for ClearDataForStorageKeyParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearDataForStorageKeyParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.clearDataForStorageKey/parameters/storageKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage key.</pre>
                    ///
                    ///
                    #[serde(rename = "storageKey")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_key: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.clearDataForStorageKey/parameters/storageTypes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Comma separated list of StorageType to clear.</pre>
                    ///
                    ///
                    #[serde(rename = "storageTypes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_types: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ClearDataForStorageKeyParams {
                    storage_key: fields.storage_key,
                    storage_types: fields.storage_types,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.command.clearDataForStorageKey。
///
///协议原文：
///
///<pre>Clears storage for storage key.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ClearDataForStorageKey;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.clearDataForStorageKey/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearDataForStorageKeyResponse {}
impl<'de> serde::Deserialize<'de> for ClearDataForStorageKeyResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearDataForStorageKeyResponse;
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
                Ok(ClearDataForStorageKeyResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ClearDataForStorageKey {
    type Params = ClearDataForStorageKeyParams;
    type Response = ClearDataForStorageKeyResponse;
    const METHOD: &'static str = "Storage.clearDataForStorageKey";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.clearTrustTokens/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearTrustTokensParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.clearTrustTokens/parameters/issuerOrigin。
    #[serde(rename = "issuerOrigin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub issuer_origin: String,
}
impl<'de> serde::Deserialize<'de> for ClearTrustTokensParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearTrustTokensParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.clearTrustTokens/parameters/issuerOrigin。
                    #[serde(rename = "issuerOrigin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub issuer_origin: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ClearTrustTokensParams {
                    issuer_origin: fields.issuer_origin,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.command.clearTrustTokens。
///
///协议原文：
///
///<pre>Removes all Trust Tokens issued by the provided issuerOrigin.
///Leaves other stored data, including the issuer's Redemption Records, intact.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ClearTrustTokens;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.clearTrustTokens/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearTrustTokensResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.clearTrustTokens/returns/didDeleteTokens。
    ///
    ///协议原文：
    ///
    ///<pre>True if any tokens were deleted, false otherwise.</pre>
    ///
    ///
    #[serde(rename = "didDeleteTokens")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub did_delete_tokens: bool,
}
impl<'de> serde::Deserialize<'de> for ClearTrustTokensResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearTrustTokensResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.clearTrustTokens/returns/didDeleteTokens。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if any tokens were deleted, false otherwise.</pre>
                    ///
                    ///
                    #[serde(rename = "didDeleteTokens")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub did_delete_tokens: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ClearTrustTokensResponse {
                    did_delete_tokens: fields.did_delete_tokens,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ClearTrustTokens {
    type Params = ClearTrustTokensParams;
    type Response = ClearTrustTokensResponse;
    const METHOD: &'static str = "Storage.clearTrustTokens";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.deleteStorageBucket/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeleteStorageBucketParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.deleteStorageBucket/parameters/bucket。
    #[serde(rename = "bucket")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub bucket: crate::storage::types::StorageBucket,
}
impl<'de> serde::Deserialize<'de> for DeleteStorageBucketParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeleteStorageBucketParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.deleteStorageBucket/parameters/bucket。
                    #[serde(rename = "bucket")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub bucket: crate::storage::types::StorageBucket,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DeleteStorageBucketParams {
                    bucket: fields.bucket,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.command.deleteStorageBucket。
///
///协议原文：
///
///<pre>Deletes the Storage Bucket with the given storage key and bucket name.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct DeleteStorageBucket;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.deleteStorageBucket/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeleteStorageBucketResponse {}
impl<'de> serde::Deserialize<'de> for DeleteStorageBucketResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeleteStorageBucketResponse;
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
                Ok(DeleteStorageBucketResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for DeleteStorageBucket {
    type Params = DeleteStorageBucketParams;
    type Response = DeleteStorageBucketResponse;
    const METHOD: &'static str = "Storage.deleteStorageBucket";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.getCookies/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetCookiesParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Storage.command.getCookies/parameters/browserContextId。
    ///
    ///协议原文：
    ///
    ///<pre>Browser context to use when called on the browser endpoint.</pre>
    ///
    ///
    #[serde(rename = "browserContextId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub browser_context_id: Option<crate::browser::types::BrowserContextId>,
}
impl<'de> serde::Deserialize<'de> for GetCookiesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetCookiesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Storage.command.getCookies/parameters/browserContextId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Browser context to use when called on the browser endpoint.</pre>
                    ///
                    ///
                    #[serde(rename = "browserContextId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub browser_context_id: Option<crate::browser::types::BrowserContextId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetCookiesParams {
                    browser_context_id: fields.browser_context_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.command.getCookies。
///
///协议原文：
///
///<pre>Returns all browser cookies.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetCookies;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.getCookies/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetCookiesResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.getCookies/returns/cookies。
    ///
    ///协议原文：
    ///
    ///<pre>Array of cookie objects.</pre>
    ///
    ///
    #[serde(rename = "cookies")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cookies: Vec<crate::network::types::Cookie>,
}
impl<'de> serde::Deserialize<'de> for GetCookiesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetCookiesResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.getCookies/returns/cookies。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Array of cookie objects.</pre>
                    ///
                    ///
                    #[serde(rename = "cookies")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cookies: Vec<crate::network::types::Cookie>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetCookiesResponse {
                    cookies: fields.cookies,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetCookies {
    type Params = GetCookiesParams;
    type Response = GetCookiesResponse;
    const METHOD: &'static str = "Storage.getCookies";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.getRelatedWebsiteSets/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetRelatedWebsiteSetsParams {}
impl<'de> serde::Deserialize<'de> for GetRelatedWebsiteSetsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetRelatedWebsiteSetsParams;
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
                Ok(GetRelatedWebsiteSetsParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.command.getRelatedWebsiteSets。
///
///协议原文：
///
///<pre>Returns the effective Related Website Sets in use by this profile for the browser
///session. The effective Related Website Sets will not change during a browser session.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetRelatedWebsiteSets;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.getRelatedWebsiteSets/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetRelatedWebsiteSetsResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.getRelatedWebsiteSets/returns/sets。
    #[serde(rename = "sets")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub sets: Vec<crate::storage::types::RelatedWebsiteSet>,
}
impl<'de> serde::Deserialize<'de> for GetRelatedWebsiteSetsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetRelatedWebsiteSetsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.getRelatedWebsiteSets/returns/sets。
                    #[serde(rename = "sets")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub sets: Vec<crate::storage::types::RelatedWebsiteSet>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetRelatedWebsiteSetsResponse { sets: fields.sets })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetRelatedWebsiteSets {
    type Params = GetRelatedWebsiteSetsParams;
    type Response = GetRelatedWebsiteSetsResponse;
    const METHOD: &'static str = "Storage.getRelatedWebsiteSets";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.getStorageKey/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetStorageKeyParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Storage.command.getStorageKey/parameters/frameId。
    #[serde(rename = "frameId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub frame_id: Option<crate::page::types::FrameId>,
}
impl<'de> serde::Deserialize<'de> for GetStorageKeyParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetStorageKeyParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Storage.command.getStorageKey/parameters/frameId。
                    #[serde(rename = "frameId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub frame_id: Option<crate::page::types::FrameId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetStorageKeyParams {
                    frame_id: fields.frame_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.command.getStorageKey。
///
///协议原文：
///
///<pre>Returns storage key for the given frame. If no frame ID is provided,
///the storage key of the target executing this command is returned.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetStorageKey;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.getStorageKey/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetStorageKeyResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.getStorageKey/returns/storageKey。
    #[serde(rename = "storageKey")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_key: crate::storage::types::SerializedStorageKey,
}
impl<'de> serde::Deserialize<'de> for GetStorageKeyResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetStorageKeyResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.getStorageKey/returns/storageKey。
                    #[serde(rename = "storageKey")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_key: crate::storage::types::SerializedStorageKey,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetStorageKeyResponse {
                    storage_key: fields.storage_key,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetStorageKey {
    type Params = GetStorageKeyParams;
    type Response = GetStorageKeyResponse;
    const METHOD: &'static str = "Storage.getStorageKey";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.getStorageKeyForFrame/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetStorageKeyForFrameParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.getStorageKeyForFrame/parameters/frameId。
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
}
impl<'de> serde::Deserialize<'de> for GetStorageKeyForFrameParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetStorageKeyForFrameParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.getStorageKeyForFrame/parameters/frameId。
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetStorageKeyForFrameParams {
                    frame_id: fields.frame_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.command.getStorageKeyForFrame。
///
///协议原文：
///
///<pre>Returns a storage key given a frame id.
///Deprecated. Please use Storage.getStorageKey instead.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct GetStorageKeyForFrame;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.getStorageKeyForFrame/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetStorageKeyForFrameResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.getStorageKeyForFrame/returns/storageKey。
    #[serde(rename = "storageKey")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_key: crate::storage::types::SerializedStorageKey,
}
impl<'de> serde::Deserialize<'de> for GetStorageKeyForFrameResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetStorageKeyForFrameResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.getStorageKeyForFrame/returns/storageKey。
                    #[serde(rename = "storageKey")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_key: crate::storage::types::SerializedStorageKey,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetStorageKeyForFrameResponse {
                    storage_key: fields.storage_key,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetStorageKeyForFrame {
    type Params = GetStorageKeyForFrameParams;
    type Response = GetStorageKeyForFrameResponse;
    const METHOD: &'static str = "Storage.getStorageKeyForFrame";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.getTrustTokens/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetTrustTokensParams {}
impl<'de> serde::Deserialize<'de> for GetTrustTokensParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetTrustTokensParams;
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
                Ok(GetTrustTokensParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.command.getTrustTokens。
///
///协议原文：
///
///<pre>Returns the number of stored Trust Tokens per issuer for the
///current browsing context.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetTrustTokens;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.getTrustTokens/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetTrustTokensResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.getTrustTokens/returns/tokens。
    #[serde(rename = "tokens")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub tokens: Vec<crate::storage::types::TrustTokens>,
}
impl<'de> serde::Deserialize<'de> for GetTrustTokensResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetTrustTokensResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.getTrustTokens/returns/tokens。
                    #[serde(rename = "tokens")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub tokens: Vec<crate::storage::types::TrustTokens>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetTrustTokensResponse {
                    tokens: fields.tokens,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetTrustTokens {
    type Params = GetTrustTokensParams;
    type Response = GetTrustTokensResponse;
    const METHOD: &'static str = "Storage.getTrustTokens";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.getUsageAndQuota/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetUsageAndQuotaParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.getUsageAndQuota/parameters/origin。
    ///
    ///协议原文：
    ///
    ///<pre>Security origin.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: String,
}
impl<'de> serde::Deserialize<'de> for GetUsageAndQuotaParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetUsageAndQuotaParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.getUsageAndQuota/parameters/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Security origin.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetUsageAndQuotaParams {
                    origin: fields.origin,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.command.getUsageAndQuota。
///
///协议原文：
///
///<pre>Returns usage and quota in bytes.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetUsageAndQuota;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.getUsageAndQuota/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetUsageAndQuotaResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.getUsageAndQuota/returns/usage。
    ///
    ///协议原文：
    ///
    ///<pre>Storage usage (bytes).</pre>
    ///
    ///
    #[serde(rename = "usage")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub usage: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.getUsageAndQuota/returns/quota。
    ///
    ///协议原文：
    ///
    ///<pre>Storage quota (bytes).</pre>
    ///
    ///
    #[serde(rename = "quota")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub quota: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.getUsageAndQuota/returns/overrideActive。
    ///
    ///协议原文：
    ///
    ///<pre>Whether or not the origin has an active storage quota override</pre>
    ///
    ///
    #[serde(rename = "overrideActive")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub override_active: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.getUsageAndQuota/returns/usageBreakdown。
    ///
    ///协议原文：
    ///
    ///<pre>Storage usage per type (bytes).</pre>
    ///
    ///
    #[serde(rename = "usageBreakdown")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub usage_breakdown: Vec<crate::storage::types::UsageForType>,
}
impl<'de> serde::Deserialize<'de> for GetUsageAndQuotaResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetUsageAndQuotaResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.getUsageAndQuota/returns/usage。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage usage (bytes).</pre>
                    ///
                    ///
                    #[serde(rename = "usage")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub usage: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.getUsageAndQuota/returns/quota。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage quota (bytes).</pre>
                    ///
                    ///
                    #[serde(rename = "quota")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub quota: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.getUsageAndQuota/returns/overrideActive。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether or not the origin has an active storage quota override</pre>
                    ///
                    ///
                    #[serde(rename = "overrideActive")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub override_active: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.getUsageAndQuota/returns/usageBreakdown。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage usage per type (bytes).</pre>
                    ///
                    ///
                    #[serde(rename = "usageBreakdown")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub usage_breakdown: Vec<crate::storage::types::UsageForType>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetUsageAndQuotaResponse {
                    usage: fields.usage,
                    quota: fields.quota,
                    override_active: fields.override_active,
                    usage_breakdown: fields.usage_breakdown,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetUsageAndQuota {
    type Params = GetUsageAndQuotaParams;
    type Response = GetUsageAndQuotaResponse;
    const METHOD: &'static str = "Storage.getUsageAndQuota";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.overrideQuotaForOrigin/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct OverrideQuotaForOriginParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.overrideQuotaForOrigin/parameters/origin。
    ///
    ///协议原文：
    ///
    ///<pre>Security origin.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Storage.command.overrideQuotaForOrigin/parameters/quotaSize。
    ///
    ///协议原文：
    ///
    ///<pre>The quota size (in bytes) to override the original quota with.
    ///If this is called multiple times, the overridden quota will be equal to
    ///the quotaSize provided in the final call. If this is called without
    ///specifying a quotaSize, the quota will be reset to the default value for
    ///the specified origin. If this is called multiple times with different
    ///origins, the override will be maintained for each origin until it is
    ///disabled (called without a quotaSize).</pre>
    ///
    ///
    #[serde(rename = "quotaSize")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub quota_size: Option<ydp_core::FiniteF64>,
}
impl<'de> serde::Deserialize<'de> for OverrideQuotaForOriginParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = OverrideQuotaForOriginParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.overrideQuotaForOrigin/parameters/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Security origin.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Storage.command.overrideQuotaForOrigin/parameters/quotaSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The quota size (in bytes) to override the original quota with.
                    ///If this is called multiple times, the overridden quota will be equal to
                    ///the quotaSize provided in the final call. If this is called without
                    ///specifying a quotaSize, the quota will be reset to the default value for
                    ///the specified origin. If this is called multiple times with different
                    ///origins, the override will be maintained for each origin until it is
                    ///disabled (called without a quotaSize).</pre>
                    ///
                    ///
                    #[serde(rename = "quotaSize")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub quota_size: Option<ydp_core::FiniteF64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(OverrideQuotaForOriginParams {
                    origin: fields.origin,
                    quota_size: fields.quota_size,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.command.overrideQuotaForOrigin。
///
///协议原文：
///
///<pre>Override quota for the specified origin</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct OverrideQuotaForOrigin;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.overrideQuotaForOrigin/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct OverrideQuotaForOriginResponse {}
impl<'de> serde::Deserialize<'de> for OverrideQuotaForOriginResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = OverrideQuotaForOriginResponse;
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
                Ok(OverrideQuotaForOriginResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for OverrideQuotaForOrigin {
    type Params = OverrideQuotaForOriginParams;
    type Response = OverrideQuotaForOriginResponse;
    const METHOD: &'static str = "Storage.overrideQuotaForOrigin";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.runBounceTrackingMitigations/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct RunBounceTrackingMitigationsParams {}
impl<'de> serde::Deserialize<'de> for RunBounceTrackingMitigationsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RunBounceTrackingMitigationsParams;
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
                Ok(RunBounceTrackingMitigationsParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.command.runBounceTrackingMitigations。
///
///协议原文：
///
///<pre>Deletes state for sites identified as potential bounce trackers, immediately.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct RunBounceTrackingMitigations;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.runBounceTrackingMitigations/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RunBounceTrackingMitigationsResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.runBounceTrackingMitigations/returns/deletedSites。
    #[serde(rename = "deletedSites")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub deleted_sites: Vec<String>,
}
impl<'de> serde::Deserialize<'de> for RunBounceTrackingMitigationsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RunBounceTrackingMitigationsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.runBounceTrackingMitigations/returns/deletedSites。
                    #[serde(rename = "deletedSites")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub deleted_sites: Vec<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RunBounceTrackingMitigationsResponse {
                    deleted_sites: fields.deleted_sites,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RunBounceTrackingMitigations {
    type Params = RunBounceTrackingMitigationsParams;
    type Response = RunBounceTrackingMitigationsResponse;
    const METHOD: &'static str = "Storage.runBounceTrackingMitigations";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.setCookies/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetCookiesParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.setCookies/parameters/cookies。
    ///
    ///协议原文：
    ///
    ///<pre>Cookies to be set.</pre>
    ///
    ///
    #[serde(rename = "cookies")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cookies: Vec<crate::network::types::CookieParam>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Storage.command.setCookies/parameters/browserContextId。
    ///
    ///协议原文：
    ///
    ///<pre>Browser context to use when called on the browser endpoint.</pre>
    ///
    ///
    #[serde(rename = "browserContextId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub browser_context_id: Option<crate::browser::types::BrowserContextId>,
}
impl<'de> serde::Deserialize<'de> for SetCookiesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetCookiesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.setCookies/parameters/cookies。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookies to be set.</pre>
                    ///
                    ///
                    #[serde(rename = "cookies")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cookies: Vec<crate::network::types::CookieParam>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Storage.command.setCookies/parameters/browserContextId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Browser context to use when called on the browser endpoint.</pre>
                    ///
                    ///
                    #[serde(rename = "browserContextId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub browser_context_id: Option<crate::browser::types::BrowserContextId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetCookiesParams {
                    cookies: fields.cookies,
                    browser_context_id: fields.browser_context_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.command.setCookies。
///
///协议原文：
///
///<pre>Sets given cookies.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetCookies;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.setCookies/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetCookiesResponse {}
impl<'de> serde::Deserialize<'de> for SetCookiesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetCookiesResponse;
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
                Ok(SetCookiesResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetCookies {
    type Params = SetCookiesParams;
    type Response = SetCookiesResponse;
    const METHOD: &'static str = "Storage.setCookies";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.setStorageBucketTracking/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetStorageBucketTrackingParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.setStorageBucketTracking/parameters/storageKey。
    #[serde(rename = "storageKey")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_key: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.setStorageBucketTracking/parameters/enable。
    #[serde(rename = "enable")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub enable: bool,
}
impl<'de> serde::Deserialize<'de> for SetStorageBucketTrackingParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetStorageBucketTrackingParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.setStorageBucketTracking/parameters/storageKey。
                    #[serde(rename = "storageKey")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_key: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.setStorageBucketTracking/parameters/enable。
                    #[serde(rename = "enable")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub enable: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetStorageBucketTrackingParams {
                    storage_key: fields.storage_key,
                    enable: fields.enable,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.command.setStorageBucketTracking。
///
///协议原文：
///
///<pre>Set tracking for a storage key's buckets.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetStorageBucketTracking;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.setStorageBucketTracking/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetStorageBucketTrackingResponse {}
impl<'de> serde::Deserialize<'de> for SetStorageBucketTrackingResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetStorageBucketTrackingResponse;
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
                Ok(SetStorageBucketTrackingResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetStorageBucketTracking {
    type Params = SetStorageBucketTrackingParams;
    type Response = SetStorageBucketTrackingResponse;
    const METHOD: &'static str = "Storage.setStorageBucketTracking";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.trackCacheStorageForOrigin/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TrackCacheStorageForOriginParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.trackCacheStorageForOrigin/parameters/origin。
    ///
    ///协议原文：
    ///
    ///<pre>Security origin.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: String,
}
impl<'de> serde::Deserialize<'de> for TrackCacheStorageForOriginParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TrackCacheStorageForOriginParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.trackCacheStorageForOrigin/parameters/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Security origin.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TrackCacheStorageForOriginParams {
                    origin: fields.origin,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.command.trackCacheStorageForOrigin。
///
///协议原文：
///
///<pre>Registers origin to be notified when an update occurs to its cache storage list.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct TrackCacheStorageForOrigin;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.trackCacheStorageForOrigin/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TrackCacheStorageForOriginResponse {}
impl<'de> serde::Deserialize<'de> for TrackCacheStorageForOriginResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TrackCacheStorageForOriginResponse;
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
                Ok(TrackCacheStorageForOriginResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for TrackCacheStorageForOrigin {
    type Params = TrackCacheStorageForOriginParams;
    type Response = TrackCacheStorageForOriginResponse;
    const METHOD: &'static str = "Storage.trackCacheStorageForOrigin";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.trackCacheStorageForStorageKey/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TrackCacheStorageForStorageKeyParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.trackCacheStorageForStorageKey/parameters/storageKey。
    ///
    ///协议原文：
    ///
    ///<pre>Storage key.</pre>
    ///
    ///
    #[serde(rename = "storageKey")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_key: String,
}
impl<'de> serde::Deserialize<'de> for TrackCacheStorageForStorageKeyParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TrackCacheStorageForStorageKeyParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.trackCacheStorageForStorageKey/parameters/storageKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage key.</pre>
                    ///
                    ///
                    #[serde(rename = "storageKey")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_key: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TrackCacheStorageForStorageKeyParams {
                    storage_key: fields.storage_key,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.command.trackCacheStorageForStorageKey。
///
///协议原文：
///
///<pre>Registers storage key to be notified when an update occurs to its cache storage list.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct TrackCacheStorageForStorageKey;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.trackCacheStorageForStorageKey/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TrackCacheStorageForStorageKeyResponse {}
impl<'de> serde::Deserialize<'de> for TrackCacheStorageForStorageKeyResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TrackCacheStorageForStorageKeyResponse;
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
                Ok(TrackCacheStorageForStorageKeyResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for TrackCacheStorageForStorageKey {
    type Params = TrackCacheStorageForStorageKeyParams;
    type Response = TrackCacheStorageForStorageKeyResponse;
    const METHOD: &'static str = "Storage.trackCacheStorageForStorageKey";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.trackIndexedDBForOrigin/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TrackIndexedDbForOriginParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.trackIndexedDBForOrigin/parameters/origin。
    ///
    ///协议原文：
    ///
    ///<pre>Security origin.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: String,
}
impl<'de> serde::Deserialize<'de> for TrackIndexedDbForOriginParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TrackIndexedDbForOriginParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.trackIndexedDBForOrigin/parameters/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Security origin.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TrackIndexedDbForOriginParams {
                    origin: fields.origin,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.command.trackIndexedDBForOrigin。
///
///协议原文：
///
///<pre>Registers origin to be notified when an update occurs to its IndexedDB.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct TrackIndexedDbForOrigin;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.trackIndexedDBForOrigin/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TrackIndexedDbForOriginResponse {}
impl<'de> serde::Deserialize<'de> for TrackIndexedDbForOriginResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TrackIndexedDbForOriginResponse;
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
                Ok(TrackIndexedDbForOriginResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for TrackIndexedDbForOrigin {
    type Params = TrackIndexedDbForOriginParams;
    type Response = TrackIndexedDbForOriginResponse;
    const METHOD: &'static str = "Storage.trackIndexedDBForOrigin";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.trackIndexedDBForStorageKey/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TrackIndexedDbForStorageKeyParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.trackIndexedDBForStorageKey/parameters/storageKey。
    ///
    ///协议原文：
    ///
    ///<pre>Storage key.</pre>
    ///
    ///
    #[serde(rename = "storageKey")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_key: String,
}
impl<'de> serde::Deserialize<'de> for TrackIndexedDbForStorageKeyParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TrackIndexedDbForStorageKeyParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.trackIndexedDBForStorageKey/parameters/storageKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage key.</pre>
                    ///
                    ///
                    #[serde(rename = "storageKey")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_key: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TrackIndexedDbForStorageKeyParams {
                    storage_key: fields.storage_key,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.command.trackIndexedDBForStorageKey。
///
///协议原文：
///
///<pre>Registers storage key to be notified when an update occurs to its IndexedDB.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct TrackIndexedDbForStorageKey;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.trackIndexedDBForStorageKey/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TrackIndexedDbForStorageKeyResponse {}
impl<'de> serde::Deserialize<'de> for TrackIndexedDbForStorageKeyResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TrackIndexedDbForStorageKeyResponse;
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
                Ok(TrackIndexedDbForStorageKeyResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for TrackIndexedDbForStorageKey {
    type Params = TrackIndexedDbForStorageKeyParams;
    type Response = TrackIndexedDbForStorageKeyResponse;
    const METHOD: &'static str = "Storage.trackIndexedDBForStorageKey";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.untrackCacheStorageForOrigin/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct UntrackCacheStorageForOriginParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.untrackCacheStorageForOrigin/parameters/origin。
    ///
    ///协议原文：
    ///
    ///<pre>Security origin.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: String,
}
impl<'de> serde::Deserialize<'de> for UntrackCacheStorageForOriginParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UntrackCacheStorageForOriginParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.untrackCacheStorageForOrigin/parameters/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Security origin.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(UntrackCacheStorageForOriginParams {
                    origin: fields.origin,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.command.untrackCacheStorageForOrigin。
///
///协议原文：
///
///<pre>Unregisters origin from receiving notifications for cache storage.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct UntrackCacheStorageForOrigin;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.untrackCacheStorageForOrigin/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct UntrackCacheStorageForOriginResponse {}
impl<'de> serde::Deserialize<'de> for UntrackCacheStorageForOriginResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UntrackCacheStorageForOriginResponse;
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
                Ok(UntrackCacheStorageForOriginResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for UntrackCacheStorageForOrigin {
    type Params = UntrackCacheStorageForOriginParams;
    type Response = UntrackCacheStorageForOriginResponse;
    const METHOD: &'static str = "Storage.untrackCacheStorageForOrigin";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.untrackCacheStorageForStorageKey/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct UntrackCacheStorageForStorageKeyParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.untrackCacheStorageForStorageKey/parameters/storageKey。
    ///
    ///协议原文：
    ///
    ///<pre>Storage key.</pre>
    ///
    ///
    #[serde(rename = "storageKey")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_key: String,
}
impl<'de> serde::Deserialize<'de> for UntrackCacheStorageForStorageKeyParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UntrackCacheStorageForStorageKeyParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.untrackCacheStorageForStorageKey/parameters/storageKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage key.</pre>
                    ///
                    ///
                    #[serde(rename = "storageKey")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_key: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(UntrackCacheStorageForStorageKeyParams {
                    storage_key: fields.storage_key,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.command.untrackCacheStorageForStorageKey。
///
///协议原文：
///
///<pre>Unregisters storage key from receiving notifications for cache storage.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct UntrackCacheStorageForStorageKey;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.untrackCacheStorageForStorageKey/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct UntrackCacheStorageForStorageKeyResponse {}
impl<'de> serde::Deserialize<'de> for UntrackCacheStorageForStorageKeyResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UntrackCacheStorageForStorageKeyResponse;
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
                Ok(UntrackCacheStorageForStorageKeyResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for UntrackCacheStorageForStorageKey {
    type Params = UntrackCacheStorageForStorageKeyParams;
    type Response = UntrackCacheStorageForStorageKeyResponse;
    const METHOD: &'static str = "Storage.untrackCacheStorageForStorageKey";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.untrackIndexedDBForOrigin/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct UntrackIndexedDbForOriginParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.untrackIndexedDBForOrigin/parameters/origin。
    ///
    ///协议原文：
    ///
    ///<pre>Security origin.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: String,
}
impl<'de> serde::Deserialize<'de> for UntrackIndexedDbForOriginParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UntrackIndexedDbForOriginParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.untrackIndexedDBForOrigin/parameters/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Security origin.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(UntrackIndexedDbForOriginParams {
                    origin: fields.origin,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.command.untrackIndexedDBForOrigin。
///
///协议原文：
///
///<pre>Unregisters origin from receiving notifications for IndexedDB.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct UntrackIndexedDbForOrigin;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.untrackIndexedDBForOrigin/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct UntrackIndexedDbForOriginResponse {}
impl<'de> serde::Deserialize<'de> for UntrackIndexedDbForOriginResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UntrackIndexedDbForOriginResponse;
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
                Ok(UntrackIndexedDbForOriginResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for UntrackIndexedDbForOrigin {
    type Params = UntrackIndexedDbForOriginParams;
    type Response = UntrackIndexedDbForOriginResponse;
    const METHOD: &'static str = "Storage.untrackIndexedDBForOrigin";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.untrackIndexedDBForStorageKey/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct UntrackIndexedDbForStorageKeyParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.untrackIndexedDBForStorageKey/parameters/storageKey。
    ///
    ///协议原文：
    ///
    ///<pre>Storage key.</pre>
    ///
    ///
    #[serde(rename = "storageKey")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_key: String,
}
impl<'de> serde::Deserialize<'de> for UntrackIndexedDbForStorageKeyParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UntrackIndexedDbForStorageKeyParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Storage.command.untrackIndexedDBForStorageKey/parameters/storageKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage key.</pre>
                    ///
                    ///
                    #[serde(rename = "storageKey")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_key: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(UntrackIndexedDbForStorageKeyParams {
                    storage_key: fields.storage_key,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Storage.command.untrackIndexedDBForStorageKey。
///
///协议原文：
///
///<pre>Unregisters storage key from receiving notifications for IndexedDB.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct UntrackIndexedDbForStorageKey;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Storage.command.untrackIndexedDBForStorageKey/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct UntrackIndexedDbForStorageKeyResponse {}
impl<'de> serde::Deserialize<'de> for UntrackIndexedDbForStorageKeyResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UntrackIndexedDbForStorageKeyResponse;
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
                Ok(UntrackIndexedDbForStorageKeyResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for UntrackIndexedDbForStorageKey {
    type Params = UntrackIndexedDbForStorageKeyParams;
    type Response = UntrackIndexedDbForStorageKeyResponse;
    const METHOD: &'static str = "Storage.untrackIndexedDBForStorageKey";
}
