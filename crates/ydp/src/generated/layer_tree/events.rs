///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：LayerTree.event.layerPainted/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct LayerPaintedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.event.layerPainted/parameters/layerId。
    ///
    ///协议原文：
    ///
    ///<pre>The id of the painted layer.</pre>
    ///
    ///
    #[serde(rename = "layerId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub layer_id: crate::layer_tree::types::LayerId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.event.layerPainted/parameters/clip。
    ///
    ///协议原文：
    ///
    ///<pre>Clip rectangle.</pre>
    ///
    ///
    #[serde(rename = "clip")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub clip: crate::dom::types::Rect,
}
impl<'de> serde::Deserialize<'de> for LayerPaintedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LayerPaintedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.event.layerPainted/parameters/layerId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The id of the painted layer.</pre>
                    ///
                    ///
                    #[serde(rename = "layerId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub layer_id: crate::layer_tree::types::LayerId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.event.layerPainted/parameters/clip。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Clip rectangle.</pre>
                    ///
                    ///
                    #[serde(rename = "clip")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub clip: crate::dom::types::Rect,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LayerPaintedParams {
                    layer_id: fields.layer_id,
                    clip: fields.clip,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：LayerTree.event.layerPainted。
#[derive(Debug, Clone, Copy)]
pub struct LayerPainted;
impl ydp_core::Event for LayerPainted {
    type Params = LayerPaintedParams;
    const METHOD: &'static str = "LayerTree.layerPainted";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：LayerTree.event.layerTreeDidChange/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct LayerTreeDidChangeParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.event.layerTreeDidChange/parameters/layers。
    ///
    ///协议原文：
    ///
    ///<pre>Layer tree, absent if not in the compositing mode.</pre>
    ///
    ///
    #[serde(rename = "layers")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub layers: Option<Vec<crate::layer_tree::types::Layer>>,
}
impl<'de> serde::Deserialize<'de> for LayerTreeDidChangeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LayerTreeDidChangeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.event.layerTreeDidChange/parameters/layers。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Layer tree, absent if not in the compositing mode.</pre>
                    ///
                    ///
                    #[serde(rename = "layers")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub layers: Option<Vec<crate::layer_tree::types::Layer>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LayerTreeDidChangeParams {
                    layers: fields.layers,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：LayerTree.event.layerTreeDidChange。
#[derive(Debug, Clone, Copy)]
pub struct LayerTreeDidChange;
impl ydp_core::Event for LayerTreeDidChange {
    type Params = LayerTreeDidChangeParams;
    const METHOD: &'static str = "LayerTree.layerTreeDidChange";
}
