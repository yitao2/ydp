///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：HeapProfiler.type.HeapSnapshotObjectId。
///
///协议原文：
///
///<pre>Heap snapshot object id.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct HeapSnapshotObjectId(String);
impl HeapSnapshotObjectId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::heap_profiler::types::HeapSnapshotObjectId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::heap_profiler::types::HeapSnapshotObjectId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::heap_profiler::types::HeapSnapshotObjectId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::heap_profiler::types::HeapSnapshotObjectId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::heap_profiler::types::HeapSnapshotObjectId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.type.SamplingHeapProfile。
///
///协议原文：
///
///<pre>Sampling profile.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct SamplingHeapProfile {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.type.SamplingHeapProfile/properties/head。
    #[serde(rename = "head")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub head: crate::heap_profiler::types::SamplingHeapProfileNode,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.type.SamplingHeapProfile/properties/samples。
    #[serde(rename = "samples")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub samples: Vec<crate::heap_profiler::types::SamplingHeapProfileSample>,
}
impl<'de> serde::Deserialize<'de> for SamplingHeapProfile {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SamplingHeapProfile;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.type.SamplingHeapProfile/properties/head。
                    #[serde(rename = "head")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub head: crate::heap_profiler::types::SamplingHeapProfileNode,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.type.SamplingHeapProfile/properties/samples。
                    #[serde(rename = "samples")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub samples: Vec<crate::heap_profiler::types::SamplingHeapProfileSample>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SamplingHeapProfile {
                    head: fields.head,
                    samples: fields.samples,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.type.SamplingHeapProfileNode。
///
///协议原文：
///
///<pre>Sampling Heap Profile node. Holds callsite information, allocation statistics and child nodes.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct SamplingHeapProfileNode {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.type.SamplingHeapProfileNode/properties/callFrame。
    ///
    ///协议原文：
    ///
    ///<pre>Function location.</pre>
    ///
    ///
    #[serde(rename = "callFrame")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub call_frame: crate::runtime::types::CallFrame,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.type.SamplingHeapProfileNode/properties/selfSize。
    ///
    ///协议原文：
    ///
    ///<pre>Allocations size in bytes for the node excluding children.</pre>
    ///
    ///
    #[serde(rename = "selfSize")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub self_size: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.type.SamplingHeapProfileNode/properties/id。
    ///
    ///协议原文：
    ///
    ///<pre>Node id. Ids are unique across all profiles collected between startSampling and stopSampling.</pre>
    ///
    ///
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.type.SamplingHeapProfileNode/properties/children。
    ///
    ///协议原文：
    ///
    ///<pre>Child nodes.</pre>
    ///
    ///
    #[serde(rename = "children")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub children: Vec<crate::heap_profiler::types::SamplingHeapProfileNode>,
}
impl<'de> serde::Deserialize<'de> for SamplingHeapProfileNode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SamplingHeapProfileNode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.type.SamplingHeapProfileNode/properties/callFrame。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Function location.</pre>
                    ///
                    ///
                    #[serde(rename = "callFrame")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub call_frame: crate::runtime::types::CallFrame,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.type.SamplingHeapProfileNode/properties/selfSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Allocations size in bytes for the node excluding children.</pre>
                    ///
                    ///
                    #[serde(rename = "selfSize")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub self_size: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.type.SamplingHeapProfileNode/properties/id。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Node id. Ids are unique across all profiles collected between startSampling and stopSampling.</pre>
                    ///
                    ///
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.type.SamplingHeapProfileNode/properties/children。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Child nodes.</pre>
                    ///
                    ///
                    #[serde(rename = "children")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub children: Vec<crate::heap_profiler::types::SamplingHeapProfileNode>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SamplingHeapProfileNode {
                    call_frame: fields.call_frame,
                    self_size: fields.self_size,
                    id: fields.id,
                    children: fields.children,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.type.SamplingHeapProfileSample。
///
///协议原文：
///
///<pre>A single sample from a sampling profile.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct SamplingHeapProfileSample {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.type.SamplingHeapProfileSample/properties/size。
    ///
    ///协议原文：
    ///
    ///<pre>Allocation size in bytes attributed to the sample.</pre>
    ///
    ///
    #[serde(rename = "size")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub size: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.type.SamplingHeapProfileSample/properties/nodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the corresponding profile tree node.</pre>
    ///
    ///
    #[serde(rename = "nodeId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub node_id: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.type.SamplingHeapProfileSample/properties/ordinal。
    ///
    ///协议原文：
    ///
    ///<pre>Time-ordered sample ordinal number. It is unique across all profiles retrieved
    ///between startSampling and stopSampling.</pre>
    ///
    ///
    #[serde(rename = "ordinal")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub ordinal: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for SamplingHeapProfileSample {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SamplingHeapProfileSample;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.type.SamplingHeapProfileSample/properties/size。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Allocation size in bytes attributed to the sample.</pre>
                    ///
                    ///
                    #[serde(rename = "size")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub size: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.type.SamplingHeapProfileSample/properties/nodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the corresponding profile tree node.</pre>
                    ///
                    ///
                    #[serde(rename = "nodeId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub node_id: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.type.SamplingHeapProfileSample/properties/ordinal。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Time-ordered sample ordinal number. It is unique across all profiles retrieved
                    ///between startSampling and stopSampling.</pre>
                    ///
                    ///
                    #[serde(rename = "ordinal")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub ordinal: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SamplingHeapProfileSample {
                    size: fields.size,
                    node_id: fields.node_id,
                    ordinal: fields.ordinal,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
