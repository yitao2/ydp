///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.event.backForwardCacheNotUsed/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct BackForwardCacheNotUsedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.backForwardCacheNotUsed/parameters/loaderId。
    ///
    ///协议原文：
    ///
    ///<pre>The loader id for the associated navigation.</pre>
    ///
    ///
    #[serde(rename = "loaderId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub loader_id: crate::network::types::LoaderId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.backForwardCacheNotUsed/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>The frame id of the associated frame.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.backForwardCacheNotUsed/parameters/notRestoredExplanations。
    ///
    ///协议原文：
    ///
    ///<pre>Array of reasons why the page could not be cached. This must not be empty.</pre>
    ///
    ///
    #[serde(rename = "notRestoredExplanations")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub not_restored_explanations: Vec<crate::page::types::BackForwardCacheNotRestoredExplanation>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.event.backForwardCacheNotUsed/parameters/notRestoredExplanationsTree。
    ///
    ///协议原文：
    ///
    ///<pre>Tree structure of reasons why the page could not be cached for each frame.</pre>
    ///
    ///
    #[serde(rename = "notRestoredExplanationsTree")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub not_restored_explanations_tree:
        Option<crate::page::types::BackForwardCacheNotRestoredExplanationTree>,
}
impl<'de> serde::Deserialize<'de> for BackForwardCacheNotUsedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BackForwardCacheNotUsedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.backForwardCacheNotUsed/parameters/loaderId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The loader id for the associated navigation.</pre>
                    ///
                    ///
                    #[serde(rename = "loaderId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub loader_id: crate::network::types::LoaderId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.backForwardCacheNotUsed/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The frame id of the associated frame.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.backForwardCacheNotUsed/parameters/notRestoredExplanations。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Array of reasons why the page could not be cached. This must not be empty.</pre>
                    ///
                    ///
                    #[serde(rename = "notRestoredExplanations")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub not_restored_explanations:
                        Vec<crate::page::types::BackForwardCacheNotRestoredExplanation>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.event.backForwardCacheNotUsed/parameters/notRestoredExplanationsTree。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Tree structure of reasons why the page could not be cached for each frame.</pre>
                    ///
                    ///
                    #[serde(rename = "notRestoredExplanationsTree")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub not_restored_explanations_tree:
                        Option<crate::page::types::BackForwardCacheNotRestoredExplanationTree>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(BackForwardCacheNotUsedParams {
                    loader_id: fields.loader_id,
                    frame_id: fields.frame_id,
                    not_restored_explanations: fields.not_restored_explanations,
                    not_restored_explanations_tree: fields.not_restored_explanations_tree,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.event.backForwardCacheNotUsed。
///
///协议原文：
///
///<pre>Fired for failed bfcache history navigations if BackForwardCache feature is enabled. Do
///not assume any ordering with the Page.frameNavigated event. This event is fired only for
///main-frame history navigation where the document changes (non-same-document navigations),
///when bfcache navigation fails.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct BackForwardCacheNotUsed;
impl ydp_core::Event for BackForwardCacheNotUsed {
    type Params = BackForwardCacheNotUsedParams;
    const METHOD: &'static str = "Page.backForwardCacheNotUsed";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.event.compilationCacheProduced/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CompilationCacheProducedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.compilationCacheProduced/parameters/url。
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.compilationCacheProduced/parameters/data。
    ///
    ///协议原文：
    ///
    ///<pre>Base64-encoded data (Encoded as a base64 string when passed over JSON)</pre>
    ///
    ///
    #[serde(rename = "data")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub data: String,
}
impl<'de> serde::Deserialize<'de> for CompilationCacheProducedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CompilationCacheProducedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.compilationCacheProduced/parameters/url。
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.compilationCacheProduced/parameters/data。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Base64-encoded data (Encoded as a base64 string when passed over JSON)</pre>
                    ///
                    ///
                    #[serde(rename = "data")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub data: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CompilationCacheProducedParams {
                    url: fields.url,
                    data: fields.data,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.event.compilationCacheProduced。
///
///协议原文：
///
///<pre>Issued for every compilation cache generated.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct CompilationCacheProduced;
impl ydp_core::Event for CompilationCacheProduced {
    type Params = CompilationCacheProducedParams;
    const METHOD: &'static str = "Page.compilationCacheProduced";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.event.documentOpened/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DocumentOpenedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.documentOpened/parameters/frame。
    ///
    ///协议原文：
    ///
    ///<pre>Frame object.</pre>
    ///
    ///
    #[serde(rename = "frame")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame: crate::page::types::Frame,
}
impl<'de> serde::Deserialize<'de> for DocumentOpenedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DocumentOpenedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.documentOpened/parameters/frame。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame object.</pre>
                    ///
                    ///
                    #[serde(rename = "frame")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame: crate::page::types::Frame,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DocumentOpenedParams {
                    frame: fields.frame,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.event.documentOpened。
///
///协议原文：
///
///<pre>Fired when opening document to write to.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct DocumentOpened;
impl ydp_core::Event for DocumentOpened {
    type Params = DocumentOpenedParams;
    const METHOD: &'static str = "Page.documentOpened";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.event.domContentEventFired/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DomContentEventFiredParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.domContentEventFired/parameters/timestamp。
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
}
impl<'de> serde::Deserialize<'de> for DomContentEventFiredParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DomContentEventFiredParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.domContentEventFired/parameters/timestamp。
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DomContentEventFiredParams {
                    timestamp: fields.timestamp,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.event.domContentEventFired。
#[derive(Debug, Clone, Copy)]
pub struct DomContentEventFired;
impl ydp_core::Event for DomContentEventFired {
    type Params = DomContentEventFiredParams;
    const METHOD: &'static str = "Page.domContentEventFired";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.event.downloadProgress/parameters/state。
///
///协议原文：
///
///<pre>Download status.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DownloadProgressParamsState {
    ///协议值 "inProgress"。
    InProgress,
    ///协议值 "completed"。
    Completed,
    ///协议值 "canceled"。
    Canceled,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl DownloadProgressParamsState {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::events::DownloadProgressParamsState::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::InProgress => "inProgress",
            Self::Completed => "completed",
            Self::Canceled => "canceled",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for DownloadProgressParamsState {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for DownloadProgressParamsState {
    fn from(value: String) -> Self {
        match value.as_str() {
            "inProgress" => DownloadProgressParamsState::InProgress,
            "completed" => DownloadProgressParamsState::Completed,
            "canceled" => DownloadProgressParamsState::Canceled,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for DownloadProgressParamsState {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DownloadProgressParamsState {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DownloadProgressParamsState;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "inProgress" => DownloadProgressParamsState::InProgress,
                    "completed" => DownloadProgressParamsState::Completed,
                    "canceled" => DownloadProgressParamsState::Canceled,
                    _ => DownloadProgressParamsState::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(DownloadProgressParamsState::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.event.downloadProgress/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DownloadProgressParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.downloadProgress/parameters/guid。
    ///
    ///协议原文：
    ///
    ///<pre>Global unique identifier of the download.</pre>
    ///
    ///
    #[serde(rename = "guid")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub guid: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.downloadProgress/parameters/totalBytes。
    ///
    ///协议原文：
    ///
    ///<pre>Total expected bytes to download.</pre>
    ///
    ///
    #[serde(rename = "totalBytes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub total_bytes: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.downloadProgress/parameters/receivedBytes。
    ///
    ///协议原文：
    ///
    ///<pre>Total bytes received.</pre>
    ///
    ///
    #[serde(rename = "receivedBytes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub received_bytes: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.downloadProgress/parameters/state。
    ///
    ///协议原文：
    ///
    ///<pre>Download status.</pre>
    ///
    ///
    #[serde(rename = "state")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub state: DownloadProgressParamsState,
}
impl<'de> serde::Deserialize<'de> for DownloadProgressParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DownloadProgressParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.downloadProgress/parameters/guid。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Global unique identifier of the download.</pre>
                    ///
                    ///
                    #[serde(rename = "guid")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub guid: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.downloadProgress/parameters/totalBytes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Total expected bytes to download.</pre>
                    ///
                    ///
                    #[serde(rename = "totalBytes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub total_bytes: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.downloadProgress/parameters/receivedBytes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Total bytes received.</pre>
                    ///
                    ///
                    #[serde(rename = "receivedBytes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub received_bytes: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.downloadProgress/parameters/state。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Download status.</pre>
                    ///
                    ///
                    #[serde(rename = "state")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub state: DownloadProgressParamsState,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DownloadProgressParams {
                    guid: fields.guid,
                    total_bytes: fields.total_bytes,
                    received_bytes: fields.received_bytes,
                    state: fields.state,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.event.downloadProgress。
///
///协议原文：
///
///<pre>Fired when download makes progress. Last call has |done| == true.
///Deprecated. Use Browser.downloadProgress instead.</pre>
///
///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct DownloadProgress;
impl ydp_core::Event for DownloadProgress {
    type Params = DownloadProgressParams;
    const METHOD: &'static str = "Page.downloadProgress";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.event.downloadWillBegin/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DownloadWillBeginParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.downloadWillBegin/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the frame that caused download to begin.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.downloadWillBegin/parameters/guid。
    ///
    ///协议原文：
    ///
    ///<pre>Global unique identifier of the download.</pre>
    ///
    ///
    #[serde(rename = "guid")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub guid: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.downloadWillBegin/parameters/url。
    ///
    ///协议原文：
    ///
    ///<pre>URL of the resource being downloaded.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.downloadWillBegin/parameters/suggestedFilename。
    ///
    ///协议原文：
    ///
    ///<pre>Suggested file name of the resource (the actual name of the file saved on disk may differ).</pre>
    ///
    ///
    #[serde(rename = "suggestedFilename")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub suggested_filename: String,
}
impl<'de> serde::Deserialize<'de> for DownloadWillBeginParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DownloadWillBeginParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.downloadWillBegin/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the frame that caused download to begin.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.downloadWillBegin/parameters/guid。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Global unique identifier of the download.</pre>
                    ///
                    ///
                    #[serde(rename = "guid")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub guid: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.downloadWillBegin/parameters/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>URL of the resource being downloaded.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.downloadWillBegin/parameters/suggestedFilename。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Suggested file name of the resource (the actual name of the file saved on disk may differ).</pre>
                    ///
                    ///
                    #[serde(rename = "suggestedFilename")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub suggested_filename: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DownloadWillBeginParams {
                    frame_id: fields.frame_id,
                    guid: fields.guid,
                    url: fields.url,
                    suggested_filename: fields.suggested_filename,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.event.downloadWillBegin。
///
///协议原文：
///
///<pre>Fired when page is about to start a download.
///Deprecated. Use Browser.downloadWillBegin instead.</pre>
///
///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct DownloadWillBegin;
impl ydp_core::Event for DownloadWillBegin {
    type Params = DownloadWillBeginParams;
    const METHOD: &'static str = "Page.downloadWillBegin";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.event.fileChooserOpened/parameters/mode。
///
///协议原文：
///
///<pre>Input mode.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FileChooserOpenedParamsMode {
    ///协议值 "selectSingle"。
    SelectSingle,
    ///协议值 "selectMultiple"。
    SelectMultiple,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl FileChooserOpenedParamsMode {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::events::FileChooserOpenedParamsMode::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::SelectSingle => "selectSingle",
            Self::SelectMultiple => "selectMultiple",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for FileChooserOpenedParamsMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for FileChooserOpenedParamsMode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "selectSingle" => FileChooserOpenedParamsMode::SelectSingle,
            "selectMultiple" => FileChooserOpenedParamsMode::SelectMultiple,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for FileChooserOpenedParamsMode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FileChooserOpenedParamsMode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FileChooserOpenedParamsMode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "selectSingle" => FileChooserOpenedParamsMode::SelectSingle,
                    "selectMultiple" => FileChooserOpenedParamsMode::SelectMultiple,
                    _ => FileChooserOpenedParamsMode::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(FileChooserOpenedParamsMode::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.event.fileChooserOpened/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FileChooserOpenedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.fileChooserOpened/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the frame containing input node.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.fileChooserOpened/parameters/mode。
    ///
    ///协议原文：
    ///
    ///<pre>Input mode.</pre>
    ///
    ///
    #[serde(rename = "mode")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub mode: FileChooserOpenedParamsMode,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.event.fileChooserOpened/parameters/backendNodeId。
    ///
    ///协议原文：
    ///
    ///<pre>Input node id. Only present for file choosers opened via an `&lt;input type="file"&gt;` element.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "backendNodeId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
}
impl<'de> serde::Deserialize<'de> for FileChooserOpenedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FileChooserOpenedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.fileChooserOpened/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the frame containing input node.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.fileChooserOpened/parameters/mode。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Input mode.</pre>
                    ///
                    ///
                    #[serde(rename = "mode")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub mode: FileChooserOpenedParamsMode,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.event.fileChooserOpened/parameters/backendNodeId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Input node id. Only present for file choosers opened via an `&lt;input type="file"&gt;` element.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "backendNodeId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub backend_node_id: Option<crate::dom::types::BackendNodeId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FileChooserOpenedParams {
                    frame_id: fields.frame_id,
                    mode: fields.mode,
                    backend_node_id: fields.backend_node_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.event.fileChooserOpened。
///
///协议原文：
///
///<pre>Emitted only when `page.interceptFileChooser` is enabled.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct FileChooserOpened;
impl ydp_core::Event for FileChooserOpened {
    type Params = FileChooserOpenedParams;
    const METHOD: &'static str = "Page.fileChooserOpened";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.event.frameAttached/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FrameAttachedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameAttached/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the frame that has been attached.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameAttached/parameters/parentFrameId。
    ///
    ///协议原文：
    ///
    ///<pre>Parent frame identifier.</pre>
    ///
    ///
    #[serde(rename = "parentFrameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub parent_frame_id: crate::page::types::FrameId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.event.frameAttached/parameters/stack。
    ///
    ///协议原文：
    ///
    ///<pre>JavaScript stack trace of when frame was attached, only set if frame initiated from script.</pre>
    ///
    ///
    #[serde(rename = "stack")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub stack: Option<crate::runtime::types::StackTrace>,
}
impl<'de> serde::Deserialize<'de> for FrameAttachedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FrameAttachedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameAttached/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the frame that has been attached.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameAttached/parameters/parentFrameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Parent frame identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "parentFrameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub parent_frame_id: crate::page::types::FrameId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.event.frameAttached/parameters/stack。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>JavaScript stack trace of when frame was attached, only set if frame initiated from script.</pre>
                    ///
                    ///
                    #[serde(rename = "stack")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub stack: Option<crate::runtime::types::StackTrace>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FrameAttachedParams {
                    frame_id: fields.frame_id,
                    parent_frame_id: fields.parent_frame_id,
                    stack: fields.stack,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.event.frameAttached。
///
///协议原文：
///
///<pre>Fired when frame has been attached to its parent.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct FrameAttached;
impl ydp_core::Event for FrameAttached {
    type Params = FrameAttachedParams;
    const METHOD: &'static str = "Page.frameAttached";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.event.frameClearedScheduledNavigation/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FrameClearedScheduledNavigationParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameClearedScheduledNavigation/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the frame that has cleared its scheduled navigation.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
}
impl<'de> serde::Deserialize<'de> for FrameClearedScheduledNavigationParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FrameClearedScheduledNavigationParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameClearedScheduledNavigation/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the frame that has cleared its scheduled navigation.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FrameClearedScheduledNavigationParams {
                    frame_id: fields.frame_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.event.frameClearedScheduledNavigation。
///
///协议原文：
///
///<pre>Fired when frame no longer has a scheduled navigation.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct FrameClearedScheduledNavigation;
impl ydp_core::Event for FrameClearedScheduledNavigation {
    type Params = FrameClearedScheduledNavigationParams;
    const METHOD: &'static str = "Page.frameClearedScheduledNavigation";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.event.frameDetached/parameters/reason。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FrameDetachedParamsReason {
    ///协议值 "remove"。
    Remove,
    ///协议值 "swap"。
    Swap,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl FrameDetachedParamsReason {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::events::FrameDetachedParamsReason::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Remove => "remove",
            Self::Swap => "swap",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for FrameDetachedParamsReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for FrameDetachedParamsReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "remove" => FrameDetachedParamsReason::Remove,
            "swap" => FrameDetachedParamsReason::Swap,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for FrameDetachedParamsReason {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FrameDetachedParamsReason {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FrameDetachedParamsReason;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "remove" => FrameDetachedParamsReason::Remove,
                    "swap" => FrameDetachedParamsReason::Swap,
                    _ => FrameDetachedParamsReason::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(FrameDetachedParamsReason::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.event.frameDetached/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FrameDetachedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameDetached/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the frame that has been detached.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameDetached/parameters/reason。此项为 experimental，端点可能不支持。
    #[serde(rename = "reason")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub reason: FrameDetachedParamsReason,
}
impl<'de> serde::Deserialize<'de> for FrameDetachedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FrameDetachedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameDetached/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the frame that has been detached.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameDetached/parameters/reason。此项为 experimental，端点可能不支持。
                    #[serde(rename = "reason")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub reason: FrameDetachedParamsReason,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FrameDetachedParams {
                    frame_id: fields.frame_id,
                    reason: fields.reason,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.event.frameDetached。
///
///协议原文：
///
///<pre>Fired when frame has been detached from its parent.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct FrameDetached;
impl ydp_core::Event for FrameDetached {
    type Params = FrameDetachedParams;
    const METHOD: &'static str = "Page.frameDetached";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.event.frameNavigated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FrameNavigatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameNavigated/parameters/frame。
    ///
    ///协议原文：
    ///
    ///<pre>Frame object.</pre>
    ///
    ///
    #[serde(rename = "frame")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame: crate::page::types::Frame,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameNavigated/parameters/type。此项为 experimental，端点可能不支持。
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: crate::page::types::NavigationType,
}
impl<'de> serde::Deserialize<'de> for FrameNavigatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FrameNavigatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameNavigated/parameters/frame。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame object.</pre>
                    ///
                    ///
                    #[serde(rename = "frame")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame: crate::page::types::Frame,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameNavigated/parameters/type。此项为 experimental，端点可能不支持。
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: crate::page::types::NavigationType,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FrameNavigatedParams {
                    frame: fields.frame,
                    r#type: fields.r#type,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.event.frameNavigated。
///
///协议原文：
///
///<pre>Fired once navigation of the frame has completed. Frame is now associated with the new loader.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct FrameNavigated;
impl ydp_core::Event for FrameNavigated {
    type Params = FrameNavigatedParams;
    const METHOD: &'static str = "Page.frameNavigated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.event.frameRequestedNavigation/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FrameRequestedNavigationParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameRequestedNavigation/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the frame that is being navigated.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameRequestedNavigation/parameters/reason。
    ///
    ///协议原文：
    ///
    ///<pre>The reason for the navigation.</pre>
    ///
    ///
    #[serde(rename = "reason")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub reason: crate::page::types::ClientNavigationReason,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameRequestedNavigation/parameters/url。
    ///
    ///协议原文：
    ///
    ///<pre>The destination URL for the requested navigation.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameRequestedNavigation/parameters/disposition。
    ///
    ///协议原文：
    ///
    ///<pre>The disposition for the navigation.</pre>
    ///
    ///
    #[serde(rename = "disposition")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub disposition: crate::page::types::ClientNavigationDisposition,
}
impl<'de> serde::Deserialize<'de> for FrameRequestedNavigationParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FrameRequestedNavigationParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameRequestedNavigation/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the frame that is being navigated.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameRequestedNavigation/parameters/reason。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The reason for the navigation.</pre>
                    ///
                    ///
                    #[serde(rename = "reason")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub reason: crate::page::types::ClientNavigationReason,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameRequestedNavigation/parameters/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The destination URL for the requested navigation.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameRequestedNavigation/parameters/disposition。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The disposition for the navigation.</pre>
                    ///
                    ///
                    #[serde(rename = "disposition")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub disposition: crate::page::types::ClientNavigationDisposition,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FrameRequestedNavigationParams {
                    frame_id: fields.frame_id,
                    reason: fields.reason,
                    url: fields.url,
                    disposition: fields.disposition,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.event.frameRequestedNavigation。
///
///协议原文：
///
///<pre>Fired when a renderer-initiated navigation is requested.
///Navigation may still be cancelled after the event is issued.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct FrameRequestedNavigation;
impl ydp_core::Event for FrameRequestedNavigation {
    type Params = FrameRequestedNavigationParams;
    const METHOD: &'static str = "Page.frameRequestedNavigation";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.event.frameResized/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct FrameResizedParams {}
impl<'de> serde::Deserialize<'de> for FrameResizedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FrameResizedParams;
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
                Ok(FrameResizedParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.event.frameResized。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct FrameResized;
impl ydp_core::Event for FrameResized {
    type Params = FrameResizedParams;
    const METHOD: &'static str = "Page.frameResized";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.event.frameScheduledNavigation/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FrameScheduledNavigationParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameScheduledNavigation/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the frame that has scheduled a navigation.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameScheduledNavigation/parameters/delay。
    ///
    ///协议原文：
    ///
    ///<pre>Delay (in seconds) until the navigation is scheduled to begin. The navigation is not
    ///guaranteed to start.</pre>
    ///
    ///
    #[serde(rename = "delay")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub delay: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameScheduledNavigation/parameters/reason。
    ///
    ///协议原文：
    ///
    ///<pre>The reason for the navigation.</pre>
    ///
    ///
    #[serde(rename = "reason")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub reason: crate::page::types::ClientNavigationReason,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameScheduledNavigation/parameters/url。
    ///
    ///协议原文：
    ///
    ///<pre>The destination URL for the scheduled navigation.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
}
impl<'de> serde::Deserialize<'de> for FrameScheduledNavigationParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FrameScheduledNavigationParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameScheduledNavigation/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the frame that has scheduled a navigation.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameScheduledNavigation/parameters/delay。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Delay (in seconds) until the navigation is scheduled to begin. The navigation is not
                    ///guaranteed to start.</pre>
                    ///
                    ///
                    #[serde(rename = "delay")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub delay: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameScheduledNavigation/parameters/reason。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The reason for the navigation.</pre>
                    ///
                    ///
                    #[serde(rename = "reason")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub reason: crate::page::types::ClientNavigationReason,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameScheduledNavigation/parameters/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The destination URL for the scheduled navigation.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FrameScheduledNavigationParams {
                    frame_id: fields.frame_id,
                    delay: fields.delay,
                    reason: fields.reason,
                    url: fields.url,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.event.frameScheduledNavigation。
///
///协议原文：
///
///<pre>Fired when frame schedules a potential navigation.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct FrameScheduledNavigation;
impl ydp_core::Event for FrameScheduledNavigation {
    type Params = FrameScheduledNavigationParams;
    const METHOD: &'static str = "Page.frameScheduledNavigation";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.event.frameStartedLoading/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FrameStartedLoadingParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameStartedLoading/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the frame that has started loading.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
}
impl<'de> serde::Deserialize<'de> for FrameStartedLoadingParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FrameStartedLoadingParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameStartedLoading/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the frame that has started loading.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FrameStartedLoadingParams {
                    frame_id: fields.frame_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.event.frameStartedLoading。
///
///协议原文：
///
///<pre>Fired when frame has started loading.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct FrameStartedLoading;
impl ydp_core::Event for FrameStartedLoading {
    type Params = FrameStartedLoadingParams;
    const METHOD: &'static str = "Page.frameStartedLoading";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.event.frameStartedNavigating/parameters/navigationType。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FrameStartedNavigatingParamsNavigationType {
    ///协议值 "reload"。
    Reload,
    ///协议值 "reloadBypassingCache"。
    ReloadBypassingCache,
    ///协议值 "restore"。
    Restore,
    ///协议值 "restoreWithPost"。
    RestoreWithPost,
    ///协议值 "historySameDocument"。
    HistorySameDocument,
    ///协议值 "historyDifferentDocument"。
    HistoryDifferentDocument,
    ///协议值 "sameDocument"。
    SameDocument,
    ///协议值 "differentDocument"。
    DifferentDocument,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl FrameStartedNavigatingParamsNavigationType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::events::FrameStartedNavigatingParamsNavigationType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Reload => "reload",
            Self::ReloadBypassingCache => "reloadBypassingCache",
            Self::Restore => "restore",
            Self::RestoreWithPost => "restoreWithPost",
            Self::HistorySameDocument => "historySameDocument",
            Self::HistoryDifferentDocument => "historyDifferentDocument",
            Self::SameDocument => "sameDocument",
            Self::DifferentDocument => "differentDocument",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for FrameStartedNavigatingParamsNavigationType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for FrameStartedNavigatingParamsNavigationType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "reload" => FrameStartedNavigatingParamsNavigationType::Reload,
            "reloadBypassingCache" => {
                FrameStartedNavigatingParamsNavigationType::ReloadBypassingCache
            }
            "restore" => FrameStartedNavigatingParamsNavigationType::Restore,
            "restoreWithPost" => FrameStartedNavigatingParamsNavigationType::RestoreWithPost,
            "historySameDocument" => {
                FrameStartedNavigatingParamsNavigationType::HistorySameDocument
            }
            "historyDifferentDocument" => {
                FrameStartedNavigatingParamsNavigationType::HistoryDifferentDocument
            }
            "sameDocument" => FrameStartedNavigatingParamsNavigationType::SameDocument,
            "differentDocument" => FrameStartedNavigatingParamsNavigationType::DifferentDocument,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for FrameStartedNavigatingParamsNavigationType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FrameStartedNavigatingParamsNavigationType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FrameStartedNavigatingParamsNavigationType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "reload" => FrameStartedNavigatingParamsNavigationType::Reload,
                    "reloadBypassingCache" => {
                        FrameStartedNavigatingParamsNavigationType::ReloadBypassingCache
                    }
                    "restore" => FrameStartedNavigatingParamsNavigationType::Restore,
                    "restoreWithPost" => {
                        FrameStartedNavigatingParamsNavigationType::RestoreWithPost
                    }
                    "historySameDocument" => {
                        FrameStartedNavigatingParamsNavigationType::HistorySameDocument
                    }
                    "historyDifferentDocument" => {
                        FrameStartedNavigatingParamsNavigationType::HistoryDifferentDocument
                    }
                    "sameDocument" => FrameStartedNavigatingParamsNavigationType::SameDocument,
                    "differentDocument" => {
                        FrameStartedNavigatingParamsNavigationType::DifferentDocument
                    }
                    _ => FrameStartedNavigatingParamsNavigationType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(FrameStartedNavigatingParamsNavigationType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.event.frameStartedNavigating/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FrameStartedNavigatingParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameStartedNavigating/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>ID of the frame that is being navigated.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameStartedNavigating/parameters/url。
    ///
    ///协议原文：
    ///
    ///<pre>The URL the navigation started with. The final URL can be different.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameStartedNavigating/parameters/loaderId。
    ///
    ///协议原文：
    ///
    ///<pre>Loader identifier. Even though it is present in case of same-document
    ///navigation, the previously committed loaderId would not change unless
    ///the navigation changes from a same-document to a cross-document
    ///navigation.</pre>
    ///
    ///
    #[serde(rename = "loaderId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub loader_id: crate::network::types::LoaderId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameStartedNavigating/parameters/navigationType。
    #[serde(rename = "navigationType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub navigation_type: FrameStartedNavigatingParamsNavigationType,
}
impl<'de> serde::Deserialize<'de> for FrameStartedNavigatingParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FrameStartedNavigatingParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameStartedNavigating/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>ID of the frame that is being navigated.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameStartedNavigating/parameters/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The URL the navigation started with. The final URL can be different.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameStartedNavigating/parameters/loaderId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Loader identifier. Even though it is present in case of same-document
                    ///navigation, the previously committed loaderId would not change unless
                    ///the navigation changes from a same-document to a cross-document
                    ///navigation.</pre>
                    ///
                    ///
                    #[serde(rename = "loaderId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub loader_id: crate::network::types::LoaderId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameStartedNavigating/parameters/navigationType。
                    #[serde(rename = "navigationType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub navigation_type: FrameStartedNavigatingParamsNavigationType,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FrameStartedNavigatingParams {
                    frame_id: fields.frame_id,
                    url: fields.url,
                    loader_id: fields.loader_id,
                    navigation_type: fields.navigation_type,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.event.frameStartedNavigating。
///
///协议原文：
///
///<pre>Fired when a navigation starts. This event is fired for both
///renderer-initiated and browser-initiated navigations. For renderer-initiated
///navigations, the event is fired after `frameRequestedNavigation`.
///Navigation may still be cancelled after the event is issued. Multiple events
///can be fired for a single navigation, for example, when a same-document
///navigation becomes a cross-document navigation (such as in the case of a
///frameset).</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct FrameStartedNavigating;
impl ydp_core::Event for FrameStartedNavigating {
    type Params = FrameStartedNavigatingParams;
    const METHOD: &'static str = "Page.frameStartedNavigating";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.event.frameStoppedLoading/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FrameStoppedLoadingParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameStoppedLoading/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the frame that has stopped loading.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
}
impl<'de> serde::Deserialize<'de> for FrameStoppedLoadingParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FrameStoppedLoadingParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameStoppedLoading/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the frame that has stopped loading.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FrameStoppedLoadingParams {
                    frame_id: fields.frame_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.event.frameStoppedLoading。
///
///协议原文：
///
///<pre>Fired when frame has stopped loading.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct FrameStoppedLoading;
impl ydp_core::Event for FrameStoppedLoading {
    type Params = FrameStoppedLoadingParams;
    const METHOD: &'static str = "Page.frameStoppedLoading";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.event.frameSubtreeWillBeDetached/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FrameSubtreeWillBeDetachedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameSubtreeWillBeDetached/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the frame that is the root of the subtree that will be detached.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
}
impl<'de> serde::Deserialize<'de> for FrameSubtreeWillBeDetachedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FrameSubtreeWillBeDetachedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.frameSubtreeWillBeDetached/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the frame that is the root of the subtree that will be detached.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FrameSubtreeWillBeDetachedParams {
                    frame_id: fields.frame_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.event.frameSubtreeWillBeDetached。
///
///协议原文：
///
///<pre>Fired before frame subtree is detached. Emitted before any frame of the
///subtree is actually detached.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct FrameSubtreeWillBeDetached;
impl ydp_core::Event for FrameSubtreeWillBeDetached {
    type Params = FrameSubtreeWillBeDetachedParams;
    const METHOD: &'static str = "Page.frameSubtreeWillBeDetached";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.event.interstitialHidden/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct InterstitialHiddenParams {}
impl<'de> serde::Deserialize<'de> for InterstitialHiddenParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InterstitialHiddenParams;
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
                Ok(InterstitialHiddenParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.event.interstitialHidden。
///
///协议原文：
///
///<pre>Fired when interstitial page was hidden</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct InterstitialHidden;
impl ydp_core::Event for InterstitialHidden {
    type Params = InterstitialHiddenParams;
    const METHOD: &'static str = "Page.interstitialHidden";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.event.interstitialShown/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct InterstitialShownParams {}
impl<'de> serde::Deserialize<'de> for InterstitialShownParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InterstitialShownParams;
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
                Ok(InterstitialShownParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.event.interstitialShown。
///
///协议原文：
///
///<pre>Fired when interstitial page was shown</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct InterstitialShown;
impl ydp_core::Event for InterstitialShown {
    type Params = InterstitialShownParams;
    const METHOD: &'static str = "Page.interstitialShown";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.event.javascriptDialogClosed/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct JavascriptDialogClosedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.javascriptDialogClosed/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Frame id.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.javascriptDialogClosed/parameters/result。
    ///
    ///协议原文：
    ///
    ///<pre>Whether dialog was confirmed.</pre>
    ///
    ///
    #[serde(rename = "result")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub result: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.javascriptDialogClosed/parameters/userInput。
    ///
    ///协议原文：
    ///
    ///<pre>User input in case of prompt.</pre>
    ///
    ///
    #[serde(rename = "userInput")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub user_input: String,
}
impl<'de> serde::Deserialize<'de> for JavascriptDialogClosedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = JavascriptDialogClosedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.javascriptDialogClosed/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame id.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.javascriptDialogClosed/parameters/result。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether dialog was confirmed.</pre>
                    ///
                    ///
                    #[serde(rename = "result")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub result: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.javascriptDialogClosed/parameters/userInput。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>User input in case of prompt.</pre>
                    ///
                    ///
                    #[serde(rename = "userInput")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub user_input: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(JavascriptDialogClosedParams {
                    frame_id: fields.frame_id,
                    result: fields.result,
                    user_input: fields.user_input,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.event.javascriptDialogClosed。
///
///协议原文：
///
///<pre>Fired when a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload) has been
///closed.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct JavascriptDialogClosed;
impl ydp_core::Event for JavascriptDialogClosed {
    type Params = JavascriptDialogClosedParams;
    const METHOD: &'static str = "Page.javascriptDialogClosed";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.event.javascriptDialogOpening/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct JavascriptDialogOpeningParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.javascriptDialogOpening/parameters/url。
    ///
    ///协议原文：
    ///
    ///<pre>Frame url.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.javascriptDialogOpening/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Frame id.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.javascriptDialogOpening/parameters/message。
    ///
    ///协议原文：
    ///
    ///<pre>Message that will be displayed by the dialog.</pre>
    ///
    ///
    #[serde(rename = "message")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub message: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.javascriptDialogOpening/parameters/type。
    ///
    ///协议原文：
    ///
    ///<pre>Dialog type.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: crate::page::types::DialogType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.javascriptDialogOpening/parameters/hasBrowserHandler。
    ///
    ///协议原文：
    ///
    ///<pre>True iff browser is capable showing or acting on the given dialog. When browser has no
    ///dialog handler for given target, calling alert while Page domain is engaged will stall
    ///the page execution. Execution can be resumed via calling Page.handleJavaScriptDialog.</pre>
    ///
    ///
    #[serde(rename = "hasBrowserHandler")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub has_browser_handler: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.event.javascriptDialogOpening/parameters/defaultPrompt。
    ///
    ///协议原文：
    ///
    ///<pre>Default dialog prompt.</pre>
    ///
    ///
    #[serde(rename = "defaultPrompt")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_prompt: Option<String>,
}
impl<'de> serde::Deserialize<'de> for JavascriptDialogOpeningParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = JavascriptDialogOpeningParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.javascriptDialogOpening/parameters/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame url.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.javascriptDialogOpening/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame id.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.javascriptDialogOpening/parameters/message。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Message that will be displayed by the dialog.</pre>
                    ///
                    ///
                    #[serde(rename = "message")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub message: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.javascriptDialogOpening/parameters/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Dialog type.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: crate::page::types::DialogType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.javascriptDialogOpening/parameters/hasBrowserHandler。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True iff browser is capable showing or acting on the given dialog. When browser has no
                    ///dialog handler for given target, calling alert while Page domain is engaged will stall
                    ///the page execution. Execution can be resumed via calling Page.handleJavaScriptDialog.</pre>
                    ///
                    ///
                    #[serde(rename = "hasBrowserHandler")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub has_browser_handler: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.event.javascriptDialogOpening/parameters/defaultPrompt。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Default dialog prompt.</pre>
                    ///
                    ///
                    #[serde(rename = "defaultPrompt")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub default_prompt: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(JavascriptDialogOpeningParams {
                    url: fields.url,
                    frame_id: fields.frame_id,
                    message: fields.message,
                    r#type: fields.r#type,
                    has_browser_handler: fields.has_browser_handler,
                    default_prompt: fields.default_prompt,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.event.javascriptDialogOpening。
///
///协议原文：
///
///<pre>Fired when a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload) is about to
///open.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct JavascriptDialogOpening;
impl ydp_core::Event for JavascriptDialogOpening {
    type Params = JavascriptDialogOpeningParams;
    const METHOD: &'static str = "Page.javascriptDialogOpening";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.event.lifecycleEvent/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct LifecycleEventParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.lifecycleEvent/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the frame.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.lifecycleEvent/parameters/loaderId。
    ///
    ///协议原文：
    ///
    ///<pre>Loader identifier. Empty string if the request is fetched from worker.</pre>
    ///
    ///
    #[serde(rename = "loaderId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub loader_id: crate::network::types::LoaderId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.lifecycleEvent/parameters/name。
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.lifecycleEvent/parameters/timestamp。
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
}
impl<'de> serde::Deserialize<'de> for LifecycleEventParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LifecycleEventParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.lifecycleEvent/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the frame.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.lifecycleEvent/parameters/loaderId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Loader identifier. Empty string if the request is fetched from worker.</pre>
                    ///
                    ///
                    #[serde(rename = "loaderId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub loader_id: crate::network::types::LoaderId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.lifecycleEvent/parameters/name。
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.lifecycleEvent/parameters/timestamp。
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LifecycleEventParams {
                    frame_id: fields.frame_id,
                    loader_id: fields.loader_id,
                    name: fields.name,
                    timestamp: fields.timestamp,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.event.lifecycleEvent。
///
///协议原文：
///
///<pre>Fired for lifecycle events (navigation, load, paint, etc) in the current
///target (including local frames).</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct LifecycleEvent;
impl ydp_core::Event for LifecycleEvent {
    type Params = LifecycleEventParams;
    const METHOD: &'static str = "Page.lifecycleEvent";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.event.loadEventFired/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct LoadEventFiredParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.loadEventFired/parameters/timestamp。
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::MonotonicTime,
}
impl<'de> serde::Deserialize<'de> for LoadEventFiredParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LoadEventFiredParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.loadEventFired/parameters/timestamp。
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::MonotonicTime,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LoadEventFiredParams {
                    timestamp: fields.timestamp,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.event.loadEventFired。
#[derive(Debug, Clone, Copy)]
pub struct LoadEventFired;
impl ydp_core::Event for LoadEventFired {
    type Params = LoadEventFiredParams;
    const METHOD: &'static str = "Page.loadEventFired";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.event.navigatedWithinDocument/parameters/navigationType。
///
///协议原文：
///
///<pre>Navigation type</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum NavigatedWithinDocumentParamsNavigationType {
    ///协议值 "fragment"。
    Fragment,
    ///协议值 "historyApi"。
    HistoryApi,
    ///协议值 "other"。
    Other,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl NavigatedWithinDocumentParamsNavigationType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::events::NavigatedWithinDocumentParamsNavigationType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Fragment => "fragment",
            Self::HistoryApi => "historyApi",
            Self::Other => "other",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for NavigatedWithinDocumentParamsNavigationType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for NavigatedWithinDocumentParamsNavigationType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "fragment" => NavigatedWithinDocumentParamsNavigationType::Fragment,
            "historyApi" => NavigatedWithinDocumentParamsNavigationType::HistoryApi,
            "other" => NavigatedWithinDocumentParamsNavigationType::Other,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for NavigatedWithinDocumentParamsNavigationType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for NavigatedWithinDocumentParamsNavigationType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NavigatedWithinDocumentParamsNavigationType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "fragment" => NavigatedWithinDocumentParamsNavigationType::Fragment,
                    "historyApi" => NavigatedWithinDocumentParamsNavigationType::HistoryApi,
                    "other" => NavigatedWithinDocumentParamsNavigationType::Other,
                    _ => NavigatedWithinDocumentParamsNavigationType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(NavigatedWithinDocumentParamsNavigationType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.event.navigatedWithinDocument/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct NavigatedWithinDocumentParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.navigatedWithinDocument/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the frame.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.navigatedWithinDocument/parameters/url。
    ///
    ///协议原文：
    ///
    ///<pre>Frame's new url.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.navigatedWithinDocument/parameters/navigationType。
    ///
    ///协议原文：
    ///
    ///<pre>Navigation type</pre>
    ///
    ///
    #[serde(rename = "navigationType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub navigation_type: NavigatedWithinDocumentParamsNavigationType,
}
impl<'de> serde::Deserialize<'de> for NavigatedWithinDocumentParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NavigatedWithinDocumentParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.navigatedWithinDocument/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the frame.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.navigatedWithinDocument/parameters/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame's new url.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.navigatedWithinDocument/parameters/navigationType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Navigation type</pre>
                    ///
                    ///
                    #[serde(rename = "navigationType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub navigation_type: NavigatedWithinDocumentParamsNavigationType,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(NavigatedWithinDocumentParams {
                    frame_id: fields.frame_id,
                    url: fields.url,
                    navigation_type: fields.navigation_type,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.event.navigatedWithinDocument。
///
///协议原文：
///
///<pre>Fired when same-document navigation happens, e.g. due to history API usage or anchor navigation.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct NavigatedWithinDocument;
impl ydp_core::Event for NavigatedWithinDocument {
    type Params = NavigatedWithinDocumentParams;
    const METHOD: &'static str = "Page.navigatedWithinDocument";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.event.screencastFrame/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ScreencastFrameParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.screencastFrame/parameters/data。
    ///
    ///协议原文：
    ///
    ///<pre>Base64-encoded compressed image. (Encoded as a base64 string when passed over JSON)</pre>
    ///
    ///
    #[serde(rename = "data")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub data: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.screencastFrame/parameters/metadata。
    ///
    ///协议原文：
    ///
    ///<pre>Screencast frame metadata.</pre>
    ///
    ///
    #[serde(rename = "metadata")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub metadata: crate::page::types::ScreencastFrameMetadata,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.screencastFrame/parameters/sessionId。
    ///
    ///协议原文：
    ///
    ///<pre>Frame number.</pre>
    ///
    ///
    #[serde(rename = "sessionId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub session_id: i64,
}
impl<'de> serde::Deserialize<'de> for ScreencastFrameParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScreencastFrameParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.screencastFrame/parameters/data。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Base64-encoded compressed image. (Encoded as a base64 string when passed over JSON)</pre>
                    ///
                    ///
                    #[serde(rename = "data")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub data: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.screencastFrame/parameters/metadata。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Screencast frame metadata.</pre>
                    ///
                    ///
                    #[serde(rename = "metadata")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub metadata: crate::page::types::ScreencastFrameMetadata,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.screencastFrame/parameters/sessionId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame number.</pre>
                    ///
                    ///
                    #[serde(rename = "sessionId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub session_id: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ScreencastFrameParams {
                    data: fields.data,
                    metadata: fields.metadata,
                    session_id: fields.session_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.event.screencastFrame。
///
///协议原文：
///
///<pre>Compressed image data requested by the `startScreencast`.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ScreencastFrame;
impl ydp_core::Event for ScreencastFrame {
    type Params = ScreencastFrameParams;
    const METHOD: &'static str = "Page.screencastFrame";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.event.screencastVisibilityChanged/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ScreencastVisibilityChangedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.screencastVisibilityChanged/parameters/visible。
    ///
    ///协议原文：
    ///
    ///<pre>True if the page is visible.</pre>
    ///
    ///
    #[serde(rename = "visible")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub visible: bool,
}
impl<'de> serde::Deserialize<'de> for ScreencastVisibilityChangedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScreencastVisibilityChangedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.screencastVisibilityChanged/parameters/visible。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if the page is visible.</pre>
                    ///
                    ///
                    #[serde(rename = "visible")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub visible: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ScreencastVisibilityChangedParams {
                    visible: fields.visible,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.event.screencastVisibilityChanged。
///
///协议原文：
///
///<pre>Fired when the page with currently enabled screencast was shown or hidden `.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ScreencastVisibilityChanged;
impl ydp_core::Event for ScreencastVisibilityChanged {
    type Params = ScreencastVisibilityChangedParams;
    const METHOD: &'static str = "Page.screencastVisibilityChanged";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.event.windowOpen/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct WindowOpenParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.windowOpen/parameters/url。
    ///
    ///协议原文：
    ///
    ///<pre>The URL for the new window.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.windowOpen/parameters/windowName。
    ///
    ///协议原文：
    ///
    ///<pre>Window name.</pre>
    ///
    ///
    #[serde(rename = "windowName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub window_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.windowOpen/parameters/windowFeatures。
    ///
    ///协议原文：
    ///
    ///<pre>An array of enabled window features.</pre>
    ///
    ///
    #[serde(rename = "windowFeatures")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub window_features: Vec<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.windowOpen/parameters/userGesture。
    ///
    ///协议原文：
    ///
    ///<pre>Whether or not it was triggered by user gesture.</pre>
    ///
    ///
    #[serde(rename = "userGesture")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub user_gesture: bool,
}
impl<'de> serde::Deserialize<'de> for WindowOpenParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WindowOpenParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.windowOpen/parameters/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The URL for the new window.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.windowOpen/parameters/windowName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Window name.</pre>
                    ///
                    ///
                    #[serde(rename = "windowName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub window_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.windowOpen/parameters/windowFeatures。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An array of enabled window features.</pre>
                    ///
                    ///
                    #[serde(rename = "windowFeatures")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub window_features: Vec<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.event.windowOpen/parameters/userGesture。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether or not it was triggered by user gesture.</pre>
                    ///
                    ///
                    #[serde(rename = "userGesture")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub user_gesture: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(WindowOpenParams {
                    url: fields.url,
                    window_name: fields.window_name,
                    window_features: fields.window_features,
                    user_gesture: fields.user_gesture,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.event.windowOpen。
///
///协议原文：
///
///<pre>Fired when a new window is going to be opened, via window.open(), link click, form submission,
///etc.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct WindowOpen;
impl ydp_core::Event for WindowOpen {
    type Params = WindowOpenParams;
    const METHOD: &'static str = "Page.windowOpen";
}
