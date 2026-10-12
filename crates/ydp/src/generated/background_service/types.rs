///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BackgroundService.type.BackgroundServiceEvent。
#[derive(Debug, Clone, serde::Serialize)]
pub struct BackgroundServiceEvent {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.type.BackgroundServiceEvent/properties/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Timestamp of the event (in seconds).</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::TimeSinceEpoch,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.type.BackgroundServiceEvent/properties/origin。
    ///
    ///协议原文：
    ///
    ///<pre>The origin this event belongs to.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.type.BackgroundServiceEvent/properties/serviceWorkerRegistrationId。
    ///
    ///协议原文：
    ///
    ///<pre>The Service Worker ID that initiated the event.</pre>
    ///
    ///
    #[serde(rename = "serviceWorkerRegistrationId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub service_worker_registration_id: crate::service_worker::types::RegistrationId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.type.BackgroundServiceEvent/properties/service。
    ///
    ///协议原文：
    ///
    ///<pre>The Background Service this event belongs to.</pre>
    ///
    ///
    #[serde(rename = "service")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub service: crate::background_service::types::ServiceName,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.type.BackgroundServiceEvent/properties/eventName。
    ///
    ///协议原文：
    ///
    ///<pre>A description of the event.</pre>
    ///
    ///
    #[serde(rename = "eventName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub event_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.type.BackgroundServiceEvent/properties/instanceId。
    ///
    ///协议原文：
    ///
    ///<pre>An identifier that groups related events together.</pre>
    ///
    ///
    #[serde(rename = "instanceId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub instance_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.type.BackgroundServiceEvent/properties/eventMetadata。
    ///
    ///协议原文：
    ///
    ///<pre>A list of event-specific information.</pre>
    ///
    ///
    #[serde(rename = "eventMetadata")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub event_metadata: Vec<crate::background_service::types::EventMetadata>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.type.BackgroundServiceEvent/properties/storageKey。
    ///
    ///协议原文：
    ///
    ///<pre>Storage key this event belongs to.</pre>
    ///
    ///
    #[serde(rename = "storageKey")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_key: String,
}
impl<'de> serde::Deserialize<'de> for BackgroundServiceEvent {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BackgroundServiceEvent;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.type.BackgroundServiceEvent/properties/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Timestamp of the event (in seconds).</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::TimeSinceEpoch,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.type.BackgroundServiceEvent/properties/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The origin this event belongs to.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.type.BackgroundServiceEvent/properties/serviceWorkerRegistrationId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The Service Worker ID that initiated the event.</pre>
                    ///
                    ///
                    #[serde(rename = "serviceWorkerRegistrationId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub service_worker_registration_id:
                        crate::service_worker::types::RegistrationId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.type.BackgroundServiceEvent/properties/service。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The Background Service this event belongs to.</pre>
                    ///
                    ///
                    #[serde(rename = "service")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub service: crate::background_service::types::ServiceName,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.type.BackgroundServiceEvent/properties/eventName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A description of the event.</pre>
                    ///
                    ///
                    #[serde(rename = "eventName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub event_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.type.BackgroundServiceEvent/properties/instanceId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An identifier that groups related events together.</pre>
                    ///
                    ///
                    #[serde(rename = "instanceId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub instance_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.type.BackgroundServiceEvent/properties/eventMetadata。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A list of event-specific information.</pre>
                    ///
                    ///
                    #[serde(rename = "eventMetadata")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub event_metadata: Vec<crate::background_service::types::EventMetadata>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.type.BackgroundServiceEvent/properties/storageKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage key this event belongs to.</pre>
                    ///
                    ///
                    #[serde(rename = "storageKey")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_key: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(BackgroundServiceEvent {
                    timestamp: fields.timestamp,
                    origin: fields.origin,
                    service_worker_registration_id: fields.service_worker_registration_id,
                    service: fields.service,
                    event_name: fields.event_name,
                    instance_id: fields.instance_id,
                    event_metadata: fields.event_metadata,
                    storage_key: fields.storage_key,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BackgroundService.type.EventMetadata。
///
///协议原文：
///
///<pre>A key-value pair for additional event information to pass along.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct EventMetadata {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.type.EventMetadata/properties/key。
    #[serde(rename = "key")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub key: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.type.EventMetadata/properties/value。
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: String,
}
impl<'de> serde::Deserialize<'de> for EventMetadata {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EventMetadata;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.type.EventMetadata/properties/key。
                    #[serde(rename = "key")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub key: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.type.EventMetadata/properties/value。
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EventMetadata {
                    key: fields.key,
                    value: fields.value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：BackgroundService.type.ServiceName。
///
///协议原文：
///
///<pre>The Background Service that will be associated with the commands/events.
///Every Background Service operates independently, but they share the same
///API.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ServiceName {
    ///协议值 "backgroundFetch"。
    BackgroundFetch,
    ///协议值 "backgroundSync"。
    BackgroundSync,
    ///协议值 "pushMessaging"。
    PushMessaging,
    ///协议值 "notifications"。
    Notifications,
    ///协议值 "paymentHandler"。
    PaymentHandler,
    ///协议值 "periodicBackgroundSync"。
    PeriodicBackgroundSync,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ServiceName {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::background_service::types::ServiceName::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::BackgroundFetch => "backgroundFetch",
            Self::BackgroundSync => "backgroundSync",
            Self::PushMessaging => "pushMessaging",
            Self::Notifications => "notifications",
            Self::PaymentHandler => "paymentHandler",
            Self::PeriodicBackgroundSync => "periodicBackgroundSync",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ServiceName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ServiceName {
    fn from(value: String) -> Self {
        match value.as_str() {
            "backgroundFetch" => ServiceName::BackgroundFetch,
            "backgroundSync" => ServiceName::BackgroundSync,
            "pushMessaging" => ServiceName::PushMessaging,
            "notifications" => ServiceName::Notifications,
            "paymentHandler" => ServiceName::PaymentHandler,
            "periodicBackgroundSync" => ServiceName::PeriodicBackgroundSync,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ServiceName {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ServiceName {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ServiceName;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "backgroundFetch" => ServiceName::BackgroundFetch,
                    "backgroundSync" => ServiceName::BackgroundSync,
                    "pushMessaging" => ServiceName::PushMessaging,
                    "notifications" => ServiceName::Notifications,
                    "paymentHandler" => ServiceName::PaymentHandler,
                    "periodicBackgroundSync" => ServiceName::PeriodicBackgroundSync,
                    _ => ServiceName::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ServiceName::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
