///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BackgroundService.event.backgroundServiceEventReceived/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct BackgroundServiceEventReceivedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.event.backgroundServiceEventReceived/parameters/backgroundServiceEvent。
    #[serde(rename = "backgroundServiceEvent")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub background_service_event: crate::background_service::types::BackgroundServiceEvent,
}
impl<'de> serde::Deserialize<'de> for BackgroundServiceEventReceivedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BackgroundServiceEventReceivedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.event.backgroundServiceEventReceived/parameters/backgroundServiceEvent。
                    #[serde(rename = "backgroundServiceEvent")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub background_service_event:
                        crate::background_service::types::BackgroundServiceEvent,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(BackgroundServiceEventReceivedParams {
                    background_service_event: fields.background_service_event,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：BackgroundService.event.backgroundServiceEventReceived。
///
///协议原文：
///
///<pre>Called with all existing backgroundServiceEvents when enabled, and all new
///events afterwards if enabled and recording.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct BackgroundServiceEventReceived;
impl ydp_core::Event for BackgroundServiceEventReceived {
    type Params = BackgroundServiceEventReceivedParams;
    const METHOD: &'static str = "BackgroundService.backgroundServiceEventReceived";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：BackgroundService.event.recordingStateChanged/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RecordingStateChangedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.event.recordingStateChanged/parameters/isRecording。
    #[serde(rename = "isRecording")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub is_recording: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.event.recordingStateChanged/parameters/service。
    #[serde(rename = "service")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub service: crate::background_service::types::ServiceName,
}
impl<'de> serde::Deserialize<'de> for RecordingStateChangedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RecordingStateChangedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.event.recordingStateChanged/parameters/isRecording。
                    #[serde(rename = "isRecording")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub is_recording: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：BackgroundService.event.recordingStateChanged/parameters/service。
                    #[serde(rename = "service")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub service: crate::background_service::types::ServiceName,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RecordingStateChangedParams {
                    is_recording: fields.is_recording,
                    service: fields.service,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：BackgroundService.event.recordingStateChanged。
///
///协议原文：
///
///<pre>Called when the recording state for the service has been updated.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RecordingStateChanged;
impl ydp_core::Event for RecordingStateChanged {
    type Params = RecordingStateChangedParams;
    const METHOD: &'static str = "BackgroundService.recordingStateChanged";
}
