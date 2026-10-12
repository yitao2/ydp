///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Inspector.event.detached/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DetachedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Inspector.event.detached/parameters/reason。
    ///
    ///协议原文：
    ///
    ///<pre>The reason why connection has been terminated.</pre>
    ///
    ///
    #[serde(rename = "reason")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub reason: String,
}
impl<'de> serde::Deserialize<'de> for DetachedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DetachedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Inspector.event.detached/parameters/reason。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The reason why connection has been terminated.</pre>
                    ///
                    ///
                    #[serde(rename = "reason")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub reason: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DetachedParams {
                    reason: fields.reason,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Inspector.event.detached。
///
///协议原文：
///
///<pre>Fired when remote debugging connection is about to be terminated. Contains detach reason.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Detached;
impl ydp_core::Event for Detached {
    type Params = DetachedParams;
    const METHOD: &'static str = "Inspector.detached";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Inspector.event.targetCrashed/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct TargetCrashedParams {}
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
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TargetCrashedParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Inspector.event.targetCrashed。
///
///协议原文：
///
///<pre>Fired when debugging target has crashed</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct TargetCrashed;
impl ydp_core::Event for TargetCrashed {
    type Params = TargetCrashedParams;
    const METHOD: &'static str = "Inspector.targetCrashed";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Inspector.event.targetReloadedAfterCrash/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct TargetReloadedAfterCrashParams {}
impl<'de> serde::Deserialize<'de> for TargetReloadedAfterCrashParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TargetReloadedAfterCrashParams;
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
                Ok(TargetReloadedAfterCrashParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Inspector.event.targetReloadedAfterCrash。
///
///协议原文：
///
///<pre>Fired when debugging target has reloaded after crash</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct TargetReloadedAfterCrash;
impl ydp_core::Event for TargetReloadedAfterCrash {
    type Params = TargetReloadedAfterCrashParams;
    const METHOD: &'static str = "Inspector.targetReloadedAfterCrash";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Inspector.event.workerScriptLoaded/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct WorkerScriptLoadedParams {}
impl<'de> serde::Deserialize<'de> for WorkerScriptLoadedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WorkerScriptLoadedParams;
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
                Ok(WorkerScriptLoadedParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Inspector.event.workerScriptLoaded。
///
///协议原文：
///
///<pre>Fired on worker targets when main worker script and any imported scripts have been evaluated.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct WorkerScriptLoaded;
impl ydp_core::Event for WorkerScriptLoaded {
    type Params = WorkerScriptLoadedParams;
    const METHOD: &'static str = "Inspector.workerScriptLoaded";
}
