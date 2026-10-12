///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：LayerTree.type.Layer。
///
///协议原文：
///
///<pre>Information about a compositing layer.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct Layer {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.type.Layer/properties/layerId。
    ///
    ///协议原文：
    ///
    ///<pre>The unique id for this layer.</pre>
    ///
    ///
    #[serde(rename = "layerId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub layer_id: crate::layer_tree::types::LayerId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.type.Layer/properties/parentLayerId。
    ///
    ///协议原文：
    ///
    ///<pre>The id of parent (not present for root).</pre>
    ///
    ///
    #[serde(rename = "parentLayerId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_layer_id: Option<crate::layer_tree::types::LayerId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.type.Layer/properties/backendNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>The backend id for the node associated with this layer.</pre>
    ///
    ///
    #[serde(rename = "backendNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.type.Layer/properties/offsetX。
    ///
    ///协议原文：
    ///
    ///<pre>Offset from parent layer, X coordinate.</pre>
    ///
    ///
    #[serde(rename = "offsetX")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub offset_x: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.type.Layer/properties/offsetY。
    ///
    ///协议原文：
    ///
    ///<pre>Offset from parent layer, Y coordinate.</pre>
    ///
    ///
    #[serde(rename = "offsetY")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub offset_y: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.type.Layer/properties/width。
    ///
    ///协议原文：
    ///
    ///<pre>Layer width.</pre>
    ///
    ///
    #[serde(rename = "width")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub width: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.type.Layer/properties/height。
    ///
    ///协议原文：
    ///
    ///<pre>Layer height.</pre>
    ///
    ///
    #[serde(rename = "height")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub height: ydp_core::FiniteF64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.type.Layer/properties/transform。
    ///
    ///协议原文：
    ///
    ///<pre>Transformation matrix for layer, default is identity matrix</pre>
    ///
    ///
    #[serde(rename = "transform")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub transform: Option<Vec<ydp_core::FiniteF64>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.type.Layer/properties/anchorX。
    ///
    ///协议原文：
    ///
    ///<pre>Transform anchor point X, absent if no transform specified</pre>
    ///
    ///
    #[serde(rename = "anchorX")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub anchor_x: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.type.Layer/properties/anchorY。
    ///
    ///协议原文：
    ///
    ///<pre>Transform anchor point Y, absent if no transform specified</pre>
    ///
    ///
    #[serde(rename = "anchorY")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub anchor_y: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.type.Layer/properties/anchorZ。
    ///
    ///协议原文：
    ///
    ///<pre>Transform anchor point Z, absent if no transform specified</pre>
    ///
    ///
    #[serde(rename = "anchorZ")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub anchor_z: Option<ydp_core::FiniteF64>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.type.Layer/properties/paintCount。
    ///
    ///协议原文：
    ///
    ///<pre>Indicates how many time this layer has painted.</pre>
    ///
    ///
    #[serde(rename = "paintCount")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub paint_count: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.type.Layer/properties/drawsContent。
    ///
    ///协议原文：
    ///
    ///<pre>Indicates whether this layer hosts any content, rather than being used for
    ///transform/scrolling purposes only.</pre>
    ///
    ///
    #[serde(rename = "drawsContent")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub draws_content: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.type.Layer/properties/invisible。
    ///
    ///协议原文：
    ///
    ///<pre>Set if layer is not visible.</pre>
    ///
    ///
    #[serde(rename = "invisible")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub invisible: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.type.Layer/properties/scrollRects。
    ///
    ///协议原文：
    ///
    ///<pre>Rectangles scrolling on main thread only.</pre>
    ///
    ///
    #[serde(rename = "scrollRects")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub scroll_rects: Option<Vec<crate::layer_tree::types::ScrollRect>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.type.Layer/properties/stickyPositionConstraint。
    ///
    ///协议原文：
    ///
    ///<pre>Sticky position constraint information</pre>
    ///
    ///
    #[serde(rename = "stickyPositionConstraint")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub sticky_position_constraint: Option<crate::layer_tree::types::StickyPositionConstraint>,
}
impl<'de> serde::Deserialize<'de> for Layer {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Layer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.type.Layer/properties/layerId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The unique id for this layer.</pre>
                    ///
                    ///
                    #[serde(rename = "layerId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub layer_id: crate::layer_tree::types::LayerId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.type.Layer/properties/parentLayerId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The id of parent (not present for root).</pre>
                    ///
                    ///
                    #[serde(rename = "parentLayerId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub parent_layer_id: Option<crate::layer_tree::types::LayerId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.type.Layer/properties/backendNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The backend id for the node associated with this layer.</pre>
                    ///
                    ///
                    #[serde(rename = "backendNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.type.Layer/properties/offsetX。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Offset from parent layer, X coordinate.</pre>
                    ///
                    ///
                    #[serde(rename = "offsetX")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub offset_x: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.type.Layer/properties/offsetY。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Offset from parent layer, Y coordinate.</pre>
                    ///
                    ///
                    #[serde(rename = "offsetY")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub offset_y: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.type.Layer/properties/width。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Layer width.</pre>
                    ///
                    ///
                    #[serde(rename = "width")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub width: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.type.Layer/properties/height。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Layer height.</pre>
                    ///
                    ///
                    #[serde(rename = "height")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub height: ydp_core::FiniteF64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.type.Layer/properties/transform。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Transformation matrix for layer, default is identity matrix</pre>
                    ///
                    ///
                    #[serde(rename = "transform")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub transform: Option<Vec<ydp_core::FiniteF64>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.type.Layer/properties/anchorX。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Transform anchor point X, absent if no transform specified</pre>
                    ///
                    ///
                    #[serde(rename = "anchorX")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub anchor_x: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.type.Layer/properties/anchorY。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Transform anchor point Y, absent if no transform specified</pre>
                    ///
                    ///
                    #[serde(rename = "anchorY")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub anchor_y: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.type.Layer/properties/anchorZ。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Transform anchor point Z, absent if no transform specified</pre>
                    ///
                    ///
                    #[serde(rename = "anchorZ")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub anchor_z: Option<ydp_core::FiniteF64>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.type.Layer/properties/paintCount。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Indicates how many time this layer has painted.</pre>
                    ///
                    ///
                    #[serde(rename = "paintCount")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub paint_count: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.type.Layer/properties/drawsContent。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Indicates whether this layer hosts any content, rather than being used for
                    ///transform/scrolling purposes only.</pre>
                    ///
                    ///
                    #[serde(rename = "drawsContent")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub draws_content: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.type.Layer/properties/invisible。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Set if layer is not visible.</pre>
                    ///
                    ///
                    #[serde(rename = "invisible")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub invisible: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.type.Layer/properties/scrollRects。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Rectangles scrolling on main thread only.</pre>
                    ///
                    ///
                    #[serde(rename = "scrollRects")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub scroll_rects: Option<Vec<crate::layer_tree::types::ScrollRect>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.type.Layer/properties/stickyPositionConstraint。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Sticky position constraint information</pre>
                    ///
                    ///
                    #[serde(rename = "stickyPositionConstraint")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub sticky_position_constraint:
                        Option<crate::layer_tree::types::StickyPositionConstraint>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Layer {
                    layer_id: fields.layer_id,
                    parent_layer_id: fields.parent_layer_id,
                    backend_node_id: fields.backend_node_id,
                    offset_x: fields.offset_x,
                    offset_y: fields.offset_y,
                    width: fields.width,
                    height: fields.height,
                    transform: fields.transform,
                    anchor_x: fields.anchor_x,
                    anchor_y: fields.anchor_y,
                    anchor_z: fields.anchor_z,
                    paint_count: fields.paint_count,
                    draws_content: fields.draws_content,
                    invisible: fields.invisible,
                    scroll_rects: fields.scroll_rects,
                    sticky_position_constraint: fields.sticky_position_constraint,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：LayerTree.type.LayerId。
///
///协议原文：
///
///<pre>Unique Layer identifier.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct LayerId(String);
impl LayerId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::layer_tree::types::LayerId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::layer_tree::types::LayerId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::layer_tree::types::LayerId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::layer_tree::types::LayerId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::layer_tree::types::LayerId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：LayerTree.type.PaintProfile。
///
///协议原文：
///
///<pre>Array of timings, one per paint step.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct PaintProfile(Vec<ydp_core::FiniteF64>);
impl PaintProfile {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::layer_tree::types::PaintProfile = serde_json::from_str("[]").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::layer_tree::types::PaintProfile::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("[]").unwrap());
    ///```
    pub fn new(value: Vec<ydp_core::FiniteF64>) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::layer_tree::types::PaintProfile = serde_json::from_str("[]").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::layer_tree::types::PaintProfile::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("[]").unwrap());
    ///```
    pub fn into_inner(self) -> Vec<ydp_core::FiniteF64> {
        self.0
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：LayerTree.type.PictureTile。
///
///协议原文：
///
///<pre>Serialized fragment of layer picture along with its offset within the layer.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct PictureTile {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.type.PictureTile/properties/x。
    ///
    ///协议原文：
    ///
    ///<pre>Offset from owning layer left boundary</pre>
    ///
    ///
    #[serde(rename = "x")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub x: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.type.PictureTile/properties/y。
    ///
    ///协议原文：
    ///
    ///<pre>Offset from owning layer top boundary</pre>
    ///
    ///
    #[serde(rename = "y")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub y: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.type.PictureTile/properties/picture。
    ///
    ///协议原文：
    ///
    ///<pre>Base64-encoded snapshot data. (Encoded as a base64 string when passed over JSON)</pre>
    ///
    ///
    #[serde(rename = "picture")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub picture: String,
}
impl<'de> serde::Deserialize<'de> for PictureTile {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PictureTile;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.type.PictureTile/properties/x。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Offset from owning layer left boundary</pre>
                    ///
                    ///
                    #[serde(rename = "x")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub x: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.type.PictureTile/properties/y。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Offset from owning layer top boundary</pre>
                    ///
                    ///
                    #[serde(rename = "y")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub y: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.type.PictureTile/properties/picture。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Base64-encoded snapshot data. (Encoded as a base64 string when passed over JSON)</pre>
                    ///
                    ///
                    #[serde(rename = "picture")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub picture: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PictureTile {
                    x: fields.x,
                    y: fields.y,
                    picture: fields.picture,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：LayerTree.type.ScrollRect/properties/type。
///
///协议原文：
///
///<pre>Reason for rectangle to force scrolling on the main thread</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ScrollRectType {
    ///协议值 "RepaintsOnScroll"。
    RepaintsOnScroll,
    ///协议值 "TouchEventHandler"。
    TouchEventHandler,
    ///协议值 "WheelEventHandler"。
    WheelEventHandler,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ScrollRectType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::layer_tree::types::ScrollRectType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::RepaintsOnScroll => "RepaintsOnScroll",
            Self::TouchEventHandler => "TouchEventHandler",
            Self::WheelEventHandler => "WheelEventHandler",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ScrollRectType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ScrollRectType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "RepaintsOnScroll" => ScrollRectType::RepaintsOnScroll,
            "TouchEventHandler" => ScrollRectType::TouchEventHandler,
            "WheelEventHandler" => ScrollRectType::WheelEventHandler,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ScrollRectType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ScrollRectType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScrollRectType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "RepaintsOnScroll" => ScrollRectType::RepaintsOnScroll,
                    "TouchEventHandler" => ScrollRectType::TouchEventHandler,
                    "WheelEventHandler" => ScrollRectType::WheelEventHandler,
                    _ => ScrollRectType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ScrollRectType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：LayerTree.type.ScrollRect。
///
///协议原文：
///
///<pre>Rectangle where scrolling happens on the main thread.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ScrollRect {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.type.ScrollRect/properties/rect。
    ///
    ///协议原文：
    ///
    ///<pre>Rectangle itself.</pre>
    ///
    ///
    #[serde(rename = "rect")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub rect: crate::dom::types::Rect,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.type.ScrollRect/properties/type。
    ///
    ///协议原文：
    ///
    ///<pre>Reason for rectangle to force scrolling on the main thread</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: ScrollRectType,
}
impl<'de> serde::Deserialize<'de> for ScrollRect {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScrollRect;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.type.ScrollRect/properties/rect。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Rectangle itself.</pre>
                    ///
                    ///
                    #[serde(rename = "rect")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub rect: crate::dom::types::Rect,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.type.ScrollRect/properties/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Reason for rectangle to force scrolling on the main thread</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: ScrollRectType,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ScrollRect {
                    rect: fields.rect,
                    r#type: fields.r#type,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：LayerTree.type.SnapshotId。
///
///协议原文：
///
///<pre>Unique snapshot identifier.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SnapshotId(String);
impl SnapshotId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::layer_tree::types::SnapshotId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::layer_tree::types::SnapshotId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::layer_tree::types::SnapshotId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::layer_tree::types::SnapshotId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::layer_tree::types::SnapshotId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：LayerTree.type.StickyPositionConstraint。
///
///协议原文：
///
///<pre>Sticky position constraints.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct StickyPositionConstraint {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.type.StickyPositionConstraint/properties/stickyBoxRect。
    ///
    ///协议原文：
    ///
    ///<pre>Layout rectangle of the sticky element before being shifted</pre>
    ///
    ///
    #[serde(rename = "stickyBoxRect")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub sticky_box_rect: crate::dom::types::Rect,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.type.StickyPositionConstraint/properties/containingBlockRect。
    ///
    ///协议原文：
    ///
    ///<pre>Layout rectangle of the containing block of the sticky element</pre>
    ///
    ///
    #[serde(rename = "containingBlockRect")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub containing_block_rect: crate::dom::types::Rect,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.type.StickyPositionConstraint/properties/nearestLayerShiftingStickyBox。
    ///
    ///协议原文：
    ///
    ///<pre>The nearest sticky layer that shifts the sticky box</pre>
    ///
    ///
    #[serde(rename = "nearestLayerShiftingStickyBox")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub nearest_layer_shifting_sticky_box: Option<crate::layer_tree::types::LayerId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.type.StickyPositionConstraint/properties/nearestLayerShiftingContainingBlock。
    ///
    ///协议原文：
    ///
    ///<pre>The nearest sticky layer that shifts the containing block</pre>
    ///
    ///
    #[serde(rename = "nearestLayerShiftingContainingBlock")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub nearest_layer_shifting_containing_block: Option<crate::layer_tree::types::LayerId>,
}
impl<'de> serde::Deserialize<'de> for StickyPositionConstraint {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StickyPositionConstraint;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.type.StickyPositionConstraint/properties/stickyBoxRect。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Layout rectangle of the sticky element before being shifted</pre>
                    ///
                    ///
                    #[serde(rename = "stickyBoxRect")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub sticky_box_rect: crate::dom::types::Rect,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.type.StickyPositionConstraint/properties/containingBlockRect。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Layout rectangle of the containing block of the sticky element</pre>
                    ///
                    ///
                    #[serde(rename = "containingBlockRect")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub containing_block_rect: crate::dom::types::Rect,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.type.StickyPositionConstraint/properties/nearestLayerShiftingStickyBox。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The nearest sticky layer that shifts the sticky box</pre>
                    ///
                    ///
                    #[serde(rename = "nearestLayerShiftingStickyBox")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub nearest_layer_shifting_sticky_box:
                        Option<crate::layer_tree::types::LayerId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.type.StickyPositionConstraint/properties/nearestLayerShiftingContainingBlock。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The nearest sticky layer that shifts the containing block</pre>
                    ///
                    ///
                    #[serde(rename = "nearestLayerShiftingContainingBlock")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub nearest_layer_shifting_containing_block:
                        Option<crate::layer_tree::types::LayerId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StickyPositionConstraint {
                    sticky_box_rect: fields.sticky_box_rect,
                    containing_block_rect: fields.containing_block_rect,
                    nearest_layer_shifting_sticky_box: fields.nearest_layer_shifting_sticky_box,
                    nearest_layer_shifting_containing_block: fields
                        .nearest_layer_shifting_containing_block,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
