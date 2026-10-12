///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：LayerTree.command.compositingReasons/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CompositingReasonsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.command.compositingReasons/parameters/layerId。
    ///
    ///协议原文：
    ///
    ///<pre>The id of the layer for which we want to get the reasons it was composited.</pre>
    ///
    ///
    #[serde(rename = "layerId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub layer_id: crate::layer_tree::types::LayerId,
}
impl<'de> serde::Deserialize<'de> for CompositingReasonsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CompositingReasonsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.command.compositingReasons/parameters/layerId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The id of the layer for which we want to get the reasons it was composited.</pre>
                    ///
                    ///
                    #[serde(rename = "layerId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub layer_id: crate::layer_tree::types::LayerId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CompositingReasonsParams {
                    layer_id: fields.layer_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：LayerTree.command.compositingReasons。
///
///协议原文：
///
///<pre>Provides the reasons why the given layer was composited.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct CompositingReasons;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：LayerTree.command.compositingReasons/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CompositingReasonsResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.command.compositingReasons/returns/compositingReasons。
    ///
    ///协议原文：
    ///
    ///<pre>A list of strings specifying reasons for the given layer to become composited.</pre>
    ///
    ///
    #[serde(rename = "compositingReasons")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub compositing_reasons: Vec<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.command.compositingReasons/returns/compositingReasonIds。
    ///
    ///协议原文：
    ///
    ///<pre>A list of strings specifying reason IDs for the given layer to become composited.</pre>
    ///
    ///
    #[serde(rename = "compositingReasonIds")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub compositing_reason_ids: Vec<String>,
}
impl<'de> serde::Deserialize<'de> for CompositingReasonsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CompositingReasonsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.command.compositingReasons/returns/compositingReasons。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A list of strings specifying reasons for the given layer to become composited.</pre>
                    ///
                    ///
                    #[serde(rename = "compositingReasons")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub compositing_reasons: Vec<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.command.compositingReasons/returns/compositingReasonIds。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A list of strings specifying reason IDs for the given layer to become composited.</pre>
                    ///
                    ///
                    #[serde(rename = "compositingReasonIds")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub compositing_reason_ids: Vec<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CompositingReasonsResponse {
                    compositing_reasons: fields.compositing_reasons,
                    compositing_reason_ids: fields.compositing_reason_ids,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for CompositingReasons {
    type Params = CompositingReasonsParams;
    type Response = CompositingReasonsResponse;
    const METHOD: &'static str = "LayerTree.compositingReasons";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：LayerTree.command.disable/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct DisableParams {}
impl<'de> serde::Deserialize<'de> for DisableParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DisableParams;
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
                Ok(DisableParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：LayerTree.command.disable。
///
///协议原文：
///
///<pre>Disables compositing tree inspection.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：LayerTree.command.disable/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DisableResponse {}
impl<'de> serde::Deserialize<'de> for DisableResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DisableResponse;
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
                Ok(DisableResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Disable {
    type Params = DisableParams;
    type Response = DisableResponse;
    const METHOD: &'static str = "LayerTree.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：LayerTree.command.enable/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct EnableParams {}
impl<'de> serde::Deserialize<'de> for EnableParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EnableParams;
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
                Ok(EnableParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：LayerTree.command.enable。
///
///协议原文：
///
///<pre>Enables compositing tree inspection.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：LayerTree.command.enable/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EnableResponse {}
impl<'de> serde::Deserialize<'de> for EnableResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EnableResponse;
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
                Ok(EnableResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Enable {
    type Params = EnableParams;
    type Response = EnableResponse;
    const METHOD: &'static str = "LayerTree.enable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：LayerTree.command.loadSnapshot/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct LoadSnapshotParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.command.loadSnapshot/parameters/tiles。
    ///
    ///协议原文：
    ///
    ///<pre>An array of tiles composing the snapshot.</pre>
    ///
    ///
    #[serde(rename = "tiles")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub tiles: Vec<crate::layer_tree::types::PictureTile>,
}
impl<'de> serde::Deserialize<'de> for LoadSnapshotParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LoadSnapshotParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.command.loadSnapshot/parameters/tiles。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An array of tiles composing the snapshot.</pre>
                    ///
                    ///
                    #[serde(rename = "tiles")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub tiles: Vec<crate::layer_tree::types::PictureTile>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LoadSnapshotParams {
                    tiles: fields.tiles,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：LayerTree.command.loadSnapshot。
///
///协议原文：
///
///<pre>Returns the snapshot identifier.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct LoadSnapshot;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：LayerTree.command.loadSnapshot/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct LoadSnapshotResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.command.loadSnapshot/returns/snapshotId。
    ///
    ///协议原文：
    ///
    ///<pre>The id of the snapshot.</pre>
    ///
    ///
    #[serde(rename = "snapshotId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub snapshot_id: crate::layer_tree::types::SnapshotId,
}
impl<'de> serde::Deserialize<'de> for LoadSnapshotResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LoadSnapshotResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.command.loadSnapshot/returns/snapshotId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The id of the snapshot.</pre>
                    ///
                    ///
                    #[serde(rename = "snapshotId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub snapshot_id: crate::layer_tree::types::SnapshotId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LoadSnapshotResponse {
                    snapshot_id: fields.snapshot_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for LoadSnapshot {
    type Params = LoadSnapshotParams;
    type Response = LoadSnapshotResponse;
    const METHOD: &'static str = "LayerTree.loadSnapshot";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：LayerTree.command.makeSnapshot/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct MakeSnapshotParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.command.makeSnapshot/parameters/layerId。
    ///
    ///协议原文：
    ///
    ///<pre>The id of the layer.</pre>
    ///
    ///
    #[serde(rename = "layerId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub layer_id: crate::layer_tree::types::LayerId,
}
impl<'de> serde::Deserialize<'de> for MakeSnapshotParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MakeSnapshotParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.command.makeSnapshot/parameters/layerId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The id of the layer.</pre>
                    ///
                    ///
                    #[serde(rename = "layerId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub layer_id: crate::layer_tree::types::LayerId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(MakeSnapshotParams {
                    layer_id: fields.layer_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：LayerTree.command.makeSnapshot。
///
///协议原文：
///
///<pre>Returns the layer snapshot identifier.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct MakeSnapshot;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：LayerTree.command.makeSnapshot/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct MakeSnapshotResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.command.makeSnapshot/returns/snapshotId。
    ///
    ///协议原文：
    ///
    ///<pre>The id of the layer snapshot.</pre>
    ///
    ///
    #[serde(rename = "snapshotId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub snapshot_id: crate::layer_tree::types::SnapshotId,
}
impl<'de> serde::Deserialize<'de> for MakeSnapshotResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MakeSnapshotResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.command.makeSnapshot/returns/snapshotId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The id of the layer snapshot.</pre>
                    ///
                    ///
                    #[serde(rename = "snapshotId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub snapshot_id: crate::layer_tree::types::SnapshotId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(MakeSnapshotResponse {
                    snapshot_id: fields.snapshot_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for MakeSnapshot {
    type Params = MakeSnapshotParams;
    type Response = MakeSnapshotResponse;
    const METHOD: &'static str = "LayerTree.makeSnapshot";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：LayerTree.command.profileSnapshot/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProfileSnapshotParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.command.profileSnapshot/parameters/snapshotId。
    ///
    ///协议原文：
    ///
    ///<pre>The id of the layer snapshot.</pre>
    ///
    ///
    #[serde(rename = "snapshotId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub snapshot_id: crate::layer_tree::types::SnapshotId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.command.profileSnapshot/parameters/minRepeatCount。
    ///
    ///协议原文：
    ///
    ///<pre>The maximum number of times to replay the snapshot (1, if not specified).</pre>
    ///
    ///
    #[serde(rename = "minRepeatCount")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_repeat_count: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.command.profileSnapshot/parameters/minDuration。
    ///
    ///协议原文：
    ///
    ///<pre>The minimum duration (in seconds) to replay the snapshot.</pre>
    ///
    ///
    #[serde(rename = "minDuration")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_duration: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.command.profileSnapshot/parameters/clipRect。
    ///
    ///协议原文：
    ///
    ///<pre>The clip rectangle to apply when replaying the snapshot.</pre>
    ///
    ///
    #[serde(rename = "clipRect")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub clip_rect: Option<crate::dom::types::Rect>,
}
impl<'de> serde::Deserialize<'de> for ProfileSnapshotParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ProfileSnapshotParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.command.profileSnapshot/parameters/snapshotId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The id of the layer snapshot.</pre>
                    ///
                    ///
                    #[serde(rename = "snapshotId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub snapshot_id: crate::layer_tree::types::SnapshotId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.command.profileSnapshot/parameters/minRepeatCount。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The maximum number of times to replay the snapshot (1, if not specified).</pre>
                    ///
                    ///
                    #[serde(rename = "minRepeatCount")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub min_repeat_count: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.command.profileSnapshot/parameters/minDuration。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The minimum duration (in seconds) to replay the snapshot.</pre>
                    ///
                    ///
                    #[serde(rename = "minDuration")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub min_duration: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.command.profileSnapshot/parameters/clipRect。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The clip rectangle to apply when replaying the snapshot.</pre>
                    ///
                    ///
                    #[serde(rename = "clipRect")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub clip_rect: Option<crate::dom::types::Rect>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ProfileSnapshotParams {
                    snapshot_id: fields.snapshot_id,
                    min_repeat_count: fields.min_repeat_count,
                    min_duration: fields.min_duration,
                    clip_rect: fields.clip_rect,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：LayerTree.command.profileSnapshot。
#[derive(Debug, Clone, Copy)]
pub struct ProfileSnapshot;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：LayerTree.command.profileSnapshot/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProfileSnapshotResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.command.profileSnapshot/returns/timings。
    ///
    ///协议原文：
    ///
    ///<pre>The array of paint profiles, one per run.</pre>
    ///
    ///
    #[serde(rename = "timings")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timings: Vec<crate::layer_tree::types::PaintProfile>,
}
impl<'de> serde::Deserialize<'de> for ProfileSnapshotResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ProfileSnapshotResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.command.profileSnapshot/returns/timings。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The array of paint profiles, one per run.</pre>
                    ///
                    ///
                    #[serde(rename = "timings")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timings: Vec<crate::layer_tree::types::PaintProfile>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ProfileSnapshotResponse {
                    timings: fields.timings,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ProfileSnapshot {
    type Params = ProfileSnapshotParams;
    type Response = ProfileSnapshotResponse;
    const METHOD: &'static str = "LayerTree.profileSnapshot";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：LayerTree.command.releaseSnapshot/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReleaseSnapshotParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.command.releaseSnapshot/parameters/snapshotId。
    ///
    ///协议原文：
    ///
    ///<pre>The id of the layer snapshot.</pre>
    ///
    ///
    #[serde(rename = "snapshotId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub snapshot_id: crate::layer_tree::types::SnapshotId,
}
impl<'de> serde::Deserialize<'de> for ReleaseSnapshotParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReleaseSnapshotParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.command.releaseSnapshot/parameters/snapshotId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The id of the layer snapshot.</pre>
                    ///
                    ///
                    #[serde(rename = "snapshotId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub snapshot_id: crate::layer_tree::types::SnapshotId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReleaseSnapshotParams {
                    snapshot_id: fields.snapshot_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：LayerTree.command.releaseSnapshot。
///
///协议原文：
///
///<pre>Releases layer snapshot captured by the back-end.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ReleaseSnapshot;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：LayerTree.command.releaseSnapshot/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReleaseSnapshotResponse {}
impl<'de> serde::Deserialize<'de> for ReleaseSnapshotResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReleaseSnapshotResponse;
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
                Ok(ReleaseSnapshotResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ReleaseSnapshot {
    type Params = ReleaseSnapshotParams;
    type Response = ReleaseSnapshotResponse;
    const METHOD: &'static str = "LayerTree.releaseSnapshot";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：LayerTree.command.replaySnapshot/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReplaySnapshotParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.command.replaySnapshot/parameters/snapshotId。
    ///
    ///协议原文：
    ///
    ///<pre>The id of the layer snapshot.</pre>
    ///
    ///
    #[serde(rename = "snapshotId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub snapshot_id: crate::layer_tree::types::SnapshotId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.command.replaySnapshot/parameters/fromStep。
    ///
    ///协议原文：
    ///
    ///<pre>The first step to replay from (replay from the very start if not specified).</pre>
    ///
    ///
    #[serde(rename = "fromStep")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub from_step: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.command.replaySnapshot/parameters/toStep。
    ///
    ///协议原文：
    ///
    ///<pre>The last step to replay to (replay till the end if not specified).</pre>
    ///
    ///
    #[serde(rename = "toStep")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub to_step: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.command.replaySnapshot/parameters/scale。
    ///
    ///协议原文：
    ///
    ///<pre>The scale to apply while replaying (defaults to 1).</pre>
    ///
    ///
    #[serde(rename = "scale")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub scale: Option<ydp_core::FiniteF64>,
}
impl<'de> serde::Deserialize<'de> for ReplaySnapshotParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReplaySnapshotParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.command.replaySnapshot/parameters/snapshotId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The id of the layer snapshot.</pre>
                    ///
                    ///
                    #[serde(rename = "snapshotId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub snapshot_id: crate::layer_tree::types::SnapshotId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.command.replaySnapshot/parameters/fromStep。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The first step to replay from (replay from the very start if not specified).</pre>
                    ///
                    ///
                    #[serde(rename = "fromStep")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub from_step: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.command.replaySnapshot/parameters/toStep。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The last step to replay to (replay till the end if not specified).</pre>
                    ///
                    ///
                    #[serde(rename = "toStep")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub to_step: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：LayerTree.command.replaySnapshot/parameters/scale。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The scale to apply while replaying (defaults to 1).</pre>
                    ///
                    ///
                    #[serde(rename = "scale")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub scale: Option<ydp_core::FiniteF64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReplaySnapshotParams {
                    snapshot_id: fields.snapshot_id,
                    from_step: fields.from_step,
                    to_step: fields.to_step,
                    scale: fields.scale,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：LayerTree.command.replaySnapshot。
///
///协议原文：
///
///<pre>Replays the layer snapshot and returns the resulting bitmap.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ReplaySnapshot;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：LayerTree.command.replaySnapshot/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReplaySnapshotResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.command.replaySnapshot/returns/dataURL。
    ///
    ///协议原文：
    ///
    ///<pre>A data: URL for resulting image.</pre>
    ///
    ///
    #[serde(rename = "dataURL")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub data_url: String,
}
impl<'de> serde::Deserialize<'de> for ReplaySnapshotResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReplaySnapshotResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.command.replaySnapshot/returns/dataURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A data: URL for resulting image.</pre>
                    ///
                    ///
                    #[serde(rename = "dataURL")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub data_url: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReplaySnapshotResponse {
                    data_url: fields.data_url,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ReplaySnapshot {
    type Params = ReplaySnapshotParams;
    type Response = ReplaySnapshotResponse;
    const METHOD: &'static str = "LayerTree.replaySnapshot";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：LayerTree.command.snapshotCommandLog/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SnapshotCommandLogParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.command.snapshotCommandLog/parameters/snapshotId。
    ///
    ///协议原文：
    ///
    ///<pre>The id of the layer snapshot.</pre>
    ///
    ///
    #[serde(rename = "snapshotId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub snapshot_id: crate::layer_tree::types::SnapshotId,
}
impl<'de> serde::Deserialize<'de> for SnapshotCommandLogParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SnapshotCommandLogParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.command.snapshotCommandLog/parameters/snapshotId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The id of the layer snapshot.</pre>
                    ///
                    ///
                    #[serde(rename = "snapshotId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub snapshot_id: crate::layer_tree::types::SnapshotId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SnapshotCommandLogParams {
                    snapshot_id: fields.snapshot_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：LayerTree.command.snapshotCommandLog。
///
///协议原文：
///
///<pre>Replays the layer snapshot and returns canvas log.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SnapshotCommandLog;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：LayerTree.command.snapshotCommandLog/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SnapshotCommandLogResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.command.snapshotCommandLog/returns/commandLog。
    ///
    ///协议原文：
    ///
    ///<pre>The array of canvas function calls.</pre>
    ///
    ///
    #[serde(rename = "commandLog")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub command_log: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
}
impl<'de> serde::Deserialize<'de> for SnapshotCommandLogResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SnapshotCommandLogResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：LayerTree.command.snapshotCommandLog/returns/commandLog。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The array of canvas function calls.</pre>
                    ///
                    ///
                    #[serde(rename = "commandLog")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub command_log: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SnapshotCommandLogResponse {
                    command_log: fields.command_log,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SnapshotCommandLog {
    type Params = SnapshotCommandLogParams;
    type Response = SnapshotCommandLogResponse;
    const METHOD: &'static str = "LayerTree.snapshotCommandLog";
}
