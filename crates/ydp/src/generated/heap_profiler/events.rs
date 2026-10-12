///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.event.addHeapSnapshotChunk/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddHeapSnapshotChunkParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.event.addHeapSnapshotChunk/parameters/chunk。
    #[serde(rename = "chunk")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub chunk: String,
}
impl<'de> serde::Deserialize<'de> for AddHeapSnapshotChunkParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddHeapSnapshotChunkParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.event.addHeapSnapshotChunk/parameters/chunk。
                    #[serde(rename = "chunk")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub chunk: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AddHeapSnapshotChunkParams {
                    chunk: fields.chunk,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：HeapProfiler.event.addHeapSnapshotChunk。
#[derive(Debug, Clone, Copy)]
pub struct AddHeapSnapshotChunk;
impl ydp_core::Event for AddHeapSnapshotChunk {
    type Params = AddHeapSnapshotChunkParams;
    const METHOD: &'static str = "HeapProfiler.addHeapSnapshotChunk";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.event.heapStatsUpdate/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct HeapStatsUpdateParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.event.heapStatsUpdate/parameters/statsUpdate。
    ///
    ///协议原文：
    ///
    ///<pre>An array of triplets. Each triplet describes a fragment. The first integer is the fragment
    ///index, the second integer is a total count of objects for the fragment, the third integer is
    ///a total size of the objects for the fragment.</pre>
    ///
    ///
    #[serde(rename = "statsUpdate")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub stats_update: Vec<i64>,
}
impl<'de> serde::Deserialize<'de> for HeapStatsUpdateParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HeapStatsUpdateParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.event.heapStatsUpdate/parameters/statsUpdate。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An array of triplets. Each triplet describes a fragment. The first integer is the fragment
                    ///index, the second integer is a total count of objects for the fragment, the third integer is
                    ///a total size of the objects for the fragment.</pre>
                    ///
                    ///
                    #[serde(rename = "statsUpdate")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub stats_update: Vec<i64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(HeapStatsUpdateParams {
                    stats_update: fields.stats_update,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：HeapProfiler.event.heapStatsUpdate。
///
///协议原文：
///
///<pre>If heap objects tracking has been started then backend may send update for one or more fragments</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct HeapStatsUpdate;
impl ydp_core::Event for HeapStatsUpdate {
    type Params = HeapStatsUpdateParams;
    const METHOD: &'static str = "HeapProfiler.heapStatsUpdate";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.event.lastSeenObjectId/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct LastSeenObjectIdParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.event.lastSeenObjectId/parameters/lastSeenObjectId。
    #[serde(rename = "lastSeenObjectId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub last_seen_object_id: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.event.lastSeenObjectId/parameters/timestamp。
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for LastSeenObjectIdParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LastSeenObjectIdParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.event.lastSeenObjectId/parameters/lastSeenObjectId。
                    #[serde(rename = "lastSeenObjectId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub last_seen_object_id: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.event.lastSeenObjectId/parameters/timestamp。
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LastSeenObjectIdParams {
                    last_seen_object_id: fields.last_seen_object_id,
                    timestamp: fields.timestamp,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：HeapProfiler.event.lastSeenObjectId。
///
///协议原文：
///
///<pre>If heap objects tracking has been started then backend regularly sends a current value for last
///seen object id and corresponding timestamp. If the were changes in the heap since last event
///then one or more heapStatsUpdate events will be sent before a new lastSeenObjectId event.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct LastSeenObjectId;
impl ydp_core::Event for LastSeenObjectId {
    type Params = LastSeenObjectIdParams;
    const METHOD: &'static str = "HeapProfiler.lastSeenObjectId";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.event.reportHeapSnapshotProgress/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReportHeapSnapshotProgressParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.event.reportHeapSnapshotProgress/parameters/done。
    #[serde(rename = "done")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub done: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.event.reportHeapSnapshotProgress/parameters/total。
    #[serde(rename = "total")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub total: i64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.event.reportHeapSnapshotProgress/parameters/finished。
    #[serde(rename = "finished")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub finished: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for ReportHeapSnapshotProgressParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReportHeapSnapshotProgressParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.event.reportHeapSnapshotProgress/parameters/done。
                    #[serde(rename = "done")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub done: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeapProfiler.event.reportHeapSnapshotProgress/parameters/total。
                    #[serde(rename = "total")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub total: i64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeapProfiler.event.reportHeapSnapshotProgress/parameters/finished。
                    #[serde(rename = "finished")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub finished: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReportHeapSnapshotProgressParams {
                    done: fields.done,
                    total: fields.total,
                    finished: fields.finished,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：HeapProfiler.event.reportHeapSnapshotProgress。
#[derive(Debug, Clone, Copy)]
pub struct ReportHeapSnapshotProgress;
impl ydp_core::Event for ReportHeapSnapshotProgress {
    type Params = ReportHeapSnapshotProgressParams;
    const METHOD: &'static str = "HeapProfiler.reportHeapSnapshotProgress";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeapProfiler.event.resetProfiles/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ResetProfilesParams {}
impl<'de> serde::Deserialize<'de> for ResetProfilesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResetProfilesParams;
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
                Ok(ResetProfilesParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：HeapProfiler.event.resetProfiles。
#[derive(Debug, Clone, Copy)]
pub struct ResetProfiles;
impl ydp_core::Event for ResetProfiles {
    type Params = ResetProfilesParams;
    const METHOD: &'static str = "HeapProfiler.resetProfiles";
}
