///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.dataReceived/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DataReceivedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.dataReceived/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Request identifier.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.dataReceived/parameters/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Timestamp.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.dataReceived/parameters/dataLength。
    ///
    ///协议原文：
    ///
    ///<pre>Data chunk length.</pre>
    ///
    ///
    #[serde(rename = "dataLength")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub data_length: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.dataReceived/parameters/encodedDataLength。
    ///
    ///协议原文：
    ///
    ///<pre>Actual bytes received (might be less than dataLength for compressed encodings).</pre>
    ///
    ///
    #[serde(rename = "encodedDataLength")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub encoded_data_length: i64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.dataReceived/parameters/data。
    ///
    ///协议原文：
    ///
    ///<pre>Data that was received. (Encoded as a base64 string when passed over JSON)</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "data")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub data: Option<String>,
}
impl<'de> serde::Deserialize<'de> for DataReceivedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DataReceivedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.dataReceived/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.dataReceived/parameters/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Timestamp.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.dataReceived/parameters/dataLength。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Data chunk length.</pre>
                    ///
                    ///
                    #[serde(rename = "dataLength")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub data_length: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.dataReceived/parameters/encodedDataLength。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Actual bytes received (might be less than dataLength for compressed encodings).</pre>
                    ///
                    ///
                    #[serde(rename = "encodedDataLength")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub encoded_data_length: i64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.dataReceived/parameters/data。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Data that was received. (Encoded as a base64 string when passed over JSON)</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "data")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub data: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DataReceivedParams {
                    request_id: fields.request_id,
                    timestamp: fields.timestamp,
                    data_length: fields.data_length,
                    encoded_data_length: fields.encoded_data_length,
                    data: fields.data,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.dataReceived。
///
///协议原文：
///
///<pre>Fired when data chunk was received over the network.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct DataReceived;
impl ydp_core::Event for DataReceived {
    type Params = DataReceivedParams;
    const METHOD: &'static str = "Network.dataReceived";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.deviceBoundSessionEventOccurred/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeviceBoundSessionEventOccurredParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.deviceBoundSessionEventOccurred/parameters/eventId。
    ///
    ///协议原文：
    ///
    ///<pre>A unique identifier for this session event.</pre>
    ///
    ///
    #[serde(rename = "eventId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub event_id: crate::network::types::DeviceBoundSessionEventId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.deviceBoundSessionEventOccurred/parameters/site。
    ///
    ///协议原文：
    ///
    ///<pre>The site this session event is associated with.</pre>
    ///
    ///
    #[serde(rename = "site")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub site: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.deviceBoundSessionEventOccurred/parameters/succeeded。
    ///
    ///协议原文：
    ///
    ///<pre>Whether this event was considered successful.</pre>
    ///
    ///
    #[serde(rename = "succeeded")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub succeeded: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.deviceBoundSessionEventOccurred/parameters/sessionId。
    ///
    ///协议原文：
    ///
    ///<pre>The session ID this event is associated with. May not be populated for
    ///failed events.</pre>
    ///
    ///
    #[serde(rename = "sessionId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub session_id: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.deviceBoundSessionEventOccurred/parameters/creationEventDetails。
    ///
    ///协议原文：
    ///
    ///<pre>The below are the different session event type details. Exactly one is populated.</pre>
    ///
    ///
    #[serde(rename = "creationEventDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub creation_event_details: Option<crate::network::types::CreationEventDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.deviceBoundSessionEventOccurred/parameters/refreshEventDetails。
    #[serde(rename = "refreshEventDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub refresh_event_details: Option<crate::network::types::RefreshEventDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.deviceBoundSessionEventOccurred/parameters/terminationEventDetails。
    #[serde(rename = "terminationEventDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub termination_event_details: Option<crate::network::types::TerminationEventDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.deviceBoundSessionEventOccurred/parameters/challengeEventDetails。
    #[serde(rename = "challengeEventDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub challenge_event_details: Option<crate::network::types::ChallengeEventDetails>,
}
impl<'de> serde::Deserialize<'de> for DeviceBoundSessionEventOccurredParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceBoundSessionEventOccurredParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.deviceBoundSessionEventOccurred/parameters/eventId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A unique identifier for this session event.</pre>
                    ///
                    ///
                    #[serde(rename = "eventId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub event_id: crate::network::types::DeviceBoundSessionEventId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.deviceBoundSessionEventOccurred/parameters/site。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The site this session event is associated with.</pre>
                    ///
                    ///
                    #[serde(rename = "site")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub site: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.deviceBoundSessionEventOccurred/parameters/succeeded。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether this event was considered successful.</pre>
                    ///
                    ///
                    #[serde(rename = "succeeded")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub succeeded: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.deviceBoundSessionEventOccurred/parameters/sessionId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The session ID this event is associated with. May not be populated for
                    ///failed events.</pre>
                    ///
                    ///
                    #[serde(rename = "sessionId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub session_id: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.deviceBoundSessionEventOccurred/parameters/creationEventDetails。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The below are the different session event type details. Exactly one is populated.</pre>
                    ///
                    ///
                    #[serde(rename = "creationEventDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub creation_event_details: Option<crate::network::types::CreationEventDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.deviceBoundSessionEventOccurred/parameters/refreshEventDetails。
                    #[serde(rename = "refreshEventDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub refresh_event_details: Option<crate::network::types::RefreshEventDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.deviceBoundSessionEventOccurred/parameters/terminationEventDetails。
                    #[serde(rename = "terminationEventDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub termination_event_details:
                        Option<crate::network::types::TerminationEventDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.deviceBoundSessionEventOccurred/parameters/challengeEventDetails。
                    #[serde(rename = "challengeEventDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub challenge_event_details:
                        Option<crate::network::types::ChallengeEventDetails>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DeviceBoundSessionEventOccurredParams {
                    event_id: fields.event_id,
                    site: fields.site,
                    succeeded: fields.succeeded,
                    session_id: fields.session_id,
                    creation_event_details: fields.creation_event_details,
                    refresh_event_details: fields.refresh_event_details,
                    termination_event_details: fields.termination_event_details,
                    challenge_event_details: fields.challenge_event_details,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.deviceBoundSessionEventOccurred。
///
///协议原文：
///
///<pre>Triggered when a device bound session event occurs.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct DeviceBoundSessionEventOccurred;
impl ydp_core::Event for DeviceBoundSessionEventOccurred {
    type Params = DeviceBoundSessionEventOccurredParams;
    const METHOD: &'static str = "Network.deviceBoundSessionEventOccurred";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.deviceBoundSessionsAdded/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeviceBoundSessionsAddedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.deviceBoundSessionsAdded/parameters/sessions。
    ///
    ///协议原文：
    ///
    ///<pre>The device bound sessions.</pre>
    ///
    ///
    #[serde(rename = "sessions")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub sessions: Vec<crate::network::types::DeviceBoundSession>,
}
impl<'de> serde::Deserialize<'de> for DeviceBoundSessionsAddedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceBoundSessionsAddedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.deviceBoundSessionsAdded/parameters/sessions。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The device bound sessions.</pre>
                    ///
                    ///
                    #[serde(rename = "sessions")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub sessions: Vec<crate::network::types::DeviceBoundSession>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DeviceBoundSessionsAddedParams {
                    sessions: fields.sessions,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.deviceBoundSessionsAdded。
///
///协议原文：
///
///<pre>Triggered when the initial set of device bound sessions is added.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct DeviceBoundSessionsAdded;
impl ydp_core::Event for DeviceBoundSessionsAdded {
    type Params = DeviceBoundSessionsAddedParams;
    const METHOD: &'static str = "Network.deviceBoundSessionsAdded";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.directTCPSocketAborted/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DirectTcpSocketAbortedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketAborted/parameters/identifier。
    #[serde(rename = "identifier")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub identifier: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketAborted/parameters/errorMessage。
    #[serde(rename = "errorMessage")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub error_message: crate::network::types::ErrorReason,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketAborted/parameters/timestamp。
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
}
impl<'de> serde::Deserialize<'de> for DirectTcpSocketAbortedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DirectTcpSocketAbortedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketAborted/parameters/identifier。
                    #[serde(rename = "identifier")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub identifier: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketAborted/parameters/errorMessage。
                    #[serde(rename = "errorMessage")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub error_message: crate::network::types::ErrorReason,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketAborted/parameters/timestamp。
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DirectTcpSocketAbortedParams {
                    identifier: fields.identifier,
                    error_message: fields.error_message,
                    timestamp: fields.timestamp,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.directTCPSocketAborted。
///
///协议原文：
///
///<pre>Fired when direct_socket.TCPSocket is aborted.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct DirectTcpSocketAborted;
impl ydp_core::Event for DirectTcpSocketAborted {
    type Params = DirectTcpSocketAbortedParams;
    const METHOD: &'static str = "Network.directTCPSocketAborted";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.directTCPSocketChunkReceived/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DirectTcpSocketChunkReceivedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketChunkReceived/parameters/identifier。
    #[serde(rename = "identifier")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub identifier: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketChunkReceived/parameters/data。
    #[serde(rename = "data")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub data: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketChunkReceived/parameters/timestamp。
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
}
impl<'de> serde::Deserialize<'de> for DirectTcpSocketChunkReceivedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DirectTcpSocketChunkReceivedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketChunkReceived/parameters/identifier。
                    #[serde(rename = "identifier")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub identifier: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketChunkReceived/parameters/data。
                    #[serde(rename = "data")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub data: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketChunkReceived/parameters/timestamp。
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DirectTcpSocketChunkReceivedParams {
                    identifier: fields.identifier,
                    data: fields.data,
                    timestamp: fields.timestamp,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.directTCPSocketChunkReceived。
///
///协议原文：
///
///<pre>Fired when data is received from tcp direct socket stream.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct DirectTcpSocketChunkReceived;
impl ydp_core::Event for DirectTcpSocketChunkReceived {
    type Params = DirectTcpSocketChunkReceivedParams;
    const METHOD: &'static str = "Network.directTCPSocketChunkReceived";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.directTCPSocketChunkSent/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DirectTcpSocketChunkSentParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketChunkSent/parameters/identifier。
    #[serde(rename = "identifier")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub identifier: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketChunkSent/parameters/data。
    #[serde(rename = "data")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub data: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketChunkSent/parameters/timestamp。
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
}
impl<'de> serde::Deserialize<'de> for DirectTcpSocketChunkSentParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DirectTcpSocketChunkSentParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketChunkSent/parameters/identifier。
                    #[serde(rename = "identifier")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub identifier: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketChunkSent/parameters/data。
                    #[serde(rename = "data")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub data: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketChunkSent/parameters/timestamp。
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DirectTcpSocketChunkSentParams {
                    identifier: fields.identifier,
                    data: fields.data,
                    timestamp: fields.timestamp,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.directTCPSocketChunkSent。
///
///协议原文：
///
///<pre>Fired when data is sent to tcp direct socket stream.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct DirectTcpSocketChunkSent;
impl ydp_core::Event for DirectTcpSocketChunkSent {
    type Params = DirectTcpSocketChunkSentParams;
    const METHOD: &'static str = "Network.directTCPSocketChunkSent";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.directTCPSocketClosed/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DirectTcpSocketClosedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketClosed/parameters/identifier。
    #[serde(rename = "identifier")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub identifier: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketClosed/parameters/timestamp。
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
}
impl<'de> serde::Deserialize<'de> for DirectTcpSocketClosedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DirectTcpSocketClosedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketClosed/parameters/identifier。
                    #[serde(rename = "identifier")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub identifier: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketClosed/parameters/timestamp。
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DirectTcpSocketClosedParams {
                    identifier: fields.identifier,
                    timestamp: fields.timestamp,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.directTCPSocketClosed。
///
///协议原文：
///
///<pre>Fired when direct_socket.TCPSocket is closed.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct DirectTcpSocketClosed;
impl ydp_core::Event for DirectTcpSocketClosed {
    type Params = DirectTcpSocketClosedParams;
    const METHOD: &'static str = "Network.directTCPSocketClosed";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.directTCPSocketCreated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DirectTcpSocketCreatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketCreated/parameters/identifier。
    #[serde(rename = "identifier")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub identifier: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketCreated/parameters/remoteAddr。
    #[serde(rename = "remoteAddr")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub remote_addr: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketCreated/parameters/remotePort。
    ///
    ///协议原文：
    ///
    ///<pre>Unsigned int 16.</pre>
    ///
    ///
    #[serde(rename = "remotePort")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub remote_port: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketCreated/parameters/options。
    #[serde(rename = "options")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub options: crate::network::types::DirectTcpSocketOptions,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketCreated/parameters/timestamp。
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.directTCPSocketCreated/parameters/initiator。
    #[serde(rename = "initiator")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub initiator: Option<crate::network::types::Initiator>,
}
impl<'de> serde::Deserialize<'de> for DirectTcpSocketCreatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DirectTcpSocketCreatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketCreated/parameters/identifier。
                    #[serde(rename = "identifier")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub identifier: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketCreated/parameters/remoteAddr。
                    #[serde(rename = "remoteAddr")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub remote_addr: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketCreated/parameters/remotePort。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Unsigned int 16.</pre>
                    ///
                    ///
                    #[serde(rename = "remotePort")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub remote_port: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketCreated/parameters/options。
                    #[serde(rename = "options")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub options: crate::network::types::DirectTcpSocketOptions,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketCreated/parameters/timestamp。
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.directTCPSocketCreated/parameters/initiator。
                    #[serde(rename = "initiator")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub initiator: Option<crate::network::types::Initiator>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DirectTcpSocketCreatedParams {
                    identifier: fields.identifier,
                    remote_addr: fields.remote_addr,
                    remote_port: fields.remote_port,
                    options: fields.options,
                    timestamp: fields.timestamp,
                    initiator: fields.initiator,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.directTCPSocketCreated。
///
///协议原文：
///
///<pre>Fired upon direct_socket.TCPSocket creation.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct DirectTcpSocketCreated;
impl ydp_core::Event for DirectTcpSocketCreated {
    type Params = DirectTcpSocketCreatedParams;
    const METHOD: &'static str = "Network.directTCPSocketCreated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.directTCPSocketOpened/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DirectTcpSocketOpenedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketOpened/parameters/identifier。
    #[serde(rename = "identifier")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub identifier: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketOpened/parameters/remoteAddr。
    #[serde(rename = "remoteAddr")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub remote_addr: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketOpened/parameters/remotePort。
    ///
    ///协议原文：
    ///
    ///<pre>Expected to be unsigned integer.</pre>
    ///
    ///
    #[serde(rename = "remotePort")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub remote_port: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketOpened/parameters/timestamp。
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.directTCPSocketOpened/parameters/localAddr。
    #[serde(rename = "localAddr")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub local_addr: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.directTCPSocketOpened/parameters/localPort。
    ///
    ///协议原文：
    ///
    ///<pre>Expected to be unsigned integer.</pre>
    ///
    ///
    #[serde(rename = "localPort")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub local_port: Option<i64>,
}
impl<'de> serde::Deserialize<'de> for DirectTcpSocketOpenedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DirectTcpSocketOpenedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketOpened/parameters/identifier。
                    #[serde(rename = "identifier")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub identifier: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketOpened/parameters/remoteAddr。
                    #[serde(rename = "remoteAddr")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub remote_addr: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketOpened/parameters/remotePort。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Expected to be unsigned integer.</pre>
                    ///
                    ///
                    #[serde(rename = "remotePort")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub remote_port: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directTCPSocketOpened/parameters/timestamp。
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.directTCPSocketOpened/parameters/localAddr。
                    #[serde(rename = "localAddr")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub local_addr: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.directTCPSocketOpened/parameters/localPort。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Expected to be unsigned integer.</pre>
                    ///
                    ///
                    #[serde(rename = "localPort")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub local_port: Option<i64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DirectTcpSocketOpenedParams {
                    identifier: fields.identifier,
                    remote_addr: fields.remote_addr,
                    remote_port: fields.remote_port,
                    timestamp: fields.timestamp,
                    local_addr: fields.local_addr,
                    local_port: fields.local_port,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.directTCPSocketOpened。
///
///协议原文：
///
///<pre>Fired when direct_socket.TCPSocket connection is opened.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct DirectTcpSocketOpened;
impl ydp_core::Event for DirectTcpSocketOpened {
    type Params = DirectTcpSocketOpenedParams;
    const METHOD: &'static str = "Network.directTCPSocketOpened";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.directUDPSocketAborted/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DirectUdpSocketAbortedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketAborted/parameters/identifier。
    #[serde(rename = "identifier")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub identifier: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketAborted/parameters/errorMessage。
    #[serde(rename = "errorMessage")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub error_message: crate::network::types::ErrorReason,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketAborted/parameters/timestamp。
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
}
impl<'de> serde::Deserialize<'de> for DirectUdpSocketAbortedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DirectUdpSocketAbortedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketAborted/parameters/identifier。
                    #[serde(rename = "identifier")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub identifier: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketAborted/parameters/errorMessage。
                    #[serde(rename = "errorMessage")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub error_message: crate::network::types::ErrorReason,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketAborted/parameters/timestamp。
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DirectUdpSocketAbortedParams {
                    identifier: fields.identifier,
                    error_message: fields.error_message,
                    timestamp: fields.timestamp,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.directUDPSocketAborted。
///
///协议原文：
///
///<pre>Fired when direct_socket.UDPSocket is aborted.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct DirectUdpSocketAborted;
impl ydp_core::Event for DirectUdpSocketAborted {
    type Params = DirectUdpSocketAbortedParams;
    const METHOD: &'static str = "Network.directUDPSocketAborted";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.directUDPSocketChunkReceived/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DirectUdpSocketChunkReceivedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketChunkReceived/parameters/identifier。
    #[serde(rename = "identifier")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub identifier: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketChunkReceived/parameters/message。
    #[serde(rename = "message")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub message: crate::network::types::DirectUdpMessage,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketChunkReceived/parameters/timestamp。
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
}
impl<'de> serde::Deserialize<'de> for DirectUdpSocketChunkReceivedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DirectUdpSocketChunkReceivedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketChunkReceived/parameters/identifier。
                    #[serde(rename = "identifier")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub identifier: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketChunkReceived/parameters/message。
                    #[serde(rename = "message")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub message: crate::network::types::DirectUdpMessage,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketChunkReceived/parameters/timestamp。
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DirectUdpSocketChunkReceivedParams {
                    identifier: fields.identifier,
                    message: fields.message,
                    timestamp: fields.timestamp,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.directUDPSocketChunkReceived。
///
///协议原文：
///
///<pre>Fired when message is received from udp direct socket stream.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct DirectUdpSocketChunkReceived;
impl ydp_core::Event for DirectUdpSocketChunkReceived {
    type Params = DirectUdpSocketChunkReceivedParams;
    const METHOD: &'static str = "Network.directUDPSocketChunkReceived";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.directUDPSocketChunkSent/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DirectUdpSocketChunkSentParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketChunkSent/parameters/identifier。
    #[serde(rename = "identifier")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub identifier: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketChunkSent/parameters/message。
    #[serde(rename = "message")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub message: crate::network::types::DirectUdpMessage,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketChunkSent/parameters/timestamp。
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
}
impl<'de> serde::Deserialize<'de> for DirectUdpSocketChunkSentParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DirectUdpSocketChunkSentParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketChunkSent/parameters/identifier。
                    #[serde(rename = "identifier")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub identifier: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketChunkSent/parameters/message。
                    #[serde(rename = "message")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub message: crate::network::types::DirectUdpMessage,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketChunkSent/parameters/timestamp。
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DirectUdpSocketChunkSentParams {
                    identifier: fields.identifier,
                    message: fields.message,
                    timestamp: fields.timestamp,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.directUDPSocketChunkSent。
///
///协议原文：
///
///<pre>Fired when message is sent to udp direct socket stream.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct DirectUdpSocketChunkSent;
impl ydp_core::Event for DirectUdpSocketChunkSent {
    type Params = DirectUdpSocketChunkSentParams;
    const METHOD: &'static str = "Network.directUDPSocketChunkSent";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.directUDPSocketClosed/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DirectUdpSocketClosedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketClosed/parameters/identifier。
    #[serde(rename = "identifier")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub identifier: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketClosed/parameters/timestamp。
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
}
impl<'de> serde::Deserialize<'de> for DirectUdpSocketClosedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DirectUdpSocketClosedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketClosed/parameters/identifier。
                    #[serde(rename = "identifier")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub identifier: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketClosed/parameters/timestamp。
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DirectUdpSocketClosedParams {
                    identifier: fields.identifier,
                    timestamp: fields.timestamp,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.directUDPSocketClosed。
///
///协议原文：
///
///<pre>Fired when direct_socket.UDPSocket is closed.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct DirectUdpSocketClosed;
impl ydp_core::Event for DirectUdpSocketClosed {
    type Params = DirectUdpSocketClosedParams;
    const METHOD: &'static str = "Network.directUDPSocketClosed";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.directUDPSocketCreated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DirectUdpSocketCreatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketCreated/parameters/identifier。
    #[serde(rename = "identifier")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub identifier: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketCreated/parameters/options。
    #[serde(rename = "options")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub options: crate::network::types::DirectUdpSocketOptions,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketCreated/parameters/timestamp。
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.directUDPSocketCreated/parameters/initiator。
    #[serde(rename = "initiator")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub initiator: Option<crate::network::types::Initiator>,
}
impl<'de> serde::Deserialize<'de> for DirectUdpSocketCreatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DirectUdpSocketCreatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketCreated/parameters/identifier。
                    #[serde(rename = "identifier")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub identifier: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketCreated/parameters/options。
                    #[serde(rename = "options")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub options: crate::network::types::DirectUdpSocketOptions,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketCreated/parameters/timestamp。
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.directUDPSocketCreated/parameters/initiator。
                    #[serde(rename = "initiator")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub initiator: Option<crate::network::types::Initiator>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DirectUdpSocketCreatedParams {
                    identifier: fields.identifier,
                    options: fields.options,
                    timestamp: fields.timestamp,
                    initiator: fields.initiator,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.directUDPSocketCreated。
///
///协议原文：
///
///<pre>Fired upon direct_socket.UDPSocket creation.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct DirectUdpSocketCreated;
impl ydp_core::Event for DirectUdpSocketCreated {
    type Params = DirectUdpSocketCreatedParams;
    const METHOD: &'static str = "Network.directUDPSocketCreated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.directUDPSocketJoinedMulticastGroup/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DirectUdpSocketJoinedMulticastGroupParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketJoinedMulticastGroup/parameters/identifier。
    #[serde(rename = "identifier")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub identifier: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketJoinedMulticastGroup/parameters/IPAddress。
    #[serde(rename = "IPAddress")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub ip_address: String,
}
impl<'de> serde::Deserialize<'de> for DirectUdpSocketJoinedMulticastGroupParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DirectUdpSocketJoinedMulticastGroupParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketJoinedMulticastGroup/parameters/identifier。
                    #[serde(rename = "identifier")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub identifier: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketJoinedMulticastGroup/parameters/IPAddress。
                    #[serde(rename = "IPAddress")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub ip_address: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DirectUdpSocketJoinedMulticastGroupParams {
                    identifier: fields.identifier,
                    ip_address: fields.ip_address,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.directUDPSocketJoinedMulticastGroup。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct DirectUdpSocketJoinedMulticastGroup;
impl ydp_core::Event for DirectUdpSocketJoinedMulticastGroup {
    type Params = DirectUdpSocketJoinedMulticastGroupParams;
    const METHOD: &'static str = "Network.directUDPSocketJoinedMulticastGroup";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.directUDPSocketLeftMulticastGroup/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DirectUdpSocketLeftMulticastGroupParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketLeftMulticastGroup/parameters/identifier。
    #[serde(rename = "identifier")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub identifier: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketLeftMulticastGroup/parameters/IPAddress。
    #[serde(rename = "IPAddress")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub ip_address: String,
}
impl<'de> serde::Deserialize<'de> for DirectUdpSocketLeftMulticastGroupParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DirectUdpSocketLeftMulticastGroupParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketLeftMulticastGroup/parameters/identifier。
                    #[serde(rename = "identifier")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub identifier: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketLeftMulticastGroup/parameters/IPAddress。
                    #[serde(rename = "IPAddress")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub ip_address: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DirectUdpSocketLeftMulticastGroupParams {
                    identifier: fields.identifier,
                    ip_address: fields.ip_address,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.directUDPSocketLeftMulticastGroup。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct DirectUdpSocketLeftMulticastGroup;
impl ydp_core::Event for DirectUdpSocketLeftMulticastGroup {
    type Params = DirectUdpSocketLeftMulticastGroupParams;
    const METHOD: &'static str = "Network.directUDPSocketLeftMulticastGroup";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.directUDPSocketOpened/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DirectUdpSocketOpenedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketOpened/parameters/identifier。
    #[serde(rename = "identifier")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub identifier: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketOpened/parameters/localAddr。
    #[serde(rename = "localAddr")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub local_addr: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketOpened/parameters/localPort。
    ///
    ///协议原文：
    ///
    ///<pre>Expected to be unsigned integer.</pre>
    ///
    ///
    #[serde(rename = "localPort")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub local_port: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketOpened/parameters/timestamp。
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.directUDPSocketOpened/parameters/remoteAddr。
    #[serde(rename = "remoteAddr")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_addr: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.directUDPSocketOpened/parameters/remotePort。
    ///
    ///协议原文：
    ///
    ///<pre>Expected to be unsigned integer.</pre>
    ///
    ///
    #[serde(rename = "remotePort")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_port: Option<i64>,
}
impl<'de> serde::Deserialize<'de> for DirectUdpSocketOpenedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DirectUdpSocketOpenedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketOpened/parameters/identifier。
                    #[serde(rename = "identifier")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub identifier: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketOpened/parameters/localAddr。
                    #[serde(rename = "localAddr")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub local_addr: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketOpened/parameters/localPort。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Expected to be unsigned integer.</pre>
                    ///
                    ///
                    #[serde(rename = "localPort")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub local_port: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.directUDPSocketOpened/parameters/timestamp。
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.directUDPSocketOpened/parameters/remoteAddr。
                    #[serde(rename = "remoteAddr")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub remote_addr: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.directUDPSocketOpened/parameters/remotePort。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Expected to be unsigned integer.</pre>
                    ///
                    ///
                    #[serde(rename = "remotePort")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub remote_port: Option<i64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DirectUdpSocketOpenedParams {
                    identifier: fields.identifier,
                    local_addr: fields.local_addr,
                    local_port: fields.local_port,
                    timestamp: fields.timestamp,
                    remote_addr: fields.remote_addr,
                    remote_port: fields.remote_port,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.directUDPSocketOpened。
///
///协议原文：
///
///<pre>Fired when direct_socket.UDPSocket connection is opened.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct DirectUdpSocketOpened;
impl ydp_core::Event for DirectUdpSocketOpened {
    type Params = DirectUdpSocketOpenedParams;
    const METHOD: &'static str = "Network.directUDPSocketOpened";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.eventSourceMessageReceived/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EventSourceMessageReceivedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.eventSourceMessageReceived/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Request identifier.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.eventSourceMessageReceived/parameters/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Timestamp.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.eventSourceMessageReceived/parameters/eventName。
    ///
    ///协议原文：
    ///
    ///<pre>Message type.</pre>
    ///
    ///
    #[serde(rename = "eventName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub event_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.eventSourceMessageReceived/parameters/eventId。
    ///
    ///协议原文：
    ///
    ///<pre>Message identifier.</pre>
    ///
    ///
    #[serde(rename = "eventId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub event_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.eventSourceMessageReceived/parameters/data。
    ///
    ///协议原文：
    ///
    ///<pre>Message content.</pre>
    ///
    ///
    #[serde(rename = "data")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub data: String,
}
impl<'de> serde::Deserialize<'de> for EventSourceMessageReceivedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EventSourceMessageReceivedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.eventSourceMessageReceived/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.eventSourceMessageReceived/parameters/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Timestamp.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.eventSourceMessageReceived/parameters/eventName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Message type.</pre>
                    ///
                    ///
                    #[serde(rename = "eventName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub event_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.eventSourceMessageReceived/parameters/eventId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Message identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "eventId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub event_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.eventSourceMessageReceived/parameters/data。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Message content.</pre>
                    ///
                    ///
                    #[serde(rename = "data")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub data: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EventSourceMessageReceivedParams {
                    request_id: fields.request_id,
                    timestamp: fields.timestamp,
                    event_name: fields.event_name,
                    event_id: fields.event_id,
                    data: fields.data,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.eventSourceMessageReceived。
///
///协议原文：
///
///<pre>Fired when EventSource message is received.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct EventSourceMessageReceived;
impl ydp_core::Event for EventSourceMessageReceived {
    type Params = EventSourceMessageReceivedParams;
    const METHOD: &'static str = "Network.eventSourceMessageReceived";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.loadingFailed/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct LoadingFailedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.loadingFailed/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Request identifier.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.loadingFailed/parameters/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Timestamp.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.loadingFailed/parameters/type。
    ///
    ///协议原文：
    ///
    ///<pre>Resource type.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: crate::network::types::ResourceType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.loadingFailed/parameters/errorText。
    ///
    ///协议原文：
    ///
    ///<pre>Error message. List of network errors: https://cs.chromium.org/chromium/src/net/base/net_error_list.h</pre>
    ///
    ///
    #[serde(rename = "errorText")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub error_text: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.loadingFailed/parameters/canceled。
    ///
    ///协议原文：
    ///
    ///<pre>True if loading was canceled.</pre>
    ///
    ///
    #[serde(rename = "canceled")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub canceled: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.loadingFailed/parameters/blockedReason。
    ///
    ///协议原文：
    ///
    ///<pre>The reason why loading was blocked, if any.</pre>
    ///
    ///
    #[serde(rename = "blockedReason")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub blocked_reason: Option<crate::network::types::BlockedReason>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.loadingFailed/parameters/corsErrorStatus。
    ///
    ///协议原文：
    ///
    ///<pre>The reason why loading was blocked by CORS, if any.</pre>
    ///
    ///
    #[serde(rename = "corsErrorStatus")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub cors_error_status: Option<crate::network::types::CorsErrorStatus>,
}
impl<'de> serde::Deserialize<'de> for LoadingFailedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LoadingFailedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.loadingFailed/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.loadingFailed/parameters/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Timestamp.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.loadingFailed/parameters/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Resource type.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: crate::network::types::ResourceType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.loadingFailed/parameters/errorText。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Error message. List of network errors: https://cs.chromium.org/chromium/src/net/base/net_error_list.h</pre>
                    ///
                    ///
                    #[serde(rename = "errorText")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub error_text: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.loadingFailed/parameters/canceled。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if loading was canceled.</pre>
                    ///
                    ///
                    #[serde(rename = "canceled")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub canceled: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.loadingFailed/parameters/blockedReason。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The reason why loading was blocked, if any.</pre>
                    ///
                    ///
                    #[serde(rename = "blockedReason")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub blocked_reason: Option<crate::network::types::BlockedReason>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.loadingFailed/parameters/corsErrorStatus。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The reason why loading was blocked by CORS, if any.</pre>
                    ///
                    ///
                    #[serde(rename = "corsErrorStatus")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub cors_error_status: Option<crate::network::types::CorsErrorStatus>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LoadingFailedParams {
                    request_id: fields.request_id,
                    timestamp: fields.timestamp,
                    r#type: fields.r#type,
                    error_text: fields.error_text,
                    canceled: fields.canceled,
                    blocked_reason: fields.blocked_reason,
                    cors_error_status: fields.cors_error_status,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.loadingFailed。
///
///协议原文：
///
///<pre>Fired when HTTP request has failed to load.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct LoadingFailed;
impl ydp_core::Event for LoadingFailed {
    type Params = LoadingFailedParams;
    const METHOD: &'static str = "Network.loadingFailed";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.loadingFinished/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct LoadingFinishedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.loadingFinished/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Request identifier.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.loadingFinished/parameters/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Timestamp.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.loadingFinished/parameters/encodedDataLength。
    ///
    ///协议原文：
    ///
    ///<pre>Total number of bytes received for this request.</pre>
    ///
    ///
    #[serde(rename = "encodedDataLength")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub encoded_data_length: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for LoadingFinishedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LoadingFinishedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.loadingFinished/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.loadingFinished/parameters/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Timestamp.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.loadingFinished/parameters/encodedDataLength。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Total number of bytes received for this request.</pre>
                    ///
                    ///
                    #[serde(rename = "encodedDataLength")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub encoded_data_length: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LoadingFinishedParams {
                    request_id: fields.request_id,
                    timestamp: fields.timestamp,
                    encoded_data_length: fields.encoded_data_length,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.loadingFinished。
///
///协议原文：
///
///<pre>Fired when HTTP request has finished loading.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct LoadingFinished;
impl ydp_core::Event for LoadingFinished {
    type Params = LoadingFinishedParams;
    const METHOD: &'static str = "Network.loadingFinished";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.policyUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct PolicyUpdatedParams {}
impl<'de> serde::Deserialize<'de> for PolicyUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PolicyUpdatedParams;
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
                Ok(PolicyUpdatedParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.policyUpdated。
///
///协议原文：
///
///<pre>Fired once security policy has been updated.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct PolicyUpdated;
impl ydp_core::Event for PolicyUpdated {
    type Params = PolicyUpdatedParams;
    const METHOD: &'static str = "Network.policyUpdated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.reportingApiEndpointsChangedForOrigin/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReportingApiEndpointsChangedForOriginParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.reportingApiEndpointsChangedForOrigin/parameters/origin。
    ///
    ///协议原文：
    ///
    ///<pre>Origin of the document(s) which configured the endpoints.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.reportingApiEndpointsChangedForOrigin/parameters/endpoints。
    #[serde(rename = "endpoints")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub endpoints: Vec<crate::network::types::ReportingApiEndpoint>,
}
impl<'de> serde::Deserialize<'de> for ReportingApiEndpointsChangedForOriginParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReportingApiEndpointsChangedForOriginParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.reportingApiEndpointsChangedForOrigin/parameters/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Origin of the document(s) which configured the endpoints.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.reportingApiEndpointsChangedForOrigin/parameters/endpoints。
                    #[serde(rename = "endpoints")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub endpoints: Vec<crate::network::types::ReportingApiEndpoint>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReportingApiEndpointsChangedForOriginParams {
                    origin: fields.origin,
                    endpoints: fields.endpoints,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.reportingApiEndpointsChangedForOrigin。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ReportingApiEndpointsChangedForOrigin;
impl ydp_core::Event for ReportingApiEndpointsChangedForOrigin {
    type Params = ReportingApiEndpointsChangedForOriginParams;
    const METHOD: &'static str = "Network.reportingApiEndpointsChangedForOrigin";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.reportingApiReportAdded/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReportingApiReportAddedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.reportingApiReportAdded/parameters/report。
    #[serde(rename = "report")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub report: crate::network::types::ReportingApiReport,
}
impl<'de> serde::Deserialize<'de> for ReportingApiReportAddedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReportingApiReportAddedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.reportingApiReportAdded/parameters/report。
                    #[serde(rename = "report")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub report: crate::network::types::ReportingApiReport,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReportingApiReportAddedParams {
                    report: fields.report,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.reportingApiReportAdded。
///
///协议原文：
///
///<pre>Is sent whenever a new report is added.
///And after 'enableReportingApi' for all existing reports.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ReportingApiReportAdded;
impl ydp_core::Event for ReportingApiReportAdded {
    type Params = ReportingApiReportAddedParams;
    const METHOD: &'static str = "Network.reportingApiReportAdded";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.reportingApiReportUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReportingApiReportUpdatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.reportingApiReportUpdated/parameters/report。
    #[serde(rename = "report")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub report: crate::network::types::ReportingApiReport,
}
impl<'de> serde::Deserialize<'de> for ReportingApiReportUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReportingApiReportUpdatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.reportingApiReportUpdated/parameters/report。
                    #[serde(rename = "report")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub report: crate::network::types::ReportingApiReport,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReportingApiReportUpdatedParams {
                    report: fields.report,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.reportingApiReportUpdated。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ReportingApiReportUpdated;
impl ydp_core::Event for ReportingApiReportUpdated {
    type Params = ReportingApiReportUpdatedParams;
    const METHOD: &'static str = "Network.reportingApiReportUpdated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.requestServedFromCache/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequestServedFromCacheParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.requestServedFromCache/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Request identifier.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::network::types::RequestId,
}
impl<'de> serde::Deserialize<'de> for RequestServedFromCacheParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RequestServedFromCacheParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.requestServedFromCache/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::network::types::RequestId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RequestServedFromCacheParams {
                    request_id: fields.request_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.requestServedFromCache。
///
///协议原文：
///
///<pre>Fired if request ended up loading from cache.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RequestServedFromCache;
impl ydp_core::Event for RequestServedFromCache {
    type Params = RequestServedFromCacheParams;
    const METHOD: &'static str = "Network.requestServedFromCache";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.requestWillBeSent/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequestWillBeSentParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.requestWillBeSent/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Request identifier.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.requestWillBeSent/parameters/loaderId。
    ///
    ///协议原文：
    ///
    ///<pre>Loader identifier. Empty string if the request is fetched from worker.</pre>
    ///
    ///
    #[serde(rename = "loaderId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub loader_id: crate::network::types::LoaderId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.requestWillBeSent/parameters/documentURL。
    ///
    ///协议原文：
    ///
    ///<pre>URL of the document this request is loaded for.</pre>
    ///
    ///
    #[serde(rename = "documentURL")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub document_url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.requestWillBeSent/parameters/request。
    ///
    ///协议原文：
    ///
    ///<pre>Request data.</pre>
    ///
    ///
    #[serde(rename = "request")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request: crate::network::types::Request,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.requestWillBeSent/parameters/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Timestamp.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.requestWillBeSent/parameters/wallTime。
    ///
    ///协议原文：
    ///
    ///<pre>Timestamp.</pre>
    ///
    ///
    #[serde(rename = "wallTime")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub wall_time: crate::network::types::TimeSinceEpoch,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.requestWillBeSent/parameters/initiator。
    ///
    ///协议原文：
    ///
    ///<pre>Request initiator.</pre>
    ///
    ///
    #[serde(rename = "initiator")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub initiator: crate::network::types::Initiator,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.requestWillBeSent/parameters/redirectHasExtraInfo。
    ///
    ///协议原文：
    ///
    ///<pre>In the case that redirectResponse is populated, this flag indicates whether
    ///requestWillBeSentExtraInfo and responseReceivedExtraInfo events will be or were emitted
    ///for the request which was just redirected.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "redirectHasExtraInfo")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub redirect_has_extra_info: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.requestWillBeSent/parameters/redirectResponse。
    ///
    ///协议原文：
    ///
    ///<pre>Redirect response data.</pre>
    ///
    ///
    #[serde(rename = "redirectResponse")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub redirect_response: Option<crate::network::types::Response>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.requestWillBeSent/parameters/type。
    ///
    ///协议原文：
    ///
    ///<pre>Type of this resource.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<crate::network::types::ResourceType>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.requestWillBeSent/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Frame identifier.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub frame_id: Option<crate::page::types::FrameId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.requestWillBeSent/parameters/hasUserGesture。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the request is initiated by a user gesture. Defaults to false.</pre>
    ///
    ///
    #[serde(rename = "hasUserGesture")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_user_gesture: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.requestWillBeSent/parameters/renderBlockingBehavior。
    ///
    ///协议原文：
    ///
    ///<pre>The render-blocking behavior of the request.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "renderBlockingBehavior")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub render_blocking_behavior: Option<crate::network::types::RenderBlockingBehavior>,
}
impl<'de> serde::Deserialize<'de> for RequestWillBeSentParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RequestWillBeSentParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.requestWillBeSent/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.requestWillBeSent/parameters/loaderId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Loader identifier. Empty string if the request is fetched from worker.</pre>
                    ///
                    ///
                    #[serde(rename = "loaderId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub loader_id: crate::network::types::LoaderId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.requestWillBeSent/parameters/documentURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>URL of the document this request is loaded for.</pre>
                    ///
                    ///
                    #[serde(rename = "documentURL")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub document_url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.requestWillBeSent/parameters/request。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request data.</pre>
                    ///
                    ///
                    #[serde(rename = "request")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request: crate::network::types::Request,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.requestWillBeSent/parameters/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Timestamp.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.requestWillBeSent/parameters/wallTime。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Timestamp.</pre>
                    ///
                    ///
                    #[serde(rename = "wallTime")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub wall_time: crate::network::types::TimeSinceEpoch,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.requestWillBeSent/parameters/initiator。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request initiator.</pre>
                    ///
                    ///
                    #[serde(rename = "initiator")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub initiator: crate::network::types::Initiator,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.requestWillBeSent/parameters/redirectHasExtraInfo。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>In the case that redirectResponse is populated, this flag indicates whether
                    ///requestWillBeSentExtraInfo and responseReceivedExtraInfo events will be or were emitted
                    ///for the request which was just redirected.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "redirectHasExtraInfo")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub redirect_has_extra_info: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.requestWillBeSent/parameters/redirectResponse。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Redirect response data.</pre>
                    ///
                    ///
                    #[serde(rename = "redirectResponse")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub redirect_response: Option<crate::network::types::Response>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.requestWillBeSent/parameters/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Type of this resource.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub r#type: Option<crate::network::types::ResourceType>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.requestWillBeSent/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub frame_id: Option<crate::page::types::FrameId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.requestWillBeSent/parameters/hasUserGesture。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the request is initiated by a user gesture. Defaults to false.</pre>
                    ///
                    ///
                    #[serde(rename = "hasUserGesture")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub has_user_gesture: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.requestWillBeSent/parameters/renderBlockingBehavior。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The render-blocking behavior of the request.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "renderBlockingBehavior")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub render_blocking_behavior:
                        Option<crate::network::types::RenderBlockingBehavior>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RequestWillBeSentParams {
                    request_id: fields.request_id,
                    loader_id: fields.loader_id,
                    document_url: fields.document_url,
                    request: fields.request,
                    timestamp: fields.timestamp,
                    wall_time: fields.wall_time,
                    initiator: fields.initiator,
                    redirect_has_extra_info: fields.redirect_has_extra_info,
                    redirect_response: fields.redirect_response,
                    r#type: fields.r#type,
                    frame_id: fields.frame_id,
                    has_user_gesture: fields.has_user_gesture,
                    render_blocking_behavior: fields.render_blocking_behavior,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.requestWillBeSent。
///
///协议原文：
///
///<pre>Fired when page is about to send HTTP request.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RequestWillBeSent;
impl ydp_core::Event for RequestWillBeSent {
    type Params = RequestWillBeSentParams;
    const METHOD: &'static str = "Network.requestWillBeSent";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.requestWillBeSentExtraInfo/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequestWillBeSentExtraInfoParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.requestWillBeSentExtraInfo/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Request identifier. Used to match this information to an existing requestWillBeSent event.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.requestWillBeSentExtraInfo/parameters/associatedCookies。
    ///
    ///协议原文：
    ///
    ///<pre>A list of cookies potentially associated to the requested URL. This includes both cookies sent with
    ///the request and the ones not sent; the latter are distinguished by having blockedReasons field set.</pre>
    ///
    ///
    #[serde(rename = "associatedCookies")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub associated_cookies: Vec<crate::network::types::AssociatedCookie>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.requestWillBeSentExtraInfo/parameters/headers。
    ///
    ///协议原文：
    ///
    ///<pre>Raw request headers as they will be sent over the wire.</pre>
    ///
    ///
    #[serde(rename = "headers")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub headers: crate::network::types::Headers,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.requestWillBeSentExtraInfo/parameters/connectTiming。
    ///
    ///协议原文：
    ///
    ///<pre>Connection timing information for the request.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "connectTiming")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub connect_timing: crate::network::types::ConnectTiming,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.requestWillBeSentExtraInfo/parameters/deviceBoundSessionUsages。
    ///
    ///协议原文：
    ///
    ///<pre>How the request site's device bound sessions were used during this request.</pre>
    ///
    ///
    #[serde(rename = "deviceBoundSessionUsages")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub device_bound_session_usages:
        Option<Vec<crate::network::types::DeviceBoundSessionWithUsage>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.requestWillBeSentExtraInfo/parameters/clientSecurityState。
    ///
    ///协议原文：
    ///
    ///<pre>The client security state set for the request.</pre>
    ///
    ///
    #[serde(rename = "clientSecurityState")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub client_security_state: Option<crate::network::types::ClientSecurityState>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.requestWillBeSentExtraInfo/parameters/siteHasCookieInOtherPartition。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the site has partitioned cookies stored in a partition different than the current one.</pre>
    ///
    ///
    #[serde(rename = "siteHasCookieInOtherPartition")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub site_has_cookie_in_other_partition: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.requestWillBeSentExtraInfo/parameters/appliedNetworkConditionsId。
    ///
    ///协议原文：
    ///
    ///<pre>The network conditions id if this request was affected by network conditions configured via
    ///emulateNetworkConditionsByRule.</pre>
    ///
    ///
    #[serde(rename = "appliedNetworkConditionsId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub applied_network_conditions_id: Option<String>,
}
impl<'de> serde::Deserialize<'de> for RequestWillBeSentExtraInfoParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RequestWillBeSentExtraInfoParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.requestWillBeSentExtraInfo/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request identifier. Used to match this information to an existing requestWillBeSent event.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.requestWillBeSentExtraInfo/parameters/associatedCookies。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A list of cookies potentially associated to the requested URL. This includes both cookies sent with
                    ///the request and the ones not sent; the latter are distinguished by having blockedReasons field set.</pre>
                    ///
                    ///
                    #[serde(rename = "associatedCookies")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub associated_cookies: Vec<crate::network::types::AssociatedCookie>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.requestWillBeSentExtraInfo/parameters/headers。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Raw request headers as they will be sent over the wire.</pre>
                    ///
                    ///
                    #[serde(rename = "headers")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub headers: crate::network::types::Headers,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.requestWillBeSentExtraInfo/parameters/connectTiming。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Connection timing information for the request.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "connectTiming")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub connect_timing: crate::network::types::ConnectTiming,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.requestWillBeSentExtraInfo/parameters/deviceBoundSessionUsages。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>How the request site's device bound sessions were used during this request.</pre>
                    ///
                    ///
                    #[serde(rename = "deviceBoundSessionUsages")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub device_bound_session_usages:
                        Option<Vec<crate::network::types::DeviceBoundSessionWithUsage>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.requestWillBeSentExtraInfo/parameters/clientSecurityState。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The client security state set for the request.</pre>
                    ///
                    ///
                    #[serde(rename = "clientSecurityState")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub client_security_state: Option<crate::network::types::ClientSecurityState>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.requestWillBeSentExtraInfo/parameters/siteHasCookieInOtherPartition。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the site has partitioned cookies stored in a partition different than the current one.</pre>
                    ///
                    ///
                    #[serde(rename = "siteHasCookieInOtherPartition")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub site_has_cookie_in_other_partition: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.requestWillBeSentExtraInfo/parameters/appliedNetworkConditionsId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The network conditions id if this request was affected by network conditions configured via
                    ///emulateNetworkConditionsByRule.</pre>
                    ///
                    ///
                    #[serde(rename = "appliedNetworkConditionsId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub applied_network_conditions_id: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RequestWillBeSentExtraInfoParams {
                    request_id: fields.request_id,
                    associated_cookies: fields.associated_cookies,
                    headers: fields.headers,
                    connect_timing: fields.connect_timing,
                    device_bound_session_usages: fields.device_bound_session_usages,
                    client_security_state: fields.client_security_state,
                    site_has_cookie_in_other_partition: fields.site_has_cookie_in_other_partition,
                    applied_network_conditions_id: fields.applied_network_conditions_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.requestWillBeSentExtraInfo。
///
///协议原文：
///
///<pre>Fired when additional information about a requestWillBeSent event is available from the
///network stack. Not every requestWillBeSent event will have an additional
///requestWillBeSentExtraInfo fired for it, and there is no guarantee whether requestWillBeSent
///or requestWillBeSentExtraInfo will be fired first for the same request.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct RequestWillBeSentExtraInfo;
impl ydp_core::Event for RequestWillBeSentExtraInfo {
    type Params = RequestWillBeSentExtraInfoParams;
    const METHOD: &'static str = "Network.requestWillBeSentExtraInfo";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.resourceChangedPriority/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ResourceChangedPriorityParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.resourceChangedPriority/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Request identifier.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.resourceChangedPriority/parameters/newPriority。
    ///
    ///协议原文：
    ///
    ///<pre>New priority</pre>
    ///
    ///
    #[serde(rename = "newPriority")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub new_priority: crate::network::types::ResourcePriority,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.resourceChangedPriority/parameters/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Timestamp.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
}
impl<'de> serde::Deserialize<'de> for ResourceChangedPriorityParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResourceChangedPriorityParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.resourceChangedPriority/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.resourceChangedPriority/parameters/newPriority。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>New priority</pre>
                    ///
                    ///
                    #[serde(rename = "newPriority")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub new_priority: crate::network::types::ResourcePriority,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.resourceChangedPriority/parameters/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Timestamp.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ResourceChangedPriorityParams {
                    request_id: fields.request_id,
                    new_priority: fields.new_priority,
                    timestamp: fields.timestamp,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.resourceChangedPriority。
///
///协议原文：
///
///<pre>Fired when resource loading priority is changed</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ResourceChangedPriority;
impl ydp_core::Event for ResourceChangedPriority {
    type Params = ResourceChangedPriorityParams;
    const METHOD: &'static str = "Network.resourceChangedPriority";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.responseReceived/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ResponseReceivedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.responseReceived/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Request identifier.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.responseReceived/parameters/loaderId。
    ///
    ///协议原文：
    ///
    ///<pre>Loader identifier. Empty string if the request is fetched from worker.</pre>
    ///
    ///
    #[serde(rename = "loaderId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub loader_id: crate::network::types::LoaderId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.responseReceived/parameters/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Timestamp.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.responseReceived/parameters/type。
    ///
    ///协议原文：
    ///
    ///<pre>Resource type.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: crate::network::types::ResourceType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.responseReceived/parameters/response。
    ///
    ///协议原文：
    ///
    ///<pre>Response data.</pre>
    ///
    ///
    #[serde(rename = "response")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub response: crate::network::types::Response,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.responseReceived/parameters/hasExtraInfo。
    ///
    ///协议原文：
    ///
    ///<pre>Indicates whether requestWillBeSentExtraInfo and responseReceivedExtraInfo events will be
    ///or were emitted for this request.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "hasExtraInfo")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub has_extra_info: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.responseReceived/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Frame identifier.</pre>
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
impl<'de> serde::Deserialize<'de> for ResponseReceivedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResponseReceivedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.responseReceived/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.responseReceived/parameters/loaderId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Loader identifier. Empty string if the request is fetched from worker.</pre>
                    ///
                    ///
                    #[serde(rename = "loaderId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub loader_id: crate::network::types::LoaderId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.responseReceived/parameters/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Timestamp.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.responseReceived/parameters/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Resource type.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: crate::network::types::ResourceType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.responseReceived/parameters/response。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Response data.</pre>
                    ///
                    ///
                    #[serde(rename = "response")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub response: crate::network::types::Response,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.responseReceived/parameters/hasExtraInfo。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Indicates whether requestWillBeSentExtraInfo and responseReceivedExtraInfo events will be
                    ///or were emitted for this request.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "hasExtraInfo")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub has_extra_info: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.responseReceived/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame identifier.</pre>
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
                Ok(ResponseReceivedParams {
                    request_id: fields.request_id,
                    loader_id: fields.loader_id,
                    timestamp: fields.timestamp,
                    r#type: fields.r#type,
                    response: fields.response,
                    has_extra_info: fields.has_extra_info,
                    frame_id: fields.frame_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.responseReceived。
///
///协议原文：
///
///<pre>Fired when HTTP response is available.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ResponseReceived;
impl ydp_core::Event for ResponseReceived {
    type Params = ResponseReceivedParams;
    const METHOD: &'static str = "Network.responseReceived";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.responseReceivedEarlyHints/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ResponseReceivedEarlyHintsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.responseReceivedEarlyHints/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Request identifier. Used to match this information to another responseReceived event.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.responseReceivedEarlyHints/parameters/headers。
    ///
    ///协议原文：
    ///
    ///<pre>Raw response headers as they were received over the wire.
    ///Duplicate headers in the response are represented as a single key with their values
    ///concatentated using `\n` as the separator.
    ///See also `headersText` that contains verbatim text for HTTP/1.*.</pre>
    ///
    ///
    #[serde(rename = "headers")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub headers: crate::network::types::Headers,
}
impl<'de> serde::Deserialize<'de> for ResponseReceivedEarlyHintsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResponseReceivedEarlyHintsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.responseReceivedEarlyHints/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request identifier. Used to match this information to another responseReceived event.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.responseReceivedEarlyHints/parameters/headers。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Raw response headers as they were received over the wire.
                    ///Duplicate headers in the response are represented as a single key with their values
                    ///concatentated using `\n` as the separator.
                    ///See also `headersText` that contains verbatim text for HTTP/1.*.</pre>
                    ///
                    ///
                    #[serde(rename = "headers")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub headers: crate::network::types::Headers,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ResponseReceivedEarlyHintsParams {
                    request_id: fields.request_id,
                    headers: fields.headers,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.responseReceivedEarlyHints。
///
///协议原文：
///
///<pre>Fired when 103 Early Hints headers is received in addition to the common response.
///Not every responseReceived event will have an responseReceivedEarlyHints fired.
///Only one responseReceivedEarlyHints may be fired for eached responseReceived event.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ResponseReceivedEarlyHints;
impl ydp_core::Event for ResponseReceivedEarlyHints {
    type Params = ResponseReceivedEarlyHintsParams;
    const METHOD: &'static str = "Network.responseReceivedEarlyHints";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.responseReceivedExtraInfo/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ResponseReceivedExtraInfoParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.responseReceivedExtraInfo/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Request identifier. Used to match this information to another responseReceived event.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.responseReceivedExtraInfo/parameters/blockedCookies。
    ///
    ///协议原文：
    ///
    ///<pre>A list of cookies which were not stored from the response along with the corresponding
    ///reasons for blocking. The cookies here may not be valid due to syntax errors, which
    ///are represented by the invalid cookie line string instead of a proper cookie.</pre>
    ///
    ///
    #[serde(rename = "blockedCookies")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub blocked_cookies: Vec<crate::network::types::BlockedSetCookieWithReason>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.responseReceivedExtraInfo/parameters/headers。
    ///
    ///协议原文：
    ///
    ///<pre>Raw response headers as they were received over the wire.
    ///Duplicate headers in the response are represented as a single key with their values
    ///concatentated using `\n` as the separator.
    ///See also `headersText` that contains verbatim text for HTTP/1.*.</pre>
    ///
    ///
    #[serde(rename = "headers")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub headers: crate::network::types::Headers,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.responseReceivedExtraInfo/parameters/resourceIPAddressSpace。
    ///
    ///协议原文：
    ///
    ///<pre>The IP address space of the resource. The address space can only be determined once the transport
    ///established the connection, so we can't send it in `requestWillBeSentExtraInfo`.</pre>
    ///
    ///
    #[serde(rename = "resourceIPAddressSpace")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub resource_ip_address_space: crate::network::types::IpAddressSpace,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.responseReceivedExtraInfo/parameters/statusCode。
    ///
    ///协议原文：
    ///
    ///<pre>The status code of the response. This is useful in cases the request failed and no responseReceived
    ///event is triggered, which is the case for, e.g., CORS errors. This is also the correct status code
    ///for cached requests, where the status in responseReceived is a 200 and this will be 304.</pre>
    ///
    ///
    #[serde(rename = "statusCode")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub status_code: i64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.responseReceivedExtraInfo/parameters/headersText。
    ///
    ///协议原文：
    ///
    ///<pre>Raw response header text as it was received over the wire. The raw text may not always be
    ///available, such as in the case of HTTP/2 or QUIC.</pre>
    ///
    ///
    #[serde(rename = "headersText")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub headers_text: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.responseReceivedExtraInfo/parameters/cookiePartitionKey。
    ///
    ///协议原文：
    ///
    ///<pre>The cookie partition key that will be used to store partitioned cookies set in this response.
    ///Only sent when partitioned cookies are enabled.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "cookiePartitionKey")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub cookie_partition_key: Option<crate::network::types::CookiePartitionKey>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.responseReceivedExtraInfo/parameters/cookiePartitionKeyOpaque。
    ///
    ///协议原文：
    ///
    ///<pre>True if partitioned cookies are enabled, but the partition key is not serializable to string.</pre>
    ///
    ///
    #[serde(rename = "cookiePartitionKeyOpaque")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub cookie_partition_key_opaque: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.responseReceivedExtraInfo/parameters/exemptedCookies。
    ///
    ///协议原文：
    ///
    ///<pre>A list of cookies which should have been blocked by 3PCD but are exempted and stored from
    ///the response with the corresponding reason.</pre>
    ///
    ///
    #[serde(rename = "exemptedCookies")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub exempted_cookies: Option<Vec<crate::network::types::ExemptedSetCookieWithReason>>,
}
impl<'de> serde::Deserialize<'de> for ResponseReceivedExtraInfoParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResponseReceivedExtraInfoParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.responseReceivedExtraInfo/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request identifier. Used to match this information to another responseReceived event.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.responseReceivedExtraInfo/parameters/blockedCookies。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A list of cookies which were not stored from the response along with the corresponding
                    ///reasons for blocking. The cookies here may not be valid due to syntax errors, which
                    ///are represented by the invalid cookie line string instead of a proper cookie.</pre>
                    ///
                    ///
                    #[serde(rename = "blockedCookies")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub blocked_cookies: Vec<crate::network::types::BlockedSetCookieWithReason>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.responseReceivedExtraInfo/parameters/headers。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Raw response headers as they were received over the wire.
                    ///Duplicate headers in the response are represented as a single key with their values
                    ///concatentated using `\n` as the separator.
                    ///See also `headersText` that contains verbatim text for HTTP/1.*.</pre>
                    ///
                    ///
                    #[serde(rename = "headers")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub headers: crate::network::types::Headers,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.responseReceivedExtraInfo/parameters/resourceIPAddressSpace。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The IP address space of the resource. The address space can only be determined once the transport
                    ///established the connection, so we can't send it in `requestWillBeSentExtraInfo`.</pre>
                    ///
                    ///
                    #[serde(rename = "resourceIPAddressSpace")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub resource_ip_address_space: crate::network::types::IpAddressSpace,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.responseReceivedExtraInfo/parameters/statusCode。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The status code of the response. This is useful in cases the request failed and no responseReceived
                    ///event is triggered, which is the case for, e.g., CORS errors. This is also the correct status code
                    ///for cached requests, where the status in responseReceived is a 200 and this will be 304.</pre>
                    ///
                    ///
                    #[serde(rename = "statusCode")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub status_code: i64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.responseReceivedExtraInfo/parameters/headersText。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Raw response header text as it was received over the wire. The raw text may not always be
                    ///available, such as in the case of HTTP/2 or QUIC.</pre>
                    ///
                    ///
                    #[serde(rename = "headersText")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub headers_text: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.responseReceivedExtraInfo/parameters/cookiePartitionKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The cookie partition key that will be used to store partitioned cookies set in this response.
                    ///Only sent when partitioned cookies are enabled.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "cookiePartitionKey")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub cookie_partition_key: Option<crate::network::types::CookiePartitionKey>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.responseReceivedExtraInfo/parameters/cookiePartitionKeyOpaque。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if partitioned cookies are enabled, but the partition key is not serializable to string.</pre>
                    ///
                    ///
                    #[serde(rename = "cookiePartitionKeyOpaque")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub cookie_partition_key_opaque: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.responseReceivedExtraInfo/parameters/exemptedCookies。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A list of cookies which should have been blocked by 3PCD but are exempted and stored from
                    ///the response with the corresponding reason.</pre>
                    ///
                    ///
                    #[serde(rename = "exemptedCookies")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub exempted_cookies:
                        Option<Vec<crate::network::types::ExemptedSetCookieWithReason>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ResponseReceivedExtraInfoParams {
                    request_id: fields.request_id,
                    blocked_cookies: fields.blocked_cookies,
                    headers: fields.headers,
                    resource_ip_address_space: fields.resource_ip_address_space,
                    status_code: fields.status_code,
                    headers_text: fields.headers_text,
                    cookie_partition_key: fields.cookie_partition_key,
                    cookie_partition_key_opaque: fields.cookie_partition_key_opaque,
                    exempted_cookies: fields.exempted_cookies,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.responseReceivedExtraInfo。
///
///协议原文：
///
///<pre>Fired when additional information about a responseReceived event is available from the network
///stack. Not every responseReceived event will have an additional responseReceivedExtraInfo for
///it, and responseReceivedExtraInfo may be fired before or after responseReceived.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ResponseReceivedExtraInfo;
impl ydp_core::Event for ResponseReceivedExtraInfo {
    type Params = ResponseReceivedExtraInfoParams;
    const METHOD: &'static str = "Network.responseReceivedExtraInfo";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.signedExchangeReceived/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SignedExchangeReceivedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.signedExchangeReceived/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Request identifier.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.signedExchangeReceived/parameters/info。
    ///
    ///协议原文：
    ///
    ///<pre>Information about the signed exchange response.</pre>
    ///
    ///
    #[serde(rename = "info")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub info: crate::network::types::SignedExchangeInfo,
}
impl<'de> serde::Deserialize<'de> for SignedExchangeReceivedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SignedExchangeReceivedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.signedExchangeReceived/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.signedExchangeReceived/parameters/info。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Information about the signed exchange response.</pre>
                    ///
                    ///
                    #[serde(rename = "info")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub info: crate::network::types::SignedExchangeInfo,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SignedExchangeReceivedParams {
                    request_id: fields.request_id,
                    info: fields.info,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.signedExchangeReceived。
///
///协议原文：
///
///<pre>Fired when a signed exchange was received over the network</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SignedExchangeReceived;
impl ydp_core::Event for SignedExchangeReceived {
    type Params = SignedExchangeReceivedParams;
    const METHOD: &'static str = "Network.signedExchangeReceived";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.event.trustTokenOperationDone/parameters/status。
///
///协议原文：
///
///<pre>Detailed success or error status of the operation.
///'AlreadyExists' also signifies a successful operation, as the result
///of the operation already exists und thus, the operation was abort
///preemptively (e.g. a cache hit).</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TrustTokenOperationDoneParamsStatus {
    ///协议值 "Ok"。
    Ok,
    ///协议值 "InvalidArgument"。
    InvalidArgument,
    ///协议值 "MissingIssuerKeys"。
    MissingIssuerKeys,
    ///协议值 "FailedPrecondition"。
    FailedPrecondition,
    ///协议值 "ResourceExhausted"。
    ResourceExhausted,
    ///协议值 "AlreadyExists"。
    AlreadyExists,
    ///协议值 "ResourceLimited"。
    ResourceLimited,
    ///协议值 "Unauthorized"。
    Unauthorized,
    ///协议值 "BadResponse"。
    BadResponse,
    ///协议值 "InternalError"。
    InternalError,
    ///协议值 "UnknownError"。
    UnknownError,
    ///协议值 "FulfilledLocally"。
    FulfilledLocally,
    ///协议值 "SiteIssuerLimit"。
    SiteIssuerLimit,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl TrustTokenOperationDoneParamsStatus {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::events::TrustTokenOperationDoneParamsStatus::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::InvalidArgument => "InvalidArgument",
            Self::MissingIssuerKeys => "MissingIssuerKeys",
            Self::FailedPrecondition => "FailedPrecondition",
            Self::ResourceExhausted => "ResourceExhausted",
            Self::AlreadyExists => "AlreadyExists",
            Self::ResourceLimited => "ResourceLimited",
            Self::Unauthorized => "Unauthorized",
            Self::BadResponse => "BadResponse",
            Self::InternalError => "InternalError",
            Self::UnknownError => "UnknownError",
            Self::FulfilledLocally => "FulfilledLocally",
            Self::SiteIssuerLimit => "SiteIssuerLimit",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for TrustTokenOperationDoneParamsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for TrustTokenOperationDoneParamsStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Ok" => TrustTokenOperationDoneParamsStatus::Ok,
            "InvalidArgument" => TrustTokenOperationDoneParamsStatus::InvalidArgument,
            "MissingIssuerKeys" => TrustTokenOperationDoneParamsStatus::MissingIssuerKeys,
            "FailedPrecondition" => TrustTokenOperationDoneParamsStatus::FailedPrecondition,
            "ResourceExhausted" => TrustTokenOperationDoneParamsStatus::ResourceExhausted,
            "AlreadyExists" => TrustTokenOperationDoneParamsStatus::AlreadyExists,
            "ResourceLimited" => TrustTokenOperationDoneParamsStatus::ResourceLimited,
            "Unauthorized" => TrustTokenOperationDoneParamsStatus::Unauthorized,
            "BadResponse" => TrustTokenOperationDoneParamsStatus::BadResponse,
            "InternalError" => TrustTokenOperationDoneParamsStatus::InternalError,
            "UnknownError" => TrustTokenOperationDoneParamsStatus::UnknownError,
            "FulfilledLocally" => TrustTokenOperationDoneParamsStatus::FulfilledLocally,
            "SiteIssuerLimit" => TrustTokenOperationDoneParamsStatus::SiteIssuerLimit,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for TrustTokenOperationDoneParamsStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TrustTokenOperationDoneParamsStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TrustTokenOperationDoneParamsStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Ok" => TrustTokenOperationDoneParamsStatus::Ok,
                    "InvalidArgument" => TrustTokenOperationDoneParamsStatus::InvalidArgument,
                    "MissingIssuerKeys" => TrustTokenOperationDoneParamsStatus::MissingIssuerKeys,
                    "FailedPrecondition" => TrustTokenOperationDoneParamsStatus::FailedPrecondition,
                    "ResourceExhausted" => TrustTokenOperationDoneParamsStatus::ResourceExhausted,
                    "AlreadyExists" => TrustTokenOperationDoneParamsStatus::AlreadyExists,
                    "ResourceLimited" => TrustTokenOperationDoneParamsStatus::ResourceLimited,
                    "Unauthorized" => TrustTokenOperationDoneParamsStatus::Unauthorized,
                    "BadResponse" => TrustTokenOperationDoneParamsStatus::BadResponse,
                    "InternalError" => TrustTokenOperationDoneParamsStatus::InternalError,
                    "UnknownError" => TrustTokenOperationDoneParamsStatus::UnknownError,
                    "FulfilledLocally" => TrustTokenOperationDoneParamsStatus::FulfilledLocally,
                    "SiteIssuerLimit" => TrustTokenOperationDoneParamsStatus::SiteIssuerLimit,
                    _ => TrustTokenOperationDoneParamsStatus::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(TrustTokenOperationDoneParamsStatus::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.trustTokenOperationDone/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TrustTokenOperationDoneParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.trustTokenOperationDone/parameters/status。
    ///
    ///协议原文：
    ///
    ///<pre>Detailed success or error status of the operation.
    ///'AlreadyExists' also signifies a successful operation, as the result
    ///of the operation already exists und thus, the operation was abort
    ///preemptively (e.g. a cache hit).</pre>
    ///
    ///
    #[serde(rename = "status")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub status: TrustTokenOperationDoneParamsStatus,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.trustTokenOperationDone/parameters/type。
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: crate::network::types::TrustTokenOperationType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.trustTokenOperationDone/parameters/requestId。
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::network::types::RequestId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.trustTokenOperationDone/parameters/topLevelOrigin。
    ///
    ///协议原文：
    ///
    ///<pre>Top level origin. The context in which the operation was attempted.</pre>
    ///
    ///
    #[serde(rename = "topLevelOrigin")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub top_level_origin: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.trustTokenOperationDone/parameters/issuerOrigin。
    ///
    ///协议原文：
    ///
    ///<pre>Origin of the issuer in case of a "Issuance" or "Redemption" operation.</pre>
    ///
    ///
    #[serde(rename = "issuerOrigin")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub issuer_origin: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.trustTokenOperationDone/parameters/issuedTokenCount。
    ///
    ///协议原文：
    ///
    ///<pre>The number of obtained Trust Tokens on a successful "Issuance" operation.</pre>
    ///
    ///
    #[serde(rename = "issuedTokenCount")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub issued_token_count: Option<i64>,
}
impl<'de> serde::Deserialize<'de> for TrustTokenOperationDoneParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TrustTokenOperationDoneParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.trustTokenOperationDone/parameters/status。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Detailed success or error status of the operation.
                    ///'AlreadyExists' also signifies a successful operation, as the result
                    ///of the operation already exists und thus, the operation was abort
                    ///preemptively (e.g. a cache hit).</pre>
                    ///
                    ///
                    #[serde(rename = "status")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub status: TrustTokenOperationDoneParamsStatus,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.trustTokenOperationDone/parameters/type。
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: crate::network::types::TrustTokenOperationType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.trustTokenOperationDone/parameters/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::network::types::RequestId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.trustTokenOperationDone/parameters/topLevelOrigin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Top level origin. The context in which the operation was attempted.</pre>
                    ///
                    ///
                    #[serde(rename = "topLevelOrigin")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub top_level_origin: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.trustTokenOperationDone/parameters/issuerOrigin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Origin of the issuer in case of a "Issuance" or "Redemption" operation.</pre>
                    ///
                    ///
                    #[serde(rename = "issuerOrigin")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub issuer_origin: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.trustTokenOperationDone/parameters/issuedTokenCount。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The number of obtained Trust Tokens on a successful "Issuance" operation.</pre>
                    ///
                    ///
                    #[serde(rename = "issuedTokenCount")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub issued_token_count: Option<i64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TrustTokenOperationDoneParams {
                    status: fields.status,
                    r#type: fields.r#type,
                    request_id: fields.request_id,
                    top_level_origin: fields.top_level_origin,
                    issuer_origin: fields.issuer_origin,
                    issued_token_count: fields.issued_token_count,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.trustTokenOperationDone。
///
///协议原文：
///
///<pre>Fired exactly once for each Trust Token operation. Depending on
///the type of the operation and whether the operation succeeded or
///failed, the event is fired before the corresponding request was sent
///or after the response was received.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct TrustTokenOperationDone;
impl ydp_core::Event for TrustTokenOperationDone {
    type Params = TrustTokenOperationDoneParams;
    const METHOD: &'static str = "Network.trustTokenOperationDone";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.webSocketClosed/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct WebSocketClosedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketClosed/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Request identifier.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketClosed/parameters/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Timestamp.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
}
impl<'de> serde::Deserialize<'de> for WebSocketClosedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WebSocketClosedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketClosed/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketClosed/parameters/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Timestamp.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(WebSocketClosedParams {
                    request_id: fields.request_id,
                    timestamp: fields.timestamp,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.webSocketClosed。
///
///协议原文：
///
///<pre>Fired when WebSocket is closed.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct WebSocketClosed;
impl ydp_core::Event for WebSocketClosed {
    type Params = WebSocketClosedParams;
    const METHOD: &'static str = "Network.webSocketClosed";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.webSocketCreated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct WebSocketCreatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketCreated/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Request identifier.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketCreated/parameters/url。
    ///
    ///协议原文：
    ///
    ///<pre>WebSocket request URL.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.webSocketCreated/parameters/initiator。
    ///
    ///协议原文：
    ///
    ///<pre>Request initiator.</pre>
    ///
    ///
    #[serde(rename = "initiator")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub initiator: Option<crate::network::types::Initiator>,
}
impl<'de> serde::Deserialize<'de> for WebSocketCreatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WebSocketCreatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketCreated/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketCreated/parameters/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>WebSocket request URL.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.webSocketCreated/parameters/initiator。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request initiator.</pre>
                    ///
                    ///
                    #[serde(rename = "initiator")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub initiator: Option<crate::network::types::Initiator>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(WebSocketCreatedParams {
                    request_id: fields.request_id,
                    url: fields.url,
                    initiator: fields.initiator,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.webSocketCreated。
///
///协议原文：
///
///<pre>Fired upon WebSocket creation.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct WebSocketCreated;
impl ydp_core::Event for WebSocketCreated {
    type Params = WebSocketCreatedParams;
    const METHOD: &'static str = "Network.webSocketCreated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.webSocketFrameError/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct WebSocketFrameErrorParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketFrameError/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Request identifier.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketFrameError/parameters/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Timestamp.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketFrameError/parameters/errorMessage。
    ///
    ///协议原文：
    ///
    ///<pre>WebSocket error message.</pre>
    ///
    ///
    #[serde(rename = "errorMessage")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub error_message: String,
}
impl<'de> serde::Deserialize<'de> for WebSocketFrameErrorParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WebSocketFrameErrorParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketFrameError/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketFrameError/parameters/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Timestamp.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketFrameError/parameters/errorMessage。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>WebSocket error message.</pre>
                    ///
                    ///
                    #[serde(rename = "errorMessage")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub error_message: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(WebSocketFrameErrorParams {
                    request_id: fields.request_id,
                    timestamp: fields.timestamp,
                    error_message: fields.error_message,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.webSocketFrameError。
///
///协议原文：
///
///<pre>Fired when WebSocket message error occurs.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct WebSocketFrameError;
impl ydp_core::Event for WebSocketFrameError {
    type Params = WebSocketFrameErrorParams;
    const METHOD: &'static str = "Network.webSocketFrameError";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.webSocketFrameReceived/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct WebSocketFrameReceivedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketFrameReceived/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Request identifier.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketFrameReceived/parameters/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Timestamp.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketFrameReceived/parameters/response。
    ///
    ///协议原文：
    ///
    ///<pre>WebSocket response data.</pre>
    ///
    ///
    #[serde(rename = "response")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub response: crate::network::types::WebSocketFrame,
}
impl<'de> serde::Deserialize<'de> for WebSocketFrameReceivedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WebSocketFrameReceivedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketFrameReceived/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketFrameReceived/parameters/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Timestamp.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketFrameReceived/parameters/response。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>WebSocket response data.</pre>
                    ///
                    ///
                    #[serde(rename = "response")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub response: crate::network::types::WebSocketFrame,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(WebSocketFrameReceivedParams {
                    request_id: fields.request_id,
                    timestamp: fields.timestamp,
                    response: fields.response,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.webSocketFrameReceived。
///
///协议原文：
///
///<pre>Fired when WebSocket message is received.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct WebSocketFrameReceived;
impl ydp_core::Event for WebSocketFrameReceived {
    type Params = WebSocketFrameReceivedParams;
    const METHOD: &'static str = "Network.webSocketFrameReceived";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.webSocketFrameSent/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct WebSocketFrameSentParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketFrameSent/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Request identifier.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketFrameSent/parameters/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Timestamp.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketFrameSent/parameters/response。
    ///
    ///协议原文：
    ///
    ///<pre>WebSocket response data.</pre>
    ///
    ///
    #[serde(rename = "response")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub response: crate::network::types::WebSocketFrame,
}
impl<'de> serde::Deserialize<'de> for WebSocketFrameSentParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WebSocketFrameSentParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketFrameSent/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketFrameSent/parameters/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Timestamp.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketFrameSent/parameters/response。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>WebSocket response data.</pre>
                    ///
                    ///
                    #[serde(rename = "response")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub response: crate::network::types::WebSocketFrame,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(WebSocketFrameSentParams {
                    request_id: fields.request_id,
                    timestamp: fields.timestamp,
                    response: fields.response,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.webSocketFrameSent。
///
///协议原文：
///
///<pre>Fired when WebSocket message is sent.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct WebSocketFrameSent;
impl ydp_core::Event for WebSocketFrameSent {
    type Params = WebSocketFrameSentParams;
    const METHOD: &'static str = "Network.webSocketFrameSent";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.webSocketHandshakeResponseReceived/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct WebSocketHandshakeResponseReceivedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketHandshakeResponseReceived/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Request identifier.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketHandshakeResponseReceived/parameters/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Timestamp.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketHandshakeResponseReceived/parameters/response。
    ///
    ///协议原文：
    ///
    ///<pre>WebSocket response data.</pre>
    ///
    ///
    #[serde(rename = "response")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub response: crate::network::types::WebSocketResponse,
}
impl<'de> serde::Deserialize<'de> for WebSocketHandshakeResponseReceivedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WebSocketHandshakeResponseReceivedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketHandshakeResponseReceived/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketHandshakeResponseReceived/parameters/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Timestamp.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketHandshakeResponseReceived/parameters/response。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>WebSocket response data.</pre>
                    ///
                    ///
                    #[serde(rename = "response")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub response: crate::network::types::WebSocketResponse,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(WebSocketHandshakeResponseReceivedParams {
                    request_id: fields.request_id,
                    timestamp: fields.timestamp,
                    response: fields.response,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.webSocketHandshakeResponseReceived。
///
///协议原文：
///
///<pre>Fired when WebSocket handshake response becomes available.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct WebSocketHandshakeResponseReceived;
impl ydp_core::Event for WebSocketHandshakeResponseReceived {
    type Params = WebSocketHandshakeResponseReceivedParams;
    const METHOD: &'static str = "Network.webSocketHandshakeResponseReceived";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.webSocketWillSendHandshakeRequest/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct WebSocketWillSendHandshakeRequestParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketWillSendHandshakeRequest/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Request identifier.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketWillSendHandshakeRequest/parameters/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Timestamp.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketWillSendHandshakeRequest/parameters/wallTime。
    ///
    ///协议原文：
    ///
    ///<pre>UTC Timestamp.</pre>
    ///
    ///
    #[serde(rename = "wallTime")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub wall_time: crate::network::types::TimeSinceEpoch,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketWillSendHandshakeRequest/parameters/request。
    ///
    ///协议原文：
    ///
    ///<pre>WebSocket request data.</pre>
    ///
    ///
    #[serde(rename = "request")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request: crate::network::types::WebSocketRequest,
}
impl<'de> serde::Deserialize<'de> for WebSocketWillSendHandshakeRequestParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WebSocketWillSendHandshakeRequestParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketWillSendHandshakeRequest/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketWillSendHandshakeRequest/parameters/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Timestamp.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketWillSendHandshakeRequest/parameters/wallTime。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>UTC Timestamp.</pre>
                    ///
                    ///
                    #[serde(rename = "wallTime")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub wall_time: crate::network::types::TimeSinceEpoch,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webSocketWillSendHandshakeRequest/parameters/request。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>WebSocket request data.</pre>
                    ///
                    ///
                    #[serde(rename = "request")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request: crate::network::types::WebSocketRequest,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(WebSocketWillSendHandshakeRequestParams {
                    request_id: fields.request_id,
                    timestamp: fields.timestamp,
                    wall_time: fields.wall_time,
                    request: fields.request,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.webSocketWillSendHandshakeRequest。
///
///协议原文：
///
///<pre>Fired when WebSocket is about to initiate handshake.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct WebSocketWillSendHandshakeRequest;
impl ydp_core::Event for WebSocketWillSendHandshakeRequest {
    type Params = WebSocketWillSendHandshakeRequestParams;
    const METHOD: &'static str = "Network.webSocketWillSendHandshakeRequest";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.webTransportClosed/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct WebTransportClosedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webTransportClosed/parameters/transportId。
    ///
    ///协议原文：
    ///
    ///<pre>WebTransport identifier.</pre>
    ///
    ///
    #[serde(rename = "transportId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub transport_id: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webTransportClosed/parameters/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Timestamp.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
}
impl<'de> serde::Deserialize<'de> for WebTransportClosedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WebTransportClosedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webTransportClosed/parameters/transportId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>WebTransport identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "transportId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub transport_id: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webTransportClosed/parameters/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Timestamp.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(WebTransportClosedParams {
                    transport_id: fields.transport_id,
                    timestamp: fields.timestamp,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.webTransportClosed。
///
///协议原文：
///
///<pre>Fired when WebTransport is disposed.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct WebTransportClosed;
impl ydp_core::Event for WebTransportClosed {
    type Params = WebTransportClosedParams;
    const METHOD: &'static str = "Network.webTransportClosed";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.webTransportConnectionEstablished/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct WebTransportConnectionEstablishedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webTransportConnectionEstablished/parameters/transportId。
    ///
    ///协议原文：
    ///
    ///<pre>WebTransport identifier.</pre>
    ///
    ///
    #[serde(rename = "transportId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub transport_id: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webTransportConnectionEstablished/parameters/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Timestamp.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
}
impl<'de> serde::Deserialize<'de> for WebTransportConnectionEstablishedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WebTransportConnectionEstablishedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webTransportConnectionEstablished/parameters/transportId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>WebTransport identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "transportId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub transport_id: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webTransportConnectionEstablished/parameters/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Timestamp.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(WebTransportConnectionEstablishedParams {
                    transport_id: fields.transport_id,
                    timestamp: fields.timestamp,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.webTransportConnectionEstablished。
///
///协议原文：
///
///<pre>Fired when WebTransport handshake is finished.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct WebTransportConnectionEstablished;
impl ydp_core::Event for WebTransportConnectionEstablished {
    type Params = WebTransportConnectionEstablishedParams;
    const METHOD: &'static str = "Network.webTransportConnectionEstablished";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.event.webTransportCreated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct WebTransportCreatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webTransportCreated/parameters/transportId。
    ///
    ///协议原文：
    ///
    ///<pre>WebTransport identifier.</pre>
    ///
    ///
    #[serde(rename = "transportId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub transport_id: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webTransportCreated/parameters/url。
    ///
    ///协议原文：
    ///
    ///<pre>WebTransport request URL.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webTransportCreated/parameters/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Timestamp.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.webTransportCreated/parameters/initiator。
    ///
    ///协议原文：
    ///
    ///<pre>Request initiator.</pre>
    ///
    ///
    #[serde(rename = "initiator")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub initiator: Option<crate::network::types::Initiator>,
}
impl<'de> serde::Deserialize<'de> for WebTransportCreatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WebTransportCreatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webTransportCreated/parameters/transportId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>WebTransport identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "transportId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub transport_id: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webTransportCreated/parameters/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>WebTransport request URL.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.event.webTransportCreated/parameters/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Timestamp.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.event.webTransportCreated/parameters/initiator。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request initiator.</pre>
                    ///
                    ///
                    #[serde(rename = "initiator")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub initiator: Option<crate::network::types::Initiator>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(WebTransportCreatedParams {
                    transport_id: fields.transport_id,
                    url: fields.url,
                    timestamp: fields.timestamp,
                    initiator: fields.initiator,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Network.event.webTransportCreated。
///
///协议原文：
///
///<pre>Fired upon WebTransport creation.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct WebTransportCreated;
impl ydp_core::Event for WebTransportCreated {
    type Params = WebTransportCreatedParams;
    const METHOD: &'static str = "Network.webTransportCreated";
}
