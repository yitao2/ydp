///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Console.event.messageAdded/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct MessageAddedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Console.event.messageAdded/parameters/message。
    ///
    ///协议原文：
    ///
    ///<pre>Console message that has been added.</pre>
    ///
    ///
    #[serde(rename = "message")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub message: crate::console::types::ConsoleMessage,
}
impl<'de> serde::Deserialize<'de> for MessageAddedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MessageAddedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Console.event.messageAdded/parameters/message。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Console message that has been added.</pre>
                    ///
                    ///
                    #[serde(rename = "message")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub message: crate::console::types::ConsoleMessage,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(MessageAddedParams {
                    message: fields.message,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Console.event.messageAdded。
///
///协议原文：
///
///<pre>Issued when new console message is added.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct MessageAdded;
impl ydp_core::Event for MessageAdded {
    type Params = MessageAddedParams;
    const METHOD: &'static str = "Console.messageAdded";
}
