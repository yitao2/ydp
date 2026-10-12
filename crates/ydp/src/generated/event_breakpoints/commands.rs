///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：EventBreakpoints.command.disable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：EventBreakpoints.command.disable。
///
///协议原文：
///
///<pre>Removes all breakpoints</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：EventBreakpoints.command.disable/returns。
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
    const METHOD: &'static str = "EventBreakpoints.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：EventBreakpoints.command.removeInstrumentationBreakpoint/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveInstrumentationBreakpointParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：EventBreakpoints.command.removeInstrumentationBreakpoint/parameters/eventName。
    ///
    ///协议原文：
    ///
    ///<pre>Instrumentation name to stop on.</pre>
    ///
    ///
    #[serde(rename = "eventName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub event_name: String,
}
impl<'de> serde::Deserialize<'de> for RemoveInstrumentationBreakpointParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveInstrumentationBreakpointParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：EventBreakpoints.command.removeInstrumentationBreakpoint/parameters/eventName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Instrumentation name to stop on.</pre>
                    ///
                    ///
                    #[serde(rename = "eventName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub event_name: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RemoveInstrumentationBreakpointParams {
                    event_name: fields.event_name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：EventBreakpoints.command.removeInstrumentationBreakpoint。
///
///协议原文：
///
///<pre>Removes breakpoint on particular native event.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RemoveInstrumentationBreakpoint;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：EventBreakpoints.command.removeInstrumentationBreakpoint/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveInstrumentationBreakpointResponse {}
impl<'de> serde::Deserialize<'de> for RemoveInstrumentationBreakpointResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveInstrumentationBreakpointResponse;
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
                Ok(RemoveInstrumentationBreakpointResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RemoveInstrumentationBreakpoint {
    type Params = RemoveInstrumentationBreakpointParams;
    type Response = RemoveInstrumentationBreakpointResponse;
    const METHOD: &'static str = "EventBreakpoints.removeInstrumentationBreakpoint";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：EventBreakpoints.command.setInstrumentationBreakpoint/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetInstrumentationBreakpointParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：EventBreakpoints.command.setInstrumentationBreakpoint/parameters/eventName。
    ///
    ///协议原文：
    ///
    ///<pre>Instrumentation name to stop on.</pre>
    ///
    ///
    #[serde(rename = "eventName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub event_name: String,
}
impl<'de> serde::Deserialize<'de> for SetInstrumentationBreakpointParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetInstrumentationBreakpointParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：EventBreakpoints.command.setInstrumentationBreakpoint/parameters/eventName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Instrumentation name to stop on.</pre>
                    ///
                    ///
                    #[serde(rename = "eventName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub event_name: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetInstrumentationBreakpointParams {
                    event_name: fields.event_name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：EventBreakpoints.command.setInstrumentationBreakpoint。
///
///协议原文：
///
///<pre>Sets breakpoint on particular native event.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetInstrumentationBreakpoint;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：EventBreakpoints.command.setInstrumentationBreakpoint/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetInstrumentationBreakpointResponse {}
impl<'de> serde::Deserialize<'de> for SetInstrumentationBreakpointResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetInstrumentationBreakpointResponse;
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
                Ok(SetInstrumentationBreakpointResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetInstrumentationBreakpoint {
    type Params = SetInstrumentationBreakpointParams;
    type Response = SetInstrumentationBreakpointResponse;
    const METHOD: &'static str = "EventBreakpoints.setInstrumentationBreakpoint";
}
