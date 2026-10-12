///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：DOMDebugger.type.CSPViolationType。
///
///协议原文：
///
///<pre>CSP Violation type.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CspViolationType {
    ///协议值 "trustedtype-sink-violation"。
    TrustedtypeSinkViolation,
    ///协议值 "trustedtype-policy-violation"。
    TrustedtypePolicyViolation,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl CspViolationType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::dom_debugger::types::CspViolationType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::TrustedtypeSinkViolation => "trustedtype-sink-violation",
            Self::TrustedtypePolicyViolation => "trustedtype-policy-violation",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for CspViolationType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for CspViolationType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "trustedtype-sink-violation" => CspViolationType::TrustedtypeSinkViolation,
            "trustedtype-policy-violation" => CspViolationType::TrustedtypePolicyViolation,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for CspViolationType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CspViolationType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CspViolationType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "trustedtype-sink-violation" => CspViolationType::TrustedtypeSinkViolation,
                    "trustedtype-policy-violation" => CspViolationType::TrustedtypePolicyViolation,
                    _ => CspViolationType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(CspViolationType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：DOMDebugger.type.DOMBreakpointType。
///
///协议原文：
///
///<pre>DOM breakpoint type.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DomBreakpointType {
    ///协议值 "subtree-modified"。
    SubtreeModified,
    ///协议值 "attribute-modified"。
    AttributeModified,
    ///协议值 "node-removed"。
    NodeRemoved,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl DomBreakpointType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::dom_debugger::types::DomBreakpointType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::SubtreeModified => "subtree-modified",
            Self::AttributeModified => "attribute-modified",
            Self::NodeRemoved => "node-removed",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for DomBreakpointType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for DomBreakpointType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "subtree-modified" => DomBreakpointType::SubtreeModified,
            "attribute-modified" => DomBreakpointType::AttributeModified,
            "node-removed" => DomBreakpointType::NodeRemoved,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for DomBreakpointType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DomBreakpointType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DomBreakpointType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "subtree-modified" => DomBreakpointType::SubtreeModified,
                    "attribute-modified" => DomBreakpointType::AttributeModified,
                    "node-removed" => DomBreakpointType::NodeRemoved,
                    _ => DomBreakpointType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(DomBreakpointType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMDebugger.type.EventListener。
///
///协议原文：
///
///<pre>Object event listener.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct EventListener {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.type.EventListener/properties/type。
    ///
    ///协议原文：
    ///
    ///<pre>`EventListener`'s type.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.type.EventListener/properties/useCapture。
    ///
    ///协议原文：
    ///
    ///<pre>`EventListener`'s useCapture.</pre>
    ///
    ///
    #[serde(rename = "useCapture")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub use_capture: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.type.EventListener/properties/passive。
    ///
    ///协议原文：
    ///
    ///<pre>`EventListener`'s passive flag.</pre>
    ///
    ///
    #[serde(rename = "passive")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub passive: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.type.EventListener/properties/once。
    ///
    ///协议原文：
    ///
    ///<pre>`EventListener`'s once flag.</pre>
    ///
    ///
    #[serde(rename = "once")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub once: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.type.EventListener/properties/scriptId。
    ///
    ///协议原文：
    ///
    ///<pre>Script id of the handler code.</pre>
    ///
    ///
    #[serde(rename = "scriptId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub script_id: crate::runtime::types::ScriptId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.type.EventListener/properties/lineNumber。
    ///
    ///协议原文：
    ///
    ///<pre>Line number in the script (0-based).</pre>
    ///
    ///
    #[serde(rename = "lineNumber")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub line_number: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.type.EventListener/properties/columnNumber。
    ///
    ///协议原文：
    ///
    ///<pre>Column number in the script (0-based).</pre>
    ///
    ///
    #[serde(rename = "columnNumber")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub column_number: i64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMDebugger.type.EventListener/properties/handler。
    ///
    ///协议原文：
    ///
    ///<pre>Event handler function value.</pre>
    ///
    ///
    #[serde(rename = "handler")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub handler: Option<crate::runtime::types::RemoteObject>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMDebugger.type.EventListener/properties/originalHandler。
    ///
    ///协议原文：
    ///
    ///<pre>Event original handler function value.</pre>
    ///
    ///
    #[serde(rename = "originalHandler")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub original_handler: Option<crate::runtime::types::RemoteObject>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMDebugger.type.EventListener/properties/backendNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Node the listener is added to (if any).</pre>
    ///
    ///
    #[serde(rename = "backendNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
}
impl<'de> serde::Deserialize<'de> for EventListener {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EventListener;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.type.EventListener/properties/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`EventListener`'s type.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.type.EventListener/properties/useCapture。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`EventListener`'s useCapture.</pre>
                    ///
                    ///
                    #[serde(rename = "useCapture")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub use_capture: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.type.EventListener/properties/passive。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`EventListener`'s passive flag.</pre>
                    ///
                    ///
                    #[serde(rename = "passive")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub passive: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.type.EventListener/properties/once。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`EventListener`'s once flag.</pre>
                    ///
                    ///
                    #[serde(rename = "once")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub once: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.type.EventListener/properties/scriptId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Script id of the handler code.</pre>
                    ///
                    ///
                    #[serde(rename = "scriptId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub script_id: crate::runtime::types::ScriptId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.type.EventListener/properties/lineNumber。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Line number in the script (0-based).</pre>
                    ///
                    ///
                    #[serde(rename = "lineNumber")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub line_number: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMDebugger.type.EventListener/properties/columnNumber。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Column number in the script (0-based).</pre>
                    ///
                    ///
                    #[serde(rename = "columnNumber")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub column_number: i64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMDebugger.type.EventListener/properties/handler。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Event handler function value.</pre>
                    ///
                    ///
                    #[serde(rename = "handler")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub handler: Option<crate::runtime::types::RemoteObject>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMDebugger.type.EventListener/properties/originalHandler。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Event original handler function value.</pre>
                    ///
                    ///
                    #[serde(rename = "originalHandler")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub original_handler: Option<crate::runtime::types::RemoteObject>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOMDebugger.type.EventListener/properties/backendNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Node the listener is added to (if any).</pre>
                    ///
                    ///
                    #[serde(rename = "backendNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EventListener {
                    r#type: fields.r#type,
                    use_capture: fields.use_capture,
                    passive: fields.passive,
                    once: fields.once,
                    script_id: fields.script_id,
                    line_number: fields.line_number,
                    column_number: fields.column_number,
                    handler: fields.handler,
                    original_handler: fields.original_handler,
                    backend_node_id: fields.backend_node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
