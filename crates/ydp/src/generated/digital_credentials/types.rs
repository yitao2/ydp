///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：DigitalCredentials.type.VirtualWalletAction。
///
///协议原文：
///
///<pre>The type of virtual wallet action.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum VirtualWalletAction {
    ///协议值 "respond"。
    Respond,
    ///协议值 "decline"。
    Decline,
    ///协议值 "wait"。
    Wait,
    ///协议值 "clear"。
    Clear,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl VirtualWalletAction {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::digital_credentials::types::VirtualWalletAction::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Respond => "respond",
            Self::Decline => "decline",
            Self::Wait => "wait",
            Self::Clear => "clear",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for VirtualWalletAction {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for VirtualWalletAction {
    fn from(value: String) -> Self {
        match value.as_str() {
            "respond" => VirtualWalletAction::Respond,
            "decline" => VirtualWalletAction::Decline,
            "wait" => VirtualWalletAction::Wait,
            "clear" => VirtualWalletAction::Clear,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for VirtualWalletAction {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for VirtualWalletAction {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = VirtualWalletAction;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "respond" => VirtualWalletAction::Respond,
                    "decline" => VirtualWalletAction::Decline,
                    "wait" => VirtualWalletAction::Wait,
                    "clear" => VirtualWalletAction::Clear,
                    _ => VirtualWalletAction::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(VirtualWalletAction::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
