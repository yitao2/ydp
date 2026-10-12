///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Browser.event.downloadProgress/parameters/state。
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
    ///let value = ydp::browser::events::DownloadProgressParamsState::from(String::from("future-value"));
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
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.event.downloadProgress/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DownloadProgressParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.event.downloadProgress/parameters/guid。
    ///
    ///协议原文：
    ///
    ///<pre>Global unique identifier of the download.</pre>
    ///
    ///
    #[serde(rename = "guid")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub guid: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.event.downloadProgress/parameters/totalBytes。
    ///
    ///协议原文：
    ///
    ///<pre>Total expected bytes to download.</pre>
    ///
    ///
    #[serde(rename = "totalBytes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub total_bytes: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.event.downloadProgress/parameters/receivedBytes。
    ///
    ///协议原文：
    ///
    ///<pre>Total bytes received.</pre>
    ///
    ///
    #[serde(rename = "receivedBytes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub received_bytes: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.event.downloadProgress/parameters/state。
    ///
    ///协议原文：
    ///
    ///<pre>Download status.</pre>
    ///
    ///
    #[serde(rename = "state")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub state: DownloadProgressParamsState,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.event.downloadProgress/parameters/filePath。
    ///
    ///协议原文：
    ///
    ///<pre>If download is "completed", provides the path of the downloaded file.
    ///Depending on the platform, it is not guaranteed to be set, nor the file
    ///is guaranteed to exist.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "filePath")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub file_path: Option<String>,
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
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.event.downloadProgress/parameters/guid。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Global unique identifier of the download.</pre>
                    ///
                    ///
                    #[serde(rename = "guid")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub guid: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.event.downloadProgress/parameters/totalBytes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Total expected bytes to download.</pre>
                    ///
                    ///
                    #[serde(rename = "totalBytes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub total_bytes: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.event.downloadProgress/parameters/receivedBytes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Total bytes received.</pre>
                    ///
                    ///
                    #[serde(rename = "receivedBytes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub received_bytes: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.event.downloadProgress/parameters/state。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Download status.</pre>
                    ///
                    ///
                    #[serde(rename = "state")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub state: DownloadProgressParamsState,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Browser.event.downloadProgress/parameters/filePath。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If download is "completed", provides the path of the downloaded file.
                    ///Depending on the platform, it is not guaranteed to be set, nor the file
                    ///is guaranteed to exist.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "filePath")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub file_path: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DownloadProgressParams {
                    guid: fields.guid,
                    total_bytes: fields.total_bytes,
                    received_bytes: fields.received_bytes,
                    state: fields.state,
                    file_path: fields.file_path,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Browser.event.downloadProgress。
///
///协议原文：
///
///<pre>Fired when download makes progress. Last call has |done| == true.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct DownloadProgress;
impl ydp_core::Event for DownloadProgress {
    type Params = DownloadProgressParams;
    const METHOD: &'static str = "Browser.downloadProgress";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Browser.event.downloadWillBegin/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DownloadWillBeginParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.event.downloadWillBegin/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the frame that caused the download to begin.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.event.downloadWillBegin/parameters/guid。
    ///
    ///协议原文：
    ///
    ///<pre>Global unique identifier of the download.</pre>
    ///
    ///
    #[serde(rename = "guid")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub guid: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.event.downloadWillBegin/parameters/url。
    ///
    ///协议原文：
    ///
    ///<pre>URL of the resource being downloaded.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.event.downloadWillBegin/parameters/suggestedFilename。
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
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.event.downloadWillBegin/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the frame that caused the download to begin.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.event.downloadWillBegin/parameters/guid。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Global unique identifier of the download.</pre>
                    ///
                    ///
                    #[serde(rename = "guid")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub guid: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.event.downloadWillBegin/parameters/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>URL of the resource being downloaded.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Browser.event.downloadWillBegin/parameters/suggestedFilename。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Browser.event.downloadWillBegin。
///
///协议原文：
///
///<pre>Fired when page is about to start a download.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct DownloadWillBegin;
impl ydp_core::Event for DownloadWillBegin {
    type Params = DownloadWillBeginParams;
    const METHOD: &'static str = "Browser.downloadWillBegin";
}
