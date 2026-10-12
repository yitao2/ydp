///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.type.AdFrameExplanation。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AdFrameExplanation {
    ///协议值 "ParentIsAd"。
    ParentIsAd,
    ///协议值 "CreatedByAdScript"。
    CreatedByAdScript,
    ///协议值 "MatchedBlockingRule"。
    MatchedBlockingRule,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl AdFrameExplanation {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::types::AdFrameExplanation::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::ParentIsAd => "ParentIsAd",
            Self::CreatedByAdScript => "CreatedByAdScript",
            Self::MatchedBlockingRule => "MatchedBlockingRule",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for AdFrameExplanation {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for AdFrameExplanation {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ParentIsAd" => AdFrameExplanation::ParentIsAd,
            "CreatedByAdScript" => AdFrameExplanation::CreatedByAdScript,
            "MatchedBlockingRule" => AdFrameExplanation::MatchedBlockingRule,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for AdFrameExplanation {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AdFrameExplanation {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AdFrameExplanation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "ParentIsAd" => AdFrameExplanation::ParentIsAd,
                    "CreatedByAdScript" => AdFrameExplanation::CreatedByAdScript,
                    "MatchedBlockingRule" => AdFrameExplanation::MatchedBlockingRule,
                    _ => AdFrameExplanation::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(AdFrameExplanation::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.AdFrameStatus。
///
///协议原文：
///
///<pre>Indicates whether a frame has been identified as an ad and why.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AdFrameStatus {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.AdFrameStatus/properties/adFrameType。
    #[serde(rename = "adFrameType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub ad_frame_type: crate::page::types::AdFrameType,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.AdFrameStatus/properties/explanations。
    #[serde(rename = "explanations")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub explanations: Option<Vec<crate::page::types::AdFrameExplanation>>,
}
impl<'de> serde::Deserialize<'de> for AdFrameStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AdFrameStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.AdFrameStatus/properties/adFrameType。
                    #[serde(rename = "adFrameType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub ad_frame_type: crate::page::types::AdFrameType,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.AdFrameStatus/properties/explanations。
                    #[serde(rename = "explanations")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub explanations: Option<Vec<crate::page::types::AdFrameExplanation>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AdFrameStatus {
                    ad_frame_type: fields.ad_frame_type,
                    explanations: fields.explanations,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.type.AdFrameType。
///
///协议原文：
///
///<pre>Indicates whether a frame has been identified as an ad.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AdFrameType {
    ///协议值 "none"。
    None,
    ///协议值 "child"。
    Child,
    ///协议值 "root"。
    Root,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl AdFrameType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::types::AdFrameType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::None => "none",
            Self::Child => "child",
            Self::Root => "root",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for AdFrameType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for AdFrameType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "none" => AdFrameType::None,
            "child" => AdFrameType::Child,
            "root" => AdFrameType::Root,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for AdFrameType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AdFrameType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AdFrameType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "none" => AdFrameType::None,
                    "child" => AdFrameType::Child,
                    "root" => AdFrameType::Root,
                    _ => AdFrameType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(AdFrameType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.AppManifestError。
///
///协议原文：
///
///<pre>Error while paring app manifest.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct AppManifestError {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.AppManifestError/properties/message。
    ///
    ///协议原文：
    ///
    ///<pre>Error message.</pre>
    ///
    ///
    #[serde(rename = "message")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub message: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.AppManifestError/properties/critical。
    ///
    ///协议原文：
    ///
    ///<pre>If critical, this is a non-recoverable parse error.</pre>
    ///
    ///
    #[serde(rename = "critical")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub critical: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.AppManifestError/properties/line。
    ///
    ///协议原文：
    ///
    ///<pre>Error line.</pre>
    ///
    ///
    #[serde(rename = "line")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub line: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.AppManifestError/properties/column。
    ///
    ///协议原文：
    ///
    ///<pre>Error column.</pre>
    ///
    ///
    #[serde(rename = "column")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub column: i64,
}
impl<'de> serde::Deserialize<'de> for AppManifestError {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AppManifestError;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.AppManifestError/properties/message。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Error message.</pre>
                    ///
                    ///
                    #[serde(rename = "message")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub message: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.AppManifestError/properties/critical。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If critical, this is a non-recoverable parse error.</pre>
                    ///
                    ///
                    #[serde(rename = "critical")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub critical: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.AppManifestError/properties/line。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Error line.</pre>
                    ///
                    ///
                    #[serde(rename = "line")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub line: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.AppManifestError/properties/column。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Error column.</pre>
                    ///
                    ///
                    #[serde(rename = "column")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub column: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AppManifestError {
                    message: fields.message,
                    critical: fields.critical,
                    line: fields.line,
                    column: fields.column,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.AppManifestParsedProperties。
///
///协议原文：
///
///<pre>Parsed app manifest properties.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AppManifestParsedProperties {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.AppManifestParsedProperties/properties/scope。
    ///
    ///协议原文：
    ///
    ///<pre>Computed scope value</pre>
    ///
    ///
    #[serde(rename = "scope")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub scope: String,
}
impl<'de> serde::Deserialize<'de> for AppManifestParsedProperties {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AppManifestParsedProperties;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.AppManifestParsedProperties/properties/scope。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Computed scope value</pre>
                    ///
                    ///
                    #[serde(rename = "scope")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub scope: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AppManifestParsedProperties {
                    scope: fields.scope,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.BackForwardCacheBlockingDetails。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct BackForwardCacheBlockingDetails {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.BackForwardCacheBlockingDetails/properties/url。
    ///
    ///协议原文：
    ///
    ///<pre>Url of the file where blockage happened. Optional because of tests.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.BackForwardCacheBlockingDetails/properties/function。
    ///
    ///协议原文：
    ///
    ///<pre>Function name where blockage happened. Optional because of anonymous functions and tests.</pre>
    ///
    ///
    #[serde(rename = "function")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub function: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.BackForwardCacheBlockingDetails/properties/lineNumber。
    ///
    ///协议原文：
    ///
    ///<pre>Line number in the script (0-based).</pre>
    ///
    ///
    #[serde(rename = "lineNumber")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub line_number: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.BackForwardCacheBlockingDetails/properties/columnNumber。
    ///
    ///协议原文：
    ///
    ///<pre>Column number in the script (0-based).</pre>
    ///
    ///
    #[serde(rename = "columnNumber")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub column_number: i64,
}
impl<'de> serde::Deserialize<'de> for BackForwardCacheBlockingDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BackForwardCacheBlockingDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.BackForwardCacheBlockingDetails/properties/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Url of the file where blockage happened. Optional because of tests.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub url: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.BackForwardCacheBlockingDetails/properties/function。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Function name where blockage happened. Optional because of anonymous functions and tests.</pre>
                    ///
                    ///
                    #[serde(rename = "function")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub function: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.BackForwardCacheBlockingDetails/properties/lineNumber。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Line number in the script (0-based).</pre>
                    ///
                    ///
                    #[serde(rename = "lineNumber")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub line_number: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.BackForwardCacheBlockingDetails/properties/columnNumber。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Column number in the script (0-based).</pre>
                    ///
                    ///
                    #[serde(rename = "columnNumber")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub column_number: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(BackForwardCacheBlockingDetails {
                    url: fields.url,
                    function: fields.function,
                    line_number: fields.line_number,
                    column_number: fields.column_number,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.BackForwardCacheNotRestoredExplanation。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct BackForwardCacheNotRestoredExplanation {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.BackForwardCacheNotRestoredExplanation/properties/type。
    ///
    ///协议原文：
    ///
    ///<pre>Type of the reason</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: crate::page::types::BackForwardCacheNotRestoredReasonType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.BackForwardCacheNotRestoredExplanation/properties/reason。
    ///
    ///协议原文：
    ///
    ///<pre>Not restored reason</pre>
    ///
    ///
    #[serde(rename = "reason")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub reason: crate::page::types::BackForwardCacheNotRestoredReason,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.BackForwardCacheNotRestoredExplanation/properties/context。
    ///
    ///协议原文：
    ///
    ///<pre>Context associated with the reason. The meaning of this context is
    ///dependent on the reason:
    ///- EmbedderExtensionSentMessageToCachedFrame: the extension ID.</pre>
    ///
    ///
    #[serde(rename = "context")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub context: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.BackForwardCacheNotRestoredExplanation/properties/details。
    #[serde(rename = "details")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub details: Option<Vec<crate::page::types::BackForwardCacheBlockingDetails>>,
}
impl<'de> serde::Deserialize<'de> for BackForwardCacheNotRestoredExplanation {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BackForwardCacheNotRestoredExplanation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.BackForwardCacheNotRestoredExplanation/properties/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Type of the reason</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: crate::page::types::BackForwardCacheNotRestoredReasonType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.BackForwardCacheNotRestoredExplanation/properties/reason。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Not restored reason</pre>
                    ///
                    ///
                    #[serde(rename = "reason")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub reason: crate::page::types::BackForwardCacheNotRestoredReason,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.BackForwardCacheNotRestoredExplanation/properties/context。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Context associated with the reason. The meaning of this context is
                    ///dependent on the reason:
                    ///- EmbedderExtensionSentMessageToCachedFrame: the extension ID.</pre>
                    ///
                    ///
                    #[serde(rename = "context")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub context: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.BackForwardCacheNotRestoredExplanation/properties/details。
                    #[serde(rename = "details")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub details: Option<Vec<crate::page::types::BackForwardCacheBlockingDetails>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(BackForwardCacheNotRestoredExplanation {
                    r#type: fields.r#type,
                    reason: fields.reason,
                    context: fields.context,
                    details: fields.details,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.BackForwardCacheNotRestoredExplanationTree。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct BackForwardCacheNotRestoredExplanationTree {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.BackForwardCacheNotRestoredExplanationTree/properties/url。
    ///
    ///协议原文：
    ///
    ///<pre>URL of each frame</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.BackForwardCacheNotRestoredExplanationTree/properties/explanations。
    ///
    ///协议原文：
    ///
    ///<pre>Not restored reasons of each frame</pre>
    ///
    ///
    #[serde(rename = "explanations")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub explanations: Vec<crate::page::types::BackForwardCacheNotRestoredExplanation>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.BackForwardCacheNotRestoredExplanationTree/properties/children。
    ///
    ///协议原文：
    ///
    ///<pre>Array of children frame</pre>
    ///
    ///
    #[serde(rename = "children")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub children: Vec<crate::page::types::BackForwardCacheNotRestoredExplanationTree>,
}
impl<'de> serde::Deserialize<'de> for BackForwardCacheNotRestoredExplanationTree {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BackForwardCacheNotRestoredExplanationTree;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.BackForwardCacheNotRestoredExplanationTree/properties/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>URL of each frame</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.BackForwardCacheNotRestoredExplanationTree/properties/explanations。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Not restored reasons of each frame</pre>
                    ///
                    ///
                    #[serde(rename = "explanations")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub explanations:
                        Vec<crate::page::types::BackForwardCacheNotRestoredExplanation>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.BackForwardCacheNotRestoredExplanationTree/properties/children。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Array of children frame</pre>
                    ///
                    ///
                    #[serde(rename = "children")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub children:
                        Vec<crate::page::types::BackForwardCacheNotRestoredExplanationTree>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(BackForwardCacheNotRestoredExplanationTree {
                    url: fields.url,
                    explanations: fields.explanations,
                    children: fields.children,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.type.BackForwardCacheNotRestoredReason。
///
///协议原文：
///
///<pre>List of not restored reasons for back-forward cache.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum BackForwardCacheNotRestoredReason {
    ///协议值 "NotPrimaryMainFrame"。
    NotPrimaryMainFrame,
    ///协议值 "BackForwardCacheDisabled"。
    BackForwardCacheDisabled,
    ///协议值 "RelatedActiveContentsExist"。
    RelatedActiveContentsExist,
    ///协议值 "HTTPStatusNotOK"。
    HttpStatusNotOk,
    ///协议值 "SchemeNotHTTPOrHTTPS"。
    SchemeNotHttpOrHttps,
    ///协议值 "Loading"。
    Loading,
    ///协议值 "WasGrantedMediaAccess"。
    WasGrantedMediaAccess,
    ///协议值 "DisableForRenderFrameHostCalled"。
    DisableForRenderFrameHostCalled,
    ///协议值 "DomainNotAllowed"。
    DomainNotAllowed,
    ///协议值 "HTTPMethodNotGET"。
    HttpMethodNotGet,
    ///协议值 "SubframeIsNavigating"。
    SubframeIsNavigating,
    ///协议值 "Timeout"。
    Timeout,
    ///协议值 "CacheLimit"。
    CacheLimit,
    ///协议值 "JavaScriptExecution"。
    JavaScriptExecution,
    ///协议值 "RendererProcessKilled"。
    RendererProcessKilled,
    ///协议值 "RendererProcessCrashed"。
    RendererProcessCrashed,
    ///协议值 "SchedulerTrackedFeatureUsed"。
    SchedulerTrackedFeatureUsed,
    ///协议值 "ConflictingBrowsingInstance"。
    ConflictingBrowsingInstance,
    ///协议值 "CacheFlushed"。
    CacheFlushed,
    ///协议值 "ServiceWorkerVersionActivation"。
    ServiceWorkerVersionActivation,
    ///协议值 "SessionRestored"。
    SessionRestored,
    ///协议值 "ServiceWorkerPostMessage"。
    ServiceWorkerPostMessage,
    ///协议值 "EnteredBackForwardCacheBeforeServiceWorkerHostAdded"。
    EnteredBackForwardCacheBeforeServiceWorkerHostAdded,
    ///协议值 "RenderFrameHostReused_SameSite"。
    RenderFrameHostReusedSameSite,
    ///协议值 "RenderFrameHostReused_CrossSite"。
    RenderFrameHostReusedCrossSite,
    ///协议值 "ServiceWorkerClaim"。
    ServiceWorkerClaim,
    ///协议值 "IgnoreEventAndEvict"。
    IgnoreEventAndEvict,
    ///协议值 "HaveInnerContents"。
    HaveInnerContents,
    ///协议值 "TimeoutPuttingInCache"。
    TimeoutPuttingInCache,
    ///协议值 "BackForwardCacheDisabledByLowMemory"。
    BackForwardCacheDisabledByLowMemory,
    ///协议值 "BackForwardCacheDisabledByCommandLine"。
    BackForwardCacheDisabledByCommandLine,
    ///协议值 "NetworkRequestDatapipeDrainedAsBytesConsumer"。
    NetworkRequestDatapipeDrainedAsBytesConsumer,
    ///协议值 "NetworkRequestRedirected"。
    NetworkRequestRedirected,
    ///协议值 "NetworkRequestTimeout"。
    NetworkRequestTimeout,
    ///协议值 "NetworkExceedsBufferLimit"。
    NetworkExceedsBufferLimit,
    ///协议值 "NavigationCancelledWhileRestoring"。
    NavigationCancelledWhileRestoring,
    ///协议值 "NotMostRecentNavigationEntry"。
    NotMostRecentNavigationEntry,
    ///协议值 "BackForwardCacheDisabledForPrerender"。
    BackForwardCacheDisabledForPrerender,
    ///协议值 "UserAgentOverrideDiffers"。
    UserAgentOverrideDiffers,
    ///协议值 "ForegroundCacheLimit"。
    ForegroundCacheLimit,
    ///协议值 "ForwardCacheDisabled"。
    ForwardCacheDisabled,
    ///协议值 "BrowsingInstanceNotSwapped"。
    BrowsingInstanceNotSwapped,
    ///协议值 "BackForwardCacheDisabledForDelegate"。
    BackForwardCacheDisabledForDelegate,
    ///协议值 "UnloadHandlerExistsInMainFrame"。
    UnloadHandlerExistsInMainFrame,
    ///协议值 "UnloadHandlerExistsInSubFrame"。
    UnloadHandlerExistsInSubFrame,
    ///协议值 "ServiceWorkerUnregistration"。
    ServiceWorkerUnregistration,
    ///协议值 "CacheControlNoStore"。
    CacheControlNoStore,
    ///协议值 "CacheControlNoStoreCookieModified"。
    CacheControlNoStoreCookieModified,
    ///协议值 "CacheControlNoStoreHTTPOnlyCookieModified"。
    CacheControlNoStoreHttpOnlyCookieModified,
    ///协议值 "NoResponseHead"。
    NoResponseHead,
    ///协议值 "Unknown"。
    KnownUnknown,
    ///协议值 "ActivationNavigationsDisallowedForBug1234857"。
    ActivationNavigationsDisallowedForBug1234857,
    ///协议值 "ErrorDocument"。
    ErrorDocument,
    ///协议值 "FencedFramesEmbedder"。
    FencedFramesEmbedder,
    ///协议值 "CookieDisabled"。
    CookieDisabled,
    ///协议值 "HTTPAuthRequired"。
    HttpAuthRequired,
    ///协议值 "CookieFlushed"。
    CookieFlushed,
    ///协议值 "BroadcastChannelOnMessage"。
    BroadcastChannelOnMessage,
    ///协议值 "WebViewSettingsChanged"。
    WebViewSettingsChanged,
    ///协议值 "WebViewJavaScriptObjectChanged"。
    WebViewJavaScriptObjectChanged,
    ///协议值 "WebViewMessageListenerInjected"。
    WebViewMessageListenerInjected,
    ///协议值 "WebViewSafeBrowsingAllowlistChanged"。
    WebViewSafeBrowsingAllowlistChanged,
    ///协议值 "WebViewDocumentStartJavascriptChanged"。
    WebViewDocumentStartJavascriptChanged,
    ///协议值 "WebSocket"。
    WebSocket,
    ///协议值 "WebTransport"。
    WebTransport,
    ///协议值 "WebRTC"。
    WebRtc,
    ///协议值 "MainResourceHasCacheControlNoStore"。
    MainResourceHasCacheControlNoStore,
    ///协议值 "MainResourceHasCacheControlNoCache"。
    MainResourceHasCacheControlNoCache,
    ///协议值 "SubresourceHasCacheControlNoStore"。
    SubresourceHasCacheControlNoStore,
    ///协议值 "SubresourceHasCacheControlNoCache"。
    SubresourceHasCacheControlNoCache,
    ///协议值 "ContainsPlugins"。
    ContainsPlugins,
    ///协议值 "DocumentLoaded"。
    DocumentLoaded,
    ///协议值 "OutstandingNetworkRequestOthers"。
    OutstandingNetworkRequestOthers,
    ///协议值 "RequestedMIDIPermission"。
    RequestedMidiPermission,
    ///协议值 "RequestedAudioCapturePermission"。
    RequestedAudioCapturePermission,
    ///协议值 "RequestedVideoCapturePermission"。
    RequestedVideoCapturePermission,
    ///协议值 "RequestedBackForwardCacheBlockedSensors"。
    RequestedBackForwardCacheBlockedSensors,
    ///协议值 "RequestedBackgroundWorkPermission"。
    RequestedBackgroundWorkPermission,
    ///协议值 "BroadcastChannel"。
    BroadcastChannel,
    ///协议值 "WebXR"。
    WebXr,
    ///协议值 "SharedWorker"。
    SharedWorker,
    ///协议值 "SharedWorkerMessage"。
    SharedWorkerMessage,
    ///协议值 "SharedWorkerWithNoActiveClient"。
    SharedWorkerWithNoActiveClient,
    ///协议值 "WebLocks"。
    WebLocks,
    ///协议值 "WebLocksContention"。
    WebLocksContention,
    ///协议值 "WebHID"。
    WebHid,
    ///协议值 "WebBluetooth"。
    WebBluetooth,
    ///协议值 "WebShare"。
    WebShare,
    ///协议值 "RequestedStorageAccessGrant"。
    RequestedStorageAccessGrant,
    ///协议值 "WebNfc"。
    WebNfc,
    ///协议值 "OutstandingNetworkRequestFetch"。
    OutstandingNetworkRequestFetch,
    ///协议值 "OutstandingNetworkRequestXHR"。
    OutstandingNetworkRequestXhr,
    ///协议值 "AppBanner"。
    AppBanner,
    ///协议值 "Printing"。
    Printing,
    ///协议值 "WebDatabase"。
    WebDatabase,
    ///协议值 "PictureInPicture"。
    PictureInPicture,
    ///协议值 "SpeechRecognizer"。
    SpeechRecognizer,
    ///协议值 "IdleManager"。
    IdleManager,
    ///协议值 "PaymentManager"。
    PaymentManager,
    ///协议值 "SpeechSynthesis"。
    SpeechSynthesis,
    ///协议值 "KeyboardLock"。
    KeyboardLock,
    ///协议值 "WebOTPService"。
    WebOtpService,
    ///协议值 "OutstandingNetworkRequestDirectSocket"。
    OutstandingNetworkRequestDirectSocket,
    ///协议值 "InjectedJavascript"。
    InjectedJavascript,
    ///协议值 "InjectedStyleSheet"。
    InjectedStyleSheet,
    ///协议值 "KeepaliveRequest"。
    KeepaliveRequest,
    ///协议值 "IndexedDBEvent"。
    IndexedDbEvent,
    ///协议值 "Dummy"。
    Dummy,
    ///协议值 "JsNetworkRequestReceivedCacheControlNoStoreResource"。
    JsNetworkRequestReceivedCacheControlNoStoreResource,
    ///协议值 "WebRTCUsedWithCCNS"。
    WebRtcUsedWithCcns,
    ///协议值 "WebTransportUsedWithCCNS"。
    WebTransportUsedWithCcns,
    ///协议值 "WebSocketUsedWithCCNS"。
    WebSocketUsedWithCcns,
    ///协议值 "SmartCard"。
    SmartCard,
    ///协议值 "LiveMediaStreamTrack"。
    LiveMediaStreamTrack,
    ///协议值 "UnloadHandler"。
    UnloadHandler,
    ///协议值 "ParserAborted"。
    ParserAborted,
    ///协议值 "ContentSecurityHandler"。
    ContentSecurityHandler,
    ///协议值 "ContentWebAuthenticationAPI"。
    ContentWebAuthenticationApi,
    ///协议值 "ContentFileChooser"。
    ContentFileChooser,
    ///协议值 "ContentSerial"。
    ContentSerial,
    ///协议值 "ContentFileSystemAccess"。
    ContentFileSystemAccess,
    ///协议值 "ContentMediaDevicesDispatcherHost"。
    ContentMediaDevicesDispatcherHost,
    ///协议值 "ContentWebBluetooth"。
    ContentWebBluetooth,
    ///协议值 "ContentWebUSB"。
    ContentWebUsb,
    ///协议值 "ContentMediaSessionService"。
    ContentMediaSessionService,
    ///协议值 "ContentScreenReader"。
    ContentScreenReader,
    ///协议值 "ContentDiscarded"。
    ContentDiscarded,
    ///协议值 "EmbedderPopupBlockerTabHelper"。
    EmbedderPopupBlockerTabHelper,
    ///协议值 "EmbedderSafeBrowsingTriggeredPopupBlocker"。
    EmbedderSafeBrowsingTriggeredPopupBlocker,
    ///协议值 "EmbedderSafeBrowsingThreatDetails"。
    EmbedderSafeBrowsingThreatDetails,
    ///协议值 "EmbedderAppBannerManager"。
    EmbedderAppBannerManager,
    ///协议值 "EmbedderDomDistillerViewerSource"。
    EmbedderDomDistillerViewerSource,
    ///协议值 "EmbedderDomDistillerSelfDeletingRequestDelegate"。
    EmbedderDomDistillerSelfDeletingRequestDelegate,
    ///协议值 "EmbedderOomInterventionTabHelper"。
    EmbedderOomInterventionTabHelper,
    ///协议值 "EmbedderOfflinePage"。
    EmbedderOfflinePage,
    ///协议值 "EmbedderChromePasswordManagerClientBindCredentialManager"。
    EmbedderChromePasswordManagerClientBindCredentialManager,
    ///协议值 "EmbedderPermissionRequestManager"。
    EmbedderPermissionRequestManager,
    ///协议值 "EmbedderModalDialog"。
    EmbedderModalDialog,
    ///协议值 "EmbedderExtensions"。
    EmbedderExtensions,
    ///协议值 "EmbedderExtensionMessaging"。
    EmbedderExtensionMessaging,
    ///协议值 "EmbedderExtensionMessagingForOpenPort"。
    EmbedderExtensionMessagingForOpenPort,
    ///协议值 "EmbedderExtensionSentMessageToCachedFrame"。
    EmbedderExtensionSentMessageToCachedFrame,
    ///协议值 "EmbedderExtensionFrame"。
    EmbedderExtensionFrame,
    ///协议值 "EmbedderPrivilegedWebContents"。
    EmbedderPrivilegedWebContents,
    ///协议值 "RequestedByWebViewClient"。
    RequestedByWebViewClient,
    ///协议值 "PostMessageByWebViewClient"。
    PostMessageByWebViewClient,
    ///协议值 "CacheControlNoStoreDeviceBoundSessionTerminated"。
    CacheControlNoStoreDeviceBoundSessionTerminated,
    ///协议值 "CacheLimitPrunedOnModerateMemoryPressure"。
    CacheLimitPrunedOnModerateMemoryPressure,
    ///协议值 "CacheLimitPrunedOnCriticalMemoryPressure"。
    CacheLimitPrunedOnCriticalMemoryPressure,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl BackForwardCacheNotRestoredReason {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::types::BackForwardCacheNotRestoredReason::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::NotPrimaryMainFrame => "NotPrimaryMainFrame",
            Self::BackForwardCacheDisabled => "BackForwardCacheDisabled",
            Self::RelatedActiveContentsExist => "RelatedActiveContentsExist",
            Self::HttpStatusNotOk => "HTTPStatusNotOK",
            Self::SchemeNotHttpOrHttps => "SchemeNotHTTPOrHTTPS",
            Self::Loading => "Loading",
            Self::WasGrantedMediaAccess => "WasGrantedMediaAccess",
            Self::DisableForRenderFrameHostCalled => "DisableForRenderFrameHostCalled",
            Self::DomainNotAllowed => "DomainNotAllowed",
            Self::HttpMethodNotGet => "HTTPMethodNotGET",
            Self::SubframeIsNavigating => "SubframeIsNavigating",
            Self::Timeout => "Timeout",
            Self::CacheLimit => "CacheLimit",
            Self::JavaScriptExecution => "JavaScriptExecution",
            Self::RendererProcessKilled => "RendererProcessKilled",
            Self::RendererProcessCrashed => "RendererProcessCrashed",
            Self::SchedulerTrackedFeatureUsed => "SchedulerTrackedFeatureUsed",
            Self::ConflictingBrowsingInstance => "ConflictingBrowsingInstance",
            Self::CacheFlushed => "CacheFlushed",
            Self::ServiceWorkerVersionActivation => "ServiceWorkerVersionActivation",
            Self::SessionRestored => "SessionRestored",
            Self::ServiceWorkerPostMessage => "ServiceWorkerPostMessage",
            Self::EnteredBackForwardCacheBeforeServiceWorkerHostAdded => {
                "EnteredBackForwardCacheBeforeServiceWorkerHostAdded"
            }
            Self::RenderFrameHostReusedSameSite => "RenderFrameHostReused_SameSite",
            Self::RenderFrameHostReusedCrossSite => "RenderFrameHostReused_CrossSite",
            Self::ServiceWorkerClaim => "ServiceWorkerClaim",
            Self::IgnoreEventAndEvict => "IgnoreEventAndEvict",
            Self::HaveInnerContents => "HaveInnerContents",
            Self::TimeoutPuttingInCache => "TimeoutPuttingInCache",
            Self::BackForwardCacheDisabledByLowMemory => "BackForwardCacheDisabledByLowMemory",
            Self::BackForwardCacheDisabledByCommandLine => "BackForwardCacheDisabledByCommandLine",
            Self::NetworkRequestDatapipeDrainedAsBytesConsumer => {
                "NetworkRequestDatapipeDrainedAsBytesConsumer"
            }
            Self::NetworkRequestRedirected => "NetworkRequestRedirected",
            Self::NetworkRequestTimeout => "NetworkRequestTimeout",
            Self::NetworkExceedsBufferLimit => "NetworkExceedsBufferLimit",
            Self::NavigationCancelledWhileRestoring => "NavigationCancelledWhileRestoring",
            Self::NotMostRecentNavigationEntry => "NotMostRecentNavigationEntry",
            Self::BackForwardCacheDisabledForPrerender => "BackForwardCacheDisabledForPrerender",
            Self::UserAgentOverrideDiffers => "UserAgentOverrideDiffers",
            Self::ForegroundCacheLimit => "ForegroundCacheLimit",
            Self::ForwardCacheDisabled => "ForwardCacheDisabled",
            Self::BrowsingInstanceNotSwapped => "BrowsingInstanceNotSwapped",
            Self::BackForwardCacheDisabledForDelegate => "BackForwardCacheDisabledForDelegate",
            Self::UnloadHandlerExistsInMainFrame => "UnloadHandlerExistsInMainFrame",
            Self::UnloadHandlerExistsInSubFrame => "UnloadHandlerExistsInSubFrame",
            Self::ServiceWorkerUnregistration => "ServiceWorkerUnregistration",
            Self::CacheControlNoStore => "CacheControlNoStore",
            Self::CacheControlNoStoreCookieModified => "CacheControlNoStoreCookieModified",
            Self::CacheControlNoStoreHttpOnlyCookieModified => {
                "CacheControlNoStoreHTTPOnlyCookieModified"
            }
            Self::NoResponseHead => "NoResponseHead",
            Self::KnownUnknown => "Unknown",
            Self::ActivationNavigationsDisallowedForBug1234857 => {
                "ActivationNavigationsDisallowedForBug1234857"
            }
            Self::ErrorDocument => "ErrorDocument",
            Self::FencedFramesEmbedder => "FencedFramesEmbedder",
            Self::CookieDisabled => "CookieDisabled",
            Self::HttpAuthRequired => "HTTPAuthRequired",
            Self::CookieFlushed => "CookieFlushed",
            Self::BroadcastChannelOnMessage => "BroadcastChannelOnMessage",
            Self::WebViewSettingsChanged => "WebViewSettingsChanged",
            Self::WebViewJavaScriptObjectChanged => "WebViewJavaScriptObjectChanged",
            Self::WebViewMessageListenerInjected => "WebViewMessageListenerInjected",
            Self::WebViewSafeBrowsingAllowlistChanged => "WebViewSafeBrowsingAllowlistChanged",
            Self::WebViewDocumentStartJavascriptChanged => "WebViewDocumentStartJavascriptChanged",
            Self::WebSocket => "WebSocket",
            Self::WebTransport => "WebTransport",
            Self::WebRtc => "WebRTC",
            Self::MainResourceHasCacheControlNoStore => "MainResourceHasCacheControlNoStore",
            Self::MainResourceHasCacheControlNoCache => "MainResourceHasCacheControlNoCache",
            Self::SubresourceHasCacheControlNoStore => "SubresourceHasCacheControlNoStore",
            Self::SubresourceHasCacheControlNoCache => "SubresourceHasCacheControlNoCache",
            Self::ContainsPlugins => "ContainsPlugins",
            Self::DocumentLoaded => "DocumentLoaded",
            Self::OutstandingNetworkRequestOthers => "OutstandingNetworkRequestOthers",
            Self::RequestedMidiPermission => "RequestedMIDIPermission",
            Self::RequestedAudioCapturePermission => "RequestedAudioCapturePermission",
            Self::RequestedVideoCapturePermission => "RequestedVideoCapturePermission",
            Self::RequestedBackForwardCacheBlockedSensors => {
                "RequestedBackForwardCacheBlockedSensors"
            }
            Self::RequestedBackgroundWorkPermission => "RequestedBackgroundWorkPermission",
            Self::BroadcastChannel => "BroadcastChannel",
            Self::WebXr => "WebXR",
            Self::SharedWorker => "SharedWorker",
            Self::SharedWorkerMessage => "SharedWorkerMessage",
            Self::SharedWorkerWithNoActiveClient => "SharedWorkerWithNoActiveClient",
            Self::WebLocks => "WebLocks",
            Self::WebLocksContention => "WebLocksContention",
            Self::WebHid => "WebHID",
            Self::WebBluetooth => "WebBluetooth",
            Self::WebShare => "WebShare",
            Self::RequestedStorageAccessGrant => "RequestedStorageAccessGrant",
            Self::WebNfc => "WebNfc",
            Self::OutstandingNetworkRequestFetch => "OutstandingNetworkRequestFetch",
            Self::OutstandingNetworkRequestXhr => "OutstandingNetworkRequestXHR",
            Self::AppBanner => "AppBanner",
            Self::Printing => "Printing",
            Self::WebDatabase => "WebDatabase",
            Self::PictureInPicture => "PictureInPicture",
            Self::SpeechRecognizer => "SpeechRecognizer",
            Self::IdleManager => "IdleManager",
            Self::PaymentManager => "PaymentManager",
            Self::SpeechSynthesis => "SpeechSynthesis",
            Self::KeyboardLock => "KeyboardLock",
            Self::WebOtpService => "WebOTPService",
            Self::OutstandingNetworkRequestDirectSocket => "OutstandingNetworkRequestDirectSocket",
            Self::InjectedJavascript => "InjectedJavascript",
            Self::InjectedStyleSheet => "InjectedStyleSheet",
            Self::KeepaliveRequest => "KeepaliveRequest",
            Self::IndexedDbEvent => "IndexedDBEvent",
            Self::Dummy => "Dummy",
            Self::JsNetworkRequestReceivedCacheControlNoStoreResource => {
                "JsNetworkRequestReceivedCacheControlNoStoreResource"
            }
            Self::WebRtcUsedWithCcns => "WebRTCUsedWithCCNS",
            Self::WebTransportUsedWithCcns => "WebTransportUsedWithCCNS",
            Self::WebSocketUsedWithCcns => "WebSocketUsedWithCCNS",
            Self::SmartCard => "SmartCard",
            Self::LiveMediaStreamTrack => "LiveMediaStreamTrack",
            Self::UnloadHandler => "UnloadHandler",
            Self::ParserAborted => "ParserAborted",
            Self::ContentSecurityHandler => "ContentSecurityHandler",
            Self::ContentWebAuthenticationApi => "ContentWebAuthenticationAPI",
            Self::ContentFileChooser => "ContentFileChooser",
            Self::ContentSerial => "ContentSerial",
            Self::ContentFileSystemAccess => "ContentFileSystemAccess",
            Self::ContentMediaDevicesDispatcherHost => "ContentMediaDevicesDispatcherHost",
            Self::ContentWebBluetooth => "ContentWebBluetooth",
            Self::ContentWebUsb => "ContentWebUSB",
            Self::ContentMediaSessionService => "ContentMediaSessionService",
            Self::ContentScreenReader => "ContentScreenReader",
            Self::ContentDiscarded => "ContentDiscarded",
            Self::EmbedderPopupBlockerTabHelper => "EmbedderPopupBlockerTabHelper",
            Self::EmbedderSafeBrowsingTriggeredPopupBlocker => {
                "EmbedderSafeBrowsingTriggeredPopupBlocker"
            }
            Self::EmbedderSafeBrowsingThreatDetails => "EmbedderSafeBrowsingThreatDetails",
            Self::EmbedderAppBannerManager => "EmbedderAppBannerManager",
            Self::EmbedderDomDistillerViewerSource => "EmbedderDomDistillerViewerSource",
            Self::EmbedderDomDistillerSelfDeletingRequestDelegate => {
                "EmbedderDomDistillerSelfDeletingRequestDelegate"
            }
            Self::EmbedderOomInterventionTabHelper => "EmbedderOomInterventionTabHelper",
            Self::EmbedderOfflinePage => "EmbedderOfflinePage",
            Self::EmbedderChromePasswordManagerClientBindCredentialManager => {
                "EmbedderChromePasswordManagerClientBindCredentialManager"
            }
            Self::EmbedderPermissionRequestManager => "EmbedderPermissionRequestManager",
            Self::EmbedderModalDialog => "EmbedderModalDialog",
            Self::EmbedderExtensions => "EmbedderExtensions",
            Self::EmbedderExtensionMessaging => "EmbedderExtensionMessaging",
            Self::EmbedderExtensionMessagingForOpenPort => "EmbedderExtensionMessagingForOpenPort",
            Self::EmbedderExtensionSentMessageToCachedFrame => {
                "EmbedderExtensionSentMessageToCachedFrame"
            }
            Self::EmbedderExtensionFrame => "EmbedderExtensionFrame",
            Self::EmbedderPrivilegedWebContents => "EmbedderPrivilegedWebContents",
            Self::RequestedByWebViewClient => "RequestedByWebViewClient",
            Self::PostMessageByWebViewClient => "PostMessageByWebViewClient",
            Self::CacheControlNoStoreDeviceBoundSessionTerminated => {
                "CacheControlNoStoreDeviceBoundSessionTerminated"
            }
            Self::CacheLimitPrunedOnModerateMemoryPressure => {
                "CacheLimitPrunedOnModerateMemoryPressure"
            }
            Self::CacheLimitPrunedOnCriticalMemoryPressure => {
                "CacheLimitPrunedOnCriticalMemoryPressure"
            }
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for BackForwardCacheNotRestoredReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for BackForwardCacheNotRestoredReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "NotPrimaryMainFrame" => {
                BackForwardCacheNotRestoredReason::NotPrimaryMainFrame
            }
            "BackForwardCacheDisabled" => {
                BackForwardCacheNotRestoredReason::BackForwardCacheDisabled
            }
            "RelatedActiveContentsExist" => {
                BackForwardCacheNotRestoredReason::RelatedActiveContentsExist
            }
            "HTTPStatusNotOK" => BackForwardCacheNotRestoredReason::HttpStatusNotOk,
            "SchemeNotHTTPOrHTTPS" => {
                BackForwardCacheNotRestoredReason::SchemeNotHttpOrHttps
            }
            "Loading" => BackForwardCacheNotRestoredReason::Loading,
            "WasGrantedMediaAccess" => {
                BackForwardCacheNotRestoredReason::WasGrantedMediaAccess
            }
            "DisableForRenderFrameHostCalled" => {
                BackForwardCacheNotRestoredReason::DisableForRenderFrameHostCalled
            }
            "DomainNotAllowed" => BackForwardCacheNotRestoredReason::DomainNotAllowed,
            "HTTPMethodNotGET" => BackForwardCacheNotRestoredReason::HttpMethodNotGet,
            "SubframeIsNavigating" => {
                BackForwardCacheNotRestoredReason::SubframeIsNavigating
            }
            "Timeout" => BackForwardCacheNotRestoredReason::Timeout,
            "CacheLimit" => BackForwardCacheNotRestoredReason::CacheLimit,
            "JavaScriptExecution" => {
                BackForwardCacheNotRestoredReason::JavaScriptExecution
            }
            "RendererProcessKilled" => {
                BackForwardCacheNotRestoredReason::RendererProcessKilled
            }
            "RendererProcessCrashed" => {
                BackForwardCacheNotRestoredReason::RendererProcessCrashed
            }
            "SchedulerTrackedFeatureUsed" => {
                BackForwardCacheNotRestoredReason::SchedulerTrackedFeatureUsed
            }
            "ConflictingBrowsingInstance" => {
                BackForwardCacheNotRestoredReason::ConflictingBrowsingInstance
            }
            "CacheFlushed" => BackForwardCacheNotRestoredReason::CacheFlushed,
            "ServiceWorkerVersionActivation" => {
                BackForwardCacheNotRestoredReason::ServiceWorkerVersionActivation
            }
            "SessionRestored" => BackForwardCacheNotRestoredReason::SessionRestored,
            "ServiceWorkerPostMessage" => {
                BackForwardCacheNotRestoredReason::ServiceWorkerPostMessage
            }
            "EnteredBackForwardCacheBeforeServiceWorkerHostAdded" => {
                BackForwardCacheNotRestoredReason::EnteredBackForwardCacheBeforeServiceWorkerHostAdded
            }
            "RenderFrameHostReused_SameSite" => {
                BackForwardCacheNotRestoredReason::RenderFrameHostReusedSameSite
            }
            "RenderFrameHostReused_CrossSite" => {
                BackForwardCacheNotRestoredReason::RenderFrameHostReusedCrossSite
            }
            "ServiceWorkerClaim" => BackForwardCacheNotRestoredReason::ServiceWorkerClaim,
            "IgnoreEventAndEvict" => {
                BackForwardCacheNotRestoredReason::IgnoreEventAndEvict
            }
            "HaveInnerContents" => BackForwardCacheNotRestoredReason::HaveInnerContents,
            "TimeoutPuttingInCache" => {
                BackForwardCacheNotRestoredReason::TimeoutPuttingInCache
            }
            "BackForwardCacheDisabledByLowMemory" => {
                BackForwardCacheNotRestoredReason::BackForwardCacheDisabledByLowMemory
            }
            "BackForwardCacheDisabledByCommandLine" => {
                BackForwardCacheNotRestoredReason::BackForwardCacheDisabledByCommandLine
            }
            "NetworkRequestDatapipeDrainedAsBytesConsumer" => {
                BackForwardCacheNotRestoredReason::NetworkRequestDatapipeDrainedAsBytesConsumer
            }
            "NetworkRequestRedirected" => {
                BackForwardCacheNotRestoredReason::NetworkRequestRedirected
            }
            "NetworkRequestTimeout" => {
                BackForwardCacheNotRestoredReason::NetworkRequestTimeout
            }
            "NetworkExceedsBufferLimit" => {
                BackForwardCacheNotRestoredReason::NetworkExceedsBufferLimit
            }
            "NavigationCancelledWhileRestoring" => {
                BackForwardCacheNotRestoredReason::NavigationCancelledWhileRestoring
            }
            "NotMostRecentNavigationEntry" => {
                BackForwardCacheNotRestoredReason::NotMostRecentNavigationEntry
            }
            "BackForwardCacheDisabledForPrerender" => {
                BackForwardCacheNotRestoredReason::BackForwardCacheDisabledForPrerender
            }
            "UserAgentOverrideDiffers" => {
                BackForwardCacheNotRestoredReason::UserAgentOverrideDiffers
            }
            "ForegroundCacheLimit" => {
                BackForwardCacheNotRestoredReason::ForegroundCacheLimit
            }
            "ForwardCacheDisabled" => {
                BackForwardCacheNotRestoredReason::ForwardCacheDisabled
            }
            "BrowsingInstanceNotSwapped" => {
                BackForwardCacheNotRestoredReason::BrowsingInstanceNotSwapped
            }
            "BackForwardCacheDisabledForDelegate" => {
                BackForwardCacheNotRestoredReason::BackForwardCacheDisabledForDelegate
            }
            "UnloadHandlerExistsInMainFrame" => {
                BackForwardCacheNotRestoredReason::UnloadHandlerExistsInMainFrame
            }
            "UnloadHandlerExistsInSubFrame" => {
                BackForwardCacheNotRestoredReason::UnloadHandlerExistsInSubFrame
            }
            "ServiceWorkerUnregistration" => {
                BackForwardCacheNotRestoredReason::ServiceWorkerUnregistration
            }
            "CacheControlNoStore" => {
                BackForwardCacheNotRestoredReason::CacheControlNoStore
            }
            "CacheControlNoStoreCookieModified" => {
                BackForwardCacheNotRestoredReason::CacheControlNoStoreCookieModified
            }
            "CacheControlNoStoreHTTPOnlyCookieModified" => {
                BackForwardCacheNotRestoredReason::CacheControlNoStoreHttpOnlyCookieModified
            }
            "NoResponseHead" => BackForwardCacheNotRestoredReason::NoResponseHead,
            "Unknown" => BackForwardCacheNotRestoredReason::KnownUnknown,
            "ActivationNavigationsDisallowedForBug1234857" => {
                BackForwardCacheNotRestoredReason::ActivationNavigationsDisallowedForBug1234857
            }
            "ErrorDocument" => BackForwardCacheNotRestoredReason::ErrorDocument,
            "FencedFramesEmbedder" => {
                BackForwardCacheNotRestoredReason::FencedFramesEmbedder
            }
            "CookieDisabled" => BackForwardCacheNotRestoredReason::CookieDisabled,
            "HTTPAuthRequired" => BackForwardCacheNotRestoredReason::HttpAuthRequired,
            "CookieFlushed" => BackForwardCacheNotRestoredReason::CookieFlushed,
            "BroadcastChannelOnMessage" => {
                BackForwardCacheNotRestoredReason::BroadcastChannelOnMessage
            }
            "WebViewSettingsChanged" => {
                BackForwardCacheNotRestoredReason::WebViewSettingsChanged
            }
            "WebViewJavaScriptObjectChanged" => {
                BackForwardCacheNotRestoredReason::WebViewJavaScriptObjectChanged
            }
            "WebViewMessageListenerInjected" => {
                BackForwardCacheNotRestoredReason::WebViewMessageListenerInjected
            }
            "WebViewSafeBrowsingAllowlistChanged" => {
                BackForwardCacheNotRestoredReason::WebViewSafeBrowsingAllowlistChanged
            }
            "WebViewDocumentStartJavascriptChanged" => {
                BackForwardCacheNotRestoredReason::WebViewDocumentStartJavascriptChanged
            }
            "WebSocket" => BackForwardCacheNotRestoredReason::WebSocket,
            "WebTransport" => BackForwardCacheNotRestoredReason::WebTransport,
            "WebRTC" => BackForwardCacheNotRestoredReason::WebRtc,
            "MainResourceHasCacheControlNoStore" => {
                BackForwardCacheNotRestoredReason::MainResourceHasCacheControlNoStore
            }
            "MainResourceHasCacheControlNoCache" => {
                BackForwardCacheNotRestoredReason::MainResourceHasCacheControlNoCache
            }
            "SubresourceHasCacheControlNoStore" => {
                BackForwardCacheNotRestoredReason::SubresourceHasCacheControlNoStore
            }
            "SubresourceHasCacheControlNoCache" => {
                BackForwardCacheNotRestoredReason::SubresourceHasCacheControlNoCache
            }
            "ContainsPlugins" => BackForwardCacheNotRestoredReason::ContainsPlugins,
            "DocumentLoaded" => BackForwardCacheNotRestoredReason::DocumentLoaded,
            "OutstandingNetworkRequestOthers" => {
                BackForwardCacheNotRestoredReason::OutstandingNetworkRequestOthers
            }
            "RequestedMIDIPermission" => {
                BackForwardCacheNotRestoredReason::RequestedMidiPermission
            }
            "RequestedAudioCapturePermission" => {
                BackForwardCacheNotRestoredReason::RequestedAudioCapturePermission
            }
            "RequestedVideoCapturePermission" => {
                BackForwardCacheNotRestoredReason::RequestedVideoCapturePermission
            }
            "RequestedBackForwardCacheBlockedSensors" => {
                BackForwardCacheNotRestoredReason::RequestedBackForwardCacheBlockedSensors
            }
            "RequestedBackgroundWorkPermission" => {
                BackForwardCacheNotRestoredReason::RequestedBackgroundWorkPermission
            }
            "BroadcastChannel" => BackForwardCacheNotRestoredReason::BroadcastChannel,
            "WebXR" => BackForwardCacheNotRestoredReason::WebXr,
            "SharedWorker" => BackForwardCacheNotRestoredReason::SharedWorker,
            "SharedWorkerMessage" => {
                BackForwardCacheNotRestoredReason::SharedWorkerMessage
            }
            "SharedWorkerWithNoActiveClient" => {
                BackForwardCacheNotRestoredReason::SharedWorkerWithNoActiveClient
            }
            "WebLocks" => BackForwardCacheNotRestoredReason::WebLocks,
            "WebLocksContention" => BackForwardCacheNotRestoredReason::WebLocksContention,
            "WebHID" => BackForwardCacheNotRestoredReason::WebHid,
            "WebBluetooth" => BackForwardCacheNotRestoredReason::WebBluetooth,
            "WebShare" => BackForwardCacheNotRestoredReason::WebShare,
            "RequestedStorageAccessGrant" => {
                BackForwardCacheNotRestoredReason::RequestedStorageAccessGrant
            }
            "WebNfc" => BackForwardCacheNotRestoredReason::WebNfc,
            "OutstandingNetworkRequestFetch" => {
                BackForwardCacheNotRestoredReason::OutstandingNetworkRequestFetch
            }
            "OutstandingNetworkRequestXHR" => {
                BackForwardCacheNotRestoredReason::OutstandingNetworkRequestXhr
            }
            "AppBanner" => BackForwardCacheNotRestoredReason::AppBanner,
            "Printing" => BackForwardCacheNotRestoredReason::Printing,
            "WebDatabase" => BackForwardCacheNotRestoredReason::WebDatabase,
            "PictureInPicture" => BackForwardCacheNotRestoredReason::PictureInPicture,
            "SpeechRecognizer" => BackForwardCacheNotRestoredReason::SpeechRecognizer,
            "IdleManager" => BackForwardCacheNotRestoredReason::IdleManager,
            "PaymentManager" => BackForwardCacheNotRestoredReason::PaymentManager,
            "SpeechSynthesis" => BackForwardCacheNotRestoredReason::SpeechSynthesis,
            "KeyboardLock" => BackForwardCacheNotRestoredReason::KeyboardLock,
            "WebOTPService" => BackForwardCacheNotRestoredReason::WebOtpService,
            "OutstandingNetworkRequestDirectSocket" => {
                BackForwardCacheNotRestoredReason::OutstandingNetworkRequestDirectSocket
            }
            "InjectedJavascript" => BackForwardCacheNotRestoredReason::InjectedJavascript,
            "InjectedStyleSheet" => BackForwardCacheNotRestoredReason::InjectedStyleSheet,
            "KeepaliveRequest" => BackForwardCacheNotRestoredReason::KeepaliveRequest,
            "IndexedDBEvent" => BackForwardCacheNotRestoredReason::IndexedDbEvent,
            "Dummy" => BackForwardCacheNotRestoredReason::Dummy,
            "JsNetworkRequestReceivedCacheControlNoStoreResource" => {
                BackForwardCacheNotRestoredReason::JsNetworkRequestReceivedCacheControlNoStoreResource
            }
            "WebRTCUsedWithCCNS" => BackForwardCacheNotRestoredReason::WebRtcUsedWithCcns,
            "WebTransportUsedWithCCNS" => {
                BackForwardCacheNotRestoredReason::WebTransportUsedWithCcns
            }
            "WebSocketUsedWithCCNS" => {
                BackForwardCacheNotRestoredReason::WebSocketUsedWithCcns
            }
            "SmartCard" => BackForwardCacheNotRestoredReason::SmartCard,
            "LiveMediaStreamTrack" => {
                BackForwardCacheNotRestoredReason::LiveMediaStreamTrack
            }
            "UnloadHandler" => BackForwardCacheNotRestoredReason::UnloadHandler,
            "ParserAborted" => BackForwardCacheNotRestoredReason::ParserAborted,
            "ContentSecurityHandler" => {
                BackForwardCacheNotRestoredReason::ContentSecurityHandler
            }
            "ContentWebAuthenticationAPI" => {
                BackForwardCacheNotRestoredReason::ContentWebAuthenticationApi
            }
            "ContentFileChooser" => BackForwardCacheNotRestoredReason::ContentFileChooser,
            "ContentSerial" => BackForwardCacheNotRestoredReason::ContentSerial,
            "ContentFileSystemAccess" => {
                BackForwardCacheNotRestoredReason::ContentFileSystemAccess
            }
            "ContentMediaDevicesDispatcherHost" => {
                BackForwardCacheNotRestoredReason::ContentMediaDevicesDispatcherHost
            }
            "ContentWebBluetooth" => {
                BackForwardCacheNotRestoredReason::ContentWebBluetooth
            }
            "ContentWebUSB" => BackForwardCacheNotRestoredReason::ContentWebUsb,
            "ContentMediaSessionService" => {
                BackForwardCacheNotRestoredReason::ContentMediaSessionService
            }
            "ContentScreenReader" => {
                BackForwardCacheNotRestoredReason::ContentScreenReader
            }
            "ContentDiscarded" => BackForwardCacheNotRestoredReason::ContentDiscarded,
            "EmbedderPopupBlockerTabHelper" => {
                BackForwardCacheNotRestoredReason::EmbedderPopupBlockerTabHelper
            }
            "EmbedderSafeBrowsingTriggeredPopupBlocker" => {
                BackForwardCacheNotRestoredReason::EmbedderSafeBrowsingTriggeredPopupBlocker
            }
            "EmbedderSafeBrowsingThreatDetails" => {
                BackForwardCacheNotRestoredReason::EmbedderSafeBrowsingThreatDetails
            }
            "EmbedderAppBannerManager" => {
                BackForwardCacheNotRestoredReason::EmbedderAppBannerManager
            }
            "EmbedderDomDistillerViewerSource" => {
                BackForwardCacheNotRestoredReason::EmbedderDomDistillerViewerSource
            }
            "EmbedderDomDistillerSelfDeletingRequestDelegate" => {
                BackForwardCacheNotRestoredReason::EmbedderDomDistillerSelfDeletingRequestDelegate
            }
            "EmbedderOomInterventionTabHelper" => {
                BackForwardCacheNotRestoredReason::EmbedderOomInterventionTabHelper
            }
            "EmbedderOfflinePage" => {
                BackForwardCacheNotRestoredReason::EmbedderOfflinePage
            }
            "EmbedderChromePasswordManagerClientBindCredentialManager" => {
                BackForwardCacheNotRestoredReason::EmbedderChromePasswordManagerClientBindCredentialManager
            }
            "EmbedderPermissionRequestManager" => {
                BackForwardCacheNotRestoredReason::EmbedderPermissionRequestManager
            }
            "EmbedderModalDialog" => {
                BackForwardCacheNotRestoredReason::EmbedderModalDialog
            }
            "EmbedderExtensions" => BackForwardCacheNotRestoredReason::EmbedderExtensions,
            "EmbedderExtensionMessaging" => {
                BackForwardCacheNotRestoredReason::EmbedderExtensionMessaging
            }
            "EmbedderExtensionMessagingForOpenPort" => {
                BackForwardCacheNotRestoredReason::EmbedderExtensionMessagingForOpenPort
            }
            "EmbedderExtensionSentMessageToCachedFrame" => {
                BackForwardCacheNotRestoredReason::EmbedderExtensionSentMessageToCachedFrame
            }
            "EmbedderExtensionFrame" => {
                BackForwardCacheNotRestoredReason::EmbedderExtensionFrame
            }
            "EmbedderPrivilegedWebContents" => {
                BackForwardCacheNotRestoredReason::EmbedderPrivilegedWebContents
            }
            "RequestedByWebViewClient" => {
                BackForwardCacheNotRestoredReason::RequestedByWebViewClient
            }
            "PostMessageByWebViewClient" => {
                BackForwardCacheNotRestoredReason::PostMessageByWebViewClient
            }
            "CacheControlNoStoreDeviceBoundSessionTerminated" => {
                BackForwardCacheNotRestoredReason::CacheControlNoStoreDeviceBoundSessionTerminated
            }
            "CacheLimitPrunedOnModerateMemoryPressure" => {
                BackForwardCacheNotRestoredReason::CacheLimitPrunedOnModerateMemoryPressure
            }
            "CacheLimitPrunedOnCriticalMemoryPressure" => {
                BackForwardCacheNotRestoredReason::CacheLimitPrunedOnCriticalMemoryPressure
            }
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for BackForwardCacheNotRestoredReason {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BackForwardCacheNotRestoredReason {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BackForwardCacheNotRestoredReason;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(
                    match v {
                        "NotPrimaryMainFrame" => {
                            BackForwardCacheNotRestoredReason::NotPrimaryMainFrame
                        }
                        "BackForwardCacheDisabled" => {
                            BackForwardCacheNotRestoredReason::BackForwardCacheDisabled
                        }
                        "RelatedActiveContentsExist" => {
                            BackForwardCacheNotRestoredReason::RelatedActiveContentsExist
                        }
                        "HTTPStatusNotOK" => {
                            BackForwardCacheNotRestoredReason::HttpStatusNotOk
                        }
                        "SchemeNotHTTPOrHTTPS" => {
                            BackForwardCacheNotRestoredReason::SchemeNotHttpOrHttps
                        }
                        "Loading" => BackForwardCacheNotRestoredReason::Loading,
                        "WasGrantedMediaAccess" => {
                            BackForwardCacheNotRestoredReason::WasGrantedMediaAccess
                        }
                        "DisableForRenderFrameHostCalled" => {
                            BackForwardCacheNotRestoredReason::DisableForRenderFrameHostCalled
                        }
                        "DomainNotAllowed" => {
                            BackForwardCacheNotRestoredReason::DomainNotAllowed
                        }
                        "HTTPMethodNotGET" => {
                            BackForwardCacheNotRestoredReason::HttpMethodNotGet
                        }
                        "SubframeIsNavigating" => {
                            BackForwardCacheNotRestoredReason::SubframeIsNavigating
                        }
                        "Timeout" => BackForwardCacheNotRestoredReason::Timeout,
                        "CacheLimit" => BackForwardCacheNotRestoredReason::CacheLimit,
                        "JavaScriptExecution" => {
                            BackForwardCacheNotRestoredReason::JavaScriptExecution
                        }
                        "RendererProcessKilled" => {
                            BackForwardCacheNotRestoredReason::RendererProcessKilled
                        }
                        "RendererProcessCrashed" => {
                            BackForwardCacheNotRestoredReason::RendererProcessCrashed
                        }
                        "SchedulerTrackedFeatureUsed" => {
                            BackForwardCacheNotRestoredReason::SchedulerTrackedFeatureUsed
                        }
                        "ConflictingBrowsingInstance" => {
                            BackForwardCacheNotRestoredReason::ConflictingBrowsingInstance
                        }
                        "CacheFlushed" => BackForwardCacheNotRestoredReason::CacheFlushed,
                        "ServiceWorkerVersionActivation" => {
                            BackForwardCacheNotRestoredReason::ServiceWorkerVersionActivation
                        }
                        "SessionRestored" => {
                            BackForwardCacheNotRestoredReason::SessionRestored
                        }
                        "ServiceWorkerPostMessage" => {
                            BackForwardCacheNotRestoredReason::ServiceWorkerPostMessage
                        }
                        "EnteredBackForwardCacheBeforeServiceWorkerHostAdded" => {
                            BackForwardCacheNotRestoredReason::EnteredBackForwardCacheBeforeServiceWorkerHostAdded
                        }
                        "RenderFrameHostReused_SameSite" => {
                            BackForwardCacheNotRestoredReason::RenderFrameHostReusedSameSite
                        }
                        "RenderFrameHostReused_CrossSite" => {
                            BackForwardCacheNotRestoredReason::RenderFrameHostReusedCrossSite
                        }
                        "ServiceWorkerClaim" => {
                            BackForwardCacheNotRestoredReason::ServiceWorkerClaim
                        }
                        "IgnoreEventAndEvict" => {
                            BackForwardCacheNotRestoredReason::IgnoreEventAndEvict
                        }
                        "HaveInnerContents" => {
                            BackForwardCacheNotRestoredReason::HaveInnerContents
                        }
                        "TimeoutPuttingInCache" => {
                            BackForwardCacheNotRestoredReason::TimeoutPuttingInCache
                        }
                        "BackForwardCacheDisabledByLowMemory" => {
                            BackForwardCacheNotRestoredReason::BackForwardCacheDisabledByLowMemory
                        }
                        "BackForwardCacheDisabledByCommandLine" => {
                            BackForwardCacheNotRestoredReason::BackForwardCacheDisabledByCommandLine
                        }
                        "NetworkRequestDatapipeDrainedAsBytesConsumer" => {
                            BackForwardCacheNotRestoredReason::NetworkRequestDatapipeDrainedAsBytesConsumer
                        }
                        "NetworkRequestRedirected" => {
                            BackForwardCacheNotRestoredReason::NetworkRequestRedirected
                        }
                        "NetworkRequestTimeout" => {
                            BackForwardCacheNotRestoredReason::NetworkRequestTimeout
                        }
                        "NetworkExceedsBufferLimit" => {
                            BackForwardCacheNotRestoredReason::NetworkExceedsBufferLimit
                        }
                        "NavigationCancelledWhileRestoring" => {
                            BackForwardCacheNotRestoredReason::NavigationCancelledWhileRestoring
                        }
                        "NotMostRecentNavigationEntry" => {
                            BackForwardCacheNotRestoredReason::NotMostRecentNavigationEntry
                        }
                        "BackForwardCacheDisabledForPrerender" => {
                            BackForwardCacheNotRestoredReason::BackForwardCacheDisabledForPrerender
                        }
                        "UserAgentOverrideDiffers" => {
                            BackForwardCacheNotRestoredReason::UserAgentOverrideDiffers
                        }
                        "ForegroundCacheLimit" => {
                            BackForwardCacheNotRestoredReason::ForegroundCacheLimit
                        }
                        "ForwardCacheDisabled" => {
                            BackForwardCacheNotRestoredReason::ForwardCacheDisabled
                        }
                        "BrowsingInstanceNotSwapped" => {
                            BackForwardCacheNotRestoredReason::BrowsingInstanceNotSwapped
                        }
                        "BackForwardCacheDisabledForDelegate" => {
                            BackForwardCacheNotRestoredReason::BackForwardCacheDisabledForDelegate
                        }
                        "UnloadHandlerExistsInMainFrame" => {
                            BackForwardCacheNotRestoredReason::UnloadHandlerExistsInMainFrame
                        }
                        "UnloadHandlerExistsInSubFrame" => {
                            BackForwardCacheNotRestoredReason::UnloadHandlerExistsInSubFrame
                        }
                        "ServiceWorkerUnregistration" => {
                            BackForwardCacheNotRestoredReason::ServiceWorkerUnregistration
                        }
                        "CacheControlNoStore" => {
                            BackForwardCacheNotRestoredReason::CacheControlNoStore
                        }
                        "CacheControlNoStoreCookieModified" => {
                            BackForwardCacheNotRestoredReason::CacheControlNoStoreCookieModified
                        }
                        "CacheControlNoStoreHTTPOnlyCookieModified" => {
                            BackForwardCacheNotRestoredReason::CacheControlNoStoreHttpOnlyCookieModified
                        }
                        "NoResponseHead" => {
                            BackForwardCacheNotRestoredReason::NoResponseHead
                        }
                        "Unknown" => BackForwardCacheNotRestoredReason::KnownUnknown,
                        "ActivationNavigationsDisallowedForBug1234857" => {
                            BackForwardCacheNotRestoredReason::ActivationNavigationsDisallowedForBug1234857
                        }
                        "ErrorDocument" => {
                            BackForwardCacheNotRestoredReason::ErrorDocument
                        }
                        "FencedFramesEmbedder" => {
                            BackForwardCacheNotRestoredReason::FencedFramesEmbedder
                        }
                        "CookieDisabled" => {
                            BackForwardCacheNotRestoredReason::CookieDisabled
                        }
                        "HTTPAuthRequired" => {
                            BackForwardCacheNotRestoredReason::HttpAuthRequired
                        }
                        "CookieFlushed" => {
                            BackForwardCacheNotRestoredReason::CookieFlushed
                        }
                        "BroadcastChannelOnMessage" => {
                            BackForwardCacheNotRestoredReason::BroadcastChannelOnMessage
                        }
                        "WebViewSettingsChanged" => {
                            BackForwardCacheNotRestoredReason::WebViewSettingsChanged
                        }
                        "WebViewJavaScriptObjectChanged" => {
                            BackForwardCacheNotRestoredReason::WebViewJavaScriptObjectChanged
                        }
                        "WebViewMessageListenerInjected" => {
                            BackForwardCacheNotRestoredReason::WebViewMessageListenerInjected
                        }
                        "WebViewSafeBrowsingAllowlistChanged" => {
                            BackForwardCacheNotRestoredReason::WebViewSafeBrowsingAllowlistChanged
                        }
                        "WebViewDocumentStartJavascriptChanged" => {
                            BackForwardCacheNotRestoredReason::WebViewDocumentStartJavascriptChanged
                        }
                        "WebSocket" => BackForwardCacheNotRestoredReason::WebSocket,
                        "WebTransport" => BackForwardCacheNotRestoredReason::WebTransport,
                        "WebRTC" => BackForwardCacheNotRestoredReason::WebRtc,
                        "MainResourceHasCacheControlNoStore" => {
                            BackForwardCacheNotRestoredReason::MainResourceHasCacheControlNoStore
                        }
                        "MainResourceHasCacheControlNoCache" => {
                            BackForwardCacheNotRestoredReason::MainResourceHasCacheControlNoCache
                        }
                        "SubresourceHasCacheControlNoStore" => {
                            BackForwardCacheNotRestoredReason::SubresourceHasCacheControlNoStore
                        }
                        "SubresourceHasCacheControlNoCache" => {
                            BackForwardCacheNotRestoredReason::SubresourceHasCacheControlNoCache
                        }
                        "ContainsPlugins" => {
                            BackForwardCacheNotRestoredReason::ContainsPlugins
                        }
                        "DocumentLoaded" => {
                            BackForwardCacheNotRestoredReason::DocumentLoaded
                        }
                        "OutstandingNetworkRequestOthers" => {
                            BackForwardCacheNotRestoredReason::OutstandingNetworkRequestOthers
                        }
                        "RequestedMIDIPermission" => {
                            BackForwardCacheNotRestoredReason::RequestedMidiPermission
                        }
                        "RequestedAudioCapturePermission" => {
                            BackForwardCacheNotRestoredReason::RequestedAudioCapturePermission
                        }
                        "RequestedVideoCapturePermission" => {
                            BackForwardCacheNotRestoredReason::RequestedVideoCapturePermission
                        }
                        "RequestedBackForwardCacheBlockedSensors" => {
                            BackForwardCacheNotRestoredReason::RequestedBackForwardCacheBlockedSensors
                        }
                        "RequestedBackgroundWorkPermission" => {
                            BackForwardCacheNotRestoredReason::RequestedBackgroundWorkPermission
                        }
                        "BroadcastChannel" => {
                            BackForwardCacheNotRestoredReason::BroadcastChannel
                        }
                        "WebXR" => BackForwardCacheNotRestoredReason::WebXr,
                        "SharedWorker" => BackForwardCacheNotRestoredReason::SharedWorker,
                        "SharedWorkerMessage" => {
                            BackForwardCacheNotRestoredReason::SharedWorkerMessage
                        }
                        "SharedWorkerWithNoActiveClient" => {
                            BackForwardCacheNotRestoredReason::SharedWorkerWithNoActiveClient
                        }
                        "WebLocks" => BackForwardCacheNotRestoredReason::WebLocks,
                        "WebLocksContention" => {
                            BackForwardCacheNotRestoredReason::WebLocksContention
                        }
                        "WebHID" => BackForwardCacheNotRestoredReason::WebHid,
                        "WebBluetooth" => BackForwardCacheNotRestoredReason::WebBluetooth,
                        "WebShare" => BackForwardCacheNotRestoredReason::WebShare,
                        "RequestedStorageAccessGrant" => {
                            BackForwardCacheNotRestoredReason::RequestedStorageAccessGrant
                        }
                        "WebNfc" => BackForwardCacheNotRestoredReason::WebNfc,
                        "OutstandingNetworkRequestFetch" => {
                            BackForwardCacheNotRestoredReason::OutstandingNetworkRequestFetch
                        }
                        "OutstandingNetworkRequestXHR" => {
                            BackForwardCacheNotRestoredReason::OutstandingNetworkRequestXhr
                        }
                        "AppBanner" => BackForwardCacheNotRestoredReason::AppBanner,
                        "Printing" => BackForwardCacheNotRestoredReason::Printing,
                        "WebDatabase" => BackForwardCacheNotRestoredReason::WebDatabase,
                        "PictureInPicture" => {
                            BackForwardCacheNotRestoredReason::PictureInPicture
                        }
                        "SpeechRecognizer" => {
                            BackForwardCacheNotRestoredReason::SpeechRecognizer
                        }
                        "IdleManager" => BackForwardCacheNotRestoredReason::IdleManager,
                        "PaymentManager" => {
                            BackForwardCacheNotRestoredReason::PaymentManager
                        }
                        "SpeechSynthesis" => {
                            BackForwardCacheNotRestoredReason::SpeechSynthesis
                        }
                        "KeyboardLock" => BackForwardCacheNotRestoredReason::KeyboardLock,
                        "WebOTPService" => {
                            BackForwardCacheNotRestoredReason::WebOtpService
                        }
                        "OutstandingNetworkRequestDirectSocket" => {
                            BackForwardCacheNotRestoredReason::OutstandingNetworkRequestDirectSocket
                        }
                        "InjectedJavascript" => {
                            BackForwardCacheNotRestoredReason::InjectedJavascript
                        }
                        "InjectedStyleSheet" => {
                            BackForwardCacheNotRestoredReason::InjectedStyleSheet
                        }
                        "KeepaliveRequest" => {
                            BackForwardCacheNotRestoredReason::KeepaliveRequest
                        }
                        "IndexedDBEvent" => {
                            BackForwardCacheNotRestoredReason::IndexedDbEvent
                        }
                        "Dummy" => BackForwardCacheNotRestoredReason::Dummy,
                        "JsNetworkRequestReceivedCacheControlNoStoreResource" => {
                            BackForwardCacheNotRestoredReason::JsNetworkRequestReceivedCacheControlNoStoreResource
                        }
                        "WebRTCUsedWithCCNS" => {
                            BackForwardCacheNotRestoredReason::WebRtcUsedWithCcns
                        }
                        "WebTransportUsedWithCCNS" => {
                            BackForwardCacheNotRestoredReason::WebTransportUsedWithCcns
                        }
                        "WebSocketUsedWithCCNS" => {
                            BackForwardCacheNotRestoredReason::WebSocketUsedWithCcns
                        }
                        "SmartCard" => BackForwardCacheNotRestoredReason::SmartCard,
                        "LiveMediaStreamTrack" => {
                            BackForwardCacheNotRestoredReason::LiveMediaStreamTrack
                        }
                        "UnloadHandler" => {
                            BackForwardCacheNotRestoredReason::UnloadHandler
                        }
                        "ParserAborted" => {
                            BackForwardCacheNotRestoredReason::ParserAborted
                        }
                        "ContentSecurityHandler" => {
                            BackForwardCacheNotRestoredReason::ContentSecurityHandler
                        }
                        "ContentWebAuthenticationAPI" => {
                            BackForwardCacheNotRestoredReason::ContentWebAuthenticationApi
                        }
                        "ContentFileChooser" => {
                            BackForwardCacheNotRestoredReason::ContentFileChooser
                        }
                        "ContentSerial" => {
                            BackForwardCacheNotRestoredReason::ContentSerial
                        }
                        "ContentFileSystemAccess" => {
                            BackForwardCacheNotRestoredReason::ContentFileSystemAccess
                        }
                        "ContentMediaDevicesDispatcherHost" => {
                            BackForwardCacheNotRestoredReason::ContentMediaDevicesDispatcherHost
                        }
                        "ContentWebBluetooth" => {
                            BackForwardCacheNotRestoredReason::ContentWebBluetooth
                        }
                        "ContentWebUSB" => {
                            BackForwardCacheNotRestoredReason::ContentWebUsb
                        }
                        "ContentMediaSessionService" => {
                            BackForwardCacheNotRestoredReason::ContentMediaSessionService
                        }
                        "ContentScreenReader" => {
                            BackForwardCacheNotRestoredReason::ContentScreenReader
                        }
                        "ContentDiscarded" => {
                            BackForwardCacheNotRestoredReason::ContentDiscarded
                        }
                        "EmbedderPopupBlockerTabHelper" => {
                            BackForwardCacheNotRestoredReason::EmbedderPopupBlockerTabHelper
                        }
                        "EmbedderSafeBrowsingTriggeredPopupBlocker" => {
                            BackForwardCacheNotRestoredReason::EmbedderSafeBrowsingTriggeredPopupBlocker
                        }
                        "EmbedderSafeBrowsingThreatDetails" => {
                            BackForwardCacheNotRestoredReason::EmbedderSafeBrowsingThreatDetails
                        }
                        "EmbedderAppBannerManager" => {
                            BackForwardCacheNotRestoredReason::EmbedderAppBannerManager
                        }
                        "EmbedderDomDistillerViewerSource" => {
                            BackForwardCacheNotRestoredReason::EmbedderDomDistillerViewerSource
                        }
                        "EmbedderDomDistillerSelfDeletingRequestDelegate" => {
                            BackForwardCacheNotRestoredReason::EmbedderDomDistillerSelfDeletingRequestDelegate
                        }
                        "EmbedderOomInterventionTabHelper" => {
                            BackForwardCacheNotRestoredReason::EmbedderOomInterventionTabHelper
                        }
                        "EmbedderOfflinePage" => {
                            BackForwardCacheNotRestoredReason::EmbedderOfflinePage
                        }
                        "EmbedderChromePasswordManagerClientBindCredentialManager" => {
                            BackForwardCacheNotRestoredReason::EmbedderChromePasswordManagerClientBindCredentialManager
                        }
                        "EmbedderPermissionRequestManager" => {
                            BackForwardCacheNotRestoredReason::EmbedderPermissionRequestManager
                        }
                        "EmbedderModalDialog" => {
                            BackForwardCacheNotRestoredReason::EmbedderModalDialog
                        }
                        "EmbedderExtensions" => {
                            BackForwardCacheNotRestoredReason::EmbedderExtensions
                        }
                        "EmbedderExtensionMessaging" => {
                            BackForwardCacheNotRestoredReason::EmbedderExtensionMessaging
                        }
                        "EmbedderExtensionMessagingForOpenPort" => {
                            BackForwardCacheNotRestoredReason::EmbedderExtensionMessagingForOpenPort
                        }
                        "EmbedderExtensionSentMessageToCachedFrame" => {
                            BackForwardCacheNotRestoredReason::EmbedderExtensionSentMessageToCachedFrame
                        }
                        "EmbedderExtensionFrame" => {
                            BackForwardCacheNotRestoredReason::EmbedderExtensionFrame
                        }
                        "EmbedderPrivilegedWebContents" => {
                            BackForwardCacheNotRestoredReason::EmbedderPrivilegedWebContents
                        }
                        "RequestedByWebViewClient" => {
                            BackForwardCacheNotRestoredReason::RequestedByWebViewClient
                        }
                        "PostMessageByWebViewClient" => {
                            BackForwardCacheNotRestoredReason::PostMessageByWebViewClient
                        }
                        "CacheControlNoStoreDeviceBoundSessionTerminated" => {
                            BackForwardCacheNotRestoredReason::CacheControlNoStoreDeviceBoundSessionTerminated
                        }
                        "CacheLimitPrunedOnModerateMemoryPressure" => {
                            BackForwardCacheNotRestoredReason::CacheLimitPrunedOnModerateMemoryPressure
                        }
                        "CacheLimitPrunedOnCriticalMemoryPressure" => {
                            BackForwardCacheNotRestoredReason::CacheLimitPrunedOnCriticalMemoryPressure
                        }
                        _ => BackForwardCacheNotRestoredReason::Unknown(v.to_owned()),
                    },
                )
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(BackForwardCacheNotRestoredReason::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.type.BackForwardCacheNotRestoredReasonType。
///
///协议原文：
///
///<pre>Types of not restored reasons for back-forward cache.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum BackForwardCacheNotRestoredReasonType {
    ///协议值 "SupportPending"。
    SupportPending,
    ///协议值 "PageSupportNeeded"。
    PageSupportNeeded,
    ///协议值 "Circumstantial"。
    Circumstantial,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl BackForwardCacheNotRestoredReasonType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::types::BackForwardCacheNotRestoredReasonType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::SupportPending => "SupportPending",
            Self::PageSupportNeeded => "PageSupportNeeded",
            Self::Circumstantial => "Circumstantial",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for BackForwardCacheNotRestoredReasonType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for BackForwardCacheNotRestoredReasonType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "SupportPending" => BackForwardCacheNotRestoredReasonType::SupportPending,
            "PageSupportNeeded" => BackForwardCacheNotRestoredReasonType::PageSupportNeeded,
            "Circumstantial" => BackForwardCacheNotRestoredReasonType::Circumstantial,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for BackForwardCacheNotRestoredReasonType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BackForwardCacheNotRestoredReasonType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BackForwardCacheNotRestoredReasonType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "SupportPending" => BackForwardCacheNotRestoredReasonType::SupportPending,
                    "PageSupportNeeded" => BackForwardCacheNotRestoredReasonType::PageSupportNeeded,
                    "Circumstantial" => BackForwardCacheNotRestoredReasonType::Circumstantial,
                    _ => BackForwardCacheNotRestoredReasonType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(BackForwardCacheNotRestoredReasonType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.type.ClientNavigationDisposition。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ClientNavigationDisposition {
    ///协议值 "currentTab"。
    CurrentTab,
    ///协议值 "newTab"。
    NewTab,
    ///协议值 "newWindow"。
    NewWindow,
    ///协议值 "download"。
    Download,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ClientNavigationDisposition {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::types::ClientNavigationDisposition::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::CurrentTab => "currentTab",
            Self::NewTab => "newTab",
            Self::NewWindow => "newWindow",
            Self::Download => "download",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ClientNavigationDisposition {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ClientNavigationDisposition {
    fn from(value: String) -> Self {
        match value.as_str() {
            "currentTab" => ClientNavigationDisposition::CurrentTab,
            "newTab" => ClientNavigationDisposition::NewTab,
            "newWindow" => ClientNavigationDisposition::NewWindow,
            "download" => ClientNavigationDisposition::Download,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ClientNavigationDisposition {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ClientNavigationDisposition {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClientNavigationDisposition;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "currentTab" => ClientNavigationDisposition::CurrentTab,
                    "newTab" => ClientNavigationDisposition::NewTab,
                    "newWindow" => ClientNavigationDisposition::NewWindow,
                    "download" => ClientNavigationDisposition::Download,
                    _ => ClientNavigationDisposition::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ClientNavigationDisposition::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.type.ClientNavigationReason。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ClientNavigationReason {
    ///协议值 "anchorClick"。
    AnchorClick,
    ///协议值 "formSubmissionGet"。
    FormSubmissionGet,
    ///协议值 "formSubmissionPost"。
    FormSubmissionPost,
    ///协议值 "httpHeaderRefresh"。
    HttpHeaderRefresh,
    ///协议值 "initialFrameNavigation"。
    InitialFrameNavigation,
    ///协议值 "metaTagRefresh"。
    MetaTagRefresh,
    ///协议值 "other"。
    Other,
    ///协议值 "pageBlockInterstitial"。
    PageBlockInterstitial,
    ///协议值 "reload"。
    Reload,
    ///协议值 "scriptInitiated"。
    ScriptInitiated,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ClientNavigationReason {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::types::ClientNavigationReason::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::AnchorClick => "anchorClick",
            Self::FormSubmissionGet => "formSubmissionGet",
            Self::FormSubmissionPost => "formSubmissionPost",
            Self::HttpHeaderRefresh => "httpHeaderRefresh",
            Self::InitialFrameNavigation => "initialFrameNavigation",
            Self::MetaTagRefresh => "metaTagRefresh",
            Self::Other => "other",
            Self::PageBlockInterstitial => "pageBlockInterstitial",
            Self::Reload => "reload",
            Self::ScriptInitiated => "scriptInitiated",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ClientNavigationReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ClientNavigationReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "anchorClick" => ClientNavigationReason::AnchorClick,
            "formSubmissionGet" => ClientNavigationReason::FormSubmissionGet,
            "formSubmissionPost" => ClientNavigationReason::FormSubmissionPost,
            "httpHeaderRefresh" => ClientNavigationReason::HttpHeaderRefresh,
            "initialFrameNavigation" => ClientNavigationReason::InitialFrameNavigation,
            "metaTagRefresh" => ClientNavigationReason::MetaTagRefresh,
            "other" => ClientNavigationReason::Other,
            "pageBlockInterstitial" => ClientNavigationReason::PageBlockInterstitial,
            "reload" => ClientNavigationReason::Reload,
            "scriptInitiated" => ClientNavigationReason::ScriptInitiated,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ClientNavigationReason {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ClientNavigationReason {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClientNavigationReason;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "anchorClick" => ClientNavigationReason::AnchorClick,
                    "formSubmissionGet" => ClientNavigationReason::FormSubmissionGet,
                    "formSubmissionPost" => ClientNavigationReason::FormSubmissionPost,
                    "httpHeaderRefresh" => ClientNavigationReason::HttpHeaderRefresh,
                    "initialFrameNavigation" => ClientNavigationReason::InitialFrameNavigation,
                    "metaTagRefresh" => ClientNavigationReason::MetaTagRefresh,
                    "other" => ClientNavigationReason::Other,
                    "pageBlockInterstitial" => ClientNavigationReason::PageBlockInterstitial,
                    "reload" => ClientNavigationReason::Reload,
                    "scriptInitiated" => ClientNavigationReason::ScriptInitiated,
                    _ => ClientNavigationReason::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ClientNavigationReason::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.CompilationCacheParams。
///
///协议原文：
///
///<pre>Per-script compilation cache parameters for `Page.produceCompilationCache`</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CompilationCacheParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.CompilationCacheParams/properties/url。
    ///
    ///协议原文：
    ///
    ///<pre>The URL of the script to produce a compilation cache entry for.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.CompilationCacheParams/properties/eager。
    ///
    ///协议原文：
    ///
    ///<pre>A hint to the backend whether eager compilation is recommended.
    ///(the actual compilation mode used is upon backend discretion).</pre>
    ///
    ///
    #[serde(rename = "eager")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub eager: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for CompilationCacheParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CompilationCacheParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.CompilationCacheParams/properties/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The URL of the script to produce a compilation cache entry for.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.CompilationCacheParams/properties/eager。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A hint to the backend whether eager compilation is recommended.
                    ///(the actual compilation mode used is upon backend discretion).</pre>
                    ///
                    ///
                    #[serde(rename = "eager")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub eager: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CompilationCacheParams {
                    url: fields.url,
                    eager: fields.eager,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.type.CrossOriginIsolatedContextType。
///
///协议原文：
///
///<pre>Indicates whether the frame is cross-origin isolated and why it is the case.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CrossOriginIsolatedContextType {
    ///协议值 "Isolated"。
    Isolated,
    ///协议值 "NotIsolated"。
    NotIsolated,
    ///协议值 "NotIsolatedFeatureDisabled"。
    NotIsolatedFeatureDisabled,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl CrossOriginIsolatedContextType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::types::CrossOriginIsolatedContextType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Isolated => "Isolated",
            Self::NotIsolated => "NotIsolated",
            Self::NotIsolatedFeatureDisabled => "NotIsolatedFeatureDisabled",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for CrossOriginIsolatedContextType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for CrossOriginIsolatedContextType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Isolated" => CrossOriginIsolatedContextType::Isolated,
            "NotIsolated" => CrossOriginIsolatedContextType::NotIsolated,
            "NotIsolatedFeatureDisabled" => {
                CrossOriginIsolatedContextType::NotIsolatedFeatureDisabled
            }
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for CrossOriginIsolatedContextType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CrossOriginIsolatedContextType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CrossOriginIsolatedContextType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Isolated" => CrossOriginIsolatedContextType::Isolated,
                    "NotIsolated" => CrossOriginIsolatedContextType::NotIsolated,
                    "NotIsolatedFeatureDisabled" => {
                        CrossOriginIsolatedContextType::NotIsolatedFeatureDisabled
                    }
                    _ => CrossOriginIsolatedContextType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(CrossOriginIsolatedContextType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.type.DialogType。
///
///协议原文：
///
///<pre>Javascript dialog type.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DialogType {
    ///协议值 "alert"。
    Alert,
    ///协议值 "confirm"。
    Confirm,
    ///协议值 "prompt"。
    Prompt,
    ///协议值 "beforeunload"。
    Beforeunload,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl DialogType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::types::DialogType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Alert => "alert",
            Self::Confirm => "confirm",
            Self::Prompt => "prompt",
            Self::Beforeunload => "beforeunload",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for DialogType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for DialogType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "alert" => DialogType::Alert,
            "confirm" => DialogType::Confirm,
            "prompt" => DialogType::Prompt,
            "beforeunload" => DialogType::Beforeunload,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for DialogType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DialogType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DialogType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "alert" => DialogType::Alert,
                    "confirm" => DialogType::Confirm,
                    "prompt" => DialogType::Prompt,
                    "beforeunload" => DialogType::Beforeunload,
                    _ => DialogType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(DialogType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.FileFilter。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FileFilter {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FileFilter/properties/name。
    #[serde(rename = "name")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FileFilter/properties/accepts。
    #[serde(rename = "accepts")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub accepts: Option<Vec<String>>,
}
impl<'de> serde::Deserialize<'de> for FileFilter {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FileFilter;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FileFilter/properties/name。
                    #[serde(rename = "name")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub name: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FileFilter/properties/accepts。
                    #[serde(rename = "accepts")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub accepts: Option<Vec<String>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FileFilter {
                    name: fields.name,
                    accepts: fields.accepts,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.FileHandler。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FileHandler {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.FileHandler/properties/action。
    #[serde(rename = "action")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub action: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.FileHandler/properties/name。
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FileHandler/properties/accepts。
    ///
    ///协议原文：
    ///
    ///<pre>Mimic a map, name is the key, accepts is the value.</pre>
    ///
    ///
    #[serde(rename = "accepts")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub accepts: Option<Vec<crate::page::types::FileFilter>>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.FileHandler/properties/launchType。
    ///
    ///协议原文：
    ///
    ///<pre>Won't repeat the enums, using string for easy comparison. Same as the
    ///other enums below.</pre>
    ///
    ///
    #[serde(rename = "launchType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub launch_type: String,
}
impl<'de> serde::Deserialize<'de> for FileHandler {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FileHandler;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.FileHandler/properties/action。
                    #[serde(rename = "action")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub action: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.FileHandler/properties/name。
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FileHandler/properties/accepts。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Mimic a map, name is the key, accepts is the value.</pre>
                    ///
                    ///
                    #[serde(rename = "accepts")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub accepts: Option<Vec<crate::page::types::FileFilter>>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.FileHandler/properties/launchType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Won't repeat the enums, using string for easy comparison. Same as the
                    ///other enums below.</pre>
                    ///
                    ///
                    #[serde(rename = "launchType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub launch_type: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FileHandler {
                    action: fields.action,
                    name: fields.name,
                    accepts: fields.accepts,
                    launch_type: fields.launch_type,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.FontFamilies。
///
///协议原文：
///
///<pre>Generic font families collection.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FontFamilies {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FontFamilies/properties/standard。
    ///
    ///协议原文：
    ///
    ///<pre>The standard font-family.</pre>
    ///
    ///
    #[serde(rename = "standard")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub standard: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FontFamilies/properties/fixed。
    ///
    ///协议原文：
    ///
    ///<pre>The fixed font-family.</pre>
    ///
    ///
    #[serde(rename = "fixed")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub fixed: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FontFamilies/properties/serif。
    ///
    ///协议原文：
    ///
    ///<pre>The serif font-family.</pre>
    ///
    ///
    #[serde(rename = "serif")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub serif: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FontFamilies/properties/sansSerif。
    ///
    ///协议原文：
    ///
    ///<pre>The sansSerif font-family.</pre>
    ///
    ///
    #[serde(rename = "sansSerif")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub sans_serif: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FontFamilies/properties/cursive。
    ///
    ///协议原文：
    ///
    ///<pre>The cursive font-family.</pre>
    ///
    ///
    #[serde(rename = "cursive")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub cursive: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FontFamilies/properties/fantasy。
    ///
    ///协议原文：
    ///
    ///<pre>The fantasy font-family.</pre>
    ///
    ///
    #[serde(rename = "fantasy")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub fantasy: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FontFamilies/properties/math。
    ///
    ///协议原文：
    ///
    ///<pre>The math font-family.</pre>
    ///
    ///
    #[serde(rename = "math")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub math: Option<String>,
}
impl<'de> serde::Deserialize<'de> for FontFamilies {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FontFamilies;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FontFamilies/properties/standard。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The standard font-family.</pre>
                    ///
                    ///
                    #[serde(rename = "standard")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub standard: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FontFamilies/properties/fixed。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The fixed font-family.</pre>
                    ///
                    ///
                    #[serde(rename = "fixed")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub fixed: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FontFamilies/properties/serif。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The serif font-family.</pre>
                    ///
                    ///
                    #[serde(rename = "serif")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub serif: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FontFamilies/properties/sansSerif。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The sansSerif font-family.</pre>
                    ///
                    ///
                    #[serde(rename = "sansSerif")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub sans_serif: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FontFamilies/properties/cursive。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The cursive font-family.</pre>
                    ///
                    ///
                    #[serde(rename = "cursive")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub cursive: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FontFamilies/properties/fantasy。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The fantasy font-family.</pre>
                    ///
                    ///
                    #[serde(rename = "fantasy")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub fantasy: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FontFamilies/properties/math。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The math font-family.</pre>
                    ///
                    ///
                    #[serde(rename = "math")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub math: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FontFamilies {
                    standard: fields.standard,
                    fixed: fields.fixed,
                    serif: fields.serif,
                    sans_serif: fields.sans_serif,
                    cursive: fields.cursive,
                    fantasy: fields.fantasy,
                    math: fields.math,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.FontSizes。
///
///协议原文：
///
///<pre>Default font sizes.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FontSizes {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FontSizes/properties/standard。
    ///
    ///协议原文：
    ///
    ///<pre>Default standard font size.</pre>
    ///
    ///
    #[serde(rename = "standard")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub standard: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FontSizes/properties/fixed。
    ///
    ///协议原文：
    ///
    ///<pre>Default fixed font size.</pre>
    ///
    ///
    #[serde(rename = "fixed")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub fixed: Option<i64>,
}
impl<'de> serde::Deserialize<'de> for FontSizes {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FontSizes;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FontSizes/properties/standard。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Default standard font size.</pre>
                    ///
                    ///
                    #[serde(rename = "standard")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub standard: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FontSizes/properties/fixed。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Default fixed font size.</pre>
                    ///
                    ///
                    #[serde(rename = "fixed")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub fixed: Option<i64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FontSizes {
                    standard: fields.standard,
                    fixed: fields.fixed,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.Frame。
///
///协议原文：
///
///<pre>Information about the Frame on the page.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct Frame {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Frame/properties/id。
    ///
    ///协议原文：
    ///
    ///<pre>Frame unique identifier.</pre>
    ///
    ///
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: crate::page::types::FrameId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.Frame/properties/parentId。
    ///
    ///协议原文：
    ///
    ///<pre>Parent frame identifier.</pre>
    ///
    ///
    #[serde(rename = "parentId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_id: Option<crate::page::types::FrameId>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Frame/properties/loaderId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the loader associated with this frame.</pre>
    ///
    ///
    #[serde(rename = "loaderId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub loader_id: crate::network::types::LoaderId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.Frame/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Frame's name as specified in the tag.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Frame/properties/url。
    ///
    ///协议原文：
    ///
    ///<pre>Frame document's URL without fragment.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.Frame/properties/urlFragment。
    ///
    ///协议原文：
    ///
    ///<pre>Frame document's URL fragment including the '#'.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "urlFragment")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub url_fragment: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Frame/properties/domainAndRegistry。
    ///
    ///协议原文：
    ///
    ///<pre>Frame document's registered domain, taking the public suffixes list into account.
    ///Extracted from the Frame's url.
    ///Example URLs: http://www.google.com/file.html -&gt; "google.com"
    ///              http://a.b.co.uk/file.html      -&gt; "b.co.uk"</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "domainAndRegistry")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub domain_and_registry: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Frame/properties/securityOrigin。
    ///
    ///协议原文：
    ///
    ///<pre>Frame document's security origin.</pre>
    ///
    ///
    #[serde(rename = "securityOrigin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub security_origin: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.Frame/properties/securityOriginDetails。
    ///
    ///协议原文：
    ///
    ///<pre>Additional details about the frame document's security origin.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "securityOriginDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub security_origin_details: Option<crate::page::types::SecurityOriginDetails>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Frame/properties/mimeType。
    ///
    ///协议原文：
    ///
    ///<pre>Frame document's mimeType as determined by the browser.</pre>
    ///
    ///
    #[serde(rename = "mimeType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub mime_type: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.Frame/properties/unreachableUrl。
    ///
    ///协议原文：
    ///
    ///<pre>If the frame failed to load, this contains the URL that could not be loaded. Note that unlike url above, this URL may contain a fragment.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "unreachableUrl")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub unreachable_url: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.Frame/properties/adFrameStatus。
    ///
    ///协议原文：
    ///
    ///<pre>Indicates whether this frame was tagged as an ad and why.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "adFrameStatus")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub ad_frame_status: Option<crate::page::types::AdFrameStatus>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Frame/properties/secureContextType。
    ///
    ///协议原文：
    ///
    ///<pre>Indicates whether the main document is a secure context and explains why that is the case.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "secureContextType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub secure_context_type: crate::page::types::SecureContextType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Frame/properties/crossOriginIsolatedContextType。
    ///
    ///协议原文：
    ///
    ///<pre>Indicates whether this is a cross origin isolated context.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "crossOriginIsolatedContextType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cross_origin_isolated_context_type: crate::page::types::CrossOriginIsolatedContextType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Frame/properties/gatedAPIFeatures。
    ///
    ///协议原文：
    ///
    ///<pre>Indicated which gated APIs / features are available.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "gatedAPIFeatures")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub gated_api_features: Vec<crate::page::types::GatedApiFeatures>,
}
impl<'de> serde::Deserialize<'de> for Frame {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Frame;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Frame/properties/id。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame unique identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: crate::page::types::FrameId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.Frame/properties/parentId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Parent frame identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "parentId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub parent_id: Option<crate::page::types::FrameId>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Frame/properties/loaderId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the loader associated with this frame.</pre>
                    ///
                    ///
                    #[serde(rename = "loaderId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub loader_id: crate::network::types::LoaderId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.Frame/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame's name as specified in the tag.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub name: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Frame/properties/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame document's URL without fragment.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.Frame/properties/urlFragment。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame document's URL fragment including the '#'.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "urlFragment")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub url_fragment: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Frame/properties/domainAndRegistry。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame document's registered domain, taking the public suffixes list into account.
                    ///Extracted from the Frame's url.
                    ///Example URLs: http://www.google.com/file.html -&gt; "google.com"
                    ///              http://a.b.co.uk/file.html      -&gt; "b.co.uk"</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "domainAndRegistry")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub domain_and_registry: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Frame/properties/securityOrigin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame document's security origin.</pre>
                    ///
                    ///
                    #[serde(rename = "securityOrigin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub security_origin: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.Frame/properties/securityOriginDetails。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Additional details about the frame document's security origin.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "securityOriginDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub security_origin_details: Option<crate::page::types::SecurityOriginDetails>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Frame/properties/mimeType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame document's mimeType as determined by the browser.</pre>
                    ///
                    ///
                    #[serde(rename = "mimeType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub mime_type: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.Frame/properties/unreachableUrl。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If the frame failed to load, this contains the URL that could not be loaded. Note that unlike url above, this URL may contain a fragment.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "unreachableUrl")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub unreachable_url: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.Frame/properties/adFrameStatus。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Indicates whether this frame was tagged as an ad and why.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "adFrameStatus")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub ad_frame_status: Option<crate::page::types::AdFrameStatus>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Frame/properties/secureContextType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Indicates whether the main document is a secure context and explains why that is the case.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "secureContextType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub secure_context_type: crate::page::types::SecureContextType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Frame/properties/crossOriginIsolatedContextType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Indicates whether this is a cross origin isolated context.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "crossOriginIsolatedContextType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cross_origin_isolated_context_type:
                        crate::page::types::CrossOriginIsolatedContextType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Frame/properties/gatedAPIFeatures。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Indicated which gated APIs / features are available.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "gatedAPIFeatures")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub gated_api_features: Vec<crate::page::types::GatedApiFeatures>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Frame {
                    id: fields.id,
                    parent_id: fields.parent_id,
                    loader_id: fields.loader_id,
                    name: fields.name,
                    url: fields.url,
                    url_fragment: fields.url_fragment,
                    domain_and_registry: fields.domain_and_registry,
                    security_origin: fields.security_origin,
                    security_origin_details: fields.security_origin_details,
                    mime_type: fields.mime_type,
                    unreachable_url: fields.unreachable_url,
                    ad_frame_status: fields.ad_frame_status,
                    secure_context_type: fields.secure_context_type,
                    cross_origin_isolated_context_type: fields.cross_origin_isolated_context_type,
                    gated_api_features: fields.gated_api_features,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Page.type.FrameId。
///
///协议原文：
///
///<pre>Unique frame identifier.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct FrameId(String);
impl FrameId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::page::types::FrameId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::page::types::FrameId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::page::types::FrameId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::page::types::FrameId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::page::types::FrameId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.FrameResource。
///
///协议原文：
///
///<pre>Information about the Resource on the page.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FrameResource {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.FrameResource/properties/url。
    ///
    ///协议原文：
    ///
    ///<pre>Resource URL.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.FrameResource/properties/type。
    ///
    ///协议原文：
    ///
    ///<pre>Type of this resource.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: crate::network::types::ResourceType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.FrameResource/properties/mimeType。
    ///
    ///协议原文：
    ///
    ///<pre>Resource mimeType as determined by the browser.</pre>
    ///
    ///
    #[serde(rename = "mimeType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub mime_type: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FrameResource/properties/lastModified。
    ///
    ///协议原文：
    ///
    ///<pre>last-modified timestamp as reported by server.</pre>
    ///
    ///
    #[serde(rename = "lastModified")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_modified: Option<crate::network::types::TimeSinceEpoch>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FrameResource/properties/contentSize。
    ///
    ///协议原文：
    ///
    ///<pre>Resource content size.</pre>
    ///
    ///
    #[serde(rename = "contentSize")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub content_size: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FrameResource/properties/failed。
    ///
    ///协议原文：
    ///
    ///<pre>True if the resource failed to load.</pre>
    ///
    ///
    #[serde(rename = "failed")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub failed: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FrameResource/properties/canceled。
    ///
    ///协议原文：
    ///
    ///<pre>True if the resource was canceled during loading.</pre>
    ///
    ///
    #[serde(rename = "canceled")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub canceled: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for FrameResource {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FrameResource;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.FrameResource/properties/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Resource URL.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.FrameResource/properties/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Type of this resource.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: crate::network::types::ResourceType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.FrameResource/properties/mimeType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Resource mimeType as determined by the browser.</pre>
                    ///
                    ///
                    #[serde(rename = "mimeType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub mime_type: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FrameResource/properties/lastModified。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>last-modified timestamp as reported by server.</pre>
                    ///
                    ///
                    #[serde(rename = "lastModified")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub last_modified: Option<crate::network::types::TimeSinceEpoch>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FrameResource/properties/contentSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Resource content size.</pre>
                    ///
                    ///
                    #[serde(rename = "contentSize")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub content_size: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FrameResource/properties/failed。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if the resource failed to load.</pre>
                    ///
                    ///
                    #[serde(rename = "failed")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub failed: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FrameResource/properties/canceled。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if the resource was canceled during loading.</pre>
                    ///
                    ///
                    #[serde(rename = "canceled")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub canceled: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FrameResource {
                    url: fields.url,
                    r#type: fields.r#type,
                    mime_type: fields.mime_type,
                    last_modified: fields.last_modified,
                    content_size: fields.content_size,
                    failed: fields.failed,
                    canceled: fields.canceled,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.FrameResourceTree。
///
///协议原文：
///
///<pre>Information about the Frame hierarchy along with their cached resources.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FrameResourceTree {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.FrameResourceTree/properties/frame。
    ///
    ///协议原文：
    ///
    ///<pre>Frame information for this tree item.</pre>
    ///
    ///
    #[serde(rename = "frame")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame: crate::page::types::Frame,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FrameResourceTree/properties/childFrames。
    ///
    ///协议原文：
    ///
    ///<pre>Child frames.</pre>
    ///
    ///
    #[serde(rename = "childFrames")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub child_frames: Option<Vec<crate::page::types::FrameResourceTree>>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.FrameResourceTree/properties/resources。
    ///
    ///协议原文：
    ///
    ///<pre>Information about frame resources.</pre>
    ///
    ///
    #[serde(rename = "resources")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub resources: Vec<crate::page::types::FrameResource>,
}
impl<'de> serde::Deserialize<'de> for FrameResourceTree {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FrameResourceTree;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.FrameResourceTree/properties/frame。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame information for this tree item.</pre>
                    ///
                    ///
                    #[serde(rename = "frame")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame: crate::page::types::Frame,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FrameResourceTree/properties/childFrames。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Child frames.</pre>
                    ///
                    ///
                    #[serde(rename = "childFrames")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub child_frames: Option<Vec<crate::page::types::FrameResourceTree>>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.FrameResourceTree/properties/resources。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Information about frame resources.</pre>
                    ///
                    ///
                    #[serde(rename = "resources")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub resources: Vec<crate::page::types::FrameResource>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FrameResourceTree {
                    frame: fields.frame,
                    child_frames: fields.child_frames,
                    resources: fields.resources,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.FrameTree。
///
///协议原文：
///
///<pre>Information about the Frame hierarchy.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct FrameTree {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.FrameTree/properties/frame。
    ///
    ///协议原文：
    ///
    ///<pre>Frame information for this tree item.</pre>
    ///
    ///
    #[serde(rename = "frame")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame: crate::page::types::Frame,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FrameTree/properties/childFrames。
    ///
    ///协议原文：
    ///
    ///<pre>Child frames.</pre>
    ///
    ///
    #[serde(rename = "childFrames")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub child_frames: Option<Vec<crate::page::types::FrameTree>>,
}
impl<'de> serde::Deserialize<'de> for FrameTree {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FrameTree;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.FrameTree/properties/frame。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame information for this tree item.</pre>
                    ///
                    ///
                    #[serde(rename = "frame")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame: crate::page::types::Frame,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.FrameTree/properties/childFrames。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Child frames.</pre>
                    ///
                    ///
                    #[serde(rename = "childFrames")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub child_frames: Option<Vec<crate::page::types::FrameTree>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FrameTree {
                    frame: fields.frame,
                    child_frames: fields.child_frames,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.type.GatedAPIFeatures。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum GatedApiFeatures {
    ///协议值 "SharedArrayBuffers"。
    SharedArrayBuffers,
    ///协议值 "SharedArrayBuffersTransferAllowed"。
    SharedArrayBuffersTransferAllowed,
    ///协议值 "PerformanceMeasureMemory"。
    PerformanceMeasureMemory,
    ///协议值 "PerformanceProfile"。
    PerformanceProfile,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl GatedApiFeatures {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::types::GatedApiFeatures::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::SharedArrayBuffers => "SharedArrayBuffers",
            Self::SharedArrayBuffersTransferAllowed => "SharedArrayBuffersTransferAllowed",
            Self::PerformanceMeasureMemory => "PerformanceMeasureMemory",
            Self::PerformanceProfile => "PerformanceProfile",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for GatedApiFeatures {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for GatedApiFeatures {
    fn from(value: String) -> Self {
        match value.as_str() {
            "SharedArrayBuffers" => GatedApiFeatures::SharedArrayBuffers,
            "SharedArrayBuffersTransferAllowed" => {
                GatedApiFeatures::SharedArrayBuffersTransferAllowed
            }
            "PerformanceMeasureMemory" => GatedApiFeatures::PerformanceMeasureMemory,
            "PerformanceProfile" => GatedApiFeatures::PerformanceProfile,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for GatedApiFeatures {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GatedApiFeatures {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GatedApiFeatures;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "SharedArrayBuffers" => GatedApiFeatures::SharedArrayBuffers,
                    "SharedArrayBuffersTransferAllowed" => {
                        GatedApiFeatures::SharedArrayBuffersTransferAllowed
                    }
                    "PerformanceMeasureMemory" => GatedApiFeatures::PerformanceMeasureMemory,
                    "PerformanceProfile" => GatedApiFeatures::PerformanceProfile,
                    _ => GatedApiFeatures::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(GatedApiFeatures::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.ImageResource。
///
///协议原文：
///
///<pre>The image definition used in both icon and screenshot.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ImageResource {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ImageResource/properties/url。
    ///
    ///协议原文：
    ///
    ///<pre>The src field in the definition, but changing to url in favor of
    ///consistency.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.ImageResource/properties/sizes。
    #[serde(rename = "sizes")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub sizes: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.ImageResource/properties/type。
    #[serde(rename = "type")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<String>,
}
impl<'de> serde::Deserialize<'de> for ImageResource {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImageResource;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ImageResource/properties/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The src field in the definition, but changing to url in favor of
                    ///consistency.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.ImageResource/properties/sizes。
                    #[serde(rename = "sizes")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub sizes: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.ImageResource/properties/type。
                    #[serde(rename = "type")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub r#type: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ImageResource {
                    url: fields.url,
                    sizes: fields.sizes,
                    r#type: fields.r#type,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.InstallabilityError。
///
///协议原文：
///
///<pre>The installability error</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct InstallabilityError {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.InstallabilityError/properties/errorId。
    ///
    ///协议原文：
    ///
    ///<pre>The error id (e.g. 'manifest-missing-suitable-icon').</pre>
    ///
    ///
    #[serde(rename = "errorId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub error_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.InstallabilityError/properties/errorArguments。
    ///
    ///协议原文：
    ///
    ///<pre>The list of error arguments (e.g. {name:'minimum-icon-size-in-pixels', value:'64'}).</pre>
    ///
    ///
    #[serde(rename = "errorArguments")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub error_arguments: Vec<crate::page::types::InstallabilityErrorArgument>,
}
impl<'de> serde::Deserialize<'de> for InstallabilityError {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InstallabilityError;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.InstallabilityError/properties/errorId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The error id (e.g. 'manifest-missing-suitable-icon').</pre>
                    ///
                    ///
                    #[serde(rename = "errorId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub error_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.InstallabilityError/properties/errorArguments。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The list of error arguments (e.g. {name:'minimum-icon-size-in-pixels', value:'64'}).</pre>
                    ///
                    ///
                    #[serde(rename = "errorArguments")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub error_arguments: Vec<crate::page::types::InstallabilityErrorArgument>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(InstallabilityError {
                    error_id: fields.error_id,
                    error_arguments: fields.error_arguments,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.InstallabilityErrorArgument。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct InstallabilityErrorArgument {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.InstallabilityErrorArgument/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Argument name (e.g. name:'minimum-icon-size-in-pixels').</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.InstallabilityErrorArgument/properties/value。
    ///
    ///协议原文：
    ///
    ///<pre>Argument value (e.g. value:'64').</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: String,
}
impl<'de> serde::Deserialize<'de> for InstallabilityErrorArgument {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InstallabilityErrorArgument;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.InstallabilityErrorArgument/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Argument name (e.g. name:'minimum-icon-size-in-pixels').</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.InstallabilityErrorArgument/properties/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Argument value (e.g. value:'64').</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(InstallabilityErrorArgument {
                    name: fields.name,
                    value: fields.value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.LaunchHandler。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct LaunchHandler {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.LaunchHandler/properties/clientMode。
    #[serde(rename = "clientMode")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub client_mode: String,
}
impl<'de> serde::Deserialize<'de> for LaunchHandler {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LaunchHandler;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.LaunchHandler/properties/clientMode。
                    #[serde(rename = "clientMode")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub client_mode: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LaunchHandler {
                    client_mode: fields.client_mode,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.LayoutViewport。
///
///协议原文：
///
///<pre>Layout viewport position and dimensions.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct LayoutViewport {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.LayoutViewport/properties/pageX。
    ///
    ///协议原文：
    ///
    ///<pre>Horizontal offset relative to the document (CSS pixels).</pre>
    ///
    ///
    #[serde(rename = "pageX")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub page_x: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.LayoutViewport/properties/pageY。
    ///
    ///协议原文：
    ///
    ///<pre>Vertical offset relative to the document (CSS pixels).</pre>
    ///
    ///
    #[serde(rename = "pageY")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub page_y: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.LayoutViewport/properties/clientWidth。
    ///
    ///协议原文：
    ///
    ///<pre>Width (CSS pixels), excludes scrollbar if present.</pre>
    ///
    ///
    #[serde(rename = "clientWidth")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub client_width: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.LayoutViewport/properties/clientHeight。
    ///
    ///协议原文：
    ///
    ///<pre>Height (CSS pixels), excludes scrollbar if present.</pre>
    ///
    ///
    #[serde(rename = "clientHeight")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub client_height: i64,
}
impl<'de> serde::Deserialize<'de> for LayoutViewport {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LayoutViewport;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.LayoutViewport/properties/pageX。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Horizontal offset relative to the document (CSS pixels).</pre>
                    ///
                    ///
                    #[serde(rename = "pageX")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub page_x: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.LayoutViewport/properties/pageY。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Vertical offset relative to the document (CSS pixels).</pre>
                    ///
                    ///
                    #[serde(rename = "pageY")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub page_y: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.LayoutViewport/properties/clientWidth。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Width (CSS pixels), excludes scrollbar if present.</pre>
                    ///
                    ///
                    #[serde(rename = "clientWidth")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub client_width: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.LayoutViewport/properties/clientHeight。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Height (CSS pixels), excludes scrollbar if present.</pre>
                    ///
                    ///
                    #[serde(rename = "clientHeight")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub client_height: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LayoutViewport {
                    page_x: fields.page_x,
                    page_y: fields.page_y,
                    client_width: fields.client_width,
                    client_height: fields.client_height,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.NavigationEntry。
///
///协议原文：
///
///<pre>Navigation history entry.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct NavigationEntry {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.NavigationEntry/properties/id。
    ///
    ///协议原文：
    ///
    ///<pre>Unique id of the navigation history entry.</pre>
    ///
    ///
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.NavigationEntry/properties/url。
    ///
    ///协议原文：
    ///
    ///<pre>URL of the navigation history entry.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.NavigationEntry/properties/userTypedURL。
    ///
    ///协议原文：
    ///
    ///<pre>URL that the user typed in the url bar.</pre>
    ///
    ///
    #[serde(rename = "userTypedURL")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub user_typed_url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.NavigationEntry/properties/title。
    ///
    ///协议原文：
    ///
    ///<pre>Title of the navigation history entry.</pre>
    ///
    ///
    #[serde(rename = "title")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub title: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.NavigationEntry/properties/transitionType。
    ///
    ///协议原文：
    ///
    ///<pre>Transition type.</pre>
    ///
    ///
    #[serde(rename = "transitionType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub transition_type: crate::page::types::TransitionType,
}
impl<'de> serde::Deserialize<'de> for NavigationEntry {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NavigationEntry;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.NavigationEntry/properties/id。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Unique id of the navigation history entry.</pre>
                    ///
                    ///
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.NavigationEntry/properties/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>URL of the navigation history entry.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.NavigationEntry/properties/userTypedURL。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>URL that the user typed in the url bar.</pre>
                    ///
                    ///
                    #[serde(rename = "userTypedURL")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub user_typed_url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.NavigationEntry/properties/title。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Title of the navigation history entry.</pre>
                    ///
                    ///
                    #[serde(rename = "title")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub title: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.NavigationEntry/properties/transitionType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Transition type.</pre>
                    ///
                    ///
                    #[serde(rename = "transitionType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub transition_type: crate::page::types::TransitionType,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(NavigationEntry {
                    id: fields.id,
                    url: fields.url,
                    user_typed_url: fields.user_typed_url,
                    title: fields.title,
                    transition_type: fields.transition_type,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.type.NavigationType。
///
///协议原文：
///
///<pre>The type of a frameNavigated event.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum NavigationType {
    ///协议值 "Navigation"。
    Navigation,
    ///协议值 "BackForwardCacheRestore"。
    BackForwardCacheRestore,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl NavigationType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::types::NavigationType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Navigation => "Navigation",
            Self::BackForwardCacheRestore => "BackForwardCacheRestore",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for NavigationType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for NavigationType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Navigation" => NavigationType::Navigation,
            "BackForwardCacheRestore" => NavigationType::BackForwardCacheRestore,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for NavigationType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for NavigationType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NavigationType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Navigation" => NavigationType::Navigation,
                    "BackForwardCacheRestore" => NavigationType::BackForwardCacheRestore,
                    _ => NavigationType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(NavigationType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.OriginTrial。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct OriginTrial {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.OriginTrial/properties/trialName。
    #[serde(rename = "trialName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub trial_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.OriginTrial/properties/status。
    #[serde(rename = "status")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub status: crate::page::types::OriginTrialStatus,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.OriginTrial/properties/tokensWithStatus。
    #[serde(rename = "tokensWithStatus")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub tokens_with_status: Vec<crate::page::types::OriginTrialTokenWithStatus>,
}
impl<'de> serde::Deserialize<'de> for OriginTrial {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = OriginTrial;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.OriginTrial/properties/trialName。
                    #[serde(rename = "trialName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub trial_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.OriginTrial/properties/status。
                    #[serde(rename = "status")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub status: crate::page::types::OriginTrialStatus,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.OriginTrial/properties/tokensWithStatus。
                    #[serde(rename = "tokensWithStatus")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub tokens_with_status: Vec<crate::page::types::OriginTrialTokenWithStatus>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(OriginTrial {
                    trial_name: fields.trial_name,
                    status: fields.status,
                    tokens_with_status: fields.tokens_with_status,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.type.OriginTrialStatus。
///
///协议原文：
///
///<pre>Status for an Origin Trial.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum OriginTrialStatus {
    ///协议值 "Enabled"。
    Enabled,
    ///协议值 "ValidTokenNotProvided"。
    ValidTokenNotProvided,
    ///协议值 "OSNotSupported"。
    OsNotSupported,
    ///协议值 "TrialNotAllowed"。
    TrialNotAllowed,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl OriginTrialStatus {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::types::OriginTrialStatus::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Enabled => "Enabled",
            Self::ValidTokenNotProvided => "ValidTokenNotProvided",
            Self::OsNotSupported => "OSNotSupported",
            Self::TrialNotAllowed => "TrialNotAllowed",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for OriginTrialStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for OriginTrialStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Enabled" => OriginTrialStatus::Enabled,
            "ValidTokenNotProvided" => OriginTrialStatus::ValidTokenNotProvided,
            "OSNotSupported" => OriginTrialStatus::OsNotSupported,
            "TrialNotAllowed" => OriginTrialStatus::TrialNotAllowed,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for OriginTrialStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OriginTrialStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = OriginTrialStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Enabled" => OriginTrialStatus::Enabled,
                    "ValidTokenNotProvided" => OriginTrialStatus::ValidTokenNotProvided,
                    "OSNotSupported" => OriginTrialStatus::OsNotSupported,
                    "TrialNotAllowed" => OriginTrialStatus::TrialNotAllowed,
                    _ => OriginTrialStatus::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(OriginTrialStatus::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.OriginTrialToken。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct OriginTrialToken {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.OriginTrialToken/properties/origin。
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.OriginTrialToken/properties/matchSubDomains。
    #[serde(rename = "matchSubDomains")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub match_sub_domains: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.OriginTrialToken/properties/trialName。
    #[serde(rename = "trialName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub trial_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.OriginTrialToken/properties/expiryTime。
    #[serde(rename = "expiryTime")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub expiry_time: crate::network::types::TimeSinceEpoch,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.OriginTrialToken/properties/isThirdParty。
    #[serde(rename = "isThirdParty")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub is_third_party: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.OriginTrialToken/properties/usageRestriction。
    #[serde(rename = "usageRestriction")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub usage_restriction: crate::page::types::OriginTrialUsageRestriction,
}
impl<'de> serde::Deserialize<'de> for OriginTrialToken {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = OriginTrialToken;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.OriginTrialToken/properties/origin。
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.OriginTrialToken/properties/matchSubDomains。
                    #[serde(rename = "matchSubDomains")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub match_sub_domains: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.OriginTrialToken/properties/trialName。
                    #[serde(rename = "trialName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub trial_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.OriginTrialToken/properties/expiryTime。
                    #[serde(rename = "expiryTime")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub expiry_time: crate::network::types::TimeSinceEpoch,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.OriginTrialToken/properties/isThirdParty。
                    #[serde(rename = "isThirdParty")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub is_third_party: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.OriginTrialToken/properties/usageRestriction。
                    #[serde(rename = "usageRestriction")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub usage_restriction: crate::page::types::OriginTrialUsageRestriction,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(OriginTrialToken {
                    origin: fields.origin,
                    match_sub_domains: fields.match_sub_domains,
                    trial_name: fields.trial_name,
                    expiry_time: fields.expiry_time,
                    is_third_party: fields.is_third_party,
                    usage_restriction: fields.usage_restriction,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.type.OriginTrialTokenStatus。
///
///协议原文：
///
///<pre>Origin Trial(https://www.chromium.org/blink/origin-trials) support.
///Status for an Origin Trial token.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum OriginTrialTokenStatus {
    ///协议值 "Success"。
    Success,
    ///协议值 "NotSupported"。
    NotSupported,
    ///协议值 "Insecure"。
    Insecure,
    ///协议值 "Expired"。
    Expired,
    ///协议值 "WrongOrigin"。
    WrongOrigin,
    ///协议值 "InvalidSignature"。
    InvalidSignature,
    ///协议值 "Malformed"。
    Malformed,
    ///协议值 "WrongVersion"。
    WrongVersion,
    ///协议值 "FeatureDisabled"。
    FeatureDisabled,
    ///协议值 "TokenDisabled"。
    TokenDisabled,
    ///协议值 "FeatureDisabledForUser"。
    FeatureDisabledForUser,
    ///协议值 "UnknownTrial"。
    UnknownTrial,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl OriginTrialTokenStatus {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::types::OriginTrialTokenStatus::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Success => "Success",
            Self::NotSupported => "NotSupported",
            Self::Insecure => "Insecure",
            Self::Expired => "Expired",
            Self::WrongOrigin => "WrongOrigin",
            Self::InvalidSignature => "InvalidSignature",
            Self::Malformed => "Malformed",
            Self::WrongVersion => "WrongVersion",
            Self::FeatureDisabled => "FeatureDisabled",
            Self::TokenDisabled => "TokenDisabled",
            Self::FeatureDisabledForUser => "FeatureDisabledForUser",
            Self::UnknownTrial => "UnknownTrial",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for OriginTrialTokenStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for OriginTrialTokenStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Success" => OriginTrialTokenStatus::Success,
            "NotSupported" => OriginTrialTokenStatus::NotSupported,
            "Insecure" => OriginTrialTokenStatus::Insecure,
            "Expired" => OriginTrialTokenStatus::Expired,
            "WrongOrigin" => OriginTrialTokenStatus::WrongOrigin,
            "InvalidSignature" => OriginTrialTokenStatus::InvalidSignature,
            "Malformed" => OriginTrialTokenStatus::Malformed,
            "WrongVersion" => OriginTrialTokenStatus::WrongVersion,
            "FeatureDisabled" => OriginTrialTokenStatus::FeatureDisabled,
            "TokenDisabled" => OriginTrialTokenStatus::TokenDisabled,
            "FeatureDisabledForUser" => OriginTrialTokenStatus::FeatureDisabledForUser,
            "UnknownTrial" => OriginTrialTokenStatus::UnknownTrial,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for OriginTrialTokenStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OriginTrialTokenStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = OriginTrialTokenStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Success" => OriginTrialTokenStatus::Success,
                    "NotSupported" => OriginTrialTokenStatus::NotSupported,
                    "Insecure" => OriginTrialTokenStatus::Insecure,
                    "Expired" => OriginTrialTokenStatus::Expired,
                    "WrongOrigin" => OriginTrialTokenStatus::WrongOrigin,
                    "InvalidSignature" => OriginTrialTokenStatus::InvalidSignature,
                    "Malformed" => OriginTrialTokenStatus::Malformed,
                    "WrongVersion" => OriginTrialTokenStatus::WrongVersion,
                    "FeatureDisabled" => OriginTrialTokenStatus::FeatureDisabled,
                    "TokenDisabled" => OriginTrialTokenStatus::TokenDisabled,
                    "FeatureDisabledForUser" => OriginTrialTokenStatus::FeatureDisabledForUser,
                    "UnknownTrial" => OriginTrialTokenStatus::UnknownTrial,
                    _ => OriginTrialTokenStatus::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(OriginTrialTokenStatus::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.OriginTrialTokenWithStatus。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct OriginTrialTokenWithStatus {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.OriginTrialTokenWithStatus/properties/rawTokenText。
    #[serde(rename = "rawTokenText")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub raw_token_text: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.OriginTrialTokenWithStatus/properties/parsedToken。
    ///
    ///协议原文：
    ///
    ///<pre>`parsedToken` is present only when the token is extractable and
    ///parsable.</pre>
    ///
    ///
    #[serde(rename = "parsedToken")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub parsed_token: Option<crate::page::types::OriginTrialToken>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.OriginTrialTokenWithStatus/properties/status。
    #[serde(rename = "status")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub status: crate::page::types::OriginTrialTokenStatus,
}
impl<'de> serde::Deserialize<'de> for OriginTrialTokenWithStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = OriginTrialTokenWithStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.OriginTrialTokenWithStatus/properties/rawTokenText。
                    #[serde(rename = "rawTokenText")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub raw_token_text: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.OriginTrialTokenWithStatus/properties/parsedToken。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>`parsedToken` is present only when the token is extractable and
                    ///parsable.</pre>
                    ///
                    ///
                    #[serde(rename = "parsedToken")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub parsed_token: Option<crate::page::types::OriginTrialToken>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.OriginTrialTokenWithStatus/properties/status。
                    #[serde(rename = "status")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub status: crate::page::types::OriginTrialTokenStatus,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(OriginTrialTokenWithStatus {
                    raw_token_text: fields.raw_token_text,
                    parsed_token: fields.parsed_token,
                    status: fields.status,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.type.OriginTrialUsageRestriction。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum OriginTrialUsageRestriction {
    ///协议值 "None"。
    None,
    ///协议值 "Subset"。
    Subset,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl OriginTrialUsageRestriction {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::types::OriginTrialUsageRestriction::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::None => "None",
            Self::Subset => "Subset",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for OriginTrialUsageRestriction {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for OriginTrialUsageRestriction {
    fn from(value: String) -> Self {
        match value.as_str() {
            "None" => OriginTrialUsageRestriction::None,
            "Subset" => OriginTrialUsageRestriction::Subset,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for OriginTrialUsageRestriction {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OriginTrialUsageRestriction {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = OriginTrialUsageRestriction;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "None" => OriginTrialUsageRestriction::None,
                    "Subset" => OriginTrialUsageRestriction::Subset,
                    _ => OriginTrialUsageRestriction::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(OriginTrialUsageRestriction::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.PermissionsPolicyBlockLocator。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PermissionsPolicyBlockLocator {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.PermissionsPolicyBlockLocator/properties/frameId。
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.PermissionsPolicyBlockLocator/properties/blockReason。
    #[serde(rename = "blockReason")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub block_reason: crate::page::types::PermissionsPolicyBlockReason,
}
impl<'de> serde::Deserialize<'de> for PermissionsPolicyBlockLocator {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PermissionsPolicyBlockLocator;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.PermissionsPolicyBlockLocator/properties/frameId。
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.PermissionsPolicyBlockLocator/properties/blockReason。
                    #[serde(rename = "blockReason")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub block_reason: crate::page::types::PermissionsPolicyBlockReason,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PermissionsPolicyBlockLocator {
                    frame_id: fields.frame_id,
                    block_reason: fields.block_reason,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.type.PermissionsPolicyBlockReason。
///
///协议原文：
///
///<pre>Reason for a permissions policy feature to be disabled.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PermissionsPolicyBlockReason {
    ///协议值 "Header"。
    Header,
    ///协议值 "IframeAttribute"。
    IframeAttribute,
    ///协议值 "InFencedFrameTree"。
    InFencedFrameTree,
    ///协议值 "InIsolatedApp"。
    InIsolatedApp,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl PermissionsPolicyBlockReason {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::types::PermissionsPolicyBlockReason::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Header => "Header",
            Self::IframeAttribute => "IframeAttribute",
            Self::InFencedFrameTree => "InFencedFrameTree",
            Self::InIsolatedApp => "InIsolatedApp",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for PermissionsPolicyBlockReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for PermissionsPolicyBlockReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Header" => PermissionsPolicyBlockReason::Header,
            "IframeAttribute" => PermissionsPolicyBlockReason::IframeAttribute,
            "InFencedFrameTree" => PermissionsPolicyBlockReason::InFencedFrameTree,
            "InIsolatedApp" => PermissionsPolicyBlockReason::InIsolatedApp,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for PermissionsPolicyBlockReason {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PermissionsPolicyBlockReason {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PermissionsPolicyBlockReason;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Header" => PermissionsPolicyBlockReason::Header,
                    "IframeAttribute" => PermissionsPolicyBlockReason::IframeAttribute,
                    "InFencedFrameTree" => PermissionsPolicyBlockReason::InFencedFrameTree,
                    "InIsolatedApp" => PermissionsPolicyBlockReason::InIsolatedApp,
                    _ => PermissionsPolicyBlockReason::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(PermissionsPolicyBlockReason::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.type.PermissionsPolicyFeature。
///
///协议原文：
///
///<pre>All Permissions Policy features. This enum should match the one defined
///in services/network/public/cpp/permissions_policy/permissions_policy_features.json5.
///LINT.IfChange(PermissionsPolicyFeature)</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PermissionsPolicyFeature {
    ///协议值 "accelerometer"。
    Accelerometer,
    ///协议值 "all-screens-capture"。
    AllScreensCapture,
    ///协议值 "ambient-light-sensor"。
    AmbientLightSensor,
    ///协议值 "aria-notify"。
    AriaNotify,
    ///协议值 "autofill"。
    Autofill,
    ///协议值 "autoplay"。
    Autoplay,
    ///协议值 "bluetooth"。
    Bluetooth,
    ///协议值 "browsing-topics"。
    BrowsingTopics,
    ///协议值 "camera"。
    Camera,
    ///协议值 "captured-surface-control"。
    CapturedSurfaceControl,
    ///协议值 "ch-dpr"。
    ChDpr,
    ///协议值 "ch-device-memory"。
    ChDeviceMemory,
    ///协议值 "ch-downlink"。
    ChDownlink,
    ///协议值 "ch-ect"。
    ChEct,
    ///协议值 "ch-prefers-color-scheme"。
    ChPrefersColorScheme,
    ///协议值 "ch-prefers-reduced-motion"。
    ChPrefersReducedMotion,
    ///协议值 "ch-prefers-reduced-transparency"。
    ChPrefersReducedTransparency,
    ///协议值 "ch-rtt"。
    ChRtt,
    ///协议值 "ch-save-data"。
    ChSaveData,
    ///协议值 "ch-ua"。
    ChUa,
    ///协议值 "ch-ua-arch"。
    ChUaArch,
    ///协议值 "ch-ua-bitness"。
    ChUaBitness,
    ///协议值 "ch-ua-high-entropy-values"。
    ChUaHighEntropyValues,
    ///协议值 "ch-ua-platform"。
    ChUaPlatform,
    ///协议值 "ch-ua-model"。
    ChUaModel,
    ///协议值 "ch-ua-mobile"。
    ChUaMobile,
    ///协议值 "ch-ua-form-factors"。
    ChUaFormFactors,
    ///协议值 "ch-ua-full-version"。
    ChUaFullVersion,
    ///协议值 "ch-ua-full-version-list"。
    ChUaFullVersionList,
    ///协议值 "ch-ua-platform-version"。
    ChUaPlatformVersion,
    ///协议值 "ch-ua-wow64"。
    ChUaWow64,
    ///协议值 "ch-viewport-height"。
    ChViewportHeight,
    ///协议值 "ch-viewport-width"。
    ChViewportWidth,
    ///协议值 "ch-width"。
    ChWidth,
    ///协议值 "clipboard-read"。
    ClipboardRead,
    ///协议值 "clipboard-write"。
    ClipboardWrite,
    ///协议值 "compute-pressure"。
    ComputePressure,
    ///协议值 "controlled-frame"。
    ControlledFrame,
    ///协议值 "cross-origin-isolated"。
    CrossOriginIsolated,
    ///协议值 "deferred-fetch"。
    DeferredFetch,
    ///协议值 "deferred-fetch-minimal"。
    DeferredFetchMinimal,
    ///协议值 "device-attributes"。
    DeviceAttributes,
    ///协议值 "digital-credentials-create"。
    DigitalCredentialsCreate,
    ///协议值 "digital-credentials-get"。
    DigitalCredentialsGet,
    ///协议值 "direct-sockets"。
    DirectSockets,
    ///协议值 "direct-sockets-multicast"。
    DirectSocketsMulticast,
    ///协议值 "display-capture"。
    DisplayCapture,
    ///协议值 "document-domain"。
    DocumentDomain,
    ///协议值 "encrypted-media"。
    EncryptedMedia,
    ///协议值 "execution-while-out-of-viewport"。
    ExecutionWhileOutOfViewport,
    ///协议值 "execution-while-not-rendered"。
    ExecutionWhileNotRendered,
    ///协议值 "focus-without-user-activation"。
    FocusWithoutUserActivation,
    ///协议值 "fullscreen"。
    Fullscreen,
    ///协议值 "frobulate"。
    Frobulate,
    ///协议值 "gamepad"。
    Gamepad,
    ///协议值 "geolocation"。
    Geolocation,
    ///协议值 "gyroscope"。
    Gyroscope,
    ///协议值 "haptics"。
    Haptics,
    ///协议值 "hid"。
    Hid,
    ///协议值 "identity-credentials-get"。
    IdentityCredentialsGet,
    ///协议值 "idle-detection"。
    IdleDetection,
    ///协议值 "interest-cohort"。
    InterestCohort,
    ///协议值 "keyboard-map"。
    KeyboardMap,
    ///协议值 "language-detector"。
    LanguageDetector,
    ///协议值 "language-model"。
    LanguageModel,
    ///协议值 "local-fonts"。
    LocalFonts,
    ///协议值 "local-network"。
    LocalNetwork,
    ///协议值 "local-network-access"。
    LocalNetworkAccess,
    ///协议值 "loopback-network"。
    LoopbackNetwork,
    ///协议值 "magnetometer"。
    Magnetometer,
    ///协议值 "manual-text"。
    ManualText,
    ///协议值 "media-playback-while-not-visible"。
    MediaPlaybackWhileNotVisible,
    ///协议值 "microphone"。
    Microphone,
    ///协议值 "midi"。
    Midi,
    ///协议值 "on-device-speech-recognition"。
    OnDeviceSpeechRecognition,
    ///协议值 "otp-credentials"。
    OtpCredentials,
    ///协议值 "payment"。
    Payment,
    ///协议值 "picture-in-picture"。
    PictureInPicture,
    ///协议值 "private-state-token-issuance"。
    PrivateStateTokenIssuance,
    ///协议值 "private-state-token-redemption"。
    PrivateStateTokenRedemption,
    ///协议值 "publickey-credentials-create"。
    PublickeyCredentialsCreate,
    ///协议值 "publickey-credentials-get"。
    PublickeyCredentialsGet,
    ///协议值 "rewriter"。
    Rewriter,
    ///协议值 "screen-wake-lock"。
    ScreenWakeLock,
    ///协议值 "serial"。
    Serial,
    ///协议值 "shared-storage"。
    SharedStorage,
    ///协议值 "shared-storage-select-url"。
    SharedStorageSelectUrl,
    ///协议值 "smart-card"。
    SmartCard,
    ///协议值 "speaker-selection"。
    SpeakerSelection,
    ///协议值 "storage-access"。
    StorageAccess,
    ///协议值 "sub-apps"。
    SubApps,
    ///协议值 "summarizer"。
    Summarizer,
    ///协议值 "sync-xhr"。
    SyncXhr,
    ///协议值 "tools"。
    Tools,
    ///协议值 "translator"。
    Translator,
    ///协议值 "unload"。
    Unload,
    ///协议值 "usb"。
    Usb,
    ///协议值 "usb-unrestricted"。
    UsbUnrestricted,
    ///协议值 "vertical-scroll"。
    VerticalScroll,
    ///协议值 "web-app-installation"。
    WebAppInstallation,
    ///协议值 "webnn"。
    Webnn,
    ///协议值 "web-printing"。
    WebPrinting,
    ///协议值 "web-share"。
    WebShare,
    ///协议值 "window-management"。
    WindowManagement,
    ///协议值 "writer"。
    Writer,
    ///协议值 "xr-spatial-tracking"。
    XrSpatialTracking,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl PermissionsPolicyFeature {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::types::PermissionsPolicyFeature::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Accelerometer => "accelerometer",
            Self::AllScreensCapture => "all-screens-capture",
            Self::AmbientLightSensor => "ambient-light-sensor",
            Self::AriaNotify => "aria-notify",
            Self::Autofill => "autofill",
            Self::Autoplay => "autoplay",
            Self::Bluetooth => "bluetooth",
            Self::BrowsingTopics => "browsing-topics",
            Self::Camera => "camera",
            Self::CapturedSurfaceControl => "captured-surface-control",
            Self::ChDpr => "ch-dpr",
            Self::ChDeviceMemory => "ch-device-memory",
            Self::ChDownlink => "ch-downlink",
            Self::ChEct => "ch-ect",
            Self::ChPrefersColorScheme => "ch-prefers-color-scheme",
            Self::ChPrefersReducedMotion => "ch-prefers-reduced-motion",
            Self::ChPrefersReducedTransparency => "ch-prefers-reduced-transparency",
            Self::ChRtt => "ch-rtt",
            Self::ChSaveData => "ch-save-data",
            Self::ChUa => "ch-ua",
            Self::ChUaArch => "ch-ua-arch",
            Self::ChUaBitness => "ch-ua-bitness",
            Self::ChUaHighEntropyValues => "ch-ua-high-entropy-values",
            Self::ChUaPlatform => "ch-ua-platform",
            Self::ChUaModel => "ch-ua-model",
            Self::ChUaMobile => "ch-ua-mobile",
            Self::ChUaFormFactors => "ch-ua-form-factors",
            Self::ChUaFullVersion => "ch-ua-full-version",
            Self::ChUaFullVersionList => "ch-ua-full-version-list",
            Self::ChUaPlatformVersion => "ch-ua-platform-version",
            Self::ChUaWow64 => "ch-ua-wow64",
            Self::ChViewportHeight => "ch-viewport-height",
            Self::ChViewportWidth => "ch-viewport-width",
            Self::ChWidth => "ch-width",
            Self::ClipboardRead => "clipboard-read",
            Self::ClipboardWrite => "clipboard-write",
            Self::ComputePressure => "compute-pressure",
            Self::ControlledFrame => "controlled-frame",
            Self::CrossOriginIsolated => "cross-origin-isolated",
            Self::DeferredFetch => "deferred-fetch",
            Self::DeferredFetchMinimal => "deferred-fetch-minimal",
            Self::DeviceAttributes => "device-attributes",
            Self::DigitalCredentialsCreate => "digital-credentials-create",
            Self::DigitalCredentialsGet => "digital-credentials-get",
            Self::DirectSockets => "direct-sockets",
            Self::DirectSocketsMulticast => "direct-sockets-multicast",
            Self::DisplayCapture => "display-capture",
            Self::DocumentDomain => "document-domain",
            Self::EncryptedMedia => "encrypted-media",
            Self::ExecutionWhileOutOfViewport => "execution-while-out-of-viewport",
            Self::ExecutionWhileNotRendered => "execution-while-not-rendered",
            Self::FocusWithoutUserActivation => "focus-without-user-activation",
            Self::Fullscreen => "fullscreen",
            Self::Frobulate => "frobulate",
            Self::Gamepad => "gamepad",
            Self::Geolocation => "geolocation",
            Self::Gyroscope => "gyroscope",
            Self::Haptics => "haptics",
            Self::Hid => "hid",
            Self::IdentityCredentialsGet => "identity-credentials-get",
            Self::IdleDetection => "idle-detection",
            Self::InterestCohort => "interest-cohort",
            Self::KeyboardMap => "keyboard-map",
            Self::LanguageDetector => "language-detector",
            Self::LanguageModel => "language-model",
            Self::LocalFonts => "local-fonts",
            Self::LocalNetwork => "local-network",
            Self::LocalNetworkAccess => "local-network-access",
            Self::LoopbackNetwork => "loopback-network",
            Self::Magnetometer => "magnetometer",
            Self::ManualText => "manual-text",
            Self::MediaPlaybackWhileNotVisible => "media-playback-while-not-visible",
            Self::Microphone => "microphone",
            Self::Midi => "midi",
            Self::OnDeviceSpeechRecognition => "on-device-speech-recognition",
            Self::OtpCredentials => "otp-credentials",
            Self::Payment => "payment",
            Self::PictureInPicture => "picture-in-picture",
            Self::PrivateStateTokenIssuance => "private-state-token-issuance",
            Self::PrivateStateTokenRedemption => "private-state-token-redemption",
            Self::PublickeyCredentialsCreate => "publickey-credentials-create",
            Self::PublickeyCredentialsGet => "publickey-credentials-get",
            Self::Rewriter => "rewriter",
            Self::ScreenWakeLock => "screen-wake-lock",
            Self::Serial => "serial",
            Self::SharedStorage => "shared-storage",
            Self::SharedStorageSelectUrl => "shared-storage-select-url",
            Self::SmartCard => "smart-card",
            Self::SpeakerSelection => "speaker-selection",
            Self::StorageAccess => "storage-access",
            Self::SubApps => "sub-apps",
            Self::Summarizer => "summarizer",
            Self::SyncXhr => "sync-xhr",
            Self::Tools => "tools",
            Self::Translator => "translator",
            Self::Unload => "unload",
            Self::Usb => "usb",
            Self::UsbUnrestricted => "usb-unrestricted",
            Self::VerticalScroll => "vertical-scroll",
            Self::WebAppInstallation => "web-app-installation",
            Self::Webnn => "webnn",
            Self::WebPrinting => "web-printing",
            Self::WebShare => "web-share",
            Self::WindowManagement => "window-management",
            Self::Writer => "writer",
            Self::XrSpatialTracking => "xr-spatial-tracking",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for PermissionsPolicyFeature {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for PermissionsPolicyFeature {
    fn from(value: String) -> Self {
        match value.as_str() {
            "accelerometer" => PermissionsPolicyFeature::Accelerometer,
            "all-screens-capture" => PermissionsPolicyFeature::AllScreensCapture,
            "ambient-light-sensor" => PermissionsPolicyFeature::AmbientLightSensor,
            "aria-notify" => PermissionsPolicyFeature::AriaNotify,
            "autofill" => PermissionsPolicyFeature::Autofill,
            "autoplay" => PermissionsPolicyFeature::Autoplay,
            "bluetooth" => PermissionsPolicyFeature::Bluetooth,
            "browsing-topics" => PermissionsPolicyFeature::BrowsingTopics,
            "camera" => PermissionsPolicyFeature::Camera,
            "captured-surface-control" => PermissionsPolicyFeature::CapturedSurfaceControl,
            "ch-dpr" => PermissionsPolicyFeature::ChDpr,
            "ch-device-memory" => PermissionsPolicyFeature::ChDeviceMemory,
            "ch-downlink" => PermissionsPolicyFeature::ChDownlink,
            "ch-ect" => PermissionsPolicyFeature::ChEct,
            "ch-prefers-color-scheme" => PermissionsPolicyFeature::ChPrefersColorScheme,
            "ch-prefers-reduced-motion" => PermissionsPolicyFeature::ChPrefersReducedMotion,
            "ch-prefers-reduced-transparency" => {
                PermissionsPolicyFeature::ChPrefersReducedTransparency
            }
            "ch-rtt" => PermissionsPolicyFeature::ChRtt,
            "ch-save-data" => PermissionsPolicyFeature::ChSaveData,
            "ch-ua" => PermissionsPolicyFeature::ChUa,
            "ch-ua-arch" => PermissionsPolicyFeature::ChUaArch,
            "ch-ua-bitness" => PermissionsPolicyFeature::ChUaBitness,
            "ch-ua-high-entropy-values" => PermissionsPolicyFeature::ChUaHighEntropyValues,
            "ch-ua-platform" => PermissionsPolicyFeature::ChUaPlatform,
            "ch-ua-model" => PermissionsPolicyFeature::ChUaModel,
            "ch-ua-mobile" => PermissionsPolicyFeature::ChUaMobile,
            "ch-ua-form-factors" => PermissionsPolicyFeature::ChUaFormFactors,
            "ch-ua-full-version" => PermissionsPolicyFeature::ChUaFullVersion,
            "ch-ua-full-version-list" => PermissionsPolicyFeature::ChUaFullVersionList,
            "ch-ua-platform-version" => PermissionsPolicyFeature::ChUaPlatformVersion,
            "ch-ua-wow64" => PermissionsPolicyFeature::ChUaWow64,
            "ch-viewport-height" => PermissionsPolicyFeature::ChViewportHeight,
            "ch-viewport-width" => PermissionsPolicyFeature::ChViewportWidth,
            "ch-width" => PermissionsPolicyFeature::ChWidth,
            "clipboard-read" => PermissionsPolicyFeature::ClipboardRead,
            "clipboard-write" => PermissionsPolicyFeature::ClipboardWrite,
            "compute-pressure" => PermissionsPolicyFeature::ComputePressure,
            "controlled-frame" => PermissionsPolicyFeature::ControlledFrame,
            "cross-origin-isolated" => PermissionsPolicyFeature::CrossOriginIsolated,
            "deferred-fetch" => PermissionsPolicyFeature::DeferredFetch,
            "deferred-fetch-minimal" => PermissionsPolicyFeature::DeferredFetchMinimal,
            "device-attributes" => PermissionsPolicyFeature::DeviceAttributes,
            "digital-credentials-create" => PermissionsPolicyFeature::DigitalCredentialsCreate,
            "digital-credentials-get" => PermissionsPolicyFeature::DigitalCredentialsGet,
            "direct-sockets" => PermissionsPolicyFeature::DirectSockets,
            "direct-sockets-multicast" => PermissionsPolicyFeature::DirectSocketsMulticast,
            "display-capture" => PermissionsPolicyFeature::DisplayCapture,
            "document-domain" => PermissionsPolicyFeature::DocumentDomain,
            "encrypted-media" => PermissionsPolicyFeature::EncryptedMedia,
            "execution-while-out-of-viewport" => {
                PermissionsPolicyFeature::ExecutionWhileOutOfViewport
            }
            "execution-while-not-rendered" => PermissionsPolicyFeature::ExecutionWhileNotRendered,
            "focus-without-user-activation" => PermissionsPolicyFeature::FocusWithoutUserActivation,
            "fullscreen" => PermissionsPolicyFeature::Fullscreen,
            "frobulate" => PermissionsPolicyFeature::Frobulate,
            "gamepad" => PermissionsPolicyFeature::Gamepad,
            "geolocation" => PermissionsPolicyFeature::Geolocation,
            "gyroscope" => PermissionsPolicyFeature::Gyroscope,
            "haptics" => PermissionsPolicyFeature::Haptics,
            "hid" => PermissionsPolicyFeature::Hid,
            "identity-credentials-get" => PermissionsPolicyFeature::IdentityCredentialsGet,
            "idle-detection" => PermissionsPolicyFeature::IdleDetection,
            "interest-cohort" => PermissionsPolicyFeature::InterestCohort,
            "keyboard-map" => PermissionsPolicyFeature::KeyboardMap,
            "language-detector" => PermissionsPolicyFeature::LanguageDetector,
            "language-model" => PermissionsPolicyFeature::LanguageModel,
            "local-fonts" => PermissionsPolicyFeature::LocalFonts,
            "local-network" => PermissionsPolicyFeature::LocalNetwork,
            "local-network-access" => PermissionsPolicyFeature::LocalNetworkAccess,
            "loopback-network" => PermissionsPolicyFeature::LoopbackNetwork,
            "magnetometer" => PermissionsPolicyFeature::Magnetometer,
            "manual-text" => PermissionsPolicyFeature::ManualText,
            "media-playback-while-not-visible" => {
                PermissionsPolicyFeature::MediaPlaybackWhileNotVisible
            }
            "microphone" => PermissionsPolicyFeature::Microphone,
            "midi" => PermissionsPolicyFeature::Midi,
            "on-device-speech-recognition" => PermissionsPolicyFeature::OnDeviceSpeechRecognition,
            "otp-credentials" => PermissionsPolicyFeature::OtpCredentials,
            "payment" => PermissionsPolicyFeature::Payment,
            "picture-in-picture" => PermissionsPolicyFeature::PictureInPicture,
            "private-state-token-issuance" => PermissionsPolicyFeature::PrivateStateTokenIssuance,
            "private-state-token-redemption" => {
                PermissionsPolicyFeature::PrivateStateTokenRedemption
            }
            "publickey-credentials-create" => PermissionsPolicyFeature::PublickeyCredentialsCreate,
            "publickey-credentials-get" => PermissionsPolicyFeature::PublickeyCredentialsGet,
            "rewriter" => PermissionsPolicyFeature::Rewriter,
            "screen-wake-lock" => PermissionsPolicyFeature::ScreenWakeLock,
            "serial" => PermissionsPolicyFeature::Serial,
            "shared-storage" => PermissionsPolicyFeature::SharedStorage,
            "shared-storage-select-url" => PermissionsPolicyFeature::SharedStorageSelectUrl,
            "smart-card" => PermissionsPolicyFeature::SmartCard,
            "speaker-selection" => PermissionsPolicyFeature::SpeakerSelection,
            "storage-access" => PermissionsPolicyFeature::StorageAccess,
            "sub-apps" => PermissionsPolicyFeature::SubApps,
            "summarizer" => PermissionsPolicyFeature::Summarizer,
            "sync-xhr" => PermissionsPolicyFeature::SyncXhr,
            "tools" => PermissionsPolicyFeature::Tools,
            "translator" => PermissionsPolicyFeature::Translator,
            "unload" => PermissionsPolicyFeature::Unload,
            "usb" => PermissionsPolicyFeature::Usb,
            "usb-unrestricted" => PermissionsPolicyFeature::UsbUnrestricted,
            "vertical-scroll" => PermissionsPolicyFeature::VerticalScroll,
            "web-app-installation" => PermissionsPolicyFeature::WebAppInstallation,
            "webnn" => PermissionsPolicyFeature::Webnn,
            "web-printing" => PermissionsPolicyFeature::WebPrinting,
            "web-share" => PermissionsPolicyFeature::WebShare,
            "window-management" => PermissionsPolicyFeature::WindowManagement,
            "writer" => PermissionsPolicyFeature::Writer,
            "xr-spatial-tracking" => PermissionsPolicyFeature::XrSpatialTracking,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for PermissionsPolicyFeature {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PermissionsPolicyFeature {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PermissionsPolicyFeature;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "accelerometer" => PermissionsPolicyFeature::Accelerometer,
                    "all-screens-capture" => PermissionsPolicyFeature::AllScreensCapture,
                    "ambient-light-sensor" => PermissionsPolicyFeature::AmbientLightSensor,
                    "aria-notify" => PermissionsPolicyFeature::AriaNotify,
                    "autofill" => PermissionsPolicyFeature::Autofill,
                    "autoplay" => PermissionsPolicyFeature::Autoplay,
                    "bluetooth" => PermissionsPolicyFeature::Bluetooth,
                    "browsing-topics" => PermissionsPolicyFeature::BrowsingTopics,
                    "camera" => PermissionsPolicyFeature::Camera,
                    "captured-surface-control" => PermissionsPolicyFeature::CapturedSurfaceControl,
                    "ch-dpr" => PermissionsPolicyFeature::ChDpr,
                    "ch-device-memory" => PermissionsPolicyFeature::ChDeviceMemory,
                    "ch-downlink" => PermissionsPolicyFeature::ChDownlink,
                    "ch-ect" => PermissionsPolicyFeature::ChEct,
                    "ch-prefers-color-scheme" => PermissionsPolicyFeature::ChPrefersColorScheme,
                    "ch-prefers-reduced-motion" => PermissionsPolicyFeature::ChPrefersReducedMotion,
                    "ch-prefers-reduced-transparency" => {
                        PermissionsPolicyFeature::ChPrefersReducedTransparency
                    }
                    "ch-rtt" => PermissionsPolicyFeature::ChRtt,
                    "ch-save-data" => PermissionsPolicyFeature::ChSaveData,
                    "ch-ua" => PermissionsPolicyFeature::ChUa,
                    "ch-ua-arch" => PermissionsPolicyFeature::ChUaArch,
                    "ch-ua-bitness" => PermissionsPolicyFeature::ChUaBitness,
                    "ch-ua-high-entropy-values" => PermissionsPolicyFeature::ChUaHighEntropyValues,
                    "ch-ua-platform" => PermissionsPolicyFeature::ChUaPlatform,
                    "ch-ua-model" => PermissionsPolicyFeature::ChUaModel,
                    "ch-ua-mobile" => PermissionsPolicyFeature::ChUaMobile,
                    "ch-ua-form-factors" => PermissionsPolicyFeature::ChUaFormFactors,
                    "ch-ua-full-version" => PermissionsPolicyFeature::ChUaFullVersion,
                    "ch-ua-full-version-list" => PermissionsPolicyFeature::ChUaFullVersionList,
                    "ch-ua-platform-version" => PermissionsPolicyFeature::ChUaPlatformVersion,
                    "ch-ua-wow64" => PermissionsPolicyFeature::ChUaWow64,
                    "ch-viewport-height" => PermissionsPolicyFeature::ChViewportHeight,
                    "ch-viewport-width" => PermissionsPolicyFeature::ChViewportWidth,
                    "ch-width" => PermissionsPolicyFeature::ChWidth,
                    "clipboard-read" => PermissionsPolicyFeature::ClipboardRead,
                    "clipboard-write" => PermissionsPolicyFeature::ClipboardWrite,
                    "compute-pressure" => PermissionsPolicyFeature::ComputePressure,
                    "controlled-frame" => PermissionsPolicyFeature::ControlledFrame,
                    "cross-origin-isolated" => PermissionsPolicyFeature::CrossOriginIsolated,
                    "deferred-fetch" => PermissionsPolicyFeature::DeferredFetch,
                    "deferred-fetch-minimal" => PermissionsPolicyFeature::DeferredFetchMinimal,
                    "device-attributes" => PermissionsPolicyFeature::DeviceAttributes,
                    "digital-credentials-create" => {
                        PermissionsPolicyFeature::DigitalCredentialsCreate
                    }
                    "digital-credentials-get" => PermissionsPolicyFeature::DigitalCredentialsGet,
                    "direct-sockets" => PermissionsPolicyFeature::DirectSockets,
                    "direct-sockets-multicast" => PermissionsPolicyFeature::DirectSocketsMulticast,
                    "display-capture" => PermissionsPolicyFeature::DisplayCapture,
                    "document-domain" => PermissionsPolicyFeature::DocumentDomain,
                    "encrypted-media" => PermissionsPolicyFeature::EncryptedMedia,
                    "execution-while-out-of-viewport" => {
                        PermissionsPolicyFeature::ExecutionWhileOutOfViewport
                    }
                    "execution-while-not-rendered" => {
                        PermissionsPolicyFeature::ExecutionWhileNotRendered
                    }
                    "focus-without-user-activation" => {
                        PermissionsPolicyFeature::FocusWithoutUserActivation
                    }
                    "fullscreen" => PermissionsPolicyFeature::Fullscreen,
                    "frobulate" => PermissionsPolicyFeature::Frobulate,
                    "gamepad" => PermissionsPolicyFeature::Gamepad,
                    "geolocation" => PermissionsPolicyFeature::Geolocation,
                    "gyroscope" => PermissionsPolicyFeature::Gyroscope,
                    "haptics" => PermissionsPolicyFeature::Haptics,
                    "hid" => PermissionsPolicyFeature::Hid,
                    "identity-credentials-get" => PermissionsPolicyFeature::IdentityCredentialsGet,
                    "idle-detection" => PermissionsPolicyFeature::IdleDetection,
                    "interest-cohort" => PermissionsPolicyFeature::InterestCohort,
                    "keyboard-map" => PermissionsPolicyFeature::KeyboardMap,
                    "language-detector" => PermissionsPolicyFeature::LanguageDetector,
                    "language-model" => PermissionsPolicyFeature::LanguageModel,
                    "local-fonts" => PermissionsPolicyFeature::LocalFonts,
                    "local-network" => PermissionsPolicyFeature::LocalNetwork,
                    "local-network-access" => PermissionsPolicyFeature::LocalNetworkAccess,
                    "loopback-network" => PermissionsPolicyFeature::LoopbackNetwork,
                    "magnetometer" => PermissionsPolicyFeature::Magnetometer,
                    "manual-text" => PermissionsPolicyFeature::ManualText,
                    "media-playback-while-not-visible" => {
                        PermissionsPolicyFeature::MediaPlaybackWhileNotVisible
                    }
                    "microphone" => PermissionsPolicyFeature::Microphone,
                    "midi" => PermissionsPolicyFeature::Midi,
                    "on-device-speech-recognition" => {
                        PermissionsPolicyFeature::OnDeviceSpeechRecognition
                    }
                    "otp-credentials" => PermissionsPolicyFeature::OtpCredentials,
                    "payment" => PermissionsPolicyFeature::Payment,
                    "picture-in-picture" => PermissionsPolicyFeature::PictureInPicture,
                    "private-state-token-issuance" => {
                        PermissionsPolicyFeature::PrivateStateTokenIssuance
                    }
                    "private-state-token-redemption" => {
                        PermissionsPolicyFeature::PrivateStateTokenRedemption
                    }
                    "publickey-credentials-create" => {
                        PermissionsPolicyFeature::PublickeyCredentialsCreate
                    }
                    "publickey-credentials-get" => {
                        PermissionsPolicyFeature::PublickeyCredentialsGet
                    }
                    "rewriter" => PermissionsPolicyFeature::Rewriter,
                    "screen-wake-lock" => PermissionsPolicyFeature::ScreenWakeLock,
                    "serial" => PermissionsPolicyFeature::Serial,
                    "shared-storage" => PermissionsPolicyFeature::SharedStorage,
                    "shared-storage-select-url" => PermissionsPolicyFeature::SharedStorageSelectUrl,
                    "smart-card" => PermissionsPolicyFeature::SmartCard,
                    "speaker-selection" => PermissionsPolicyFeature::SpeakerSelection,
                    "storage-access" => PermissionsPolicyFeature::StorageAccess,
                    "sub-apps" => PermissionsPolicyFeature::SubApps,
                    "summarizer" => PermissionsPolicyFeature::Summarizer,
                    "sync-xhr" => PermissionsPolicyFeature::SyncXhr,
                    "tools" => PermissionsPolicyFeature::Tools,
                    "translator" => PermissionsPolicyFeature::Translator,
                    "unload" => PermissionsPolicyFeature::Unload,
                    "usb" => PermissionsPolicyFeature::Usb,
                    "usb-unrestricted" => PermissionsPolicyFeature::UsbUnrestricted,
                    "vertical-scroll" => PermissionsPolicyFeature::VerticalScroll,
                    "web-app-installation" => PermissionsPolicyFeature::WebAppInstallation,
                    "webnn" => PermissionsPolicyFeature::Webnn,
                    "web-printing" => PermissionsPolicyFeature::WebPrinting,
                    "web-share" => PermissionsPolicyFeature::WebShare,
                    "window-management" => PermissionsPolicyFeature::WindowManagement,
                    "writer" => PermissionsPolicyFeature::Writer,
                    "xr-spatial-tracking" => PermissionsPolicyFeature::XrSpatialTracking,
                    _ => PermissionsPolicyFeature::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(PermissionsPolicyFeature::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.PermissionsPolicyFeatureState。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct PermissionsPolicyFeatureState {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.PermissionsPolicyFeatureState/properties/feature。
    #[serde(rename = "feature")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub feature: crate::page::types::PermissionsPolicyFeature,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.PermissionsPolicyFeatureState/properties/allowed。
    #[serde(rename = "allowed")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub allowed: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.PermissionsPolicyFeatureState/properties/locator。
    #[serde(rename = "locator")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub locator: Option<crate::page::types::PermissionsPolicyBlockLocator>,
}
impl<'de> serde::Deserialize<'de> for PermissionsPolicyFeatureState {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PermissionsPolicyFeatureState;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.PermissionsPolicyFeatureState/properties/feature。
                    #[serde(rename = "feature")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub feature: crate::page::types::PermissionsPolicyFeature,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.PermissionsPolicyFeatureState/properties/allowed。
                    #[serde(rename = "allowed")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub allowed: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.PermissionsPolicyFeatureState/properties/locator。
                    #[serde(rename = "locator")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub locator: Option<crate::page::types::PermissionsPolicyBlockLocator>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PermissionsPolicyFeatureState {
                    feature: fields.feature,
                    allowed: fields.allowed,
                    locator: fields.locator,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.ProtocolHandler。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProtocolHandler {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ProtocolHandler/properties/protocol。
    #[serde(rename = "protocol")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub protocol: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ProtocolHandler/properties/url。
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
}
impl<'de> serde::Deserialize<'de> for ProtocolHandler {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ProtocolHandler;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ProtocolHandler/properties/protocol。
                    #[serde(rename = "protocol")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub protocol: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ProtocolHandler/properties/url。
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ProtocolHandler {
                    protocol: fields.protocol,
                    url: fields.url,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.type.ReferrerPolicy。
///
///协议原文：
///
///<pre>The referring-policy used for the navigation.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ReferrerPolicy {
    ///协议值 "noReferrer"。
    NoReferrer,
    ///协议值 "noReferrerWhenDowngrade"。
    NoReferrerWhenDowngrade,
    ///协议值 "origin"。
    Origin,
    ///协议值 "originWhenCrossOrigin"。
    OriginWhenCrossOrigin,
    ///协议值 "sameOrigin"。
    SameOrigin,
    ///协议值 "strictOrigin"。
    StrictOrigin,
    ///协议值 "strictOriginWhenCrossOrigin"。
    StrictOriginWhenCrossOrigin,
    ///协议值 "unsafeUrl"。
    UnsafeUrl,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ReferrerPolicy {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::types::ReferrerPolicy::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::NoReferrer => "noReferrer",
            Self::NoReferrerWhenDowngrade => "noReferrerWhenDowngrade",
            Self::Origin => "origin",
            Self::OriginWhenCrossOrigin => "originWhenCrossOrigin",
            Self::SameOrigin => "sameOrigin",
            Self::StrictOrigin => "strictOrigin",
            Self::StrictOriginWhenCrossOrigin => "strictOriginWhenCrossOrigin",
            Self::UnsafeUrl => "unsafeUrl",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ReferrerPolicy {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ReferrerPolicy {
    fn from(value: String) -> Self {
        match value.as_str() {
            "noReferrer" => ReferrerPolicy::NoReferrer,
            "noReferrerWhenDowngrade" => ReferrerPolicy::NoReferrerWhenDowngrade,
            "origin" => ReferrerPolicy::Origin,
            "originWhenCrossOrigin" => ReferrerPolicy::OriginWhenCrossOrigin,
            "sameOrigin" => ReferrerPolicy::SameOrigin,
            "strictOrigin" => ReferrerPolicy::StrictOrigin,
            "strictOriginWhenCrossOrigin" => ReferrerPolicy::StrictOriginWhenCrossOrigin,
            "unsafeUrl" => ReferrerPolicy::UnsafeUrl,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ReferrerPolicy {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReferrerPolicy {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReferrerPolicy;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "noReferrer" => ReferrerPolicy::NoReferrer,
                    "noReferrerWhenDowngrade" => ReferrerPolicy::NoReferrerWhenDowngrade,
                    "origin" => ReferrerPolicy::Origin,
                    "originWhenCrossOrigin" => ReferrerPolicy::OriginWhenCrossOrigin,
                    "sameOrigin" => ReferrerPolicy::SameOrigin,
                    "strictOrigin" => ReferrerPolicy::StrictOrigin,
                    "strictOriginWhenCrossOrigin" => ReferrerPolicy::StrictOriginWhenCrossOrigin,
                    "unsafeUrl" => ReferrerPolicy::UnsafeUrl,
                    _ => ReferrerPolicy::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ReferrerPolicy::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.RelatedApplication。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RelatedApplication {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.RelatedApplication/properties/id。
    #[serde(rename = "id")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.RelatedApplication/properties/url。
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
}
impl<'de> serde::Deserialize<'de> for RelatedApplication {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RelatedApplication;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.RelatedApplication/properties/id。
                    #[serde(rename = "id")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub id: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.RelatedApplication/properties/url。
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RelatedApplication {
                    id: fields.id,
                    url: fields.url,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.ScopeExtension。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ScopeExtension {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ScopeExtension/properties/origin。
    ///
    ///协议原文：
    ///
    ///<pre>Instead of using tuple, this field always returns the serialized string
    ///for easy understanding and comparison.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ScopeExtension/properties/hasOriginWildcard。
    #[serde(rename = "hasOriginWildcard")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub has_origin_wildcard: bool,
}
impl<'de> serde::Deserialize<'de> for ScopeExtension {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScopeExtension;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ScopeExtension/properties/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Instead of using tuple, this field always returns the serialized string
                    ///for easy understanding and comparison.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ScopeExtension/properties/hasOriginWildcard。
                    #[serde(rename = "hasOriginWildcard")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub has_origin_wildcard: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ScopeExtension {
                    origin: fields.origin,
                    has_origin_wildcard: fields.has_origin_wildcard,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.ScreencastFrameMetadata。
///
///协议原文：
///
///<pre>Screencast frame metadata.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ScreencastFrameMetadata {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ScreencastFrameMetadata/properties/offsetTop。
    ///
    ///协议原文：
    ///
    ///<pre>Top offset in DIP.</pre>
    ///
    ///
    #[serde(rename = "offsetTop")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub offset_top: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ScreencastFrameMetadata/properties/pageScaleFactor。
    ///
    ///协议原文：
    ///
    ///<pre>Page scale factor.</pre>
    ///
    ///
    #[serde(rename = "pageScaleFactor")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub page_scale_factor: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ScreencastFrameMetadata/properties/deviceWidth。
    ///
    ///协议原文：
    ///
    ///<pre>Device screen width in DIP.</pre>
    ///
    ///
    #[serde(rename = "deviceWidth")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub device_width: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ScreencastFrameMetadata/properties/deviceHeight。
    ///
    ///协议原文：
    ///
    ///<pre>Device screen height in DIP.</pre>
    ///
    ///
    #[serde(rename = "deviceHeight")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub device_height: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ScreencastFrameMetadata/properties/scrollOffsetX。
    ///
    ///协议原文：
    ///
    ///<pre>Position of horizontal scroll in CSS pixels.</pre>
    ///
    ///
    #[serde(rename = "scrollOffsetX")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub scroll_offset_x: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ScreencastFrameMetadata/properties/scrollOffsetY。
    ///
    ///协议原文：
    ///
    ///<pre>Position of vertical scroll in CSS pixels.</pre>
    ///
    ///
    #[serde(rename = "scrollOffsetY")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub scroll_offset_y: ydp_core::FiniteF64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.ScreencastFrameMetadata/properties/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Frame swap timestamp.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub timestamp: Option<crate::network::types::TimeSinceEpoch>,
}
impl<'de> serde::Deserialize<'de> for ScreencastFrameMetadata {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScreencastFrameMetadata;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ScreencastFrameMetadata/properties/offsetTop。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Top offset in DIP.</pre>
                    ///
                    ///
                    #[serde(rename = "offsetTop")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub offset_top: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ScreencastFrameMetadata/properties/pageScaleFactor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Page scale factor.</pre>
                    ///
                    ///
                    #[serde(rename = "pageScaleFactor")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub page_scale_factor: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ScreencastFrameMetadata/properties/deviceWidth。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Device screen width in DIP.</pre>
                    ///
                    ///
                    #[serde(rename = "deviceWidth")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub device_width: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ScreencastFrameMetadata/properties/deviceHeight。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Device screen height in DIP.</pre>
                    ///
                    ///
                    #[serde(rename = "deviceHeight")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub device_height: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ScreencastFrameMetadata/properties/scrollOffsetX。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Position of horizontal scroll in CSS pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "scrollOffsetX")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub scroll_offset_x: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ScreencastFrameMetadata/properties/scrollOffsetY。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Position of vertical scroll in CSS pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "scrollOffsetY")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub scroll_offset_y: ydp_core::FiniteF64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.ScreencastFrameMetadata/properties/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame swap timestamp.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub timestamp: Option<crate::network::types::TimeSinceEpoch>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ScreencastFrameMetadata {
                    offset_top: fields.offset_top,
                    page_scale_factor: fields.page_scale_factor,
                    device_width: fields.device_width,
                    device_height: fields.device_height,
                    scroll_offset_x: fields.scroll_offset_x,
                    scroll_offset_y: fields.scroll_offset_y,
                    timestamp: fields.timestamp,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.Screenshot。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct Screenshot {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Screenshot/properties/image。
    #[serde(rename = "image")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub image: crate::page::types::ImageResource,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Screenshot/properties/formFactor。
    #[serde(rename = "formFactor")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub form_factor: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.Screenshot/properties/label。
    #[serde(rename = "label")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub label: Option<String>,
}
impl<'de> serde::Deserialize<'de> for Screenshot {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Screenshot;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Screenshot/properties/image。
                    #[serde(rename = "image")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub image: crate::page::types::ImageResource,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Screenshot/properties/formFactor。
                    #[serde(rename = "formFactor")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub form_factor: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.Screenshot/properties/label。
                    #[serde(rename = "label")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub label: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Screenshot {
                    image: fields.image,
                    form_factor: fields.form_factor,
                    label: fields.label,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.ScriptFontFamilies。
///
///协议原文：
///
///<pre>Font families collection for a script.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ScriptFontFamilies {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ScriptFontFamilies/properties/script。
    ///
    ///协议原文：
    ///
    ///<pre>Name of the script which these font families are defined for.</pre>
    ///
    ///
    #[serde(rename = "script")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub script: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ScriptFontFamilies/properties/fontFamilies。
    ///
    ///协议原文：
    ///
    ///<pre>Generic font families collection for the script.</pre>
    ///
    ///
    #[serde(rename = "fontFamilies")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub font_families: crate::page::types::FontFamilies,
}
impl<'de> serde::Deserialize<'de> for ScriptFontFamilies {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScriptFontFamilies;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ScriptFontFamilies/properties/script。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Name of the script which these font families are defined for.</pre>
                    ///
                    ///
                    #[serde(rename = "script")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub script: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ScriptFontFamilies/properties/fontFamilies。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Generic font families collection for the script.</pre>
                    ///
                    ///
                    #[serde(rename = "fontFamilies")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub font_families: crate::page::types::FontFamilies,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ScriptFontFamilies {
                    script: fields.script,
                    font_families: fields.font_families,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Page.type.ScriptIdentifier。
///
///协议原文：
///
///<pre>Unique script identifier.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ScriptIdentifier(String);
impl ScriptIdentifier {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::page::types::ScriptIdentifier = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::page::types::ScriptIdentifier::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::page::types::ScriptIdentifier = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::page::types::ScriptIdentifier::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::page::types::ScriptIdentifier::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.type.SecureContextType。
///
///协议原文：
///
///<pre>Indicates whether the frame is a secure context and why it is the case.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SecureContextType {
    ///协议值 "Secure"。
    Secure,
    ///协议值 "SecureLocalhost"。
    SecureLocalhost,
    ///协议值 "InsecureScheme"。
    InsecureScheme,
    ///协议值 "InsecureAncestor"。
    InsecureAncestor,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl SecureContextType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::types::SecureContextType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Secure => "Secure",
            Self::SecureLocalhost => "SecureLocalhost",
            Self::InsecureScheme => "InsecureScheme",
            Self::InsecureAncestor => "InsecureAncestor",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for SecureContextType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for SecureContextType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Secure" => SecureContextType::Secure,
            "SecureLocalhost" => SecureContextType::SecureLocalhost,
            "InsecureScheme" => SecureContextType::InsecureScheme,
            "InsecureAncestor" => SecureContextType::InsecureAncestor,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for SecureContextType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SecureContextType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SecureContextType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Secure" => SecureContextType::Secure,
                    "SecureLocalhost" => SecureContextType::SecureLocalhost,
                    "InsecureScheme" => SecureContextType::InsecureScheme,
                    "InsecureAncestor" => SecureContextType::InsecureAncestor,
                    _ => SecureContextType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(SecureContextType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.SecurityOriginDetails。
///
///协议原文：
///
///<pre>Additional information about the frame document's security origin.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SecurityOriginDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.SecurityOriginDetails/properties/isLocalhost。
    ///
    ///协议原文：
    ///
    ///<pre>Indicates whether the frame document's security origin is one
    ///of the local hostnames (e.g. "localhost") or IP addresses (IPv4
    ///127.0.0.0/8 or IPv6 ::1).</pre>
    ///
    ///
    #[serde(rename = "isLocalhost")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub is_localhost: bool,
}
impl<'de> serde::Deserialize<'de> for SecurityOriginDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SecurityOriginDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.SecurityOriginDetails/properties/isLocalhost。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Indicates whether the frame document's security origin is one
                    ///of the local hostnames (e.g. "localhost") or IP addresses (IPv4
                    ///127.0.0.0/8 or IPv6 ::1).</pre>
                    ///
                    ///
                    #[serde(rename = "isLocalhost")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub is_localhost: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SecurityOriginDetails {
                    is_localhost: fields.is_localhost,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.ShareTarget。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ShareTarget {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ShareTarget/properties/action。
    #[serde(rename = "action")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub action: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ShareTarget/properties/method。
    #[serde(rename = "method")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub method: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ShareTarget/properties/enctype。
    #[serde(rename = "enctype")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub enctype: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.ShareTarget/properties/title。
    ///
    ///协议原文：
    ///
    ///<pre>Embed the ShareTargetParams</pre>
    ///
    ///
    #[serde(rename = "title")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub title: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.ShareTarget/properties/text。
    #[serde(rename = "text")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub text: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.ShareTarget/properties/url。
    #[serde(rename = "url")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.ShareTarget/properties/files。
    #[serde(rename = "files")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub files: Option<Vec<crate::page::types::FileFilter>>,
}
impl<'de> serde::Deserialize<'de> for ShareTarget {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ShareTarget;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ShareTarget/properties/action。
                    #[serde(rename = "action")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub action: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ShareTarget/properties/method。
                    #[serde(rename = "method")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub method: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.ShareTarget/properties/enctype。
                    #[serde(rename = "enctype")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub enctype: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.ShareTarget/properties/title。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Embed the ShareTargetParams</pre>
                    ///
                    ///
                    #[serde(rename = "title")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub title: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.ShareTarget/properties/text。
                    #[serde(rename = "text")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub text: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.ShareTarget/properties/url。
                    #[serde(rename = "url")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub url: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.ShareTarget/properties/files。
                    #[serde(rename = "files")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub files: Option<Vec<crate::page::types::FileFilter>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ShareTarget {
                    action: fields.action,
                    method: fields.method,
                    enctype: fields.enctype,
                    title: fields.title,
                    text: fields.text,
                    url: fields.url,
                    files: fields.files,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.Shortcut。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct Shortcut {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Shortcut/properties/name。
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Shortcut/properties/url。
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
}
impl<'de> serde::Deserialize<'de> for Shortcut {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Shortcut;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Shortcut/properties/name。
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Shortcut/properties/url。
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Shortcut {
                    name: fields.name,
                    url: fields.url,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Page.type.TransitionType。
///
///协议原文：
///
///<pre>Transition type.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TransitionType {
    ///协议值 "link"。
    Link,
    ///协议值 "typed"。
    Typed,
    ///协议值 "address_bar"。
    AddressBar,
    ///协议值 "auto_bookmark"。
    AutoBookmark,
    ///协议值 "auto_subframe"。
    AutoSubframe,
    ///协议值 "manual_subframe"。
    ManualSubframe,
    ///协议值 "generated"。
    Generated,
    ///协议值 "auto_toplevel"。
    AutoToplevel,
    ///协议值 "form_submit"。
    FormSubmit,
    ///协议值 "reload"。
    Reload,
    ///协议值 "keyword"。
    Keyword,
    ///协议值 "keyword_generated"。
    KeywordGenerated,
    ///协议值 "other"。
    Other,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl TransitionType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::page::types::TransitionType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Link => "link",
            Self::Typed => "typed",
            Self::AddressBar => "address_bar",
            Self::AutoBookmark => "auto_bookmark",
            Self::AutoSubframe => "auto_subframe",
            Self::ManualSubframe => "manual_subframe",
            Self::Generated => "generated",
            Self::AutoToplevel => "auto_toplevel",
            Self::FormSubmit => "form_submit",
            Self::Reload => "reload",
            Self::Keyword => "keyword",
            Self::KeywordGenerated => "keyword_generated",
            Self::Other => "other",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for TransitionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for TransitionType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "link" => TransitionType::Link,
            "typed" => TransitionType::Typed,
            "address_bar" => TransitionType::AddressBar,
            "auto_bookmark" => TransitionType::AutoBookmark,
            "auto_subframe" => TransitionType::AutoSubframe,
            "manual_subframe" => TransitionType::ManualSubframe,
            "generated" => TransitionType::Generated,
            "auto_toplevel" => TransitionType::AutoToplevel,
            "form_submit" => TransitionType::FormSubmit,
            "reload" => TransitionType::Reload,
            "keyword" => TransitionType::Keyword,
            "keyword_generated" => TransitionType::KeywordGenerated,
            "other" => TransitionType::Other,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for TransitionType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TransitionType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TransitionType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "link" => TransitionType::Link,
                    "typed" => TransitionType::Typed,
                    "address_bar" => TransitionType::AddressBar,
                    "auto_bookmark" => TransitionType::AutoBookmark,
                    "auto_subframe" => TransitionType::AutoSubframe,
                    "manual_subframe" => TransitionType::ManualSubframe,
                    "generated" => TransitionType::Generated,
                    "auto_toplevel" => TransitionType::AutoToplevel,
                    "form_submit" => TransitionType::FormSubmit,
                    "reload" => TransitionType::Reload,
                    "keyword" => TransitionType::Keyword,
                    "keyword_generated" => TransitionType::KeywordGenerated,
                    "other" => TransitionType::Other,
                    _ => TransitionType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(TransitionType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.Viewport。
///
///协议原文：
///
///<pre>Viewport for capturing screenshot.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct Viewport {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Viewport/properties/x。
    ///
    ///协议原文：
    ///
    ///<pre>X offset in device independent pixels (dip).</pre>
    ///
    ///
    #[serde(rename = "x")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub x: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Viewport/properties/y。
    ///
    ///协议原文：
    ///
    ///<pre>Y offset in device independent pixels (dip).</pre>
    ///
    ///
    #[serde(rename = "y")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub y: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Viewport/properties/width。
    ///
    ///协议原文：
    ///
    ///<pre>Rectangle width in device independent pixels (dip).</pre>
    ///
    ///
    #[serde(rename = "width")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub width: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Viewport/properties/height。
    ///
    ///协议原文：
    ///
    ///<pre>Rectangle height in device independent pixels (dip).</pre>
    ///
    ///
    #[serde(rename = "height")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub height: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Viewport/properties/scale。
    ///
    ///协议原文：
    ///
    ///<pre>Page scale factor.</pre>
    ///
    ///
    #[serde(rename = "scale")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub scale: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for Viewport {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Viewport;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Viewport/properties/x。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>X offset in device independent pixels (dip).</pre>
                    ///
                    ///
                    #[serde(rename = "x")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub x: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Viewport/properties/y。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Y offset in device independent pixels (dip).</pre>
                    ///
                    ///
                    #[serde(rename = "y")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub y: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Viewport/properties/width。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Rectangle width in device independent pixels (dip).</pre>
                    ///
                    ///
                    #[serde(rename = "width")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub width: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Viewport/properties/height。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Rectangle height in device independent pixels (dip).</pre>
                    ///
                    ///
                    #[serde(rename = "height")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub height: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.Viewport/properties/scale。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Page scale factor.</pre>
                    ///
                    ///
                    #[serde(rename = "scale")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub scale: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Viewport {
                    x: fields.x,
                    y: fields.y,
                    width: fields.width,
                    height: fields.height,
                    scale: fields.scale,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.VisualViewport。
///
///协议原文：
///
///<pre>Visual viewport position, dimensions, and scale.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct VisualViewport {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.VisualViewport/properties/offsetX。
    ///
    ///协议原文：
    ///
    ///<pre>Horizontal offset relative to the layout viewport (CSS pixels).</pre>
    ///
    ///
    #[serde(rename = "offsetX")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub offset_x: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.VisualViewport/properties/offsetY。
    ///
    ///协议原文：
    ///
    ///<pre>Vertical offset relative to the layout viewport (CSS pixels).</pre>
    ///
    ///
    #[serde(rename = "offsetY")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub offset_y: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.VisualViewport/properties/pageX。
    ///
    ///协议原文：
    ///
    ///<pre>Horizontal offset relative to the document (CSS pixels).</pre>
    ///
    ///
    #[serde(rename = "pageX")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub page_x: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.VisualViewport/properties/pageY。
    ///
    ///协议原文：
    ///
    ///<pre>Vertical offset relative to the document (CSS pixels).</pre>
    ///
    ///
    #[serde(rename = "pageY")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub page_y: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.VisualViewport/properties/clientWidth。
    ///
    ///协议原文：
    ///
    ///<pre>Width (CSS pixels), excludes scrollbar if present.</pre>
    ///
    ///
    #[serde(rename = "clientWidth")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub client_width: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.VisualViewport/properties/clientHeight。
    ///
    ///协议原文：
    ///
    ///<pre>Height (CSS pixels), excludes scrollbar if present.</pre>
    ///
    ///
    #[serde(rename = "clientHeight")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub client_height: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.VisualViewport/properties/scale。
    ///
    ///协议原文：
    ///
    ///<pre>Scale relative to the ideal viewport (size at width=device-width).</pre>
    ///
    ///
    #[serde(rename = "scale")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub scale: ydp_core::FiniteF64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.VisualViewport/properties/zoom。
    ///
    ///协议原文：
    ///
    ///<pre>Page zoom factor (CSS to device independent pixels ratio).</pre>
    ///
    ///
    #[serde(rename = "zoom")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub zoom: Option<ydp_core::FiniteF64>,
}
impl<'de> serde::Deserialize<'de> for VisualViewport {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = VisualViewport;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.VisualViewport/properties/offsetX。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Horizontal offset relative to the layout viewport (CSS pixels).</pre>
                    ///
                    ///
                    #[serde(rename = "offsetX")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub offset_x: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.VisualViewport/properties/offsetY。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Vertical offset relative to the layout viewport (CSS pixels).</pre>
                    ///
                    ///
                    #[serde(rename = "offsetY")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub offset_y: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.VisualViewport/properties/pageX。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Horizontal offset relative to the document (CSS pixels).</pre>
                    ///
                    ///
                    #[serde(rename = "pageX")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub page_x: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.VisualViewport/properties/pageY。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Vertical offset relative to the document (CSS pixels).</pre>
                    ///
                    ///
                    #[serde(rename = "pageY")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub page_y: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.VisualViewport/properties/clientWidth。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Width (CSS pixels), excludes scrollbar if present.</pre>
                    ///
                    ///
                    #[serde(rename = "clientWidth")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub client_width: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.VisualViewport/properties/clientHeight。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Height (CSS pixels), excludes scrollbar if present.</pre>
                    ///
                    ///
                    #[serde(rename = "clientHeight")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub client_height: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Page.type.VisualViewport/properties/scale。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Scale relative to the ideal viewport (size at width=device-width).</pre>
                    ///
                    ///
                    #[serde(rename = "scale")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub scale: ydp_core::FiniteF64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.VisualViewport/properties/zoom。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Page zoom factor (CSS to device independent pixels ratio).</pre>
                    ///
                    ///
                    #[serde(rename = "zoom")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub zoom: Option<ydp_core::FiniteF64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(VisualViewport {
                    offset_x: fields.offset_x,
                    offset_y: fields.offset_y,
                    page_x: fields.page_x,
                    page_y: fields.page_y,
                    client_width: fields.client_width,
                    client_height: fields.client_height,
                    scale: fields.scale,
                    zoom: fields.zoom,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Page.type.WebAppManifest。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct WebAppManifest {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/backgroundColor。
    #[serde(rename = "backgroundColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub background_color: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/description。
    ///
    ///协议原文：
    ///
    ///<pre>The extra description provided by the manifest.</pre>
    ///
    ///
    #[serde(rename = "description")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/dir。
    #[serde(rename = "dir")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub dir: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/display。
    #[serde(rename = "display")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub display: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/displayOverrides。
    ///
    ///协议原文：
    ///
    ///<pre>The overrided display mode controlled by the user.</pre>
    ///
    ///
    #[serde(rename = "displayOverrides")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_overrides: Option<Vec<String>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/fileHandlers。
    ///
    ///协议原文：
    ///
    ///<pre>The handlers to open files.</pre>
    ///
    ///
    #[serde(rename = "fileHandlers")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub file_handlers: Option<Vec<crate::page::types::FileHandler>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/icons。
    #[serde(rename = "icons")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub icons: Option<Vec<crate::page::types::ImageResource>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/id。
    #[serde(rename = "id")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/lang。
    #[serde(rename = "lang")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub lang: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/launchHandler。
    ///
    ///协议原文：
    ///
    ///<pre>TODO(crbug.com/1231886): This field is non-standard and part of a Chrome
    ///experiment. See:
    ///https://github.com/WICG/web-app-launch/blob/main/launch_handler.md</pre>
    ///
    ///
    #[serde(rename = "launchHandler")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub launch_handler: Option<crate::page::types::LaunchHandler>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/name。
    #[serde(rename = "name")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/orientation。
    #[serde(rename = "orientation")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub orientation: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/preferRelatedApplications。
    #[serde(rename = "preferRelatedApplications")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub prefer_related_applications: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/protocolHandlers。
    ///
    ///协议原文：
    ///
    ///<pre>The handlers to open protocols.</pre>
    ///
    ///
    #[serde(rename = "protocolHandlers")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub protocol_handlers: Option<Vec<crate::page::types::ProtocolHandler>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/relatedApplications。
    #[serde(rename = "relatedApplications")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub related_applications: Option<Vec<crate::page::types::RelatedApplication>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/scope。
    #[serde(rename = "scope")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub scope: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/scopeExtensions。
    ///
    ///协议原文：
    ///
    ///<pre>Non-standard, see
    ///https://github.com/WICG/manifest-incubations/blob/gh-pages/scope_extensions-explainer.md</pre>
    ///
    ///
    #[serde(rename = "scopeExtensions")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub scope_extensions: Option<Vec<crate::page::types::ScopeExtension>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/screenshots。
    ///
    ///协议原文：
    ///
    ///<pre>The screenshots used by chromium.</pre>
    ///
    ///
    #[serde(rename = "screenshots")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub screenshots: Option<Vec<crate::page::types::Screenshot>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/shareTarget。
    #[serde(rename = "shareTarget")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub share_target: Option<crate::page::types::ShareTarget>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/shortName。
    #[serde(rename = "shortName")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub short_name: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/shortcuts。
    #[serde(rename = "shortcuts")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub shortcuts: Option<Vec<crate::page::types::Shortcut>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/startUrl。
    #[serde(rename = "startUrl")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub start_url: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/themeColor。
    #[serde(rename = "themeColor")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub theme_color: Option<String>,
}
impl<'de> serde::Deserialize<'de> for WebAppManifest {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WebAppManifest;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/backgroundColor。
                    #[serde(rename = "backgroundColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub background_color: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/description。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The extra description provided by the manifest.</pre>
                    ///
                    ///
                    #[serde(rename = "description")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub description: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/dir。
                    #[serde(rename = "dir")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub dir: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/display。
                    #[serde(rename = "display")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub display: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/displayOverrides。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The overrided display mode controlled by the user.</pre>
                    ///
                    ///
                    #[serde(rename = "displayOverrides")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub display_overrides: Option<Vec<String>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/fileHandlers。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The handlers to open files.</pre>
                    ///
                    ///
                    #[serde(rename = "fileHandlers")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub file_handlers: Option<Vec<crate::page::types::FileHandler>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/icons。
                    #[serde(rename = "icons")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub icons: Option<Vec<crate::page::types::ImageResource>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/id。
                    #[serde(rename = "id")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub id: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/lang。
                    #[serde(rename = "lang")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub lang: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/launchHandler。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>TODO(crbug.com/1231886): This field is non-standard and part of a Chrome
                    ///experiment. See:
                    ///https://github.com/WICG/web-app-launch/blob/main/launch_handler.md</pre>
                    ///
                    ///
                    #[serde(rename = "launchHandler")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub launch_handler: Option<crate::page::types::LaunchHandler>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/name。
                    #[serde(rename = "name")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub name: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/orientation。
                    #[serde(rename = "orientation")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub orientation: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/preferRelatedApplications。
                    #[serde(rename = "preferRelatedApplications")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub prefer_related_applications: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/protocolHandlers。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The handlers to open protocols.</pre>
                    ///
                    ///
                    #[serde(rename = "protocolHandlers")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub protocol_handlers: Option<Vec<crate::page::types::ProtocolHandler>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/relatedApplications。
                    #[serde(rename = "relatedApplications")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub related_applications: Option<Vec<crate::page::types::RelatedApplication>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/scope。
                    #[serde(rename = "scope")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub scope: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/scopeExtensions。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Non-standard, see
                    ///https://github.com/WICG/manifest-incubations/blob/gh-pages/scope_extensions-explainer.md</pre>
                    ///
                    ///
                    #[serde(rename = "scopeExtensions")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub scope_extensions: Option<Vec<crate::page::types::ScopeExtension>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/screenshots。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The screenshots used by chromium.</pre>
                    ///
                    ///
                    #[serde(rename = "screenshots")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub screenshots: Option<Vec<crate::page::types::Screenshot>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/shareTarget。
                    #[serde(rename = "shareTarget")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub share_target: Option<crate::page::types::ShareTarget>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/shortName。
                    #[serde(rename = "shortName")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub short_name: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/shortcuts。
                    #[serde(rename = "shortcuts")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub shortcuts: Option<Vec<crate::page::types::Shortcut>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/startUrl。
                    #[serde(rename = "startUrl")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub start_url: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Page.type.WebAppManifest/properties/themeColor。
                    #[serde(rename = "themeColor")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub theme_color: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(WebAppManifest {
                    background_color: fields.background_color,
                    description: fields.description,
                    dir: fields.dir,
                    display: fields.display,
                    display_overrides: fields.display_overrides,
                    file_handlers: fields.file_handlers,
                    icons: fields.icons,
                    id: fields.id,
                    lang: fields.lang,
                    launch_handler: fields.launch_handler,
                    name: fields.name,
                    orientation: fields.orientation,
                    prefer_related_applications: fields.prefer_related_applications,
                    protocol_handlers: fields.protocol_handlers,
                    related_applications: fields.related_applications,
                    scope: fields.scope,
                    scope_extensions: fields.scope_extensions,
                    screenshots: fields.screenshots,
                    share_target: fields.share_target,
                    short_name: fields.short_name,
                    shortcuts: fields.shortcuts,
                    start_url: fields.start_url,
                    theme_color: fields.theme_color,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
