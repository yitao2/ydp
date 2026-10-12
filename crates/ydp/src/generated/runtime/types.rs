///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.type.CallArgument。
///
///协议原文：
///
///<pre>Represents function call argument. Either remote object id `objectId`, primitive `value`,
///unserializable primitive value or neither of (for undefined) them should be specified.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CallArgument {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.CallArgument/properties/value。
    ///
    ///协议原文：
    ///
    ///<pre>Primitive value or serializable javascript object.</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<serde_json::Value>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.CallArgument/properties/unserializableValue。
    ///
    ///协议原文：
    ///
    ///<pre>Primitive value which can not be JSON-stringified.</pre>
    ///
    ///
    #[serde(rename = "unserializableValue")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub unserializable_value: Option<crate::runtime::types::UnserializableValue>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.CallArgument/properties/objectId。
    ///
    ///协议原文：
    ///
    ///<pre>Remote object handle.</pre>
    ///
    ///
    #[serde(rename = "objectId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub object_id: Option<crate::runtime::types::RemoteObjectId>,
}
impl<'de> serde::Deserialize<'de> for CallArgument {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CallArgument;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.CallArgument/properties/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Primitive value or serializable javascript object.</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub value: Option<serde_json::Value>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.CallArgument/properties/unserializableValue。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Primitive value which can not be JSON-stringified.</pre>
                    ///
                    ///
                    #[serde(rename = "unserializableValue")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub unserializable_value: Option<crate::runtime::types::UnserializableValue>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.CallArgument/properties/objectId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Remote object handle.</pre>
                    ///
                    ///
                    #[serde(rename = "objectId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub object_id: Option<crate::runtime::types::RemoteObjectId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CallArgument {
                    value: fields.value,
                    unserializable_value: fields.unserializable_value,
                    object_id: fields.object_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.type.CallFrame。
///
///协议原文：
///
///<pre>Stack entry for runtime errors and assertions.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CallFrame {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.CallFrame/properties/functionName。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript function name.</pre>
    ///
    ///
    #[serde(rename = "functionName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub function_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.CallFrame/properties/scriptId。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript script id.</pre>
    ///
    ///
    #[serde(rename = "scriptId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub script_id: crate::runtime::types::ScriptId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.CallFrame/properties/url。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript script name or url.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.CallFrame/properties/lineNumber。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript script line number (0-based).</pre>
    ///
    ///
    #[serde(rename = "lineNumber")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub line_number: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.CallFrame/properties/columnNumber。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript script column number (0-based).</pre>
    ///
    ///
    #[serde(rename = "columnNumber")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub column_number: i64,
}
impl<'de> serde::Deserialize<'de> for CallFrame {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CallFrame;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.CallFrame/properties/functionName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript function name.</pre>
                    ///
                    ///
                    #[serde(rename = "functionName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub function_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.CallFrame/properties/scriptId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript script id.</pre>
                    ///
                    ///
                    #[serde(rename = "scriptId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub script_id: crate::runtime::types::ScriptId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.CallFrame/properties/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript script name or url.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.CallFrame/properties/lineNumber。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript script line number (0-based).</pre>
                    ///
                    ///
                    #[serde(rename = "lineNumber")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub line_number: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.CallFrame/properties/columnNumber。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript script column number (0-based).</pre>
                    ///
                    ///
                    #[serde(rename = "columnNumber")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub column_number: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CallFrame {
                    function_name: fields.function_name,
                    script_id: fields.script_id,
                    url: fields.url,
                    line_number: fields.line_number,
                    column_number: fields.column_number,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.type.CustomPreview。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CustomPreview {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.CustomPreview/properties/header。
    ///
    ///协议原文：
    ///
    ///<pre>The JSON-stringified result of formatter.header(object, config) call.
    ///It contains json ML array that represents RemoteObject.</pre>
    ///
    ///
    #[serde(rename = "header")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub header: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.CustomPreview/properties/bodyGetterId。
    ///
    ///协议原文：
    ///
    ///<pre>If formatter returns true as a result of formatter.hasBody call then bodyGetterId will
    ///contain RemoteObjectId for the function that returns result of formatter.body(object, config) call.
    ///The result value is json ML array.</pre>
    ///
    ///
    #[serde(rename = "bodyGetterId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub body_getter_id: Option<crate::runtime::types::RemoteObjectId>,
}
impl<'de> serde::Deserialize<'de> for CustomPreview {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CustomPreview;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.CustomPreview/properties/header。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The JSON-stringified result of formatter.header(object, config) call.
                    ///It contains json ML array that represents RemoteObject.</pre>
                    ///
                    ///
                    #[serde(rename = "header")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub header: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.CustomPreview/properties/bodyGetterId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If formatter returns true as a result of formatter.hasBody call then bodyGetterId will
                    ///contain RemoteObjectId for the function that returns result of formatter.body(object, config) call.
                    ///The result value is json ML array.</pre>
                    ///
                    ///
                    #[serde(rename = "bodyGetterId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub body_getter_id: Option<crate::runtime::types::RemoteObjectId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CustomPreview {
                    header: fields.header,
                    body_getter_id: fields.body_getter_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Runtime.type.DeepSerializedValue/properties/type。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DeepSerializedValueType {
    ///协议值 "undefined"。
    Undefined,
    ///协议值 "null"。
    Null,
    ///协议值 "string"。
    String,
    ///协议值 "number"。
    Number,
    ///协议值 "boolean"。
    Boolean,
    ///协议值 "bigint"。
    Bigint,
    ///协议值 "regexp"。
    Regexp,
    ///协议值 "date"。
    Date,
    ///协议值 "symbol"。
    Symbol,
    ///协议值 "array"。
    Array,
    ///协议值 "object"。
    Object,
    ///协议值 "function"。
    Function,
    ///协议值 "map"。
    Map,
    ///协议值 "set"。
    Set,
    ///协议值 "weakmap"。
    Weakmap,
    ///协议值 "weakset"。
    Weakset,
    ///协议值 "error"。
    Error,
    ///协议值 "proxy"。
    Proxy,
    ///协议值 "promise"。
    Promise,
    ///协议值 "typedarray"。
    Typedarray,
    ///协议值 "arraybuffer"。
    Arraybuffer,
    ///协议值 "node"。
    Node,
    ///协议值 "window"。
    Window,
    ///协议值 "generator"。
    Generator,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl DeepSerializedValueType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::runtime::types::DeepSerializedValueType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Undefined => "undefined",
            Self::Null => "null",
            Self::String => "string",
            Self::Number => "number",
            Self::Boolean => "boolean",
            Self::Bigint => "bigint",
            Self::Regexp => "regexp",
            Self::Date => "date",
            Self::Symbol => "symbol",
            Self::Array => "array",
            Self::Object => "object",
            Self::Function => "function",
            Self::Map => "map",
            Self::Set => "set",
            Self::Weakmap => "weakmap",
            Self::Weakset => "weakset",
            Self::Error => "error",
            Self::Proxy => "proxy",
            Self::Promise => "promise",
            Self::Typedarray => "typedarray",
            Self::Arraybuffer => "arraybuffer",
            Self::Node => "node",
            Self::Window => "window",
            Self::Generator => "generator",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for DeepSerializedValueType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for DeepSerializedValueType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "undefined" => DeepSerializedValueType::Undefined,
            "null" => DeepSerializedValueType::Null,
            "string" => DeepSerializedValueType::String,
            "number" => DeepSerializedValueType::Number,
            "boolean" => DeepSerializedValueType::Boolean,
            "bigint" => DeepSerializedValueType::Bigint,
            "regexp" => DeepSerializedValueType::Regexp,
            "date" => DeepSerializedValueType::Date,
            "symbol" => DeepSerializedValueType::Symbol,
            "array" => DeepSerializedValueType::Array,
            "object" => DeepSerializedValueType::Object,
            "function" => DeepSerializedValueType::Function,
            "map" => DeepSerializedValueType::Map,
            "set" => DeepSerializedValueType::Set,
            "weakmap" => DeepSerializedValueType::Weakmap,
            "weakset" => DeepSerializedValueType::Weakset,
            "error" => DeepSerializedValueType::Error,
            "proxy" => DeepSerializedValueType::Proxy,
            "promise" => DeepSerializedValueType::Promise,
            "typedarray" => DeepSerializedValueType::Typedarray,
            "arraybuffer" => DeepSerializedValueType::Arraybuffer,
            "node" => DeepSerializedValueType::Node,
            "window" => DeepSerializedValueType::Window,
            "generator" => DeepSerializedValueType::Generator,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for DeepSerializedValueType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeepSerializedValueType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeepSerializedValueType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "undefined" => DeepSerializedValueType::Undefined,
                    "null" => DeepSerializedValueType::Null,
                    "string" => DeepSerializedValueType::String,
                    "number" => DeepSerializedValueType::Number,
                    "boolean" => DeepSerializedValueType::Boolean,
                    "bigint" => DeepSerializedValueType::Bigint,
                    "regexp" => DeepSerializedValueType::Regexp,
                    "date" => DeepSerializedValueType::Date,
                    "symbol" => DeepSerializedValueType::Symbol,
                    "array" => DeepSerializedValueType::Array,
                    "object" => DeepSerializedValueType::Object,
                    "function" => DeepSerializedValueType::Function,
                    "map" => DeepSerializedValueType::Map,
                    "set" => DeepSerializedValueType::Set,
                    "weakmap" => DeepSerializedValueType::Weakmap,
                    "weakset" => DeepSerializedValueType::Weakset,
                    "error" => DeepSerializedValueType::Error,
                    "proxy" => DeepSerializedValueType::Proxy,
                    "promise" => DeepSerializedValueType::Promise,
                    "typedarray" => DeepSerializedValueType::Typedarray,
                    "arraybuffer" => DeepSerializedValueType::Arraybuffer,
                    "node" => DeepSerializedValueType::Node,
                    "window" => DeepSerializedValueType::Window,
                    "generator" => DeepSerializedValueType::Generator,
                    _ => DeepSerializedValueType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(DeepSerializedValueType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.type.DeepSerializedValue。
///
///协议原文：
///
///<pre>Represents deep serialized value.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeepSerializedValue {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.DeepSerializedValue/properties/type。
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: DeepSerializedValueType,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.DeepSerializedValue/properties/value。
    #[serde(rename = "value")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<serde_json::Value>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.DeepSerializedValue/properties/objectId。
    #[serde(rename = "objectId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub object_id: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.DeepSerializedValue/properties/weakLocalObjectReference。
    ///
    ///协议原文：
    ///
    ///<pre>Set if value reference met more then once during serialization. In such
    ///case, value is provided only to one of the serialized values. Unique
    ///per value in the scope of one CDP call.</pre>
    ///
    ///
    #[serde(rename = "weakLocalObjectReference")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub weak_local_object_reference: Option<i64>,
}
impl<'de> serde::Deserialize<'de> for DeepSerializedValue {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeepSerializedValue;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.DeepSerializedValue/properties/type。
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: DeepSerializedValueType,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.DeepSerializedValue/properties/value。
                    #[serde(rename = "value")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub value: Option<serde_json::Value>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.DeepSerializedValue/properties/objectId。
                    #[serde(rename = "objectId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub object_id: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.DeepSerializedValue/properties/weakLocalObjectReference。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Set if value reference met more then once during serialization. In such
                    ///case, value is provided only to one of the serialized values. Unique
                    ///per value in the scope of one CDP call.</pre>
                    ///
                    ///
                    #[serde(rename = "weakLocalObjectReference")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub weak_local_object_reference: Option<i64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DeepSerializedValue {
                    r#type: fields.r#type,
                    value: fields.value,
                    object_id: fields.object_id,
                    weak_local_object_reference: fields.weak_local_object_reference,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.type.EntryPreview。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EntryPreview {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.EntryPreview/properties/key。
    ///
    ///协议原文：
    ///
    ///<pre>Preview of the key. Specified for map-like collection entries.</pre>
    ///
    ///
    #[serde(rename = "key")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub key: Option<crate::runtime::types::ObjectPreview>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.EntryPreview/properties/value。
    ///
    ///协议原文：
    ///
    ///<pre>Preview of the value.</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: crate::runtime::types::ObjectPreview,
}
impl<'de> serde::Deserialize<'de> for EntryPreview {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EntryPreview;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.EntryPreview/properties/key。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Preview of the key. Specified for map-like collection entries.</pre>
                    ///
                    ///
                    #[serde(rename = "key")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub key: Option<crate::runtime::types::ObjectPreview>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.EntryPreview/properties/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Preview of the value.</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: crate::runtime::types::ObjectPreview,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EntryPreview {
                    key: fields.key,
                    value: fields.value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.type.ExceptionDetails。
///
///协议原文：
///
///<pre>Detailed information about exception (or error) that was thrown during script compilation or
///execution.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ExceptionDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.ExceptionDetails/properties/exceptionId。
    ///
    ///协议原文：
    ///
    ///<pre>Exception id.</pre>
    ///
    ///
    #[serde(rename = "exceptionId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub exception_id: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.ExceptionDetails/properties/text。
    ///
    ///协议原文：
    ///
    ///<pre>Exception text, which should be used together with exception object when available.</pre>
    ///
    ///
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.ExceptionDetails/properties/lineNumber。
    ///
    ///协议原文：
    ///
    ///<pre>Line number of the exception location (0-based).</pre>
    ///
    ///
    #[serde(rename = "lineNumber")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub line_number: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.ExceptionDetails/properties/columnNumber。
    ///
    ///协议原文：
    ///
    ///<pre>Column number of the exception location (0-based).</pre>
    ///
    ///
    #[serde(rename = "columnNumber")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub column_number: i64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.ExceptionDetails/properties/scriptId。
    ///
    ///协议原文：
    ///
    ///<pre>Script ID of the exception location.</pre>
    ///
    ///
    #[serde(rename = "scriptId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub script_id: Option<crate::runtime::types::ScriptId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.ExceptionDetails/properties/url。
    ///
    ///协议原文：
    ///
    ///<pre>URL of the exception location, to be used when the script was not reported.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.ExceptionDetails/properties/stackTrace。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript stack trace if available.</pre>
    ///
    ///
    #[serde(rename = "stackTrace")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub stack_trace: Option<crate::runtime::types::StackTrace>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.ExceptionDetails/properties/exception。
    ///
    ///协议原文：
    ///
    ///<pre>Exception object if available.</pre>
    ///
    ///
    #[serde(rename = "exception")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub exception: Option<crate::runtime::types::RemoteObject>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.ExceptionDetails/properties/executionContextId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the context where exception happened.</pre>
    ///
    ///
    #[serde(rename = "executionContextId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_context_id: Option<crate::runtime::types::ExecutionContextId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.ExceptionDetails/properties/exceptionMetaData。
    ///
    ///协议原文：
    ///
    ///<pre>Dictionary with entries of meta data that the client associated
    ///with this exception, such as information about associated network
    ///requests, etc.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "exceptionMetaData")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub exception_meta_data: Option<std::collections::BTreeMap<String, serde_json::Value>>,
}
impl<'de> serde::Deserialize<'de> for ExceptionDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExceptionDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.ExceptionDetails/properties/exceptionId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Exception id.</pre>
                    ///
                    ///
                    #[serde(rename = "exceptionId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub exception_id: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.ExceptionDetails/properties/text。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Exception text, which should be used together with exception object when available.</pre>
                    ///
                    ///
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.ExceptionDetails/properties/lineNumber。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Line number of the exception location (0-based).</pre>
                    ///
                    ///
                    #[serde(rename = "lineNumber")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub line_number: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.ExceptionDetails/properties/columnNumber。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Column number of the exception location (0-based).</pre>
                    ///
                    ///
                    #[serde(rename = "columnNumber")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub column_number: i64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.ExceptionDetails/properties/scriptId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Script ID of the exception location.</pre>
                    ///
                    ///
                    #[serde(rename = "scriptId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub script_id: Option<crate::runtime::types::ScriptId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.ExceptionDetails/properties/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>URL of the exception location, to be used when the script was not reported.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub url: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.ExceptionDetails/properties/stackTrace。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript stack trace if available.</pre>
                    ///
                    ///
                    #[serde(rename = "stackTrace")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub stack_trace: Option<crate::runtime::types::StackTrace>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.ExceptionDetails/properties/exception。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Exception object if available.</pre>
                    ///
                    ///
                    #[serde(rename = "exception")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub exception: Option<crate::runtime::types::RemoteObject>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.ExceptionDetails/properties/executionContextId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the context where exception happened.</pre>
                    ///
                    ///
                    #[serde(rename = "executionContextId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub execution_context_id: Option<crate::runtime::types::ExecutionContextId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.ExceptionDetails/properties/exceptionMetaData。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Dictionary with entries of meta data that the client associated
                    ///with this exception, such as information about associated network
                    ///requests, etc.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "exceptionMetaData")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub exception_meta_data:
                        Option<std::collections::BTreeMap<String, serde_json::Value>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ExceptionDetails {
                    exception_id: fields.exception_id,
                    text: fields.text,
                    line_number: fields.line_number,
                    column_number: fields.column_number,
                    script_id: fields.script_id,
                    url: fields.url,
                    stack_trace: fields.stack_trace,
                    exception: fields.exception,
                    execution_context_id: fields.execution_context_id,
                    exception_meta_data: fields.exception_meta_data,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.type.ExecutionContextDescription。
///
///协议原文：
///
///<pre>Description of an isolated world.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ExecutionContextDescription {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.ExecutionContextDescription/properties/id。
    ///
    ///协议原文：
    ///
    ///<pre>Unique id of the execution context. It can be used to specify in which execution context
    ///script evaluation should be performed.</pre>
    ///
    ///
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: crate::runtime::types::ExecutionContextId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.ExecutionContextDescription/properties/origin。
    ///
    ///协议原文：
    ///
    ///<pre>Execution context origin.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.ExecutionContextDescription/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Human readable name describing given context.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.ExecutionContextDescription/properties/uniqueId。
    ///
    ///协议原文：
    ///
    ///<pre>A system-unique execution context identifier. Unlike the id, this is unique across
    ///multiple processes, so can be reliably used to identify specific context while backend
    ///performs a cross-process navigation.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "uniqueId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub unique_id: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.ExecutionContextDescription/properties/auxData。
    ///
    ///协议原文：
    ///
    ///<pre>Embedder-specific auxiliary data likely matching {isDefault: boolean, type: 'default'|'isolated'|'worker', frameId: string}</pre>
    ///
    ///
    #[serde(rename = "auxData")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub aux_data: Option<std::collections::BTreeMap<String, serde_json::Value>>,
}
impl<'de> serde::Deserialize<'de> for ExecutionContextDescription {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExecutionContextDescription;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.ExecutionContextDescription/properties/id。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Unique id of the execution context. It can be used to specify in which execution context
                    ///script evaluation should be performed.</pre>
                    ///
                    ///
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: crate::runtime::types::ExecutionContextId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.ExecutionContextDescription/properties/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Execution context origin.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.ExecutionContextDescription/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Human readable name describing given context.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.ExecutionContextDescription/properties/uniqueId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A system-unique execution context identifier. Unlike the id, this is unique across
                    ///multiple processes, so can be reliably used to identify specific context while backend
                    ///performs a cross-process navigation.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "uniqueId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub unique_id: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.ExecutionContextDescription/properties/auxData。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Embedder-specific auxiliary data likely matching {isDefault: boolean, type: 'default'|'isolated'|'worker', frameId: string}</pre>
                    ///
                    ///
                    #[serde(rename = "auxData")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub aux_data: Option<std::collections::BTreeMap<String, serde_json::Value>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ExecutionContextDescription {
                    id: fields.id,
                    origin: fields.origin,
                    name: fields.name,
                    unique_id: fields.unique_id,
                    aux_data: fields.aux_data,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Runtime.type.ExecutionContextId。
///
///协议原文：
///
///<pre>Id of an execution context.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExecutionContextId(i64);
impl ExecutionContextId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::runtime::types::ExecutionContextId = serde_json::from_str("0").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::runtime::types::ExecutionContextId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("0").unwrap());
    ///```
    pub fn new(value: i64) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::runtime::types::ExecutionContextId = serde_json::from_str("0").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::runtime::types::ExecutionContextId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("0").unwrap());
    ///```
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.type.InternalPropertyDescriptor。
///
///协议原文：
///
///<pre>Object internal property descriptor. This property isn't normally visible in JavaScript code.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct InternalPropertyDescriptor {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.InternalPropertyDescriptor/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Conventional property name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.InternalPropertyDescriptor/properties/value。
    ///
    ///协议原文：
    ///
    ///<pre>The value associated with the property.</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<crate::runtime::types::RemoteObject>,
}
impl<'de> serde::Deserialize<'de> for InternalPropertyDescriptor {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InternalPropertyDescriptor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.InternalPropertyDescriptor/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Conventional property name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.InternalPropertyDescriptor/properties/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The value associated with the property.</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub value: Option<crate::runtime::types::RemoteObject>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(InternalPropertyDescriptor {
                    name: fields.name,
                    value: fields.value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Runtime.type.ObjectPreview/properties/type。
///
///协议原文：
///
///<pre>Object type.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ObjectPreviewType {
    ///协议值 "object"。
    Object,
    ///协议值 "function"。
    Function,
    ///协议值 "undefined"。
    Undefined,
    ///协议值 "string"。
    String,
    ///协议值 "number"。
    Number,
    ///协议值 "boolean"。
    Boolean,
    ///协议值 "symbol"。
    Symbol,
    ///协议值 "bigint"。
    Bigint,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ObjectPreviewType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::runtime::types::ObjectPreviewType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Object => "object",
            Self::Function => "function",
            Self::Undefined => "undefined",
            Self::String => "string",
            Self::Number => "number",
            Self::Boolean => "boolean",
            Self::Symbol => "symbol",
            Self::Bigint => "bigint",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ObjectPreviewType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ObjectPreviewType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "object" => ObjectPreviewType::Object,
            "function" => ObjectPreviewType::Function,
            "undefined" => ObjectPreviewType::Undefined,
            "string" => ObjectPreviewType::String,
            "number" => ObjectPreviewType::Number,
            "boolean" => ObjectPreviewType::Boolean,
            "symbol" => ObjectPreviewType::Symbol,
            "bigint" => ObjectPreviewType::Bigint,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ObjectPreviewType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ObjectPreviewType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ObjectPreviewType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "object" => ObjectPreviewType::Object,
                    "function" => ObjectPreviewType::Function,
                    "undefined" => ObjectPreviewType::Undefined,
                    "string" => ObjectPreviewType::String,
                    "number" => ObjectPreviewType::Number,
                    "boolean" => ObjectPreviewType::Boolean,
                    "symbol" => ObjectPreviewType::Symbol,
                    "bigint" => ObjectPreviewType::Bigint,
                    _ => ObjectPreviewType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ObjectPreviewType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Runtime.type.ObjectPreview/properties/subtype。
///
///协议原文：
///
///<pre>Object subtype hint. Specified for `object` type values only.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ObjectPreviewSubtype {
    ///协议值 "array"。
    Array,
    ///协议值 "null"。
    Null,
    ///协议值 "node"。
    Node,
    ///协议值 "regexp"。
    Regexp,
    ///协议值 "date"。
    Date,
    ///协议值 "map"。
    Map,
    ///协议值 "set"。
    Set,
    ///协议值 "weakmap"。
    Weakmap,
    ///协议值 "weakset"。
    Weakset,
    ///协议值 "iterator"。
    Iterator,
    ///协议值 "generator"。
    Generator,
    ///协议值 "error"。
    Error,
    ///协议值 "proxy"。
    Proxy,
    ///协议值 "promise"。
    Promise,
    ///协议值 "typedarray"。
    Typedarray,
    ///协议值 "arraybuffer"。
    Arraybuffer,
    ///协议值 "dataview"。
    Dataview,
    ///协议值 "webassemblymemory"。
    Webassemblymemory,
    ///协议值 "wasmvalue"。
    Wasmvalue,
    ///协议值 "trustedtype"。
    Trustedtype,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ObjectPreviewSubtype {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::runtime::types::ObjectPreviewSubtype::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Array => "array",
            Self::Null => "null",
            Self::Node => "node",
            Self::Regexp => "regexp",
            Self::Date => "date",
            Self::Map => "map",
            Self::Set => "set",
            Self::Weakmap => "weakmap",
            Self::Weakset => "weakset",
            Self::Iterator => "iterator",
            Self::Generator => "generator",
            Self::Error => "error",
            Self::Proxy => "proxy",
            Self::Promise => "promise",
            Self::Typedarray => "typedarray",
            Self::Arraybuffer => "arraybuffer",
            Self::Dataview => "dataview",
            Self::Webassemblymemory => "webassemblymemory",
            Self::Wasmvalue => "wasmvalue",
            Self::Trustedtype => "trustedtype",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ObjectPreviewSubtype {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ObjectPreviewSubtype {
    fn from(value: String) -> Self {
        match value.as_str() {
            "array" => ObjectPreviewSubtype::Array,
            "null" => ObjectPreviewSubtype::Null,
            "node" => ObjectPreviewSubtype::Node,
            "regexp" => ObjectPreviewSubtype::Regexp,
            "date" => ObjectPreviewSubtype::Date,
            "map" => ObjectPreviewSubtype::Map,
            "set" => ObjectPreviewSubtype::Set,
            "weakmap" => ObjectPreviewSubtype::Weakmap,
            "weakset" => ObjectPreviewSubtype::Weakset,
            "iterator" => ObjectPreviewSubtype::Iterator,
            "generator" => ObjectPreviewSubtype::Generator,
            "error" => ObjectPreviewSubtype::Error,
            "proxy" => ObjectPreviewSubtype::Proxy,
            "promise" => ObjectPreviewSubtype::Promise,
            "typedarray" => ObjectPreviewSubtype::Typedarray,
            "arraybuffer" => ObjectPreviewSubtype::Arraybuffer,
            "dataview" => ObjectPreviewSubtype::Dataview,
            "webassemblymemory" => ObjectPreviewSubtype::Webassemblymemory,
            "wasmvalue" => ObjectPreviewSubtype::Wasmvalue,
            "trustedtype" => ObjectPreviewSubtype::Trustedtype,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ObjectPreviewSubtype {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ObjectPreviewSubtype {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ObjectPreviewSubtype;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "array" => ObjectPreviewSubtype::Array,
                    "null" => ObjectPreviewSubtype::Null,
                    "node" => ObjectPreviewSubtype::Node,
                    "regexp" => ObjectPreviewSubtype::Regexp,
                    "date" => ObjectPreviewSubtype::Date,
                    "map" => ObjectPreviewSubtype::Map,
                    "set" => ObjectPreviewSubtype::Set,
                    "weakmap" => ObjectPreviewSubtype::Weakmap,
                    "weakset" => ObjectPreviewSubtype::Weakset,
                    "iterator" => ObjectPreviewSubtype::Iterator,
                    "generator" => ObjectPreviewSubtype::Generator,
                    "error" => ObjectPreviewSubtype::Error,
                    "proxy" => ObjectPreviewSubtype::Proxy,
                    "promise" => ObjectPreviewSubtype::Promise,
                    "typedarray" => ObjectPreviewSubtype::Typedarray,
                    "arraybuffer" => ObjectPreviewSubtype::Arraybuffer,
                    "dataview" => ObjectPreviewSubtype::Dataview,
                    "webassemblymemory" => ObjectPreviewSubtype::Webassemblymemory,
                    "wasmvalue" => ObjectPreviewSubtype::Wasmvalue,
                    "trustedtype" => ObjectPreviewSubtype::Trustedtype,
                    _ => ObjectPreviewSubtype::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ObjectPreviewSubtype::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.type.ObjectPreview。
///
///协议原文：
///
///<pre>Object containing abbreviated remote object value.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ObjectPreview {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.ObjectPreview/properties/type。
    ///
    ///协议原文：
    ///
    ///<pre>Object type.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: ObjectPreviewType,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.ObjectPreview/properties/subtype。
    ///
    ///协议原文：
    ///
    ///<pre>Object subtype hint. Specified for `object` type values only.</pre>
    ///
    ///
    #[serde(rename = "subtype")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub subtype: Option<ObjectPreviewSubtype>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.ObjectPreview/properties/description。
    ///
    ///协议原文：
    ///
    ///<pre>String representation of the object.</pre>
    ///
    ///
    #[serde(rename = "description")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.ObjectPreview/properties/overflow。
    ///
    ///协议原文：
    ///
    ///<pre>True iff some of the properties or entries of the original object did not fit.</pre>
    ///
    ///
    #[serde(rename = "overflow")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub overflow: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.ObjectPreview/properties/properties。
    ///
    ///协议原文：
    ///
    ///<pre>List of the properties.</pre>
    ///
    ///
    #[serde(rename = "properties")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub properties: Vec<crate::runtime::types::PropertyPreview>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.ObjectPreview/properties/entries。
    ///
    ///协议原文：
    ///
    ///<pre>List of the entries. Specified for `map` and `set` subtype values only.</pre>
    ///
    ///
    #[serde(rename = "entries")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub entries: Option<Vec<crate::runtime::types::EntryPreview>>,
}
impl<'de> serde::Deserialize<'de> for ObjectPreview {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ObjectPreview;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.ObjectPreview/properties/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Object type.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: ObjectPreviewType,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.ObjectPreview/properties/subtype。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Object subtype hint. Specified for `object` type values only.</pre>
                    ///
                    ///
                    #[serde(rename = "subtype")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub subtype: Option<ObjectPreviewSubtype>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.ObjectPreview/properties/description。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>String representation of the object.</pre>
                    ///
                    ///
                    #[serde(rename = "description")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub description: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.ObjectPreview/properties/overflow。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True iff some of the properties or entries of the original object did not fit.</pre>
                    ///
                    ///
                    #[serde(rename = "overflow")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub overflow: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.ObjectPreview/properties/properties。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>List of the properties.</pre>
                    ///
                    ///
                    #[serde(rename = "properties")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub properties: Vec<crate::runtime::types::PropertyPreview>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.ObjectPreview/properties/entries。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>List of the entries. Specified for `map` and `set` subtype values only.</pre>
                    ///
                    ///
                    #[serde(rename = "entries")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub entries: Option<Vec<crate::runtime::types::EntryPreview>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ObjectPreview {
                    r#type: fields.r#type,
                    subtype: fields.subtype,
                    description: fields.description,
                    overflow: fields.overflow,
                    properties: fields.properties,
                    entries: fields.entries,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.type.PrivatePropertyDescriptor。
///
///协议原文：
///
///<pre>Object private field descriptor.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PrivatePropertyDescriptor {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.PrivatePropertyDescriptor/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Private property name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.PrivatePropertyDescriptor/properties/value。
    ///
    ///协议原文：
    ///
    ///<pre>The value associated with the private property.</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<crate::runtime::types::RemoteObject>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.PrivatePropertyDescriptor/properties/get。
    ///
    ///协议原文：
    ///
    ///<pre>A function which serves as a getter for the private property,
    ///or `undefined` if there is no getter (accessor descriptors only).</pre>
    ///
    ///
    #[serde(rename = "get")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub get: Option<crate::runtime::types::RemoteObject>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.PrivatePropertyDescriptor/properties/set。
    ///
    ///协议原文：
    ///
    ///<pre>A function which serves as a setter for the private property,
    ///or `undefined` if there is no setter (accessor descriptors only).</pre>
    ///
    ///
    #[serde(rename = "set")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub set: Option<crate::runtime::types::RemoteObject>,
}
impl<'de> serde::Deserialize<'de> for PrivatePropertyDescriptor {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PrivatePropertyDescriptor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.PrivatePropertyDescriptor/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Private property name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.PrivatePropertyDescriptor/properties/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The value associated with the private property.</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub value: Option<crate::runtime::types::RemoteObject>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.PrivatePropertyDescriptor/properties/get。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A function which serves as a getter for the private property,
                    ///or `undefined` if there is no getter (accessor descriptors only).</pre>
                    ///
                    ///
                    #[serde(rename = "get")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub get: Option<crate::runtime::types::RemoteObject>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.PrivatePropertyDescriptor/properties/set。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A function which serves as a setter for the private property,
                    ///or `undefined` if there is no setter (accessor descriptors only).</pre>
                    ///
                    ///
                    #[serde(rename = "set")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub set: Option<crate::runtime::types::RemoteObject>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PrivatePropertyDescriptor {
                    name: fields.name,
                    value: fields.value,
                    get: fields.get,
                    set: fields.set,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.type.PropertyDescriptor。
///
///协议原文：
///
///<pre>Object property descriptor.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct PropertyDescriptor {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.PropertyDescriptor/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Property name or symbol description.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.PropertyDescriptor/properties/value。
    ///
    ///协议原文：
    ///
    ///<pre>The value associated with the property.</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<crate::runtime::types::RemoteObject>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.PropertyDescriptor/properties/writable。
    ///
    ///协议原文：
    ///
    ///<pre>True if the value associated with the property may be changed (data descriptors only).</pre>
    ///
    ///
    #[serde(rename = "writable")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub writable: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.PropertyDescriptor/properties/get。
    ///
    ///协议原文：
    ///
    ///<pre>A function which serves as a getter for the property, or `undefined` if there is no getter
    ///(accessor descriptors only).</pre>
    ///
    ///
    #[serde(rename = "get")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub get: Option<crate::runtime::types::RemoteObject>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.PropertyDescriptor/properties/set。
    ///
    ///协议原文：
    ///
    ///<pre>A function which serves as a setter for the property, or `undefined` if there is no setter
    ///(accessor descriptors only).</pre>
    ///
    ///
    #[serde(rename = "set")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub set: Option<crate::runtime::types::RemoteObject>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.PropertyDescriptor/properties/configurable。
    ///
    ///协议原文：
    ///
    ///<pre>True if the type of this property descriptor may be changed and if the property may be
    ///deleted from the corresponding object.</pre>
    ///
    ///
    #[serde(rename = "configurable")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub configurable: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.PropertyDescriptor/properties/enumerable。
    ///
    ///协议原文：
    ///
    ///<pre>True if this property shows up during enumeration of the properties on the corresponding
    ///object.</pre>
    ///
    ///
    #[serde(rename = "enumerable")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub enumerable: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.PropertyDescriptor/properties/wasThrown。
    ///
    ///协议原文：
    ///
    ///<pre>True if the result was thrown during the evaluation.</pre>
    ///
    ///
    #[serde(rename = "wasThrown")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub was_thrown: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.PropertyDescriptor/properties/isOwn。
    ///
    ///协议原文：
    ///
    ///<pre>True if the property is owned for the object.</pre>
    ///
    ///
    #[serde(rename = "isOwn")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_own: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.PropertyDescriptor/properties/symbol。
    ///
    ///协议原文：
    ///
    ///<pre>Property symbol object, if the property is of the `symbol` type.</pre>
    ///
    ///
    #[serde(rename = "symbol")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub symbol: Option<crate::runtime::types::RemoteObject>,
}
impl<'de> serde::Deserialize<'de> for PropertyDescriptor {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PropertyDescriptor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.PropertyDescriptor/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Property name or symbol description.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.PropertyDescriptor/properties/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The value associated with the property.</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub value: Option<crate::runtime::types::RemoteObject>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.PropertyDescriptor/properties/writable。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if the value associated with the property may be changed (data descriptors only).</pre>
                    ///
                    ///
                    #[serde(rename = "writable")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub writable: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.PropertyDescriptor/properties/get。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A function which serves as a getter for the property, or `undefined` if there is no getter
                    ///(accessor descriptors only).</pre>
                    ///
                    ///
                    #[serde(rename = "get")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub get: Option<crate::runtime::types::RemoteObject>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.PropertyDescriptor/properties/set。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A function which serves as a setter for the property, or `undefined` if there is no setter
                    ///(accessor descriptors only).</pre>
                    ///
                    ///
                    #[serde(rename = "set")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub set: Option<crate::runtime::types::RemoteObject>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.PropertyDescriptor/properties/configurable。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if the type of this property descriptor may be changed and if the property may be
                    ///deleted from the corresponding object.</pre>
                    ///
                    ///
                    #[serde(rename = "configurable")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub configurable: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.PropertyDescriptor/properties/enumerable。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if this property shows up during enumeration of the properties on the corresponding
                    ///object.</pre>
                    ///
                    ///
                    #[serde(rename = "enumerable")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub enumerable: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.PropertyDescriptor/properties/wasThrown。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if the result was thrown during the evaluation.</pre>
                    ///
                    ///
                    #[serde(rename = "wasThrown")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub was_thrown: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.PropertyDescriptor/properties/isOwn。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if the property is owned for the object.</pre>
                    ///
                    ///
                    #[serde(rename = "isOwn")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub is_own: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.PropertyDescriptor/properties/symbol。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Property symbol object, if the property is of the `symbol` type.</pre>
                    ///
                    ///
                    #[serde(rename = "symbol")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub symbol: Option<crate::runtime::types::RemoteObject>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PropertyDescriptor {
                    name: fields.name,
                    value: fields.value,
                    writable: fields.writable,
                    get: fields.get,
                    set: fields.set,
                    configurable: fields.configurable,
                    enumerable: fields.enumerable,
                    was_thrown: fields.was_thrown,
                    is_own: fields.is_own,
                    symbol: fields.symbol,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Runtime.type.PropertyPreview/properties/type。
///
///协议原文：
///
///<pre>Object type. Accessor means that the property itself is an accessor property.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PropertyPreviewType {
    ///协议值 "object"。
    Object,
    ///协议值 "function"。
    Function,
    ///协议值 "undefined"。
    Undefined,
    ///协议值 "string"。
    String,
    ///协议值 "number"。
    Number,
    ///协议值 "boolean"。
    Boolean,
    ///协议值 "symbol"。
    Symbol,
    ///协议值 "accessor"。
    Accessor,
    ///协议值 "bigint"。
    Bigint,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl PropertyPreviewType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::runtime::types::PropertyPreviewType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Object => "object",
            Self::Function => "function",
            Self::Undefined => "undefined",
            Self::String => "string",
            Self::Number => "number",
            Self::Boolean => "boolean",
            Self::Symbol => "symbol",
            Self::Accessor => "accessor",
            Self::Bigint => "bigint",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for PropertyPreviewType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for PropertyPreviewType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "object" => PropertyPreviewType::Object,
            "function" => PropertyPreviewType::Function,
            "undefined" => PropertyPreviewType::Undefined,
            "string" => PropertyPreviewType::String,
            "number" => PropertyPreviewType::Number,
            "boolean" => PropertyPreviewType::Boolean,
            "symbol" => PropertyPreviewType::Symbol,
            "accessor" => PropertyPreviewType::Accessor,
            "bigint" => PropertyPreviewType::Bigint,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for PropertyPreviewType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PropertyPreviewType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PropertyPreviewType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "object" => PropertyPreviewType::Object,
                    "function" => PropertyPreviewType::Function,
                    "undefined" => PropertyPreviewType::Undefined,
                    "string" => PropertyPreviewType::String,
                    "number" => PropertyPreviewType::Number,
                    "boolean" => PropertyPreviewType::Boolean,
                    "symbol" => PropertyPreviewType::Symbol,
                    "accessor" => PropertyPreviewType::Accessor,
                    "bigint" => PropertyPreviewType::Bigint,
                    _ => PropertyPreviewType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(PropertyPreviewType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Runtime.type.PropertyPreview/properties/subtype。
///
///协议原文：
///
///<pre>Object subtype hint. Specified for `object` type values only.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PropertyPreviewSubtype {
    ///协议值 "array"。
    Array,
    ///协议值 "null"。
    Null,
    ///协议值 "node"。
    Node,
    ///协议值 "regexp"。
    Regexp,
    ///协议值 "date"。
    Date,
    ///协议值 "map"。
    Map,
    ///协议值 "set"。
    Set,
    ///协议值 "weakmap"。
    Weakmap,
    ///协议值 "weakset"。
    Weakset,
    ///协议值 "iterator"。
    Iterator,
    ///协议值 "generator"。
    Generator,
    ///协议值 "error"。
    Error,
    ///协议值 "proxy"。
    Proxy,
    ///协议值 "promise"。
    Promise,
    ///协议值 "typedarray"。
    Typedarray,
    ///协议值 "arraybuffer"。
    Arraybuffer,
    ///协议值 "dataview"。
    Dataview,
    ///协议值 "webassemblymemory"。
    Webassemblymemory,
    ///协议值 "wasmvalue"。
    Wasmvalue,
    ///协议值 "trustedtype"。
    Trustedtype,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl PropertyPreviewSubtype {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::runtime::types::PropertyPreviewSubtype::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Array => "array",
            Self::Null => "null",
            Self::Node => "node",
            Self::Regexp => "regexp",
            Self::Date => "date",
            Self::Map => "map",
            Self::Set => "set",
            Self::Weakmap => "weakmap",
            Self::Weakset => "weakset",
            Self::Iterator => "iterator",
            Self::Generator => "generator",
            Self::Error => "error",
            Self::Proxy => "proxy",
            Self::Promise => "promise",
            Self::Typedarray => "typedarray",
            Self::Arraybuffer => "arraybuffer",
            Self::Dataview => "dataview",
            Self::Webassemblymemory => "webassemblymemory",
            Self::Wasmvalue => "wasmvalue",
            Self::Trustedtype => "trustedtype",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for PropertyPreviewSubtype {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for PropertyPreviewSubtype {
    fn from(value: String) -> Self {
        match value.as_str() {
            "array" => PropertyPreviewSubtype::Array,
            "null" => PropertyPreviewSubtype::Null,
            "node" => PropertyPreviewSubtype::Node,
            "regexp" => PropertyPreviewSubtype::Regexp,
            "date" => PropertyPreviewSubtype::Date,
            "map" => PropertyPreviewSubtype::Map,
            "set" => PropertyPreviewSubtype::Set,
            "weakmap" => PropertyPreviewSubtype::Weakmap,
            "weakset" => PropertyPreviewSubtype::Weakset,
            "iterator" => PropertyPreviewSubtype::Iterator,
            "generator" => PropertyPreviewSubtype::Generator,
            "error" => PropertyPreviewSubtype::Error,
            "proxy" => PropertyPreviewSubtype::Proxy,
            "promise" => PropertyPreviewSubtype::Promise,
            "typedarray" => PropertyPreviewSubtype::Typedarray,
            "arraybuffer" => PropertyPreviewSubtype::Arraybuffer,
            "dataview" => PropertyPreviewSubtype::Dataview,
            "webassemblymemory" => PropertyPreviewSubtype::Webassemblymemory,
            "wasmvalue" => PropertyPreviewSubtype::Wasmvalue,
            "trustedtype" => PropertyPreviewSubtype::Trustedtype,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for PropertyPreviewSubtype {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PropertyPreviewSubtype {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PropertyPreviewSubtype;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "array" => PropertyPreviewSubtype::Array,
                    "null" => PropertyPreviewSubtype::Null,
                    "node" => PropertyPreviewSubtype::Node,
                    "regexp" => PropertyPreviewSubtype::Regexp,
                    "date" => PropertyPreviewSubtype::Date,
                    "map" => PropertyPreviewSubtype::Map,
                    "set" => PropertyPreviewSubtype::Set,
                    "weakmap" => PropertyPreviewSubtype::Weakmap,
                    "weakset" => PropertyPreviewSubtype::Weakset,
                    "iterator" => PropertyPreviewSubtype::Iterator,
                    "generator" => PropertyPreviewSubtype::Generator,
                    "error" => PropertyPreviewSubtype::Error,
                    "proxy" => PropertyPreviewSubtype::Proxy,
                    "promise" => PropertyPreviewSubtype::Promise,
                    "typedarray" => PropertyPreviewSubtype::Typedarray,
                    "arraybuffer" => PropertyPreviewSubtype::Arraybuffer,
                    "dataview" => PropertyPreviewSubtype::Dataview,
                    "webassemblymemory" => PropertyPreviewSubtype::Webassemblymemory,
                    "wasmvalue" => PropertyPreviewSubtype::Wasmvalue,
                    "trustedtype" => PropertyPreviewSubtype::Trustedtype,
                    _ => PropertyPreviewSubtype::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(PropertyPreviewSubtype::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.type.PropertyPreview。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PropertyPreview {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.PropertyPreview/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Property name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.PropertyPreview/properties/type。
    ///
    ///协议原文：
    ///
    ///<pre>Object type. Accessor means that the property itself is an accessor property.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: PropertyPreviewType,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.PropertyPreview/properties/value。
    ///
    ///协议原文：
    ///
    ///<pre>User-friendly property value string.</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.PropertyPreview/properties/valuePreview。
    ///
    ///协议原文：
    ///
    ///<pre>Nested value preview.</pre>
    ///
    ///
    #[serde(rename = "valuePreview")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub value_preview: Option<crate::runtime::types::ObjectPreview>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.PropertyPreview/properties/subtype。
    ///
    ///协议原文：
    ///
    ///<pre>Object subtype hint. Specified for `object` type values only.</pre>
    ///
    ///
    #[serde(rename = "subtype")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub subtype: Option<PropertyPreviewSubtype>,
}
impl<'de> serde::Deserialize<'de> for PropertyPreview {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PropertyPreview;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.PropertyPreview/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Property name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.PropertyPreview/properties/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Object type. Accessor means that the property itself is an accessor property.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: PropertyPreviewType,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.PropertyPreview/properties/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>User-friendly property value string.</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub value: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.PropertyPreview/properties/valuePreview。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Nested value preview.</pre>
                    ///
                    ///
                    #[serde(rename = "valuePreview")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub value_preview: Option<crate::runtime::types::ObjectPreview>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.PropertyPreview/properties/subtype。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Object subtype hint. Specified for `object` type values only.</pre>
                    ///
                    ///
                    #[serde(rename = "subtype")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub subtype: Option<PropertyPreviewSubtype>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PropertyPreview {
                    name: fields.name,
                    r#type: fields.r#type,
                    value: fields.value,
                    value_preview: fields.value_preview,
                    subtype: fields.subtype,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Runtime.type.RemoteObject/properties/type。
///
///协议原文：
///
///<pre>Object type.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RemoteObjectType {
    ///协议值 "object"。
    Object,
    ///协议值 "function"。
    Function,
    ///协议值 "undefined"。
    Undefined,
    ///协议值 "string"。
    String,
    ///协议值 "number"。
    Number,
    ///协议值 "boolean"。
    Boolean,
    ///协议值 "symbol"。
    Symbol,
    ///协议值 "bigint"。
    Bigint,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl RemoteObjectType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::runtime::types::RemoteObjectType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Object => "object",
            Self::Function => "function",
            Self::Undefined => "undefined",
            Self::String => "string",
            Self::Number => "number",
            Self::Boolean => "boolean",
            Self::Symbol => "symbol",
            Self::Bigint => "bigint",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for RemoteObjectType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for RemoteObjectType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "object" => RemoteObjectType::Object,
            "function" => RemoteObjectType::Function,
            "undefined" => RemoteObjectType::Undefined,
            "string" => RemoteObjectType::String,
            "number" => RemoteObjectType::Number,
            "boolean" => RemoteObjectType::Boolean,
            "symbol" => RemoteObjectType::Symbol,
            "bigint" => RemoteObjectType::Bigint,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for RemoteObjectType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RemoteObjectType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoteObjectType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "object" => RemoteObjectType::Object,
                    "function" => RemoteObjectType::Function,
                    "undefined" => RemoteObjectType::Undefined,
                    "string" => RemoteObjectType::String,
                    "number" => RemoteObjectType::Number,
                    "boolean" => RemoteObjectType::Boolean,
                    "symbol" => RemoteObjectType::Symbol,
                    "bigint" => RemoteObjectType::Bigint,
                    _ => RemoteObjectType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(RemoteObjectType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Runtime.type.RemoteObject/properties/subtype。
///
///协议原文：
///
///<pre>Object subtype hint. Specified for `object` type values only.
///NOTE: If you change anything here, make sure to also update
///`subtype` in `ObjectPreview` and `PropertyPreview` below.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RemoteObjectSubtype {
    ///协议值 "array"。
    Array,
    ///协议值 "null"。
    Null,
    ///协议值 "node"。
    Node,
    ///协议值 "regexp"。
    Regexp,
    ///协议值 "date"。
    Date,
    ///协议值 "map"。
    Map,
    ///协议值 "set"。
    Set,
    ///协议值 "weakmap"。
    Weakmap,
    ///协议值 "weakset"。
    Weakset,
    ///协议值 "iterator"。
    Iterator,
    ///协议值 "generator"。
    Generator,
    ///协议值 "error"。
    Error,
    ///协议值 "proxy"。
    Proxy,
    ///协议值 "promise"。
    Promise,
    ///协议值 "typedarray"。
    Typedarray,
    ///协议值 "arraybuffer"。
    Arraybuffer,
    ///协议值 "dataview"。
    Dataview,
    ///协议值 "webassemblymemory"。
    Webassemblymemory,
    ///协议值 "wasmvalue"。
    Wasmvalue,
    ///协议值 "trustedtype"。
    Trustedtype,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl RemoteObjectSubtype {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::runtime::types::RemoteObjectSubtype::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Array => "array",
            Self::Null => "null",
            Self::Node => "node",
            Self::Regexp => "regexp",
            Self::Date => "date",
            Self::Map => "map",
            Self::Set => "set",
            Self::Weakmap => "weakmap",
            Self::Weakset => "weakset",
            Self::Iterator => "iterator",
            Self::Generator => "generator",
            Self::Error => "error",
            Self::Proxy => "proxy",
            Self::Promise => "promise",
            Self::Typedarray => "typedarray",
            Self::Arraybuffer => "arraybuffer",
            Self::Dataview => "dataview",
            Self::Webassemblymemory => "webassemblymemory",
            Self::Wasmvalue => "wasmvalue",
            Self::Trustedtype => "trustedtype",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for RemoteObjectSubtype {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for RemoteObjectSubtype {
    fn from(value: String) -> Self {
        match value.as_str() {
            "array" => RemoteObjectSubtype::Array,
            "null" => RemoteObjectSubtype::Null,
            "node" => RemoteObjectSubtype::Node,
            "regexp" => RemoteObjectSubtype::Regexp,
            "date" => RemoteObjectSubtype::Date,
            "map" => RemoteObjectSubtype::Map,
            "set" => RemoteObjectSubtype::Set,
            "weakmap" => RemoteObjectSubtype::Weakmap,
            "weakset" => RemoteObjectSubtype::Weakset,
            "iterator" => RemoteObjectSubtype::Iterator,
            "generator" => RemoteObjectSubtype::Generator,
            "error" => RemoteObjectSubtype::Error,
            "proxy" => RemoteObjectSubtype::Proxy,
            "promise" => RemoteObjectSubtype::Promise,
            "typedarray" => RemoteObjectSubtype::Typedarray,
            "arraybuffer" => RemoteObjectSubtype::Arraybuffer,
            "dataview" => RemoteObjectSubtype::Dataview,
            "webassemblymemory" => RemoteObjectSubtype::Webassemblymemory,
            "wasmvalue" => RemoteObjectSubtype::Wasmvalue,
            "trustedtype" => RemoteObjectSubtype::Trustedtype,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for RemoteObjectSubtype {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RemoteObjectSubtype {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoteObjectSubtype;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "array" => RemoteObjectSubtype::Array,
                    "null" => RemoteObjectSubtype::Null,
                    "node" => RemoteObjectSubtype::Node,
                    "regexp" => RemoteObjectSubtype::Regexp,
                    "date" => RemoteObjectSubtype::Date,
                    "map" => RemoteObjectSubtype::Map,
                    "set" => RemoteObjectSubtype::Set,
                    "weakmap" => RemoteObjectSubtype::Weakmap,
                    "weakset" => RemoteObjectSubtype::Weakset,
                    "iterator" => RemoteObjectSubtype::Iterator,
                    "generator" => RemoteObjectSubtype::Generator,
                    "error" => RemoteObjectSubtype::Error,
                    "proxy" => RemoteObjectSubtype::Proxy,
                    "promise" => RemoteObjectSubtype::Promise,
                    "typedarray" => RemoteObjectSubtype::Typedarray,
                    "arraybuffer" => RemoteObjectSubtype::Arraybuffer,
                    "dataview" => RemoteObjectSubtype::Dataview,
                    "webassemblymemory" => RemoteObjectSubtype::Webassemblymemory,
                    "wasmvalue" => RemoteObjectSubtype::Wasmvalue,
                    "trustedtype" => RemoteObjectSubtype::Trustedtype,
                    _ => RemoteObjectSubtype::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(RemoteObjectSubtype::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.type.RemoteObject。
///
///协议原文：
///
///<pre>Mirror object referencing original JavaScript object.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoteObject {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.RemoteObject/properties/type。
    ///
    ///协议原文：
    ///
    ///<pre>Object type.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: RemoteObjectType,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.RemoteObject/properties/subtype。
    ///
    ///协议原文：
    ///
    ///<pre>Object subtype hint. Specified for `object` type values only.
    ///NOTE: If you change anything here, make sure to also update
    ///`subtype` in `ObjectPreview` and `PropertyPreview` below.</pre>
    ///
    ///
    #[serde(rename = "subtype")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub subtype: Option<RemoteObjectSubtype>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.RemoteObject/properties/className。
    ///
    ///协议原文：
    ///
    ///<pre>Object class (constructor) name. Specified for `object` type values only.</pre>
    ///
    ///
    #[serde(rename = "className")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub class_name: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.RemoteObject/properties/value。
    ///
    ///协议原文：
    ///
    ///<pre>Remote object value in case of primitive values or JSON values (if it was requested).</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<serde_json::Value>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.RemoteObject/properties/unserializableValue。
    ///
    ///协议原文：
    ///
    ///<pre>Primitive value which can not be JSON-stringified does not have `value`, but gets this
    ///property.</pre>
    ///
    ///
    #[serde(rename = "unserializableValue")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub unserializable_value: Option<crate::runtime::types::UnserializableValue>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.RemoteObject/properties/description。
    ///
    ///协议原文：
    ///
    ///<pre>String representation of the object.</pre>
    ///
    ///
    #[serde(rename = "description")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.RemoteObject/properties/deepSerializedValue。
    ///
    ///协议原文：
    ///
    ///<pre>Deep serialized value.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "deepSerializedValue")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub deep_serialized_value: Option<crate::runtime::types::DeepSerializedValue>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.RemoteObject/properties/objectId。
    ///
    ///协议原文：
    ///
    ///<pre>Unique object identifier (for non-primitive values).</pre>
    ///
    ///
    #[serde(rename = "objectId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub object_id: Option<crate::runtime::types::RemoteObjectId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.RemoteObject/properties/preview。
    ///
    ///协议原文：
    ///
    ///<pre>Preview containing abbreviated property values. Specified for `object` type values only.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "preview")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub preview: Option<crate::runtime::types::ObjectPreview>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.RemoteObject/properties/customPreview。此项为 experimental，端点可能不支持。
    #[serde(rename = "customPreview")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_preview: Option<crate::runtime::types::CustomPreview>,
}
impl<'de> serde::Deserialize<'de> for RemoteObject {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoteObject;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.RemoteObject/properties/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Object type.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: RemoteObjectType,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.RemoteObject/properties/subtype。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Object subtype hint. Specified for `object` type values only.
                    ///NOTE: If you change anything here, make sure to also update
                    ///`subtype` in `ObjectPreview` and `PropertyPreview` below.</pre>
                    ///
                    ///
                    #[serde(rename = "subtype")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub subtype: Option<RemoteObjectSubtype>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.RemoteObject/properties/className。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Object class (constructor) name. Specified for `object` type values only.</pre>
                    ///
                    ///
                    #[serde(rename = "className")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub class_name: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.RemoteObject/properties/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Remote object value in case of primitive values or JSON values (if it was requested).</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub value: Option<serde_json::Value>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.RemoteObject/properties/unserializableValue。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Primitive value which can not be JSON-stringified does not have `value`, but gets this
                    ///property.</pre>
                    ///
                    ///
                    #[serde(rename = "unserializableValue")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub unserializable_value: Option<crate::runtime::types::UnserializableValue>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.RemoteObject/properties/description。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>String representation of the object.</pre>
                    ///
                    ///
                    #[serde(rename = "description")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub description: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.RemoteObject/properties/deepSerializedValue。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Deep serialized value.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "deepSerializedValue")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub deep_serialized_value: Option<crate::runtime::types::DeepSerializedValue>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.RemoteObject/properties/objectId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Unique object identifier (for non-primitive values).</pre>
                    ///
                    ///
                    #[serde(rename = "objectId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub object_id: Option<crate::runtime::types::RemoteObjectId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.RemoteObject/properties/preview。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Preview containing abbreviated property values. Specified for `object` type values only.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "preview")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub preview: Option<crate::runtime::types::ObjectPreview>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.RemoteObject/properties/customPreview。此项为 experimental，端点可能不支持。
                    #[serde(rename = "customPreview")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub custom_preview: Option<crate::runtime::types::CustomPreview>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RemoteObject {
                    r#type: fields.r#type,
                    subtype: fields.subtype,
                    class_name: fields.class_name,
                    value: fields.value,
                    unserializable_value: fields.unserializable_value,
                    description: fields.description,
                    deep_serialized_value: fields.deep_serialized_value,
                    object_id: fields.object_id,
                    preview: fields.preview,
                    custom_preview: fields.custom_preview,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Runtime.type.RemoteObjectId。
///
///协议原文：
///
///<pre>Unique object identifier.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct RemoteObjectId(String);
impl RemoteObjectId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::runtime::types::RemoteObjectId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::runtime::types::RemoteObjectId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::runtime::types::RemoteObjectId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::runtime::types::RemoteObjectId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::runtime::types::RemoteObjectId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Runtime.type.ScriptId。
///
///协议原文：
///
///<pre>Unique script identifier.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ScriptId(String);
impl ScriptId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::runtime::types::ScriptId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::runtime::types::ScriptId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::runtime::types::ScriptId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::runtime::types::ScriptId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::runtime::types::ScriptId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Runtime.type.SerializationOptions/properties/serialization。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SerializationOptionsSerialization {
    ///协议值 "deep"。
    Deep,
    ///协议值 "json"。
    Json,
    ///协议值 "idOnly"。
    IdOnly,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl SerializationOptionsSerialization {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::runtime::types::SerializationOptionsSerialization::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Deep => "deep",
            Self::Json => "json",
            Self::IdOnly => "idOnly",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for SerializationOptionsSerialization {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for SerializationOptionsSerialization {
    fn from(value: String) -> Self {
        match value.as_str() {
            "deep" => SerializationOptionsSerialization::Deep,
            "json" => SerializationOptionsSerialization::Json,
            "idOnly" => SerializationOptionsSerialization::IdOnly,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for SerializationOptionsSerialization {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SerializationOptionsSerialization {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SerializationOptionsSerialization;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "deep" => SerializationOptionsSerialization::Deep,
                    "json" => SerializationOptionsSerialization::Json,
                    "idOnly" => SerializationOptionsSerialization::IdOnly,
                    _ => SerializationOptionsSerialization::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(SerializationOptionsSerialization::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.type.SerializationOptions。
///
///协议原文：
///
///<pre>Represents options for serialization. Overrides `generatePreview` and `returnByValue`.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct SerializationOptions {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.SerializationOptions/properties/serialization。
    #[serde(rename = "serialization")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub serialization: SerializationOptionsSerialization,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.SerializationOptions/properties/maxDepth。
    ///
    ///协议原文：
    ///
    ///<pre>Deep serialization depth. Default is full depth. Respected only in `deep` serialization mode.</pre>
    ///
    ///
    #[serde(rename = "maxDepth")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_depth: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.SerializationOptions/properties/additionalParameters。
    ///
    ///协议原文：
    ///
    ///<pre>Embedder-specific parameters. For example if connected to V8 in Chrome these control DOM
    ///serialization via `maxNodeDepth: integer` and `includeShadowTree: "none" | "open" | "all"`.
    ///Values can be only of type string or integer.</pre>
    ///
    ///
    #[serde(rename = "additionalParameters")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_parameters: Option<std::collections::BTreeMap<String, serde_json::Value>>,
}
impl<'de> serde::Deserialize<'de> for SerializationOptions {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SerializationOptions;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.SerializationOptions/properties/serialization。
                    #[serde(rename = "serialization")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub serialization: SerializationOptionsSerialization,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.SerializationOptions/properties/maxDepth。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Deep serialization depth. Default is full depth. Respected only in `deep` serialization mode.</pre>
                    ///
                    ///
                    #[serde(rename = "maxDepth")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub max_depth: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.SerializationOptions/properties/additionalParameters。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Embedder-specific parameters. For example if connected to V8 in Chrome these control DOM
                    ///serialization via `maxNodeDepth: integer` and `includeShadowTree: "none" | "open" | "all"`.
                    ///Values can be only of type string or integer.</pre>
                    ///
                    ///
                    #[serde(rename = "additionalParameters")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub additional_parameters:
                        Option<std::collections::BTreeMap<String, serde_json::Value>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SerializationOptions {
                    serialization: fields.serialization,
                    max_depth: fields.max_depth,
                    additional_parameters: fields.additional_parameters,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.type.StackTrace。
///
///协议原文：
///
///<pre>Call frames for assertions or error messages.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct StackTrace {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.StackTrace/properties/description。
    ///
    ///协议原文：
    ///
    ///<pre>String label of this stack trace. For async traces this may be a name of the function that
    ///initiated the async call.</pre>
    ///
    ///
    #[serde(rename = "description")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.StackTrace/properties/callFrames。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript function name.</pre>
    ///
    ///
    #[serde(rename = "callFrames")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub call_frames: Vec<crate::runtime::types::CallFrame>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.StackTrace/properties/parent。
    ///
    ///协议原文：
    ///
    ///<pre>Asynchronous JavaScript stack trace that preceded this stack, if available.</pre>
    ///
    ///
    #[serde(rename = "parent")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent: Option<Box<crate::runtime::types::StackTrace>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.StackTrace/properties/parentId。
    ///
    ///协议原文：
    ///
    ///<pre>Asynchronous JavaScript stack trace that preceded this stack, if available.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "parentId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_id: Option<crate::runtime::types::StackTraceId>,
}
impl<'de> serde::Deserialize<'de> for StackTrace {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StackTrace;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.StackTrace/properties/description。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>String label of this stack trace. For async traces this may be a name of the function that
                    ///initiated the async call.</pre>
                    ///
                    ///
                    #[serde(rename = "description")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub description: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.StackTrace/properties/callFrames。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript function name.</pre>
                    ///
                    ///
                    #[serde(rename = "callFrames")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub call_frames: Vec<crate::runtime::types::CallFrame>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.StackTrace/properties/parent。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Asynchronous JavaScript stack trace that preceded this stack, if available.</pre>
                    ///
                    ///
                    #[serde(rename = "parent")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub parent: Option<Box<crate::runtime::types::StackTrace>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.StackTrace/properties/parentId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Asynchronous JavaScript stack trace that preceded this stack, if available.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "parentId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub parent_id: Option<crate::runtime::types::StackTraceId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StackTrace {
                    description: fields.description,
                    call_frames: fields.call_frames,
                    parent: fields.parent,
                    parent_id: fields.parent_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Runtime.type.StackTraceId。
///
///协议原文：
///
///<pre>If `debuggerId` is set stack trace comes from another debugger and can be resolved there. This
///allows to track cross-debugger calls. See `Runtime.StackTrace` and `Debugger.paused` for usages.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StackTraceId {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.StackTraceId/properties/id。
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.StackTraceId/properties/debuggerId。
    #[serde(rename = "debuggerId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub debugger_id: Option<crate::runtime::types::UniqueDebuggerId>,
}
impl<'de> serde::Deserialize<'de> for StackTraceId {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StackTraceId;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Runtime.type.StackTraceId/properties/id。
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Runtime.type.StackTraceId/properties/debuggerId。
                    #[serde(rename = "debuggerId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub debugger_id: Option<crate::runtime::types::UniqueDebuggerId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StackTraceId {
                    id: fields.id,
                    debugger_id: fields.debugger_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Runtime.type.TimeDelta。
///
///协议原文：
///
///<pre>Number of milliseconds.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct TimeDelta(ydp_core::FiniteF64);
impl TimeDelta {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::runtime::types::TimeDelta = serde_json::from_str("0.0").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::runtime::types::TimeDelta::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("0.0").unwrap());
    ///```
    pub fn new(value: ydp_core::FiniteF64) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::runtime::types::TimeDelta = serde_json::from_str("0.0").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::runtime::types::TimeDelta::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("0.0").unwrap());
    ///```
    pub fn into_inner(self) -> ydp_core::FiniteF64 {
        self.0
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Runtime.type.Timestamp。
///
///协议原文：
///
///<pre>Number of milliseconds since epoch.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Timestamp(ydp_core::FiniteF64);
impl Timestamp {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::runtime::types::Timestamp = serde_json::from_str("0.0").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::runtime::types::Timestamp::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("0.0").unwrap());
    ///```
    pub fn new(value: ydp_core::FiniteF64) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::runtime::types::Timestamp = serde_json::from_str("0.0").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::runtime::types::Timestamp::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("0.0").unwrap());
    ///```
    pub fn into_inner(self) -> ydp_core::FiniteF64 {
        self.0
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Runtime.type.UniqueDebuggerId。
///
///协议原文：
///
///<pre>Unique identifier of current debugger.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct UniqueDebuggerId(String);
impl UniqueDebuggerId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::runtime::types::UniqueDebuggerId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::runtime::types::UniqueDebuggerId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::runtime::types::UniqueDebuggerId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::runtime::types::UniqueDebuggerId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::runtime::types::UniqueDebuggerId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Runtime.type.UnserializableValue。
///
///协议原文：
///
///<pre>Primitive value which cannot be JSON-stringified. Includes values `-0`, `NaN`, `Infinity`,
///`-Infinity`, and bigint literals.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct UnserializableValue(String);
impl UnserializableValue {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::runtime::types::UnserializableValue = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::runtime::types::UnserializableValue::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::runtime::types::UnserializableValue = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::runtime::types::UnserializableValue::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::runtime::types::UnserializableValue::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
