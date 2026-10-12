///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.command.cancelDragging/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CancelDraggingParams {}
impl<'de> serde::Deserialize<'de> for CancelDraggingParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CancelDraggingParams;
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
                Ok(CancelDraggingParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Input.command.cancelDragging。
///
///协议原文：
///
///<pre>Cancels any active dragging in the page.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct CancelDragging;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.command.cancelDragging/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CancelDraggingResponse {}
impl<'de> serde::Deserialize<'de> for CancelDraggingResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CancelDraggingResponse;
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
                Ok(CancelDraggingResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for CancelDragging {
    type Params = CancelDraggingParams;
    type Response = CancelDraggingResponse;
    const METHOD: &'static str = "Input.cancelDragging";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Input.command.dispatchDragEvent/parameters/type。
///
///协议原文：
///
///<pre>Type of the drag event.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DispatchDragEventParamsType {
    ///协议值 "dragEnter"。
    DragEnter,
    ///协议值 "dragOver"。
    DragOver,
    ///协议值 "drop"。
    Drop,
    ///协议值 "dragCancel"。
    DragCancel,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl DispatchDragEventParamsType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::input::commands::DispatchDragEventParamsType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::DragEnter => "dragEnter",
            Self::DragOver => "dragOver",
            Self::Drop => "drop",
            Self::DragCancel => "dragCancel",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for DispatchDragEventParamsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for DispatchDragEventParamsType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "dragEnter" => DispatchDragEventParamsType::DragEnter,
            "dragOver" => DispatchDragEventParamsType::DragOver,
            "drop" => DispatchDragEventParamsType::Drop,
            "dragCancel" => DispatchDragEventParamsType::DragCancel,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for DispatchDragEventParamsType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DispatchDragEventParamsType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DispatchDragEventParamsType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "dragEnter" => DispatchDragEventParamsType::DragEnter,
                    "dragOver" => DispatchDragEventParamsType::DragOver,
                    "drop" => DispatchDragEventParamsType::Drop,
                    "dragCancel" => DispatchDragEventParamsType::DragCancel,
                    _ => DispatchDragEventParamsType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(DispatchDragEventParamsType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.command.dispatchDragEvent/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DispatchDragEventParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.dispatchDragEvent/parameters/type。
    ///
    ///协议原文：
    ///
    ///<pre>Type of the drag event.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: DispatchDragEventParamsType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.dispatchDragEvent/parameters/x。
    ///
    ///协议原文：
    ///
    ///<pre>X coordinate of the event relative to the main frame's viewport in CSS pixels.</pre>
    ///
    ///
    #[serde(rename = "x")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub x: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.dispatchDragEvent/parameters/y。
    ///
    ///协议原文：
    ///
    ///<pre>Y coordinate of the event relative to the main frame's viewport in CSS pixels. 0 refers to
    ///the top of the viewport and Y increases as it proceeds towards the bottom of the viewport.</pre>
    ///
    ///
    #[serde(rename = "y")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub y: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.dispatchDragEvent/parameters/data。
    #[serde(rename = "data")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub data: crate::input::types::DragData,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchDragEvent/parameters/modifiers。
    ///
    ///协议原文：
    ///
    ///<pre>Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    ///(default: 0).</pre>
    ///
    ///
    #[serde(rename = "modifiers")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub modifiers: Option<i64>,
}
impl<'de> serde::Deserialize<'de> for DispatchDragEventParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DispatchDragEventParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.dispatchDragEvent/parameters/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Type of the drag event.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: DispatchDragEventParamsType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.dispatchDragEvent/parameters/x。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>X coordinate of the event relative to the main frame's viewport in CSS pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "x")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub x: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.dispatchDragEvent/parameters/y。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Y coordinate of the event relative to the main frame's viewport in CSS pixels. 0 refers to
                    ///the top of the viewport and Y increases as it proceeds towards the bottom of the viewport.</pre>
                    ///
                    ///
                    #[serde(rename = "y")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub y: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.dispatchDragEvent/parameters/data。
                    #[serde(rename = "data")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub data: crate::input::types::DragData,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchDragEvent/parameters/modifiers。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
                    ///(default: 0).</pre>
                    ///
                    ///
                    #[serde(rename = "modifiers")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub modifiers: Option<i64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DispatchDragEventParams {
                    r#type: fields.r#type,
                    x: fields.x,
                    y: fields.y,
                    data: fields.data,
                    modifiers: fields.modifiers,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Input.command.dispatchDragEvent。
///
///协议原文：
///
///<pre>Dispatches a drag event into the page.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct DispatchDragEvent;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.command.dispatchDragEvent/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DispatchDragEventResponse {}
impl<'de> serde::Deserialize<'de> for DispatchDragEventResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DispatchDragEventResponse;
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
                Ok(DispatchDragEventResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for DispatchDragEvent {
    type Params = DispatchDragEventParams;
    type Response = DispatchDragEventResponse;
    const METHOD: &'static str = "Input.dispatchDragEvent";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Input.command.dispatchKeyEvent/parameters/type。
///
///协议原文：
///
///<pre>Type of the key event.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DispatchKeyEventParamsType {
    ///协议值 "keyDown"。
    KeyDown,
    ///协议值 "keyUp"。
    KeyUp,
    ///协议值 "rawKeyDown"。
    RawKeyDown,
    ///协议值 "char"。
    Char,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl DispatchKeyEventParamsType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::input::commands::DispatchKeyEventParamsType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::KeyDown => "keyDown",
            Self::KeyUp => "keyUp",
            Self::RawKeyDown => "rawKeyDown",
            Self::Char => "char",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for DispatchKeyEventParamsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for DispatchKeyEventParamsType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "keyDown" => DispatchKeyEventParamsType::KeyDown,
            "keyUp" => DispatchKeyEventParamsType::KeyUp,
            "rawKeyDown" => DispatchKeyEventParamsType::RawKeyDown,
            "char" => DispatchKeyEventParamsType::Char,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for DispatchKeyEventParamsType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DispatchKeyEventParamsType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DispatchKeyEventParamsType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "keyDown" => DispatchKeyEventParamsType::KeyDown,
                    "keyUp" => DispatchKeyEventParamsType::KeyUp,
                    "rawKeyDown" => DispatchKeyEventParamsType::RawKeyDown,
                    "char" => DispatchKeyEventParamsType::Char,
                    _ => DispatchKeyEventParamsType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(DispatchKeyEventParamsType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.command.dispatchKeyEvent/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DispatchKeyEventParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.dispatchKeyEvent/parameters/type。
    ///
    ///协议原文：
    ///
    ///<pre>Type of the key event.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: DispatchKeyEventParamsType,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchKeyEvent/parameters/modifiers。
    ///
    ///协议原文：
    ///
    ///<pre>Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    ///(default: 0).</pre>
    ///
    ///
    #[serde(rename = "modifiers")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub modifiers: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchKeyEvent/parameters/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Time at which the event occurred.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub timestamp: Option<crate::input::types::TimeSinceEpoch>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchKeyEvent/parameters/text。
    ///
    ///协议原文：
    ///
    ///<pre>Text as generated by processing a virtual key code with a keyboard layout. Not needed for
    ///for `keyUp` and `rawKeyDown` events (default: "")</pre>
    ///
    ///
    #[serde(rename = "text")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub text: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchKeyEvent/parameters/unmodifiedText。
    ///
    ///协议原文：
    ///
    ///<pre>Text that would have been generated by the keyboard if no modifiers were pressed (except for
    ///shift). Useful for shortcut (accelerator) key handling (default: "").</pre>
    ///
    ///
    #[serde(rename = "unmodifiedText")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub unmodified_text: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchKeyEvent/parameters/keyIdentifier。
    ///
    ///协议原文：
    ///
    ///<pre>Unique key identifier (e.g., 'U+0041') (default: "").</pre>
    ///
    ///
    #[serde(rename = "keyIdentifier")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub key_identifier: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchKeyEvent/parameters/code。
    ///
    ///协议原文：
    ///
    ///<pre>Unique DOM defined string value for each physical key (e.g., 'KeyA') (default: "").</pre>
    ///
    ///
    #[serde(rename = "code")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub code: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchKeyEvent/parameters/key。
    ///
    ///协议原文：
    ///
    ///<pre>Unique DOM defined string value describing the meaning of the key in the context of active
    ///modifiers, keyboard layout, etc (e.g., 'AltGr') (default: "").</pre>
    ///
    ///
    #[serde(rename = "key")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub key: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchKeyEvent/parameters/windowsVirtualKeyCode。
    ///
    ///协议原文：
    ///
    ///<pre>Windows virtual key code (default: 0).</pre>
    ///
    ///
    #[serde(rename = "windowsVirtualKeyCode")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub windows_virtual_key_code: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchKeyEvent/parameters/nativeVirtualKeyCode。
    ///
    ///协议原文：
    ///
    ///<pre>Native virtual key code (default: 0).</pre>
    ///
    ///
    #[serde(rename = "nativeVirtualKeyCode")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub native_virtual_key_code: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchKeyEvent/parameters/autoRepeat。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the event was generated from auto repeat (default: false).</pre>
    ///
    ///
    #[serde(rename = "autoRepeat")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_repeat: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchKeyEvent/parameters/isKeypad。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the event was generated from the keypad (default: false).</pre>
    ///
    ///
    #[serde(rename = "isKeypad")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_keypad: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchKeyEvent/parameters/isSystemKey。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the event was a system key event (default: false).</pre>
    ///
    ///
    #[serde(rename = "isSystemKey")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_system_key: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchKeyEvent/parameters/location。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the event was from the left or right side of the keyboard. 1=Left, 2=Right (default:
    ///0).</pre>
    ///
    ///
    #[serde(rename = "location")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub location: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchKeyEvent/parameters/commands。
    ///
    ///协议原文：
    ///
    ///<pre>Editing commands to send with the key event (e.g., 'selectAll') (default: []).
    ///These are related to but not equal the command names used in `document.execCommand` and NSStandardKeyBindingResponding.
    ///See https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/editing/commands/editor_command_names.h for valid command names.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "commands")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub commands: Option<Vec<String>>,
}
impl<'de> serde::Deserialize<'de> for DispatchKeyEventParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DispatchKeyEventParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.dispatchKeyEvent/parameters/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Type of the key event.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: DispatchKeyEventParamsType,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchKeyEvent/parameters/modifiers。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
                    ///(default: 0).</pre>
                    ///
                    ///
                    #[serde(rename = "modifiers")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub modifiers: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchKeyEvent/parameters/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Time at which the event occurred.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub timestamp: Option<crate::input::types::TimeSinceEpoch>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchKeyEvent/parameters/text。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Text as generated by processing a virtual key code with a keyboard layout. Not needed for
                    ///for `keyUp` and `rawKeyDown` events (default: "")</pre>
                    ///
                    ///
                    #[serde(rename = "text")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub text: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchKeyEvent/parameters/unmodifiedText。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Text that would have been generated by the keyboard if no modifiers were pressed (except for
                    ///shift). Useful for shortcut (accelerator) key handling (default: "").</pre>
                    ///
                    ///
                    #[serde(rename = "unmodifiedText")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub unmodified_text: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchKeyEvent/parameters/keyIdentifier。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Unique key identifier (e.g., 'U+0041') (default: "").</pre>
                    ///
                    ///
                    #[serde(rename = "keyIdentifier")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub key_identifier: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchKeyEvent/parameters/code。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Unique DOM defined string value for each physical key (e.g., 'KeyA') (default: "").</pre>
                    ///
                    ///
                    #[serde(rename = "code")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub code: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchKeyEvent/parameters/key。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Unique DOM defined string value describing the meaning of the key in the context of active
                    ///modifiers, keyboard layout, etc (e.g., 'AltGr') (default: "").</pre>
                    ///
                    ///
                    #[serde(rename = "key")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub key: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchKeyEvent/parameters/windowsVirtualKeyCode。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Windows virtual key code (default: 0).</pre>
                    ///
                    ///
                    #[serde(rename = "windowsVirtualKeyCode")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub windows_virtual_key_code: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchKeyEvent/parameters/nativeVirtualKeyCode。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Native virtual key code (default: 0).</pre>
                    ///
                    ///
                    #[serde(rename = "nativeVirtualKeyCode")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub native_virtual_key_code: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchKeyEvent/parameters/autoRepeat。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the event was generated from auto repeat (default: false).</pre>
                    ///
                    ///
                    #[serde(rename = "autoRepeat")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub auto_repeat: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchKeyEvent/parameters/isKeypad。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the event was generated from the keypad (default: false).</pre>
                    ///
                    ///
                    #[serde(rename = "isKeypad")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub is_keypad: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchKeyEvent/parameters/isSystemKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the event was a system key event (default: false).</pre>
                    ///
                    ///
                    #[serde(rename = "isSystemKey")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub is_system_key: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchKeyEvent/parameters/location。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the event was from the left or right side of the keyboard. 1=Left, 2=Right (default:
                    ///0).</pre>
                    ///
                    ///
                    #[serde(rename = "location")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub location: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchKeyEvent/parameters/commands。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Editing commands to send with the key event (e.g., 'selectAll') (default: []).
                    ///These are related to but not equal the command names used in `document.execCommand` and NSStandardKeyBindingResponding.
                    ///See https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/editing/commands/editor_command_names.h for valid command names.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "commands")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub commands: Option<Vec<String>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DispatchKeyEventParams {
                    r#type: fields.r#type,
                    modifiers: fields.modifiers,
                    timestamp: fields.timestamp,
                    text: fields.text,
                    unmodified_text: fields.unmodified_text,
                    key_identifier: fields.key_identifier,
                    code: fields.code,
                    key: fields.key,
                    windows_virtual_key_code: fields.windows_virtual_key_code,
                    native_virtual_key_code: fields.native_virtual_key_code,
                    auto_repeat: fields.auto_repeat,
                    is_keypad: fields.is_keypad,
                    is_system_key: fields.is_system_key,
                    location: fields.location,
                    commands: fields.commands,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Input.command.dispatchKeyEvent。
///
///协议原文：
///
///<pre>Dispatches a key event to the page.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct DispatchKeyEvent;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.command.dispatchKeyEvent/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DispatchKeyEventResponse {}
impl<'de> serde::Deserialize<'de> for DispatchKeyEventResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DispatchKeyEventResponse;
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
                Ok(DispatchKeyEventResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for DispatchKeyEvent {
    type Params = DispatchKeyEventParams;
    type Response = DispatchKeyEventResponse;
    const METHOD: &'static str = "Input.dispatchKeyEvent";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Input.command.dispatchMouseEvent/parameters/type。
///
///协议原文：
///
///<pre>Type of the mouse event.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DispatchMouseEventParamsType {
    ///协议值 "mousePressed"。
    MousePressed,
    ///协议值 "mouseReleased"。
    MouseReleased,
    ///协议值 "mouseMoved"。
    MouseMoved,
    ///协议值 "mouseWheel"。
    MouseWheel,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl DispatchMouseEventParamsType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::input::commands::DispatchMouseEventParamsType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::MousePressed => "mousePressed",
            Self::MouseReleased => "mouseReleased",
            Self::MouseMoved => "mouseMoved",
            Self::MouseWheel => "mouseWheel",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for DispatchMouseEventParamsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for DispatchMouseEventParamsType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "mousePressed" => DispatchMouseEventParamsType::MousePressed,
            "mouseReleased" => DispatchMouseEventParamsType::MouseReleased,
            "mouseMoved" => DispatchMouseEventParamsType::MouseMoved,
            "mouseWheel" => DispatchMouseEventParamsType::MouseWheel,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for DispatchMouseEventParamsType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DispatchMouseEventParamsType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DispatchMouseEventParamsType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "mousePressed" => DispatchMouseEventParamsType::MousePressed,
                    "mouseReleased" => DispatchMouseEventParamsType::MouseReleased,
                    "mouseMoved" => DispatchMouseEventParamsType::MouseMoved,
                    "mouseWheel" => DispatchMouseEventParamsType::MouseWheel,
                    _ => DispatchMouseEventParamsType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(DispatchMouseEventParamsType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Input.command.dispatchMouseEvent/parameters/pointerType。
///
///协议原文：
///
///<pre>Pointer type (default: "mouse").</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DispatchMouseEventParamsPointerType {
    ///协议值 "mouse"。
    Mouse,
    ///协议值 "pen"。
    Pen,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl DispatchMouseEventParamsPointerType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::input::commands::DispatchMouseEventParamsPointerType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Mouse => "mouse",
            Self::Pen => "pen",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for DispatchMouseEventParamsPointerType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for DispatchMouseEventParamsPointerType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "mouse" => DispatchMouseEventParamsPointerType::Mouse,
            "pen" => DispatchMouseEventParamsPointerType::Pen,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for DispatchMouseEventParamsPointerType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DispatchMouseEventParamsPointerType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DispatchMouseEventParamsPointerType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "mouse" => DispatchMouseEventParamsPointerType::Mouse,
                    "pen" => DispatchMouseEventParamsPointerType::Pen,
                    _ => DispatchMouseEventParamsPointerType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(DispatchMouseEventParamsPointerType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.command.dispatchMouseEvent/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DispatchMouseEventParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.dispatchMouseEvent/parameters/type。
    ///
    ///协议原文：
    ///
    ///<pre>Type of the mouse event.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: DispatchMouseEventParamsType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.dispatchMouseEvent/parameters/x。
    ///
    ///协议原文：
    ///
    ///<pre>X coordinate of the event relative to the main frame's viewport in CSS pixels.</pre>
    ///
    ///
    #[serde(rename = "x")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub x: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.dispatchMouseEvent/parameters/y。
    ///
    ///协议原文：
    ///
    ///<pre>Y coordinate of the event relative to the main frame's viewport in CSS pixels. 0 refers to
    ///the top of the viewport and Y increases as it proceeds towards the bottom of the viewport.</pre>
    ///
    ///
    #[serde(rename = "y")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub y: ydp_core::FiniteF64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchMouseEvent/parameters/modifiers。
    ///
    ///协议原文：
    ///
    ///<pre>Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    ///(default: 0).</pre>
    ///
    ///
    #[serde(rename = "modifiers")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub modifiers: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchMouseEvent/parameters/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Time at which the event occurred.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub timestamp: Option<crate::input::types::TimeSinceEpoch>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchMouseEvent/parameters/button。
    ///
    ///协议原文：
    ///
    ///<pre>Mouse button (default: "none").</pre>
    ///
    ///
    #[serde(rename = "button")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub button: Option<crate::input::types::MouseButton>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchMouseEvent/parameters/buttons。
    ///
    ///协议原文：
    ///
    ///<pre>A number indicating which buttons are pressed on the mouse when a mouse event is triggered.
    ///Left=1, Right=2, Middle=4, Back=8, Forward=16, None=0.</pre>
    ///
    ///
    #[serde(rename = "buttons")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub buttons: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchMouseEvent/parameters/clickCount。
    ///
    ///协议原文：
    ///
    ///<pre>Number of times the mouse button was clicked (default: 0).</pre>
    ///
    ///
    #[serde(rename = "clickCount")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub click_count: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchMouseEvent/parameters/force。
    ///
    ///协议原文：
    ///
    ///<pre>The normalized pressure, which has a range of [0,1] (default: 0).</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "force")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub force: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchMouseEvent/parameters/tangentialPressure。
    ///
    ///协议原文：
    ///
    ///<pre>The normalized tangential pressure, which has a range of [-1,1] (default: 0).</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "tangentialPressure")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub tangential_pressure: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchMouseEvent/parameters/tiltX。
    ///
    ///协议原文：
    ///
    ///<pre>The plane angle between the Y-Z plane and the plane containing both the stylus axis and the Y axis, in degrees of the range [-90,90], a positive tiltX is to the right (default: 0).</pre>
    ///
    ///
    #[serde(rename = "tiltX")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub tilt_x: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchMouseEvent/parameters/tiltY。
    ///
    ///协议原文：
    ///
    ///<pre>The plane angle between the X-Z plane and the plane containing both the stylus axis and the X axis, in degrees of the range [-90,90], a positive tiltY is towards the user (default: 0).</pre>
    ///
    ///
    #[serde(rename = "tiltY")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub tilt_y: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchMouseEvent/parameters/twist。
    ///
    ///协议原文：
    ///
    ///<pre>The clockwise rotation of a pen stylus around its own major axis, in degrees in the range [0,359] (default: 0).</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "twist")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub twist: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchMouseEvent/parameters/deltaX。
    ///
    ///协议原文：
    ///
    ///<pre>X delta in CSS pixels for mouse wheel event (default: 0).</pre>
    ///
    ///
    #[serde(rename = "deltaX")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub delta_x: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchMouseEvent/parameters/deltaY。
    ///
    ///协议原文：
    ///
    ///<pre>Y delta in CSS pixels for mouse wheel event (default: 0).</pre>
    ///
    ///
    #[serde(rename = "deltaY")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub delta_y: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchMouseEvent/parameters/pointerType。
    ///
    ///协议原文：
    ///
    ///<pre>Pointer type (default: "mouse").</pre>
    ///
    ///
    #[serde(rename = "pointerType")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub pointer_type: Option<DispatchMouseEventParamsPointerType>,
}
impl<'de> serde::Deserialize<'de> for DispatchMouseEventParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DispatchMouseEventParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.dispatchMouseEvent/parameters/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Type of the mouse event.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: DispatchMouseEventParamsType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.dispatchMouseEvent/parameters/x。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>X coordinate of the event relative to the main frame's viewport in CSS pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "x")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub x: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.dispatchMouseEvent/parameters/y。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Y coordinate of the event relative to the main frame's viewport in CSS pixels. 0 refers to
                    ///the top of the viewport and Y increases as it proceeds towards the bottom of the viewport.</pre>
                    ///
                    ///
                    #[serde(rename = "y")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub y: ydp_core::FiniteF64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchMouseEvent/parameters/modifiers。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
                    ///(default: 0).</pre>
                    ///
                    ///
                    #[serde(rename = "modifiers")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub modifiers: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchMouseEvent/parameters/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Time at which the event occurred.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub timestamp: Option<crate::input::types::TimeSinceEpoch>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchMouseEvent/parameters/button。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Mouse button (default: "none").</pre>
                    ///
                    ///
                    #[serde(rename = "button")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub button: Option<crate::input::types::MouseButton>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchMouseEvent/parameters/buttons。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A number indicating which buttons are pressed on the mouse when a mouse event is triggered.
                    ///Left=1, Right=2, Middle=4, Back=8, Forward=16, None=0.</pre>
                    ///
                    ///
                    #[serde(rename = "buttons")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub buttons: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchMouseEvent/parameters/clickCount。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Number of times the mouse button was clicked (default: 0).</pre>
                    ///
                    ///
                    #[serde(rename = "clickCount")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub click_count: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchMouseEvent/parameters/force。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The normalized pressure, which has a range of [0,1] (default: 0).</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "force")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub force: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchMouseEvent/parameters/tangentialPressure。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The normalized tangential pressure, which has a range of [-1,1] (default: 0).</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "tangentialPressure")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub tangential_pressure: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchMouseEvent/parameters/tiltX。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The plane angle between the Y-Z plane and the plane containing both the stylus axis and the Y axis, in degrees of the range [-90,90], a positive tiltX is to the right (default: 0).</pre>
                    ///
                    ///
                    #[serde(rename = "tiltX")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub tilt_x: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchMouseEvent/parameters/tiltY。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The plane angle between the X-Z plane and the plane containing both the stylus axis and the X axis, in degrees of the range [-90,90], a positive tiltY is towards the user (default: 0).</pre>
                    ///
                    ///
                    #[serde(rename = "tiltY")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub tilt_y: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchMouseEvent/parameters/twist。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The clockwise rotation of a pen stylus around its own major axis, in degrees in the range [0,359] (default: 0).</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "twist")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub twist: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchMouseEvent/parameters/deltaX。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>X delta in CSS pixels for mouse wheel event (default: 0).</pre>
                    ///
                    ///
                    #[serde(rename = "deltaX")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub delta_x: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchMouseEvent/parameters/deltaY。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Y delta in CSS pixels for mouse wheel event (default: 0).</pre>
                    ///
                    ///
                    #[serde(rename = "deltaY")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub delta_y: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchMouseEvent/parameters/pointerType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Pointer type (default: "mouse").</pre>
                    ///
                    ///
                    #[serde(rename = "pointerType")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub pointer_type: Option<DispatchMouseEventParamsPointerType>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DispatchMouseEventParams {
                    r#type: fields.r#type,
                    x: fields.x,
                    y: fields.y,
                    modifiers: fields.modifiers,
                    timestamp: fields.timestamp,
                    button: fields.button,
                    buttons: fields.buttons,
                    click_count: fields.click_count,
                    force: fields.force,
                    tangential_pressure: fields.tangential_pressure,
                    tilt_x: fields.tilt_x,
                    tilt_y: fields.tilt_y,
                    twist: fields.twist,
                    delta_x: fields.delta_x,
                    delta_y: fields.delta_y,
                    pointer_type: fields.pointer_type,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Input.command.dispatchMouseEvent。
///
///协议原文：
///
///<pre>Dispatches a mouse event to the page.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct DispatchMouseEvent;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.command.dispatchMouseEvent/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DispatchMouseEventResponse {}
impl<'de> serde::Deserialize<'de> for DispatchMouseEventResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DispatchMouseEventResponse;
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
                Ok(DispatchMouseEventResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for DispatchMouseEvent {
    type Params = DispatchMouseEventParams;
    type Response = DispatchMouseEventResponse;
    const METHOD: &'static str = "Input.dispatchMouseEvent";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Input.command.dispatchTouchEvent/parameters/type。
///
///协议原文：
///
///<pre>Type of the touch event. TouchEnd and TouchCancel must not contain any touch points, while
///TouchStart and TouchMove must contains at least one.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DispatchTouchEventParamsType {
    ///协议值 "touchStart"。
    TouchStart,
    ///协议值 "touchEnd"。
    TouchEnd,
    ///协议值 "touchMove"。
    TouchMove,
    ///协议值 "touchCancel"。
    TouchCancel,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl DispatchTouchEventParamsType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::input::commands::DispatchTouchEventParamsType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::TouchStart => "touchStart",
            Self::TouchEnd => "touchEnd",
            Self::TouchMove => "touchMove",
            Self::TouchCancel => "touchCancel",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for DispatchTouchEventParamsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for DispatchTouchEventParamsType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "touchStart" => DispatchTouchEventParamsType::TouchStart,
            "touchEnd" => DispatchTouchEventParamsType::TouchEnd,
            "touchMove" => DispatchTouchEventParamsType::TouchMove,
            "touchCancel" => DispatchTouchEventParamsType::TouchCancel,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for DispatchTouchEventParamsType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DispatchTouchEventParamsType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DispatchTouchEventParamsType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "touchStart" => DispatchTouchEventParamsType::TouchStart,
                    "touchEnd" => DispatchTouchEventParamsType::TouchEnd,
                    "touchMove" => DispatchTouchEventParamsType::TouchMove,
                    "touchCancel" => DispatchTouchEventParamsType::TouchCancel,
                    _ => DispatchTouchEventParamsType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(DispatchTouchEventParamsType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.command.dispatchTouchEvent/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DispatchTouchEventParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.dispatchTouchEvent/parameters/type。
    ///
    ///协议原文：
    ///
    ///<pre>Type of the touch event. TouchEnd and TouchCancel must not contain any touch points, while
    ///TouchStart and TouchMove must contains at least one.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: DispatchTouchEventParamsType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.dispatchTouchEvent/parameters/touchPoints。
    ///
    ///协议原文：
    ///
    ///<pre>Active touch points on the touch device. One event per any changed point (compared to
    ///previous touch event in a sequence) is generated, emulating pressing/moving/releasing points
    ///one by one.</pre>
    ///
    ///
    #[serde(rename = "touchPoints")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub touch_points: Vec<crate::input::types::TouchPoint>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchTouchEvent/parameters/modifiers。
    ///
    ///协议原文：
    ///
    ///<pre>Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    ///(default: 0).</pre>
    ///
    ///
    #[serde(rename = "modifiers")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub modifiers: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchTouchEvent/parameters/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Time at which the event occurred.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub timestamp: Option<crate::input::types::TimeSinceEpoch>,
}
impl<'de> serde::Deserialize<'de> for DispatchTouchEventParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DispatchTouchEventParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.dispatchTouchEvent/parameters/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Type of the touch event. TouchEnd and TouchCancel must not contain any touch points, while
                    ///TouchStart and TouchMove must contains at least one.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: DispatchTouchEventParamsType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.dispatchTouchEvent/parameters/touchPoints。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Active touch points on the touch device. One event per any changed point (compared to
                    ///previous touch event in a sequence) is generated, emulating pressing/moving/releasing points
                    ///one by one.</pre>
                    ///
                    ///
                    #[serde(rename = "touchPoints")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub touch_points: Vec<crate::input::types::TouchPoint>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchTouchEvent/parameters/modifiers。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
                    ///(default: 0).</pre>
                    ///
                    ///
                    #[serde(rename = "modifiers")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub modifiers: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.dispatchTouchEvent/parameters/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Time at which the event occurred.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub timestamp: Option<crate::input::types::TimeSinceEpoch>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DispatchTouchEventParams {
                    r#type: fields.r#type,
                    touch_points: fields.touch_points,
                    modifiers: fields.modifiers,
                    timestamp: fields.timestamp,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Input.command.dispatchTouchEvent。
///
///协议原文：
///
///<pre>Dispatches a touch event to the page.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct DispatchTouchEvent;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.command.dispatchTouchEvent/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DispatchTouchEventResponse {}
impl<'de> serde::Deserialize<'de> for DispatchTouchEventResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DispatchTouchEventResponse;
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
                Ok(DispatchTouchEventResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for DispatchTouchEvent {
    type Params = DispatchTouchEventParams;
    type Response = DispatchTouchEventResponse;
    const METHOD: &'static str = "Input.dispatchTouchEvent";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Input.command.emulateTouchFromMouseEvent/parameters/type。
///
///协议原文：
///
///<pre>Type of the mouse event.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum EmulateTouchFromMouseEventParamsType {
    ///协议值 "mousePressed"。
    MousePressed,
    ///协议值 "mouseReleased"。
    MouseReleased,
    ///协议值 "mouseMoved"。
    MouseMoved,
    ///协议值 "mouseWheel"。
    MouseWheel,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl EmulateTouchFromMouseEventParamsType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::input::commands::EmulateTouchFromMouseEventParamsType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::MousePressed => "mousePressed",
            Self::MouseReleased => "mouseReleased",
            Self::MouseMoved => "mouseMoved",
            Self::MouseWheel => "mouseWheel",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for EmulateTouchFromMouseEventParamsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for EmulateTouchFromMouseEventParamsType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "mousePressed" => EmulateTouchFromMouseEventParamsType::MousePressed,
            "mouseReleased" => EmulateTouchFromMouseEventParamsType::MouseReleased,
            "mouseMoved" => EmulateTouchFromMouseEventParamsType::MouseMoved,
            "mouseWheel" => EmulateTouchFromMouseEventParamsType::MouseWheel,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for EmulateTouchFromMouseEventParamsType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for EmulateTouchFromMouseEventParamsType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EmulateTouchFromMouseEventParamsType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "mousePressed" => EmulateTouchFromMouseEventParamsType::MousePressed,
                    "mouseReleased" => EmulateTouchFromMouseEventParamsType::MouseReleased,
                    "mouseMoved" => EmulateTouchFromMouseEventParamsType::MouseMoved,
                    "mouseWheel" => EmulateTouchFromMouseEventParamsType::MouseWheel,
                    _ => EmulateTouchFromMouseEventParamsType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(EmulateTouchFromMouseEventParamsType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.command.emulateTouchFromMouseEvent/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EmulateTouchFromMouseEventParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.emulateTouchFromMouseEvent/parameters/type。
    ///
    ///协议原文：
    ///
    ///<pre>Type of the mouse event.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: EmulateTouchFromMouseEventParamsType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.emulateTouchFromMouseEvent/parameters/x。
    ///
    ///协议原文：
    ///
    ///<pre>X coordinate of the mouse pointer in DIP.</pre>
    ///
    ///
    #[serde(rename = "x")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub x: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.emulateTouchFromMouseEvent/parameters/y。
    ///
    ///协议原文：
    ///
    ///<pre>Y coordinate of the mouse pointer in DIP.</pre>
    ///
    ///
    #[serde(rename = "y")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub y: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.emulateTouchFromMouseEvent/parameters/button。
    ///
    ///协议原文：
    ///
    ///<pre>Mouse button. Only "none", "left", "right" are supported.</pre>
    ///
    ///
    #[serde(rename = "button")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub button: crate::input::types::MouseButton,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.emulateTouchFromMouseEvent/parameters/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Time at which the event occurred (default: current time).</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub timestamp: Option<crate::input::types::TimeSinceEpoch>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.emulateTouchFromMouseEvent/parameters/deltaX。
    ///
    ///协议原文：
    ///
    ///<pre>X delta in DIP for mouse wheel event (default: 0).</pre>
    ///
    ///
    #[serde(rename = "deltaX")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub delta_x: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.emulateTouchFromMouseEvent/parameters/deltaY。
    ///
    ///协议原文：
    ///
    ///<pre>Y delta in DIP for mouse wheel event (default: 0).</pre>
    ///
    ///
    #[serde(rename = "deltaY")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub delta_y: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.emulateTouchFromMouseEvent/parameters/modifiers。
    ///
    ///协议原文：
    ///
    ///<pre>Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
    ///(default: 0).</pre>
    ///
    ///
    #[serde(rename = "modifiers")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub modifiers: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.emulateTouchFromMouseEvent/parameters/clickCount。
    ///
    ///协议原文：
    ///
    ///<pre>Number of times the mouse button was clicked (default: 0).</pre>
    ///
    ///
    #[serde(rename = "clickCount")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub click_count: Option<i64>,
}
impl<'de> serde::Deserialize<'de> for EmulateTouchFromMouseEventParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EmulateTouchFromMouseEventParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.emulateTouchFromMouseEvent/parameters/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Type of the mouse event.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: EmulateTouchFromMouseEventParamsType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.emulateTouchFromMouseEvent/parameters/x。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>X coordinate of the mouse pointer in DIP.</pre>
                    ///
                    ///
                    #[serde(rename = "x")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub x: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.emulateTouchFromMouseEvent/parameters/y。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Y coordinate of the mouse pointer in DIP.</pre>
                    ///
                    ///
                    #[serde(rename = "y")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub y: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.emulateTouchFromMouseEvent/parameters/button。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Mouse button. Only "none", "left", "right" are supported.</pre>
                    ///
                    ///
                    #[serde(rename = "button")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub button: crate::input::types::MouseButton,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.emulateTouchFromMouseEvent/parameters/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Time at which the event occurred (default: current time).</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub timestamp: Option<crate::input::types::TimeSinceEpoch>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.emulateTouchFromMouseEvent/parameters/deltaX。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>X delta in DIP for mouse wheel event (default: 0).</pre>
                    ///
                    ///
                    #[serde(rename = "deltaX")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub delta_x: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.emulateTouchFromMouseEvent/parameters/deltaY。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Y delta in DIP for mouse wheel event (default: 0).</pre>
                    ///
                    ///
                    #[serde(rename = "deltaY")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub delta_y: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.emulateTouchFromMouseEvent/parameters/modifiers。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8
                    ///(default: 0).</pre>
                    ///
                    ///
                    #[serde(rename = "modifiers")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub modifiers: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.emulateTouchFromMouseEvent/parameters/clickCount。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Number of times the mouse button was clicked (default: 0).</pre>
                    ///
                    ///
                    #[serde(rename = "clickCount")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub click_count: Option<i64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EmulateTouchFromMouseEventParams {
                    r#type: fields.r#type,
                    x: fields.x,
                    y: fields.y,
                    button: fields.button,
                    timestamp: fields.timestamp,
                    delta_x: fields.delta_x,
                    delta_y: fields.delta_y,
                    modifiers: fields.modifiers,
                    click_count: fields.click_count,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Input.command.emulateTouchFromMouseEvent。
///
///协议原文：
///
///<pre>Emulates touch event from the mouse event parameters.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct EmulateTouchFromMouseEvent;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.command.emulateTouchFromMouseEvent/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EmulateTouchFromMouseEventResponse {}
impl<'de> serde::Deserialize<'de> for EmulateTouchFromMouseEventResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EmulateTouchFromMouseEventResponse;
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
                Ok(EmulateTouchFromMouseEventResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for EmulateTouchFromMouseEvent {
    type Params = EmulateTouchFromMouseEventParams;
    type Response = EmulateTouchFromMouseEventResponse;
    const METHOD: &'static str = "Input.emulateTouchFromMouseEvent";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.command.imeSetComposition/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ImeSetCompositionParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.imeSetComposition/parameters/text。
    ///
    ///协议原文：
    ///
    ///<pre>The text to insert</pre>
    ///
    ///
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.imeSetComposition/parameters/selectionStart。
    ///
    ///协议原文：
    ///
    ///<pre>selection start</pre>
    ///
    ///
    #[serde(rename = "selectionStart")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub selection_start: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.imeSetComposition/parameters/selectionEnd。
    ///
    ///协议原文：
    ///
    ///<pre>selection end</pre>
    ///
    ///
    #[serde(rename = "selectionEnd")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub selection_end: i64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.imeSetComposition/parameters/replacementStart。
    ///
    ///协议原文：
    ///
    ///<pre>replacement start</pre>
    ///
    ///
    #[serde(rename = "replacementStart")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub replacement_start: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.imeSetComposition/parameters/replacementEnd。
    ///
    ///协议原文：
    ///
    ///<pre>replacement end</pre>
    ///
    ///
    #[serde(rename = "replacementEnd")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub replacement_end: Option<i64>,
}
impl<'de> serde::Deserialize<'de> for ImeSetCompositionParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImeSetCompositionParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.imeSetComposition/parameters/text。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The text to insert</pre>
                    ///
                    ///
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.imeSetComposition/parameters/selectionStart。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>selection start</pre>
                    ///
                    ///
                    #[serde(rename = "selectionStart")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub selection_start: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.imeSetComposition/parameters/selectionEnd。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>selection end</pre>
                    ///
                    ///
                    #[serde(rename = "selectionEnd")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub selection_end: i64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.imeSetComposition/parameters/replacementStart。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>replacement start</pre>
                    ///
                    ///
                    #[serde(rename = "replacementStart")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub replacement_start: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.imeSetComposition/parameters/replacementEnd。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>replacement end</pre>
                    ///
                    ///
                    #[serde(rename = "replacementEnd")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub replacement_end: Option<i64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ImeSetCompositionParams {
                    text: fields.text,
                    selection_start: fields.selection_start,
                    selection_end: fields.selection_end,
                    replacement_start: fields.replacement_start,
                    replacement_end: fields.replacement_end,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Input.command.imeSetComposition。
///
///协议原文：
///
///<pre>This method sets the current candidate text for IME.
///Use imeCommitComposition to commit the final text.
///Use imeSetComposition with empty string as text to cancel composition.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ImeSetComposition;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.command.imeSetComposition/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ImeSetCompositionResponse {}
impl<'de> serde::Deserialize<'de> for ImeSetCompositionResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImeSetCompositionResponse;
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
                Ok(ImeSetCompositionResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ImeSetComposition {
    type Params = ImeSetCompositionParams;
    type Response = ImeSetCompositionResponse;
    const METHOD: &'static str = "Input.imeSetComposition";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.command.insertText/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct InsertTextParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.insertText/parameters/text。
    ///
    ///协议原文：
    ///
    ///<pre>The text to insert.</pre>
    ///
    ///
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: String,
}
impl<'de> serde::Deserialize<'de> for InsertTextParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InsertTextParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.insertText/parameters/text。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The text to insert.</pre>
                    ///
                    ///
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(InsertTextParams { text: fields.text })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Input.command.insertText。
///
///协议原文：
///
///<pre>This method emulates inserting text that doesn't come from a key press,
///for example an emoji keyboard or an IME.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct InsertText;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.command.insertText/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct InsertTextResponse {}
impl<'de> serde::Deserialize<'de> for InsertTextResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InsertTextResponse;
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
                Ok(InsertTextResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for InsertText {
    type Params = InsertTextParams;
    type Response = InsertTextResponse;
    const METHOD: &'static str = "Input.insertText";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.command.setIgnoreInputEvents/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetIgnoreInputEventsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.setIgnoreInputEvents/parameters/ignore。
    ///
    ///协议原文：
    ///
    ///<pre>Ignores input events processing when set to true.</pre>
    ///
    ///
    #[serde(rename = "ignore")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub ignore: bool,
}
impl<'de> serde::Deserialize<'de> for SetIgnoreInputEventsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetIgnoreInputEventsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.setIgnoreInputEvents/parameters/ignore。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Ignores input events processing when set to true.</pre>
                    ///
                    ///
                    #[serde(rename = "ignore")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub ignore: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetIgnoreInputEventsParams {
                    ignore: fields.ignore,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Input.command.setIgnoreInputEvents。
///
///协议原文：
///
///<pre>Ignores input events (useful while auditing page).</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetIgnoreInputEvents;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.command.setIgnoreInputEvents/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetIgnoreInputEventsResponse {}
impl<'de> serde::Deserialize<'de> for SetIgnoreInputEventsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetIgnoreInputEventsResponse;
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
                Ok(SetIgnoreInputEventsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetIgnoreInputEvents {
    type Params = SetIgnoreInputEventsParams;
    type Response = SetIgnoreInputEventsResponse;
    const METHOD: &'static str = "Input.setIgnoreInputEvents";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.command.setInterceptDrags/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetInterceptDragsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.setInterceptDrags/parameters/enabled。
    #[serde(rename = "enabled")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub enabled: bool,
}
impl<'de> serde::Deserialize<'de> for SetInterceptDragsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetInterceptDragsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.setInterceptDrags/parameters/enabled。
                    #[serde(rename = "enabled")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub enabled: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetInterceptDragsParams {
                    enabled: fields.enabled,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Input.command.setInterceptDrags。
///
///协议原文：
///
///<pre>Prevents default drag and drop behavior and instead emits `Input.dragIntercepted` events.
///Drag and drop behavior can be directly controlled via `Input.dispatchDragEvent`.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetInterceptDrags;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.command.setInterceptDrags/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetInterceptDragsResponse {}
impl<'de> serde::Deserialize<'de> for SetInterceptDragsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetInterceptDragsResponse;
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
                Ok(SetInterceptDragsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetInterceptDrags {
    type Params = SetInterceptDragsParams;
    type Response = SetInterceptDragsResponse;
    const METHOD: &'static str = "Input.setInterceptDrags";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.command.synthesizePinchGesture/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SynthesizePinchGestureParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.synthesizePinchGesture/parameters/x。
    ///
    ///协议原文：
    ///
    ///<pre>X coordinate of the start of the gesture in CSS pixels.</pre>
    ///
    ///
    #[serde(rename = "x")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub x: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.synthesizePinchGesture/parameters/y。
    ///
    ///协议原文：
    ///
    ///<pre>Y coordinate of the start of the gesture in CSS pixels.</pre>
    ///
    ///
    #[serde(rename = "y")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub y: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.synthesizePinchGesture/parameters/scaleFactor。
    ///
    ///协议原文：
    ///
    ///<pre>Relative scale factor after zooming (&gt;1.0 zooms in, &lt;1.0 zooms out).</pre>
    ///
    ///
    #[serde(rename = "scaleFactor")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub scale_factor: ydp_core::FiniteF64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizePinchGesture/parameters/relativeSpeed。
    ///
    ///协议原文：
    ///
    ///<pre>Relative pointer speed in pixels per second (default: 800).</pre>
    ///
    ///
    #[serde(rename = "relativeSpeed")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_speed: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizePinchGesture/parameters/gestureSourceType。
    ///
    ///协议原文：
    ///
    ///<pre>Which type of input events to be generated (default: 'default', which queries the platform
    ///for the preferred input type).</pre>
    ///
    ///
    #[serde(rename = "gestureSourceType")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub gesture_source_type: Option<crate::input::types::GestureSourceType>,
}
impl<'de> serde::Deserialize<'de> for SynthesizePinchGestureParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SynthesizePinchGestureParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.synthesizePinchGesture/parameters/x。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>X coordinate of the start of the gesture in CSS pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "x")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub x: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.synthesizePinchGesture/parameters/y。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Y coordinate of the start of the gesture in CSS pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "y")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub y: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.synthesizePinchGesture/parameters/scaleFactor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Relative scale factor after zooming (&gt;1.0 zooms in, &lt;1.0 zooms out).</pre>
                    ///
                    ///
                    #[serde(rename = "scaleFactor")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub scale_factor: ydp_core::FiniteF64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizePinchGesture/parameters/relativeSpeed。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Relative pointer speed in pixels per second (default: 800).</pre>
                    ///
                    ///
                    #[serde(rename = "relativeSpeed")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub relative_speed: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizePinchGesture/parameters/gestureSourceType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Which type of input events to be generated (default: 'default', which queries the platform
                    ///for the preferred input type).</pre>
                    ///
                    ///
                    #[serde(rename = "gestureSourceType")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub gesture_source_type: Option<crate::input::types::GestureSourceType>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SynthesizePinchGestureParams {
                    x: fields.x,
                    y: fields.y,
                    scale_factor: fields.scale_factor,
                    relative_speed: fields.relative_speed,
                    gesture_source_type: fields.gesture_source_type,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Input.command.synthesizePinchGesture。
///
///协议原文：
///
///<pre>Synthesizes a pinch gesture over a time period by issuing appropriate touch events.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SynthesizePinchGesture;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.command.synthesizePinchGesture/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SynthesizePinchGestureResponse {}
impl<'de> serde::Deserialize<'de> for SynthesizePinchGestureResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SynthesizePinchGestureResponse;
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
                Ok(SynthesizePinchGestureResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SynthesizePinchGesture {
    type Params = SynthesizePinchGestureParams;
    type Response = SynthesizePinchGestureResponse;
    const METHOD: &'static str = "Input.synthesizePinchGesture";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.command.synthesizeScrollGesture/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SynthesizeScrollGestureParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.synthesizeScrollGesture/parameters/x。
    ///
    ///协议原文：
    ///
    ///<pre>X coordinate of the start of the gesture in CSS pixels.</pre>
    ///
    ///
    #[serde(rename = "x")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub x: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.synthesizeScrollGesture/parameters/y。
    ///
    ///协议原文：
    ///
    ///<pre>Y coordinate of the start of the gesture in CSS pixels.</pre>
    ///
    ///
    #[serde(rename = "y")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub y: ydp_core::FiniteF64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizeScrollGesture/parameters/xDistance。
    ///
    ///协议原文：
    ///
    ///<pre>The distance to scroll along the X axis (positive to scroll left).</pre>
    ///
    ///
    #[serde(rename = "xDistance")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub x_distance: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizeScrollGesture/parameters/yDistance。
    ///
    ///协议原文：
    ///
    ///<pre>The distance to scroll along the Y axis (positive to scroll up).</pre>
    ///
    ///
    #[serde(rename = "yDistance")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub y_distance: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizeScrollGesture/parameters/xOverscroll。
    ///
    ///协议原文：
    ///
    ///<pre>The number of additional pixels to scroll back along the X axis, in addition to the given
    ///distance.</pre>
    ///
    ///
    #[serde(rename = "xOverscroll")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub x_overscroll: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizeScrollGesture/parameters/yOverscroll。
    ///
    ///协议原文：
    ///
    ///<pre>The number of additional pixels to scroll back along the Y axis, in addition to the given
    ///distance.</pre>
    ///
    ///
    #[serde(rename = "yOverscroll")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub y_overscroll: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizeScrollGesture/parameters/preventFling。
    ///
    ///协议原文：
    ///
    ///<pre>Prevent fling (default: true).</pre>
    ///
    ///
    #[serde(rename = "preventFling")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub prevent_fling: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizeScrollGesture/parameters/speed。
    ///
    ///协议原文：
    ///
    ///<pre>Swipe speed in pixels per second (default: 800).</pre>
    ///
    ///
    #[serde(rename = "speed")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub speed: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizeScrollGesture/parameters/gestureSourceType。
    ///
    ///协议原文：
    ///
    ///<pre>Which type of input events to be generated (default: 'default', which queries the platform
    ///for the preferred input type).</pre>
    ///
    ///
    #[serde(rename = "gestureSourceType")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub gesture_source_type: Option<crate::input::types::GestureSourceType>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizeScrollGesture/parameters/repeatCount。
    ///
    ///协议原文：
    ///
    ///<pre>The number of times to repeat the gesture (default: 0).</pre>
    ///
    ///
    #[serde(rename = "repeatCount")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub repeat_count: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizeScrollGesture/parameters/repeatDelayMs。
    ///
    ///协议原文：
    ///
    ///<pre>The number of milliseconds delay between each repeat. (default: 250).</pre>
    ///
    ///
    #[serde(rename = "repeatDelayMs")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub repeat_delay_ms: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizeScrollGesture/parameters/interactionMarkerName。
    ///
    ///协议原文：
    ///
    ///<pre>The name of the interaction markers to generate, if not empty (default: "").</pre>
    ///
    ///
    #[serde(rename = "interactionMarkerName")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub interaction_marker_name: Option<String>,
}
impl<'de> serde::Deserialize<'de> for SynthesizeScrollGestureParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SynthesizeScrollGestureParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.synthesizeScrollGesture/parameters/x。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>X coordinate of the start of the gesture in CSS pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "x")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub x: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.synthesizeScrollGesture/parameters/y。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Y coordinate of the start of the gesture in CSS pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "y")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub y: ydp_core::FiniteF64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizeScrollGesture/parameters/xDistance。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The distance to scroll along the X axis (positive to scroll left).</pre>
                    ///
                    ///
                    #[serde(rename = "xDistance")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub x_distance: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizeScrollGesture/parameters/yDistance。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The distance to scroll along the Y axis (positive to scroll up).</pre>
                    ///
                    ///
                    #[serde(rename = "yDistance")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub y_distance: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizeScrollGesture/parameters/xOverscroll。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The number of additional pixels to scroll back along the X axis, in addition to the given
                    ///distance.</pre>
                    ///
                    ///
                    #[serde(rename = "xOverscroll")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub x_overscroll: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizeScrollGesture/parameters/yOverscroll。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The number of additional pixels to scroll back along the Y axis, in addition to the given
                    ///distance.</pre>
                    ///
                    ///
                    #[serde(rename = "yOverscroll")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub y_overscroll: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizeScrollGesture/parameters/preventFling。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Prevent fling (default: true).</pre>
                    ///
                    ///
                    #[serde(rename = "preventFling")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub prevent_fling: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizeScrollGesture/parameters/speed。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Swipe speed in pixels per second (default: 800).</pre>
                    ///
                    ///
                    #[serde(rename = "speed")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub speed: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizeScrollGesture/parameters/gestureSourceType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Which type of input events to be generated (default: 'default', which queries the platform
                    ///for the preferred input type).</pre>
                    ///
                    ///
                    #[serde(rename = "gestureSourceType")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub gesture_source_type: Option<crate::input::types::GestureSourceType>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizeScrollGesture/parameters/repeatCount。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The number of times to repeat the gesture (default: 0).</pre>
                    ///
                    ///
                    #[serde(rename = "repeatCount")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub repeat_count: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizeScrollGesture/parameters/repeatDelayMs。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The number of milliseconds delay between each repeat. (default: 250).</pre>
                    ///
                    ///
                    #[serde(rename = "repeatDelayMs")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub repeat_delay_ms: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizeScrollGesture/parameters/interactionMarkerName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The name of the interaction markers to generate, if not empty (default: "").</pre>
                    ///
                    ///
                    #[serde(rename = "interactionMarkerName")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub interaction_marker_name: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SynthesizeScrollGestureParams {
                    x: fields.x,
                    y: fields.y,
                    x_distance: fields.x_distance,
                    y_distance: fields.y_distance,
                    x_overscroll: fields.x_overscroll,
                    y_overscroll: fields.y_overscroll,
                    prevent_fling: fields.prevent_fling,
                    speed: fields.speed,
                    gesture_source_type: fields.gesture_source_type,
                    repeat_count: fields.repeat_count,
                    repeat_delay_ms: fields.repeat_delay_ms,
                    interaction_marker_name: fields.interaction_marker_name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Input.command.synthesizeScrollGesture。
///
///协议原文：
///
///<pre>Synthesizes a scroll gesture over a time period by issuing appropriate touch events.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SynthesizeScrollGesture;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.command.synthesizeScrollGesture/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SynthesizeScrollGestureResponse {}
impl<'de> serde::Deserialize<'de> for SynthesizeScrollGestureResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SynthesizeScrollGestureResponse;
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
                Ok(SynthesizeScrollGestureResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SynthesizeScrollGesture {
    type Params = SynthesizeScrollGestureParams;
    type Response = SynthesizeScrollGestureResponse;
    const METHOD: &'static str = "Input.synthesizeScrollGesture";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.command.synthesizeTapGesture/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SynthesizeTapGestureParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.synthesizeTapGesture/parameters/x。
    ///
    ///协议原文：
    ///
    ///<pre>X coordinate of the start of the gesture in CSS pixels.</pre>
    ///
    ///
    #[serde(rename = "x")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub x: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.synthesizeTapGesture/parameters/y。
    ///
    ///协议原文：
    ///
    ///<pre>Y coordinate of the start of the gesture in CSS pixels.</pre>
    ///
    ///
    #[serde(rename = "y")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub y: ydp_core::FiniteF64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizeTapGesture/parameters/duration。
    ///
    ///协议原文：
    ///
    ///<pre>Duration between touchdown and touchup events in ms (default: 50).</pre>
    ///
    ///
    #[serde(rename = "duration")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub duration: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizeTapGesture/parameters/tapCount。
    ///
    ///协议原文：
    ///
    ///<pre>Number of times to perform the tap (e.g. 2 for double tap, default: 1).</pre>
    ///
    ///
    #[serde(rename = "tapCount")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub tap_count: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizeTapGesture/parameters/gestureSourceType。
    ///
    ///协议原文：
    ///
    ///<pre>Which type of input events to be generated (default: 'default', which queries the platform
    ///for the preferred input type).</pre>
    ///
    ///
    #[serde(rename = "gestureSourceType")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub gesture_source_type: Option<crate::input::types::GestureSourceType>,
}
impl<'de> serde::Deserialize<'de> for SynthesizeTapGestureParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SynthesizeTapGestureParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.synthesizeTapGesture/parameters/x。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>X coordinate of the start of the gesture in CSS pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "x")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub x: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.command.synthesizeTapGesture/parameters/y。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Y coordinate of the start of the gesture in CSS pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "y")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub y: ydp_core::FiniteF64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizeTapGesture/parameters/duration。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Duration between touchdown and touchup events in ms (default: 50).</pre>
                    ///
                    ///
                    #[serde(rename = "duration")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub duration: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizeTapGesture/parameters/tapCount。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Number of times to perform the tap (e.g. 2 for double tap, default: 1).</pre>
                    ///
                    ///
                    #[serde(rename = "tapCount")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub tap_count: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Input.command.synthesizeTapGesture/parameters/gestureSourceType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Which type of input events to be generated (default: 'default', which queries the platform
                    ///for the preferred input type).</pre>
                    ///
                    ///
                    #[serde(rename = "gestureSourceType")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub gesture_source_type: Option<crate::input::types::GestureSourceType>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SynthesizeTapGestureParams {
                    x: fields.x,
                    y: fields.y,
                    duration: fields.duration,
                    tap_count: fields.tap_count,
                    gesture_source_type: fields.gesture_source_type,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Input.command.synthesizeTapGesture。
///
///协议原文：
///
///<pre>Synthesizes a tap gesture over a time period by issuing appropriate touch events.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SynthesizeTapGesture;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.command.synthesizeTapGesture/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SynthesizeTapGestureResponse {}
impl<'de> serde::Deserialize<'de> for SynthesizeTapGestureResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SynthesizeTapGestureResponse;
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
                Ok(SynthesizeTapGestureResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SynthesizeTapGesture {
    type Params = SynthesizeTapGestureParams;
    type Response = SynthesizeTapGestureResponse;
    const METHOD: &'static str = "Input.synthesizeTapGesture";
}
