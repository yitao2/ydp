///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.command.deliverPushMessage/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeliverPushMessageParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.deliverPushMessage/parameters/origin。
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.deliverPushMessage/parameters/registrationId。
    #[serde(rename = "registrationId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub registration_id: crate::service_worker::types::RegistrationId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.deliverPushMessage/parameters/data。
    #[serde(rename = "data")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub data: String,
}
impl<'de> serde::Deserialize<'de> for DeliverPushMessageParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeliverPushMessageParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.deliverPushMessage/parameters/origin。
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.deliverPushMessage/parameters/registrationId。
                    #[serde(rename = "registrationId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub registration_id: crate::service_worker::types::RegistrationId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.deliverPushMessage/parameters/data。
                    #[serde(rename = "data")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub data: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DeliverPushMessageParams {
                    origin: fields.origin,
                    registration_id: fields.registration_id,
                    data: fields.data,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：ServiceWorker.command.deliverPushMessage。
#[derive(Debug, Clone, Copy)]
pub struct DeliverPushMessage;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.command.deliverPushMessage/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeliverPushMessageResponse {}
impl<'de> serde::Deserialize<'de> for DeliverPushMessageResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeliverPushMessageResponse;
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
                Ok(DeliverPushMessageResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for DeliverPushMessage {
    type Params = DeliverPushMessageParams;
    type Response = DeliverPushMessageResponse;
    const METHOD: &'static str = "ServiceWorker.deliverPushMessage";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.command.disable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：ServiceWorker.command.disable。
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.command.disable/returns。
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
    const METHOD: &'static str = "ServiceWorker.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.command.dispatchPeriodicSyncEvent/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DispatchPeriodicSyncEventParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.dispatchPeriodicSyncEvent/parameters/origin。
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.dispatchPeriodicSyncEvent/parameters/registrationId。
    #[serde(rename = "registrationId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub registration_id: crate::service_worker::types::RegistrationId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.dispatchPeriodicSyncEvent/parameters/tag。
    #[serde(rename = "tag")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub tag: String,
}
impl<'de> serde::Deserialize<'de> for DispatchPeriodicSyncEventParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DispatchPeriodicSyncEventParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.dispatchPeriodicSyncEvent/parameters/origin。
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.dispatchPeriodicSyncEvent/parameters/registrationId。
                    #[serde(rename = "registrationId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub registration_id: crate::service_worker::types::RegistrationId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.dispatchPeriodicSyncEvent/parameters/tag。
                    #[serde(rename = "tag")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub tag: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DispatchPeriodicSyncEventParams {
                    origin: fields.origin,
                    registration_id: fields.registration_id,
                    tag: fields.tag,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：ServiceWorker.command.dispatchPeriodicSyncEvent。
#[derive(Debug, Clone, Copy)]
pub struct DispatchPeriodicSyncEvent;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.command.dispatchPeriodicSyncEvent/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DispatchPeriodicSyncEventResponse {}
impl<'de> serde::Deserialize<'de> for DispatchPeriodicSyncEventResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DispatchPeriodicSyncEventResponse;
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
                Ok(DispatchPeriodicSyncEventResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for DispatchPeriodicSyncEvent {
    type Params = DispatchPeriodicSyncEventParams;
    type Response = DispatchPeriodicSyncEventResponse;
    const METHOD: &'static str = "ServiceWorker.dispatchPeriodicSyncEvent";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.command.dispatchSyncEvent/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DispatchSyncEventParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.dispatchSyncEvent/parameters/origin。
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.dispatchSyncEvent/parameters/registrationId。
    #[serde(rename = "registrationId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub registration_id: crate::service_worker::types::RegistrationId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.dispatchSyncEvent/parameters/tag。
    #[serde(rename = "tag")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub tag: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.dispatchSyncEvent/parameters/lastChance。
    #[serde(rename = "lastChance")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub last_chance: bool,
}
impl<'de> serde::Deserialize<'de> for DispatchSyncEventParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DispatchSyncEventParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.dispatchSyncEvent/parameters/origin。
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.dispatchSyncEvent/parameters/registrationId。
                    #[serde(rename = "registrationId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub registration_id: crate::service_worker::types::RegistrationId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.dispatchSyncEvent/parameters/tag。
                    #[serde(rename = "tag")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub tag: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.dispatchSyncEvent/parameters/lastChance。
                    #[serde(rename = "lastChance")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub last_chance: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DispatchSyncEventParams {
                    origin: fields.origin,
                    registration_id: fields.registration_id,
                    tag: fields.tag,
                    last_chance: fields.last_chance,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：ServiceWorker.command.dispatchSyncEvent。
#[derive(Debug, Clone, Copy)]
pub struct DispatchSyncEvent;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.command.dispatchSyncEvent/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DispatchSyncEventResponse {}
impl<'de> serde::Deserialize<'de> for DispatchSyncEventResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DispatchSyncEventResponse;
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
                Ok(DispatchSyncEventResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for DispatchSyncEvent {
    type Params = DispatchSyncEventParams;
    type Response = DispatchSyncEventResponse;
    const METHOD: &'static str = "ServiceWorker.dispatchSyncEvent";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.command.enable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：ServiceWorker.command.enable。
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.command.enable/returns。
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
    const METHOD: &'static str = "ServiceWorker.enable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.command.setForceUpdateOnPageLoad/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetForceUpdateOnPageLoadParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.setForceUpdateOnPageLoad/parameters/forceUpdateOnPageLoad。
    #[serde(rename = "forceUpdateOnPageLoad")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub force_update_on_page_load: bool,
}
impl<'de> serde::Deserialize<'de> for SetForceUpdateOnPageLoadParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetForceUpdateOnPageLoadParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.setForceUpdateOnPageLoad/parameters/forceUpdateOnPageLoad。
                    #[serde(rename = "forceUpdateOnPageLoad")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub force_update_on_page_load: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetForceUpdateOnPageLoadParams {
                    force_update_on_page_load: fields.force_update_on_page_load,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：ServiceWorker.command.setForceUpdateOnPageLoad。
#[derive(Debug, Clone, Copy)]
pub struct SetForceUpdateOnPageLoad;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.command.setForceUpdateOnPageLoad/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetForceUpdateOnPageLoadResponse {}
impl<'de> serde::Deserialize<'de> for SetForceUpdateOnPageLoadResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetForceUpdateOnPageLoadResponse;
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
                Ok(SetForceUpdateOnPageLoadResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetForceUpdateOnPageLoad {
    type Params = SetForceUpdateOnPageLoadParams;
    type Response = SetForceUpdateOnPageLoadResponse;
    const METHOD: &'static str = "ServiceWorker.setForceUpdateOnPageLoad";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.command.skipWaiting/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SkipWaitingParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.skipWaiting/parameters/scopeURL。
    #[serde(rename = "scopeURL")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub scope_url: String,
}
impl<'de> serde::Deserialize<'de> for SkipWaitingParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SkipWaitingParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.skipWaiting/parameters/scopeURL。
                    #[serde(rename = "scopeURL")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub scope_url: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SkipWaitingParams {
                    scope_url: fields.scope_url,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：ServiceWorker.command.skipWaiting。
#[derive(Debug, Clone, Copy)]
pub struct SkipWaiting;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.command.skipWaiting/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SkipWaitingResponse {}
impl<'de> serde::Deserialize<'de> for SkipWaitingResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SkipWaitingResponse;
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
                Ok(SkipWaitingResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SkipWaiting {
    type Params = SkipWaitingParams;
    type Response = SkipWaitingResponse;
    const METHOD: &'static str = "ServiceWorker.skipWaiting";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.command.startWorker/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StartWorkerParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.startWorker/parameters/scopeURL。
    #[serde(rename = "scopeURL")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub scope_url: String,
}
impl<'de> serde::Deserialize<'de> for StartWorkerParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StartWorkerParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.startWorker/parameters/scopeURL。
                    #[serde(rename = "scopeURL")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub scope_url: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StartWorkerParams {
                    scope_url: fields.scope_url,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：ServiceWorker.command.startWorker。
#[derive(Debug, Clone, Copy)]
pub struct StartWorker;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.command.startWorker/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StartWorkerResponse {}
impl<'de> serde::Deserialize<'de> for StartWorkerResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StartWorkerResponse;
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
                Ok(StartWorkerResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for StartWorker {
    type Params = StartWorkerParams;
    type Response = StartWorkerResponse;
    const METHOD: &'static str = "ServiceWorker.startWorker";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.command.stopAllWorkers/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct StopAllWorkersParams {}
impl<'de> serde::Deserialize<'de> for StopAllWorkersParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StopAllWorkersParams;
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
                Ok(StopAllWorkersParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：ServiceWorker.command.stopAllWorkers。
#[derive(Debug, Clone, Copy)]
pub struct StopAllWorkers;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.command.stopAllWorkers/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StopAllWorkersResponse {}
impl<'de> serde::Deserialize<'de> for StopAllWorkersResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StopAllWorkersResponse;
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
                Ok(StopAllWorkersResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for StopAllWorkers {
    type Params = StopAllWorkersParams;
    type Response = StopAllWorkersResponse;
    const METHOD: &'static str = "ServiceWorker.stopAllWorkers";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.command.stopWorker/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StopWorkerParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.stopWorker/parameters/versionId。
    #[serde(rename = "versionId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub version_id: String,
}
impl<'de> serde::Deserialize<'de> for StopWorkerParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StopWorkerParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.stopWorker/parameters/versionId。
                    #[serde(rename = "versionId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub version_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StopWorkerParams {
                    version_id: fields.version_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：ServiceWorker.command.stopWorker。
#[derive(Debug, Clone, Copy)]
pub struct StopWorker;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.command.stopWorker/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StopWorkerResponse {}
impl<'de> serde::Deserialize<'de> for StopWorkerResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StopWorkerResponse;
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
                Ok(StopWorkerResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for StopWorker {
    type Params = StopWorkerParams;
    type Response = StopWorkerResponse;
    const METHOD: &'static str = "ServiceWorker.stopWorker";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.command.unregister/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct UnregisterParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.unregister/parameters/scopeURL。
    #[serde(rename = "scopeURL")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub scope_url: String,
}
impl<'de> serde::Deserialize<'de> for UnregisterParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UnregisterParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.unregister/parameters/scopeURL。
                    #[serde(rename = "scopeURL")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub scope_url: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(UnregisterParams {
                    scope_url: fields.scope_url,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：ServiceWorker.command.unregister。
#[derive(Debug, Clone, Copy)]
pub struct Unregister;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.command.unregister/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct UnregisterResponse {}
impl<'de> serde::Deserialize<'de> for UnregisterResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UnregisterResponse;
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
                Ok(UnregisterResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Unregister {
    type Params = UnregisterParams;
    type Response = UnregisterResponse;
    const METHOD: &'static str = "ServiceWorker.unregister";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.command.updateRegistration/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct UpdateRegistrationParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.updateRegistration/parameters/scopeURL。
    #[serde(rename = "scopeURL")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub scope_url: String,
}
impl<'de> serde::Deserialize<'de> for UpdateRegistrationParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UpdateRegistrationParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：ServiceWorker.command.updateRegistration/parameters/scopeURL。
                    #[serde(rename = "scopeURL")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub scope_url: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(UpdateRegistrationParams {
                    scope_url: fields.scope_url,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：ServiceWorker.command.updateRegistration。
#[derive(Debug, Clone, Copy)]
pub struct UpdateRegistration;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：ServiceWorker.command.updateRegistration/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct UpdateRegistrationResponse {}
impl<'de> serde::Deserialize<'de> for UpdateRegistrationResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UpdateRegistrationResponse;
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
                Ok(UpdateRegistrationResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for UpdateRegistration {
    type Params = UpdateRegistrationParams;
    type Response = UpdateRegistrationResponse;
    const METHOD: &'static str = "ServiceWorker.updateRegistration";
}
