///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.addCompilationCache/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddCompilationCacheParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.addCompilationCache/parameters/url。
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.addCompilationCache/parameters/data。
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
impl<'de> serde::Deserialize<'de> for AddCompilationCacheParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddCompilationCacheParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.addCompilationCache/parameters/url。
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.addCompilationCache/parameters/data。
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
                Ok(AddCompilationCacheParams {
                    url: fields.url,
                    data: fields.data,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.addCompilationCache。
///
///协议原文：
///
///<pre>Seeds compilation cache for given url. Compilation cache does not survive
///cross-process navigation.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct AddCompilationCache;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.addCompilationCache/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddCompilationCacheResponse {}
impl<'de> serde::Deserialize<'de> for AddCompilationCacheResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddCompilationCacheResponse;
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
                Ok(AddCompilationCacheResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for AddCompilationCache {
    type Params = AddCompilationCacheParams;
    type Response = AddCompilationCacheResponse;
    const METHOD: &'static str = "Page.addCompilationCache";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.addScriptToEvaluateOnLoad/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddScriptToEvaluateOnLoadParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.addScriptToEvaluateOnLoad/parameters/scriptSource。
    #[serde(rename = "scriptSource")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub script_source: String,
}
impl<'de> serde::Deserialize<'de> for AddScriptToEvaluateOnLoadParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddScriptToEvaluateOnLoadParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.addScriptToEvaluateOnLoad/parameters/scriptSource。
                    #[serde(rename = "scriptSource")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub script_source: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AddScriptToEvaluateOnLoadParams {
                    script_source: fields.script_source,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.addScriptToEvaluateOnLoad。
///
///协议原文：
///
///<pre>Deprecated, please use addScriptToEvaluateOnNewDocument instead.</pre>
///
///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct AddScriptToEvaluateOnLoad;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.addScriptToEvaluateOnLoad/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddScriptToEvaluateOnLoadResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.addScriptToEvaluateOnLoad/returns/identifier。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the added script.</pre>
    ///
    ///
    #[serde(rename = "identifier")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub identifier: crate::page::types::ScriptIdentifier,
}
impl<'de> serde::Deserialize<'de> for AddScriptToEvaluateOnLoadResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddScriptToEvaluateOnLoadResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.addScriptToEvaluateOnLoad/returns/identifier。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the added script.</pre>
                    ///
                    ///
                    #[serde(rename = "identifier")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub identifier: crate::page::types::ScriptIdentifier,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AddScriptToEvaluateOnLoadResponse {
                    identifier: fields.identifier,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for AddScriptToEvaluateOnLoad {
    type Params = AddScriptToEvaluateOnLoadParams;
    type Response = AddScriptToEvaluateOnLoadResponse;
    const METHOD: &'static str = "Page.addScriptToEvaluateOnLoad";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.addScriptToEvaluateOnNewDocument/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddScriptToEvaluateOnNewDocumentParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.addScriptToEvaluateOnNewDocument/parameters/source。
    #[serde(rename = "source")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub source: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.addScriptToEvaluateOnNewDocument/parameters/worldName。
    ///
    ///协议原文：
    ///
    ///<pre>If specified, creates an isolated world with the given name and evaluates given script in it.
    ///This world name will be used as the ExecutionContextDescription::name when the corresponding
    ///event is emitted.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "worldName")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub world_name: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.addScriptToEvaluateOnNewDocument/parameters/includeCommandLineAPI。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies whether command line API should be available to the script, defaults
    ///to false.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "includeCommandLineAPI")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_command_line_api: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.addScriptToEvaluateOnNewDocument/parameters/runImmediately。
    ///
    ///协议原文：
    ///
    ///<pre>If true, runs the script immediately on existing execution contexts or worlds.
    ///Default: false.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "runImmediately")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub run_immediately: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for AddScriptToEvaluateOnNewDocumentParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddScriptToEvaluateOnNewDocumentParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.addScriptToEvaluateOnNewDocument/parameters/source。
                    #[serde(rename = "source")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub source: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.addScriptToEvaluateOnNewDocument/parameters/worldName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If specified, creates an isolated world with the given name and evaluates given script in it.
                    ///This world name will be used as the ExecutionContextDescription::name when the corresponding
                    ///event is emitted.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "worldName")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub world_name: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.addScriptToEvaluateOnNewDocument/parameters/includeCommandLineAPI。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies whether command line API should be available to the script, defaults
                    ///to false.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "includeCommandLineAPI")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub include_command_line_api: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.addScriptToEvaluateOnNewDocument/parameters/runImmediately。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, runs the script immediately on existing execution contexts or worlds.
                    ///Default: false.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "runImmediately")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub run_immediately: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AddScriptToEvaluateOnNewDocumentParams {
                    source: fields.source,
                    world_name: fields.world_name,
                    include_command_line_api: fields.include_command_line_api,
                    run_immediately: fields.run_immediately,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.addScriptToEvaluateOnNewDocument。
///
///协议原文：
///
///<pre>Evaluates given script in every frame upon creation (before loading frame's scripts).</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct AddScriptToEvaluateOnNewDocument;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.addScriptToEvaluateOnNewDocument/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddScriptToEvaluateOnNewDocumentResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.addScriptToEvaluateOnNewDocument/returns/identifier。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the added script.</pre>
    ///
    ///
    #[serde(rename = "identifier")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub identifier: crate::page::types::ScriptIdentifier,
}
impl<'de> serde::Deserialize<'de> for AddScriptToEvaluateOnNewDocumentResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddScriptToEvaluateOnNewDocumentResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.addScriptToEvaluateOnNewDocument/returns/identifier。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the added script.</pre>
                    ///
                    ///
                    #[serde(rename = "identifier")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub identifier: crate::page::types::ScriptIdentifier,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AddScriptToEvaluateOnNewDocumentResponse {
                    identifier: fields.identifier,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for AddScriptToEvaluateOnNewDocument {
    type Params = AddScriptToEvaluateOnNewDocumentParams;
    type Response = AddScriptToEvaluateOnNewDocumentResponse;
    const METHOD: &'static str = "Page.addScriptToEvaluateOnNewDocument";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.bringToFront/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct BringToFrontParams {}
impl<'de> serde::Deserialize<'de> for BringToFrontParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BringToFrontParams;
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
                Ok(BringToFrontParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.bringToFront。
///
///协议原文：
///
///<pre>Brings page to front (activates tab).</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct BringToFront;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.bringToFront/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct BringToFrontResponse {}
impl<'de> serde::Deserialize<'de> for BringToFrontResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BringToFrontResponse;
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
                Ok(BringToFrontResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for BringToFront {
    type Params = BringToFrontParams;
    type Response = BringToFrontResponse;
    const METHOD: &'static str = "Page.bringToFront";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.command.captureScreenshot/parameters/format。
///
///协议原文：
///
///<pre>Image compression format (defaults to png).</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CaptureScreenshotParamsFormat {
    ///协议值 "jpeg"。
    Jpeg,
    ///协议值 "png"。
    Png,
    ///协议值 "webp"。
    Webp,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl CaptureScreenshotParamsFormat {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::commands::CaptureScreenshotParamsFormat::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Jpeg => "jpeg",
            Self::Png => "png",
            Self::Webp => "webp",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for CaptureScreenshotParamsFormat {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for CaptureScreenshotParamsFormat {
    fn from(value: String) -> Self {
        match value.as_str() {
            "jpeg" => CaptureScreenshotParamsFormat::Jpeg,
            "png" => CaptureScreenshotParamsFormat::Png,
            "webp" => CaptureScreenshotParamsFormat::Webp,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for CaptureScreenshotParamsFormat {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CaptureScreenshotParamsFormat {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CaptureScreenshotParamsFormat;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "jpeg" => CaptureScreenshotParamsFormat::Jpeg,
                    "png" => CaptureScreenshotParamsFormat::Png,
                    "webp" => CaptureScreenshotParamsFormat::Webp,
                    _ => CaptureScreenshotParamsFormat::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(CaptureScreenshotParamsFormat::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.captureScreenshot/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CaptureScreenshotParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.captureScreenshot/parameters/format。
    ///
    ///协议原文：
    ///
    ///<pre>Image compression format (defaults to png).</pre>
    ///
    ///
    #[serde(rename = "format")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub format: Option<CaptureScreenshotParamsFormat>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.captureScreenshot/parameters/quality。
    ///
    ///协议原文：
    ///
    ///<pre>Compression quality from range [0..100] (jpeg only).</pre>
    ///
    ///
    #[serde(rename = "quality")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub quality: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.captureScreenshot/parameters/clip。
    ///
    ///协议原文：
    ///
    ///<pre>Capture the screenshot of a given region only.</pre>
    ///
    ///
    #[serde(rename = "clip")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub clip: Option<crate::page::types::Viewport>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.captureScreenshot/parameters/fromSurface。
    ///
    ///协议原文：
    ///
    ///<pre>Capture the screenshot from the surface, rather than the view. Defaults to true.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "fromSurface")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub from_surface: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.captureScreenshot/parameters/captureBeyondViewport。
    ///
    ///协议原文：
    ///
    ///<pre>Capture the screenshot beyond the viewport. Defaults to false.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "captureBeyondViewport")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub capture_beyond_viewport: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.captureScreenshot/parameters/optimizeForSpeed。
    ///
    ///协议原文：
    ///
    ///<pre>Optimize image encoding for speed, not for resulting size (defaults to false)</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "optimizeForSpeed")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub optimize_for_speed: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for CaptureScreenshotParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CaptureScreenshotParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.captureScreenshot/parameters/format。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Image compression format (defaults to png).</pre>
                    ///
                    ///
                    #[serde(rename = "format")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub format: Option<CaptureScreenshotParamsFormat>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.captureScreenshot/parameters/quality。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Compression quality from range [0..100] (jpeg only).</pre>
                    ///
                    ///
                    #[serde(rename = "quality")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub quality: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.captureScreenshot/parameters/clip。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Capture the screenshot of a given region only.</pre>
                    ///
                    ///
                    #[serde(rename = "clip")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub clip: Option<crate::page::types::Viewport>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.captureScreenshot/parameters/fromSurface。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Capture the screenshot from the surface, rather than the view. Defaults to true.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "fromSurface")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub from_surface: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.captureScreenshot/parameters/captureBeyondViewport。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Capture the screenshot beyond the viewport. Defaults to false.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "captureBeyondViewport")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub capture_beyond_viewport: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.captureScreenshot/parameters/optimizeForSpeed。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Optimize image encoding for speed, not for resulting size (defaults to false)</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "optimizeForSpeed")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub optimize_for_speed: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CaptureScreenshotParams {
                    format: fields.format,
                    quality: fields.quality,
                    clip: fields.clip,
                    from_surface: fields.from_surface,
                    capture_beyond_viewport: fields.capture_beyond_viewport,
                    optimize_for_speed: fields.optimize_for_speed,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.captureScreenshot。
///
///协议原文：
///
///<pre>Capture page screenshot.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct CaptureScreenshot;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.captureScreenshot/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CaptureScreenshotResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.captureScreenshot/returns/data。
    ///
    ///协议原文：
    ///
    ///<pre>Base64-encoded image data. (Encoded as a base64 string when passed over JSON)</pre>
    ///
    ///
    #[serde(rename = "data")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub data: String,
}
impl<'de> serde::Deserialize<'de> for CaptureScreenshotResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CaptureScreenshotResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.captureScreenshot/returns/data。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Base64-encoded image data. (Encoded as a base64 string when passed over JSON)</pre>
                    ///
                    ///
                    #[serde(rename = "data")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub data: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CaptureScreenshotResponse { data: fields.data })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for CaptureScreenshot {
    type Params = CaptureScreenshotParams;
    type Response = CaptureScreenshotResponse;
    const METHOD: &'static str = "Page.captureScreenshot";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.command.captureSnapshot/parameters/format。
///
///协议原文：
///
///<pre>Format (defaults to mhtml).</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CaptureSnapshotParamsFormat {
    ///协议值 "mhtml"。
    Mhtml,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl CaptureSnapshotParamsFormat {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::commands::CaptureSnapshotParamsFormat::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Mhtml => "mhtml",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for CaptureSnapshotParamsFormat {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for CaptureSnapshotParamsFormat {
    fn from(value: String) -> Self {
        match value.as_str() {
            "mhtml" => CaptureSnapshotParamsFormat::Mhtml,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for CaptureSnapshotParamsFormat {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CaptureSnapshotParamsFormat {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CaptureSnapshotParamsFormat;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "mhtml" => CaptureSnapshotParamsFormat::Mhtml,
                    _ => CaptureSnapshotParamsFormat::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(CaptureSnapshotParamsFormat::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.captureSnapshot/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CaptureSnapshotParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.captureSnapshot/parameters/format。
    ///
    ///协议原文：
    ///
    ///<pre>Format (defaults to mhtml).</pre>
    ///
    ///
    #[serde(rename = "format")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub format: Option<CaptureSnapshotParamsFormat>,
}
impl<'de> serde::Deserialize<'de> for CaptureSnapshotParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CaptureSnapshotParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.captureSnapshot/parameters/format。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Format (defaults to mhtml).</pre>
                    ///
                    ///
                    #[serde(rename = "format")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub format: Option<CaptureSnapshotParamsFormat>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CaptureSnapshotParams {
                    format: fields.format,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.captureSnapshot。
///
///协议原文：
///
///<pre>Returns a snapshot of the page as a string. For MHTML format, the serialization includes
///iframes, shadow DOM, external resources, and element-inline styles.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct CaptureSnapshot;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.captureSnapshot/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CaptureSnapshotResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.captureSnapshot/returns/data。
    ///
    ///协议原文：
    ///
    ///<pre>Serialized page data.</pre>
    ///
    ///
    #[serde(rename = "data")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub data: String,
}
impl<'de> serde::Deserialize<'de> for CaptureSnapshotResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CaptureSnapshotResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.captureSnapshot/returns/data。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Serialized page data.</pre>
                    ///
                    ///
                    #[serde(rename = "data")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub data: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CaptureSnapshotResponse { data: fields.data })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for CaptureSnapshot {
    type Params = CaptureSnapshotParams;
    type Response = CaptureSnapshotResponse;
    const METHOD: &'static str = "Page.captureSnapshot";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.clearCompilationCache/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ClearCompilationCacheParams {}
impl<'de> serde::Deserialize<'de> for ClearCompilationCacheParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearCompilationCacheParams;
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
                Ok(ClearCompilationCacheParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.clearCompilationCache。
///
///协议原文：
///
///<pre>Clears seeded compilation cache.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ClearCompilationCache;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.clearCompilationCache/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearCompilationCacheResponse {}
impl<'de> serde::Deserialize<'de> for ClearCompilationCacheResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearCompilationCacheResponse;
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
                Ok(ClearCompilationCacheResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ClearCompilationCache {
    type Params = ClearCompilationCacheParams;
    type Response = ClearCompilationCacheResponse;
    const METHOD: &'static str = "Page.clearCompilationCache";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.clearDeviceMetricsOverride/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ClearDeviceMetricsOverrideParams {}
impl<'de> serde::Deserialize<'de> for ClearDeviceMetricsOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearDeviceMetricsOverrideParams;
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
                Ok(ClearDeviceMetricsOverrideParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.clearDeviceMetricsOverride。
///
///协议原文：
///
///<pre>Clears the overridden device metrics.</pre>
///
///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。此命令由协议重定向到 Emulation 域处理。
#[derive(Debug, Clone, Copy)]
pub struct ClearDeviceMetricsOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.clearDeviceMetricsOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearDeviceMetricsOverrideResponse {}
impl<'de> serde::Deserialize<'de> for ClearDeviceMetricsOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearDeviceMetricsOverrideResponse;
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
                Ok(ClearDeviceMetricsOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ClearDeviceMetricsOverride {
    type Params = ClearDeviceMetricsOverrideParams;
    type Response = ClearDeviceMetricsOverrideResponse;
    const METHOD: &'static str = "Page.clearDeviceMetricsOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.clearDeviceOrientationOverride/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ClearDeviceOrientationOverrideParams {}
impl<'de> serde::Deserialize<'de> for ClearDeviceOrientationOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearDeviceOrientationOverrideParams;
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
                Ok(ClearDeviceOrientationOverrideParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.clearDeviceOrientationOverride。
///
///协议原文：
///
///<pre>Clears the overridden Device Orientation.</pre>
///
///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。此命令由协议重定向到 DeviceOrientation 域处理。
#[derive(Debug, Clone, Copy)]
pub struct ClearDeviceOrientationOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.clearDeviceOrientationOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearDeviceOrientationOverrideResponse {}
impl<'de> serde::Deserialize<'de> for ClearDeviceOrientationOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearDeviceOrientationOverrideResponse;
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
                Ok(ClearDeviceOrientationOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ClearDeviceOrientationOverride {
    type Params = ClearDeviceOrientationOverrideParams;
    type Response = ClearDeviceOrientationOverrideResponse;
    const METHOD: &'static str = "Page.clearDeviceOrientationOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.clearGeolocationOverride/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ClearGeolocationOverrideParams {}
impl<'de> serde::Deserialize<'de> for ClearGeolocationOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearGeolocationOverrideParams;
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
                Ok(ClearGeolocationOverrideParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.clearGeolocationOverride。
///
///协议原文：
///
///<pre>Clears the overridden Geolocation Position and Error.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。此命令由协议重定向到 Emulation 域处理。
#[derive(Debug, Clone, Copy)]
pub struct ClearGeolocationOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.clearGeolocationOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearGeolocationOverrideResponse {}
impl<'de> serde::Deserialize<'de> for ClearGeolocationOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearGeolocationOverrideResponse;
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
                Ok(ClearGeolocationOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ClearGeolocationOverride {
    type Params = ClearGeolocationOverrideParams;
    type Response = ClearGeolocationOverrideResponse;
    const METHOD: &'static str = "Page.clearGeolocationOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.close/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CloseParams {}
impl<'de> serde::Deserialize<'de> for CloseParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CloseParams;
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
                Ok(CloseParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.close。
///
///协议原文：
///
///<pre>Tries to close page, running its beforeunload hooks, if any.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Close;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.close/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CloseResponse {}
impl<'de> serde::Deserialize<'de> for CloseResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CloseResponse;
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
                Ok(CloseResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Close {
    type Params = CloseParams;
    type Response = CloseResponse;
    const METHOD: &'static str = "Page.close";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.crash/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CrashParams {}
impl<'de> serde::Deserialize<'de> for CrashParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CrashParams;
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
                Ok(CrashParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.crash。
///
///协议原文：
///
///<pre>Crashes renderer on the IO thread, generates minidumps.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct Crash;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.crash/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CrashResponse {}
impl<'de> serde::Deserialize<'de> for CrashResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CrashResponse;
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
                Ok(CrashResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Crash {
    type Params = CrashParams;
    type Response = CrashResponse;
    const METHOD: &'static str = "Page.crash";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.createIsolatedWorld/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CreateIsolatedWorldParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.createIsolatedWorld/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the frame in which the isolated world should be created.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.createIsolatedWorld/parameters/worldName。
    ///
    ///协议原文：
    ///
    ///<pre>An optional name which is reported in the Execution Context.</pre>
    ///
    ///
    #[serde(rename = "worldName")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub world_name: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.createIsolatedWorld/parameters/grantUniveralAccess。
    ///
    ///协议原文：
    ///
    ///<pre>Whether or not universal access should be granted to the isolated world. This is a powerful
    ///option, use with caution.</pre>
    ///
    ///
    #[serde(rename = "grantUniveralAccess")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub grant_univeral_access: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.createIsolatedWorld/parameters/contentSecurityPolicy。
    ///
    ///协议原文：
    ///
    ///<pre>An optional content security policy to set for the isolated world.
    ///If omitted, any existing CSP for the world will be cleared.
    ///Note that clearing or updating the CSP does not immediately affect the active
    ///context in the same document because LocalDOMWindow caches the
    ///ContentSecurityPolicy object. The change takes effect on subsequent
    ///navigations when a new window context is created.</pre>
    ///
    ///
    #[serde(rename = "contentSecurityPolicy")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub content_security_policy: Option<String>,
}
impl<'de> serde::Deserialize<'de> for CreateIsolatedWorldParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CreateIsolatedWorldParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.createIsolatedWorld/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the frame in which the isolated world should be created.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.createIsolatedWorld/parameters/worldName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An optional name which is reported in the Execution Context.</pre>
                    ///
                    ///
                    #[serde(rename = "worldName")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub world_name: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.createIsolatedWorld/parameters/grantUniveralAccess。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether or not universal access should be granted to the isolated world. This is a powerful
                    ///option, use with caution.</pre>
                    ///
                    ///
                    #[serde(rename = "grantUniveralAccess")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub grant_univeral_access: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.createIsolatedWorld/parameters/contentSecurityPolicy。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An optional content security policy to set for the isolated world.
                    ///If omitted, any existing CSP for the world will be cleared.
                    ///Note that clearing or updating the CSP does not immediately affect the active
                    ///context in the same document because LocalDOMWindow caches the
                    ///ContentSecurityPolicy object. The change takes effect on subsequent
                    ///navigations when a new window context is created.</pre>
                    ///
                    ///
                    #[serde(rename = "contentSecurityPolicy")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub content_security_policy: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CreateIsolatedWorldParams {
                    frame_id: fields.frame_id,
                    world_name: fields.world_name,
                    grant_univeral_access: fields.grant_univeral_access,
                    content_security_policy: fields.content_security_policy,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.createIsolatedWorld。
///
///协议原文：
///
///<pre>Creates an isolated world for the given frame.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct CreateIsolatedWorld;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.createIsolatedWorld/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CreateIsolatedWorldResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.createIsolatedWorld/returns/executionContextId。
    ///
    ///协议原文：
    ///
    ///<pre>Execution context of the isolated world.</pre>
    ///
    ///
    #[serde(rename = "executionContextId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub execution_context_id: crate::runtime::types::ExecutionContextId,
}
impl<'de> serde::Deserialize<'de> for CreateIsolatedWorldResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CreateIsolatedWorldResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.createIsolatedWorld/returns/executionContextId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Execution context of the isolated world.</pre>
                    ///
                    ///
                    #[serde(rename = "executionContextId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub execution_context_id: crate::runtime::types::ExecutionContextId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CreateIsolatedWorldResponse {
                    execution_context_id: fields.execution_context_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for CreateIsolatedWorld {
    type Params = CreateIsolatedWorldParams;
    type Response = CreateIsolatedWorldResponse;
    const METHOD: &'static str = "Page.createIsolatedWorld";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.deleteCookie/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeleteCookieParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.deleteCookie/parameters/cookieName。
    ///
    ///协议原文：
    ///
    ///<pre>Name of the cookie to remove.</pre>
    ///
    ///
    #[serde(rename = "cookieName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cookie_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.deleteCookie/parameters/url。
    ///
    ///协议原文：
    ///
    ///<pre>URL to match cooke domain and path.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
}
impl<'de> serde::Deserialize<'de> for DeleteCookieParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeleteCookieParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.deleteCookie/parameters/cookieName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Name of the cookie to remove.</pre>
                    ///
                    ///
                    #[serde(rename = "cookieName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cookie_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.deleteCookie/parameters/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>URL to match cooke domain and path.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DeleteCookieParams {
                    cookie_name: fields.cookie_name,
                    url: fields.url,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.deleteCookie。
///
///协议原文：
///
///<pre>Deletes browser cookie with given name, domain and path.</pre>
///
///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。此命令由协议重定向到 Network 域处理。
#[derive(Debug, Clone, Copy)]
pub struct DeleteCookie;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.deleteCookie/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeleteCookieResponse {}
impl<'de> serde::Deserialize<'de> for DeleteCookieResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeleteCookieResponse;
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
                Ok(DeleteCookieResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for DeleteCookie {
    type Params = DeleteCookieParams;
    type Response = DeleteCookieResponse;
    const METHOD: &'static str = "Page.deleteCookie";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.disable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.disable。
///
///协议原文：
///
///<pre>Disables page domain notifications.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.disable/returns。
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
    const METHOD: &'static str = "Page.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.enable/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct EnableParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.enable/parameters/enableFileChooserOpenedEvent。
    ///
    ///协议原文：
    ///
    ///<pre>If true, the `Page.fileChooserOpened` event will be emitted regardless of the state set by
    ///`Page.setInterceptFileChooserDialog` command (default: false).</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "enableFileChooserOpenedEvent")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_file_chooser_opened_event: Option<bool>,
}
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
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.enable/parameters/enableFileChooserOpenedEvent。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, the `Page.fileChooserOpened` event will be emitted regardless of the state set by
                    ///`Page.setInterceptFileChooserDialog` command (default: false).</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "enableFileChooserOpenedEvent")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub enable_file_chooser_opened_event: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EnableParams {
                    enable_file_chooser_opened_event: fields.enable_file_chooser_opened_event,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.enable。
///
///协议原文：
///
///<pre>Enables page domain notifications.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.enable/returns。
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
    const METHOD: &'static str = "Page.enable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.generateTestReport/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GenerateTestReportParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.generateTestReport/parameters/message。
    ///
    ///协议原文：
    ///
    ///<pre>Message to be displayed in the report.</pre>
    ///
    ///
    #[serde(rename = "message")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub message: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.generateTestReport/parameters/group。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies the endpoint group to deliver the report to.</pre>
    ///
    ///
    #[serde(rename = "group")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub group: Option<String>,
}
impl<'de> serde::Deserialize<'de> for GenerateTestReportParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GenerateTestReportParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.generateTestReport/parameters/message。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Message to be displayed in the report.</pre>
                    ///
                    ///
                    #[serde(rename = "message")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub message: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.generateTestReport/parameters/group。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies the endpoint group to deliver the report to.</pre>
                    ///
                    ///
                    #[serde(rename = "group")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub group: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GenerateTestReportParams {
                    message: fields.message,
                    group: fields.group,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.generateTestReport。
///
///协议原文：
///
///<pre>Generates a report for testing.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GenerateTestReport;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.generateTestReport/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GenerateTestReportResponse {}
impl<'de> serde::Deserialize<'de> for GenerateTestReportResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GenerateTestReportResponse;
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
                Ok(GenerateTestReportResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GenerateTestReport {
    type Params = GenerateTestReportParams;
    type Response = GenerateTestReportResponse;
    const METHOD: &'static str = "Page.generateTestReport";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.getAdScriptAncestry/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetAdScriptAncestryParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getAdScriptAncestry/parameters/frameId。
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
}
impl<'de> serde::Deserialize<'de> for GetAdScriptAncestryParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetAdScriptAncestryParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getAdScriptAncestry/parameters/frameId。
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetAdScriptAncestryParams {
                    frame_id: fields.frame_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.getAdScriptAncestry。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetAdScriptAncestry;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.getAdScriptAncestry/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetAdScriptAncestryResponse {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.getAdScriptAncestry/returns/adScriptAncestry。
    ///
    ///协议原文：
    ///
    ///<pre>The ancestry chain of ad script identifiers leading to this frame's
    ///creation, along with the root script's filterlist rule. The ancestry
    ///chain is ordered from the most immediate script (in the frame creation
    ///stack) to more distant ancestors (that created the immediately preceding
    ///script). Only sent if frame is labelled as an ad and ids are available.</pre>
    ///
    ///
    #[serde(rename = "adScriptAncestry")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub ad_script_ancestry: Option<crate::network::types::AdAncestry>,
}
impl<'de> serde::Deserialize<'de> for GetAdScriptAncestryResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetAdScriptAncestryResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.getAdScriptAncestry/returns/adScriptAncestry。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The ancestry chain of ad script identifiers leading to this frame's
                    ///creation, along with the root script's filterlist rule. The ancestry
                    ///chain is ordered from the most immediate script (in the frame creation
                    ///stack) to more distant ancestors (that created the immediately preceding
                    ///script). Only sent if frame is labelled as an ad and ids are available.</pre>
                    ///
                    ///
                    #[serde(rename = "adScriptAncestry")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub ad_script_ancestry: Option<crate::network::types::AdAncestry>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetAdScriptAncestryResponse {
                    ad_script_ancestry: fields.ad_script_ancestry,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetAdScriptAncestry {
    type Params = GetAdScriptAncestryParams;
    type Response = GetAdScriptAncestryResponse;
    const METHOD: &'static str = "Page.getAdScriptAncestry";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.getAnnotatedPageContent/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetAnnotatedPageContentParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.getAnnotatedPageContent/parameters/includeActionableInformation。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to include actionable information. Defaults to true.</pre>
    ///
    ///
    #[serde(rename = "includeActionableInformation")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_actionable_information: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for GetAnnotatedPageContentParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetAnnotatedPageContentParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.getAnnotatedPageContent/parameters/includeActionableInformation。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to include actionable information. Defaults to true.</pre>
                    ///
                    ///
                    #[serde(rename = "includeActionableInformation")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub include_actionable_information: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetAnnotatedPageContentParams {
                    include_actionable_information: fields.include_actionable_information,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.getAnnotatedPageContent。
///
///协议原文：
///
///<pre>Get the annotated page content for the main frame.
///This is an experimental command that is subject to change.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetAnnotatedPageContent;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.getAnnotatedPageContent/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetAnnotatedPageContentResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getAnnotatedPageContent/returns/content。
    ///
    ///协议原文：
    ///
    ///<pre>The annotated page content as a base64 encoded protobuf.
    ///The format is defined by the `AnnotatedPageContent` message in
    ///components/optimization_guide/proto/features/common_quality_data.proto (Encoded as a base64 string when passed over JSON)</pre>
    ///
    ///
    #[serde(rename = "content")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub content: String,
}
impl<'de> serde::Deserialize<'de> for GetAnnotatedPageContentResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetAnnotatedPageContentResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getAnnotatedPageContent/returns/content。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The annotated page content as a base64 encoded protobuf.
                    ///The format is defined by the `AnnotatedPageContent` message in
                    ///components/optimization_guide/proto/features/common_quality_data.proto (Encoded as a base64 string when passed over JSON)</pre>
                    ///
                    ///
                    #[serde(rename = "content")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub content: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetAnnotatedPageContentResponse {
                    content: fields.content,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetAnnotatedPageContent {
    type Params = GetAnnotatedPageContentParams;
    type Response = GetAnnotatedPageContentResponse;
    const METHOD: &'static str = "Page.getAnnotatedPageContent";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.getAppId/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetAppIdParams {}
impl<'de> serde::Deserialize<'de> for GetAppIdParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetAppIdParams;
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
                Ok(GetAppIdParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.getAppId。
///
///协议原文：
///
///<pre>Returns the unique (PWA) app id.
///Only returns values if the feature flag 'WebAppEnableManifestId' is enabled</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetAppId;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.getAppId/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetAppIdResponse {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.getAppId/returns/appId。
    ///
    ///协议原文：
    ///
    ///<pre>App id, either from manifest's id attribute or computed from start_url</pre>
    ///
    ///
    #[serde(rename = "appId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_id: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.getAppId/returns/recommendedId。
    ///
    ///协议原文：
    ///
    ///<pre>Recommendation for manifest's id attribute to match current id computed from start_url</pre>
    ///
    ///
    #[serde(rename = "recommendedId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub recommended_id: Option<String>,
}
impl<'de> serde::Deserialize<'de> for GetAppIdResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetAppIdResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.getAppId/returns/appId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>App id, either from manifest's id attribute or computed from start_url</pre>
                    ///
                    ///
                    #[serde(rename = "appId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub app_id: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.getAppId/returns/recommendedId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Recommendation for manifest's id attribute to match current id computed from start_url</pre>
                    ///
                    ///
                    #[serde(rename = "recommendedId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub recommended_id: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetAppIdResponse {
                    app_id: fields.app_id,
                    recommended_id: fields.recommended_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetAppId {
    type Params = GetAppIdParams;
    type Response = GetAppIdResponse;
    const METHOD: &'static str = "Page.getAppId";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.getAppManifest/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetAppManifestParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.getAppManifest/parameters/manifestId。
    #[serde(rename = "manifestId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub manifest_id: Option<String>,
}
impl<'de> serde::Deserialize<'de> for GetAppManifestParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetAppManifestParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.getAppManifest/parameters/manifestId。
                    #[serde(rename = "manifestId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub manifest_id: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetAppManifestParams {
                    manifest_id: fields.manifest_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.getAppManifest。
///
///协议原文：
///
///<pre>Gets the processed manifest for this current document.
///  This API always waits for the manifest to be loaded.
///  If manifestId is provided, and it does not match the manifest of the
///    current document, this API errors out.
///  If there is not a loaded page, this API errors out immediately.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetAppManifest;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.getAppManifest/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetAppManifestResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getAppManifest/returns/url。
    ///
    ///协议原文：
    ///
    ///<pre>Manifest location.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getAppManifest/returns/errors。
    #[serde(rename = "errors")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub errors: Vec<crate::page::types::AppManifestError>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.getAppManifest/returns/data。
    ///
    ///协议原文：
    ///
    ///<pre>Manifest content.</pre>
    ///
    ///
    #[serde(rename = "data")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub data: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.getAppManifest/returns/parsed。
    ///
    ///协议原文：
    ///
    ///<pre>Parsed manifest properties. Deprecated, use manifest instead.</pre>
    ///
    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "parsed")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub parsed: Option<crate::page::types::AppManifestParsedProperties>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getAppManifest/returns/manifest。此项为 experimental，端点可能不支持。
    #[serde(rename = "manifest")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub manifest: crate::page::types::WebAppManifest,
}
impl<'de> serde::Deserialize<'de> for GetAppManifestResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetAppManifestResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getAppManifest/returns/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Manifest location.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getAppManifest/returns/errors。
                    #[serde(rename = "errors")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub errors: Vec<crate::page::types::AppManifestError>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.getAppManifest/returns/data。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Manifest content.</pre>
                    ///
                    ///
                    #[serde(rename = "data")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub data: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.getAppManifest/returns/parsed。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Parsed manifest properties. Deprecated, use manifest instead.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "parsed")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub parsed: Option<crate::page::types::AppManifestParsedProperties>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getAppManifest/returns/manifest。此项为 experimental，端点可能不支持。
                    #[serde(rename = "manifest")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub manifest: crate::page::types::WebAppManifest,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetAppManifestResponse {
                    url: fields.url,
                    errors: fields.errors,
                    data: fields.data,
                    parsed: fields.parsed,
                    manifest: fields.manifest,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetAppManifest {
    type Params = GetAppManifestParams;
    type Response = GetAppManifestResponse;
    const METHOD: &'static str = "Page.getAppManifest";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.getFrameTree/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetFrameTreeParams {}
impl<'de> serde::Deserialize<'de> for GetFrameTreeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetFrameTreeParams;
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
                Ok(GetFrameTreeParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.getFrameTree。
///
///协议原文：
///
///<pre>Returns present frame tree structure.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetFrameTree;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.getFrameTree/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetFrameTreeResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getFrameTree/returns/frameTree。
    ///
    ///协议原文：
    ///
    ///<pre>Present frame tree structure.</pre>
    ///
    ///
    #[serde(rename = "frameTree")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_tree: crate::page::types::FrameTree,
}
impl<'de> serde::Deserialize<'de> for GetFrameTreeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetFrameTreeResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getFrameTree/returns/frameTree。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Present frame tree structure.</pre>
                    ///
                    ///
                    #[serde(rename = "frameTree")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_tree: crate::page::types::FrameTree,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetFrameTreeResponse {
                    frame_tree: fields.frame_tree,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetFrameTree {
    type Params = GetFrameTreeParams;
    type Response = GetFrameTreeResponse;
    const METHOD: &'static str = "Page.getFrameTree";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.getInstallabilityErrors/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetInstallabilityErrorsParams {}
impl<'de> serde::Deserialize<'de> for GetInstallabilityErrorsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetInstallabilityErrorsParams;
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
                Ok(GetInstallabilityErrorsParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.getInstallabilityErrors。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetInstallabilityErrors;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.getInstallabilityErrors/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetInstallabilityErrorsResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getInstallabilityErrors/returns/installabilityErrors。
    #[serde(rename = "installabilityErrors")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub installability_errors: Vec<crate::page::types::InstallabilityError>,
}
impl<'de> serde::Deserialize<'de> for GetInstallabilityErrorsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetInstallabilityErrorsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getInstallabilityErrors/returns/installabilityErrors。
                    #[serde(rename = "installabilityErrors")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub installability_errors: Vec<crate::page::types::InstallabilityError>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetInstallabilityErrorsResponse {
                    installability_errors: fields.installability_errors,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetInstallabilityErrors {
    type Params = GetInstallabilityErrorsParams;
    type Response = GetInstallabilityErrorsResponse;
    const METHOD: &'static str = "Page.getInstallabilityErrors";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.getLayoutMetrics/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetLayoutMetricsParams {}
impl<'de> serde::Deserialize<'de> for GetLayoutMetricsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetLayoutMetricsParams;
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
                Ok(GetLayoutMetricsParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.getLayoutMetrics。
///
///协议原文：
///
///<pre>Returns metrics relating to the layouting of the page, such as viewport bounds/scale.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetLayoutMetrics;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.getLayoutMetrics/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetLayoutMetricsResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getLayoutMetrics/returns/layoutViewport。
    ///
    ///协议原文：
    ///
    ///<pre>Deprecated metrics relating to the layout viewport. Is in device pixels. Use `cssLayoutViewport` instead.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "layoutViewport")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub layout_viewport: crate::page::types::LayoutViewport,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getLayoutMetrics/returns/visualViewport。
    ///
    ///协议原文：
    ///
    ///<pre>Deprecated metrics relating to the visual viewport. Is in device pixels. Use `cssVisualViewport` instead.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "visualViewport")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub visual_viewport: crate::page::types::VisualViewport,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getLayoutMetrics/returns/contentSize。
    ///
    ///协议原文：
    ///
    ///<pre>Deprecated size of scrollable area. Is in DP. Use `cssContentSize` instead.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "contentSize")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub content_size: crate::dom::types::Rect,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getLayoutMetrics/returns/cssLayoutViewport。
    ///
    ///协议原文：
    ///
    ///<pre>Metrics relating to the layout viewport in CSS pixels.</pre>
    ///
    ///
    #[serde(rename = "cssLayoutViewport")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub css_layout_viewport: crate::page::types::LayoutViewport,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getLayoutMetrics/returns/cssVisualViewport。
    ///
    ///协议原文：
    ///
    ///<pre>Metrics relating to the visual viewport in CSS pixels.</pre>
    ///
    ///
    #[serde(rename = "cssVisualViewport")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub css_visual_viewport: crate::page::types::VisualViewport,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getLayoutMetrics/returns/cssContentSize。
    ///
    ///协议原文：
    ///
    ///<pre>Size of scrollable area in CSS pixels.</pre>
    ///
    ///
    #[serde(rename = "cssContentSize")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub css_content_size: crate::dom::types::Rect,
}
impl<'de> serde::Deserialize<'de> for GetLayoutMetricsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetLayoutMetricsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getLayoutMetrics/returns/layoutViewport。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Deprecated metrics relating to the layout viewport. Is in device pixels. Use `cssLayoutViewport` instead.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "layoutViewport")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub layout_viewport: crate::page::types::LayoutViewport,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getLayoutMetrics/returns/visualViewport。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Deprecated metrics relating to the visual viewport. Is in device pixels. Use `cssVisualViewport` instead.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "visualViewport")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub visual_viewport: crate::page::types::VisualViewport,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getLayoutMetrics/returns/contentSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Deprecated size of scrollable area. Is in DP. Use `cssContentSize` instead.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "contentSize")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub content_size: crate::dom::types::Rect,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getLayoutMetrics/returns/cssLayoutViewport。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Metrics relating to the layout viewport in CSS pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "cssLayoutViewport")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub css_layout_viewport: crate::page::types::LayoutViewport,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getLayoutMetrics/returns/cssVisualViewport。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Metrics relating to the visual viewport in CSS pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "cssVisualViewport")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub css_visual_viewport: crate::page::types::VisualViewport,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getLayoutMetrics/returns/cssContentSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Size of scrollable area in CSS pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "cssContentSize")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub css_content_size: crate::dom::types::Rect,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetLayoutMetricsResponse {
                    layout_viewport: fields.layout_viewport,
                    visual_viewport: fields.visual_viewport,
                    content_size: fields.content_size,
                    css_layout_viewport: fields.css_layout_viewport,
                    css_visual_viewport: fields.css_visual_viewport,
                    css_content_size: fields.css_content_size,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetLayoutMetrics {
    type Params = GetLayoutMetricsParams;
    type Response = GetLayoutMetricsResponse;
    const METHOD: &'static str = "Page.getLayoutMetrics";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.getManifestIcons/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetManifestIconsParams {}
impl<'de> serde::Deserialize<'de> for GetManifestIconsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetManifestIconsParams;
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
                Ok(GetManifestIconsParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.getManifestIcons。
///
///协议原文：
///
///<pre>Deprecated because it's not guaranteed that the returned icon is in fact the one used for PWA installation.</pre>
///
///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct GetManifestIcons;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.getManifestIcons/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetManifestIconsResponse {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.getManifestIcons/returns/primaryIcon。
    #[serde(rename = "primaryIcon")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_icon: Option<String>,
}
impl<'de> serde::Deserialize<'de> for GetManifestIconsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetManifestIconsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.getManifestIcons/returns/primaryIcon。
                    #[serde(rename = "primaryIcon")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub primary_icon: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetManifestIconsResponse {
                    primary_icon: fields.primary_icon,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetManifestIcons {
    type Params = GetManifestIconsParams;
    type Response = GetManifestIconsResponse;
    const METHOD: &'static str = "Page.getManifestIcons";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.getNavigationHistory/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetNavigationHistoryParams {}
impl<'de> serde::Deserialize<'de> for GetNavigationHistoryParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetNavigationHistoryParams;
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
                Ok(GetNavigationHistoryParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.getNavigationHistory。
///
///协议原文：
///
///<pre>Returns navigation history for the current page.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetNavigationHistory;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.getNavigationHistory/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetNavigationHistoryResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getNavigationHistory/returns/currentIndex。
    ///
    ///协议原文：
    ///
    ///<pre>Index of the current navigation history entry.</pre>
    ///
    ///
    #[serde(rename = "currentIndex")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub current_index: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getNavigationHistory/returns/entries。
    ///
    ///协议原文：
    ///
    ///<pre>Array of navigation history entries.</pre>
    ///
    ///
    #[serde(rename = "entries")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub entries: Vec<crate::page::types::NavigationEntry>,
}
impl<'de> serde::Deserialize<'de> for GetNavigationHistoryResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetNavigationHistoryResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getNavigationHistory/returns/currentIndex。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Index of the current navigation history entry.</pre>
                    ///
                    ///
                    #[serde(rename = "currentIndex")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub current_index: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getNavigationHistory/returns/entries。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Array of navigation history entries.</pre>
                    ///
                    ///
                    #[serde(rename = "entries")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub entries: Vec<crate::page::types::NavigationEntry>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetNavigationHistoryResponse {
                    current_index: fields.current_index,
                    entries: fields.entries,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetNavigationHistory {
    type Params = GetNavigationHistoryParams;
    type Response = GetNavigationHistoryResponse;
    const METHOD: &'static str = "Page.getNavigationHistory";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.getOriginTrials/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetOriginTrialsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getOriginTrials/parameters/frameId。
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
}
impl<'de> serde::Deserialize<'de> for GetOriginTrialsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetOriginTrialsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getOriginTrials/parameters/frameId。
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetOriginTrialsParams {
                    frame_id: fields.frame_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.getOriginTrials。
///
///协议原文：
///
///<pre>Get Origin Trials on given frame.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetOriginTrials;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.getOriginTrials/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetOriginTrialsResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getOriginTrials/returns/originTrials。
    #[serde(rename = "originTrials")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin_trials: Vec<crate::page::types::OriginTrial>,
}
impl<'de> serde::Deserialize<'de> for GetOriginTrialsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetOriginTrialsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getOriginTrials/returns/originTrials。
                    #[serde(rename = "originTrials")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin_trials: Vec<crate::page::types::OriginTrial>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetOriginTrialsResponse {
                    origin_trials: fields.origin_trials,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetOriginTrials {
    type Params = GetOriginTrialsParams;
    type Response = GetOriginTrialsResponse;
    const METHOD: &'static str = "Page.getOriginTrials";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.getPermissionsPolicyState/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetPermissionsPolicyStateParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getPermissionsPolicyState/parameters/frameId。
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
}
impl<'de> serde::Deserialize<'de> for GetPermissionsPolicyStateParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetPermissionsPolicyStateParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getPermissionsPolicyState/parameters/frameId。
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetPermissionsPolicyStateParams {
                    frame_id: fields.frame_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.getPermissionsPolicyState。
///
///协议原文：
///
///<pre>Get Permissions Policy state on given frame.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetPermissionsPolicyState;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.getPermissionsPolicyState/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetPermissionsPolicyStateResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getPermissionsPolicyState/returns/states。
    #[serde(rename = "states")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub states: Vec<crate::page::types::PermissionsPolicyFeatureState>,
}
impl<'de> serde::Deserialize<'de> for GetPermissionsPolicyStateResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetPermissionsPolicyStateResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getPermissionsPolicyState/returns/states。
                    #[serde(rename = "states")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub states: Vec<crate::page::types::PermissionsPolicyFeatureState>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetPermissionsPolicyStateResponse {
                    states: fields.states,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetPermissionsPolicyState {
    type Params = GetPermissionsPolicyStateParams;
    type Response = GetPermissionsPolicyStateResponse;
    const METHOD: &'static str = "Page.getPermissionsPolicyState";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.getResourceContent/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetResourceContentParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getResourceContent/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Frame id to get resource for.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getResourceContent/parameters/url。
    ///
    ///协议原文：
    ///
    ///<pre>URL of the resource to get content for.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
}
impl<'de> serde::Deserialize<'de> for GetResourceContentParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetResourceContentParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getResourceContent/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame id to get resource for.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getResourceContent/parameters/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>URL of the resource to get content for.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetResourceContentParams {
                    frame_id: fields.frame_id,
                    url: fields.url,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.getResourceContent。
///
///协议原文：
///
///<pre>Returns content of the given resource.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetResourceContent;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.getResourceContent/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetResourceContentResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getResourceContent/returns/content。
    ///
    ///协议原文：
    ///
    ///<pre>Resource content.</pre>
    ///
    ///
    #[serde(rename = "content")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub content: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getResourceContent/returns/base64Encoded。
    ///
    ///协议原文：
    ///
    ///<pre>True, if content was served as base64.</pre>
    ///
    ///
    #[serde(rename = "base64Encoded")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub base64_encoded: bool,
}
impl<'de> serde::Deserialize<'de> for GetResourceContentResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetResourceContentResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getResourceContent/returns/content。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Resource content.</pre>
                    ///
                    ///
                    #[serde(rename = "content")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub content: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getResourceContent/returns/base64Encoded。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True, if content was served as base64.</pre>
                    ///
                    ///
                    #[serde(rename = "base64Encoded")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub base64_encoded: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetResourceContentResponse {
                    content: fields.content,
                    base64_encoded: fields.base64_encoded,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetResourceContent {
    type Params = GetResourceContentParams;
    type Response = GetResourceContentResponse;
    const METHOD: &'static str = "Page.getResourceContent";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.getResourceTree/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetResourceTreeParams {}
impl<'de> serde::Deserialize<'de> for GetResourceTreeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetResourceTreeParams;
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
                Ok(GetResourceTreeParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.getResourceTree。
///
///协议原文：
///
///<pre>Returns present frame / resource tree structure.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetResourceTree;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.getResourceTree/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetResourceTreeResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getResourceTree/returns/frameTree。
    ///
    ///协议原文：
    ///
    ///<pre>Present frame / resource tree structure.</pre>
    ///
    ///
    #[serde(rename = "frameTree")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_tree: crate::page::types::FrameResourceTree,
}
impl<'de> serde::Deserialize<'de> for GetResourceTreeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetResourceTreeResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.getResourceTree/returns/frameTree。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Present frame / resource tree structure.</pre>
                    ///
                    ///
                    #[serde(rename = "frameTree")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_tree: crate::page::types::FrameResourceTree,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetResourceTreeResponse {
                    frame_tree: fields.frame_tree,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetResourceTree {
    type Params = GetResourceTreeParams;
    type Response = GetResourceTreeResponse;
    const METHOD: &'static str = "Page.getResourceTree";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.handleJavaScriptDialog/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct HandleJavaScriptDialogParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.handleJavaScriptDialog/parameters/accept。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to accept or dismiss the dialog.</pre>
    ///
    ///
    #[serde(rename = "accept")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub accept: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.handleJavaScriptDialog/parameters/promptText。
    ///
    ///协议原文：
    ///
    ///<pre>The text to enter into the dialog prompt before accepting. Used only if this is a prompt
    ///dialog.</pre>
    ///
    ///
    #[serde(rename = "promptText")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub prompt_text: Option<String>,
}
impl<'de> serde::Deserialize<'de> for HandleJavaScriptDialogParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HandleJavaScriptDialogParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.handleJavaScriptDialog/parameters/accept。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to accept or dismiss the dialog.</pre>
                    ///
                    ///
                    #[serde(rename = "accept")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub accept: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.handleJavaScriptDialog/parameters/promptText。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The text to enter into the dialog prompt before accepting. Used only if this is a prompt
                    ///dialog.</pre>
                    ///
                    ///
                    #[serde(rename = "promptText")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub prompt_text: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(HandleJavaScriptDialogParams {
                    accept: fields.accept,
                    prompt_text: fields.prompt_text,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.handleJavaScriptDialog。
///
///协议原文：
///
///<pre>Accepts or dismisses a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload).</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct HandleJavaScriptDialog;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.handleJavaScriptDialog/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct HandleJavaScriptDialogResponse {}
impl<'de> serde::Deserialize<'de> for HandleJavaScriptDialogResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HandleJavaScriptDialogResponse;
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
                Ok(HandleJavaScriptDialogResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for HandleJavaScriptDialog {
    type Params = HandleJavaScriptDialogParams;
    type Response = HandleJavaScriptDialogResponse;
    const METHOD: &'static str = "Page.handleJavaScriptDialog";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.navigate/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct NavigateParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.navigate/parameters/url。
    ///
    ///协议原文：
    ///
    ///<pre>URL to navigate the page to.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.navigate/parameters/referrer。
    ///
    ///协议原文：
    ///
    ///<pre>Referrer URL.</pre>
    ///
    ///
    #[serde(rename = "referrer")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub referrer: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.navigate/parameters/transitionType。
    ///
    ///协议原文：
    ///
    ///<pre>Intended transition type.</pre>
    ///
    ///
    #[serde(rename = "transitionType")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub transition_type: Option<crate::page::types::TransitionType>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.navigate/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Frame id to navigate, if not specified navigates the top frame.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub frame_id: Option<crate::page::types::FrameId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.navigate/parameters/referrerPolicy。
    ///
    ///协议原文：
    ///
    ///<pre>Referrer-policy used for the navigation.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "referrerPolicy")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub referrer_policy: Option<crate::page::types::ReferrerPolicy>,
}
impl<'de> serde::Deserialize<'de> for NavigateParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NavigateParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.navigate/parameters/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>URL to navigate the page to.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.navigate/parameters/referrer。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Referrer URL.</pre>
                    ///
                    ///
                    #[serde(rename = "referrer")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub referrer: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.navigate/parameters/transitionType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Intended transition type.</pre>
                    ///
                    ///
                    #[serde(rename = "transitionType")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub transition_type: Option<crate::page::types::TransitionType>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.navigate/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame id to navigate, if not specified navigates the top frame.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub frame_id: Option<crate::page::types::FrameId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.navigate/parameters/referrerPolicy。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Referrer-policy used for the navigation.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "referrerPolicy")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub referrer_policy: Option<crate::page::types::ReferrerPolicy>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(NavigateParams {
                    url: fields.url,
                    referrer: fields.referrer,
                    transition_type: fields.transition_type,
                    frame_id: fields.frame_id,
                    referrer_policy: fields.referrer_policy,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.navigate。
///
///协议原文：
///
///<pre>Navigates current page to the given URL.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Navigate;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.navigate/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct NavigateResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.navigate/returns/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Frame id that has navigated (or failed to navigate)</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.navigate/returns/loaderId。
    ///
    ///协议原文：
    ///
    ///<pre>Loader identifier. This is omitted in case of same-document navigation,
    ///as the previously committed loaderId would not change.</pre>
    ///
    ///
    #[serde(rename = "loaderId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub loader_id: Option<crate::network::types::LoaderId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.navigate/returns/errorText。
    ///
    ///协议原文：
    ///
    ///<pre>User friendly error message, present if and only if navigation has failed.</pre>
    ///
    ///
    #[serde(rename = "errorText")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub error_text: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.navigate/returns/isDownload。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the navigation resulted in a download.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "isDownload")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_download: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for NavigateResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NavigateResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.navigate/returns/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame id that has navigated (or failed to navigate)</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.navigate/returns/loaderId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Loader identifier. This is omitted in case of same-document navigation,
                    ///as the previously committed loaderId would not change.</pre>
                    ///
                    ///
                    #[serde(rename = "loaderId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub loader_id: Option<crate::network::types::LoaderId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.navigate/returns/errorText。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>User friendly error message, present if and only if navigation has failed.</pre>
                    ///
                    ///
                    #[serde(rename = "errorText")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub error_text: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.navigate/returns/isDownload。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the navigation resulted in a download.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "isDownload")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub is_download: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(NavigateResponse {
                    frame_id: fields.frame_id,
                    loader_id: fields.loader_id,
                    error_text: fields.error_text,
                    is_download: fields.is_download,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Navigate {
    type Params = NavigateParams;
    type Response = NavigateResponse;
    const METHOD: &'static str = "Page.navigate";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.navigateToHistoryEntry/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct NavigateToHistoryEntryParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.navigateToHistoryEntry/parameters/entryId。
    ///
    ///协议原文：
    ///
    ///<pre>Unique id of the entry to navigate to.</pre>
    ///
    ///
    #[serde(rename = "entryId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub entry_id: i64,
}
impl<'de> serde::Deserialize<'de> for NavigateToHistoryEntryParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NavigateToHistoryEntryParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.navigateToHistoryEntry/parameters/entryId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Unique id of the entry to navigate to.</pre>
                    ///
                    ///
                    #[serde(rename = "entryId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub entry_id: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(NavigateToHistoryEntryParams {
                    entry_id: fields.entry_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.navigateToHistoryEntry。
///
///协议原文：
///
///<pre>Navigates current page to the given history entry.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct NavigateToHistoryEntry;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.navigateToHistoryEntry/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct NavigateToHistoryEntryResponse {}
impl<'de> serde::Deserialize<'de> for NavigateToHistoryEntryResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NavigateToHistoryEntryResponse;
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
                Ok(NavigateToHistoryEntryResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for NavigateToHistoryEntry {
    type Params = NavigateToHistoryEntryParams;
    type Response = NavigateToHistoryEntryResponse;
    const METHOD: &'static str = "Page.navigateToHistoryEntry";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.command.printToPDF/parameters/transferMode。
///
///协议原文：
///
///<pre>return as stream</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PrintToPdfParamsTransferMode {
    ///协议值 "ReturnAsBase64"。
    ReturnAsBase64,
    ///协议值 "ReturnAsStream"。
    ReturnAsStream,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl PrintToPdfParamsTransferMode {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::commands::PrintToPdfParamsTransferMode::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::ReturnAsBase64 => "ReturnAsBase64",
            Self::ReturnAsStream => "ReturnAsStream",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for PrintToPdfParamsTransferMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for PrintToPdfParamsTransferMode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ReturnAsBase64" => PrintToPdfParamsTransferMode::ReturnAsBase64,
            "ReturnAsStream" => PrintToPdfParamsTransferMode::ReturnAsStream,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for PrintToPdfParamsTransferMode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PrintToPdfParamsTransferMode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PrintToPdfParamsTransferMode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "ReturnAsBase64" => PrintToPdfParamsTransferMode::ReturnAsBase64,
                    "ReturnAsStream" => PrintToPdfParamsTransferMode::ReturnAsStream,
                    _ => PrintToPdfParamsTransferMode::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(PrintToPdfParamsTransferMode::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.printToPDF/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct PrintToPdfParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/landscape。
    ///
    ///协议原文：
    ///
    ///<pre>Paper orientation. Defaults to false.</pre>
    ///
    ///
    #[serde(rename = "landscape")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub landscape: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/displayHeaderFooter。
    ///
    ///协议原文：
    ///
    ///<pre>Display header and footer. Defaults to false.</pre>
    ///
    ///
    #[serde(rename = "displayHeaderFooter")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_header_footer: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/printBackground。
    ///
    ///协议原文：
    ///
    ///<pre>Print background graphics. Defaults to false.</pre>
    ///
    ///
    #[serde(rename = "printBackground")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub print_background: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/scale。
    ///
    ///协议原文：
    ///
    ///<pre>Scale of the webpage rendering. Defaults to 1.</pre>
    ///
    ///
    #[serde(rename = "scale")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub scale: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/paperWidth。
    ///
    ///协议原文：
    ///
    ///<pre>Paper width in inches. Defaults to 8.5 inches.</pre>
    ///
    ///
    #[serde(rename = "paperWidth")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub paper_width: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/paperHeight。
    ///
    ///协议原文：
    ///
    ///<pre>Paper height in inches. Defaults to 11 inches.</pre>
    ///
    ///
    #[serde(rename = "paperHeight")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub paper_height: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/marginTop。
    ///
    ///协议原文：
    ///
    ///<pre>Top margin in inches. Defaults to 1cm (~0.4 inches).</pre>
    ///
    ///
    #[serde(rename = "marginTop")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_top: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/marginBottom。
    ///
    ///协议原文：
    ///
    ///<pre>Bottom margin in inches. Defaults to 1cm (~0.4 inches).</pre>
    ///
    ///
    #[serde(rename = "marginBottom")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_bottom: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/marginLeft。
    ///
    ///协议原文：
    ///
    ///<pre>Left margin in inches. Defaults to 1cm (~0.4 inches).</pre>
    ///
    ///
    #[serde(rename = "marginLeft")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_left: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/marginRight。
    ///
    ///协议原文：
    ///
    ///<pre>Right margin in inches. Defaults to 1cm (~0.4 inches).</pre>
    ///
    ///
    #[serde(rename = "marginRight")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_right: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/pageRanges。
    ///
    ///协议原文：
    ///
    ///<pre>Paper ranges to print, one based, e.g., '1-5, 8, 11-13'. Pages are
    ///printed in the document order, not in the order specified, and no
    ///more than once.
    ///Defaults to empty string, which implies the entire document is printed.
    ///The page numbers are quietly capped to actual page count of the
    ///document, and ranges beyond the end of the document are ignored.
    ///If this results in no pages to print, an error is reported.
    ///It is an error to specify a range with start greater than end.</pre>
    ///
    ///
    #[serde(rename = "pageRanges")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub page_ranges: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/headerTemplate。
    ///
    ///协议原文：
    ///
    ///<pre>HTML template for the print header. Should be valid HTML markup with following
    ///classes used to inject printing values into them:
    ///- `date`: formatted print date
    ///- `title`: document title
    ///- `url`: document location
    ///- `pageNumber`: current page number
    ///- `totalPages`: total pages in the document
    ///
    ///For example, `&lt;span class=title&gt;&lt;/span&gt;` would generate span containing the title.</pre>
    ///
    ///
    #[serde(rename = "headerTemplate")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub header_template: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/footerTemplate。
    ///
    ///协议原文：
    ///
    ///<pre>HTML template for the print footer. Should use the same format as the `headerTemplate`.</pre>
    ///
    ///
    #[serde(rename = "footerTemplate")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub footer_template: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/preferCSSPageSize。
    ///
    ///协议原文：
    ///
    ///<pre>Whether or not to prefer page size as defined by css. Defaults to false,
    ///in which case the content will be scaled to fit the paper size.</pre>
    ///
    ///
    #[serde(rename = "preferCSSPageSize")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub prefer_css_page_size: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/transferMode。
    ///
    ///协议原文：
    ///
    ///<pre>return as stream</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "transferMode")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub transfer_mode: Option<PrintToPdfParamsTransferMode>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/generateTaggedPDF。
    ///
    ///协议原文：
    ///
    ///<pre>Whether or not to generate tagged (accessible) PDF. Defaults to embedder choice.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "generateTaggedPDF")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub generate_tagged_pdf: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/generateDocumentOutline。
    ///
    ///协议原文：
    ///
    ///<pre>Whether or not to embed the document outline into the PDF.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "generateDocumentOutline")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub generate_document_outline: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for PrintToPdfParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PrintToPdfParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/landscape。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Paper orientation. Defaults to false.</pre>
                    ///
                    ///
                    #[serde(rename = "landscape")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub landscape: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/displayHeaderFooter。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Display header and footer. Defaults to false.</pre>
                    ///
                    ///
                    #[serde(rename = "displayHeaderFooter")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub display_header_footer: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/printBackground。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Print background graphics. Defaults to false.</pre>
                    ///
                    ///
                    #[serde(rename = "printBackground")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub print_background: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/scale。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Scale of the webpage rendering. Defaults to 1.</pre>
                    ///
                    ///
                    #[serde(rename = "scale")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub scale: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/paperWidth。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Paper width in inches. Defaults to 8.5 inches.</pre>
                    ///
                    ///
                    #[serde(rename = "paperWidth")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub paper_width: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/paperHeight。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Paper height in inches. Defaults to 11 inches.</pre>
                    ///
                    ///
                    #[serde(rename = "paperHeight")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub paper_height: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/marginTop。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Top margin in inches. Defaults to 1cm (~0.4 inches).</pre>
                    ///
                    ///
                    #[serde(rename = "marginTop")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub margin_top: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/marginBottom。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Bottom margin in inches. Defaults to 1cm (~0.4 inches).</pre>
                    ///
                    ///
                    #[serde(rename = "marginBottom")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub margin_bottom: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/marginLeft。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Left margin in inches. Defaults to 1cm (~0.4 inches).</pre>
                    ///
                    ///
                    #[serde(rename = "marginLeft")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub margin_left: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/marginRight。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Right margin in inches. Defaults to 1cm (~0.4 inches).</pre>
                    ///
                    ///
                    #[serde(rename = "marginRight")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub margin_right: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/pageRanges。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Paper ranges to print, one based, e.g., '1-5, 8, 11-13'. Pages are
                    ///printed in the document order, not in the order specified, and no
                    ///more than once.
                    ///Defaults to empty string, which implies the entire document is printed.
                    ///The page numbers are quietly capped to actual page count of the
                    ///document, and ranges beyond the end of the document are ignored.
                    ///If this results in no pages to print, an error is reported.
                    ///It is an error to specify a range with start greater than end.</pre>
                    ///
                    ///
                    #[serde(rename = "pageRanges")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub page_ranges: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/headerTemplate。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>HTML template for the print header. Should be valid HTML markup with following
                    ///classes used to inject printing values into them:
                    ///- `date`: formatted print date
                    ///- `title`: document title
                    ///- `url`: document location
                    ///- `pageNumber`: current page number
                    ///- `totalPages`: total pages in the document
                    ///
                    ///For example, `&lt;span class=title&gt;&lt;/span&gt;` would generate span containing the title.</pre>
                    ///
                    ///
                    #[serde(rename = "headerTemplate")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub header_template: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/footerTemplate。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>HTML template for the print footer. Should use the same format as the `headerTemplate`.</pre>
                    ///
                    ///
                    #[serde(rename = "footerTemplate")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub footer_template: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/preferCSSPageSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether or not to prefer page size as defined by css. Defaults to false,
                    ///in which case the content will be scaled to fit the paper size.</pre>
                    ///
                    ///
                    #[serde(rename = "preferCSSPageSize")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub prefer_css_page_size: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/transferMode。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>return as stream</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "transferMode")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub transfer_mode: Option<PrintToPdfParamsTransferMode>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/generateTaggedPDF。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether or not to generate tagged (accessible) PDF. Defaults to embedder choice.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "generateTaggedPDF")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub generate_tagged_pdf: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/parameters/generateDocumentOutline。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether or not to embed the document outline into the PDF.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "generateDocumentOutline")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub generate_document_outline: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PrintToPdfParams {
                    landscape: fields.landscape,
                    display_header_footer: fields.display_header_footer,
                    print_background: fields.print_background,
                    scale: fields.scale,
                    paper_width: fields.paper_width,
                    paper_height: fields.paper_height,
                    margin_top: fields.margin_top,
                    margin_bottom: fields.margin_bottom,
                    margin_left: fields.margin_left,
                    margin_right: fields.margin_right,
                    page_ranges: fields.page_ranges,
                    header_template: fields.header_template,
                    footer_template: fields.footer_template,
                    prefer_css_page_size: fields.prefer_css_page_size,
                    transfer_mode: fields.transfer_mode,
                    generate_tagged_pdf: fields.generate_tagged_pdf,
                    generate_document_outline: fields.generate_document_outline,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.printToPDF。
///
///协议原文：
///
///<pre>Print page as PDF.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct PrintToPdf;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.printToPDF/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PrintToPdfResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.printToPDF/returns/data。
    ///
    ///协议原文：
    ///
    ///<pre>Base64-encoded pdf data. Empty if |returnAsStream| is specified. (Encoded as a base64 string when passed over JSON)</pre>
    ///
    ///
    #[serde(rename = "data")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub data: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/returns/stream。
    ///
    ///协议原文：
    ///
    ///<pre>A handle of the stream that holds resulting PDF data.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "stream")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub stream: Option<crate::io::types::StreamHandle>,
}
impl<'de> serde::Deserialize<'de> for PrintToPdfResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PrintToPdfResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.printToPDF/returns/data。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Base64-encoded pdf data. Empty if |returnAsStream| is specified. (Encoded as a base64 string when passed over JSON)</pre>
                    ///
                    ///
                    #[serde(rename = "data")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub data: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.printToPDF/returns/stream。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A handle of the stream that holds resulting PDF data.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "stream")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub stream: Option<crate::io::types::StreamHandle>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PrintToPdfResponse {
                    data: fields.data,
                    stream: fields.stream,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for PrintToPdf {
    type Params = PrintToPdfParams;
    type Response = PrintToPdfResponse;
    const METHOD: &'static str = "Page.printToPDF";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.produceCompilationCache/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProduceCompilationCacheParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.produceCompilationCache/parameters/scripts。
    #[serde(rename = "scripts")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub scripts: Vec<crate::page::types::CompilationCacheParams>,
}
impl<'de> serde::Deserialize<'de> for ProduceCompilationCacheParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ProduceCompilationCacheParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.produceCompilationCache/parameters/scripts。
                    #[serde(rename = "scripts")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub scripts: Vec<crate::page::types::CompilationCacheParams>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ProduceCompilationCacheParams {
                    scripts: fields.scripts,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.produceCompilationCache。
///
///协议原文：
///
///<pre>Requests backend to produce compilation cache for the specified scripts.
///`scripts` are appended to the list of scripts for which the cache
///would be produced. The list may be reset during page navigation.
///When script with a matching URL is encountered, the cache is optionally
///produced upon backend discretion, based on internal heuristics.
///See also: `Page.compilationCacheProduced`.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ProduceCompilationCache;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.produceCompilationCache/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProduceCompilationCacheResponse {}
impl<'de> serde::Deserialize<'de> for ProduceCompilationCacheResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ProduceCompilationCacheResponse;
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
                Ok(ProduceCompilationCacheResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ProduceCompilationCache {
    type Params = ProduceCompilationCacheParams;
    type Response = ProduceCompilationCacheResponse;
    const METHOD: &'static str = "Page.produceCompilationCache";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.reload/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ReloadParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.reload/parameters/ignoreCache。
    ///
    ///协议原文：
    ///
    ///<pre>If true, browser cache is ignored (as if the user pressed Shift+refresh).</pre>
    ///
    ///
    #[serde(rename = "ignoreCache")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_cache: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.reload/parameters/scriptToEvaluateOnLoad。
    ///
    ///协议原文：
    ///
    ///<pre>If set, the script will be injected into all frames of the inspected page after reload.
    ///Argument will be ignored if reloading dataURL origin.</pre>
    ///
    ///
    #[serde(rename = "scriptToEvaluateOnLoad")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub script_to_evaluate_on_load: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.reload/parameters/loaderId。
    ///
    ///协议原文：
    ///
    ///<pre>If set, an error will be thrown if the target page's main frame's
    ///loader id does not match the provided id. This prevents accidentally
    ///reloading an unintended target in case there's a racing navigation.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "loaderId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub loader_id: Option<crate::network::types::LoaderId>,
}
impl<'de> serde::Deserialize<'de> for ReloadParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReloadParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.reload/parameters/ignoreCache。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, browser cache is ignored (as if the user pressed Shift+refresh).</pre>
                    ///
                    ///
                    #[serde(rename = "ignoreCache")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub ignore_cache: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.reload/parameters/scriptToEvaluateOnLoad。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If set, the script will be injected into all frames of the inspected page after reload.
                    ///Argument will be ignored if reloading dataURL origin.</pre>
                    ///
                    ///
                    #[serde(rename = "scriptToEvaluateOnLoad")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub script_to_evaluate_on_load: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.reload/parameters/loaderId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If set, an error will be thrown if the target page's main frame's
                    ///loader id does not match the provided id. This prevents accidentally
                    ///reloading an unintended target in case there's a racing navigation.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "loaderId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub loader_id: Option<crate::network::types::LoaderId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReloadParams {
                    ignore_cache: fields.ignore_cache,
                    script_to_evaluate_on_load: fields.script_to_evaluate_on_load,
                    loader_id: fields.loader_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.reload。
///
///协议原文：
///
///<pre>Reloads given page optionally ignoring the cache.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Reload;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.reload/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReloadResponse {}
impl<'de> serde::Deserialize<'de> for ReloadResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReloadResponse;
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
                Ok(ReloadResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Reload {
    type Params = ReloadParams;
    type Response = ReloadResponse;
    const METHOD: &'static str = "Page.reload";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.removeScriptToEvaluateOnLoad/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveScriptToEvaluateOnLoadParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.removeScriptToEvaluateOnLoad/parameters/identifier。
    #[serde(rename = "identifier")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub identifier: crate::page::types::ScriptIdentifier,
}
impl<'de> serde::Deserialize<'de> for RemoveScriptToEvaluateOnLoadParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveScriptToEvaluateOnLoadParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.removeScriptToEvaluateOnLoad/parameters/identifier。
                    #[serde(rename = "identifier")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub identifier: crate::page::types::ScriptIdentifier,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RemoveScriptToEvaluateOnLoadParams {
                    identifier: fields.identifier,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.removeScriptToEvaluateOnLoad。
///
///协议原文：
///
///<pre>Deprecated, please use removeScriptToEvaluateOnNewDocument instead.</pre>
///
///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct RemoveScriptToEvaluateOnLoad;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.removeScriptToEvaluateOnLoad/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveScriptToEvaluateOnLoadResponse {}
impl<'de> serde::Deserialize<'de> for RemoveScriptToEvaluateOnLoadResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveScriptToEvaluateOnLoadResponse;
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
                Ok(RemoveScriptToEvaluateOnLoadResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RemoveScriptToEvaluateOnLoad {
    type Params = RemoveScriptToEvaluateOnLoadParams;
    type Response = RemoveScriptToEvaluateOnLoadResponse;
    const METHOD: &'static str = "Page.removeScriptToEvaluateOnLoad";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.removeScriptToEvaluateOnNewDocument/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveScriptToEvaluateOnNewDocumentParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.removeScriptToEvaluateOnNewDocument/parameters/identifier。
    #[serde(rename = "identifier")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub identifier: crate::page::types::ScriptIdentifier,
}
impl<'de> serde::Deserialize<'de> for RemoveScriptToEvaluateOnNewDocumentParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveScriptToEvaluateOnNewDocumentParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.removeScriptToEvaluateOnNewDocument/parameters/identifier。
                    #[serde(rename = "identifier")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub identifier: crate::page::types::ScriptIdentifier,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RemoveScriptToEvaluateOnNewDocumentParams {
                    identifier: fields.identifier,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.removeScriptToEvaluateOnNewDocument。
///
///协议原文：
///
///<pre>Removes given script from the list.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RemoveScriptToEvaluateOnNewDocument;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.removeScriptToEvaluateOnNewDocument/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveScriptToEvaluateOnNewDocumentResponse {}
impl<'de> serde::Deserialize<'de> for RemoveScriptToEvaluateOnNewDocumentResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveScriptToEvaluateOnNewDocumentResponse;
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
                Ok(RemoveScriptToEvaluateOnNewDocumentResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RemoveScriptToEvaluateOnNewDocument {
    type Params = RemoveScriptToEvaluateOnNewDocumentParams;
    type Response = RemoveScriptToEvaluateOnNewDocumentResponse;
    const METHOD: &'static str = "Page.removeScriptToEvaluateOnNewDocument";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.resetNavigationHistory/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ResetNavigationHistoryParams {}
impl<'de> serde::Deserialize<'de> for ResetNavigationHistoryParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResetNavigationHistoryParams;
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
                Ok(ResetNavigationHistoryParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.resetNavigationHistory。
///
///协议原文：
///
///<pre>Resets navigation history for the current page.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ResetNavigationHistory;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.resetNavigationHistory/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ResetNavigationHistoryResponse {}
impl<'de> serde::Deserialize<'de> for ResetNavigationHistoryResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResetNavigationHistoryResponse;
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
                Ok(ResetNavigationHistoryResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ResetNavigationHistory {
    type Params = ResetNavigationHistoryParams;
    type Response = ResetNavigationHistoryResponse;
    const METHOD: &'static str = "Page.resetNavigationHistory";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.screencastFrameAck/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ScreencastFrameAckParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.screencastFrameAck/parameters/sessionId。
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
impl<'de> serde::Deserialize<'de> for ScreencastFrameAckParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScreencastFrameAckParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.screencastFrameAck/parameters/sessionId。
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
                Ok(ScreencastFrameAckParams {
                    session_id: fields.session_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.screencastFrameAck。
///
///协议原文：
///
///<pre>Acknowledges that a screencast frame has been received by the frontend.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ScreencastFrameAck;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.screencastFrameAck/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ScreencastFrameAckResponse {}
impl<'de> serde::Deserialize<'de> for ScreencastFrameAckResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScreencastFrameAckResponse;
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
                Ok(ScreencastFrameAckResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ScreencastFrameAck {
    type Params = ScreencastFrameAckParams;
    type Response = ScreencastFrameAckResponse;
    const METHOD: &'static str = "Page.screencastFrameAck";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.searchInResource/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SearchInResourceParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.searchInResource/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Frame id for resource to search in.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.searchInResource/parameters/url。
    ///
    ///协议原文：
    ///
    ///<pre>URL of the resource to search in.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.searchInResource/parameters/query。
    ///
    ///协议原文：
    ///
    ///<pre>String to search for.</pre>
    ///
    ///
    #[serde(rename = "query")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub query: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.searchInResource/parameters/caseSensitive。
    ///
    ///协议原文：
    ///
    ///<pre>If true, search is case sensitive.</pre>
    ///
    ///
    #[serde(rename = "caseSensitive")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub case_sensitive: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.searchInResource/parameters/isRegex。
    ///
    ///协议原文：
    ///
    ///<pre>If true, treats string parameter as regex.</pre>
    ///
    ///
    #[serde(rename = "isRegex")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_regex: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for SearchInResourceParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SearchInResourceParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.searchInResource/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame id for resource to search in.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.searchInResource/parameters/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>URL of the resource to search in.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.searchInResource/parameters/query。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>String to search for.</pre>
                    ///
                    ///
                    #[serde(rename = "query")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub query: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.searchInResource/parameters/caseSensitive。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, search is case sensitive.</pre>
                    ///
                    ///
                    #[serde(rename = "caseSensitive")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub case_sensitive: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.searchInResource/parameters/isRegex。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, treats string parameter as regex.</pre>
                    ///
                    ///
                    #[serde(rename = "isRegex")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub is_regex: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SearchInResourceParams {
                    frame_id: fields.frame_id,
                    url: fields.url,
                    query: fields.query,
                    case_sensitive: fields.case_sensitive,
                    is_regex: fields.is_regex,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.searchInResource。
///
///协议原文：
///
///<pre>Searches for given string in resource content.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SearchInResource;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.searchInResource/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SearchInResourceResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.searchInResource/returns/result。
    ///
    ///协议原文：
    ///
    ///<pre>List of search matches.</pre>
    ///
    ///
    #[serde(rename = "result")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub result: Vec<crate::debugger::types::SearchMatch>,
}
impl<'de> serde::Deserialize<'de> for SearchInResourceResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SearchInResourceResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.searchInResource/returns/result。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>List of search matches.</pre>
                    ///
                    ///
                    #[serde(rename = "result")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub result: Vec<crate::debugger::types::SearchMatch>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SearchInResourceResponse {
                    result: fields.result,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SearchInResource {
    type Params = SearchInResourceParams;
    type Response = SearchInResourceResponse;
    const METHOD: &'static str = "Page.searchInResource";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setAdBlockingEnabled/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetAdBlockingEnabledParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setAdBlockingEnabled/parameters/enabled。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to block ads.</pre>
    ///
    ///
    #[serde(rename = "enabled")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub enabled: bool,
}
impl<'de> serde::Deserialize<'de> for SetAdBlockingEnabledParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetAdBlockingEnabledParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setAdBlockingEnabled/parameters/enabled。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to block ads.</pre>
                    ///
                    ///
                    #[serde(rename = "enabled")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub enabled: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetAdBlockingEnabledParams {
                    enabled: fields.enabled,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.setAdBlockingEnabled。
///
///协议原文：
///
///<pre>Enable Chrome's experimental ad filter on all sites.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetAdBlockingEnabled;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setAdBlockingEnabled/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetAdBlockingEnabledResponse {}
impl<'de> serde::Deserialize<'de> for SetAdBlockingEnabledResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetAdBlockingEnabledResponse;
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
                Ok(SetAdBlockingEnabledResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetAdBlockingEnabled {
    type Params = SetAdBlockingEnabledParams;
    type Response = SetAdBlockingEnabledResponse;
    const METHOD: &'static str = "Page.setAdBlockingEnabled";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setBypassCSP/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetBypassCspParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setBypassCSP/parameters/enabled。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to bypass page CSP.</pre>
    ///
    ///
    #[serde(rename = "enabled")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub enabled: bool,
}
impl<'de> serde::Deserialize<'de> for SetBypassCspParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetBypassCspParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setBypassCSP/parameters/enabled。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to bypass page CSP.</pre>
                    ///
                    ///
                    #[serde(rename = "enabled")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub enabled: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetBypassCspParams {
                    enabled: fields.enabled,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.setBypassCSP。
///
///协议原文：
///
///<pre>Enable page Content Security Policy by-passing.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetBypassCsp;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setBypassCSP/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetBypassCspResponse {}
impl<'de> serde::Deserialize<'de> for SetBypassCspResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetBypassCspResponse;
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
                Ok(SetBypassCspResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetBypassCsp {
    type Params = SetBypassCspParams;
    type Response = SetBypassCspResponse;
    const METHOD: &'static str = "Page.setBypassCSP";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setDeviceMetricsOverride/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDeviceMetricsOverrideParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setDeviceMetricsOverride/parameters/width。
    ///
    ///协议原文：
    ///
    ///<pre>Overriding width value in pixels (minimum 0, maximum 10000000). 0 disables the override.</pre>
    ///
    ///
    #[serde(rename = "width")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub width: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setDeviceMetricsOverride/parameters/height。
    ///
    ///协议原文：
    ///
    ///<pre>Overriding height value in pixels (minimum 0, maximum 10000000). 0 disables the override.</pre>
    ///
    ///
    #[serde(rename = "height")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub height: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setDeviceMetricsOverride/parameters/deviceScaleFactor。
    ///
    ///协议原文：
    ///
    ///<pre>Overriding device scale factor value. 0 disables the override.</pre>
    ///
    ///
    #[serde(rename = "deviceScaleFactor")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub device_scale_factor: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setDeviceMetricsOverride/parameters/mobile。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to emulate mobile device. This includes viewport meta tag, overlay scrollbars, text
    ///autosizing and more.</pre>
    ///
    ///
    #[serde(rename = "mobile")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub mobile: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setDeviceMetricsOverride/parameters/scale。
    ///
    ///协议原文：
    ///
    ///<pre>Scale to apply to resulting view image.</pre>
    ///
    ///
    #[serde(rename = "scale")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub scale: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setDeviceMetricsOverride/parameters/screenWidth。
    ///
    ///协议原文：
    ///
    ///<pre>Overriding screen width value in pixels (minimum 0, maximum 10000000).</pre>
    ///
    ///
    #[serde(rename = "screenWidth")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub screen_width: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setDeviceMetricsOverride/parameters/screenHeight。
    ///
    ///协议原文：
    ///
    ///<pre>Overriding screen height value in pixels (minimum 0, maximum 10000000).</pre>
    ///
    ///
    #[serde(rename = "screenHeight")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub screen_height: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setDeviceMetricsOverride/parameters/positionX。
    ///
    ///协议原文：
    ///
    ///<pre>Overriding view X position on screen in pixels (minimum 0, maximum 10000000).</pre>
    ///
    ///
    #[serde(rename = "positionX")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub position_x: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setDeviceMetricsOverride/parameters/positionY。
    ///
    ///协议原文：
    ///
    ///<pre>Overriding view Y position on screen in pixels (minimum 0, maximum 10000000).</pre>
    ///
    ///
    #[serde(rename = "positionY")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub position_y: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setDeviceMetricsOverride/parameters/dontSetVisibleSize。
    ///
    ///协议原文：
    ///
    ///<pre>Do not set visible view size, rely upon explicit setVisibleSize call.</pre>
    ///
    ///
    #[serde(rename = "dontSetVisibleSize")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub dont_set_visible_size: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setDeviceMetricsOverride/parameters/screenOrientation。
    ///
    ///协议原文：
    ///
    ///<pre>Screen orientation override.</pre>
    ///
    ///
    #[serde(rename = "screenOrientation")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub screen_orientation: Option<crate::emulation::types::ScreenOrientation>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setDeviceMetricsOverride/parameters/viewport。
    ///
    ///协议原文：
    ///
    ///<pre>The viewport dimensions and scale. If not set, the override is cleared.</pre>
    ///
    ///
    #[serde(rename = "viewport")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub viewport: Option<crate::page::types::Viewport>,
}
impl<'de> serde::Deserialize<'de> for SetDeviceMetricsOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDeviceMetricsOverrideParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setDeviceMetricsOverride/parameters/width。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Overriding width value in pixels (minimum 0, maximum 10000000). 0 disables the override.</pre>
                    ///
                    ///
                    #[serde(rename = "width")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub width: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setDeviceMetricsOverride/parameters/height。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Overriding height value in pixels (minimum 0, maximum 10000000). 0 disables the override.</pre>
                    ///
                    ///
                    #[serde(rename = "height")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub height: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setDeviceMetricsOverride/parameters/deviceScaleFactor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Overriding device scale factor value. 0 disables the override.</pre>
                    ///
                    ///
                    #[serde(rename = "deviceScaleFactor")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub device_scale_factor: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setDeviceMetricsOverride/parameters/mobile。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to emulate mobile device. This includes viewport meta tag, overlay scrollbars, text
                    ///autosizing and more.</pre>
                    ///
                    ///
                    #[serde(rename = "mobile")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub mobile: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setDeviceMetricsOverride/parameters/scale。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Scale to apply to resulting view image.</pre>
                    ///
                    ///
                    #[serde(rename = "scale")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub scale: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setDeviceMetricsOverride/parameters/screenWidth。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Overriding screen width value in pixels (minimum 0, maximum 10000000).</pre>
                    ///
                    ///
                    #[serde(rename = "screenWidth")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub screen_width: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setDeviceMetricsOverride/parameters/screenHeight。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Overriding screen height value in pixels (minimum 0, maximum 10000000).</pre>
                    ///
                    ///
                    #[serde(rename = "screenHeight")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub screen_height: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setDeviceMetricsOverride/parameters/positionX。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Overriding view X position on screen in pixels (minimum 0, maximum 10000000).</pre>
                    ///
                    ///
                    #[serde(rename = "positionX")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub position_x: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setDeviceMetricsOverride/parameters/positionY。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Overriding view Y position on screen in pixels (minimum 0, maximum 10000000).</pre>
                    ///
                    ///
                    #[serde(rename = "positionY")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub position_y: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setDeviceMetricsOverride/parameters/dontSetVisibleSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Do not set visible view size, rely upon explicit setVisibleSize call.</pre>
                    ///
                    ///
                    #[serde(rename = "dontSetVisibleSize")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub dont_set_visible_size: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setDeviceMetricsOverride/parameters/screenOrientation。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Screen orientation override.</pre>
                    ///
                    ///
                    #[serde(rename = "screenOrientation")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub screen_orientation: Option<crate::emulation::types::ScreenOrientation>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setDeviceMetricsOverride/parameters/viewport。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The viewport dimensions and scale. If not set, the override is cleared.</pre>
                    ///
                    ///
                    #[serde(rename = "viewport")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub viewport: Option<crate::page::types::Viewport>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetDeviceMetricsOverrideParams {
                    width: fields.width,
                    height: fields.height,
                    device_scale_factor: fields.device_scale_factor,
                    mobile: fields.mobile,
                    scale: fields.scale,
                    screen_width: fields.screen_width,
                    screen_height: fields.screen_height,
                    position_x: fields.position_x,
                    position_y: fields.position_y,
                    dont_set_visible_size: fields.dont_set_visible_size,
                    screen_orientation: fields.screen_orientation,
                    viewport: fields.viewport,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.setDeviceMetricsOverride。
///
///协议原文：
///
///<pre>Overrides the values of device screen dimensions (window.screen.width, window.screen.height,
///window.innerWidth, window.innerHeight, and "device-width"/"device-height"-related CSS media
///query results).</pre>
///
///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。此命令由协议重定向到 Emulation 域处理。
#[derive(Debug, Clone, Copy)]
pub struct SetDeviceMetricsOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setDeviceMetricsOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDeviceMetricsOverrideResponse {}
impl<'de> serde::Deserialize<'de> for SetDeviceMetricsOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDeviceMetricsOverrideResponse;
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
                Ok(SetDeviceMetricsOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetDeviceMetricsOverride {
    type Params = SetDeviceMetricsOverrideParams;
    type Response = SetDeviceMetricsOverrideResponse;
    const METHOD: &'static str = "Page.setDeviceMetricsOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setDeviceOrientationOverride/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDeviceOrientationOverrideParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setDeviceOrientationOverride/parameters/alpha。
    ///
    ///协议原文：
    ///
    ///<pre>Mock alpha</pre>
    ///
    ///
    #[serde(rename = "alpha")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub alpha: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setDeviceOrientationOverride/parameters/beta。
    ///
    ///协议原文：
    ///
    ///<pre>Mock beta</pre>
    ///
    ///
    #[serde(rename = "beta")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub beta: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setDeviceOrientationOverride/parameters/gamma。
    ///
    ///协议原文：
    ///
    ///<pre>Mock gamma</pre>
    ///
    ///
    #[serde(rename = "gamma")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub gamma: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for SetDeviceOrientationOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDeviceOrientationOverrideParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setDeviceOrientationOverride/parameters/alpha。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Mock alpha</pre>
                    ///
                    ///
                    #[serde(rename = "alpha")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub alpha: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setDeviceOrientationOverride/parameters/beta。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Mock beta</pre>
                    ///
                    ///
                    #[serde(rename = "beta")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub beta: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setDeviceOrientationOverride/parameters/gamma。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Mock gamma</pre>
                    ///
                    ///
                    #[serde(rename = "gamma")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub gamma: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetDeviceOrientationOverrideParams {
                    alpha: fields.alpha,
                    beta: fields.beta,
                    gamma: fields.gamma,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.setDeviceOrientationOverride。
///
///协议原文：
///
///<pre>Overrides the Device Orientation.</pre>
///
///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。此命令由协议重定向到 DeviceOrientation 域处理。
#[derive(Debug, Clone, Copy)]
pub struct SetDeviceOrientationOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setDeviceOrientationOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDeviceOrientationOverrideResponse {}
impl<'de> serde::Deserialize<'de> for SetDeviceOrientationOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDeviceOrientationOverrideResponse;
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
                Ok(SetDeviceOrientationOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetDeviceOrientationOverride {
    type Params = SetDeviceOrientationOverrideParams;
    type Response = SetDeviceOrientationOverrideResponse;
    const METHOD: &'static str = "Page.setDeviceOrientationOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setDocumentContent/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDocumentContentParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setDocumentContent/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Frame id to set HTML for.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setDocumentContent/parameters/html。
    ///
    ///协议原文：
    ///
    ///<pre>HTML content to set.</pre>
    ///
    ///
    #[serde(rename = "html")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub html: String,
}
impl<'de> serde::Deserialize<'de> for SetDocumentContentParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDocumentContentParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setDocumentContent/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame id to set HTML for.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setDocumentContent/parameters/html。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>HTML content to set.</pre>
                    ///
                    ///
                    #[serde(rename = "html")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub html: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetDocumentContentParams {
                    frame_id: fields.frame_id,
                    html: fields.html,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.setDocumentContent。
///
///协议原文：
///
///<pre>Sets given markup as the document's HTML.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetDocumentContent;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setDocumentContent/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDocumentContentResponse {}
impl<'de> serde::Deserialize<'de> for SetDocumentContentResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDocumentContentResponse;
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
                Ok(SetDocumentContentResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetDocumentContent {
    type Params = SetDocumentContentParams;
    type Response = SetDocumentContentResponse;
    const METHOD: &'static str = "Page.setDocumentContent";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.command.setDownloadBehavior/parameters/behavior。
///
///协议原文：
///
///<pre>Whether to allow all or deny all download requests, or use default Chrome behavior if
///available (otherwise deny).</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SetDownloadBehaviorParamsBehavior {
    ///协议值 "deny"。
    Deny,
    ///协议值 "allow"。
    Allow,
    ///协议值 "default"。
    Default,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl SetDownloadBehaviorParamsBehavior {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::commands::SetDownloadBehaviorParamsBehavior::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Deny => "deny",
            Self::Allow => "allow",
            Self::Default => "default",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for SetDownloadBehaviorParamsBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for SetDownloadBehaviorParamsBehavior {
    fn from(value: String) -> Self {
        match value.as_str() {
            "deny" => SetDownloadBehaviorParamsBehavior::Deny,
            "allow" => SetDownloadBehaviorParamsBehavior::Allow,
            "default" => SetDownloadBehaviorParamsBehavior::Default,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for SetDownloadBehaviorParamsBehavior {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetDownloadBehaviorParamsBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDownloadBehaviorParamsBehavior;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "deny" => SetDownloadBehaviorParamsBehavior::Deny,
                    "allow" => SetDownloadBehaviorParamsBehavior::Allow,
                    "default" => SetDownloadBehaviorParamsBehavior::Default,
                    _ => SetDownloadBehaviorParamsBehavior::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(SetDownloadBehaviorParamsBehavior::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setDownloadBehavior/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDownloadBehaviorParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setDownloadBehavior/parameters/behavior。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to allow all or deny all download requests, or use default Chrome behavior if
    ///available (otherwise deny).</pre>
    ///
    ///
    #[serde(rename = "behavior")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub behavior: SetDownloadBehaviorParamsBehavior,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setDownloadBehavior/parameters/downloadPath。
    ///
    ///协议原文：
    ///
    ///<pre>The default path to save downloaded files to. This is required if behavior is set to 'allow'</pre>
    ///
    ///
    #[serde(rename = "downloadPath")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub download_path: Option<String>,
}
impl<'de> serde::Deserialize<'de> for SetDownloadBehaviorParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDownloadBehaviorParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setDownloadBehavior/parameters/behavior。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to allow all or deny all download requests, or use default Chrome behavior if
                    ///available (otherwise deny).</pre>
                    ///
                    ///
                    #[serde(rename = "behavior")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub behavior: SetDownloadBehaviorParamsBehavior,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setDownloadBehavior/parameters/downloadPath。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The default path to save downloaded files to. This is required if behavior is set to 'allow'</pre>
                    ///
                    ///
                    #[serde(rename = "downloadPath")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub download_path: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetDownloadBehaviorParams {
                    behavior: fields.behavior,
                    download_path: fields.download_path,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.setDownloadBehavior。
///
///协议原文：
///
///<pre>Set the behavior when downloading a file.</pre>
///
///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct SetDownloadBehavior;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setDownloadBehavior/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDownloadBehaviorResponse {}
impl<'de> serde::Deserialize<'de> for SetDownloadBehaviorResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDownloadBehaviorResponse;
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
                Ok(SetDownloadBehaviorResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetDownloadBehavior {
    type Params = SetDownloadBehaviorParams;
    type Response = SetDownloadBehaviorResponse;
    const METHOD: &'static str = "Page.setDownloadBehavior";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setFontFamilies/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetFontFamiliesParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setFontFamilies/parameters/fontFamilies。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies font families to set. If a font family is not specified, it won't be changed.</pre>
    ///
    ///
    #[serde(rename = "fontFamilies")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub font_families: crate::page::types::FontFamilies,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setFontFamilies/parameters/forScripts。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies font families to set for individual scripts.</pre>
    ///
    ///
    #[serde(rename = "forScripts")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub for_scripts: Option<Vec<crate::page::types::ScriptFontFamilies>>,
}
impl<'de> serde::Deserialize<'de> for SetFontFamiliesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetFontFamiliesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setFontFamilies/parameters/fontFamilies。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies font families to set. If a font family is not specified, it won't be changed.</pre>
                    ///
                    ///
                    #[serde(rename = "fontFamilies")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub font_families: crate::page::types::FontFamilies,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setFontFamilies/parameters/forScripts。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies font families to set for individual scripts.</pre>
                    ///
                    ///
                    #[serde(rename = "forScripts")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub for_scripts: Option<Vec<crate::page::types::ScriptFontFamilies>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetFontFamiliesParams {
                    font_families: fields.font_families,
                    for_scripts: fields.for_scripts,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.setFontFamilies。
///
///协议原文：
///
///<pre>Set generic font families.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetFontFamilies;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setFontFamilies/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetFontFamiliesResponse {}
impl<'de> serde::Deserialize<'de> for SetFontFamiliesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetFontFamiliesResponse;
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
                Ok(SetFontFamiliesResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetFontFamilies {
    type Params = SetFontFamiliesParams;
    type Response = SetFontFamiliesResponse;
    const METHOD: &'static str = "Page.setFontFamilies";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setFontSizes/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetFontSizesParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setFontSizes/parameters/fontSizes。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies font sizes to set. If a font size is not specified, it won't be changed.</pre>
    ///
    ///
    #[serde(rename = "fontSizes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub font_sizes: crate::page::types::FontSizes,
}
impl<'de> serde::Deserialize<'de> for SetFontSizesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetFontSizesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setFontSizes/parameters/fontSizes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies font sizes to set. If a font size is not specified, it won't be changed.</pre>
                    ///
                    ///
                    #[serde(rename = "fontSizes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub font_sizes: crate::page::types::FontSizes,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetFontSizesParams {
                    font_sizes: fields.font_sizes,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.setFontSizes。
///
///协议原文：
///
///<pre>Set default font sizes.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetFontSizes;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setFontSizes/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetFontSizesResponse {}
impl<'de> serde::Deserialize<'de> for SetFontSizesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetFontSizesResponse;
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
                Ok(SetFontSizesResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetFontSizes {
    type Params = SetFontSizesParams;
    type Response = SetFontSizesResponse;
    const METHOD: &'static str = "Page.setFontSizes";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setGeolocationOverride/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct SetGeolocationOverrideParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setGeolocationOverride/parameters/latitude。
    ///
    ///协议原文：
    ///
    ///<pre>Mock latitude</pre>
    ///
    ///
    #[serde(rename = "latitude")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub latitude: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setGeolocationOverride/parameters/longitude。
    ///
    ///协议原文：
    ///
    ///<pre>Mock longitude</pre>
    ///
    ///
    #[serde(rename = "longitude")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub longitude: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setGeolocationOverride/parameters/accuracy。
    ///
    ///协议原文：
    ///
    ///<pre>Mock accuracy</pre>
    ///
    ///
    #[serde(rename = "accuracy")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub accuracy: Option<ydp_core::FiniteF64>,
}
impl<'de> serde::Deserialize<'de> for SetGeolocationOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetGeolocationOverrideParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setGeolocationOverride/parameters/latitude。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Mock latitude</pre>
                    ///
                    ///
                    #[serde(rename = "latitude")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub latitude: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setGeolocationOverride/parameters/longitude。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Mock longitude</pre>
                    ///
                    ///
                    #[serde(rename = "longitude")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub longitude: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setGeolocationOverride/parameters/accuracy。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Mock accuracy</pre>
                    ///
                    ///
                    #[serde(rename = "accuracy")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub accuracy: Option<ydp_core::FiniteF64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetGeolocationOverrideParams {
                    latitude: fields.latitude,
                    longitude: fields.longitude,
                    accuracy: fields.accuracy,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.setGeolocationOverride。
///
///协议原文：
///
///<pre>Overrides the Geolocation Position or Error. Omitting any of the parameters emulates position
///unavailable.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。此命令由协议重定向到 Emulation 域处理。
#[derive(Debug, Clone, Copy)]
pub struct SetGeolocationOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setGeolocationOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetGeolocationOverrideResponse {}
impl<'de> serde::Deserialize<'de> for SetGeolocationOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetGeolocationOverrideResponse;
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
                Ok(SetGeolocationOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetGeolocationOverride {
    type Params = SetGeolocationOverrideParams;
    type Response = SetGeolocationOverrideResponse;
    const METHOD: &'static str = "Page.setGeolocationOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setInterceptFileChooserDialog/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetInterceptFileChooserDialogParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setInterceptFileChooserDialog/parameters/enabled。
    #[serde(rename = "enabled")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub enabled: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setInterceptFileChooserDialog/parameters/cancel。
    ///
    ///协议原文：
    ///
    ///<pre>If true, cancels the dialog by emitting relevant events (if any)
    ///in addition to not showing it if the interception is enabled
    ///(default: false).</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "cancel")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub cancel: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for SetInterceptFileChooserDialogParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetInterceptFileChooserDialogParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setInterceptFileChooserDialog/parameters/enabled。
                    #[serde(rename = "enabled")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub enabled: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setInterceptFileChooserDialog/parameters/cancel。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, cancels the dialog by emitting relevant events (if any)
                    ///in addition to not showing it if the interception is enabled
                    ///(default: false).</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "cancel")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub cancel: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetInterceptFileChooserDialogParams {
                    enabled: fields.enabled,
                    cancel: fields.cancel,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.setInterceptFileChooserDialog。
///
///协议原文：
///
///<pre>Intercept file chooser requests and transfer control to protocol clients.
///When file chooser interception is enabled, native file chooser dialog is not shown.
///Instead, a protocol event `Page.fileChooserOpened` is emitted.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetInterceptFileChooserDialog;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setInterceptFileChooserDialog/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetInterceptFileChooserDialogResponse {}
impl<'de> serde::Deserialize<'de> for SetInterceptFileChooserDialogResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetInterceptFileChooserDialogResponse;
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
                Ok(SetInterceptFileChooserDialogResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetInterceptFileChooserDialog {
    type Params = SetInterceptFileChooserDialogParams;
    type Response = SetInterceptFileChooserDialogResponse;
    const METHOD: &'static str = "Page.setInterceptFileChooserDialog";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setLifecycleEventsEnabled/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetLifecycleEventsEnabledParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setLifecycleEventsEnabled/parameters/enabled。
    ///
    ///协议原文：
    ///
    ///<pre>If true, starts emitting lifecycle events.</pre>
    ///
    ///
    #[serde(rename = "enabled")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub enabled: bool,
}
impl<'de> serde::Deserialize<'de> for SetLifecycleEventsEnabledParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetLifecycleEventsEnabledParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setLifecycleEventsEnabled/parameters/enabled。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, starts emitting lifecycle events.</pre>
                    ///
                    ///
                    #[serde(rename = "enabled")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub enabled: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetLifecycleEventsEnabledParams {
                    enabled: fields.enabled,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.setLifecycleEventsEnabled。
///
///协议原文：
///
///<pre>Controls whether page will emit lifecycle events.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetLifecycleEventsEnabled;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setLifecycleEventsEnabled/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetLifecycleEventsEnabledResponse {}
impl<'de> serde::Deserialize<'de> for SetLifecycleEventsEnabledResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetLifecycleEventsEnabledResponse;
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
                Ok(SetLifecycleEventsEnabledResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetLifecycleEventsEnabled {
    type Params = SetLifecycleEventsEnabledParams;
    type Response = SetLifecycleEventsEnabledResponse;
    const METHOD: &'static str = "Page.setLifecycleEventsEnabled";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setPrerenderingAllowed/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetPrerenderingAllowedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setPrerenderingAllowed/parameters/isAllowed。
    #[serde(rename = "isAllowed")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub is_allowed: bool,
}
impl<'de> serde::Deserialize<'de> for SetPrerenderingAllowedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetPrerenderingAllowedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setPrerenderingAllowed/parameters/isAllowed。
                    #[serde(rename = "isAllowed")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub is_allowed: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetPrerenderingAllowedParams {
                    is_allowed: fields.is_allowed,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.setPrerenderingAllowed。
///
///协议原文：
///
///<pre>Enable/disable prerendering manually.
///
///This command is a short-term solution for https://crbug.com/1440085.
///See https://docs.google.com/document/d/12HVmFxYj5Jc-eJr5OmWsa2bqTJsbgGLKI6ZIyx0_wpA
///for more details.
///
///TODO(https://crbug.com/1440085): Remove this once Puppeteer supports tab targets.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetPrerenderingAllowed;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setPrerenderingAllowed/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetPrerenderingAllowedResponse {}
impl<'de> serde::Deserialize<'de> for SetPrerenderingAllowedResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetPrerenderingAllowedResponse;
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
                Ok(SetPrerenderingAllowedResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetPrerenderingAllowed {
    type Params = SetPrerenderingAllowedParams;
    type Response = SetPrerenderingAllowedResponse;
    const METHOD: &'static str = "Page.setPrerenderingAllowed";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.command.setRPHRegistrationMode/parameters/mode。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SetRphRegistrationModeParamsMode {
    ///协议值 "none"。
    None,
    ///协议值 "autoAccept"。
    AutoAccept,
    ///协议值 "autoReject"。
    AutoReject,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl SetRphRegistrationModeParamsMode {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::commands::SetRphRegistrationModeParamsMode::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::None => "none",
            Self::AutoAccept => "autoAccept",
            Self::AutoReject => "autoReject",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for SetRphRegistrationModeParamsMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for SetRphRegistrationModeParamsMode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "none" => SetRphRegistrationModeParamsMode::None,
            "autoAccept" => SetRphRegistrationModeParamsMode::AutoAccept,
            "autoReject" => SetRphRegistrationModeParamsMode::AutoReject,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for SetRphRegistrationModeParamsMode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetRphRegistrationModeParamsMode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetRphRegistrationModeParamsMode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "none" => SetRphRegistrationModeParamsMode::None,
                    "autoAccept" => SetRphRegistrationModeParamsMode::AutoAccept,
                    "autoReject" => SetRphRegistrationModeParamsMode::AutoReject,
                    _ => SetRphRegistrationModeParamsMode::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(SetRphRegistrationModeParamsMode::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setRPHRegistrationMode/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetRphRegistrationModeParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setRPHRegistrationMode/parameters/mode。
    #[serde(rename = "mode")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub mode: SetRphRegistrationModeParamsMode,
}
impl<'de> serde::Deserialize<'de> for SetRphRegistrationModeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetRphRegistrationModeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setRPHRegistrationMode/parameters/mode。
                    #[serde(rename = "mode")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub mode: SetRphRegistrationModeParamsMode,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetRphRegistrationModeParams { mode: fields.mode })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.setRPHRegistrationMode。
///
///协议原文：
///
///<pre>Extensions for Custom Handlers API:
///https://html.spec.whatwg.org/multipage/system-state.html#rph-automation</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetRphRegistrationMode;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setRPHRegistrationMode/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetRphRegistrationModeResponse {}
impl<'de> serde::Deserialize<'de> for SetRphRegistrationModeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetRphRegistrationModeResponse;
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
                Ok(SetRphRegistrationModeResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetRphRegistrationMode {
    type Params = SetRphRegistrationModeParams;
    type Response = SetRphRegistrationModeResponse;
    const METHOD: &'static str = "Page.setRPHRegistrationMode";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.command.setSPCTransactionMode/parameters/mode。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SetSpcTransactionModeParamsMode {
    ///协议值 "none"。
    None,
    ///协议值 "autoAccept"。
    AutoAccept,
    ///协议值 "autoChooseToAuthAnotherWay"。
    AutoChooseToAuthAnotherWay,
    ///协议值 "autoReject"。
    AutoReject,
    ///协议值 "autoOptOut"。
    AutoOptOut,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl SetSpcTransactionModeParamsMode {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::commands::SetSpcTransactionModeParamsMode::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::None => "none",
            Self::AutoAccept => "autoAccept",
            Self::AutoChooseToAuthAnotherWay => "autoChooseToAuthAnotherWay",
            Self::AutoReject => "autoReject",
            Self::AutoOptOut => "autoOptOut",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for SetSpcTransactionModeParamsMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for SetSpcTransactionModeParamsMode {
    fn from(value: String) -> Self {
        match value.as_str() {
            "none" => SetSpcTransactionModeParamsMode::None,
            "autoAccept" => SetSpcTransactionModeParamsMode::AutoAccept,
            "autoChooseToAuthAnotherWay" => {
                SetSpcTransactionModeParamsMode::AutoChooseToAuthAnotherWay
            }
            "autoReject" => SetSpcTransactionModeParamsMode::AutoReject,
            "autoOptOut" => SetSpcTransactionModeParamsMode::AutoOptOut,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for SetSpcTransactionModeParamsMode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetSpcTransactionModeParamsMode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetSpcTransactionModeParamsMode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "none" => SetSpcTransactionModeParamsMode::None,
                    "autoAccept" => SetSpcTransactionModeParamsMode::AutoAccept,
                    "autoChooseToAuthAnotherWay" => {
                        SetSpcTransactionModeParamsMode::AutoChooseToAuthAnotherWay
                    }
                    "autoReject" => SetSpcTransactionModeParamsMode::AutoReject,
                    "autoOptOut" => SetSpcTransactionModeParamsMode::AutoOptOut,
                    _ => SetSpcTransactionModeParamsMode::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(SetSpcTransactionModeParamsMode::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setSPCTransactionMode/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetSpcTransactionModeParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setSPCTransactionMode/parameters/mode。
    #[serde(rename = "mode")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub mode: SetSpcTransactionModeParamsMode,
}
impl<'de> serde::Deserialize<'de> for SetSpcTransactionModeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetSpcTransactionModeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setSPCTransactionMode/parameters/mode。
                    #[serde(rename = "mode")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub mode: SetSpcTransactionModeParamsMode,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetSpcTransactionModeParams { mode: fields.mode })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.setSPCTransactionMode。
///
///协议原文：
///
///<pre>Sets the Secure Payment Confirmation transaction mode.
///https://w3c.github.io/secure-payment-confirmation/#sctn-automation-set-spc-transaction-mode</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetSpcTransactionMode;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setSPCTransactionMode/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetSpcTransactionModeResponse {}
impl<'de> serde::Deserialize<'de> for SetSpcTransactionModeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetSpcTransactionModeResponse;
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
                Ok(SetSpcTransactionModeResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetSpcTransactionMode {
    type Params = SetSpcTransactionModeParams;
    type Response = SetSpcTransactionModeResponse;
    const METHOD: &'static str = "Page.setSPCTransactionMode";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.command.setTouchEmulationEnabled/parameters/configuration。
///
///协议原文：
///
///<pre>Touch/gesture events configuration. Default: current platform.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SetTouchEmulationEnabledParamsConfiguration {
    ///协议值 "mobile"。
    Mobile,
    ///协议值 "desktop"。
    Desktop,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl SetTouchEmulationEnabledParamsConfiguration {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::commands::SetTouchEmulationEnabledParamsConfiguration::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Mobile => "mobile",
            Self::Desktop => "desktop",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for SetTouchEmulationEnabledParamsConfiguration {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for SetTouchEmulationEnabledParamsConfiguration {
    fn from(value: String) -> Self {
        match value.as_str() {
            "mobile" => SetTouchEmulationEnabledParamsConfiguration::Mobile,
            "desktop" => SetTouchEmulationEnabledParamsConfiguration::Desktop,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for SetTouchEmulationEnabledParamsConfiguration {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetTouchEmulationEnabledParamsConfiguration {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetTouchEmulationEnabledParamsConfiguration;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "mobile" => SetTouchEmulationEnabledParamsConfiguration::Mobile,
                    "desktop" => SetTouchEmulationEnabledParamsConfiguration::Desktop,
                    _ => SetTouchEmulationEnabledParamsConfiguration::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(SetTouchEmulationEnabledParamsConfiguration::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setTouchEmulationEnabled/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetTouchEmulationEnabledParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setTouchEmulationEnabled/parameters/enabled。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the touch event emulation should be enabled.</pre>
    ///
    ///
    #[serde(rename = "enabled")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub enabled: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setTouchEmulationEnabled/parameters/configuration。
    ///
    ///协议原文：
    ///
    ///<pre>Touch/gesture events configuration. Default: current platform.</pre>
    ///
    ///
    #[serde(rename = "configuration")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub configuration: Option<SetTouchEmulationEnabledParamsConfiguration>,
}
impl<'de> serde::Deserialize<'de> for SetTouchEmulationEnabledParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetTouchEmulationEnabledParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setTouchEmulationEnabled/parameters/enabled。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the touch event emulation should be enabled.</pre>
                    ///
                    ///
                    #[serde(rename = "enabled")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub enabled: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.setTouchEmulationEnabled/parameters/configuration。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Touch/gesture events configuration. Default: current platform.</pre>
                    ///
                    ///
                    #[serde(rename = "configuration")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub configuration: Option<SetTouchEmulationEnabledParamsConfiguration>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetTouchEmulationEnabledParams {
                    enabled: fields.enabled,
                    configuration: fields.configuration,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.setTouchEmulationEnabled。
///
///协议原文：
///
///<pre>Toggles mouse event-based touch event emulation.</pre>
///
///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。此命令由协议重定向到 Emulation 域处理。
#[derive(Debug, Clone, Copy)]
pub struct SetTouchEmulationEnabled;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setTouchEmulationEnabled/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetTouchEmulationEnabledResponse {}
impl<'de> serde::Deserialize<'de> for SetTouchEmulationEnabledResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetTouchEmulationEnabledResponse;
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
                Ok(SetTouchEmulationEnabledResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetTouchEmulationEnabled {
    type Params = SetTouchEmulationEnabledParams;
    type Response = SetTouchEmulationEnabledResponse;
    const METHOD: &'static str = "Page.setTouchEmulationEnabled";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.command.setWebLifecycleState/parameters/state。
///
///协议原文：
///
///<pre>Target lifecycle state</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SetWebLifecycleStateParamsState {
    ///协议值 "frozen"。
    Frozen,
    ///协议值 "active"。
    Active,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl SetWebLifecycleStateParamsState {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::commands::SetWebLifecycleStateParamsState::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Frozen => "frozen",
            Self::Active => "active",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for SetWebLifecycleStateParamsState {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for SetWebLifecycleStateParamsState {
    fn from(value: String) -> Self {
        match value.as_str() {
            "frozen" => SetWebLifecycleStateParamsState::Frozen,
            "active" => SetWebLifecycleStateParamsState::Active,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for SetWebLifecycleStateParamsState {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetWebLifecycleStateParamsState {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetWebLifecycleStateParamsState;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "frozen" => SetWebLifecycleStateParamsState::Frozen,
                    "active" => SetWebLifecycleStateParamsState::Active,
                    _ => SetWebLifecycleStateParamsState::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(SetWebLifecycleStateParamsState::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setWebLifecycleState/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetWebLifecycleStateParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setWebLifecycleState/parameters/state。
    ///
    ///协议原文：
    ///
    ///<pre>Target lifecycle state</pre>
    ///
    ///
    #[serde(rename = "state")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub state: SetWebLifecycleStateParamsState,
}
impl<'de> serde::Deserialize<'de> for SetWebLifecycleStateParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetWebLifecycleStateParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.setWebLifecycleState/parameters/state。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Target lifecycle state</pre>
                    ///
                    ///
                    #[serde(rename = "state")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub state: SetWebLifecycleStateParamsState,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetWebLifecycleStateParams {
                    state: fields.state,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.setWebLifecycleState。
///
///协议原文：
///
///<pre>Tries to update the web lifecycle state of the page.
///It will transition the page to the given state according to:
///https://github.com/WICG/web-lifecycle/</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetWebLifecycleState;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.setWebLifecycleState/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetWebLifecycleStateResponse {}
impl<'de> serde::Deserialize<'de> for SetWebLifecycleStateResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetWebLifecycleStateResponse;
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
                Ok(SetWebLifecycleStateResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetWebLifecycleState {
    type Params = SetWebLifecycleStateParams;
    type Response = SetWebLifecycleStateResponse;
    const METHOD: &'static str = "Page.setWebLifecycleState";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.startScreenRecording/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct StartScreenRecordingParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.startScreenRecording/parameters/audio。
    #[serde(rename = "audio")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub audio: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.startScreenRecording/parameters/maxWidth。
    ///
    ///协议原文：
    ///
    ///<pre>Maximum frame width in pixels.</pre>
    ///
    ///
    #[serde(rename = "maxWidth")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_width: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.startScreenRecording/parameters/maxHeight。
    ///
    ///协议原文：
    ///
    ///<pre>Maximum frame height in pixels.</pre>
    ///
    ///
    #[serde(rename = "maxHeight")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_height: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.startScreenRecording/parameters/frameRate。
    ///
    ///协议原文：
    ///
    ///<pre>Maximum frame rate in frames per second.</pre>
    ///
    ///
    #[serde(rename = "frameRate")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub frame_rate: Option<i64>,
}
impl<'de> serde::Deserialize<'de> for StartScreenRecordingParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StartScreenRecordingParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.startScreenRecording/parameters/audio。
                    #[serde(rename = "audio")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub audio: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.startScreenRecording/parameters/maxWidth。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Maximum frame width in pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "maxWidth")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub max_width: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.startScreenRecording/parameters/maxHeight。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Maximum frame height in pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "maxHeight")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub max_height: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.startScreenRecording/parameters/frameRate。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Maximum frame rate in frames per second.</pre>
                    ///
                    ///
                    #[serde(rename = "frameRate")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub frame_rate: Option<i64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StartScreenRecordingParams {
                    audio: fields.audio,
                    max_width: fields.max_width,
                    max_height: fields.max_height,
                    frame_rate: fields.frame_rate,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.startScreenRecording。
///
///协议原文：
///
///<pre>Starts screencast video recording.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct StartScreenRecording;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.startScreenRecording/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StartScreenRecordingResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.startScreenRecording/returns/stream。
    ///
    ///协议原文：
    ///
    ///<pre>A handle of the stream that holds resulting screencast data.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "stream")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub stream: crate::io::types::StreamHandle,
}
impl<'de> serde::Deserialize<'de> for StartScreenRecordingResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StartScreenRecordingResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.startScreenRecording/returns/stream。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A handle of the stream that holds resulting screencast data.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "stream")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub stream: crate::io::types::StreamHandle,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StartScreenRecordingResponse {
                    stream: fields.stream,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for StartScreenRecording {
    type Params = StartScreenRecordingParams;
    type Response = StartScreenRecordingResponse;
    const METHOD: &'static str = "Page.startScreenRecording";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.command.startScreencast/parameters/format。
///
///协议原文：
///
///<pre>Image compression format.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StartScreencastParamsFormat {
    ///协议值 "jpeg"。
    Jpeg,
    ///协议值 "png"。
    Png,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl StartScreencastParamsFormat {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::commands::StartScreencastParamsFormat::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Jpeg => "jpeg",
            Self::Png => "png",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for StartScreencastParamsFormat {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for StartScreencastParamsFormat {
    fn from(value: String) -> Self {
        match value.as_str() {
            "jpeg" => StartScreencastParamsFormat::Jpeg,
            "png" => StartScreencastParamsFormat::Png,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for StartScreencastParamsFormat {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StartScreencastParamsFormat {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StartScreencastParamsFormat;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "jpeg" => StartScreencastParamsFormat::Jpeg,
                    "png" => StartScreencastParamsFormat::Png,
                    _ => StartScreencastParamsFormat::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(StartScreencastParamsFormat::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.startScreencast/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct StartScreencastParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.startScreencast/parameters/format。
    ///
    ///协议原文：
    ///
    ///<pre>Image compression format.</pre>
    ///
    ///
    #[serde(rename = "format")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub format: Option<StartScreencastParamsFormat>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.startScreencast/parameters/quality。
    ///
    ///协议原文：
    ///
    ///<pre>Compression quality from range [0..100].</pre>
    ///
    ///
    #[serde(rename = "quality")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub quality: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.startScreencast/parameters/maxWidth。
    ///
    ///协议原文：
    ///
    ///<pre>Maximum screenshot width.</pre>
    ///
    ///
    #[serde(rename = "maxWidth")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_width: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.startScreencast/parameters/maxHeight。
    ///
    ///协议原文：
    ///
    ///<pre>Maximum screenshot height.</pre>
    ///
    ///
    #[serde(rename = "maxHeight")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_height: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.startScreencast/parameters/everyNthFrame。
    ///
    ///协议原文：
    ///
    ///<pre>Send every n-th frame. Must be a positive integer.</pre>
    ///
    ///
    #[serde(rename = "everyNthFrame")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub every_nth_frame: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.startScreencast/parameters/maxFramesInFlight。
    ///
    ///协议原文：
    ///
    ///<pre>Maximum number of frames sent until screencastFrameAck is required.
    ///Defaults to 3. Must be a positive integer.</pre>
    ///
    ///
    #[serde(rename = "maxFramesInFlight")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_frames_in_flight: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.startScreencast/parameters/sendLastFrame。
    ///
    ///协议原文：
    ///
    ///<pre>By default, after screencastFrameAck arrives, the next produced frame is sent.
    ///Passing this flag enables storing the last produced frame in memory, which is
    ///immediately sent upon screencastFrameAck. This way, overall performance is
    ///traded for a better latency.</pre>
    ///
    ///
    #[serde(rename = "sendLastFrame")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub send_last_frame: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for StartScreencastParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StartScreencastParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.startScreencast/parameters/format。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Image compression format.</pre>
                    ///
                    ///
                    #[serde(rename = "format")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub format: Option<StartScreencastParamsFormat>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.startScreencast/parameters/quality。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Compression quality from range [0..100].</pre>
                    ///
                    ///
                    #[serde(rename = "quality")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub quality: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.startScreencast/parameters/maxWidth。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Maximum screenshot width.</pre>
                    ///
                    ///
                    #[serde(rename = "maxWidth")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub max_width: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.startScreencast/parameters/maxHeight。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Maximum screenshot height.</pre>
                    ///
                    ///
                    #[serde(rename = "maxHeight")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub max_height: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.startScreencast/parameters/everyNthFrame。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Send every n-th frame. Must be a positive integer.</pre>
                    ///
                    ///
                    #[serde(rename = "everyNthFrame")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub every_nth_frame: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.startScreencast/parameters/maxFramesInFlight。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Maximum number of frames sent until screencastFrameAck is required.
                    ///Defaults to 3. Must be a positive integer.</pre>
                    ///
                    ///
                    #[serde(rename = "maxFramesInFlight")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub max_frames_in_flight: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.command.startScreencast/parameters/sendLastFrame。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>By default, after screencastFrameAck arrives, the next produced frame is sent.
                    ///Passing this flag enables storing the last produced frame in memory, which is
                    ///immediately sent upon screencastFrameAck. This way, overall performance is
                    ///traded for a better latency.</pre>
                    ///
                    ///
                    #[serde(rename = "sendLastFrame")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub send_last_frame: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StartScreencastParams {
                    format: fields.format,
                    quality: fields.quality,
                    max_width: fields.max_width,
                    max_height: fields.max_height,
                    every_nth_frame: fields.every_nth_frame,
                    max_frames_in_flight: fields.max_frames_in_flight,
                    send_last_frame: fields.send_last_frame,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.startScreencast。
///
///协议原文：
///
///<pre>Starts sending each frame using the `screencastFrame` event.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct StartScreencast;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.startScreencast/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StartScreencastResponse {}
impl<'de> serde::Deserialize<'de> for StartScreencastResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StartScreencastResponse;
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
                Ok(StartScreencastResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for StartScreencast {
    type Params = StartScreencastParams;
    type Response = StartScreencastResponse;
    const METHOD: &'static str = "Page.startScreencast";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.stopLoading/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct StopLoadingParams {}
impl<'de> serde::Deserialize<'de> for StopLoadingParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StopLoadingParams;
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
                Ok(StopLoadingParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.stopLoading。
///
///协议原文：
///
///<pre>Force the page stop all navigations and pending resource fetches.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct StopLoading;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.stopLoading/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StopLoadingResponse {}
impl<'de> serde::Deserialize<'de> for StopLoadingResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StopLoadingResponse;
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
                Ok(StopLoadingResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for StopLoading {
    type Params = StopLoadingParams;
    type Response = StopLoadingResponse;
    const METHOD: &'static str = "Page.stopLoading";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.stopScreenRecording/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct StopScreenRecordingParams {}
impl<'de> serde::Deserialize<'de> for StopScreenRecordingParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StopScreenRecordingParams;
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
                Ok(StopScreenRecordingParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.stopScreenRecording。
///
///协议原文：
///
///<pre>Stops screencast video recording.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct StopScreenRecording;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.stopScreenRecording/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StopScreenRecordingResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.stopScreenRecording/returns/stream。
    ///
    ///协议原文：
    ///
    ///<pre>A handle of the stream that holds resulting screencast data.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "stream")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub stream: crate::io::types::StreamHandle,
}
impl<'de> serde::Deserialize<'de> for StopScreenRecordingResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StopScreenRecordingResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.command.stopScreenRecording/returns/stream。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A handle of the stream that holds resulting screencast data.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "stream")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub stream: crate::io::types::StreamHandle,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StopScreenRecordingResponse {
                    stream: fields.stream,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for StopScreenRecording {
    type Params = StopScreenRecordingParams;
    type Response = StopScreenRecordingResponse;
    const METHOD: &'static str = "Page.stopScreenRecording";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.stopScreencast/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct StopScreencastParams {}
impl<'de> serde::Deserialize<'de> for StopScreencastParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StopScreencastParams;
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
                Ok(StopScreencastParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.stopScreencast。
///
///协议原文：
///
///<pre>Stops sending each frame in the `screencastFrame`.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct StopScreencast;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.stopScreencast/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StopScreencastResponse {}
impl<'de> serde::Deserialize<'de> for StopScreencastResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StopScreencastResponse;
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
                Ok(StopScreencastResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for StopScreencast {
    type Params = StopScreencastParams;
    type Response = StopScreencastResponse;
    const METHOD: &'static str = "Page.stopScreencast";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.waitForDebugger/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct WaitForDebuggerParams {}
impl<'de> serde::Deserialize<'de> for WaitForDebuggerParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WaitForDebuggerParams;
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
                Ok(WaitForDebuggerParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Page.command.waitForDebugger。
///
///协议原文：
///
///<pre>Pauses page execution. Can be resumed using generic Runtime.runIfWaitingForDebugger.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct WaitForDebugger;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.command.waitForDebugger/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct WaitForDebuggerResponse {}
impl<'de> serde::Deserialize<'de> for WaitForDebuggerResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WaitForDebuggerResponse;
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
                Ok(WaitForDebuggerResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for WaitForDebugger {
    type Params = WaitForDebuggerParams;
    type Response = WaitForDebuggerResponse;
    const METHOD: &'static str = "Page.waitForDebugger";
}
