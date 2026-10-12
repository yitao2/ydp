///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.canClearBrowserCache/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CanClearBrowserCacheParams {}
impl<'de> serde::Deserialize<'de> for CanClearBrowserCacheParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CanClearBrowserCacheParams;
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
                Ok(CanClearBrowserCacheParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.canClearBrowserCache。
///
///协议原文：
///
///<pre>Tells whether clearing browser cache is supported.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct CanClearBrowserCache;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.canClearBrowserCache/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CanClearBrowserCacheResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.canClearBrowserCache/returns/result。
    ///
    ///协议原文：
    ///
    ///<pre>True if browser cache can be cleared.</pre>
    ///
    ///
    #[serde(rename = "result")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub result: bool,
}
impl<'de> serde::Deserialize<'de> for CanClearBrowserCacheResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CanClearBrowserCacheResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.canClearBrowserCache/returns/result。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if browser cache can be cleared.</pre>
                    ///
                    ///
                    #[serde(rename = "result")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub result: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CanClearBrowserCacheResponse {
                    result: fields.result,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for CanClearBrowserCache {
    type Params = CanClearBrowserCacheParams;
    type Response = CanClearBrowserCacheResponse;
    const METHOD: &'static str = "Network.canClearBrowserCache";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.canClearBrowserCookies/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CanClearBrowserCookiesParams {}
impl<'de> serde::Deserialize<'de> for CanClearBrowserCookiesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CanClearBrowserCookiesParams;
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
                Ok(CanClearBrowserCookiesParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.canClearBrowserCookies。
///
///协议原文：
///
///<pre>Tells whether clearing browser cookies is supported.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct CanClearBrowserCookies;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.canClearBrowserCookies/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CanClearBrowserCookiesResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.canClearBrowserCookies/returns/result。
    ///
    ///协议原文：
    ///
    ///<pre>True if browser cookies can be cleared.</pre>
    ///
    ///
    #[serde(rename = "result")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub result: bool,
}
impl<'de> serde::Deserialize<'de> for CanClearBrowserCookiesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CanClearBrowserCookiesResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.canClearBrowserCookies/returns/result。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if browser cookies can be cleared.</pre>
                    ///
                    ///
                    #[serde(rename = "result")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub result: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CanClearBrowserCookiesResponse {
                    result: fields.result,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for CanClearBrowserCookies {
    type Params = CanClearBrowserCookiesParams;
    type Response = CanClearBrowserCookiesResponse;
    const METHOD: &'static str = "Network.canClearBrowserCookies";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.canEmulateNetworkConditions/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CanEmulateNetworkConditionsParams {}
impl<'de> serde::Deserialize<'de> for CanEmulateNetworkConditionsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CanEmulateNetworkConditionsParams;
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
                Ok(CanEmulateNetworkConditionsParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.canEmulateNetworkConditions。
///
///协议原文：
///
///<pre>Tells whether emulation of network conditions is supported.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct CanEmulateNetworkConditions;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.canEmulateNetworkConditions/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CanEmulateNetworkConditionsResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.canEmulateNetworkConditions/returns/result。
    ///
    ///协议原文：
    ///
    ///<pre>True if emulation of network conditions is supported.</pre>
    ///
    ///
    #[serde(rename = "result")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub result: bool,
}
impl<'de> serde::Deserialize<'de> for CanEmulateNetworkConditionsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CanEmulateNetworkConditionsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.canEmulateNetworkConditions/returns/result。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if emulation of network conditions is supported.</pre>
                    ///
                    ///
                    #[serde(rename = "result")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub result: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CanEmulateNetworkConditionsResponse {
                    result: fields.result,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for CanEmulateNetworkConditions {
    type Params = CanEmulateNetworkConditionsParams;
    type Response = CanEmulateNetworkConditionsResponse;
    const METHOD: &'static str = "Network.canEmulateNetworkConditions";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.clearBrowserCache/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ClearBrowserCacheParams {}
impl<'de> serde::Deserialize<'de> for ClearBrowserCacheParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearBrowserCacheParams;
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
                Ok(ClearBrowserCacheParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.clearBrowserCache。
///
///协议原文：
///
///<pre>Clears browser cache.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ClearBrowserCache;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.clearBrowserCache/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearBrowserCacheResponse {}
impl<'de> serde::Deserialize<'de> for ClearBrowserCacheResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearBrowserCacheResponse;
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
                Ok(ClearBrowserCacheResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ClearBrowserCache {
    type Params = ClearBrowserCacheParams;
    type Response = ClearBrowserCacheResponse;
    const METHOD: &'static str = "Network.clearBrowserCache";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.clearBrowserCookies/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ClearBrowserCookiesParams {}
impl<'de> serde::Deserialize<'de> for ClearBrowserCookiesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearBrowserCookiesParams;
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
                Ok(ClearBrowserCookiesParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.clearBrowserCookies。
///
///协议原文：
///
///<pre>Clears browser cookies.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ClearBrowserCookies;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.clearBrowserCookies/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearBrowserCookiesResponse {}
impl<'de> serde::Deserialize<'de> for ClearBrowserCookiesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearBrowserCookiesResponse;
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
                Ok(ClearBrowserCookiesResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ClearBrowserCookies {
    type Params = ClearBrowserCookiesParams;
    type Response = ClearBrowserCookiesResponse;
    const METHOD: &'static str = "Network.clearBrowserCookies";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.configureDurableMessages/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ConfigureDurableMessagesParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.configureDurableMessages/parameters/maxTotalBufferSize。
    ///
    ///协议原文：
    ///
    ///<pre>Buffer size in bytes to use when preserving network payloads (XHRs, etc).</pre>
    ///
    ///
    #[serde(rename = "maxTotalBufferSize")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_total_buffer_size: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.configureDurableMessages/parameters/maxResourceBufferSize。
    ///
    ///协议原文：
    ///
    ///<pre>Per-resource buffer size in bytes to use when preserving network payloads (XHRs, etc).</pre>
    ///
    ///
    #[serde(rename = "maxResourceBufferSize")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_resource_buffer_size: Option<i64>,
}
impl<'de> serde::Deserialize<'de> for ConfigureDurableMessagesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ConfigureDurableMessagesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.configureDurableMessages/parameters/maxTotalBufferSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Buffer size in bytes to use when preserving network payloads (XHRs, etc).</pre>
                    ///
                    ///
                    #[serde(rename = "maxTotalBufferSize")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub max_total_buffer_size: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.configureDurableMessages/parameters/maxResourceBufferSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Per-resource buffer size in bytes to use when preserving network payloads (XHRs, etc).</pre>
                    ///
                    ///
                    #[serde(rename = "maxResourceBufferSize")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub max_resource_buffer_size: Option<i64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ConfigureDurableMessagesParams {
                    max_total_buffer_size: fields.max_total_buffer_size,
                    max_resource_buffer_size: fields.max_resource_buffer_size,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.configureDurableMessages。
///
///协议原文：
///
///<pre>Configures storing response bodies outside of renderer, so that these survive
///a cross-process navigation.
///If maxTotalBufferSize is not set, durable messages are disabled.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ConfigureDurableMessages;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.configureDurableMessages/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ConfigureDurableMessagesResponse {}
impl<'de> serde::Deserialize<'de> for ConfigureDurableMessagesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ConfigureDurableMessagesResponse;
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
                Ok(ConfigureDurableMessagesResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ConfigureDurableMessages {
    type Params = ConfigureDurableMessagesParams;
    type Response = ConfigureDurableMessagesResponse;
    const METHOD: &'static str = "Network.configureDurableMessages";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.deleteCookies/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeleteCookiesParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.deleteCookies/parameters/name。
    ///
    ///协议原文：
    ///
    ///<pre>Name of the cookies to remove.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.deleteCookies/parameters/url。
    ///
    ///协议原文：
    ///
    ///<pre>If specified, deletes all the cookies with the given name where domain and path match
    ///provided URL.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.deleteCookies/parameters/domain。
    ///
    ///协议原文：
    ///
    ///<pre>If specified, deletes only cookies with the exact domain.</pre>
    ///
    ///
    #[serde(rename = "domain")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub domain: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.deleteCookies/parameters/path。
    ///
    ///协议原文：
    ///
    ///<pre>If specified, deletes only cookies with the exact path.</pre>
    ///
    ///
    #[serde(rename = "path")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub path: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.deleteCookies/parameters/partitionKey。
    ///
    ///协议原文：
    ///
    ///<pre>If specified, deletes only cookies with the the given name and partitionKey where
    ///all partition key attributes match the cookie partition key attribute.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "partitionKey")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub partition_key: Option<crate::network::types::CookiePartitionKey>,
}
impl<'de> serde::Deserialize<'de> for DeleteCookiesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeleteCookiesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.deleteCookies/parameters/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Name of the cookies to remove.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.deleteCookies/parameters/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If specified, deletes all the cookies with the given name where domain and path match
                    ///provided URL.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub url: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.deleteCookies/parameters/domain。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If specified, deletes only cookies with the exact domain.</pre>
                    ///
                    ///
                    #[serde(rename = "domain")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub domain: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.deleteCookies/parameters/path。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If specified, deletes only cookies with the exact path.</pre>
                    ///
                    ///
                    #[serde(rename = "path")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub path: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.deleteCookies/parameters/partitionKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If specified, deletes only cookies with the the given name and partitionKey where
                    ///all partition key attributes match the cookie partition key attribute.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "partitionKey")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub partition_key: Option<crate::network::types::CookiePartitionKey>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DeleteCookiesParams {
                    name: fields.name,
                    url: fields.url,
                    domain: fields.domain,
                    path: fields.path,
                    partition_key: fields.partition_key,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.deleteCookies。
///
///协议原文：
///
///<pre>Deletes browser cookies with matching name and url or domain/path/partitionKey pair.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct DeleteCookies;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.deleteCookies/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeleteCookiesResponse {}
impl<'de> serde::Deserialize<'de> for DeleteCookiesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeleteCookiesResponse;
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
                Ok(DeleteCookiesResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for DeleteCookies {
    type Params = DeleteCookiesParams;
    type Response = DeleteCookiesResponse;
    const METHOD: &'static str = "Network.deleteCookies";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.deleteDeviceBoundSession/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeleteDeviceBoundSessionParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.deleteDeviceBoundSession/parameters/key。
    #[serde(rename = "key")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub key: crate::network::types::DeviceBoundSessionKey,
}
impl<'de> serde::Deserialize<'de> for DeleteDeviceBoundSessionParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeleteDeviceBoundSessionParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.deleteDeviceBoundSession/parameters/key。
                    #[serde(rename = "key")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub key: crate::network::types::DeviceBoundSessionKey,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DeleteDeviceBoundSessionParams { key: fields.key })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.deleteDeviceBoundSession。
///
///协议原文：
///
///<pre>Deletes a device bound session.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct DeleteDeviceBoundSession;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.deleteDeviceBoundSession/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeleteDeviceBoundSessionResponse {}
impl<'de> serde::Deserialize<'de> for DeleteDeviceBoundSessionResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeleteDeviceBoundSessionResponse;
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
                Ok(DeleteDeviceBoundSessionResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for DeleteDeviceBoundSession {
    type Params = DeleteDeviceBoundSessionParams;
    type Response = DeleteDeviceBoundSessionResponse;
    const METHOD: &'static str = "Network.deleteDeviceBoundSession";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.disable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.disable。
///
///协议原文：
///
///<pre>Disables network tracking, prevents network events from being sent to the client.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.disable/returns。
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
    const METHOD: &'static str = "Network.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.emulateNetworkConditions/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EmulateNetworkConditionsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.emulateNetworkConditions/parameters/offline。
    ///
    ///协议原文：
    ///
    ///<pre>True to emulate internet disconnection.</pre>
    ///
    ///
    #[serde(rename = "offline")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub offline: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.emulateNetworkConditions/parameters/latency。
    ///
    ///协议原文：
    ///
    ///<pre>Minimum latency from request sent to response headers received (ms).</pre>
    ///
    ///
    #[serde(rename = "latency")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub latency: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.emulateNetworkConditions/parameters/downloadThroughput。
    ///
    ///协议原文：
    ///
    ///<pre>Maximal aggregated download throughput (bytes/sec). -1 disables download throttling.</pre>
    ///
    ///
    #[serde(rename = "downloadThroughput")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub download_throughput: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.emulateNetworkConditions/parameters/uploadThroughput。
    ///
    ///协议原文：
    ///
    ///<pre>Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling.</pre>
    ///
    ///
    #[serde(rename = "uploadThroughput")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub upload_throughput: ydp_core::FiniteF64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.emulateNetworkConditions/parameters/connectionType。
    ///
    ///协议原文：
    ///
    ///<pre>Connection type if known.</pre>
    ///
    ///
    #[serde(rename = "connectionType")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub connection_type: Option<crate::network::types::ConnectionType>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.emulateNetworkConditions/parameters/packetLoss。
    ///
    ///协议原文：
    ///
    ///<pre>WebRTC packet loss (percent, 0-100). 0 disables packet loss emulation, 100 drops all the packets.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "packetLoss")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub packet_loss: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.emulateNetworkConditions/parameters/packetQueueLength。
    ///
    ///协议原文：
    ///
    ///<pre>WebRTC packet queue length (packet). 0 removes any queue length limitations.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "packetQueueLength")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub packet_queue_length: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.emulateNetworkConditions/parameters/packetReordering。
    ///
    ///协议原文：
    ///
    ///<pre>WebRTC packetReordering feature.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "packetReordering")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub packet_reordering: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for EmulateNetworkConditionsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EmulateNetworkConditionsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.emulateNetworkConditions/parameters/offline。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True to emulate internet disconnection.</pre>
                    ///
                    ///
                    #[serde(rename = "offline")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub offline: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.emulateNetworkConditions/parameters/latency。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Minimum latency from request sent to response headers received (ms).</pre>
                    ///
                    ///
                    #[serde(rename = "latency")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub latency: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.emulateNetworkConditions/parameters/downloadThroughput。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Maximal aggregated download throughput (bytes/sec). -1 disables download throttling.</pre>
                    ///
                    ///
                    #[serde(rename = "downloadThroughput")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub download_throughput: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.emulateNetworkConditions/parameters/uploadThroughput。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling.</pre>
                    ///
                    ///
                    #[serde(rename = "uploadThroughput")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub upload_throughput: ydp_core::FiniteF64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.emulateNetworkConditions/parameters/connectionType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Connection type if known.</pre>
                    ///
                    ///
                    #[serde(rename = "connectionType")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub connection_type: Option<crate::network::types::ConnectionType>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.emulateNetworkConditions/parameters/packetLoss。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>WebRTC packet loss (percent, 0-100). 0 disables packet loss emulation, 100 drops all the packets.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "packetLoss")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub packet_loss: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.emulateNetworkConditions/parameters/packetQueueLength。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>WebRTC packet queue length (packet). 0 removes any queue length limitations.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "packetQueueLength")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub packet_queue_length: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.emulateNetworkConditions/parameters/packetReordering。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>WebRTC packetReordering feature.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "packetReordering")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub packet_reordering: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EmulateNetworkConditionsParams {
                    offline: fields.offline,
                    latency: fields.latency,
                    download_throughput: fields.download_throughput,
                    upload_throughput: fields.upload_throughput,
                    connection_type: fields.connection_type,
                    packet_loss: fields.packet_loss,
                    packet_queue_length: fields.packet_queue_length,
                    packet_reordering: fields.packet_reordering,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.emulateNetworkConditions。
///
///协议原文：
///
///<pre>Activates emulation of network conditions. This command is deprecated in favor of the emulateNetworkConditionsByRule
///and overrideNetworkState commands, which can be used together to the same effect.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct EmulateNetworkConditions;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.emulateNetworkConditions/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EmulateNetworkConditionsResponse {}
impl<'de> serde::Deserialize<'de> for EmulateNetworkConditionsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EmulateNetworkConditionsResponse;
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
                Ok(EmulateNetworkConditionsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for EmulateNetworkConditions {
    type Params = EmulateNetworkConditionsParams;
    type Response = EmulateNetworkConditionsResponse;
    const METHOD: &'static str = "Network.emulateNetworkConditions";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.emulateNetworkConditionsByRule/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EmulateNetworkConditionsByRuleParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.emulateNetworkConditionsByRule/parameters/offline。
    ///
    ///协议原文：
    ///
    ///<pre>True to emulate internet disconnection. Deprecated, use the offline property in matchedNetworkConditions
    ///or emulateOfflineServiceWorker instead.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "offline")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub offline: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.emulateNetworkConditionsByRule/parameters/emulateOfflineServiceWorker。
    ///
    ///协议原文：
    ///
    ///<pre>True to emulate offline service worker.</pre>
    ///
    ///
    #[serde(rename = "emulateOfflineServiceWorker")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub emulate_offline_service_worker: Option<bool>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.emulateNetworkConditionsByRule/parameters/matchedNetworkConditions。
    ///
    ///协议原文：
    ///
    ///<pre>Configure conditions for matching requests. If multiple entries match a request, the first entry wins.  Global
    ///conditions can be configured by leaving the urlPattern for the conditions empty. These global conditions are
    ///also applied for throttling of p2p connections.</pre>
    ///
    ///
    #[serde(rename = "matchedNetworkConditions")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub matched_network_conditions: Vec<crate::network::types::NetworkConditions>,
}
impl<'de> serde::Deserialize<'de> for EmulateNetworkConditionsByRuleParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EmulateNetworkConditionsByRuleParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.emulateNetworkConditionsByRule/parameters/offline。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True to emulate internet disconnection. Deprecated, use the offline property in matchedNetworkConditions
                    ///or emulateOfflineServiceWorker instead.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "offline")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub offline: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.emulateNetworkConditionsByRule/parameters/emulateOfflineServiceWorker。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True to emulate offline service worker.</pre>
                    ///
                    ///
                    #[serde(rename = "emulateOfflineServiceWorker")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub emulate_offline_service_worker: Option<bool>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.emulateNetworkConditionsByRule/parameters/matchedNetworkConditions。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Configure conditions for matching requests. If multiple entries match a request, the first entry wins.  Global
                    ///conditions can be configured by leaving the urlPattern for the conditions empty. These global conditions are
                    ///also applied for throttling of p2p connections.</pre>
                    ///
                    ///
                    #[serde(rename = "matchedNetworkConditions")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub matched_network_conditions: Vec<crate::network::types::NetworkConditions>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EmulateNetworkConditionsByRuleParams {
                    offline: fields.offline,
                    emulate_offline_service_worker: fields.emulate_offline_service_worker,
                    matched_network_conditions: fields.matched_network_conditions,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.emulateNetworkConditionsByRule。
///
///协议原文：
///
///<pre>Activates emulation of network conditions for individual requests using URL match patterns. Unlike the deprecated
///Network.emulateNetworkConditions this method does not affect `navigator` state. Use Network.overrideNetworkState to
///explicitly modify `navigator` behavior.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct EmulateNetworkConditionsByRule;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.emulateNetworkConditionsByRule/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EmulateNetworkConditionsByRuleResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.emulateNetworkConditionsByRule/returns/ruleIds。
    ///
    ///协议原文：
    ///
    ///<pre>An id for each entry in matchedNetworkConditions. The id will be included in the requestWillBeSentExtraInfo for
    ///requests affected by a rule.</pre>
    ///
    ///
    #[serde(rename = "ruleIds")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub rule_ids: Vec<String>,
}
impl<'de> serde::Deserialize<'de> for EmulateNetworkConditionsByRuleResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EmulateNetworkConditionsByRuleResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.emulateNetworkConditionsByRule/returns/ruleIds。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An id for each entry in matchedNetworkConditions. The id will be included in the requestWillBeSentExtraInfo for
                    ///requests affected by a rule.</pre>
                    ///
                    ///
                    #[serde(rename = "ruleIds")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub rule_ids: Vec<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EmulateNetworkConditionsByRuleResponse {
                    rule_ids: fields.rule_ids,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for EmulateNetworkConditionsByRule {
    type Params = EmulateNetworkConditionsByRuleParams;
    type Response = EmulateNetworkConditionsByRuleResponse;
    const METHOD: &'static str = "Network.emulateNetworkConditionsByRule";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.enable/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct EnableParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.enable/parameters/maxTotalBufferSize。
    ///
    ///协议原文：
    ///
    ///<pre>Buffer size in bytes to use when preserving network payloads (XHRs, etc).
    ///This is the maximum number of bytes that will be collected by this
    ///DevTools session.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "maxTotalBufferSize")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_total_buffer_size: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.enable/parameters/maxResourceBufferSize。
    ///
    ///协议原文：
    ///
    ///<pre>Per-resource buffer size in bytes to use when preserving network payloads (XHRs, etc).</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "maxResourceBufferSize")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_resource_buffer_size: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.enable/parameters/maxPostDataSize。
    ///
    ///协议原文：
    ///
    ///<pre>Longest post body size (in bytes) that would be included in requestWillBeSent notification</pre>
    ///
    ///
    #[serde(rename = "maxPostDataSize")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_post_data_size: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.enable/parameters/reportDirectSocketTraffic。
    ///
    ///协议原文：
    ///
    ///<pre>Whether DirectSocket chunk send/receive events should be reported.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "reportDirectSocketTraffic")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub report_direct_socket_traffic: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.enable/parameters/enableDurableMessages。
    ///
    ///协议原文：
    ///
    ///<pre>Enable storing response bodies outside of renderer, so that these survive
    ///a cross-process navigation. Requires maxTotalBufferSize to be set.
    ///Currently defaults to false. This field is being deprecated in favor of the dedicated
    ///configureDurableMessages command, due to the possibility of deadlocks when awaiting
    ///Network.enable before issuing Runtime.runIfWaitingForDebugger.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "enableDurableMessages")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_durable_messages: Option<bool>,
}
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
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.enable/parameters/maxTotalBufferSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Buffer size in bytes to use when preserving network payloads (XHRs, etc).
                    ///This is the maximum number of bytes that will be collected by this
                    ///DevTools session.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "maxTotalBufferSize")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub max_total_buffer_size: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.enable/parameters/maxResourceBufferSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Per-resource buffer size in bytes to use when preserving network payloads (XHRs, etc).</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "maxResourceBufferSize")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub max_resource_buffer_size: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.enable/parameters/maxPostDataSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Longest post body size (in bytes) that would be included in requestWillBeSent notification</pre>
                    ///
                    ///
                    #[serde(rename = "maxPostDataSize")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub max_post_data_size: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.enable/parameters/reportDirectSocketTraffic。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether DirectSocket chunk send/receive events should be reported.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "reportDirectSocketTraffic")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub report_direct_socket_traffic: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.enable/parameters/enableDurableMessages。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Enable storing response bodies outside of renderer, so that these survive
                    ///a cross-process navigation. Requires maxTotalBufferSize to be set.
                    ///Currently defaults to false. This field is being deprecated in favor of the dedicated
                    ///configureDurableMessages command, due to the possibility of deadlocks when awaiting
                    ///Network.enable before issuing Runtime.runIfWaitingForDebugger.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "enableDurableMessages")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub enable_durable_messages: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EnableParams {
                    max_total_buffer_size: fields.max_total_buffer_size,
                    max_resource_buffer_size: fields.max_resource_buffer_size,
                    max_post_data_size: fields.max_post_data_size,
                    report_direct_socket_traffic: fields.report_direct_socket_traffic,
                    enable_durable_messages: fields.enable_durable_messages,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.enable。
///
///协议原文：
///
///<pre>Enables network tracking, network events will now be delivered to the client.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.enable/returns。
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
    const METHOD: &'static str = "Network.enable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.enableDeviceBoundSessions/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EnableDeviceBoundSessionsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.enableDeviceBoundSessions/parameters/enable。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to enable or disable events.</pre>
    ///
    ///
    #[serde(rename = "enable")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub enable: bool,
}
impl<'de> serde::Deserialize<'de> for EnableDeviceBoundSessionsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EnableDeviceBoundSessionsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.enableDeviceBoundSessions/parameters/enable。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to enable or disable events.</pre>
                    ///
                    ///
                    #[serde(rename = "enable")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub enable: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EnableDeviceBoundSessionsParams {
                    enable: fields.enable,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.enableDeviceBoundSessions。
///
///协议原文：
///
///<pre>Sets up tracking device bound sessions and fetching of initial set of sessions.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct EnableDeviceBoundSessions;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.enableDeviceBoundSessions/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EnableDeviceBoundSessionsResponse {}
impl<'de> serde::Deserialize<'de> for EnableDeviceBoundSessionsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EnableDeviceBoundSessionsResponse;
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
                Ok(EnableDeviceBoundSessionsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for EnableDeviceBoundSessions {
    type Params = EnableDeviceBoundSessionsParams;
    type Response = EnableDeviceBoundSessionsResponse;
    const METHOD: &'static str = "Network.enableDeviceBoundSessions";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.enableReportingApi/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EnableReportingApiParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.enableReportingApi/parameters/enable。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to enable or disable events for the Reporting API</pre>
    ///
    ///
    #[serde(rename = "enable")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub enable: bool,
}
impl<'de> serde::Deserialize<'de> for EnableReportingApiParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EnableReportingApiParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.enableReportingApi/parameters/enable。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to enable or disable events for the Reporting API</pre>
                    ///
                    ///
                    #[serde(rename = "enable")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub enable: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EnableReportingApiParams {
                    enable: fields.enable,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.enableReportingApi。
///
///协议原文：
///
///<pre>Enables tracking for the Reporting API, events generated by the Reporting API will now be delivered to the client.
///Enabling triggers 'reportingApiReportAdded' for all existing reports.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct EnableReportingApi;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.enableReportingApi/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EnableReportingApiResponse {}
impl<'de> serde::Deserialize<'de> for EnableReportingApiResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EnableReportingApiResponse;
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
                Ok(EnableReportingApiResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for EnableReportingApi {
    type Params = EnableReportingApiParams;
    type Response = EnableReportingApiResponse;
    const METHOD: &'static str = "Network.enableReportingApi";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.fetchSchemefulSite/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FetchSchemefulSiteParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.fetchSchemefulSite/parameters/origin。
    ///
    ///协议原文：
    ///
    ///<pre>The URL origin.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: String,
}
impl<'de> serde::Deserialize<'de> for FetchSchemefulSiteParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FetchSchemefulSiteParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.fetchSchemefulSite/parameters/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The URL origin.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FetchSchemefulSiteParams {
                    origin: fields.origin,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.fetchSchemefulSite。
///
///协议原文：
///
///<pre>Fetches the schemeful site for a specific origin.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct FetchSchemefulSite;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.fetchSchemefulSite/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FetchSchemefulSiteResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.fetchSchemefulSite/returns/schemefulSite。
    ///
    ///协议原文：
    ///
    ///<pre>The corresponding schemeful site.</pre>
    ///
    ///
    #[serde(rename = "schemefulSite")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub schemeful_site: String,
}
impl<'de> serde::Deserialize<'de> for FetchSchemefulSiteResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FetchSchemefulSiteResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.fetchSchemefulSite/returns/schemefulSite。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The corresponding schemeful site.</pre>
                    ///
                    ///
                    #[serde(rename = "schemefulSite")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub schemeful_site: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FetchSchemefulSiteResponse {
                    schemeful_site: fields.schemeful_site,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for FetchSchemefulSite {
    type Params = FetchSchemefulSiteParams;
    type Response = FetchSchemefulSiteResponse;
    const METHOD: &'static str = "Network.fetchSchemefulSite";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.getAllCookies/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetAllCookiesParams {}
impl<'de> serde::Deserialize<'de> for GetAllCookiesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetAllCookiesParams;
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
                Ok(GetAllCookiesParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.getAllCookies。
///
///协议原文：
///
///<pre>Returns all browser cookies. Depending on the backend support, will return detailed cookie
///information in the `cookies` field.
///Deprecated. Use Storage.getCookies instead.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct GetAllCookies;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.getAllCookies/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetAllCookiesResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.getAllCookies/returns/cookies。
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
impl<'de> serde::Deserialize<'de> for GetAllCookiesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetAllCookiesResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.getAllCookies/returns/cookies。
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
                Ok(GetAllCookiesResponse {
                    cookies: fields.cookies,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetAllCookies {
    type Params = GetAllCookiesParams;
    type Response = GetAllCookiesResponse;
    const METHOD: &'static str = "Network.getAllCookies";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.getCertificate/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetCertificateParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.getCertificate/parameters/origin。
    ///
    ///协议原文：
    ///
    ///<pre>Origin to get certificate for.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: String,
}
impl<'de> serde::Deserialize<'de> for GetCertificateParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetCertificateParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.getCertificate/parameters/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Origin to get certificate for.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetCertificateParams {
                    origin: fields.origin,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.getCertificate。
///
///协议原文：
///
///<pre>Returns the DER-encoded certificate.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetCertificate;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.getCertificate/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetCertificateResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.getCertificate/returns/tableNames。
    #[serde(rename = "tableNames")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub table_names: Vec<String>,
}
impl<'de> serde::Deserialize<'de> for GetCertificateResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetCertificateResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.getCertificate/returns/tableNames。
                    #[serde(rename = "tableNames")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub table_names: Vec<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetCertificateResponse {
                    table_names: fields.table_names,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetCertificate {
    type Params = GetCertificateParams;
    type Response = GetCertificateResponse;
    const METHOD: &'static str = "Network.getCertificate";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.getCookies/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetCookiesParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.getCookies/parameters/urls。
    ///
    ///协议原文：
    ///
    ///<pre>The list of URLs for which applicable cookies will be fetched.
    ///If not specified, it's assumed to be set to the list containing
    ///the URLs of the page and all of its subframes.</pre>
    ///
    ///
    #[serde(rename = "urls")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub urls: Option<Vec<String>>,
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
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.getCookies/parameters/urls。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The list of URLs for which applicable cookies will be fetched.
                    ///If not specified, it's assumed to be set to the list containing
                    ///the URLs of the page and all of its subframes.</pre>
                    ///
                    ///
                    #[serde(rename = "urls")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub urls: Option<Vec<String>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetCookiesParams { urls: fields.urls })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.getCookies。
///
///协议原文：
///
///<pre>Returns all browser cookies for the current URL. Depending on the backend support, will return
///detailed cookie information in the `cookies` field.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetCookies;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.getCookies/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetCookiesResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.getCookies/returns/cookies。
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
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.getCookies/returns/cookies。
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
    const METHOD: &'static str = "Network.getCookies";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.getRequestPostData/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetRequestPostDataParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.getRequestPostData/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the network request to get content for.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::network::types::RequestId,
}
impl<'de> serde::Deserialize<'de> for GetRequestPostDataParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetRequestPostDataParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.getRequestPostData/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the network request to get content for.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::network::types::RequestId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetRequestPostDataParams {
                    request_id: fields.request_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.getRequestPostData。
///
///协议原文：
///
///<pre>Returns post data sent with the request. Returns an error when no data was sent with the request.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetRequestPostData;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.getRequestPostData/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetRequestPostDataResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.getRequestPostData/returns/postData。
    ///
    ///协议原文：
    ///
    ///<pre>Request body string, omitting files from multipart requests</pre>
    ///
    ///
    #[serde(rename = "postData")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub post_data: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.getRequestPostData/returns/base64Encoded。
    ///
    ///协议原文：
    ///
    ///<pre>True, if content was sent as base64.</pre>
    ///
    ///
    #[serde(rename = "base64Encoded")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub base64_encoded: bool,
}
impl<'de> serde::Deserialize<'de> for GetRequestPostDataResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetRequestPostDataResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.getRequestPostData/returns/postData。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request body string, omitting files from multipart requests</pre>
                    ///
                    ///
                    #[serde(rename = "postData")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub post_data: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.getRequestPostData/returns/base64Encoded。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True, if content was sent as base64.</pre>
                    ///
                    ///
                    #[serde(rename = "base64Encoded")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub base64_encoded: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetRequestPostDataResponse {
                    post_data: fields.post_data,
                    base64_encoded: fields.base64_encoded,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetRequestPostData {
    type Params = GetRequestPostDataParams;
    type Response = GetRequestPostDataResponse;
    const METHOD: &'static str = "Network.getRequestPostData";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.getResponseBody/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetResponseBodyParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.getResponseBody/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the network request to get content for.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::network::types::RequestId,
}
impl<'de> serde::Deserialize<'de> for GetResponseBodyParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetResponseBodyParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.getResponseBody/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the network request to get content for.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::network::types::RequestId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetResponseBodyParams {
                    request_id: fields.request_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.getResponseBody。
///
///协议原文：
///
///<pre>Returns content served for the given request.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetResponseBody;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.getResponseBody/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetResponseBodyResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.getResponseBody/returns/body。
    ///
    ///协议原文：
    ///
    ///<pre>Response body.</pre>
    ///
    ///
    #[serde(rename = "body")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub body: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.getResponseBody/returns/base64Encoded。
    ///
    ///协议原文：
    ///
    ///<pre>True, if content was sent as base64.</pre>
    ///
    ///
    #[serde(rename = "base64Encoded")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub base64_encoded: bool,
}
impl<'de> serde::Deserialize<'de> for GetResponseBodyResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetResponseBodyResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.getResponseBody/returns/body。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Response body.</pre>
                    ///
                    ///
                    #[serde(rename = "body")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub body: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.getResponseBody/returns/base64Encoded。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True, if content was sent as base64.</pre>
                    ///
                    ///
                    #[serde(rename = "base64Encoded")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub base64_encoded: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetResponseBodyResponse {
                    body: fields.body,
                    base64_encoded: fields.base64_encoded,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetResponseBody {
    type Params = GetResponseBodyParams;
    type Response = GetResponseBodyResponse;
    const METHOD: &'static str = "Network.getResponseBody";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.getSecurityIsolationStatus/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetSecurityIsolationStatusParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.getSecurityIsolationStatus/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>If no frameId is provided, the status of the target is provided.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub frame_id: Option<crate::page::types::FrameId>,
}
impl<'de> serde::Deserialize<'de> for GetSecurityIsolationStatusParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetSecurityIsolationStatusParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.getSecurityIsolationStatus/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If no frameId is provided, the status of the target is provided.</pre>
                    ///
                    ///
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
                Ok(GetSecurityIsolationStatusParams {
                    frame_id: fields.frame_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.getSecurityIsolationStatus。
///
///协议原文：
///
///<pre>Returns information about the COEP/COOP isolation status.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetSecurityIsolationStatus;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.getSecurityIsolationStatus/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetSecurityIsolationStatusResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.getSecurityIsolationStatus/returns/status。
    #[serde(rename = "status")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub status: crate::network::types::SecurityIsolationStatus,
}
impl<'de> serde::Deserialize<'de> for GetSecurityIsolationStatusResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetSecurityIsolationStatusResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.getSecurityIsolationStatus/returns/status。
                    #[serde(rename = "status")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub status: crate::network::types::SecurityIsolationStatus,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetSecurityIsolationStatusResponse {
                    status: fields.status,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetSecurityIsolationStatus {
    type Params = GetSecurityIsolationStatusParams;
    type Response = GetSecurityIsolationStatusResponse;
    const METHOD: &'static str = "Network.getSecurityIsolationStatus";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.loadNetworkResource/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct LoadNetworkResourceParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.loadNetworkResource/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Frame id to get the resource for. Mandatory for frame targets, and
    ///should be omitted for worker targets.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub frame_id: Option<crate::page::types::FrameId>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.loadNetworkResource/parameters/url。
    ///
    ///协议原文：
    ///
    ///<pre>URL of the resource to get content for.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.loadNetworkResource/parameters/options。
    ///
    ///协议原文：
    ///
    ///<pre>Options for the request.</pre>
    ///
    ///
    #[serde(rename = "options")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub options: crate::network::types::LoadNetworkResourceOptions,
}
impl<'de> serde::Deserialize<'de> for LoadNetworkResourceParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LoadNetworkResourceParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.loadNetworkResource/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame id to get the resource for. Mandatory for frame targets, and
                    ///should be omitted for worker targets.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub frame_id: Option<crate::page::types::FrameId>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.loadNetworkResource/parameters/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>URL of the resource to get content for.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.loadNetworkResource/parameters/options。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Options for the request.</pre>
                    ///
                    ///
                    #[serde(rename = "options")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub options: crate::network::types::LoadNetworkResourceOptions,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LoadNetworkResourceParams {
                    frame_id: fields.frame_id,
                    url: fields.url,
                    options: fields.options,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.loadNetworkResource。
///
///协议原文：
///
///<pre>Fetches the resource and returns the content.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct LoadNetworkResource;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.loadNetworkResource/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct LoadNetworkResourceResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.loadNetworkResource/returns/resource。
    #[serde(rename = "resource")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub resource: crate::network::types::LoadNetworkResourcePageResult,
}
impl<'de> serde::Deserialize<'de> for LoadNetworkResourceResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LoadNetworkResourceResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.loadNetworkResource/returns/resource。
                    #[serde(rename = "resource")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub resource: crate::network::types::LoadNetworkResourcePageResult,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LoadNetworkResourceResponse {
                    resource: fields.resource,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for LoadNetworkResource {
    type Params = LoadNetworkResourceParams;
    type Response = LoadNetworkResourceResponse;
    const METHOD: &'static str = "Network.loadNetworkResource";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.overrideNetworkState/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct OverrideNetworkStateParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.overrideNetworkState/parameters/offline。
    ///
    ///协议原文：
    ///
    ///<pre>True to emulate internet disconnection.</pre>
    ///
    ///
    #[serde(rename = "offline")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub offline: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.overrideNetworkState/parameters/latency。
    ///
    ///协议原文：
    ///
    ///<pre>Minimum latency from request sent to response headers received (ms).</pre>
    ///
    ///
    #[serde(rename = "latency")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub latency: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.overrideNetworkState/parameters/downloadThroughput。
    ///
    ///协议原文：
    ///
    ///<pre>Maximal aggregated download throughput (bytes/sec). -1 disables download throttling.</pre>
    ///
    ///
    #[serde(rename = "downloadThroughput")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub download_throughput: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.overrideNetworkState/parameters/uploadThroughput。
    ///
    ///协议原文：
    ///
    ///<pre>Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling.</pre>
    ///
    ///
    #[serde(rename = "uploadThroughput")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub upload_throughput: ydp_core::FiniteF64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.overrideNetworkState/parameters/connectionType。
    ///
    ///协议原文：
    ///
    ///<pre>Connection type if known.</pre>
    ///
    ///
    #[serde(rename = "connectionType")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub connection_type: Option<crate::network::types::ConnectionType>,
}
impl<'de> serde::Deserialize<'de> for OverrideNetworkStateParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = OverrideNetworkStateParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.overrideNetworkState/parameters/offline。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True to emulate internet disconnection.</pre>
                    ///
                    ///
                    #[serde(rename = "offline")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub offline: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.overrideNetworkState/parameters/latency。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Minimum latency from request sent to response headers received (ms).</pre>
                    ///
                    ///
                    #[serde(rename = "latency")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub latency: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.overrideNetworkState/parameters/downloadThroughput。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Maximal aggregated download throughput (bytes/sec). -1 disables download throttling.</pre>
                    ///
                    ///
                    #[serde(rename = "downloadThroughput")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub download_throughput: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.overrideNetworkState/parameters/uploadThroughput。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling.</pre>
                    ///
                    ///
                    #[serde(rename = "uploadThroughput")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub upload_throughput: ydp_core::FiniteF64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.overrideNetworkState/parameters/connectionType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Connection type if known.</pre>
                    ///
                    ///
                    #[serde(rename = "connectionType")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub connection_type: Option<crate::network::types::ConnectionType>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(OverrideNetworkStateParams {
                    offline: fields.offline,
                    latency: fields.latency,
                    download_throughput: fields.download_throughput,
                    upload_throughput: fields.upload_throughput,
                    connection_type: fields.connection_type,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.overrideNetworkState。
///
///协议原文：
///
///<pre>Override the state of navigator.onLine and navigator.connection.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct OverrideNetworkState;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.overrideNetworkState/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct OverrideNetworkStateResponse {}
impl<'de> serde::Deserialize<'de> for OverrideNetworkStateResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = OverrideNetworkStateResponse;
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
                Ok(OverrideNetworkStateResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for OverrideNetworkState {
    type Params = OverrideNetworkStateParams;
    type Response = OverrideNetworkStateResponse;
    const METHOD: &'static str = "Network.overrideNetworkState";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.replayXHR/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReplayXhrParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.replayXHR/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of XHR to replay.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::network::types::RequestId,
}
impl<'de> serde::Deserialize<'de> for ReplayXhrParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReplayXhrParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.replayXHR/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of XHR to replay.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::network::types::RequestId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReplayXhrParams {
                    request_id: fields.request_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.replayXHR。
///
///协议原文：
///
///<pre>This method sends a new XMLHttpRequest which is identical to the original one. The following
///parameters should be identical: method, url, async, request body, extra headers, withCredentials
///attribute, user, password.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ReplayXhr;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.replayXHR/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReplayXhrResponse {}
impl<'de> serde::Deserialize<'de> for ReplayXhrResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReplayXhrResponse;
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
                Ok(ReplayXhrResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ReplayXhr {
    type Params = ReplayXhrParams;
    type Response = ReplayXhrResponse;
    const METHOD: &'static str = "Network.replayXHR";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.searchInResponseBody/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SearchInResponseBodyParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.searchInResponseBody/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the network response to search.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.searchInResponseBody/parameters/query。
    ///
    ///协议原文：
    ///
    ///<pre>String to search for.</pre>
    ///
    ///
    #[serde(rename = "query")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub query: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.searchInResponseBody/parameters/caseSensitive。
    ///
    ///协议原文：
    ///
    ///<pre>If true, search is case sensitive.</pre>
    ///
    ///
    #[serde(rename = "caseSensitive")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub case_sensitive: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.searchInResponseBody/parameters/isRegex。
    ///
    ///协议原文：
    ///
    ///<pre>If true, treats string parameter as regex.</pre>
    ///
    ///
    #[serde(rename = "isRegex")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_regex: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for SearchInResponseBodyParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SearchInResponseBodyParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.searchInResponseBody/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the network response to search.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.searchInResponseBody/parameters/query。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>String to search for.</pre>
                    ///
                    ///
                    #[serde(rename = "query")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub query: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.searchInResponseBody/parameters/caseSensitive。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, search is case sensitive.</pre>
                    ///
                    ///
                    #[serde(rename = "caseSensitive")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub case_sensitive: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.searchInResponseBody/parameters/isRegex。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, treats string parameter as regex.</pre>
                    ///
                    ///
                    #[serde(rename = "isRegex")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub is_regex: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SearchInResponseBodyParams {
                    request_id: fields.request_id,
                    query: fields.query,
                    case_sensitive: fields.case_sensitive,
                    is_regex: fields.is_regex,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.searchInResponseBody。
///
///协议原文：
///
///<pre>Searches for given string in response content.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SearchInResponseBody;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.searchInResponseBody/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SearchInResponseBodyResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.searchInResponseBody/returns/result。
    ///
    ///协议原文：
    ///
    ///<pre>List of search matches.</pre>
    ///
    ///
    #[serde(rename = "result")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub result: Vec<crate::debugger::types::SearchMatch>,
}
impl<'de> serde::Deserialize<'de> for SearchInResponseBodyResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SearchInResponseBodyResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.searchInResponseBody/returns/result。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>List of search matches.</pre>
                    ///
                    ///
                    #[serde(rename = "result")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub result: Vec<crate::debugger::types::SearchMatch>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SearchInResponseBodyResponse {
                    result: fields.result,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SearchInResponseBody {
    type Params = SearchInResponseBodyParams;
    type Response = SearchInResponseBodyResponse;
    const METHOD: &'static str = "Network.searchInResponseBody";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.setAttachDebugStack/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetAttachDebugStackParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.setAttachDebugStack/parameters/enabled。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to attach a page script stack for debugging purpose.</pre>
    ///
    ///
    #[serde(rename = "enabled")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub enabled: bool,
}
impl<'de> serde::Deserialize<'de> for SetAttachDebugStackParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetAttachDebugStackParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.setAttachDebugStack/parameters/enabled。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to attach a page script stack for debugging purpose.</pre>
                    ///
                    ///
                    #[serde(rename = "enabled")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub enabled: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetAttachDebugStackParams {
                    enabled: fields.enabled,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.setAttachDebugStack。
///
///协议原文：
///
///<pre>Specifies whether to attach a page script stack id in requests</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetAttachDebugStack;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.setAttachDebugStack/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetAttachDebugStackResponse {}
impl<'de> serde::Deserialize<'de> for SetAttachDebugStackResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetAttachDebugStackResponse;
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
                Ok(SetAttachDebugStackResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetAttachDebugStack {
    type Params = SetAttachDebugStackParams;
    type Response = SetAttachDebugStackResponse;
    const METHOD: &'static str = "Network.setAttachDebugStack";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.setBlockedURLs/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct SetBlockedUrLsParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setBlockedURLs/parameters/urlPatterns。
    ///
    ///协议原文：
    ///
    ///<pre>Patterns to match in the order in which they are given. These patterns
    ///also take precedence over any wildcard patterns defined in `urls`.</pre>
    ///
    ///
    #[serde(rename = "urlPatterns")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub url_patterns: Option<Vec<crate::network::types::BlockPattern>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setBlockedURLs/parameters/urls。
    ///
    ///协议原文：
    ///
    ///<pre>URL patterns to block. Wildcards ('*') are allowed.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "urls")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub urls: Option<Vec<String>>,
}
impl<'de> serde::Deserialize<'de> for SetBlockedUrLsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetBlockedUrLsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setBlockedURLs/parameters/urlPatterns。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Patterns to match in the order in which they are given. These patterns
                    ///also take precedence over any wildcard patterns defined in `urls`.</pre>
                    ///
                    ///
                    #[serde(rename = "urlPatterns")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub url_patterns: Option<Vec<crate::network::types::BlockPattern>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setBlockedURLs/parameters/urls。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>URL patterns to block. Wildcards ('*') are allowed.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "urls")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub urls: Option<Vec<String>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetBlockedUrLsParams {
                    url_patterns: fields.url_patterns,
                    urls: fields.urls,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.setBlockedURLs。
///
///协议原文：
///
///<pre>Blocks URLs from loading.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetBlockedUrLs;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.setBlockedURLs/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetBlockedUrLsResponse {}
impl<'de> serde::Deserialize<'de> for SetBlockedUrLsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetBlockedUrLsResponse;
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
                Ok(SetBlockedUrLsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetBlockedUrLs {
    type Params = SetBlockedUrLsParams;
    type Response = SetBlockedUrLsResponse;
    const METHOD: &'static str = "Network.setBlockedURLs";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.setBypassServiceWorker/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetBypassServiceWorkerParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.setBypassServiceWorker/parameters/bypass。
    ///
    ///协议原文：
    ///
    ///<pre>Bypass service worker and load from network.</pre>
    ///
    ///
    #[serde(rename = "bypass")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub bypass: bool,
}
impl<'de> serde::Deserialize<'de> for SetBypassServiceWorkerParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetBypassServiceWorkerParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.setBypassServiceWorker/parameters/bypass。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Bypass service worker and load from network.</pre>
                    ///
                    ///
                    #[serde(rename = "bypass")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub bypass: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetBypassServiceWorkerParams {
                    bypass: fields.bypass,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.setBypassServiceWorker。
///
///协议原文：
///
///<pre>Toggles ignoring of service worker for each request.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetBypassServiceWorker;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.setBypassServiceWorker/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetBypassServiceWorkerResponse {}
impl<'de> serde::Deserialize<'de> for SetBypassServiceWorkerResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetBypassServiceWorkerResponse;
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
                Ok(SetBypassServiceWorkerResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetBypassServiceWorker {
    type Params = SetBypassServiceWorkerParams;
    type Response = SetBypassServiceWorkerResponse;
    const METHOD: &'static str = "Network.setBypassServiceWorker";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.setCacheDisabled/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetCacheDisabledParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.setCacheDisabled/parameters/cacheDisabled。
    ///
    ///协议原文：
    ///
    ///<pre>Cache disabled state.</pre>
    ///
    ///
    #[serde(rename = "cacheDisabled")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cache_disabled: bool,
}
impl<'de> serde::Deserialize<'de> for SetCacheDisabledParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetCacheDisabledParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.setCacheDisabled/parameters/cacheDisabled。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cache disabled state.</pre>
                    ///
                    ///
                    #[serde(rename = "cacheDisabled")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cache_disabled: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetCacheDisabledParams {
                    cache_disabled: fields.cache_disabled,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.setCacheDisabled。
///
///协议原文：
///
///<pre>Toggles ignoring cache for each request. If `true`, cache will not be used.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetCacheDisabled;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.setCacheDisabled/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetCacheDisabledResponse {}
impl<'de> serde::Deserialize<'de> for SetCacheDisabledResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetCacheDisabledResponse;
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
                Ok(SetCacheDisabledResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetCacheDisabled {
    type Params = SetCacheDisabledParams;
    type Response = SetCacheDisabledResponse;
    const METHOD: &'static str = "Network.setCacheDisabled";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.setCookie/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetCookieParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.setCookie/parameters/name。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.setCookie/parameters/value。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie value.</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setCookie/parameters/url。
    ///
    ///协议原文：
    ///
    ///<pre>The request-URI to associate with the setting of the cookie. This value can affect the
    ///default domain, path, source port, and source scheme values of the created cookie.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setCookie/parameters/domain。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie domain.</pre>
    ///
    ///
    #[serde(rename = "domain")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub domain: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setCookie/parameters/path。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie path.</pre>
    ///
    ///
    #[serde(rename = "path")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub path: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setCookie/parameters/secure。
    ///
    ///协议原文：
    ///
    ///<pre>True if cookie is secure.</pre>
    ///
    ///
    #[serde(rename = "secure")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub secure: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setCookie/parameters/httpOnly。
    ///
    ///协议原文：
    ///
    ///<pre>True if cookie is http-only.</pre>
    ///
    ///
    #[serde(rename = "httpOnly")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub http_only: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setCookie/parameters/sameSite。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie SameSite type.</pre>
    ///
    ///
    #[serde(rename = "sameSite")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub same_site: Option<crate::network::types::CookieSameSite>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setCookie/parameters/expires。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie expiration date, session cookie if not set</pre>
    ///
    ///
    #[serde(rename = "expires")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub expires: Option<crate::network::types::TimeSinceEpoch>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setCookie/parameters/priority。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie Priority type.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "priority")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub priority: Option<crate::network::types::CookiePriority>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setCookie/parameters/sourceScheme。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie source scheme type.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "sourceScheme")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_scheme: Option<crate::network::types::CookieSourceScheme>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setCookie/parameters/sourcePort。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie source port. Valid values are {-1, [1, 65535]}, -1 indicates an unspecified port.
    ///An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.
    ///This is a temporary ability and it will be removed in the future.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "sourcePort")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_port: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setCookie/parameters/partitionKey。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie partition key. If not set, the cookie will be set as not partitioned.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "partitionKey")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub partition_key: Option<crate::network::types::CookiePartitionKey>,
}
impl<'de> serde::Deserialize<'de> for SetCookieParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetCookieParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.setCookie/parameters/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.setCookie/parameters/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie value.</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setCookie/parameters/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The request-URI to associate with the setting of the cookie. This value can affect the
                    ///default domain, path, source port, and source scheme values of the created cookie.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub url: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setCookie/parameters/domain。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie domain.</pre>
                    ///
                    ///
                    #[serde(rename = "domain")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub domain: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setCookie/parameters/path。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie path.</pre>
                    ///
                    ///
                    #[serde(rename = "path")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub path: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setCookie/parameters/secure。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if cookie is secure.</pre>
                    ///
                    ///
                    #[serde(rename = "secure")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub secure: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setCookie/parameters/httpOnly。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if cookie is http-only.</pre>
                    ///
                    ///
                    #[serde(rename = "httpOnly")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub http_only: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setCookie/parameters/sameSite。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie SameSite type.</pre>
                    ///
                    ///
                    #[serde(rename = "sameSite")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub same_site: Option<crate::network::types::CookieSameSite>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setCookie/parameters/expires。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie expiration date, session cookie if not set</pre>
                    ///
                    ///
                    #[serde(rename = "expires")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub expires: Option<crate::network::types::TimeSinceEpoch>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setCookie/parameters/priority。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie Priority type.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "priority")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub priority: Option<crate::network::types::CookiePriority>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setCookie/parameters/sourceScheme。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie source scheme type.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "sourceScheme")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub source_scheme: Option<crate::network::types::CookieSourceScheme>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setCookie/parameters/sourcePort。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie source port. Valid values are {-1, [1, 65535]}, -1 indicates an unspecified port.
                    ///An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.
                    ///This is a temporary ability and it will be removed in the future.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "sourcePort")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub source_port: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setCookie/parameters/partitionKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie partition key. If not set, the cookie will be set as not partitioned.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "partitionKey")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub partition_key: Option<crate::network::types::CookiePartitionKey>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetCookieParams {
                    name: fields.name,
                    value: fields.value,
                    url: fields.url,
                    domain: fields.domain,
                    path: fields.path,
                    secure: fields.secure,
                    http_only: fields.http_only,
                    same_site: fields.same_site,
                    expires: fields.expires,
                    priority: fields.priority,
                    source_scheme: fields.source_scheme,
                    source_port: fields.source_port,
                    partition_key: fields.partition_key,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.setCookie。
///
///协议原文：
///
///<pre>Sets a cookie with the given cookie data; may overwrite equivalent cookies if they exist.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetCookie;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.setCookie/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetCookieResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.setCookie/returns/success。
    ///
    ///协议原文：
    ///
    ///<pre>Always set to true. If an error occurs, the response indicates protocol error.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "success")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub success: bool,
}
impl<'de> serde::Deserialize<'de> for SetCookieResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetCookieResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.setCookie/returns/success。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Always set to true. If an error occurs, the response indicates protocol error.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "success")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub success: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetCookieResponse {
                    success: fields.success,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetCookie {
    type Params = SetCookieParams;
    type Response = SetCookieResponse;
    const METHOD: &'static str = "Network.setCookie";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.setCookieControls/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetCookieControlsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.setCookieControls/parameters/enableThirdPartyCookieRestriction。
    ///
    ///协议原文：
    ///
    ///<pre>Whether 3pc restriction is enabled.</pre>
    ///
    ///
    #[serde(rename = "enableThirdPartyCookieRestriction")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub enable_third_party_cookie_restriction: bool,
}
impl<'de> serde::Deserialize<'de> for SetCookieControlsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetCookieControlsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.setCookieControls/parameters/enableThirdPartyCookieRestriction。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether 3pc restriction is enabled.</pre>
                    ///
                    ///
                    #[serde(rename = "enableThirdPartyCookieRestriction")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub enable_third_party_cookie_restriction: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetCookieControlsParams {
                    enable_third_party_cookie_restriction: fields
                        .enable_third_party_cookie_restriction,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.setCookieControls。
///
///协议原文：
///
///<pre>Sets Controls for third-party cookie access
///Page reload is required before the new cookie behavior will be observed</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetCookieControls;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.setCookieControls/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetCookieControlsResponse {}
impl<'de> serde::Deserialize<'de> for SetCookieControlsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetCookieControlsResponse;
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
                Ok(SetCookieControlsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetCookieControls {
    type Params = SetCookieControlsParams;
    type Response = SetCookieControlsResponse;
    const METHOD: &'static str = "Network.setCookieControls";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.setCookies/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetCookiesParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.setCookies/parameters/cookies。
    ///
    ///协议原文：
    ///
    ///<pre>Cookies to be set.</pre>
    ///
    ///
    #[serde(rename = "cookies")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cookies: Vec<crate::network::types::CookieParam>,
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
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.setCookies/parameters/cookies。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookies to be set.</pre>
                    ///
                    ///
                    #[serde(rename = "cookies")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cookies: Vec<crate::network::types::CookieParam>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetCookiesParams {
                    cookies: fields.cookies,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.setCookies。
///
///协议原文：
///
///<pre>Sets given cookies.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetCookies;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.setCookies/returns。
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
    const METHOD: &'static str = "Network.setCookies";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.setExtraHTTPHeaders/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetExtraHttpHeadersParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.setExtraHTTPHeaders/parameters/headers。
    ///
    ///协议原文：
    ///
    ///<pre>Map with extra HTTP headers.</pre>
    ///
    ///
    #[serde(rename = "headers")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub headers: crate::network::types::Headers,
}
impl<'de> serde::Deserialize<'de> for SetExtraHttpHeadersParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetExtraHttpHeadersParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.setExtraHTTPHeaders/parameters/headers。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Map with extra HTTP headers.</pre>
                    ///
                    ///
                    #[serde(rename = "headers")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub headers: crate::network::types::Headers,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetExtraHttpHeadersParams {
                    headers: fields.headers,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.setExtraHTTPHeaders。
///
///协议原文：
///
///<pre>Specifies whether to always send extra HTTP headers with the requests from this page.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetExtraHttpHeaders;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.setExtraHTTPHeaders/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetExtraHttpHeadersResponse {}
impl<'de> serde::Deserialize<'de> for SetExtraHttpHeadersResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetExtraHttpHeadersResponse;
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
                Ok(SetExtraHttpHeadersResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetExtraHttpHeaders {
    type Params = SetExtraHttpHeadersParams;
    type Response = SetExtraHttpHeadersResponse;
    const METHOD: &'static str = "Network.setExtraHTTPHeaders";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.setUserAgentOverride/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetUserAgentOverrideParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.setUserAgentOverride/parameters/userAgent。
    ///
    ///协议原文：
    ///
    ///<pre>User agent to use.</pre>
    ///
    ///
    #[serde(rename = "userAgent")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub user_agent: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setUserAgentOverride/parameters/acceptLanguage。
    ///
    ///协议原文：
    ///
    ///<pre>Browser language to emulate.</pre>
    ///
    ///
    #[serde(rename = "acceptLanguage")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub accept_language: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setUserAgentOverride/parameters/platform。
    ///
    ///协议原文：
    ///
    ///<pre>The platform navigator.platform should return.</pre>
    ///
    ///
    #[serde(rename = "platform")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub platform: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setUserAgentOverride/parameters/userAgentMetadata。
    ///
    ///协议原文：
    ///
    ///<pre>To be sent in Sec-CH-UA-* headers and returned in navigator.userAgentData</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "userAgentMetadata")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_agent_metadata: Option<crate::emulation::types::UserAgentMetadata>,
}
impl<'de> serde::Deserialize<'de> for SetUserAgentOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetUserAgentOverrideParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.setUserAgentOverride/parameters/userAgent。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>User agent to use.</pre>
                    ///
                    ///
                    #[serde(rename = "userAgent")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub user_agent: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setUserAgentOverride/parameters/acceptLanguage。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Browser language to emulate.</pre>
                    ///
                    ///
                    #[serde(rename = "acceptLanguage")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub accept_language: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setUserAgentOverride/parameters/platform。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The platform navigator.platform should return.</pre>
                    ///
                    ///
                    #[serde(rename = "platform")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub platform: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.command.setUserAgentOverride/parameters/userAgentMetadata。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>To be sent in Sec-CH-UA-* headers and returned in navigator.userAgentData</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "userAgentMetadata")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub user_agent_metadata: Option<crate::emulation::types::UserAgentMetadata>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetUserAgentOverrideParams {
                    user_agent: fields.user_agent,
                    accept_language: fields.accept_language,
                    platform: fields.platform,
                    user_agent_metadata: fields.user_agent_metadata,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.setUserAgentOverride。
///
///协议原文：
///
///<pre>Allows overriding user agent with the given string.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetUserAgentOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.setUserAgentOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetUserAgentOverrideResponse {}
impl<'de> serde::Deserialize<'de> for SetUserAgentOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetUserAgentOverrideResponse;
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
                Ok(SetUserAgentOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetUserAgentOverride {
    type Params = SetUserAgentOverrideParams;
    type Response = SetUserAgentOverrideResponse;
    const METHOD: &'static str = "Network.setUserAgentOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.streamResourceContent/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StreamResourceContentParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.streamResourceContent/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the request to stream.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::network::types::RequestId,
}
impl<'de> serde::Deserialize<'de> for StreamResourceContentParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StreamResourceContentParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.streamResourceContent/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the request to stream.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::network::types::RequestId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StreamResourceContentParams {
                    request_id: fields.request_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.command.streamResourceContent。
///
///协议原文：
///
///<pre>Enables streaming of the response for the given requestId.
///If enabled, the dataReceived event contains the data that was received during streaming.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct StreamResourceContent;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.command.streamResourceContent/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StreamResourceContentResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.streamResourceContent/returns/bufferedData。
    ///
    ///协议原文：
    ///
    ///<pre>Data that has been buffered until streaming is enabled. (Encoded as a base64 string when passed over JSON)</pre>
    ///
    ///
    #[serde(rename = "bufferedData")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub buffered_data: String,
}
impl<'de> serde::Deserialize<'de> for StreamResourceContentResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StreamResourceContentResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.command.streamResourceContent/returns/bufferedData。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Data that has been buffered until streaming is enabled. (Encoded as a base64 string when passed over JSON)</pre>
                    ///
                    ///
                    #[serde(rename = "bufferedData")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub buffered_data: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StreamResourceContentResponse {
                    buffered_data: fields.buffered_data,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for StreamResourceContent {
    type Params = StreamResourceContentParams;
    type Response = StreamResourceContentResponse;
    const METHOD: &'static str = "Network.streamResourceContent";
}
