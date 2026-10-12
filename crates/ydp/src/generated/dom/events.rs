///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.event.adRelatedStateUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AdRelatedStateUpdatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.adRelatedStateUpdated/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>The id of the node.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.event.adRelatedStateUpdated/parameters/adProvenance。
    ///
    ///协议原文：
    ///
    ///<pre>The provenance of the ad related node, if it is ad related.</pre>
    ///
    ///
    #[serde(rename = "adProvenance")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub ad_provenance: Option<crate::network::types::AdProvenance>,
}
impl<'de> serde::Deserialize<'de> for AdRelatedStateUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AdRelatedStateUpdatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.adRelatedStateUpdated/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The id of the node.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：DOM.event.adRelatedStateUpdated/parameters/adProvenance。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The provenance of the ad related node, if it is ad related.</pre>
                    ///
                    ///
                    #[serde(rename = "adProvenance")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub ad_provenance: Option<crate::network::types::AdProvenance>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AdRelatedStateUpdatedParams {
                    node_id: fields.node_id,
                    ad_provenance: fields.ad_provenance,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.event.adRelatedStateUpdated。
///
///协议原文：
///
///<pre>Fired when a node's ad related state changes.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct AdRelatedStateUpdated;
impl ydp_core::Event for AdRelatedStateUpdated {
    type Params = AdRelatedStateUpdatedParams;
    const METHOD: &'static str = "DOM.adRelatedStateUpdated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.event.adoptedStyleSheetsModified/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AdoptedStyleSheetsModifiedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.adoptedStyleSheetsModified/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node that has changed.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.adoptedStyleSheetsModified/parameters/adoptedStyleSheets。
    ///
    ///协议原文：
    ///
    ///<pre>New adoptedStyleSheets array.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "adoptedStyleSheets")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub adopted_style_sheets: Vec<crate::dom::types::StyleSheetId>,
}
impl<'de> serde::Deserialize<'de> for AdoptedStyleSheetsModifiedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AdoptedStyleSheetsModifiedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.adoptedStyleSheetsModified/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node that has changed.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.adoptedStyleSheetsModified/parameters/adoptedStyleSheets。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>New adoptedStyleSheets array.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "adoptedStyleSheets")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub adopted_style_sheets: Vec<crate::dom::types::StyleSheetId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AdoptedStyleSheetsModifiedParams {
                    node_id: fields.node_id,
                    adopted_style_sheets: fields.adopted_style_sheets,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.event.adoptedStyleSheetsModified。
///
///协议原文：
///
///<pre>Fired when `Element`'s adoptedStyleSheets are modified.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct AdoptedStyleSheetsModified;
impl ydp_core::Event for AdoptedStyleSheetsModified {
    type Params = AdoptedStyleSheetsModifiedParams;
    const METHOD: &'static str = "DOM.adoptedStyleSheetsModified";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.event.affectedByStartingStylesFlagUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AffectedByStartingStylesFlagUpdatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.affectedByStartingStylesFlagUpdated/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>The id of the node.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.affectedByStartingStylesFlagUpdated/parameters/affectedByStartingStyles。
    ///
    ///协议原文：
    ///
    ///<pre>If the node has starting styles.</pre>
    ///
    ///
    #[serde(rename = "affectedByStartingStyles")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub affected_by_starting_styles: bool,
}
impl<'de> serde::Deserialize<'de> for AffectedByStartingStylesFlagUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AffectedByStartingStylesFlagUpdatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.affectedByStartingStylesFlagUpdated/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The id of the node.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.affectedByStartingStylesFlagUpdated/parameters/affectedByStartingStyles。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If the node has starting styles.</pre>
                    ///
                    ///
                    #[serde(rename = "affectedByStartingStyles")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub affected_by_starting_styles: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AffectedByStartingStylesFlagUpdatedParams {
                    node_id: fields.node_id,
                    affected_by_starting_styles: fields.affected_by_starting_styles,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.event.affectedByStartingStylesFlagUpdated。
///
///协议原文：
///
///<pre>Fired when a node's starting styles changes.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct AffectedByStartingStylesFlagUpdated;
impl ydp_core::Event for AffectedByStartingStylesFlagUpdated {
    type Params = AffectedByStartingStylesFlagUpdatedParams;
    const METHOD: &'static str = "DOM.affectedByStartingStylesFlagUpdated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.event.attributeModified/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AttributeModifiedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.attributeModified/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node that has changed.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.attributeModified/parameters/name。
    ///
    ///协议原文：
    ///
    ///<pre>Attribute name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.attributeModified/parameters/value。
    ///
    ///协议原文：
    ///
    ///<pre>Attribute value.</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: String,
}
impl<'de> serde::Deserialize<'de> for AttributeModifiedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AttributeModifiedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.attributeModified/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node that has changed.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.attributeModified/parameters/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Attribute name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.attributeModified/parameters/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Attribute value.</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AttributeModifiedParams {
                    node_id: fields.node_id,
                    name: fields.name,
                    value: fields.value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.event.attributeModified。
///
///协议原文：
///
///<pre>Fired when `Element`'s attribute is modified.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct AttributeModified;
impl ydp_core::Event for AttributeModified {
    type Params = AttributeModifiedParams;
    const METHOD: &'static str = "DOM.attributeModified";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.event.attributeRemoved/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AttributeRemovedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.attributeRemoved/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node that has changed.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.attributeRemoved/parameters/name。
    ///
    ///协议原文：
    ///
    ///<pre>A ttribute name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
}
impl<'de> serde::Deserialize<'de> for AttributeRemovedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AttributeRemovedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.attributeRemoved/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node that has changed.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.attributeRemoved/parameters/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A ttribute name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AttributeRemovedParams {
                    node_id: fields.node_id,
                    name: fields.name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.event.attributeRemoved。
///
///协议原文：
///
///<pre>Fired when `Element`'s attribute is removed.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct AttributeRemoved;
impl ydp_core::Event for AttributeRemoved {
    type Params = AttributeRemovedParams;
    const METHOD: &'static str = "DOM.attributeRemoved";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.event.characterDataModified/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CharacterDataModifiedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.characterDataModified/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node that has changed.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.characterDataModified/parameters/characterData。
    ///
    ///协议原文：
    ///
    ///<pre>New text value.</pre>
    ///
    ///
    #[serde(rename = "characterData")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub character_data: String,
}
impl<'de> serde::Deserialize<'de> for CharacterDataModifiedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CharacterDataModifiedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.characterDataModified/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node that has changed.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.characterDataModified/parameters/characterData。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>New text value.</pre>
                    ///
                    ///
                    #[serde(rename = "characterData")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub character_data: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CharacterDataModifiedParams {
                    node_id: fields.node_id,
                    character_data: fields.character_data,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.event.characterDataModified。
///
///协议原文：
///
///<pre>Mirrors `DOMCharacterDataModified` event.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct CharacterDataModified;
impl ydp_core::Event for CharacterDataModified {
    type Params = CharacterDataModifiedParams;
    const METHOD: &'static str = "DOM.characterDataModified";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.event.childNodeCountUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ChildNodeCountUpdatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.childNodeCountUpdated/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node that has changed.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.childNodeCountUpdated/parameters/childNodeCount。
    ///
    ///协议原文：
    ///
    ///<pre>New node count.</pre>
    ///
    ///
    #[serde(rename = "childNodeCount")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub child_node_count: i64,
}
impl<'de> serde::Deserialize<'de> for ChildNodeCountUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ChildNodeCountUpdatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.childNodeCountUpdated/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node that has changed.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.childNodeCountUpdated/parameters/childNodeCount。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>New node count.</pre>
                    ///
                    ///
                    #[serde(rename = "childNodeCount")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub child_node_count: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ChildNodeCountUpdatedParams {
                    node_id: fields.node_id,
                    child_node_count: fields.child_node_count,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.event.childNodeCountUpdated。
///
///协议原文：
///
///<pre>Fired when `Container`'s child node count has changed.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ChildNodeCountUpdated;
impl ydp_core::Event for ChildNodeCountUpdated {
    type Params = ChildNodeCountUpdatedParams;
    const METHOD: &'static str = "DOM.childNodeCountUpdated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.event.childNodeInserted/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ChildNodeInsertedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.childNodeInserted/parameters/parentNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node that has changed.</pre>
    ///
    ///
    #[serde(rename = "parentNodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub parent_node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.childNodeInserted/parameters/previousNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the previous sibling.</pre>
    ///
    ///
    #[serde(rename = "previousNodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub previous_node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.childNodeInserted/parameters/node。
    ///
    ///协议原文：
    ///
    ///<pre>Inserted node data.</pre>
    ///
    ///
    #[serde(rename = "node")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node: crate::dom::types::Node,
}
impl<'de> serde::Deserialize<'de> for ChildNodeInsertedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ChildNodeInsertedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.childNodeInserted/parameters/parentNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node that has changed.</pre>
                    ///
                    ///
                    #[serde(rename = "parentNodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub parent_node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.childNodeInserted/parameters/previousNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the previous sibling.</pre>
                    ///
                    ///
                    #[serde(rename = "previousNodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub previous_node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.childNodeInserted/parameters/node。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Inserted node data.</pre>
                    ///
                    ///
                    #[serde(rename = "node")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node: crate::dom::types::Node,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ChildNodeInsertedParams {
                    parent_node_id: fields.parent_node_id,
                    previous_node_id: fields.previous_node_id,
                    node: fields.node,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.event.childNodeInserted。
///
///协议原文：
///
///<pre>Mirrors `DOMNodeInserted` event.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ChildNodeInserted;
impl ydp_core::Event for ChildNodeInserted {
    type Params = ChildNodeInsertedParams;
    const METHOD: &'static str = "DOM.childNodeInserted";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.event.childNodeRemoved/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ChildNodeRemovedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.childNodeRemoved/parameters/parentNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Parent id.</pre>
    ///
    ///
    #[serde(rename = "parentNodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub parent_node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.childNodeRemoved/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node that has been removed.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for ChildNodeRemovedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ChildNodeRemovedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.childNodeRemoved/parameters/parentNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Parent id.</pre>
                    ///
                    ///
                    #[serde(rename = "parentNodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub parent_node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.childNodeRemoved/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node that has been removed.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ChildNodeRemovedParams {
                    parent_node_id: fields.parent_node_id,
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.event.childNodeRemoved。
///
///协议原文：
///
///<pre>Mirrors `DOMNodeRemoved` event.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ChildNodeRemoved;
impl ydp_core::Event for ChildNodeRemoved {
    type Params = ChildNodeRemovedParams;
    const METHOD: &'static str = "DOM.childNodeRemoved";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.event.distributedNodesUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DistributedNodesUpdatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.distributedNodesUpdated/parameters/insertionPointId。
    ///
    ///协议原文：
    ///
    ///<pre>Insertion point where distributed nodes were updated.</pre>
    ///
    ///
    #[serde(rename = "insertionPointId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub insertion_point_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.distributedNodesUpdated/parameters/distributedNodes。
    ///
    ///协议原文：
    ///
    ///<pre>Distributed nodes for given insertion point.</pre>
    ///
    ///
    #[serde(rename = "distributedNodes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub distributed_nodes: Vec<crate::dom::types::BackendNode>,
}
impl<'de> serde::Deserialize<'de> for DistributedNodesUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DistributedNodesUpdatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.distributedNodesUpdated/parameters/insertionPointId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Insertion point where distributed nodes were updated.</pre>
                    ///
                    ///
                    #[serde(rename = "insertionPointId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub insertion_point_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.distributedNodesUpdated/parameters/distributedNodes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Distributed nodes for given insertion point.</pre>
                    ///
                    ///
                    #[serde(rename = "distributedNodes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub distributed_nodes: Vec<crate::dom::types::BackendNode>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DistributedNodesUpdatedParams {
                    insertion_point_id: fields.insertion_point_id,
                    distributed_nodes: fields.distributed_nodes,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.event.distributedNodesUpdated。
///
///协议原文：
///
///<pre>Called when distribution is changed.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct DistributedNodesUpdated;
impl ydp_core::Event for DistributedNodesUpdated {
    type Params = DistributedNodesUpdatedParams;
    const METHOD: &'static str = "DOM.distributedNodesUpdated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.event.documentUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct DocumentUpdatedParams {}
impl<'de> serde::Deserialize<'de> for DocumentUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DocumentUpdatedParams;
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
                Ok(DocumentUpdatedParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.event.documentUpdated。
///
///协议原文：
///
///<pre>Fired when `Document` has been totally updated. Node ids are no longer valid.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct DocumentUpdated;
impl ydp_core::Event for DocumentUpdated {
    type Params = DocumentUpdatedParams;
    const METHOD: &'static str = "DOM.documentUpdated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.event.inlineStyleInvalidated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct InlineStyleInvalidatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.inlineStyleInvalidated/parameters/nodeIds。
    ///
    ///协议原文：
    ///
    ///<pre>Ids of the nodes for which the inline styles have been invalidated.</pre>
    ///
    ///
    #[serde(rename = "nodeIds")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_ids: Vec<crate::dom::types::NodeId>,
}
impl<'de> serde::Deserialize<'de> for InlineStyleInvalidatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InlineStyleInvalidatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.inlineStyleInvalidated/parameters/nodeIds。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Ids of the nodes for which the inline styles have been invalidated.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeIds")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_ids: Vec<crate::dom::types::NodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(InlineStyleInvalidatedParams {
                    node_ids: fields.node_ids,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.event.inlineStyleInvalidated。
///
///协议原文：
///
///<pre>Fired when `Element`'s inline style is modified via a CSS property modification.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct InlineStyleInvalidated;
impl ydp_core::Event for InlineStyleInvalidated {
    type Params = InlineStyleInvalidatedParams;
    const METHOD: &'static str = "DOM.inlineStyleInvalidated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.event.pseudoElementAdded/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PseudoElementAddedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.pseudoElementAdded/parameters/parentId。
    ///
    ///协议原文：
    ///
    ///<pre>Pseudo element's parent element id.</pre>
    ///
    ///
    #[serde(rename = "parentId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub parent_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.pseudoElementAdded/parameters/pseudoElement。
    ///
    ///协议原文：
    ///
    ///<pre>The added pseudo element.</pre>
    ///
    ///
    #[serde(rename = "pseudoElement")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub pseudo_element: crate::dom::types::Node,
}
impl<'de> serde::Deserialize<'de> for PseudoElementAddedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PseudoElementAddedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.pseudoElementAdded/parameters/parentId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Pseudo element's parent element id.</pre>
                    ///
                    ///
                    #[serde(rename = "parentId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub parent_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.pseudoElementAdded/parameters/pseudoElement。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The added pseudo element.</pre>
                    ///
                    ///
                    #[serde(rename = "pseudoElement")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub pseudo_element: crate::dom::types::Node,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PseudoElementAddedParams {
                    parent_id: fields.parent_id,
                    pseudo_element: fields.pseudo_element,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.event.pseudoElementAdded。
///
///协议原文：
///
///<pre>Called when a pseudo element is added to an element.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct PseudoElementAdded;
impl ydp_core::Event for PseudoElementAdded {
    type Params = PseudoElementAddedParams;
    const METHOD: &'static str = "DOM.pseudoElementAdded";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.event.pseudoElementRemoved/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PseudoElementRemovedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.pseudoElementRemoved/parameters/parentId。
    ///
    ///协议原文：
    ///
    ///<pre>Pseudo element's parent element id.</pre>
    ///
    ///
    #[serde(rename = "parentId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub parent_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.pseudoElementRemoved/parameters/pseudoElementId。
    ///
    ///协议原文：
    ///
    ///<pre>The removed pseudo element id.</pre>
    ///
    ///
    #[serde(rename = "pseudoElementId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub pseudo_element_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for PseudoElementRemovedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PseudoElementRemovedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.pseudoElementRemoved/parameters/parentId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Pseudo element's parent element id.</pre>
                    ///
                    ///
                    #[serde(rename = "parentId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub parent_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.pseudoElementRemoved/parameters/pseudoElementId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The removed pseudo element id.</pre>
                    ///
                    ///
                    #[serde(rename = "pseudoElementId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub pseudo_element_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PseudoElementRemovedParams {
                    parent_id: fields.parent_id,
                    pseudo_element_id: fields.pseudo_element_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.event.pseudoElementRemoved。
///
///协议原文：
///
///<pre>Called when a pseudo element is removed from an element.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct PseudoElementRemoved;
impl ydp_core::Event for PseudoElementRemoved {
    type Params = PseudoElementRemovedParams;
    const METHOD: &'static str = "DOM.pseudoElementRemoved";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.event.scrollableFlagUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ScrollableFlagUpdatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.scrollableFlagUpdated/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>The id of the node.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.scrollableFlagUpdated/parameters/isScrollable。
    ///
    ///协议原文：
    ///
    ///<pre>If the node is scrollable.</pre>
    ///
    ///
    #[serde(rename = "isScrollable")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub is_scrollable: bool,
}
impl<'de> serde::Deserialize<'de> for ScrollableFlagUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScrollableFlagUpdatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.scrollableFlagUpdated/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The id of the node.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.scrollableFlagUpdated/parameters/isScrollable。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If the node is scrollable.</pre>
                    ///
                    ///
                    #[serde(rename = "isScrollable")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub is_scrollable: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ScrollableFlagUpdatedParams {
                    node_id: fields.node_id,
                    is_scrollable: fields.is_scrollable,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.event.scrollableFlagUpdated。
///
///协议原文：
///
///<pre>Fired when a node's scrollability state changes.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ScrollableFlagUpdated;
impl ydp_core::Event for ScrollableFlagUpdated {
    type Params = ScrollableFlagUpdatedParams;
    const METHOD: &'static str = "DOM.scrollableFlagUpdated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.event.setChildNodes/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetChildNodesParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.setChildNodes/parameters/parentId。
    ///
    ///协议原文：
    ///
    ///<pre>Parent node id to populate with children.</pre>
    ///
    ///
    #[serde(rename = "parentId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub parent_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.setChildNodes/parameters/nodes。
    ///
    ///协议原文：
    ///
    ///<pre>Child nodes array.</pre>
    ///
    ///
    #[serde(rename = "nodes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub nodes: Vec<crate::dom::types::Node>,
}
impl<'de> serde::Deserialize<'de> for SetChildNodesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetChildNodesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.setChildNodes/parameters/parentId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Parent node id to populate with children.</pre>
                    ///
                    ///
                    #[serde(rename = "parentId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub parent_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.setChildNodes/parameters/nodes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Child nodes array.</pre>
                    ///
                    ///
                    #[serde(rename = "nodes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub nodes: Vec<crate::dom::types::Node>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetChildNodesParams {
                    parent_id: fields.parent_id,
                    nodes: fields.nodes,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.event.setChildNodes。
///
///协议原文：
///
///<pre>Fired when backend wants to provide client with the missing DOM structure. This happens upon
///most of the calls requesting node ids.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetChildNodes;
impl ydp_core::Event for SetChildNodes {
    type Params = SetChildNodesParams;
    const METHOD: &'static str = "DOM.setChildNodes";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.event.shadowRootPopped/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ShadowRootPoppedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.shadowRootPopped/parameters/hostId。
    ///
    ///协议原文：
    ///
    ///<pre>Host element id.</pre>
    ///
    ///
    #[serde(rename = "hostId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub host_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.shadowRootPopped/parameters/rootId。
    ///
    ///协议原文：
    ///
    ///<pre>Shadow root id.</pre>
    ///
    ///
    #[serde(rename = "rootId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub root_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for ShadowRootPoppedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ShadowRootPoppedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.shadowRootPopped/parameters/hostId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Host element id.</pre>
                    ///
                    ///
                    #[serde(rename = "hostId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub host_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.shadowRootPopped/parameters/rootId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Shadow root id.</pre>
                    ///
                    ///
                    #[serde(rename = "rootId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub root_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ShadowRootPoppedParams {
                    host_id: fields.host_id,
                    root_id: fields.root_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.event.shadowRootPopped。
///
///协议原文：
///
///<pre>Called when shadow root is popped from the element.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ShadowRootPopped;
impl ydp_core::Event for ShadowRootPopped {
    type Params = ShadowRootPoppedParams;
    const METHOD: &'static str = "DOM.shadowRootPopped";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.event.shadowRootPushed/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ShadowRootPushedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.shadowRootPushed/parameters/hostId。
    ///
    ///协议原文：
    ///
    ///<pre>Host element id.</pre>
    ///
    ///
    #[serde(rename = "hostId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub host_id: crate::dom::types::NodeId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.shadowRootPushed/parameters/root。
    ///
    ///协议原文：
    ///
    ///<pre>Shadow root.</pre>
    ///
    ///
    #[serde(rename = "root")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub root: crate::dom::types::Node,
}
impl<'de> serde::Deserialize<'de> for ShadowRootPushedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ShadowRootPushedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.shadowRootPushed/parameters/hostId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Host element id.</pre>
                    ///
                    ///
                    #[serde(rename = "hostId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub host_id: crate::dom::types::NodeId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOM.event.shadowRootPushed/parameters/root。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Shadow root.</pre>
                    ///
                    ///
                    #[serde(rename = "root")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub root: crate::dom::types::Node,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ShadowRootPushedParams {
                    host_id: fields.host_id,
                    root: fields.root,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.event.shadowRootPushed。
///
///协议原文：
///
///<pre>Called when shadow root is pushed into the element.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ShadowRootPushed;
impl ydp_core::Event for ShadowRootPushed {
    type Params = ShadowRootPushedParams;
    const METHOD: &'static str = "DOM.shadowRootPushed";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOM.event.topLayerElementsUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct TopLayerElementsUpdatedParams {}
impl<'de> serde::Deserialize<'de> for TopLayerElementsUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TopLayerElementsUpdatedParams;
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
                Ok(TopLayerElementsUpdatedParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOM.event.topLayerElementsUpdated。
///
///协议原文：
///
///<pre>Called when top layer elements are changed.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct TopLayerElementsUpdated;
impl ydp_core::Event for TopLayerElementsUpdated {
    type Params = TopLayerElementsUpdatedParams;
    const METHOD: &'static str = "DOM.topLayerElementsUpdated";
}
