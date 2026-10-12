///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Tethering.event.accepted/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AcceptedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Tethering.event.accepted/parameters/port。
    ///
    ///协议原文：
    ///
    ///<pre>Port number that was successfully bound.</pre>
    ///
    ///
    #[serde(rename = "port")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub port: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Tethering.event.accepted/parameters/connectionId。
    ///
    ///协议原文：
    ///
    ///<pre>Connection id to be used.</pre>
    ///
    ///
    #[serde(rename = "connectionId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub connection_id: String,
}
impl<'de> serde::Deserialize<'de> for AcceptedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AcceptedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Tethering.event.accepted/parameters/port。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Port number that was successfully bound.</pre>
                    ///
                    ///
                    #[serde(rename = "port")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub port: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Tethering.event.accepted/parameters/connectionId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Connection id to be used.</pre>
                    ///
                    ///
                    #[serde(rename = "connectionId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub connection_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AcceptedParams {
                    port: fields.port,
                    connection_id: fields.connection_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Tethering.event.accepted。
///
///协议原文：
///
///<pre>Informs that port was successfully bound and got a specified connection id.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Accepted;
impl ydp_core::Event for Accepted {
    type Params = AcceptedParams;
    const METHOD: &'static str = "Tethering.accepted";
}
