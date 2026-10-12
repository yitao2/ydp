///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.command.addInspectedHeapObject/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddInspectedHeapObjectParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.command.addInspectedHeapObject/parameters/heapObjectId。
    ///
    ///协议原文：
    ///
    ///<pre>Heap snapshot object id to be accessible by means of $x command line API.</pre>
    ///
    ///
    #[serde(rename = "heapObjectId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub heap_object_id: crate::heap_profiler::types::HeapSnapshotObjectId,
}
impl<'de> serde::Deserialize<'de> for AddInspectedHeapObjectParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddInspectedHeapObjectParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.command.addInspectedHeapObject/parameters/heapObjectId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Heap snapshot object id to be accessible by means of $x command line API.</pre>
                    ///
                    ///
                    #[serde(rename = "heapObjectId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub heap_object_id: crate::heap_profiler::types::HeapSnapshotObjectId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AddInspectedHeapObjectParams {
                    heap_object_id: fields.heap_object_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：HeapProfiler.command.addInspectedHeapObject。
///
///协议原文：
///
///<pre>Enables console to refer to the node with given id via $x (see Command Line API for more details
///$x functions).</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct AddInspectedHeapObject;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.command.addInspectedHeapObject/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddInspectedHeapObjectResponse {}
impl<'de> serde::Deserialize<'de> for AddInspectedHeapObjectResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddInspectedHeapObjectResponse;
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
                Ok(AddInspectedHeapObjectResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for AddInspectedHeapObject {
    type Params = AddInspectedHeapObjectParams;
    type Response = AddInspectedHeapObjectResponse;
    const METHOD: &'static str = "HeapProfiler.addInspectedHeapObject";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.command.collectGarbage/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CollectGarbageParams {}
impl<'de> serde::Deserialize<'de> for CollectGarbageParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CollectGarbageParams;
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
                Ok(CollectGarbageParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：HeapProfiler.command.collectGarbage。
#[derive(Debug, Clone, Copy)]
pub struct CollectGarbage;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.command.collectGarbage/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CollectGarbageResponse {}
impl<'de> serde::Deserialize<'de> for CollectGarbageResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CollectGarbageResponse;
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
                Ok(CollectGarbageResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for CollectGarbage {
    type Params = CollectGarbageParams;
    type Response = CollectGarbageResponse;
    const METHOD: &'static str = "HeapProfiler.collectGarbage";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.command.disable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：HeapProfiler.command.disable。
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.command.disable/returns。
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
    const METHOD: &'static str = "HeapProfiler.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.command.enable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：HeapProfiler.command.enable。
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.command.enable/returns。
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
    const METHOD: &'static str = "HeapProfiler.enable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.command.getHeapObjectId/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetHeapObjectIdParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.command.getHeapObjectId/parameters/objectId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the object to get heap object id for.</pre>
    ///
    ///
    #[serde(rename = "objectId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub object_id: crate::runtime::types::RemoteObjectId,
}
impl<'de> serde::Deserialize<'de> for GetHeapObjectIdParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetHeapObjectIdParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.command.getHeapObjectId/parameters/objectId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the object to get heap object id for.</pre>
                    ///
                    ///
                    #[serde(rename = "objectId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub object_id: crate::runtime::types::RemoteObjectId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetHeapObjectIdParams {
                    object_id: fields.object_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：HeapProfiler.command.getHeapObjectId。
#[derive(Debug, Clone, Copy)]
pub struct GetHeapObjectId;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.command.getHeapObjectId/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetHeapObjectIdResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.command.getHeapObjectId/returns/heapSnapshotObjectId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the heap snapshot object corresponding to the passed remote object id.</pre>
    ///
    ///
    #[serde(rename = "heapSnapshotObjectId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub heap_snapshot_object_id: crate::heap_profiler::types::HeapSnapshotObjectId,
}
impl<'de> serde::Deserialize<'de> for GetHeapObjectIdResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetHeapObjectIdResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.command.getHeapObjectId/returns/heapSnapshotObjectId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the heap snapshot object corresponding to the passed remote object id.</pre>
                    ///
                    ///
                    #[serde(rename = "heapSnapshotObjectId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub heap_snapshot_object_id: crate::heap_profiler::types::HeapSnapshotObjectId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetHeapObjectIdResponse {
                    heap_snapshot_object_id: fields.heap_snapshot_object_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetHeapObjectId {
    type Params = GetHeapObjectIdParams;
    type Response = GetHeapObjectIdResponse;
    const METHOD: &'static str = "HeapProfiler.getHeapObjectId";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.command.getObjectByHeapObjectId/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetObjectByHeapObjectIdParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.command.getObjectByHeapObjectId/parameters/objectId。
    #[serde(rename = "objectId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub object_id: crate::heap_profiler::types::HeapSnapshotObjectId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.command.getObjectByHeapObjectId/parameters/objectGroup。
    ///
    ///协议原文：
    ///
    ///<pre>Symbolic group name that can be used to release multiple objects.</pre>
    ///
    ///
    #[serde(rename = "objectGroup")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub object_group: Option<String>,
}
impl<'de> serde::Deserialize<'de> for GetObjectByHeapObjectIdParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetObjectByHeapObjectIdParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.command.getObjectByHeapObjectId/parameters/objectId。
                    #[serde(rename = "objectId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub object_id: crate::heap_profiler::types::HeapSnapshotObjectId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.command.getObjectByHeapObjectId/parameters/objectGroup。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Symbolic group name that can be used to release multiple objects.</pre>
                    ///
                    ///
                    #[serde(rename = "objectGroup")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub object_group: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetObjectByHeapObjectIdParams {
                    object_id: fields.object_id,
                    object_group: fields.object_group,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：HeapProfiler.command.getObjectByHeapObjectId。
#[derive(Debug, Clone, Copy)]
pub struct GetObjectByHeapObjectId;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.command.getObjectByHeapObjectId/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetObjectByHeapObjectIdResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.command.getObjectByHeapObjectId/returns/result。
    ///
    ///协议原文：
    ///
    ///<pre>Evaluation result.</pre>
    ///
    ///
    #[serde(rename = "result")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub result: crate::runtime::types::RemoteObject,
}
impl<'de> serde::Deserialize<'de> for GetObjectByHeapObjectIdResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetObjectByHeapObjectIdResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.command.getObjectByHeapObjectId/returns/result。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Evaluation result.</pre>
                    ///
                    ///
                    #[serde(rename = "result")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub result: crate::runtime::types::RemoteObject,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetObjectByHeapObjectIdResponse {
                    result: fields.result,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetObjectByHeapObjectId {
    type Params = GetObjectByHeapObjectIdParams;
    type Response = GetObjectByHeapObjectIdResponse;
    const METHOD: &'static str = "HeapProfiler.getObjectByHeapObjectId";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.command.getSamplingProfile/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetSamplingProfileParams {}
impl<'de> serde::Deserialize<'de> for GetSamplingProfileParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetSamplingProfileParams;
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
                Ok(GetSamplingProfileParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：HeapProfiler.command.getSamplingProfile。
#[derive(Debug, Clone, Copy)]
pub struct GetSamplingProfile;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.command.getSamplingProfile/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetSamplingProfileResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.command.getSamplingProfile/returns/profile。
    ///
    ///协议原文：
    ///
    ///<pre>Return the sampling profile being collected.</pre>
    ///
    ///
    #[serde(rename = "profile")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub profile: crate::heap_profiler::types::SamplingHeapProfile,
}
impl<'de> serde::Deserialize<'de> for GetSamplingProfileResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetSamplingProfileResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.command.getSamplingProfile/returns/profile。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Return the sampling profile being collected.</pre>
                    ///
                    ///
                    #[serde(rename = "profile")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub profile: crate::heap_profiler::types::SamplingHeapProfile,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetSamplingProfileResponse {
                    profile: fields.profile,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetSamplingProfile {
    type Params = GetSamplingProfileParams;
    type Response = GetSamplingProfileResponse;
    const METHOD: &'static str = "HeapProfiler.getSamplingProfile";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.command.startSampling/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct StartSamplingParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.command.startSampling/parameters/samplingInterval。
    ///
    ///协议原文：
    ///
    ///<pre>Average sample interval in bytes. Poisson distribution is used for the intervals. The
    ///default value is 32768 bytes.</pre>
    ///
    ///
    #[serde(rename = "samplingInterval")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub sampling_interval: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.command.startSampling/parameters/stackDepth。
    ///
    ///协议原文：
    ///
    ///<pre>Maximum stack depth. The default value is 128.</pre>
    ///
    ///
    #[serde(rename = "stackDepth")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub stack_depth: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.command.startSampling/parameters/includeObjectsCollectedByMajorGC。
    ///
    ///协议原文：
    ///
    ///<pre>By default, the sampling heap profiler reports only objects which are
    ///still alive when the profile is returned via getSamplingProfile or
    ///stopSampling, which is useful for determining what functions contribute
    ///the most to steady-state memory usage. This flag instructs the sampling
    ///heap profiler to also include information about objects discarded by
    ///major GC, which will show which functions cause large temporary memory
    ///usage or long GC pauses.</pre>
    ///
    ///
    #[serde(rename = "includeObjectsCollectedByMajorGC")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_objects_collected_by_major_gc: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.command.startSampling/parameters/includeObjectsCollectedByMinorGC。
    ///
    ///协议原文：
    ///
    ///<pre>By default, the sampling heap profiler reports only objects which are
    ///still alive when the profile is returned via getSamplingProfile or
    ///stopSampling, which is useful for determining what functions contribute
    ///the most to steady-state memory usage. This flag instructs the sampling
    ///heap profiler to also include information about objects discarded by
    ///minor GC, which is useful when tuning a latency-sensitive application
    ///for minimal GC activity.</pre>
    ///
    ///
    #[serde(rename = "includeObjectsCollectedByMinorGC")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_objects_collected_by_minor_gc: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for StartSamplingParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StartSamplingParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.command.startSampling/parameters/samplingInterval。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Average sample interval in bytes. Poisson distribution is used for the intervals. The
                    ///default value is 32768 bytes.</pre>
                    ///
                    ///
                    #[serde(rename = "samplingInterval")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub sampling_interval: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.command.startSampling/parameters/stackDepth。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Maximum stack depth. The default value is 128.</pre>
                    ///
                    ///
                    #[serde(rename = "stackDepth")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub stack_depth: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.command.startSampling/parameters/includeObjectsCollectedByMajorGC。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>By default, the sampling heap profiler reports only objects which are
                    ///still alive when the profile is returned via getSamplingProfile or
                    ///stopSampling, which is useful for determining what functions contribute
                    ///the most to steady-state memory usage. This flag instructs the sampling
                    ///heap profiler to also include information about objects discarded by
                    ///major GC, which will show which functions cause large temporary memory
                    ///usage or long GC pauses.</pre>
                    ///
                    ///
                    #[serde(rename = "includeObjectsCollectedByMajorGC")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub include_objects_collected_by_major_gc: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.command.startSampling/parameters/includeObjectsCollectedByMinorGC。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>By default, the sampling heap profiler reports only objects which are
                    ///still alive when the profile is returned via getSamplingProfile or
                    ///stopSampling, which is useful for determining what functions contribute
                    ///the most to steady-state memory usage. This flag instructs the sampling
                    ///heap profiler to also include information about objects discarded by
                    ///minor GC, which is useful when tuning a latency-sensitive application
                    ///for minimal GC activity.</pre>
                    ///
                    ///
                    #[serde(rename = "includeObjectsCollectedByMinorGC")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub include_objects_collected_by_minor_gc: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StartSamplingParams {
                    sampling_interval: fields.sampling_interval,
                    stack_depth: fields.stack_depth,
                    include_objects_collected_by_major_gc: fields
                        .include_objects_collected_by_major_gc,
                    include_objects_collected_by_minor_gc: fields
                        .include_objects_collected_by_minor_gc,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：HeapProfiler.command.startSampling。
#[derive(Debug, Clone, Copy)]
pub struct StartSampling;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.command.startSampling/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StartSamplingResponse {}
impl<'de> serde::Deserialize<'de> for StartSamplingResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StartSamplingResponse;
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
                Ok(StartSamplingResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for StartSampling {
    type Params = StartSamplingParams;
    type Response = StartSamplingResponse;
    const METHOD: &'static str = "HeapProfiler.startSampling";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.command.startTrackingHeapObjects/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct StartTrackingHeapObjectsParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.command.startTrackingHeapObjects/parameters/trackAllocations。
    #[serde(rename = "trackAllocations")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub track_allocations: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for StartTrackingHeapObjectsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StartTrackingHeapObjectsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.command.startTrackingHeapObjects/parameters/trackAllocations。
                    #[serde(rename = "trackAllocations")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub track_allocations: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StartTrackingHeapObjectsParams {
                    track_allocations: fields.track_allocations,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：HeapProfiler.command.startTrackingHeapObjects。
#[derive(Debug, Clone, Copy)]
pub struct StartTrackingHeapObjects;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.command.startTrackingHeapObjects/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StartTrackingHeapObjectsResponse {}
impl<'de> serde::Deserialize<'de> for StartTrackingHeapObjectsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StartTrackingHeapObjectsResponse;
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
                Ok(StartTrackingHeapObjectsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for StartTrackingHeapObjects {
    type Params = StartTrackingHeapObjectsParams;
    type Response = StartTrackingHeapObjectsResponse;
    const METHOD: &'static str = "HeapProfiler.startTrackingHeapObjects";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.command.stopSampling/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct StopSamplingParams {}
impl<'de> serde::Deserialize<'de> for StopSamplingParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StopSamplingParams;
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
                Ok(StopSamplingParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：HeapProfiler.command.stopSampling。
#[derive(Debug, Clone, Copy)]
pub struct StopSampling;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.command.stopSampling/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StopSamplingResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.command.stopSampling/returns/profile。
    ///
    ///协议原文：
    ///
    ///<pre>Recorded sampling heap profile.</pre>
    ///
    ///
    #[serde(rename = "profile")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub profile: crate::heap_profiler::types::SamplingHeapProfile,
}
impl<'de> serde::Deserialize<'de> for StopSamplingResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StopSamplingResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.command.stopSampling/returns/profile。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Recorded sampling heap profile.</pre>
                    ///
                    ///
                    #[serde(rename = "profile")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub profile: crate::heap_profiler::types::SamplingHeapProfile,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StopSamplingResponse {
                    profile: fields.profile,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for StopSampling {
    type Params = StopSamplingParams;
    type Response = StopSamplingResponse;
    const METHOD: &'static str = "HeapProfiler.stopSampling";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.command.stopTrackingHeapObjects/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct StopTrackingHeapObjectsParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.command.stopTrackingHeapObjects/parameters/reportProgress。
    ///
    ///协议原文：
    ///
    ///<pre>If true 'reportHeapSnapshotProgress' events will be generated while snapshot is being taken
    ///when the tracking is stopped.</pre>
    ///
    ///
    #[serde(rename = "reportProgress")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub report_progress: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.command.stopTrackingHeapObjects/parameters/treatGlobalObjectsAsRoots。
    ///
    ///协议原文：
    ///
    ///<pre>Deprecated in favor of `exposeInternals`.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "treatGlobalObjectsAsRoots")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub treat_global_objects_as_roots: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.command.stopTrackingHeapObjects/parameters/captureNumericValue。
    ///
    ///协议原文：
    ///
    ///<pre>If true, numerical values are included in the snapshot</pre>
    ///
    ///
    #[serde(rename = "captureNumericValue")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub capture_numeric_value: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.command.stopTrackingHeapObjects/parameters/exposeInternals。
    ///
    ///协议原文：
    ///
    ///<pre>If true, exposes internals of the snapshot.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "exposeInternals")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub expose_internals: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for StopTrackingHeapObjectsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StopTrackingHeapObjectsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.command.stopTrackingHeapObjects/parameters/reportProgress。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true 'reportHeapSnapshotProgress' events will be generated while snapshot is being taken
                    ///when the tracking is stopped.</pre>
                    ///
                    ///
                    #[serde(rename = "reportProgress")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub report_progress: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.command.stopTrackingHeapObjects/parameters/treatGlobalObjectsAsRoots。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Deprecated in favor of `exposeInternals`.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "treatGlobalObjectsAsRoots")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub treat_global_objects_as_roots: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.command.stopTrackingHeapObjects/parameters/captureNumericValue。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, numerical values are included in the snapshot</pre>
                    ///
                    ///
                    #[serde(rename = "captureNumericValue")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub capture_numeric_value: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.command.stopTrackingHeapObjects/parameters/exposeInternals。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, exposes internals of the snapshot.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "exposeInternals")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub expose_internals: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StopTrackingHeapObjectsParams {
                    report_progress: fields.report_progress,
                    treat_global_objects_as_roots: fields.treat_global_objects_as_roots,
                    capture_numeric_value: fields.capture_numeric_value,
                    expose_internals: fields.expose_internals,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：HeapProfiler.command.stopTrackingHeapObjects。
#[derive(Debug, Clone, Copy)]
pub struct StopTrackingHeapObjects;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.command.stopTrackingHeapObjects/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StopTrackingHeapObjectsResponse {}
impl<'de> serde::Deserialize<'de> for StopTrackingHeapObjectsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StopTrackingHeapObjectsResponse;
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
                Ok(StopTrackingHeapObjectsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for StopTrackingHeapObjects {
    type Params = StopTrackingHeapObjectsParams;
    type Response = StopTrackingHeapObjectsResponse;
    const METHOD: &'static str = "HeapProfiler.stopTrackingHeapObjects";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.command.takeHeapSnapshot/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct TakeHeapSnapshotParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.command.takeHeapSnapshot/parameters/reportProgress。
    ///
    ///协议原文：
    ///
    ///<pre>If true 'reportHeapSnapshotProgress' events will be generated while snapshot is being taken.</pre>
    ///
    ///
    #[serde(rename = "reportProgress")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub report_progress: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.command.takeHeapSnapshot/parameters/treatGlobalObjectsAsRoots。
    ///
    ///协议原文：
    ///
    ///<pre>If true, a raw snapshot without artificial roots will be generated.
    ///Deprecated in favor of `exposeInternals`.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "treatGlobalObjectsAsRoots")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub treat_global_objects_as_roots: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.command.takeHeapSnapshot/parameters/captureNumericValue。
    ///
    ///协议原文：
    ///
    ///<pre>If true, numerical values are included in the snapshot</pre>
    ///
    ///
    #[serde(rename = "captureNumericValue")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub capture_numeric_value: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.command.takeHeapSnapshot/parameters/exposeInternals。
    ///
    ///协议原文：
    ///
    ///<pre>If true, exposes internals of the snapshot.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "exposeInternals")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub expose_internals: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for TakeHeapSnapshotParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TakeHeapSnapshotParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.command.takeHeapSnapshot/parameters/reportProgress。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true 'reportHeapSnapshotProgress' events will be generated while snapshot is being taken.</pre>
                    ///
                    ///
                    #[serde(rename = "reportProgress")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub report_progress: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.command.takeHeapSnapshot/parameters/treatGlobalObjectsAsRoots。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, a raw snapshot without artificial roots will be generated.
                    ///Deprecated in favor of `exposeInternals`.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "treatGlobalObjectsAsRoots")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub treat_global_objects_as_roots: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.command.takeHeapSnapshot/parameters/captureNumericValue。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, numerical values are included in the snapshot</pre>
                    ///
                    ///
                    #[serde(rename = "captureNumericValue")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub capture_numeric_value: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.command.takeHeapSnapshot/parameters/exposeInternals。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, exposes internals of the snapshot.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "exposeInternals")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub expose_internals: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TakeHeapSnapshotParams {
                    report_progress: fields.report_progress,
                    treat_global_objects_as_roots: fields.treat_global_objects_as_roots,
                    capture_numeric_value: fields.capture_numeric_value,
                    expose_internals: fields.expose_internals,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：HeapProfiler.command.takeHeapSnapshot。
#[derive(Debug, Clone, Copy)]
pub struct TakeHeapSnapshot;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.command.takeHeapSnapshot/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TakeHeapSnapshotResponse {}
impl<'de> serde::Deserialize<'de> for TakeHeapSnapshotResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TakeHeapSnapshotResponse;
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
                Ok(TakeHeapSnapshotResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for TakeHeapSnapshot {
    type Params = TakeHeapSnapshotParams;
    type Response = TakeHeapSnapshotResponse;
    const METHOD: &'static str = "HeapProfiler.takeHeapSnapshot";
}
