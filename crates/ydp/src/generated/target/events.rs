///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.event.attachedToTarget/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AttachedToTargetParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.event.attachedToTarget/parameters/sessionId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier assigned to the session used to send/receive messages.</pre>
    ///
    ///
    #[serde(rename = "sessionId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub session_id: crate::target::types::SessionId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.event.attachedToTarget/parameters/targetInfo。
    #[serde(rename = "targetInfo")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub target_info: crate::target::types::TargetInfo,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.event.attachedToTarget/parameters/waitingForDebugger。
    #[serde(rename = "waitingForDebugger")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub waiting_for_debugger: bool,
}
impl<'de> serde::Deserialize<'de> for AttachedToTargetParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AttachedToTargetParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.event.attachedToTarget/parameters/sessionId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier assigned to the session used to send/receive messages.</pre>
                    ///
                    ///
                    #[serde(rename = "sessionId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub session_id: crate::target::types::SessionId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.event.attachedToTarget/parameters/targetInfo。
                    #[serde(rename = "targetInfo")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub target_info: crate::target::types::TargetInfo,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.event.attachedToTarget/parameters/waitingForDebugger。
                    #[serde(rename = "waitingForDebugger")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub waiting_for_debugger: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AttachedToTargetParams {
                    session_id: fields.session_id,
                    target_info: fields.target_info,
                    waiting_for_debugger: fields.waiting_for_debugger,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Target.event.attachedToTarget。
///
///协议原文：
///
///<pre>Issued when attached to target because of auto-attach or `attachToTarget` command.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct AttachedToTarget;
impl ydp_core::Event for AttachedToTarget {
    type Params = AttachedToTargetParams;
    const METHOD: &'static str = "Target.attachedToTarget";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.event.detachedFromTarget/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DetachedFromTargetParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.event.detachedFromTarget/parameters/sessionId。
    ///
    ///协议原文：
    ///
    ///<pre>Detached session identifier.</pre>
    ///
    ///
    #[serde(rename = "sessionId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub session_id: crate::target::types::SessionId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.event.detachedFromTarget/parameters/targetId。
    ///
    ///协议原文：
    ///
    ///<pre>Deprecated.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "targetId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_id: Option<crate::target::types::TargetId>,
}
impl<'de> serde::Deserialize<'de> for DetachedFromTargetParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DetachedFromTargetParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.event.detachedFromTarget/parameters/sessionId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Detached session identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "sessionId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub session_id: crate::target::types::SessionId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.event.detachedFromTarget/parameters/targetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Deprecated.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "targetId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub target_id: Option<crate::target::types::TargetId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DetachedFromTargetParams {
                    session_id: fields.session_id,
                    target_id: fields.target_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Target.event.detachedFromTarget。
///
///协议原文：
///
///<pre>Issued when detached from target for any reason (including `detachFromTarget` command). Can be
///issued multiple times per target if multiple sessions have been attached to it.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct DetachedFromTarget;
impl ydp_core::Event for DetachedFromTarget {
    type Params = DetachedFromTargetParams;
    const METHOD: &'static str = "Target.detachedFromTarget";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.event.receivedMessageFromTarget/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReceivedMessageFromTargetParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.event.receivedMessageFromTarget/parameters/sessionId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of a session which sends a message.</pre>
    ///
    ///
    #[serde(rename = "sessionId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub session_id: crate::target::types::SessionId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.event.receivedMessageFromTarget/parameters/message。
    #[serde(rename = "message")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub message: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.event.receivedMessageFromTarget/parameters/targetId。
    ///
    ///协议原文：
    ///
    ///<pre>Deprecated.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "targetId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_id: Option<crate::target::types::TargetId>,
}
impl<'de> serde::Deserialize<'de> for ReceivedMessageFromTargetParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReceivedMessageFromTargetParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.event.receivedMessageFromTarget/parameters/sessionId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of a session which sends a message.</pre>
                    ///
                    ///
                    #[serde(rename = "sessionId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub session_id: crate::target::types::SessionId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.event.receivedMessageFromTarget/parameters/message。
                    #[serde(rename = "message")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub message: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.event.receivedMessageFromTarget/parameters/targetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Deprecated.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "targetId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub target_id: Option<crate::target::types::TargetId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReceivedMessageFromTargetParams {
                    session_id: fields.session_id,
                    message: fields.message,
                    target_id: fields.target_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Target.event.receivedMessageFromTarget。
///
///协议原文：
///
///<pre>Notifies about a new protocol message received from the session (as reported in
///`attachedToTarget` event).</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ReceivedMessageFromTarget;
impl ydp_core::Event for ReceivedMessageFromTarget {
    type Params = ReceivedMessageFromTargetParams;
    const METHOD: &'static str = "Target.receivedMessageFromTarget";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.event.targetCrashed/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TargetCrashedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.event.targetCrashed/parameters/targetId。
    #[serde(rename = "targetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub target_id: crate::target::types::TargetId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.event.targetCrashed/parameters/status。
    ///
    ///协议原文：
    ///
    ///<pre>Termination status type.</pre>
    ///
    ///
    #[serde(rename = "status")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub status: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.event.targetCrashed/parameters/errorCode。
    ///
    ///协议原文：
    ///
    ///<pre>Termination error code.</pre>
    ///
    ///
    #[serde(rename = "errorCode")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub error_code: i64,
}
impl<'de> serde::Deserialize<'de> for TargetCrashedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TargetCrashedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.event.targetCrashed/parameters/targetId。
                    #[serde(rename = "targetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub target_id: crate::target::types::TargetId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.event.targetCrashed/parameters/status。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Termination status type.</pre>
                    ///
                    ///
                    #[serde(rename = "status")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub status: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.event.targetCrashed/parameters/errorCode。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Termination error code.</pre>
                    ///
                    ///
                    #[serde(rename = "errorCode")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub error_code: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TargetCrashedParams {
                    target_id: fields.target_id,
                    status: fields.status,
                    error_code: fields.error_code,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Target.event.targetCrashed。
///
///协议原文：
///
///<pre>Issued when a target has crashed.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct TargetCrashed;
impl ydp_core::Event for TargetCrashed {
    type Params = TargetCrashedParams;
    const METHOD: &'static str = "Target.targetCrashed";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.event.targetCreated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TargetCreatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.event.targetCreated/parameters/targetInfo。
    #[serde(rename = "targetInfo")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub target_info: crate::target::types::TargetInfo,
}
impl<'de> serde::Deserialize<'de> for TargetCreatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TargetCreatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.event.targetCreated/parameters/targetInfo。
                    #[serde(rename = "targetInfo")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub target_info: crate::target::types::TargetInfo,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TargetCreatedParams {
                    target_info: fields.target_info,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Target.event.targetCreated。
///
///协议原文：
///
///<pre>Issued when a possible inspection target is created.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct TargetCreated;
impl ydp_core::Event for TargetCreated {
    type Params = TargetCreatedParams;
    const METHOD: &'static str = "Target.targetCreated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.event.targetDestroyed/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TargetDestroyedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.event.targetDestroyed/parameters/targetId。
    #[serde(rename = "targetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub target_id: crate::target::types::TargetId,
}
impl<'de> serde::Deserialize<'de> for TargetDestroyedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TargetDestroyedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.event.targetDestroyed/parameters/targetId。
                    #[serde(rename = "targetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub target_id: crate::target::types::TargetId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TargetDestroyedParams {
                    target_id: fields.target_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Target.event.targetDestroyed。
///
///协议原文：
///
///<pre>Issued when a target is destroyed.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct TargetDestroyed;
impl ydp_core::Event for TargetDestroyed {
    type Params = TargetDestroyedParams;
    const METHOD: &'static str = "Target.targetDestroyed";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.event.targetInfoChanged/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TargetInfoChangedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.event.targetInfoChanged/parameters/targetInfo。
    #[serde(rename = "targetInfo")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub target_info: crate::target::types::TargetInfo,
}
impl<'de> serde::Deserialize<'de> for TargetInfoChangedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TargetInfoChangedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.event.targetInfoChanged/parameters/targetInfo。
                    #[serde(rename = "targetInfo")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub target_info: crate::target::types::TargetInfo,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TargetInfoChangedParams {
                    target_info: fields.target_info,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Target.event.targetInfoChanged。
///
///协议原文：
///
///<pre>Issued when some information about a target has changed. This only happens between
///`targetCreated` and `targetDestroyed`.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct TargetInfoChanged;
impl ydp_core::Event for TargetInfoChanged {
    type Params = TargetInfoChangedParams;
    const METHOD: &'static str = "Target.targetInfoChanged";
}
