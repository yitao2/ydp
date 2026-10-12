///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Input.event.dragIntercepted/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DragInterceptedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.event.dragIntercepted/parameters/data。
    #[serde(rename = "data")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub data: crate::input::types::DragData,
}
impl<'de> serde::Deserialize<'de> for DragInterceptedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DragInterceptedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Input.event.dragIntercepted/parameters/data。
                    #[serde(rename = "data")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub data: crate::input::types::DragData,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DragInterceptedParams { data: fields.data })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Input.event.dragIntercepted。
///
///协议原文：
///
///<pre>Emitted only when `Input.setInterceptDrags` is enabled. Use this data with `Input.dispatchDragEvent` to
///restore normal drag and drop behavior.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct DragIntercepted;
impl ydp_core::Event for DragIntercepted {
    type Params = DragInterceptedParams;
    const METHOD: &'static str = "Input.dragIntercepted";
}
