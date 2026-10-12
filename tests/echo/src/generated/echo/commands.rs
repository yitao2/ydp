///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Echo.command.echo/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EchoParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Echo.command.echo/parameters/text。
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: String,
}
impl<'de> serde::Deserialize<'de> for EchoParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EchoParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Echo.command.echo/parameters/text。
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EchoParams { text: fields.text })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Echo.command.echo。
#[derive(Debug, Clone, Copy)]
pub struct Echo;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Echo.command.echo/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EchoResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Echo.command.echo/returns/text。
    #[serde(rename = "text")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub text: String,
}
impl<'de> serde::Deserialize<'de> for EchoResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EchoResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Echo.command.echo/returns/text。
                    #[serde(rename = "text")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub text: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EchoResponse { text: fields.text })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Echo {
    type Params = EchoParams;
    type Response = EchoResponse;
    const METHOD: &'static str = "Echo.echo";
}
