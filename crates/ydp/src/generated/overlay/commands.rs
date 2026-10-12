///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.disable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.disable。
///
///协议原文：
///
///<pre>Disables domain notifications.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.disable/returns。
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
    const METHOD: &'static str = "Overlay.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.enable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.enable。
///
///协议原文：
///
///<pre>Enables domain notifications.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.enable/returns。
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
    const METHOD: &'static str = "Overlay.enable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.getGridHighlightObjectsForTest/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetGridHighlightObjectsForTestParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.getGridHighlightObjectsForTest/parameters/nodeIds。
    ///
    ///协议原文：
    ///
    ///<pre>Ids of the node to get highlight object for.</pre>
    ///
    ///
    #[serde(rename = "nodeIds")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_ids: Vec<crate::dom::types::NodeId>,
}
impl<'de> serde::Deserialize<'de> for GetGridHighlightObjectsForTestParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetGridHighlightObjectsForTestParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.getGridHighlightObjectsForTest/parameters/nodeIds。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Ids of the node to get highlight object for.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeIds")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_ids: Vec<crate::dom::types::NodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetGridHighlightObjectsForTestParams {
                    node_ids: fields.node_ids,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.getGridHighlightObjectsForTest。
///
///协议原文：
///
///<pre>For Persistent Grid testing.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetGridHighlightObjectsForTest;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.getGridHighlightObjectsForTest/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetGridHighlightObjectsForTestResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.getGridHighlightObjectsForTest/returns/highlights。
    ///
    ///协议原文：
    ///
    ///<pre>Grid Highlight data for the node ids provided.</pre>
    ///
    ///
    #[serde(rename = "highlights")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub highlights: std::collections::BTreeMap<String, serde_json::Value>,
}
impl<'de> serde::Deserialize<'de> for GetGridHighlightObjectsForTestResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetGridHighlightObjectsForTestResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.getGridHighlightObjectsForTest/returns/highlights。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Grid Highlight data for the node ids provided.</pre>
                    ///
                    ///
                    #[serde(rename = "highlights")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub highlights: std::collections::BTreeMap<String, serde_json::Value>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetGridHighlightObjectsForTestResponse {
                    highlights: fields.highlights,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetGridHighlightObjectsForTest {
    type Params = GetGridHighlightObjectsForTestParams;
    type Response = GetGridHighlightObjectsForTestResponse;
    const METHOD: &'static str = "Overlay.getGridHighlightObjectsForTest";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.getHighlightObjectForTest/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetHighlightObjectForTestParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.getHighlightObjectForTest/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node to get highlight object for.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.getHighlightObjectForTest/parameters/includeDistance。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to include distance info.</pre>
    ///
    ///
    #[serde(rename = "includeDistance")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_distance: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.getHighlightObjectForTest/parameters/includeStyle。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to include style info.</pre>
    ///
    ///
    #[serde(rename = "includeStyle")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_style: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.getHighlightObjectForTest/parameters/colorFormat。
    ///
    ///协议原文：
    ///
    ///<pre>The color format to get config with (default: hex).</pre>
    ///
    ///
    #[serde(rename = "colorFormat")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub color_format: Option<crate::overlay::types::ColorFormat>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.getHighlightObjectForTest/parameters/showAccessibilityInfo。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to show accessibility info (default: true).</pre>
    ///
    ///
    #[serde(rename = "showAccessibilityInfo")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub show_accessibility_info: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for GetHighlightObjectForTestParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetHighlightObjectForTestParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.getHighlightObjectForTest/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node to get highlight object for.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.getHighlightObjectForTest/parameters/includeDistance。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to include distance info.</pre>
                    ///
                    ///
                    #[serde(rename = "includeDistance")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub include_distance: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.getHighlightObjectForTest/parameters/includeStyle。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to include style info.</pre>
                    ///
                    ///
                    #[serde(rename = "includeStyle")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub include_style: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.getHighlightObjectForTest/parameters/colorFormat。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The color format to get config with (default: hex).</pre>
                    ///
                    ///
                    #[serde(rename = "colorFormat")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub color_format: Option<crate::overlay::types::ColorFormat>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.getHighlightObjectForTest/parameters/showAccessibilityInfo。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to show accessibility info (default: true).</pre>
                    ///
                    ///
                    #[serde(rename = "showAccessibilityInfo")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub show_accessibility_info: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetHighlightObjectForTestParams {
                    node_id: fields.node_id,
                    include_distance: fields.include_distance,
                    include_style: fields.include_style,
                    color_format: fields.color_format,
                    show_accessibility_info: fields.show_accessibility_info,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.getHighlightObjectForTest。
///
///协议原文：
///
///<pre>For testing.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetHighlightObjectForTest;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.getHighlightObjectForTest/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetHighlightObjectForTestResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.getHighlightObjectForTest/returns/highlight。
    ///
    ///协议原文：
    ///
    ///<pre>Highlight data for the node.</pre>
    ///
    ///
    #[serde(rename = "highlight")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub highlight: std::collections::BTreeMap<String, serde_json::Value>,
}
impl<'de> serde::Deserialize<'de> for GetHighlightObjectForTestResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetHighlightObjectForTestResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.getHighlightObjectForTest/returns/highlight。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Highlight data for the node.</pre>
                    ///
                    ///
                    #[serde(rename = "highlight")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub highlight: std::collections::BTreeMap<String, serde_json::Value>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetHighlightObjectForTestResponse {
                    highlight: fields.highlight,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetHighlightObjectForTest {
    type Params = GetHighlightObjectForTestParams;
    type Response = GetHighlightObjectForTestResponse;
    const METHOD: &'static str = "Overlay.getHighlightObjectForTest";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.getSourceOrderHighlightObjectForTest/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetSourceOrderHighlightObjectForTestParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.getSourceOrderHighlightObjectForTest/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node to highlight.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for GetSourceOrderHighlightObjectForTestParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetSourceOrderHighlightObjectForTestParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.getSourceOrderHighlightObjectForTest/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node to highlight.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetSourceOrderHighlightObjectForTestParams {
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.getSourceOrderHighlightObjectForTest。
///
///协议原文：
///
///<pre>For Source Order Viewer testing.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetSourceOrderHighlightObjectForTest;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.getSourceOrderHighlightObjectForTest/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetSourceOrderHighlightObjectForTestResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.getSourceOrderHighlightObjectForTest/returns/highlight。
    ///
    ///协议原文：
    ///
    ///<pre>Source order highlight data for the node id provided.</pre>
    ///
    ///
    #[serde(rename = "highlight")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub highlight: std::collections::BTreeMap<String, serde_json::Value>,
}
impl<'de> serde::Deserialize<'de> for GetSourceOrderHighlightObjectForTestResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetSourceOrderHighlightObjectForTestResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.getSourceOrderHighlightObjectForTest/returns/highlight。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Source order highlight data for the node id provided.</pre>
                    ///
                    ///
                    #[serde(rename = "highlight")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub highlight: std::collections::BTreeMap<String, serde_json::Value>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetSourceOrderHighlightObjectForTestResponse {
                    highlight: fields.highlight,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetSourceOrderHighlightObjectForTest {
    type Params = GetSourceOrderHighlightObjectForTestParams;
    type Response = GetSourceOrderHighlightObjectForTestResponse;
    const METHOD: &'static str = "Overlay.getSourceOrderHighlightObjectForTest";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.hideHighlight/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct HideHighlightParams {}
impl<'de> serde::Deserialize<'de> for HideHighlightParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HideHighlightParams;
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
                Ok(HideHighlightParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.hideHighlight。
///
///协议原文：
///
///<pre>Hides any highlight.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct HideHighlight;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.hideHighlight/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct HideHighlightResponse {}
impl<'de> serde::Deserialize<'de> for HideHighlightResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HideHighlightResponse;
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
                Ok(HideHighlightResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for HideHighlight {
    type Params = HideHighlightParams;
    type Response = HideHighlightResponse;
    const METHOD: &'static str = "Overlay.hideHighlight";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.highlightFrame/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct HighlightFrameParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.highlightFrame/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the frame to highlight.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.highlightFrame/parameters/contentColor。
    ///
    ///协议原文：
    ///
    ///<pre>The content box highlight fill color (default: transparent).</pre>
    ///
    ///
    #[serde(rename = "contentColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub content_color: Option<crate::dom::types::Rgba>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.highlightFrame/parameters/contentOutlineColor。
    ///
    ///协议原文：
    ///
    ///<pre>The content box highlight outline color (default: transparent).</pre>
    ///
    ///
    #[serde(rename = "contentOutlineColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub content_outline_color: Option<crate::dom::types::Rgba>,
}
impl<'de> serde::Deserialize<'de> for HighlightFrameParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HighlightFrameParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.highlightFrame/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the frame to highlight.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.highlightFrame/parameters/contentColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The content box highlight fill color (default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "contentColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub content_color: Option<crate::dom::types::Rgba>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.highlightFrame/parameters/contentOutlineColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The content box highlight outline color (default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "contentOutlineColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub content_outline_color: Option<crate::dom::types::Rgba>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(HighlightFrameParams {
                    frame_id: fields.frame_id,
                    content_color: fields.content_color,
                    content_outline_color: fields.content_outline_color,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.highlightFrame。
///
///协议原文：
///
///<pre>Highlights owner element of the frame with given id.
///Deprecated: Doesn't work reliably and cannot be fixed due to process
///separation (the owner node might be in a different process). Determine
///the owner node in the client and use highlightNode.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct HighlightFrame;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.highlightFrame/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct HighlightFrameResponse {}
impl<'de> serde::Deserialize<'de> for HighlightFrameResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HighlightFrameResponse;
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
                Ok(HighlightFrameResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for HighlightFrame {
    type Params = HighlightFrameParams;
    type Response = HighlightFrameResponse;
    const METHOD: &'static str = "Overlay.highlightFrame";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.highlightNode/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct HighlightNodeParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.highlightNode/parameters/highlightConfig。
    ///
    ///协议原文：
    ///
    ///<pre>A descriptor for the highlight appearance.</pre>
    ///
    ///
    #[serde(rename = "highlightConfig")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub highlight_config: crate::overlay::types::HighlightConfig,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.highlightNode/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the node to highlight.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub node_id: Option<crate::dom::types::NodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.highlightNode/parameters/backendNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the backend node to highlight.</pre>
    ///
    ///
    #[serde(rename = "backendNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.highlightNode/parameters/objectId。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript object id of the node to be highlighted.</pre>
    ///
    ///
    #[serde(rename = "objectId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub object_id: Option<crate::runtime::types::RemoteObjectId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.highlightNode/parameters/selector。
    ///
    ///协议原文：
    ///
    ///<pre>Selectors to highlight relevant nodes.</pre>
    ///
    ///
    #[serde(rename = "selector")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub selector: Option<String>,
}
impl<'de> serde::Deserialize<'de> for HighlightNodeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HighlightNodeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.highlightNode/parameters/highlightConfig。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A descriptor for the highlight appearance.</pre>
                    ///
                    ///
                    #[serde(rename = "highlightConfig")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub highlight_config: crate::overlay::types::HighlightConfig,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.highlightNode/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the node to highlight.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub node_id: Option<crate::dom::types::NodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.highlightNode/parameters/backendNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the backend node to highlight.</pre>
                    ///
                    ///
                    #[serde(rename = "backendNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.highlightNode/parameters/objectId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript object id of the node to be highlighted.</pre>
                    ///
                    ///
                    #[serde(rename = "objectId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub object_id: Option<crate::runtime::types::RemoteObjectId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.highlightNode/parameters/selector。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Selectors to highlight relevant nodes.</pre>
                    ///
                    ///
                    #[serde(rename = "selector")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub selector: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(HighlightNodeParams {
                    highlight_config: fields.highlight_config,
                    node_id: fields.node_id,
                    backend_node_id: fields.backend_node_id,
                    object_id: fields.object_id,
                    selector: fields.selector,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.highlightNode。
///
///协议原文：
///
///<pre>Highlights DOM node with given id or with the given JavaScript object wrapper. Either nodeId or
///objectId must be specified.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct HighlightNode;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.highlightNode/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct HighlightNodeResponse {}
impl<'de> serde::Deserialize<'de> for HighlightNodeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HighlightNodeResponse;
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
                Ok(HighlightNodeResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for HighlightNode {
    type Params = HighlightNodeParams;
    type Response = HighlightNodeResponse;
    const METHOD: &'static str = "Overlay.highlightNode";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.highlightQuad/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct HighlightQuadParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.highlightQuad/parameters/quad。
    ///
    ///协议原文：
    ///
    ///<pre>Quad to highlight</pre>
    ///
    ///
    #[serde(rename = "quad")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub quad: crate::dom::types::Quad,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.highlightQuad/parameters/color。
    ///
    ///协议原文：
    ///
    ///<pre>The highlight fill color (default: transparent).</pre>
    ///
    ///
    #[serde(rename = "color")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub color: Option<crate::dom::types::Rgba>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.highlightQuad/parameters/outlineColor。
    ///
    ///协议原文：
    ///
    ///<pre>The highlight outline color (default: transparent).</pre>
    ///
    ///
    #[serde(rename = "outlineColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub outline_color: Option<crate::dom::types::Rgba>,
}
impl<'de> serde::Deserialize<'de> for HighlightQuadParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HighlightQuadParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.highlightQuad/parameters/quad。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Quad to highlight</pre>
                    ///
                    ///
                    #[serde(rename = "quad")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub quad: crate::dom::types::Quad,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.highlightQuad/parameters/color。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The highlight fill color (default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "color")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub color: Option<crate::dom::types::Rgba>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.highlightQuad/parameters/outlineColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The highlight outline color (default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "outlineColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub outline_color: Option<crate::dom::types::Rgba>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(HighlightQuadParams {
                    quad: fields.quad,
                    color: fields.color,
                    outline_color: fields.outline_color,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.highlightQuad。
///
///协议原文：
///
///<pre>Highlights given quad. Coordinates are absolute with respect to the main frame viewport.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct HighlightQuad;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.highlightQuad/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct HighlightQuadResponse {}
impl<'de> serde::Deserialize<'de> for HighlightQuadResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HighlightQuadResponse;
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
                Ok(HighlightQuadResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for HighlightQuad {
    type Params = HighlightQuadParams;
    type Response = HighlightQuadResponse;
    const METHOD: &'static str = "Overlay.highlightQuad";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.highlightRect/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct HighlightRectParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.highlightRect/parameters/x。
    ///
    ///协议原文：
    ///
    ///<pre>X coordinate</pre>
    ///
    ///
    #[serde(rename = "x")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub x: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.highlightRect/parameters/y。
    ///
    ///协议原文：
    ///
    ///<pre>Y coordinate</pre>
    ///
    ///
    #[serde(rename = "y")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub y: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.highlightRect/parameters/width。
    ///
    ///协议原文：
    ///
    ///<pre>Rectangle width</pre>
    ///
    ///
    #[serde(rename = "width")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub width: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.highlightRect/parameters/height。
    ///
    ///协议原文：
    ///
    ///<pre>Rectangle height</pre>
    ///
    ///
    #[serde(rename = "height")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub height: i64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.highlightRect/parameters/color。
    ///
    ///协议原文：
    ///
    ///<pre>The highlight fill color (default: transparent).</pre>
    ///
    ///
    #[serde(rename = "color")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub color: Option<crate::dom::types::Rgba>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.highlightRect/parameters/outlineColor。
    ///
    ///协议原文：
    ///
    ///<pre>The highlight outline color (default: transparent).</pre>
    ///
    ///
    #[serde(rename = "outlineColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub outline_color: Option<crate::dom::types::Rgba>,
}
impl<'de> serde::Deserialize<'de> for HighlightRectParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HighlightRectParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.highlightRect/parameters/x。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>X coordinate</pre>
                    ///
                    ///
                    #[serde(rename = "x")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub x: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.highlightRect/parameters/y。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Y coordinate</pre>
                    ///
                    ///
                    #[serde(rename = "y")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub y: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.highlightRect/parameters/width。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Rectangle width</pre>
                    ///
                    ///
                    #[serde(rename = "width")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub width: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.highlightRect/parameters/height。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Rectangle height</pre>
                    ///
                    ///
                    #[serde(rename = "height")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub height: i64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.highlightRect/parameters/color。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The highlight fill color (default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "color")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub color: Option<crate::dom::types::Rgba>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.highlightRect/parameters/outlineColor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The highlight outline color (default: transparent).</pre>
                    ///
                    ///
                    #[serde(rename = "outlineColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub outline_color: Option<crate::dom::types::Rgba>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(HighlightRectParams {
                    x: fields.x,
                    y: fields.y,
                    width: fields.width,
                    height: fields.height,
                    color: fields.color,
                    outline_color: fields.outline_color,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.highlightRect。
///
///协议原文：
///
///<pre>Highlights given rectangle. Coordinates are absolute with respect to the main frame viewport.
///Issue: the method does not handle device pixel ratio (DPR) correctly.
///The coordinates currently have to be adjusted by the client
///if DPR is not 1 (see crbug.com/437807128).</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct HighlightRect;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.highlightRect/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct HighlightRectResponse {}
impl<'de> serde::Deserialize<'de> for HighlightRectResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HighlightRectResponse;
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
                Ok(HighlightRectResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for HighlightRect {
    type Params = HighlightRectParams;
    type Response = HighlightRectResponse;
    const METHOD: &'static str = "Overlay.highlightRect";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.highlightSourceOrder/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct HighlightSourceOrderParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.highlightSourceOrder/parameters/sourceOrderConfig。
    ///
    ///协议原文：
    ///
    ///<pre>A descriptor for the appearance of the overlay drawing.</pre>
    ///
    ///
    #[serde(rename = "sourceOrderConfig")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub source_order_config: crate::overlay::types::SourceOrderConfig,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.highlightSourceOrder/parameters/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the node to highlight.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub node_id: Option<crate::dom::types::NodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.highlightSourceOrder/parameters/backendNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the backend node to highlight.</pre>
    ///
    ///
    #[serde(rename = "backendNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.highlightSourceOrder/parameters/objectId。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript object id of the node to be highlighted.</pre>
    ///
    ///
    #[serde(rename = "objectId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub object_id: Option<crate::runtime::types::RemoteObjectId>,
}
impl<'de> serde::Deserialize<'de> for HighlightSourceOrderParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HighlightSourceOrderParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.highlightSourceOrder/parameters/sourceOrderConfig。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A descriptor for the appearance of the overlay drawing.</pre>
                    ///
                    ///
                    #[serde(rename = "sourceOrderConfig")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub source_order_config: crate::overlay::types::SourceOrderConfig,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.highlightSourceOrder/parameters/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the node to highlight.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub node_id: Option<crate::dom::types::NodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.highlightSourceOrder/parameters/backendNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the backend node to highlight.</pre>
                    ///
                    ///
                    #[serde(rename = "backendNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.highlightSourceOrder/parameters/objectId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript object id of the node to be highlighted.</pre>
                    ///
                    ///
                    #[serde(rename = "objectId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub object_id: Option<crate::runtime::types::RemoteObjectId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(HighlightSourceOrderParams {
                    source_order_config: fields.source_order_config,
                    node_id: fields.node_id,
                    backend_node_id: fields.backend_node_id,
                    object_id: fields.object_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.highlightSourceOrder。
///
///协议原文：
///
///<pre>Highlights the source order of the children of the DOM node with given id or with the given
///JavaScript object wrapper. Either nodeId or objectId must be specified.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct HighlightSourceOrder;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.highlightSourceOrder/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct HighlightSourceOrderResponse {}
impl<'de> serde::Deserialize<'de> for HighlightSourceOrderResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HighlightSourceOrderResponse;
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
                Ok(HighlightSourceOrderResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for HighlightSourceOrder {
    type Params = HighlightSourceOrderParams;
    type Response = HighlightSourceOrderResponse;
    const METHOD: &'static str = "Overlay.highlightSourceOrder";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setInspectMode/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetInspectModeParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setInspectMode/parameters/mode。
    ///
    ///协议原文：
    ///
    ///<pre>Set an inspection mode.</pre>
    ///
    ///
    #[serde(rename = "mode")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub mode: crate::overlay::types::InspectMode,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.setInspectMode/parameters/highlightConfig。
    ///
    ///协议原文：
    ///
    ///<pre>A descriptor for the highlight appearance of hovered-over nodes. May be omitted if `enabled
    ///== false`.</pre>
    ///
    ///
    #[serde(rename = "highlightConfig")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub highlight_config: Option<crate::overlay::types::HighlightConfig>,
}
impl<'de> serde::Deserialize<'de> for SetInspectModeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetInspectModeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setInspectMode/parameters/mode。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Set an inspection mode.</pre>
                    ///
                    ///
                    #[serde(rename = "mode")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub mode: crate::overlay::types::InspectMode,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.setInspectMode/parameters/highlightConfig。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A descriptor for the highlight appearance of hovered-over nodes. May be omitted if `enabled
                    ///== false`.</pre>
                    ///
                    ///
                    #[serde(rename = "highlightConfig")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub highlight_config: Option<crate::overlay::types::HighlightConfig>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetInspectModeParams {
                    mode: fields.mode,
                    highlight_config: fields.highlight_config,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.setInspectMode。
///
///协议原文：
///
///<pre>Enters the 'inspect' mode. In this mode, elements that user is hovering over are highlighted.
///Backend then generates 'inspectNodeRequested' event upon element selection.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetInspectMode;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setInspectMode/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetInspectModeResponse {}
impl<'de> serde::Deserialize<'de> for SetInspectModeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetInspectModeResponse;
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
                Ok(SetInspectModeResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetInspectMode {
    type Params = SetInspectModeParams;
    type Response = SetInspectModeResponse;
    const METHOD: &'static str = "Overlay.setInspectMode";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setPausedInDebuggerMessage/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct SetPausedInDebuggerMessageParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.setPausedInDebuggerMessage/parameters/message。
    ///
    ///协议原文：
    ///
    ///<pre>The message to display, also triggers resume and step over controls.</pre>
    ///
    ///
    #[serde(rename = "message")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub message: Option<String>,
}
impl<'de> serde::Deserialize<'de> for SetPausedInDebuggerMessageParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetPausedInDebuggerMessageParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.setPausedInDebuggerMessage/parameters/message。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The message to display, also triggers resume and step over controls.</pre>
                    ///
                    ///
                    #[serde(rename = "message")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub message: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetPausedInDebuggerMessageParams {
                    message: fields.message,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.setPausedInDebuggerMessage。
#[derive(Debug, Clone, Copy)]
pub struct SetPausedInDebuggerMessage;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setPausedInDebuggerMessage/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetPausedInDebuggerMessageResponse {}
impl<'de> serde::Deserialize<'de> for SetPausedInDebuggerMessageResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetPausedInDebuggerMessageResponse;
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
                Ok(SetPausedInDebuggerMessageResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetPausedInDebuggerMessage {
    type Params = SetPausedInDebuggerMessageParams;
    type Response = SetPausedInDebuggerMessageResponse;
    const METHOD: &'static str = "Overlay.setPausedInDebuggerMessage";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowAdHighlights/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowAdHighlightsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowAdHighlights/parameters/show。
    ///
    ///协议原文：
    ///
    ///<pre>True for showing ad highlights</pre>
    ///
    ///
    #[serde(rename = "show")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub show: bool,
}
impl<'de> serde::Deserialize<'de> for SetShowAdHighlightsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowAdHighlightsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowAdHighlights/parameters/show。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True for showing ad highlights</pre>
                    ///
                    ///
                    #[serde(rename = "show")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub show: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetShowAdHighlightsParams { show: fields.show })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.setShowAdHighlights。
///
///协议原文：
///
///<pre>Highlights owner element of all frames detected to be ads.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetShowAdHighlights;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowAdHighlights/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowAdHighlightsResponse {}
impl<'de> serde::Deserialize<'de> for SetShowAdHighlightsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowAdHighlightsResponse;
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
                Ok(SetShowAdHighlightsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetShowAdHighlights {
    type Params = SetShowAdHighlightsParams;
    type Response = SetShowAdHighlightsResponse;
    const METHOD: &'static str = "Overlay.setShowAdHighlights";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowContainerQueryOverlays/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowContainerQueryOverlaysParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowContainerQueryOverlays/parameters/containerQueryHighlightConfigs。
    ///
    ///协议原文：
    ///
    ///<pre>An array of node identifiers and descriptors for the highlight appearance.</pre>
    ///
    ///
    #[serde(rename = "containerQueryHighlightConfigs")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub container_query_highlight_configs:
        Vec<crate::overlay::types::ContainerQueryHighlightConfig>,
}
impl<'de> serde::Deserialize<'de> for SetShowContainerQueryOverlaysParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowContainerQueryOverlaysParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowContainerQueryOverlays/parameters/containerQueryHighlightConfigs。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An array of node identifiers and descriptors for the highlight appearance.</pre>
                    ///
                    ///
                    #[serde(rename = "containerQueryHighlightConfigs")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub container_query_highlight_configs:
                        Vec<crate::overlay::types::ContainerQueryHighlightConfig>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetShowContainerQueryOverlaysParams {
                    container_query_highlight_configs: fields.container_query_highlight_configs,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.setShowContainerQueryOverlays。
#[derive(Debug, Clone, Copy)]
pub struct SetShowContainerQueryOverlays;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowContainerQueryOverlays/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowContainerQueryOverlaysResponse {}
impl<'de> serde::Deserialize<'de> for SetShowContainerQueryOverlaysResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowContainerQueryOverlaysResponse;
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
                Ok(SetShowContainerQueryOverlaysResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetShowContainerQueryOverlays {
    type Params = SetShowContainerQueryOverlaysParams;
    type Response = SetShowContainerQueryOverlaysResponse;
    const METHOD: &'static str = "Overlay.setShowContainerQueryOverlays";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowDebugBorders/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowDebugBordersParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowDebugBorders/parameters/show。
    ///
    ///协议原文：
    ///
    ///<pre>True for showing debug borders</pre>
    ///
    ///
    #[serde(rename = "show")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub show: bool,
}
impl<'de> serde::Deserialize<'de> for SetShowDebugBordersParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowDebugBordersParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowDebugBorders/parameters/show。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True for showing debug borders</pre>
                    ///
                    ///
                    #[serde(rename = "show")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub show: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetShowDebugBordersParams { show: fields.show })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.setShowDebugBorders。
///
///协议原文：
///
///<pre>Requests that backend shows debug borders on layers</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetShowDebugBorders;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowDebugBorders/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowDebugBordersResponse {}
impl<'de> serde::Deserialize<'de> for SetShowDebugBordersResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowDebugBordersResponse;
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
                Ok(SetShowDebugBordersResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetShowDebugBorders {
    type Params = SetShowDebugBordersParams;
    type Response = SetShowDebugBordersResponse;
    const METHOD: &'static str = "Overlay.setShowDebugBorders";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowDisplayCutout/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct SetShowDisplayCutoutParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.setShowDisplayCutout/parameters/displayCutoutConfig。
    ///
    ///协议原文：
    ///
    ///<pre>display cutout data, null means hide display cutout</pre>
    ///
    ///
    #[serde(rename = "displayCutoutConfig")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_cutout_config: Option<crate::overlay::types::DisplayCutoutConfig>,
}
impl<'de> serde::Deserialize<'de> for SetShowDisplayCutoutParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowDisplayCutoutParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.setShowDisplayCutout/parameters/displayCutoutConfig。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>display cutout data, null means hide display cutout</pre>
                    ///
                    ///
                    #[serde(rename = "displayCutoutConfig")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub display_cutout_config: Option<crate::overlay::types::DisplayCutoutConfig>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetShowDisplayCutoutParams {
                    display_cutout_config: fields.display_cutout_config,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.setShowDisplayCutout。
///
///协议原文：
///
///<pre>Add a display cutout overlay.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetShowDisplayCutout;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowDisplayCutout/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowDisplayCutoutResponse {}
impl<'de> serde::Deserialize<'de> for SetShowDisplayCutoutResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowDisplayCutoutResponse;
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
                Ok(SetShowDisplayCutoutResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetShowDisplayCutout {
    type Params = SetShowDisplayCutoutParams;
    type Response = SetShowDisplayCutoutResponse;
    const METHOD: &'static str = "Overlay.setShowDisplayCutout";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowFPSCounter/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowFpsCounterParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowFPSCounter/parameters/show。
    ///
    ///协议原文：
    ///
    ///<pre>True for showing the FPS counter</pre>
    ///
    ///
    #[serde(rename = "show")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub show: bool,
}
impl<'de> serde::Deserialize<'de> for SetShowFpsCounterParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowFpsCounterParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowFPSCounter/parameters/show。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True for showing the FPS counter</pre>
                    ///
                    ///
                    #[serde(rename = "show")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub show: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetShowFpsCounterParams { show: fields.show })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.setShowFPSCounter。
///
///协议原文：
///
///<pre>Requests that backend shows the FPS counter</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetShowFpsCounter;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowFPSCounter/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowFpsCounterResponse {}
impl<'de> serde::Deserialize<'de> for SetShowFpsCounterResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowFpsCounterResponse;
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
                Ok(SetShowFpsCounterResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetShowFpsCounter {
    type Params = SetShowFpsCounterParams;
    type Response = SetShowFpsCounterResponse;
    const METHOD: &'static str = "Overlay.setShowFPSCounter";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowFlexOverlays/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowFlexOverlaysParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowFlexOverlays/parameters/flexNodeHighlightConfigs。
    ///
    ///协议原文：
    ///
    ///<pre>An array of node identifiers and descriptors for the highlight appearance.</pre>
    ///
    ///
    #[serde(rename = "flexNodeHighlightConfigs")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub flex_node_highlight_configs: Vec<crate::overlay::types::FlexNodeHighlightConfig>,
}
impl<'de> serde::Deserialize<'de> for SetShowFlexOverlaysParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowFlexOverlaysParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowFlexOverlays/parameters/flexNodeHighlightConfigs。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An array of node identifiers and descriptors for the highlight appearance.</pre>
                    ///
                    ///
                    #[serde(rename = "flexNodeHighlightConfigs")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub flex_node_highlight_configs:
                        Vec<crate::overlay::types::FlexNodeHighlightConfig>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetShowFlexOverlaysParams {
                    flex_node_highlight_configs: fields.flex_node_highlight_configs,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.setShowFlexOverlays。
#[derive(Debug, Clone, Copy)]
pub struct SetShowFlexOverlays;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowFlexOverlays/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowFlexOverlaysResponse {}
impl<'de> serde::Deserialize<'de> for SetShowFlexOverlaysResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowFlexOverlaysResponse;
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
                Ok(SetShowFlexOverlaysResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetShowFlexOverlays {
    type Params = SetShowFlexOverlaysParams;
    type Response = SetShowFlexOverlaysResponse;
    const METHOD: &'static str = "Overlay.setShowFlexOverlays";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowGridOverlays/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowGridOverlaysParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowGridOverlays/parameters/gridNodeHighlightConfigs。
    ///
    ///协议原文：
    ///
    ///<pre>An array of node identifiers and descriptors for the highlight appearance.</pre>
    ///
    ///
    #[serde(rename = "gridNodeHighlightConfigs")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub grid_node_highlight_configs: Vec<crate::overlay::types::GridNodeHighlightConfig>,
}
impl<'de> serde::Deserialize<'de> for SetShowGridOverlaysParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowGridOverlaysParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowGridOverlays/parameters/gridNodeHighlightConfigs。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An array of node identifiers and descriptors for the highlight appearance.</pre>
                    ///
                    ///
                    #[serde(rename = "gridNodeHighlightConfigs")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub grid_node_highlight_configs:
                        Vec<crate::overlay::types::GridNodeHighlightConfig>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetShowGridOverlaysParams {
                    grid_node_highlight_configs: fields.grid_node_highlight_configs,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.setShowGridOverlays。
///
///协议原文：
///
///<pre>Highlight multiple elements with the CSS Grid overlay.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetShowGridOverlays;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowGridOverlays/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowGridOverlaysResponse {}
impl<'de> serde::Deserialize<'de> for SetShowGridOverlaysResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowGridOverlaysResponse;
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
                Ok(SetShowGridOverlaysResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetShowGridOverlays {
    type Params = SetShowGridOverlaysParams;
    type Response = SetShowGridOverlaysResponse;
    const METHOD: &'static str = "Overlay.setShowGridOverlays";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowHinge/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct SetShowHingeParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.setShowHinge/parameters/hingeConfig。
    ///
    ///协议原文：
    ///
    ///<pre>hinge data, null means hideHinge</pre>
    ///
    ///
    #[serde(rename = "hingeConfig")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub hinge_config: Option<crate::overlay::types::HingeConfig>,
}
impl<'de> serde::Deserialize<'de> for SetShowHingeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowHingeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.setShowHinge/parameters/hingeConfig。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>hinge data, null means hideHinge</pre>
                    ///
                    ///
                    #[serde(rename = "hingeConfig")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub hinge_config: Option<crate::overlay::types::HingeConfig>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetShowHingeParams {
                    hinge_config: fields.hinge_config,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.setShowHinge。
///
///协议原文：
///
///<pre>Add a dual screen device hinge</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetShowHinge;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowHinge/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowHingeResponse {}
impl<'de> serde::Deserialize<'de> for SetShowHingeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowHingeResponse;
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
                Ok(SetShowHingeResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetShowHinge {
    type Params = SetShowHingeParams;
    type Response = SetShowHingeResponse;
    const METHOD: &'static str = "Overlay.setShowHinge";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowHitTestBorders/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowHitTestBordersParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowHitTestBorders/parameters/show。
    ///
    ///协议原文：
    ///
    ///<pre>True for showing hit-test borders</pre>
    ///
    ///
    #[serde(rename = "show")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub show: bool,
}
impl<'de> serde::Deserialize<'de> for SetShowHitTestBordersParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowHitTestBordersParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowHitTestBorders/parameters/show。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True for showing hit-test borders</pre>
                    ///
                    ///
                    #[serde(rename = "show")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub show: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetShowHitTestBordersParams { show: fields.show })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.setShowHitTestBorders。
///
///协议原文：
///
///<pre>Deprecated, no longer has any effect.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct SetShowHitTestBorders;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowHitTestBorders/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowHitTestBordersResponse {}
impl<'de> serde::Deserialize<'de> for SetShowHitTestBordersResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowHitTestBordersResponse;
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
                Ok(SetShowHitTestBordersResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetShowHitTestBorders {
    type Params = SetShowHitTestBordersParams;
    type Response = SetShowHitTestBordersResponse;
    const METHOD: &'static str = "Overlay.setShowHitTestBorders";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowInspectedElementAnchor/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowInspectedElementAnchorParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowInspectedElementAnchor/parameters/inspectedElementAnchorConfig。
    ///
    ///协议原文：
    ///
    ///<pre>Node identifier for which to show an anchor for.</pre>
    ///
    ///
    #[serde(rename = "inspectedElementAnchorConfig")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub inspected_element_anchor_config: crate::overlay::types::InspectedElementAnchorConfig,
}
impl<'de> serde::Deserialize<'de> for SetShowInspectedElementAnchorParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowInspectedElementAnchorParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowInspectedElementAnchor/parameters/inspectedElementAnchorConfig。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Node identifier for which to show an anchor for.</pre>
                    ///
                    ///
                    #[serde(rename = "inspectedElementAnchorConfig")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub inspected_element_anchor_config:
                        crate::overlay::types::InspectedElementAnchorConfig,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetShowInspectedElementAnchorParams {
                    inspected_element_anchor_config: fields.inspected_element_anchor_config,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.setShowInspectedElementAnchor。
#[derive(Debug, Clone, Copy)]
pub struct SetShowInspectedElementAnchor;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowInspectedElementAnchor/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowInspectedElementAnchorResponse {}
impl<'de> serde::Deserialize<'de> for SetShowInspectedElementAnchorResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowInspectedElementAnchorResponse;
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
                Ok(SetShowInspectedElementAnchorResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetShowInspectedElementAnchor {
    type Params = SetShowInspectedElementAnchorParams;
    type Response = SetShowInspectedElementAnchorResponse;
    const METHOD: &'static str = "Overlay.setShowInspectedElementAnchor";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowIsolatedElements/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowIsolatedElementsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowIsolatedElements/parameters/isolatedElementHighlightConfigs。
    ///
    ///协议原文：
    ///
    ///<pre>An array of node identifiers and descriptors for the highlight appearance.</pre>
    ///
    ///
    #[serde(rename = "isolatedElementHighlightConfigs")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub isolated_element_highlight_configs:
        Vec<crate::overlay::types::IsolatedElementHighlightConfig>,
}
impl<'de> serde::Deserialize<'de> for SetShowIsolatedElementsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowIsolatedElementsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowIsolatedElements/parameters/isolatedElementHighlightConfigs。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An array of node identifiers and descriptors for the highlight appearance.</pre>
                    ///
                    ///
                    #[serde(rename = "isolatedElementHighlightConfigs")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub isolated_element_highlight_configs:
                        Vec<crate::overlay::types::IsolatedElementHighlightConfig>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetShowIsolatedElementsParams {
                    isolated_element_highlight_configs: fields.isolated_element_highlight_configs,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.setShowIsolatedElements。
///
///协议原文：
///
///<pre>Show elements in isolation mode with overlays.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetShowIsolatedElements;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowIsolatedElements/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowIsolatedElementsResponse {}
impl<'de> serde::Deserialize<'de> for SetShowIsolatedElementsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowIsolatedElementsResponse;
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
                Ok(SetShowIsolatedElementsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetShowIsolatedElements {
    type Params = SetShowIsolatedElementsParams;
    type Response = SetShowIsolatedElementsResponse;
    const METHOD: &'static str = "Overlay.setShowIsolatedElements";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowLayoutShiftRegions/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowLayoutShiftRegionsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowLayoutShiftRegions/parameters/result。
    ///
    ///协议原文：
    ///
    ///<pre>True for showing layout shift regions</pre>
    ///
    ///
    #[serde(rename = "result")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub result: bool,
}
impl<'de> serde::Deserialize<'de> for SetShowLayoutShiftRegionsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowLayoutShiftRegionsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowLayoutShiftRegions/parameters/result。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True for showing layout shift regions</pre>
                    ///
                    ///
                    #[serde(rename = "result")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub result: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetShowLayoutShiftRegionsParams {
                    result: fields.result,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.setShowLayoutShiftRegions。
///
///协议原文：
///
///<pre>Requests that backend shows layout shift regions</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetShowLayoutShiftRegions;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowLayoutShiftRegions/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowLayoutShiftRegionsResponse {}
impl<'de> serde::Deserialize<'de> for SetShowLayoutShiftRegionsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowLayoutShiftRegionsResponse;
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
                Ok(SetShowLayoutShiftRegionsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetShowLayoutShiftRegions {
    type Params = SetShowLayoutShiftRegionsParams;
    type Response = SetShowLayoutShiftRegionsResponse;
    const METHOD: &'static str = "Overlay.setShowLayoutShiftRegions";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowPaintRects/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowPaintRectsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowPaintRects/parameters/result。
    ///
    ///协议原文：
    ///
    ///<pre>True for showing paint rectangles</pre>
    ///
    ///
    #[serde(rename = "result")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub result: bool,
}
impl<'de> serde::Deserialize<'de> for SetShowPaintRectsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowPaintRectsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowPaintRects/parameters/result。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True for showing paint rectangles</pre>
                    ///
                    ///
                    #[serde(rename = "result")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub result: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetShowPaintRectsParams {
                    result: fields.result,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.setShowPaintRects。
///
///协议原文：
///
///<pre>Requests that backend shows paint rectangles</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetShowPaintRects;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowPaintRects/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowPaintRectsResponse {}
impl<'de> serde::Deserialize<'de> for SetShowPaintRectsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowPaintRectsResponse;
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
                Ok(SetShowPaintRectsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetShowPaintRects {
    type Params = SetShowPaintRectsParams;
    type Response = SetShowPaintRectsResponse;
    const METHOD: &'static str = "Overlay.setShowPaintRects";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowScrollBottleneckRects/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowScrollBottleneckRectsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowScrollBottleneckRects/parameters/show。
    ///
    ///协议原文：
    ///
    ///<pre>True for showing scroll bottleneck rects</pre>
    ///
    ///
    #[serde(rename = "show")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub show: bool,
}
impl<'de> serde::Deserialize<'de> for SetShowScrollBottleneckRectsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowScrollBottleneckRectsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowScrollBottleneckRects/parameters/show。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True for showing scroll bottleneck rects</pre>
                    ///
                    ///
                    #[serde(rename = "show")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub show: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetShowScrollBottleneckRectsParams { show: fields.show })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.setShowScrollBottleneckRects。
///
///协议原文：
///
///<pre>Requests that backend shows scroll bottleneck rects</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetShowScrollBottleneckRects;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowScrollBottleneckRects/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowScrollBottleneckRectsResponse {}
impl<'de> serde::Deserialize<'de> for SetShowScrollBottleneckRectsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowScrollBottleneckRectsResponse;
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
                Ok(SetShowScrollBottleneckRectsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetShowScrollBottleneckRects {
    type Params = SetShowScrollBottleneckRectsParams;
    type Response = SetShowScrollBottleneckRectsResponse;
    const METHOD: &'static str = "Overlay.setShowScrollBottleneckRects";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowScrollSnapOverlays/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowScrollSnapOverlaysParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowScrollSnapOverlays/parameters/scrollSnapHighlightConfigs。
    ///
    ///协议原文：
    ///
    ///<pre>An array of node identifiers and descriptors for the highlight appearance.</pre>
    ///
    ///
    #[serde(rename = "scrollSnapHighlightConfigs")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub scroll_snap_highlight_configs: Vec<crate::overlay::types::ScrollSnapHighlightConfig>,
}
impl<'de> serde::Deserialize<'de> for SetShowScrollSnapOverlaysParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowScrollSnapOverlaysParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowScrollSnapOverlays/parameters/scrollSnapHighlightConfigs。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An array of node identifiers and descriptors for the highlight appearance.</pre>
                    ///
                    ///
                    #[serde(rename = "scrollSnapHighlightConfigs")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub scroll_snap_highlight_configs:
                        Vec<crate::overlay::types::ScrollSnapHighlightConfig>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetShowScrollSnapOverlaysParams {
                    scroll_snap_highlight_configs: fields.scroll_snap_highlight_configs,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.setShowScrollSnapOverlays。
#[derive(Debug, Clone, Copy)]
pub struct SetShowScrollSnapOverlays;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowScrollSnapOverlays/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowScrollSnapOverlaysResponse {}
impl<'de> serde::Deserialize<'de> for SetShowScrollSnapOverlaysResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowScrollSnapOverlaysResponse;
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
                Ok(SetShowScrollSnapOverlaysResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetShowScrollSnapOverlays {
    type Params = SetShowScrollSnapOverlaysParams;
    type Response = SetShowScrollSnapOverlaysResponse;
    const METHOD: &'static str = "Overlay.setShowScrollSnapOverlays";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowViewportSizeOnResize/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowViewportSizeOnResizeParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowViewportSizeOnResize/parameters/show。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to paint size or not.</pre>
    ///
    ///
    #[serde(rename = "show")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub show: bool,
}
impl<'de> serde::Deserialize<'de> for SetShowViewportSizeOnResizeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowViewportSizeOnResizeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowViewportSizeOnResize/parameters/show。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to paint size or not.</pre>
                    ///
                    ///
                    #[serde(rename = "show")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub show: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetShowViewportSizeOnResizeParams { show: fields.show })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.setShowViewportSizeOnResize。
///
///协议原文：
///
///<pre>Paints viewport size upon main frame resize.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetShowViewportSizeOnResize;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowViewportSizeOnResize/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowViewportSizeOnResizeResponse {}
impl<'de> serde::Deserialize<'de> for SetShowViewportSizeOnResizeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowViewportSizeOnResizeResponse;
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
                Ok(SetShowViewportSizeOnResizeResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetShowViewportSizeOnResize {
    type Params = SetShowViewportSizeOnResizeParams;
    type Response = SetShowViewportSizeOnResizeResponse;
    const METHOD: &'static str = "Overlay.setShowViewportSizeOnResize";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowWebVitals/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowWebVitalsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowWebVitals/parameters/show。
    #[serde(rename = "show")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub show: bool,
}
impl<'de> serde::Deserialize<'de> for SetShowWebVitalsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowWebVitalsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.command.setShowWebVitals/parameters/show。
                    #[serde(rename = "show")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub show: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetShowWebVitalsParams { show: fields.show })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.setShowWebVitals。
///
///协议原文：
///
///<pre>Deprecated, no longer has any effect.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct SetShowWebVitals;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowWebVitals/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowWebVitalsResponse {}
impl<'de> serde::Deserialize<'de> for SetShowWebVitalsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowWebVitalsResponse;
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
                Ok(SetShowWebVitalsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetShowWebVitals {
    type Params = SetShowWebVitalsParams;
    type Response = SetShowWebVitalsResponse;
    const METHOD: &'static str = "Overlay.setShowWebVitals";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowWindowControlsOverlay/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct SetShowWindowControlsOverlayParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.setShowWindowControlsOverlay/parameters/windowControlsOverlayConfig。
    ///
    ///协议原文：
    ///
    ///<pre>Window Controls Overlay data, null means hide Window Controls Overlay</pre>
    ///
    ///
    #[serde(rename = "windowControlsOverlayConfig")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub window_controls_overlay_config: Option<crate::overlay::types::WindowControlsOverlayConfig>,
}
impl<'de> serde::Deserialize<'de> for SetShowWindowControlsOverlayParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowWindowControlsOverlayParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Overlay.command.setShowWindowControlsOverlay/parameters/windowControlsOverlayConfig。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Window Controls Overlay data, null means hide Window Controls Overlay</pre>
                    ///
                    ///
                    #[serde(rename = "windowControlsOverlayConfig")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub window_controls_overlay_config:
                        Option<crate::overlay::types::WindowControlsOverlayConfig>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetShowWindowControlsOverlayParams {
                    window_controls_overlay_config: fields.window_controls_overlay_config,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.command.setShowWindowControlsOverlay。
///
///协议原文：
///
///<pre>Show Window Controls Overlay for PWA</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetShowWindowControlsOverlay;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.command.setShowWindowControlsOverlay/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetShowWindowControlsOverlayResponse {}
impl<'de> serde::Deserialize<'de> for SetShowWindowControlsOverlayResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetShowWindowControlsOverlayResponse;
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
                Ok(SetShowWindowControlsOverlayResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetShowWindowControlsOverlay {
    type Params = SetShowWindowControlsOverlayParams;
    type Response = SetShowWindowControlsOverlayResponse;
    const METHOD: &'static str = "Overlay.setShowWindowControlsOverlay";
}
