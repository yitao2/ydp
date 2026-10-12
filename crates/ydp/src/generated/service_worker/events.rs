///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.event.workerErrorReported/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct WorkerErrorReportedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.event.workerErrorReported/parameters/errorMessage。
    #[serde(rename = "errorMessage")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub error_message: crate::service_worker::types::ServiceWorkerErrorMessage,
}
impl<'de> serde::Deserialize<'de> for WorkerErrorReportedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WorkerErrorReportedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.event.workerErrorReported/parameters/errorMessage。
                    #[serde(rename = "errorMessage")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub error_message: crate::service_worker::types::ServiceWorkerErrorMessage,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(WorkerErrorReportedParams {
                    error_message: fields.error_message,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：ServiceWorker.event.workerErrorReported。
#[derive(Debug, Clone, Copy)]
pub struct WorkerErrorReported;
impl ydp_core::Event for WorkerErrorReported {
    type Params = WorkerErrorReportedParams;
    const METHOD: &'static str = "ServiceWorker.workerErrorReported";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.event.workerRegistrationUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct WorkerRegistrationUpdatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.event.workerRegistrationUpdated/parameters/registrations。
    #[serde(rename = "registrations")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub registrations: Vec<crate::service_worker::types::ServiceWorkerRegistration>,
}
impl<'de> serde::Deserialize<'de> for WorkerRegistrationUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WorkerRegistrationUpdatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.event.workerRegistrationUpdated/parameters/registrations。
                    #[serde(rename = "registrations")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub registrations: Vec<crate::service_worker::types::ServiceWorkerRegistration>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(WorkerRegistrationUpdatedParams {
                    registrations: fields.registrations,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：ServiceWorker.event.workerRegistrationUpdated。
#[derive(Debug, Clone, Copy)]
pub struct WorkerRegistrationUpdated;
impl ydp_core::Event for WorkerRegistrationUpdated {
    type Params = WorkerRegistrationUpdatedParams;
    const METHOD: &'static str = "ServiceWorker.workerRegistrationUpdated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.event.workerVersionUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct WorkerVersionUpdatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.event.workerVersionUpdated/parameters/versions。
    #[serde(rename = "versions")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub versions: Vec<crate::service_worker::types::ServiceWorkerVersion>,
}
impl<'de> serde::Deserialize<'de> for WorkerVersionUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WorkerVersionUpdatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.event.workerVersionUpdated/parameters/versions。
                    #[serde(rename = "versions")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub versions: Vec<crate::service_worker::types::ServiceWorkerVersion>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(WorkerVersionUpdatedParams {
                    versions: fields.versions,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：ServiceWorker.event.workerVersionUpdated。
#[derive(Debug, Clone, Copy)]
pub struct WorkerVersionUpdated;
impl ydp_core::Event for WorkerVersionUpdated {
    type Params = WorkerVersionUpdatedParams;
    const METHOD: &'static str = "ServiceWorker.workerVersionUpdated";
}
