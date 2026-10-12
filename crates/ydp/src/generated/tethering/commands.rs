///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Tethering.command.bind/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct BindParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Tethering.command.bind/parameters/port。
    ///
    ///协议原文：
    ///
    ///<pre>Port number to bind.</pre>
    ///
    ///
    #[serde(rename = "port")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub port: i64,
}
impl<'de> serde::Deserialize<'de> for BindParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BindParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Tethering.command.bind/parameters/port。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Port number to bind.</pre>
                    ///
                    ///
                    #[serde(rename = "port")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub port: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(BindParams { port: fields.port })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Tethering.command.bind。
///
///协议原文：
///
///<pre>Request browser port binding.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Bind;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Tethering.command.bind/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct BindResponse {}
impl<'de> serde::Deserialize<'de> for BindResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BindResponse;
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
                Ok(BindResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Bind {
    type Params = BindParams;
    type Response = BindResponse;
    const METHOD: &'static str = "Tethering.bind";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Tethering.command.unbind/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct UnbindParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Tethering.command.unbind/parameters/port。
    ///
    ///协议原文：
    ///
    ///<pre>Port number to unbind.</pre>
    ///
    ///
    #[serde(rename = "port")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub port: i64,
}
impl<'de> serde::Deserialize<'de> for UnbindParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UnbindParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Tethering.command.unbind/parameters/port。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Port number to unbind.</pre>
                    ///
                    ///
                    #[serde(rename = "port")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub port: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(UnbindParams { port: fields.port })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Tethering.command.unbind。
///
///协议原文：
///
///<pre>Request browser port unbinding.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Unbind;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Tethering.command.unbind/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct UnbindResponse {}
impl<'de> serde::Deserialize<'de> for UnbindResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UnbindResponse;
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
                Ok(UnbindResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Unbind {
    type Params = UnbindParams;
    type Response = UnbindResponse;
    const METHOD: &'static str = "Tethering.unbind";
}
