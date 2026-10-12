///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Accessibility.event.loadComplete/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct LoadCompleteParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.event.loadComplete/parameters/root。
    ///
    ///协议原文：
    ///
    ///<pre>New document root node.</pre>
    ///
    ///
    #[serde(rename = "root")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub root: crate::accessibility::types::AxNode,
}
impl<'de> serde::Deserialize<'de> for LoadCompleteParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LoadCompleteParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.event.loadComplete/parameters/root。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>New document root node.</pre>
                    ///
                    ///
                    #[serde(rename = "root")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub root: crate::accessibility::types::AxNode,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LoadCompleteParams { root: fields.root })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Accessibility.event.loadComplete。
///
///协议原文：
///
///<pre>The loadComplete event mirrors the load complete event sent by the browser to assistive
///technology when the web page has finished loading.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct LoadComplete;
impl ydp_core::Event for LoadComplete {
    type Params = LoadCompleteParams;
    const METHOD: &'static str = "Accessibility.loadComplete";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Accessibility.event.nodesUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodesUpdatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.event.nodesUpdated/parameters/nodes。
    ///
    ///协议原文：
    ///
    ///<pre>Updated node data.</pre>
    ///
    ///
    #[serde(rename = "nodes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub nodes: Vec<crate::accessibility::types::AxNode>,
}
impl<'de> serde::Deserialize<'de> for NodesUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NodesUpdatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Accessibility.event.nodesUpdated/parameters/nodes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Updated node data.</pre>
                    ///
                    ///
                    #[serde(rename = "nodes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub nodes: Vec<crate::accessibility::types::AxNode>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(NodesUpdatedParams {
                    nodes: fields.nodes,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Accessibility.event.nodesUpdated。
///
///协议原文：
///
///<pre>The nodesUpdated event is sent every time a previously requested node has changed the in tree.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct NodesUpdated;
impl ydp_core::Event for NodesUpdated {
    type Params = NodesUpdatedParams;
    const METHOD: &'static str = "Accessibility.nodesUpdated";
}
