///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BackgroundService.command.clearEvents/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearEventsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.command.clearEvents/parameters/service。
    #[serde(rename = "service")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub service: crate::background_service::types::ServiceName,
}
impl<'de> serde::Deserialize<'de> for ClearEventsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearEventsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.command.clearEvents/parameters/service。
                    #[serde(rename = "service")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub service: crate::background_service::types::ServiceName,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ClearEventsParams {
                    service: fields.service,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：BackgroundService.command.clearEvents。
///
///协议原文：
///
///<pre>Clears all stored data for the service.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ClearEvents;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BackgroundService.command.clearEvents/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearEventsResponse {}
impl<'de> serde::Deserialize<'de> for ClearEventsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearEventsResponse;
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
                Ok(ClearEventsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ClearEvents {
    type Params = ClearEventsParams;
    type Response = ClearEventsResponse;
    const METHOD: &'static str = "BackgroundService.clearEvents";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BackgroundService.command.setRecording/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetRecordingParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.command.setRecording/parameters/shouldRecord。
    #[serde(rename = "shouldRecord")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub should_record: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.command.setRecording/parameters/service。
    #[serde(rename = "service")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub service: crate::background_service::types::ServiceName,
}
impl<'de> serde::Deserialize<'de> for SetRecordingParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetRecordingParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.command.setRecording/parameters/shouldRecord。
                    #[serde(rename = "shouldRecord")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub should_record: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.command.setRecording/parameters/service。
                    #[serde(rename = "service")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub service: crate::background_service::types::ServiceName,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetRecordingParams {
                    should_record: fields.should_record,
                    service: fields.service,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：BackgroundService.command.setRecording。
///
///协议原文：
///
///<pre>Set the recording state for the service.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetRecording;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BackgroundService.command.setRecording/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetRecordingResponse {}
impl<'de> serde::Deserialize<'de> for SetRecordingResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetRecordingResponse;
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
                Ok(SetRecordingResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetRecording {
    type Params = SetRecordingParams;
    type Response = SetRecordingResponse;
    const METHOD: &'static str = "BackgroundService.setRecording";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BackgroundService.command.startObserving/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StartObservingParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.command.startObserving/parameters/service。
    #[serde(rename = "service")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub service: crate::background_service::types::ServiceName,
}
impl<'de> serde::Deserialize<'de> for StartObservingParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StartObservingParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.command.startObserving/parameters/service。
                    #[serde(rename = "service")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub service: crate::background_service::types::ServiceName,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StartObservingParams {
                    service: fields.service,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：BackgroundService.command.startObserving。
///
///协议原文：
///
///<pre>Enables event updates for the service.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct StartObserving;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BackgroundService.command.startObserving/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StartObservingResponse {}
impl<'de> serde::Deserialize<'de> for StartObservingResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StartObservingResponse;
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
                Ok(StartObservingResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for StartObserving {
    type Params = StartObservingParams;
    type Response = StartObservingResponse;
    const METHOD: &'static str = "BackgroundService.startObserving";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BackgroundService.command.stopObserving/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StopObservingParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.command.stopObserving/parameters/service。
    #[serde(rename = "service")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub service: crate::background_service::types::ServiceName,
}
impl<'de> serde::Deserialize<'de> for StopObservingParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StopObservingParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.command.stopObserving/parameters/service。
                    #[serde(rename = "service")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub service: crate::background_service::types::ServiceName,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StopObservingParams {
                    service: fields.service,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：BackgroundService.command.stopObserving。
///
///协议原文：
///
///<pre>Disables event updates for the service.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct StopObserving;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BackgroundService.command.stopObserving/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StopObservingResponse {}
impl<'de> serde::Deserialize<'de> for StopObservingResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StopObservingResponse;
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
                Ok(StopObservingResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for StopObserving {
    type Params = StopObservingParams;
    type Response = StopObservingResponse;
    const METHOD: &'static str = "BackgroundService.stopObserving";
}
