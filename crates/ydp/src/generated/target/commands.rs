///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.activateTarget/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ActivateTargetParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.activateTarget/parameters/targetId。
    #[serde(rename = "targetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub target_id: crate::target::types::TargetId,
}
impl<'de> serde::Deserialize<'de> for ActivateTargetParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ActivateTargetParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.activateTarget/parameters/targetId。
                    #[serde(rename = "targetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub target_id: crate::target::types::TargetId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ActivateTargetParams {
                    target_id: fields.target_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Target.command.activateTarget。
///
///协议原文：
///
///<pre>Activates (focuses) the target.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ActivateTarget;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.activateTarget/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ActivateTargetResponse {}
impl<'de> serde::Deserialize<'de> for ActivateTargetResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ActivateTargetResponse;
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
                Ok(ActivateTargetResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ActivateTarget {
    type Params = ActivateTargetParams;
    type Response = ActivateTargetResponse;
    const METHOD: &'static str = "Target.activateTarget";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.attachToBrowserTarget/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct AttachToBrowserTargetParams {}
impl<'de> serde::Deserialize<'de> for AttachToBrowserTargetParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AttachToBrowserTargetParams;
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
                Ok(AttachToBrowserTargetParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Target.command.attachToBrowserTarget。
///
///协议原文：
///
///<pre>Attaches to the browser target, only uses flat sessionId mode.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct AttachToBrowserTarget;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.attachToBrowserTarget/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AttachToBrowserTargetResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.attachToBrowserTarget/returns/sessionId。
    ///
    ///协议原文：
    ///
    ///<pre>Id assigned to the session.</pre>
    ///
    ///
    #[serde(rename = "sessionId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub session_id: crate::target::types::SessionId,
}
impl<'de> serde::Deserialize<'de> for AttachToBrowserTargetResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AttachToBrowserTargetResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.attachToBrowserTarget/returns/sessionId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id assigned to the session.</pre>
                    ///
                    ///
                    #[serde(rename = "sessionId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub session_id: crate::target::types::SessionId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AttachToBrowserTargetResponse {
                    session_id: fields.session_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for AttachToBrowserTarget {
    type Params = AttachToBrowserTargetParams;
    type Response = AttachToBrowserTargetResponse;
    const METHOD: &'static str = "Target.attachToBrowserTarget";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.attachToTarget/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AttachToTargetParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.attachToTarget/parameters/targetId。
    #[serde(rename = "targetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub target_id: crate::target::types::TargetId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.attachToTarget/parameters/flatten。
    ///
    ///协议原文：
    ///
    ///<pre>Enables "flat" access to the session via specifying sessionId attribute in the commands.
    ///We plan to make this the default, deprecate non-flattened mode,
    ///and eventually retire it. See crbug.com/991325.</pre>
    ///
    ///
    #[serde(rename = "flatten")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub flatten: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for AttachToTargetParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AttachToTargetParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.attachToTarget/parameters/targetId。
                    #[serde(rename = "targetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub target_id: crate::target::types::TargetId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.attachToTarget/parameters/flatten。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Enables "flat" access to the session via specifying sessionId attribute in the commands.
                    ///We plan to make this the default, deprecate non-flattened mode,
                    ///and eventually retire it. See crbug.com/991325.</pre>
                    ///
                    ///
                    #[serde(rename = "flatten")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub flatten: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AttachToTargetParams {
                    target_id: fields.target_id,
                    flatten: fields.flatten,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Target.command.attachToTarget。
///
///协议原文：
///
///<pre>Attaches to the target with given id.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct AttachToTarget;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.attachToTarget/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AttachToTargetResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.attachToTarget/returns/sessionId。
    ///
    ///协议原文：
    ///
    ///<pre>Id assigned to the session.</pre>
    ///
    ///
    #[serde(rename = "sessionId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub session_id: crate::target::types::SessionId,
}
impl<'de> serde::Deserialize<'de> for AttachToTargetResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AttachToTargetResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.attachToTarget/returns/sessionId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id assigned to the session.</pre>
                    ///
                    ///
                    #[serde(rename = "sessionId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub session_id: crate::target::types::SessionId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AttachToTargetResponse {
                    session_id: fields.session_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for AttachToTarget {
    type Params = AttachToTargetParams;
    type Response = AttachToTargetResponse;
    const METHOD: &'static str = "Target.attachToTarget";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.autoAttachRelated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AutoAttachRelatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.autoAttachRelated/parameters/targetId。
    #[serde(rename = "targetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub target_id: crate::target::types::TargetId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.autoAttachRelated/parameters/waitForDebuggerOnStart。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to pause new targets when attaching to them. Use `Runtime.runIfWaitingForDebugger`
    ///to run paused targets.</pre>
    ///
    ///
    #[serde(rename = "waitForDebuggerOnStart")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub wait_for_debugger_on_start: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.autoAttachRelated/parameters/filter。
    ///
    ///协议原文：
    ///
    ///<pre>Only targets matching filter will be attached.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "filter")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub filter: Option<crate::target::types::TargetFilter>,
}
impl<'de> serde::Deserialize<'de> for AutoAttachRelatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AutoAttachRelatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.autoAttachRelated/parameters/targetId。
                    #[serde(rename = "targetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub target_id: crate::target::types::TargetId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.autoAttachRelated/parameters/waitForDebuggerOnStart。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to pause new targets when attaching to them. Use `Runtime.runIfWaitingForDebugger`
                    ///to run paused targets.</pre>
                    ///
                    ///
                    #[serde(rename = "waitForDebuggerOnStart")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub wait_for_debugger_on_start: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.autoAttachRelated/parameters/filter。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Only targets matching filter will be attached.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "filter")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub filter: Option<crate::target::types::TargetFilter>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AutoAttachRelatedParams {
                    target_id: fields.target_id,
                    wait_for_debugger_on_start: fields.wait_for_debugger_on_start,
                    filter: fields.filter,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Target.command.autoAttachRelated。
///
///协议原文：
///
///<pre>Adds the specified target to the list of targets that will be monitored for any related target
///creation (such as child frames, child workers and new versions of service worker) and reported
///through `attachedToTarget`. The specified target is also auto-attached.
///This cancels the effect of any previous `setAutoAttach` and is also cancelled by subsequent
///`setAutoAttach`. Only available at the Browser target.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct AutoAttachRelated;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.autoAttachRelated/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AutoAttachRelatedResponse {}
impl<'de> serde::Deserialize<'de> for AutoAttachRelatedResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AutoAttachRelatedResponse;
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
                Ok(AutoAttachRelatedResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for AutoAttachRelated {
    type Params = AutoAttachRelatedParams;
    type Response = AutoAttachRelatedResponse;
    const METHOD: &'static str = "Target.autoAttachRelated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.closeTarget/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CloseTargetParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.closeTarget/parameters/targetId。
    #[serde(rename = "targetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub target_id: crate::target::types::TargetId,
}
impl<'de> serde::Deserialize<'de> for CloseTargetParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CloseTargetParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.closeTarget/parameters/targetId。
                    #[serde(rename = "targetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub target_id: crate::target::types::TargetId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CloseTargetParams {
                    target_id: fields.target_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Target.command.closeTarget。
///
///协议原文：
///
///<pre>Closes the target. If the target is a page that gets closed too.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct CloseTarget;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.closeTarget/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CloseTargetResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.closeTarget/returns/success。
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
impl<'de> serde::Deserialize<'de> for CloseTargetResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CloseTargetResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.closeTarget/returns/success。
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
                Ok(CloseTargetResponse {
                    success: fields.success,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for CloseTarget {
    type Params = CloseTargetParams;
    type Response = CloseTargetResponse;
    const METHOD: &'static str = "Target.closeTarget";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.createBrowserContext/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CreateBrowserContextParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createBrowserContext/parameters/disposeOnDetach。
    ///
    ///协议原文：
    ///
    ///<pre>If specified, disposes this context when debugging session disconnects.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "disposeOnDetach")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub dispose_on_detach: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createBrowserContext/parameters/proxyServer。
    ///
    ///协议原文：
    ///
    ///<pre>Proxy server, similar to the one passed to --proxy-server</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "proxyServer")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub proxy_server: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createBrowserContext/parameters/proxyBypassList。
    ///
    ///协议原文：
    ///
    ///<pre>Proxy bypass list, similar to the one passed to --proxy-bypass-list</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "proxyBypassList")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub proxy_bypass_list: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createBrowserContext/parameters/originsWithUniversalNetworkAccess。
    ///
    ///协议原文：
    ///
    ///<pre>An optional list of origins to grant unlimited cross-origin access to.
    ///Parts of the URL other than those constituting origin are ignored.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "originsWithUniversalNetworkAccess")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub origins_with_universal_network_access: Option<Vec<String>>,
}
impl<'de> serde::Deserialize<'de> for CreateBrowserContextParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CreateBrowserContextParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createBrowserContext/parameters/disposeOnDetach。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If specified, disposes this context when debugging session disconnects.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "disposeOnDetach")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub dispose_on_detach: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createBrowserContext/parameters/proxyServer。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Proxy server, similar to the one passed to --proxy-server</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "proxyServer")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub proxy_server: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createBrowserContext/parameters/proxyBypassList。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Proxy bypass list, similar to the one passed to --proxy-bypass-list</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "proxyBypassList")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub proxy_bypass_list: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createBrowserContext/parameters/originsWithUniversalNetworkAccess。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An optional list of origins to grant unlimited cross-origin access to.
                    ///Parts of the URL other than those constituting origin are ignored.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "originsWithUniversalNetworkAccess")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub origins_with_universal_network_access: Option<Vec<String>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CreateBrowserContextParams {
                    dispose_on_detach: fields.dispose_on_detach,
                    proxy_server: fields.proxy_server,
                    proxy_bypass_list: fields.proxy_bypass_list,
                    origins_with_universal_network_access: fields
                        .origins_with_universal_network_access,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Target.command.createBrowserContext。
///
///协议原文：
///
///<pre>Creates a new empty BrowserContext. Similar to an incognito profile but you can have more than
///one.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct CreateBrowserContext;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.createBrowserContext/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CreateBrowserContextResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.createBrowserContext/returns/browserContextId。
    ///
    ///协议原文：
    ///
    ///<pre>The id of the context created.</pre>
    ///
    ///
    #[serde(rename = "browserContextId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub browser_context_id: crate::browser::types::BrowserContextId,
}
impl<'de> serde::Deserialize<'de> for CreateBrowserContextResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CreateBrowserContextResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.createBrowserContext/returns/browserContextId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The id of the context created.</pre>
                    ///
                    ///
                    #[serde(rename = "browserContextId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub browser_context_id: crate::browser::types::BrowserContextId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CreateBrowserContextResponse {
                    browser_context_id: fields.browser_context_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for CreateBrowserContext {
    type Params = CreateBrowserContextParams;
    type Response = CreateBrowserContextResponse;
    const METHOD: &'static str = "Target.createBrowserContext";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.createTarget/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CreateTargetParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.createTarget/parameters/url。
    ///
    ///协议原文：
    ///
    ///<pre>The initial URL the page will be navigated to. An empty string indicates about:blank.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createTarget/parameters/left。
    ///
    ///协议原文：
    ///
    ///<pre>Frame left origin in DIP (requires newWindow to be true or headless shell).</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "left")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub left: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createTarget/parameters/top。
    ///
    ///协议原文：
    ///
    ///<pre>Frame top origin in DIP (requires newWindow to be true or headless shell).</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "top")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub top: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createTarget/parameters/width。
    ///
    ///协议原文：
    ///
    ///<pre>Frame width in DIP (requires newWindow to be true or headless shell).</pre>
    ///
    ///
    #[serde(rename = "width")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub width: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createTarget/parameters/height。
    ///
    ///协议原文：
    ///
    ///<pre>Frame height in DIP (requires newWindow to be true or headless shell).</pre>
    ///
    ///
    #[serde(rename = "height")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub height: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createTarget/parameters/windowState。
    ///
    ///协议原文：
    ///
    ///<pre>Frame window state (requires newWindow to be true or headless shell).
    ///Default is normal.</pre>
    ///
    ///
    #[serde(rename = "windowState")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub window_state: Option<crate::target::types::WindowState>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createTarget/parameters/browserContextId。
    ///
    ///协议原文：
    ///
    ///<pre>The browser context to create the page in.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "browserContextId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub browser_context_id: Option<crate::browser::types::BrowserContextId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createTarget/parameters/enableBeginFrameControl。
    ///
    ///协议原文：
    ///
    ///<pre>Whether BeginFrames for this target will be controlled via DevTools (headless shell only,
    ///not supported on MacOS yet, false by default).</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "enableBeginFrameControl")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_begin_frame_control: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createTarget/parameters/newWindow。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to create a new Window or Tab (false by default, not supported by headless shell).</pre>
    ///
    ///
    #[serde(rename = "newWindow")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub new_window: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createTarget/parameters/background。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to create the target in background or foreground (false by default, not supported
    ///by headless shell).</pre>
    ///
    ///
    #[serde(rename = "background")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub background: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createTarget/parameters/forTab。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to create the target of type "tab".</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "forTab")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub for_tab: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createTarget/parameters/hidden。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to create a hidden target. The hidden target is observable via protocol, but not
    ///present in the tab UI strip. Cannot be created with `forTab: true`, `newWindow: true` or
    ///`background: false`. The life-time of the tab is limited to the life-time of the session.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "hidden")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub hidden: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createTarget/parameters/focus。
    ///
    ///协议原文：
    ///
    ///<pre>If specified, determines whether the new target should be focused.
    ///By default, the focus behavior depends on the `background` parameter:
    ///- If `background` is false (default) and `focus` is omitted, the new target is focused and the browser window is brought to the foreground.
    ///- If `background` is false and `focus` is false, the target is opened but the browser window's focus remains unchanged (e.g., if the window was in the background, it stays there).
    ///- If `background` is true, setting `focus` to true is not supported and will result in an error.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "focus")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub focus: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for CreateTargetParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CreateTargetParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.createTarget/parameters/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The initial URL the page will be navigated to. An empty string indicates about:blank.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createTarget/parameters/left。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame left origin in DIP (requires newWindow to be true or headless shell).</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "left")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub left: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createTarget/parameters/top。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame top origin in DIP (requires newWindow to be true or headless shell).</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "top")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub top: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createTarget/parameters/width。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame width in DIP (requires newWindow to be true or headless shell).</pre>
                    ///
                    ///
                    #[serde(rename = "width")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub width: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createTarget/parameters/height。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame height in DIP (requires newWindow to be true or headless shell).</pre>
                    ///
                    ///
                    #[serde(rename = "height")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub height: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createTarget/parameters/windowState。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame window state (requires newWindow to be true or headless shell).
                    ///Default is normal.</pre>
                    ///
                    ///
                    #[serde(rename = "windowState")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub window_state: Option<crate::target::types::WindowState>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createTarget/parameters/browserContextId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The browser context to create the page in.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "browserContextId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub browser_context_id: Option<crate::browser::types::BrowserContextId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createTarget/parameters/enableBeginFrameControl。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether BeginFrames for this target will be controlled via DevTools (headless shell only,
                    ///not supported on MacOS yet, false by default).</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "enableBeginFrameControl")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub enable_begin_frame_control: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createTarget/parameters/newWindow。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to create a new Window or Tab (false by default, not supported by headless shell).</pre>
                    ///
                    ///
                    #[serde(rename = "newWindow")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub new_window: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createTarget/parameters/background。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to create the target in background or foreground (false by default, not supported
                    ///by headless shell).</pre>
                    ///
                    ///
                    #[serde(rename = "background")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub background: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createTarget/parameters/forTab。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to create the target of type "tab".</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "forTab")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub for_tab: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createTarget/parameters/hidden。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to create a hidden target. The hidden target is observable via protocol, but not
                    ///present in the tab UI strip. Cannot be created with `forTab: true`, `newWindow: true` or
                    ///`background: false`. The life-time of the tab is limited to the life-time of the session.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "hidden")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub hidden: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.createTarget/parameters/focus。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If specified, determines whether the new target should be focused.
                    ///By default, the focus behavior depends on the `background` parameter:
                    ///- If `background` is false (default) and `focus` is omitted, the new target is focused and the browser window is brought to the foreground.
                    ///- If `background` is false and `focus` is false, the target is opened but the browser window's focus remains unchanged (e.g., if the window was in the background, it stays there).
                    ///- If `background` is true, setting `focus` to true is not supported and will result in an error.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "focus")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub focus: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CreateTargetParams {
                    url: fields.url,
                    left: fields.left,
                    top: fields.top,
                    width: fields.width,
                    height: fields.height,
                    window_state: fields.window_state,
                    browser_context_id: fields.browser_context_id,
                    enable_begin_frame_control: fields.enable_begin_frame_control,
                    new_window: fields.new_window,
                    background: fields.background,
                    for_tab: fields.for_tab,
                    hidden: fields.hidden,
                    focus: fields.focus,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Target.command.createTarget。
///
///协议原文：
///
///<pre>Creates a new page.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct CreateTarget;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.createTarget/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CreateTargetResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.createTarget/returns/targetId。
    ///
    ///协议原文：
    ///
    ///<pre>The id of the page opened.</pre>
    ///
    ///
    #[serde(rename = "targetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub target_id: crate::target::types::TargetId,
}
impl<'de> serde::Deserialize<'de> for CreateTargetResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CreateTargetResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.createTarget/returns/targetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The id of the page opened.</pre>
                    ///
                    ///
                    #[serde(rename = "targetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub target_id: crate::target::types::TargetId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CreateTargetResponse {
                    target_id: fields.target_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for CreateTarget {
    type Params = CreateTargetParams;
    type Response = CreateTargetResponse;
    const METHOD: &'static str = "Target.createTarget";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.detachFromTarget/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct DetachFromTargetParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.detachFromTarget/parameters/sessionId。
    ///
    ///协议原文：
    ///
    ///<pre>Session to detach.</pre>
    ///
    ///
    #[serde(rename = "sessionId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub session_id: Option<crate::target::types::SessionId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.detachFromTarget/parameters/targetId。
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
impl<'de> serde::Deserialize<'de> for DetachFromTargetParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DetachFromTargetParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.detachFromTarget/parameters/sessionId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Session to detach.</pre>
                    ///
                    ///
                    #[serde(rename = "sessionId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub session_id: Option<crate::target::types::SessionId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.detachFromTarget/parameters/targetId。
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
                Ok(DetachFromTargetParams {
                    session_id: fields.session_id,
                    target_id: fields.target_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Target.command.detachFromTarget。
///
///协议原文：
///
///<pre>Detaches session with given id.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct DetachFromTarget;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.detachFromTarget/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DetachFromTargetResponse {}
impl<'de> serde::Deserialize<'de> for DetachFromTargetResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DetachFromTargetResponse;
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
                Ok(DetachFromTargetResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for DetachFromTarget {
    type Params = DetachFromTargetParams;
    type Response = DetachFromTargetResponse;
    const METHOD: &'static str = "Target.detachFromTarget";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.disposeBrowserContext/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DisposeBrowserContextParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.disposeBrowserContext/parameters/browserContextId。
    #[serde(rename = "browserContextId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub browser_context_id: crate::browser::types::BrowserContextId,
}
impl<'de> serde::Deserialize<'de> for DisposeBrowserContextParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DisposeBrowserContextParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.disposeBrowserContext/parameters/browserContextId。
                    #[serde(rename = "browserContextId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub browser_context_id: crate::browser::types::BrowserContextId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DisposeBrowserContextParams {
                    browser_context_id: fields.browser_context_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Target.command.disposeBrowserContext。
///
///协议原文：
///
///<pre>Deletes a BrowserContext. All the belonging pages will be closed without calling their
///beforeunload hooks.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct DisposeBrowserContext;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.disposeBrowserContext/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DisposeBrowserContextResponse {}
impl<'de> serde::Deserialize<'de> for DisposeBrowserContextResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DisposeBrowserContextResponse;
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
                Ok(DisposeBrowserContextResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for DisposeBrowserContext {
    type Params = DisposeBrowserContextParams;
    type Response = DisposeBrowserContextResponse;
    const METHOD: &'static str = "Target.disposeBrowserContext";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.exposeDevToolsProtocol/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ExposeDevToolsProtocolParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.exposeDevToolsProtocol/parameters/targetId。
    #[serde(rename = "targetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub target_id: crate::target::types::TargetId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.exposeDevToolsProtocol/parameters/bindingName。
    ///
    ///协议原文：
    ///
    ///<pre>Binding name, 'cdp' if not specified.</pre>
    ///
    ///
    #[serde(rename = "bindingName")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub binding_name: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.exposeDevToolsProtocol/parameters/inheritPermissions。
    ///
    ///协议原文：
    ///
    ///<pre>If true, inherits the current root session's permissions (default: false).</pre>
    ///
    ///
    #[serde(rename = "inheritPermissions")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub inherit_permissions: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for ExposeDevToolsProtocolParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExposeDevToolsProtocolParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.exposeDevToolsProtocol/parameters/targetId。
                    #[serde(rename = "targetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub target_id: crate::target::types::TargetId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.exposeDevToolsProtocol/parameters/bindingName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Binding name, 'cdp' if not specified.</pre>
                    ///
                    ///
                    #[serde(rename = "bindingName")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub binding_name: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.exposeDevToolsProtocol/parameters/inheritPermissions。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, inherits the current root session's permissions (default: false).</pre>
                    ///
                    ///
                    #[serde(rename = "inheritPermissions")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub inherit_permissions: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ExposeDevToolsProtocolParams {
                    target_id: fields.target_id,
                    binding_name: fields.binding_name,
                    inherit_permissions: fields.inherit_permissions,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Target.command.exposeDevToolsProtocol。
///
///协议原文：
///
///<pre>Inject object to the target's main frame that provides a communication
///channel with browser target.
///
///Injected object will be available as `window[bindingName]`.
///
///The object has the following API:
///- `binding.send(json)` - a method to send messages over the remote debugging protocol
///- `binding.onmessage = json =&gt; handleMessage(json)` - a callback that will be called for the protocol notifications and command responses.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ExposeDevToolsProtocol;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.exposeDevToolsProtocol/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ExposeDevToolsProtocolResponse {}
impl<'de> serde::Deserialize<'de> for ExposeDevToolsProtocolResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExposeDevToolsProtocolResponse;
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
                Ok(ExposeDevToolsProtocolResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ExposeDevToolsProtocol {
    type Params = ExposeDevToolsProtocolParams;
    type Response = ExposeDevToolsProtocolResponse;
    const METHOD: &'static str = "Target.exposeDevToolsProtocol";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.getBrowserContexts/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetBrowserContextsParams {}
impl<'de> serde::Deserialize<'de> for GetBrowserContextsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetBrowserContextsParams;
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
                Ok(GetBrowserContextsParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Target.command.getBrowserContexts。
///
///协议原文：
///
///<pre>Returns all browser contexts created with `Target.createBrowserContext` method.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetBrowserContexts;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.getBrowserContexts/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetBrowserContextsResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.getBrowserContexts/returns/browserContextIds。
    ///
    ///协议原文：
    ///
    ///<pre>An array of browser context ids.</pre>
    ///
    ///
    #[serde(rename = "browserContextIds")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub browser_context_ids: Vec<crate::browser::types::BrowserContextId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.getBrowserContexts/returns/defaultBrowserContextId。
    ///
    ///协议原文：
    ///
    ///<pre>The id of the default browser context if available.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "defaultBrowserContextId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_browser_context_id: Option<crate::browser::types::BrowserContextId>,
}
impl<'de> serde::Deserialize<'de> for GetBrowserContextsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetBrowserContextsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.getBrowserContexts/returns/browserContextIds。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An array of browser context ids.</pre>
                    ///
                    ///
                    #[serde(rename = "browserContextIds")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub browser_context_ids: Vec<crate::browser::types::BrowserContextId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.getBrowserContexts/returns/defaultBrowserContextId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The id of the default browser context if available.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "defaultBrowserContextId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub default_browser_context_id: Option<crate::browser::types::BrowserContextId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetBrowserContextsResponse {
                    browser_context_ids: fields.browser_context_ids,
                    default_browser_context_id: fields.default_browser_context_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetBrowserContexts {
    type Params = GetBrowserContextsParams;
    type Response = GetBrowserContextsResponse;
    const METHOD: &'static str = "Target.getBrowserContexts";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.getDevToolsTarget/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetDevToolsTargetParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.getDevToolsTarget/parameters/targetId。
    ///
    ///协议原文：
    ///
    ///<pre>Page or tab target ID.</pre>
    ///
    ///
    #[serde(rename = "targetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub target_id: crate::target::types::TargetId,
}
impl<'de> serde::Deserialize<'de> for GetDevToolsTargetParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetDevToolsTargetParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.getDevToolsTarget/parameters/targetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Page or tab target ID.</pre>
                    ///
                    ///
                    #[serde(rename = "targetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub target_id: crate::target::types::TargetId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetDevToolsTargetParams {
                    target_id: fields.target_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Target.command.getDevToolsTarget。
///
///协议原文：
///
///<pre>Gets the targetId of the DevTools page target opened for the given target
///(if any).</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetDevToolsTarget;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.getDevToolsTarget/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetDevToolsTargetResponse {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.getDevToolsTarget/returns/targetId。
    ///
    ///协议原文：
    ///
    ///<pre>The targetId of DevTools page target if exists.</pre>
    ///
    ///
    #[serde(rename = "targetId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_id: Option<crate::target::types::TargetId>,
}
impl<'de> serde::Deserialize<'de> for GetDevToolsTargetResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetDevToolsTargetResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.getDevToolsTarget/returns/targetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The targetId of DevTools page target if exists.</pre>
                    ///
                    ///
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
                Ok(GetDevToolsTargetResponse {
                    target_id: fields.target_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetDevToolsTarget {
    type Params = GetDevToolsTargetParams;
    type Response = GetDevToolsTargetResponse;
    const METHOD: &'static str = "Target.getDevToolsTarget";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.getTargetInfo/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetTargetInfoParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.getTargetInfo/parameters/targetId。
    #[serde(rename = "targetId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_id: Option<crate::target::types::TargetId>,
}
impl<'de> serde::Deserialize<'de> for GetTargetInfoParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetTargetInfoParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.getTargetInfo/parameters/targetId。
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
                Ok(GetTargetInfoParams {
                    target_id: fields.target_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Target.command.getTargetInfo。
///
///协议原文：
///
///<pre>Returns information about a target.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetTargetInfo;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.getTargetInfo/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetTargetInfoResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.getTargetInfo/returns/targetInfo。
    #[serde(rename = "targetInfo")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub target_info: crate::target::types::TargetInfo,
}
impl<'de> serde::Deserialize<'de> for GetTargetInfoResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetTargetInfoResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.getTargetInfo/returns/targetInfo。
                    #[serde(rename = "targetInfo")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub target_info: crate::target::types::TargetInfo,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetTargetInfoResponse {
                    target_info: fields.target_info,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetTargetInfo {
    type Params = GetTargetInfoParams;
    type Response = GetTargetInfoResponse;
    const METHOD: &'static str = "Target.getTargetInfo";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.getTargets/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetTargetsParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.getTargets/parameters/filter。
    ///
    ///协议原文：
    ///
    ///<pre>Only targets matching filter will be reported. If filter is not specified
    ///and target discovery is currently enabled, a filter used for target discovery
    ///is used for consistency.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "filter")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub filter: Option<crate::target::types::TargetFilter>,
}
impl<'de> serde::Deserialize<'de> for GetTargetsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetTargetsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.getTargets/parameters/filter。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Only targets matching filter will be reported. If filter is not specified
                    ///and target discovery is currently enabled, a filter used for target discovery
                    ///is used for consistency.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "filter")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub filter: Option<crate::target::types::TargetFilter>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetTargetsParams {
                    filter: fields.filter,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Target.command.getTargets。
///
///协议原文：
///
///<pre>Retrieves a list of available targets.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetTargets;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.getTargets/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetTargetsResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.getTargets/returns/targetInfos。
    ///
    ///协议原文：
    ///
    ///<pre>The list of targets.</pre>
    ///
    ///
    #[serde(rename = "targetInfos")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub target_infos: Vec<crate::target::types::TargetInfo>,
}
impl<'de> serde::Deserialize<'de> for GetTargetsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetTargetsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.getTargets/returns/targetInfos。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The list of targets.</pre>
                    ///
                    ///
                    #[serde(rename = "targetInfos")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub target_infos: Vec<crate::target::types::TargetInfo>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetTargetsResponse {
                    target_infos: fields.target_infos,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetTargets {
    type Params = GetTargetsParams;
    type Response = GetTargetsResponse;
    const METHOD: &'static str = "Target.getTargets";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.openDevTools/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct OpenDevToolsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.openDevTools/parameters/targetId。
    ///
    ///协议原文：
    ///
    ///<pre>This can be the page or tab target ID.</pre>
    ///
    ///
    #[serde(rename = "targetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub target_id: crate::target::types::TargetId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.openDevTools/parameters/panelId。
    ///
    ///协议原文：
    ///
    ///<pre>The id of the panel we want DevTools to open initially. Currently
    ///supported panels are elements, console, network, sources, resources,
    ///timeline, chrome-recorder, heap-profiler, lighthouse, and security.</pre>
    ///
    ///
    #[serde(rename = "panelId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub panel_id: Option<String>,
}
impl<'de> serde::Deserialize<'de> for OpenDevToolsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = OpenDevToolsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.openDevTools/parameters/targetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>This can be the page or tab target ID.</pre>
                    ///
                    ///
                    #[serde(rename = "targetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub target_id: crate::target::types::TargetId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.openDevTools/parameters/panelId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The id of the panel we want DevTools to open initially. Currently
                    ///supported panels are elements, console, network, sources, resources,
                    ///timeline, chrome-recorder, heap-profiler, lighthouse, and security.</pre>
                    ///
                    ///
                    #[serde(rename = "panelId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub panel_id: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(OpenDevToolsParams {
                    target_id: fields.target_id,
                    panel_id: fields.panel_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Target.command.openDevTools。
///
///协议原文：
///
///<pre>Opens a DevTools window for the target.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct OpenDevTools;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.openDevTools/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct OpenDevToolsResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.openDevTools/returns/targetId。
    ///
    ///协议原文：
    ///
    ///<pre>The targetId of DevTools page target.</pre>
    ///
    ///
    #[serde(rename = "targetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub target_id: crate::target::types::TargetId,
}
impl<'de> serde::Deserialize<'de> for OpenDevToolsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = OpenDevToolsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.openDevTools/returns/targetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The targetId of DevTools page target.</pre>
                    ///
                    ///
                    #[serde(rename = "targetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub target_id: crate::target::types::TargetId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(OpenDevToolsResponse {
                    target_id: fields.target_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for OpenDevTools {
    type Params = OpenDevToolsParams;
    type Response = OpenDevToolsResponse;
    const METHOD: &'static str = "Target.openDevTools";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.sendMessageToTarget/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SendMessageToTargetParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.sendMessageToTarget/parameters/message。
    #[serde(rename = "message")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub message: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.sendMessageToTarget/parameters/sessionId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the session.</pre>
    ///
    ///
    #[serde(rename = "sessionId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub session_id: Option<crate::target::types::SessionId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.sendMessageToTarget/parameters/targetId。
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
impl<'de> serde::Deserialize<'de> for SendMessageToTargetParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SendMessageToTargetParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.sendMessageToTarget/parameters/message。
                    #[serde(rename = "message")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub message: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.sendMessageToTarget/parameters/sessionId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the session.</pre>
                    ///
                    ///
                    #[serde(rename = "sessionId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub session_id: Option<crate::target::types::SessionId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.sendMessageToTarget/parameters/targetId。
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
                Ok(SendMessageToTargetParams {
                    message: fields.message,
                    session_id: fields.session_id,
                    target_id: fields.target_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Target.command.sendMessageToTarget。
///
///协议原文：
///
///<pre>Sends protocol message over session with given id.
///Consider using flat mode instead; see commands attachToTarget, setAutoAttach,
///and crbug.com/991325.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct SendMessageToTarget;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.sendMessageToTarget/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SendMessageToTargetResponse {}
impl<'de> serde::Deserialize<'de> for SendMessageToTargetResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SendMessageToTargetResponse;
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
                Ok(SendMessageToTargetResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SendMessageToTarget {
    type Params = SendMessageToTargetParams;
    type Response = SendMessageToTargetResponse;
    const METHOD: &'static str = "Target.sendMessageToTarget";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.setAutoAttach/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetAutoAttachParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.setAutoAttach/parameters/autoAttach。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to auto-attach to related targets.</pre>
    ///
    ///
    #[serde(rename = "autoAttach")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub auto_attach: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.setAutoAttach/parameters/waitForDebuggerOnStart。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to pause new targets when attaching to them. Use `Runtime.runIfWaitingForDebugger`
    ///to run paused targets.</pre>
    ///
    ///
    #[serde(rename = "waitForDebuggerOnStart")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub wait_for_debugger_on_start: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.setAutoAttach/parameters/flatten。
    ///
    ///协议原文：
    ///
    ///<pre>Enables "flat" access to the session via specifying sessionId attribute in the commands.
    ///We plan to make this the default, deprecate non-flattened mode,
    ///and eventually retire it. See crbug.com/991325.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "flatten")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub flatten: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.setAutoAttach/parameters/filter。
    ///
    ///协议原文：
    ///
    ///<pre>Only targets matching filter will be attached.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "filter")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub filter: Option<crate::target::types::TargetFilter>,
}
impl<'de> serde::Deserialize<'de> for SetAutoAttachParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetAutoAttachParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.setAutoAttach/parameters/autoAttach。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to auto-attach to related targets.</pre>
                    ///
                    ///
                    #[serde(rename = "autoAttach")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub auto_attach: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.setAutoAttach/parameters/waitForDebuggerOnStart。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to pause new targets when attaching to them. Use `Runtime.runIfWaitingForDebugger`
                    ///to run paused targets.</pre>
                    ///
                    ///
                    #[serde(rename = "waitForDebuggerOnStart")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub wait_for_debugger_on_start: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.setAutoAttach/parameters/flatten。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Enables "flat" access to the session via specifying sessionId attribute in the commands.
                    ///We plan to make this the default, deprecate non-flattened mode,
                    ///and eventually retire it. See crbug.com/991325.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "flatten")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub flatten: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.setAutoAttach/parameters/filter。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Only targets matching filter will be attached.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "filter")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub filter: Option<crate::target::types::TargetFilter>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetAutoAttachParams {
                    auto_attach: fields.auto_attach,
                    wait_for_debugger_on_start: fields.wait_for_debugger_on_start,
                    flatten: fields.flatten,
                    filter: fields.filter,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Target.command.setAutoAttach。
///
///协议原文：
///
///<pre>Controls whether to automatically attach to new targets which are considered
///to be directly related to this one (for example, iframes or workers).
///When turned on, attaches to all existing related targets as well. When turned off,
///automatically detaches from all currently attached targets.
///This also clears all targets added by `autoAttachRelated` from the list of targets to watch
///for creation of related targets.
///You might want to call this recursively for auto-attached targets to attach
///to all available targets.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetAutoAttach;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.setAutoAttach/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetAutoAttachResponse {}
impl<'de> serde::Deserialize<'de> for SetAutoAttachResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetAutoAttachResponse;
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
                Ok(SetAutoAttachResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetAutoAttach {
    type Params = SetAutoAttachParams;
    type Response = SetAutoAttachResponse;
    const METHOD: &'static str = "Target.setAutoAttach";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.setDiscoverTargets/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDiscoverTargetsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.setDiscoverTargets/parameters/discover。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to discover available targets.</pre>
    ///
    ///
    #[serde(rename = "discover")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub discover: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.setDiscoverTargets/parameters/filter。
    ///
    ///协议原文：
    ///
    ///<pre>Only targets matching filter will be attached. If `discover` is false,
    ///`filter` must be omitted or empty.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "filter")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub filter: Option<crate::target::types::TargetFilter>,
}
impl<'de> serde::Deserialize<'de> for SetDiscoverTargetsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDiscoverTargetsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.setDiscoverTargets/parameters/discover。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to discover available targets.</pre>
                    ///
                    ///
                    #[serde(rename = "discover")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub discover: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.command.setDiscoverTargets/parameters/filter。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Only targets matching filter will be attached. If `discover` is false,
                    ///`filter` must be omitted or empty.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "filter")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub filter: Option<crate::target::types::TargetFilter>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetDiscoverTargetsParams {
                    discover: fields.discover,
                    filter: fields.filter,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Target.command.setDiscoverTargets。
///
///协议原文：
///
///<pre>Controls whether to discover available targets and notify via
///`targetCreated/targetInfoChanged/targetDestroyed` events.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetDiscoverTargets;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.setDiscoverTargets/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDiscoverTargetsResponse {}
impl<'de> serde::Deserialize<'de> for SetDiscoverTargetsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDiscoverTargetsResponse;
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
                Ok(SetDiscoverTargetsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetDiscoverTargets {
    type Params = SetDiscoverTargetsParams;
    type Response = SetDiscoverTargetsResponse;
    const METHOD: &'static str = "Target.setDiscoverTargets";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.setRemoteLocations/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetRemoteLocationsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.setRemoteLocations/parameters/locations。
    ///
    ///协议原文：
    ///
    ///<pre>List of remote locations.</pre>
    ///
    ///
    #[serde(rename = "locations")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub locations: Vec<crate::target::types::RemoteLocation>,
}
impl<'de> serde::Deserialize<'de> for SetRemoteLocationsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetRemoteLocationsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.command.setRemoteLocations/parameters/locations。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>List of remote locations.</pre>
                    ///
                    ///
                    #[serde(rename = "locations")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub locations: Vec<crate::target::types::RemoteLocation>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetRemoteLocationsParams {
                    locations: fields.locations,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Target.command.setRemoteLocations。
///
///协议原文：
///
///<pre>Enables target discovery for the specified locations, when `setDiscoverTargets` was set to
///`true`.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetRemoteLocations;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.command.setRemoteLocations/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetRemoteLocationsResponse {}
impl<'de> serde::Deserialize<'de> for SetRemoteLocationsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetRemoteLocationsResponse;
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
                Ok(SetRemoteLocationsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetRemoteLocations {
    type Params = SetRemoteLocationsParams;
    type Response = SetRemoteLocationsResponse;
    const METHOD: &'static str = "Target.setRemoteLocations";
}
