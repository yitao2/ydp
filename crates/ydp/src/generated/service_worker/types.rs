///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：ServiceWorker.type.RegistrationID。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct RegistrationId(String);
impl RegistrationId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::service_worker::types::RegistrationId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::service_worker::types::RegistrationId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::service_worker::types::RegistrationId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::service_worker::types::RegistrationId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::service_worker::types::RegistrationId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.type.ServiceWorkerErrorMessage。
///
///协议原文：
///
///<pre>ServiceWorker error message.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ServiceWorkerErrorMessage {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerErrorMessage/properties/errorMessage。
    #[serde(rename = "errorMessage")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub error_message: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerErrorMessage/properties/registrationId。
    #[serde(rename = "registrationId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub registration_id: crate::service_worker::types::RegistrationId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerErrorMessage/properties/versionId。
    #[serde(rename = "versionId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub version_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerErrorMessage/properties/sourceURL。
    #[serde(rename = "sourceURL")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub source_url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerErrorMessage/properties/lineNumber。
    #[serde(rename = "lineNumber")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub line_number: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerErrorMessage/properties/columnNumber。
    #[serde(rename = "columnNumber")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub column_number: i64,
}
impl<'de> serde::Deserialize<'de> for ServiceWorkerErrorMessage {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ServiceWorkerErrorMessage;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerErrorMessage/properties/errorMessage。
                    #[serde(rename = "errorMessage")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub error_message: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerErrorMessage/properties/registrationId。
                    #[serde(rename = "registrationId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub registration_id: crate::service_worker::types::RegistrationId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerErrorMessage/properties/versionId。
                    #[serde(rename = "versionId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub version_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerErrorMessage/properties/sourceURL。
                    #[serde(rename = "sourceURL")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub source_url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerErrorMessage/properties/lineNumber。
                    #[serde(rename = "lineNumber")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub line_number: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerErrorMessage/properties/columnNumber。
                    #[serde(rename = "columnNumber")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub column_number: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ServiceWorkerErrorMessage {
                    error_message: fields.error_message,
                    registration_id: fields.registration_id,
                    version_id: fields.version_id,
                    source_url: fields.source_url,
                    line_number: fields.line_number,
                    column_number: fields.column_number,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.type.ServiceWorkerRegistration。
///
///协议原文：
///
///<pre>ServiceWorker registration.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ServiceWorkerRegistration {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerRegistration/properties/registrationId。
    #[serde(rename = "registrationId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub registration_id: crate::service_worker::types::RegistrationId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerRegistration/properties/scopeURL。
    #[serde(rename = "scopeURL")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub scope_url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerRegistration/properties/isDeleted。
    #[serde(rename = "isDeleted")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub is_deleted: bool,
}
impl<'de> serde::Deserialize<'de> for ServiceWorkerRegistration {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ServiceWorkerRegistration;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerRegistration/properties/registrationId。
                    #[serde(rename = "registrationId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub registration_id: crate::service_worker::types::RegistrationId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerRegistration/properties/scopeURL。
                    #[serde(rename = "scopeURL")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub scope_url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerRegistration/properties/isDeleted。
                    #[serde(rename = "isDeleted")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub is_deleted: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ServiceWorkerRegistration {
                    registration_id: fields.registration_id,
                    scope_url: fields.scope_url,
                    is_deleted: fields.is_deleted,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.type.ServiceWorkerRouterCondition。
///
///协议原文：
///
///<pre>Mostly corresponds to `RouterCondition` in ServiceWorker spec
///(https://www.w3.org/TR/service-workers/#dictdef-routercondition) while this
///currently lacks support for the nested conditions ("or" and "not").
///TODO(crbug.com/540469610): Support recursive conditions.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ServiceWorkerRouterCondition {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：ServiceWorker.type.ServiceWorkerRouterCondition/properties/urlPattern。
    ///
    ///协议原文：
    ///
    ///<pre>Plain text, or JSON serialization of URLPatternInit or URLPattern</pre>
    ///
    ///
    #[serde(rename = "urlPattern")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub url_pattern: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：ServiceWorker.type.ServiceWorkerRouterCondition/properties/requestMethod。
    #[serde(rename = "requestMethod")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub request_method: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：ServiceWorker.type.ServiceWorkerRouterCondition/properties/requestMode。
    #[serde(rename = "requestMode")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub request_mode: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：ServiceWorker.type.ServiceWorkerRouterCondition/properties/requestDestination。
    #[serde(rename = "requestDestination")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub request_destination: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：ServiceWorker.type.ServiceWorkerRouterCondition/properties/runningStatus。
    #[serde(rename = "runningStatus")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub running_status: Option<crate::service_worker::types::ServiceWorkerVersionRunningStatus>,
}
impl<'de> serde::Deserialize<'de> for ServiceWorkerRouterCondition {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ServiceWorkerRouterCondition;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：ServiceWorker.type.ServiceWorkerRouterCondition/properties/urlPattern。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Plain text, or JSON serialization of URLPatternInit or URLPattern</pre>
                    ///
                    ///
                    #[serde(rename = "urlPattern")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub url_pattern: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：ServiceWorker.type.ServiceWorkerRouterCondition/properties/requestMethod。
                    #[serde(rename = "requestMethod")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub request_method: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：ServiceWorker.type.ServiceWorkerRouterCondition/properties/requestMode。
                    #[serde(rename = "requestMode")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub request_mode: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：ServiceWorker.type.ServiceWorkerRouterCondition/properties/requestDestination。
                    #[serde(rename = "requestDestination")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub request_destination: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：ServiceWorker.type.ServiceWorkerRouterCondition/properties/runningStatus。
                    #[serde(rename = "runningStatus")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub running_status:
                        Option<crate::service_worker::types::ServiceWorkerVersionRunningStatus>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ServiceWorkerRouterCondition {
                    url_pattern: fields.url_pattern,
                    request_method: fields.request_method,
                    request_mode: fields.request_mode,
                    request_destination: fields.request_destination,
                    running_status: fields.running_status,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.type.ServiceWorkerRouterRule。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ServiceWorkerRouterRule {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerRouterRule/properties/condition。
    #[serde(rename = "condition")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub condition: crate::service_worker::types::ServiceWorkerRouterCondition,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerRouterRule/properties/source。
    #[serde(rename = "source")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub source: crate::service_worker::types::ServiceWorkerRouterSource,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerRouterRule/properties/id。
    ///
    ///协议原文：
    ///
    ///<pre>Rule ID assigned by the browser. Unique within each ServiceWorkerVersion.</pre>
    ///
    ///
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: i64,
}
impl<'de> serde::Deserialize<'de> for ServiceWorkerRouterRule {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ServiceWorkerRouterRule;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerRouterRule/properties/condition。
                    #[serde(rename = "condition")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub condition: crate::service_worker::types::ServiceWorkerRouterCondition,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerRouterRule/properties/source。
                    #[serde(rename = "source")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub source: crate::service_worker::types::ServiceWorkerRouterSource,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerRouterRule/properties/id。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Rule ID assigned by the browser. Unique within each ServiceWorkerVersion.</pre>
                    ///
                    ///
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ServiceWorkerRouterRule {
                    condition: fields.condition,
                    source: fields.source,
                    id: fields.id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.type.ServiceWorkerRouterSource。
///
///协议原文：
///
///<pre>Corresponds to `RouterSource` in the spec while the representation is different as follows.
///(https://www.w3.org/TR/service-workers/#typedefdef-routersource)
///- `RouterSourceEnum`: `type` equals `cache`, `sourceDict` is null.
///- `RouterSourceDict`: `type` equals `sourceDict`, `sourceDict` has valid value.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ServiceWorkerRouterSource {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerRouterSource/properties/type。
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: crate::service_worker::types::ServiceWorkerRouterSourceType,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：ServiceWorker.type.ServiceWorkerRouterSource/properties/sourceDict。
    ///
    ///协议原文：
    ///
    ///<pre>Non-empty iff `type` equals "sourceDict".</pre>
    ///
    ///
    #[serde(rename = "sourceDict")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_dict: Option<crate::service_worker::types::ServiceWorkerRouterSourceDict>,
}
impl<'de> serde::Deserialize<'de> for ServiceWorkerRouterSource {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ServiceWorkerRouterSource;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerRouterSource/properties/type。
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: crate::service_worker::types::ServiceWorkerRouterSourceType,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：ServiceWorker.type.ServiceWorkerRouterSource/properties/sourceDict。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Non-empty iff `type` equals "sourceDict".</pre>
                    ///
                    ///
                    #[serde(rename = "sourceDict")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub source_dict:
                        Option<crate::service_worker::types::ServiceWorkerRouterSourceDict>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ServiceWorkerRouterSource {
                    r#type: fields.r#type,
                    source_dict: fields.source_dict,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.type.ServiceWorkerRouterSourceDict。
///
///协议原文：
///
///<pre>https://www.w3.org/TR/service-workers/#dictdef-routersourcedict</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ServiceWorkerRouterSourceDict {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerRouterSourceDict/properties/cacheName。
    #[serde(rename = "cacheName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cache_name: String,
}
impl<'de> serde::Deserialize<'de> for ServiceWorkerRouterSourceDict {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ServiceWorkerRouterSourceDict;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerRouterSourceDict/properties/cacheName。
                    #[serde(rename = "cacheName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cache_name: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ServiceWorkerRouterSourceDict {
                    cache_name: fields.cache_name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：ServiceWorker.type.ServiceWorkerRouterSourceType。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ServiceWorkerRouterSourceType {
    ///协议值 "cache"。
    Cache,
    ///协议值 "fetchEvent"。
    FetchEvent,
    ///协议值 "network"。
    Network,
    ///协议值 "raceNetworkAndFetchHandler"。
    RaceNetworkAndFetchHandler,
    ///协议值 "raceNetworkAndCache"。
    RaceNetworkAndCache,
    ///协议值 "sourceDict"。
    SourceDict,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ServiceWorkerRouterSourceType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::service_worker::types::ServiceWorkerRouterSourceType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Cache => "cache",
            Self::FetchEvent => "fetchEvent",
            Self::Network => "network",
            Self::RaceNetworkAndFetchHandler => "raceNetworkAndFetchHandler",
            Self::RaceNetworkAndCache => "raceNetworkAndCache",
            Self::SourceDict => "sourceDict",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ServiceWorkerRouterSourceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ServiceWorkerRouterSourceType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "cache" => ServiceWorkerRouterSourceType::Cache,
            "fetchEvent" => ServiceWorkerRouterSourceType::FetchEvent,
            "network" => ServiceWorkerRouterSourceType::Network,
            "raceNetworkAndFetchHandler" => {
                ServiceWorkerRouterSourceType::RaceNetworkAndFetchHandler
            }
            "raceNetworkAndCache" => ServiceWorkerRouterSourceType::RaceNetworkAndCache,
            "sourceDict" => ServiceWorkerRouterSourceType::SourceDict,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ServiceWorkerRouterSourceType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ServiceWorkerRouterSourceType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ServiceWorkerRouterSourceType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "cache" => ServiceWorkerRouterSourceType::Cache,
                    "fetchEvent" => ServiceWorkerRouterSourceType::FetchEvent,
                    "network" => ServiceWorkerRouterSourceType::Network,
                    "raceNetworkAndFetchHandler" => {
                        ServiceWorkerRouterSourceType::RaceNetworkAndFetchHandler
                    }
                    "raceNetworkAndCache" => ServiceWorkerRouterSourceType::RaceNetworkAndCache,
                    "sourceDict" => ServiceWorkerRouterSourceType::SourceDict,
                    _ => ServiceWorkerRouterSourceType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ServiceWorkerRouterSourceType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.type.ServiceWorkerVersion。
///
///协议原文：
///
///<pre>ServiceWorker version.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ServiceWorkerVersion {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerVersion/properties/versionId。
    #[serde(rename = "versionId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub version_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerVersion/properties/registrationId。
    #[serde(rename = "registrationId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub registration_id: crate::service_worker::types::RegistrationId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerVersion/properties/scriptURL。
    #[serde(rename = "scriptURL")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub script_url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerVersion/properties/runningStatus。
    #[serde(rename = "runningStatus")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub running_status: crate::service_worker::types::ServiceWorkerVersionRunningStatus,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerVersion/properties/status。
    #[serde(rename = "status")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub status: crate::service_worker::types::ServiceWorkerVersionStatus,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：ServiceWorker.type.ServiceWorkerVersion/properties/scriptLastModified。
    ///
    ///协议原文：
    ///
    ///<pre>The Last-Modified header value of the main script.</pre>
    ///
    ///
    #[serde(rename = "scriptLastModified")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub script_last_modified: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：ServiceWorker.type.ServiceWorkerVersion/properties/scriptResponseTime。
    ///
    ///协议原文：
    ///
    ///<pre>The time at which the response headers of the main script were received from the server.
    ///For cached script it is the last time the cache entry was validated.</pre>
    ///
    ///
    #[serde(rename = "scriptResponseTime")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub script_response_time: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：ServiceWorker.type.ServiceWorkerVersion/properties/controlledClients。
    #[serde(rename = "controlledClients")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub controlled_clients: Option<Vec<crate::target::types::TargetId>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：ServiceWorker.type.ServiceWorkerVersion/properties/targetId。
    #[serde(rename = "targetId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_id: Option<crate::target::types::TargetId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：ServiceWorker.type.ServiceWorkerVersion/properties/routerRules。
    ///
    ///协议原文：
    ///
    ///<pre>Migration to `typedRouterRules` is in progress. The browser sends either
    ///`routerRules` or `typedRouterRules`.
    ///TODO(crbug.com/540469610): Remove `routerRules` after the migration.</pre>
    ///
    ///
    #[serde(rename = "routerRules")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub router_rules: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：ServiceWorker.type.ServiceWorkerVersion/properties/typedRouterRules。
    #[serde(rename = "typedRouterRules")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub typed_router_rules: Option<Vec<crate::service_worker::types::ServiceWorkerRouterRule>>,
}
impl<'de> serde::Deserialize<'de> for ServiceWorkerVersion {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ServiceWorkerVersion;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerVersion/properties/versionId。
                    #[serde(rename = "versionId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub version_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerVersion/properties/registrationId。
                    #[serde(rename = "registrationId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub registration_id: crate::service_worker::types::RegistrationId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerVersion/properties/scriptURL。
                    #[serde(rename = "scriptURL")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub script_url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerVersion/properties/runningStatus。
                    #[serde(rename = "runningStatus")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub running_status:
                        crate::service_worker::types::ServiceWorkerVersionRunningStatus,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.type.ServiceWorkerVersion/properties/status。
                    #[serde(rename = "status")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub status: crate::service_worker::types::ServiceWorkerVersionStatus,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：ServiceWorker.type.ServiceWorkerVersion/properties/scriptLastModified。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The Last-Modified header value of the main script.</pre>
                    ///
                    ///
                    #[serde(rename = "scriptLastModified")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub script_last_modified: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：ServiceWorker.type.ServiceWorkerVersion/properties/scriptResponseTime。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The time at which the response headers of the main script were received from the server.
                    ///For cached script it is the last time the cache entry was validated.</pre>
                    ///
                    ///
                    #[serde(rename = "scriptResponseTime")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub script_response_time: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：ServiceWorker.type.ServiceWorkerVersion/properties/controlledClients。
                    #[serde(rename = "controlledClients")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub controlled_clients: Option<Vec<crate::target::types::TargetId>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：ServiceWorker.type.ServiceWorkerVersion/properties/targetId。
                    #[serde(rename = "targetId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub target_id: Option<crate::target::types::TargetId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：ServiceWorker.type.ServiceWorkerVersion/properties/routerRules。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Migration to `typedRouterRules` is in progress. The browser sends either
                    ///`routerRules` or `typedRouterRules`.
                    ///TODO(crbug.com/540469610): Remove `routerRules` after the migration.</pre>
                    ///
                    ///
                    #[serde(rename = "routerRules")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub router_rules: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：ServiceWorker.type.ServiceWorkerVersion/properties/typedRouterRules。
                    #[serde(rename = "typedRouterRules")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub typed_router_rules:
                        Option<Vec<crate::service_worker::types::ServiceWorkerRouterRule>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ServiceWorkerVersion {
                    version_id: fields.version_id,
                    registration_id: fields.registration_id,
                    script_url: fields.script_url,
                    running_status: fields.running_status,
                    status: fields.status,
                    script_last_modified: fields.script_last_modified,
                    script_response_time: fields.script_response_time,
                    controlled_clients: fields.controlled_clients,
                    target_id: fields.target_id,
                    router_rules: fields.router_rules,
                    typed_router_rules: fields.typed_router_rules,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：ServiceWorker.type.ServiceWorkerVersionRunningStatus。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ServiceWorkerVersionRunningStatus {
    ///协议值 "stopped"。
    Stopped,
    ///协议值 "starting"。
    Starting,
    ///协议值 "running"。
    Running,
    ///协议值 "stopping"。
    Stopping,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ServiceWorkerVersionRunningStatus {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::service_worker::types::ServiceWorkerVersionRunningStatus::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Stopped => "stopped",
            Self::Starting => "starting",
            Self::Running => "running",
            Self::Stopping => "stopping",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ServiceWorkerVersionRunningStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ServiceWorkerVersionRunningStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "stopped" => ServiceWorkerVersionRunningStatus::Stopped,
            "starting" => ServiceWorkerVersionRunningStatus::Starting,
            "running" => ServiceWorkerVersionRunningStatus::Running,
            "stopping" => ServiceWorkerVersionRunningStatus::Stopping,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ServiceWorkerVersionRunningStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ServiceWorkerVersionRunningStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ServiceWorkerVersionRunningStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "stopped" => ServiceWorkerVersionRunningStatus::Stopped,
                    "starting" => ServiceWorkerVersionRunningStatus::Starting,
                    "running" => ServiceWorkerVersionRunningStatus::Running,
                    "stopping" => ServiceWorkerVersionRunningStatus::Stopping,
                    _ => ServiceWorkerVersionRunningStatus::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ServiceWorkerVersionRunningStatus::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：ServiceWorker.type.ServiceWorkerVersionStatus。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ServiceWorkerVersionStatus {
    ///协议值 "new"。
    New,
    ///协议值 "installing"。
    Installing,
    ///协议值 "installed"。
    Installed,
    ///协议值 "activating"。
    Activating,
    ///协议值 "activated"。
    Activated,
    ///协议值 "redundant"。
    Redundant,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ServiceWorkerVersionStatus {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::service_worker::types::ServiceWorkerVersionStatus::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::New => "new",
            Self::Installing => "installing",
            Self::Installed => "installed",
            Self::Activating => "activating",
            Self::Activated => "activated",
            Self::Redundant => "redundant",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ServiceWorkerVersionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ServiceWorkerVersionStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "new" => ServiceWorkerVersionStatus::New,
            "installing" => ServiceWorkerVersionStatus::Installing,
            "installed" => ServiceWorkerVersionStatus::Installed,
            "activating" => ServiceWorkerVersionStatus::Activating,
            "activated" => ServiceWorkerVersionStatus::Activated,
            "redundant" => ServiceWorkerVersionStatus::Redundant,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ServiceWorkerVersionStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ServiceWorkerVersionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ServiceWorkerVersionStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "new" => ServiceWorkerVersionStatus::New,
                    "installing" => ServiceWorkerVersionStatus::Installing,
                    "installed" => ServiceWorkerVersionStatus::Installed,
                    "activating" => ServiceWorkerVersionStatus::Activating,
                    "activated" => ServiceWorkerVersionStatus::Activated,
                    "redundant" => ServiceWorkerVersionStatus::Redundant,
                    _ => ServiceWorkerVersionStatus::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ServiceWorkerVersionStatus::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
