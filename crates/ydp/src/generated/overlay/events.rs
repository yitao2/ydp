///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.event.inspectModeCanceled/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct InspectModeCanceledParams {}
impl<'de> serde::Deserialize<'de> for InspectModeCanceledParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InspectModeCanceledParams;
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
                Ok(InspectModeCanceledParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.event.inspectModeCanceled。
///
///协议原文：
///
///<pre>Fired when user cancels the inspect mode.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct InspectModeCanceled;
impl ydp_core::Event for InspectModeCanceled {
    type Params = InspectModeCanceledParams;
    const METHOD: &'static str = "Overlay.inspectModeCanceled";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.event.inspectNodeRequested/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct InspectNodeRequestedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.event.inspectNodeRequested/parameters/backendNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node to inspect.</pre>
    ///
    ///
    #[serde(rename = "backendNodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub backend_node_id: crate::dom::types::BackendNodeId,
}
impl<'de> serde::Deserialize<'de> for InspectNodeRequestedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InspectNodeRequestedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.event.inspectNodeRequested/parameters/backendNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node to inspect.</pre>
                    ///
                    ///
                    #[serde(rename = "backendNodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub backend_node_id: crate::dom::types::BackendNodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(InspectNodeRequestedParams {
                    backend_node_id: fields.backend_node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.event.inspectNodeRequested。
///
///协议原文：
///
///<pre>Fired when the node should be inspected. This happens after call to `setInspectMode` or when
///user manually inspects an element.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct InspectNodeRequested;
impl ydp_core::Event for InspectNodeRequested {
    type Params = InspectNodeRequestedParams;
    const METHOD: &'static str = "Overlay.inspectNodeRequested";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.event.inspectPanelShowRequested/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct InspectPanelShowRequestedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.event.inspectPanelShowRequested/parameters/backendNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node to show in the panel.</pre>
    ///
    ///
    #[serde(rename = "backendNodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub backend_node_id: crate::dom::types::BackendNodeId,
}
impl<'de> serde::Deserialize<'de> for InspectPanelShowRequestedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InspectPanelShowRequestedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.event.inspectPanelShowRequested/parameters/backendNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node to show in the panel.</pre>
                    ///
                    ///
                    #[serde(rename = "backendNodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub backend_node_id: crate::dom::types::BackendNodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(InspectPanelShowRequestedParams {
                    backend_node_id: fields.backend_node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.event.inspectPanelShowRequested。
///
///协议原文：
///
///<pre>Fired when user asks to show the Inspect panel.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct InspectPanelShowRequested;
impl ydp_core::Event for InspectPanelShowRequested {
    type Params = InspectPanelShowRequestedParams;
    const METHOD: &'static str = "Overlay.inspectPanelShowRequested";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.event.inspectedElementWindowRestored/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct InspectedElementWindowRestoredParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.event.inspectedElementWindowRestored/parameters/backendNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the node to restore the floating window for.</pre>
    ///
    ///
    #[serde(rename = "backendNodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub backend_node_id: crate::dom::types::BackendNodeId,
}
impl<'de> serde::Deserialize<'de> for InspectedElementWindowRestoredParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InspectedElementWindowRestoredParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.event.inspectedElementWindowRestored/parameters/backendNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the node to restore the floating window for.</pre>
                    ///
                    ///
                    #[serde(rename = "backendNodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub backend_node_id: crate::dom::types::BackendNodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(InspectedElementWindowRestoredParams {
                    backend_node_id: fields.backend_node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.event.inspectedElementWindowRestored。
///
///协议原文：
///
///<pre>Fired when user asks to restore the Inspected Element floating window.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct InspectedElementWindowRestored;
impl ydp_core::Event for InspectedElementWindowRestored {
    type Params = InspectedElementWindowRestoredParams;
    const METHOD: &'static str = "Overlay.inspectedElementWindowRestored";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.event.nodeHighlightRequested/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeHighlightRequestedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.event.nodeHighlightRequested/parameters/nodeId。
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: crate::dom::types::NodeId,
}
impl<'de> serde::Deserialize<'de> for NodeHighlightRequestedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NodeHighlightRequestedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.event.nodeHighlightRequested/parameters/nodeId。
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: crate::dom::types::NodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(NodeHighlightRequestedParams {
                    node_id: fields.node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.event.nodeHighlightRequested。
///
///协议原文：
///
///<pre>Fired when the node should be highlighted. This happens after call to `setInspectMode`.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct NodeHighlightRequested;
impl ydp_core::Event for NodeHighlightRequested {
    type Params = NodeHighlightRequestedParams;
    const METHOD: &'static str = "Overlay.nodeHighlightRequested";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Overlay.event.screenshotRequested/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ScreenshotRequestedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.event.screenshotRequested/parameters/viewport。
    ///
    ///协议原文：
    ///
    ///<pre>Viewport to capture, in device independent pixels (dip).</pre>
    ///
    ///
    #[serde(rename = "viewport")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub viewport: crate::page::types::Viewport,
}
impl<'de> serde::Deserialize<'de> for ScreenshotRequestedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScreenshotRequestedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Overlay.event.screenshotRequested/parameters/viewport。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Viewport to capture, in device independent pixels (dip).</pre>
                    ///
                    ///
                    #[serde(rename = "viewport")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub viewport: crate::page::types::Viewport,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ScreenshotRequestedParams {
                    viewport: fields.viewport,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Overlay.event.screenshotRequested。
///
///协议原文：
///
///<pre>Fired when user asks to capture screenshot of some area on the page.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ScreenshotRequested;
impl ydp_core::Event for ScreenshotRequested {
    type Params = ScreenshotRequestedParams;
    const METHOD: &'static str = "Overlay.screenshotRequested";
}
