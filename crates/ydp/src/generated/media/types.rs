///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Media.type.Player。
#[derive(Debug, Clone, serde::Serialize)]
pub struct Player {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.type.Player/properties/playerId。
    #[serde(rename = "playerId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub player_id: crate::media::types::PlayerId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Media.type.Player/properties/domNodeId。
    #[serde(rename = "domNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub dom_node_id: Option<crate::dom::types::BackendNodeId>,
}
impl<'de> serde::Deserialize<'de> for Player {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Player;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.type.Player/properties/playerId。
                    #[serde(rename = "playerId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub player_id: crate::media::types::PlayerId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Media.type.Player/properties/domNodeId。
                    #[serde(rename = "domNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub dom_node_id: Option<crate::dom::types::BackendNodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Player {
                    player_id: fields.player_id,
                    dom_node_id: fields.dom_node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Media.type.PlayerError。
///
///协议原文：
///
///<pre>Corresponds to kMediaError</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct PlayerError {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.type.PlayerError/properties/errorType。
    #[serde(rename = "errorType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub error_type: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.type.PlayerError/properties/code。
    ///
    ///协议原文：
    ///
    ///<pre>Code is the numeric enum entry for a specific set of error codes, such
    ///as PipelineStatusCodes in media/base/pipeline_status.h</pre>
    ///
    ///
    #[serde(rename = "code")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub code: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.type.PlayerError/properties/stack。
    ///
    ///协议原文：
    ///
    ///<pre>A trace of where this error was caused / where it passed through.</pre>
    ///
    ///
    #[serde(rename = "stack")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub stack: Vec<crate::media::types::PlayerErrorSourceLocation>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.type.PlayerError/properties/cause。
    ///
    ///协议原文：
    ///
    ///<pre>Errors potentially have a root cause error, ie, a DecoderError might be
    ///caused by an WindowsError</pre>
    ///
    ///
    #[serde(rename = "cause")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cause: Vec<crate::media::types::PlayerError>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.type.PlayerError/properties/data。
    ///
    ///协议原文：
    ///
    ///<pre>Extra data attached to an error, such as an HRESULT, Video Codec, etc.</pre>
    ///
    ///
    #[serde(rename = "data")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub data: std::collections::BTreeMap<String, serde_json::Value>,
}
impl<'de> serde::Deserialize<'de> for PlayerError {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PlayerError;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.type.PlayerError/properties/errorType。
                    #[serde(rename = "errorType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub error_type: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.type.PlayerError/properties/code。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Code is the numeric enum entry for a specific set of error codes, such
                    ///as PipelineStatusCodes in media/base/pipeline_status.h</pre>
                    ///
                    ///
                    #[serde(rename = "code")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub code: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.type.PlayerError/properties/stack。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A trace of where this error was caused / where it passed through.</pre>
                    ///
                    ///
                    #[serde(rename = "stack")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub stack: Vec<crate::media::types::PlayerErrorSourceLocation>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.type.PlayerError/properties/cause。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Errors potentially have a root cause error, ie, a DecoderError might be
                    ///caused by an WindowsError</pre>
                    ///
                    ///
                    #[serde(rename = "cause")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cause: Vec<crate::media::types::PlayerError>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.type.PlayerError/properties/data。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Extra data attached to an error, such as an HRESULT, Video Codec, etc.</pre>
                    ///
                    ///
                    #[serde(rename = "data")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub data: std::collections::BTreeMap<String, serde_json::Value>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PlayerError {
                    error_type: fields.error_type,
                    code: fields.code,
                    stack: fields.stack,
                    cause: fields.cause,
                    data: fields.data,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Media.type.PlayerErrorSourceLocation。
///
///协议原文：
///
///<pre>Represents logged source line numbers reported in an error.
///NOTE: file and line are from chromium c++ implementation code, not js.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct PlayerErrorSourceLocation {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.type.PlayerErrorSourceLocation/properties/file。
    #[serde(rename = "file")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub file: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.type.PlayerErrorSourceLocation/properties/line。
    #[serde(rename = "line")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub line: i64,
}
impl<'de> serde::Deserialize<'de> for PlayerErrorSourceLocation {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PlayerErrorSourceLocation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.type.PlayerErrorSourceLocation/properties/file。
                    #[serde(rename = "file")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub file: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.type.PlayerErrorSourceLocation/properties/line。
                    #[serde(rename = "line")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub line: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PlayerErrorSourceLocation {
                    file: fields.file,
                    line: fields.line,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Media.type.PlayerEvent。
///
///协议原文：
///
///<pre>Corresponds to kMediaEventTriggered</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct PlayerEvent {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.type.PlayerEvent/properties/timestamp。
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::media::types::Timestamp,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.type.PlayerEvent/properties/value。
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: String,
}
impl<'de> serde::Deserialize<'de> for PlayerEvent {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PlayerEvent;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.type.PlayerEvent/properties/timestamp。
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::media::types::Timestamp,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.type.PlayerEvent/properties/value。
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PlayerEvent {
                    timestamp: fields.timestamp,
                    value: fields.value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Media.type.PlayerId。
///
///协议原文：
///
///<pre>Players will get an ID that is unique within the agent context.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct PlayerId(String);
impl PlayerId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::media::types::PlayerId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::media::types::PlayerId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::media::types::PlayerId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::media::types::PlayerId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::media::types::PlayerId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Media.type.PlayerMessage/properties/level。
///
///协议原文：
///
///<pre>Keep in sync with MediaLogMessageLevel
///We are currently keeping the message level 'error' separate from the
///PlayerError type because right now they represent different things,
///this one being a DVLOG(ERROR) style log message that gets printed
///based on what log level is selected in the UI, and the other is a
///representation of a media::PipelineStatus object. Soon however we're
///going to be moving away from using PipelineStatus for errors and
///introducing a new error type which should hopefully let us integrate
///the error log level into the PlayerError type.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PlayerMessageLevel {
    ///协议值 "error"。
    Error,
    ///协议值 "warning"。
    Warning,
    ///协议值 "info"。
    Info,
    ///协议值 "debug"。
    Debug,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl PlayerMessageLevel {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::media::types::PlayerMessageLevel::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
            Self::Debug => "debug",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for PlayerMessageLevel {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for PlayerMessageLevel {
    fn from(value: String) -> Self {
        match value.as_str() {
            "error" => PlayerMessageLevel::Error,
            "warning" => PlayerMessageLevel::Warning,
            "info" => PlayerMessageLevel::Info,
            "debug" => PlayerMessageLevel::Debug,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for PlayerMessageLevel {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PlayerMessageLevel {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PlayerMessageLevel;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "error" => PlayerMessageLevel::Error,
                    "warning" => PlayerMessageLevel::Warning,
                    "info" => PlayerMessageLevel::Info,
                    "debug" => PlayerMessageLevel::Debug,
                    _ => PlayerMessageLevel::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(PlayerMessageLevel::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Media.type.PlayerMessage。
///
///协议原文：
///
///<pre>Have one type per entry in MediaLogRecord::Type
///Corresponds to kMessage</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct PlayerMessage {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.type.PlayerMessage/properties/level。
    ///
    ///协议原文：
    ///
    ///<pre>Keep in sync with MediaLogMessageLevel
    ///We are currently keeping the message level 'error' separate from the
    ///PlayerError type because right now they represent different things,
    ///this one being a DVLOG(ERROR) style log message that gets printed
    ///based on what log level is selected in the UI, and the other is a
    ///representation of a media::PipelineStatus object. Soon however we're
    ///going to be moving away from using PipelineStatus for errors and
    ///introducing a new error type which should hopefully let us integrate
    ///the error log level into the PlayerError type.</pre>
    ///
    ///
    #[serde(rename = "level")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub level: PlayerMessageLevel,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.type.PlayerMessage/properties/message。
    #[serde(rename = "message")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub message: String,
}
impl<'de> serde::Deserialize<'de> for PlayerMessage {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PlayerMessage;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.type.PlayerMessage/properties/level。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Keep in sync with MediaLogMessageLevel
                    ///We are currently keeping the message level 'error' separate from the
                    ///PlayerError type because right now they represent different things,
                    ///this one being a DVLOG(ERROR) style log message that gets printed
                    ///based on what log level is selected in the UI, and the other is a
                    ///representation of a media::PipelineStatus object. Soon however we're
                    ///going to be moving away from using PipelineStatus for errors and
                    ///introducing a new error type which should hopefully let us integrate
                    ///the error log level into the PlayerError type.</pre>
                    ///
                    ///
                    #[serde(rename = "level")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub level: PlayerMessageLevel,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.type.PlayerMessage/properties/message。
                    #[serde(rename = "message")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub message: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PlayerMessage {
                    level: fields.level,
                    message: fields.message,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Media.type.PlayerProperty。
///
///协议原文：
///
///<pre>Corresponds to kMediaPropertyChange</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct PlayerProperty {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.type.PlayerProperty/properties/name。
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.type.PlayerProperty/properties/value。
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: String,
}
impl<'de> serde::Deserialize<'de> for PlayerProperty {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PlayerProperty;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.type.PlayerProperty/properties/name。
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Media.type.PlayerProperty/properties/value。
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PlayerProperty {
                    name: fields.name,
                    value: fields.value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Media.type.Timestamp。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Timestamp(ydp_core::FiniteF64);
impl Timestamp {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::media::types::Timestamp = serde_json::from_str("0.0").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::media::types::Timestamp::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("0.0").unwrap());
    ///```
    pub fn new(value: ydp_core::FiniteF64) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::media::types::Timestamp = serde_json::from_str("0.0").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::media::types::Timestamp::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("0.0").unwrap());
    ///```
    pub fn into_inner(self) -> ydp_core::FiniteF64 {
        self.0
    }
}
