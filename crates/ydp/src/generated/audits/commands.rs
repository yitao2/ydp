///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.command.checkFormsIssues/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CheckFormsIssuesParams {}
impl<'de> serde::Deserialize<'de> for CheckFormsIssuesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CheckFormsIssuesParams;
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
                Ok(CheckFormsIssuesParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Audits.command.checkFormsIssues。
///
///协议原文：
///
///<pre>Runs the form issues check for the target page. Found issues are reported
///using Audits.issueAdded event.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct CheckFormsIssues;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.command.checkFormsIssues/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CheckFormsIssuesResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.command.checkFormsIssues/returns/formIssues。
    #[serde(rename = "formIssues")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub form_issues: Vec<crate::audits::types::GenericIssueDetails>,
}
impl<'de> serde::Deserialize<'de> for CheckFormsIssuesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CheckFormsIssuesResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.command.checkFormsIssues/returns/formIssues。
                    #[serde(rename = "formIssues")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub form_issues: Vec<crate::audits::types::GenericIssueDetails>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CheckFormsIssuesResponse {
                    form_issues: fields.form_issues,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for CheckFormsIssues {
    type Params = CheckFormsIssuesParams;
    type Response = CheckFormsIssuesResponse;
    const METHOD: &'static str = "Audits.checkFormsIssues";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.command.disable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Audits.command.disable。
///
///协议原文：
///
///<pre>Disables issues domain, prevents further issues from being reported to the client.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.command.disable/returns。
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
    const METHOD: &'static str = "Audits.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.command.enable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Audits.command.enable。
///
///协议原文：
///
///<pre>Enables issues domain, sends the issues collected so far to the client by means of the
///`issueAdded` event.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.command.enable/returns。
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
    const METHOD: &'static str = "Audits.enable";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Audits.command.getEncodedResponse/parameters/encoding。
///
///协议原文：
///
///<pre>The encoding to use.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum GetEncodedResponseParamsEncoding {
    ///协议值 "webp"。
    Webp,
    ///协议值 "jpeg"。
    Jpeg,
    ///协议值 "png"。
    Png,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl GetEncodedResponseParamsEncoding {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::audits::commands::GetEncodedResponseParamsEncoding::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Webp => "webp",
            Self::Jpeg => "jpeg",
            Self::Png => "png",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for GetEncodedResponseParamsEncoding {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for GetEncodedResponseParamsEncoding {
    fn from(value: String) -> Self {
        match value.as_str() {
            "webp" => GetEncodedResponseParamsEncoding::Webp,
            "jpeg" => GetEncodedResponseParamsEncoding::Jpeg,
            "png" => GetEncodedResponseParamsEncoding::Png,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for GetEncodedResponseParamsEncoding {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GetEncodedResponseParamsEncoding {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetEncodedResponseParamsEncoding;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "webp" => GetEncodedResponseParamsEncoding::Webp,
                    "jpeg" => GetEncodedResponseParamsEncoding::Jpeg,
                    "png" => GetEncodedResponseParamsEncoding::Png,
                    _ => GetEncodedResponseParamsEncoding::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(GetEncodedResponseParamsEncoding::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.command.getEncodedResponse/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetEncodedResponseParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.command.getEncodedResponse/parameters/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifier of the network request to get content for.</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: crate::network::types::RequestId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.command.getEncodedResponse/parameters/encoding。
    ///
    ///协议原文：
    ///
    ///<pre>The encoding to use.</pre>
    ///
    ///
    #[serde(rename = "encoding")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub encoding: GetEncodedResponseParamsEncoding,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.command.getEncodedResponse/parameters/quality。
    ///
    ///协议原文：
    ///
    ///<pre>The quality of the encoding (0-1). (defaults to 1)</pre>
    ///
    ///
    #[serde(rename = "quality")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub quality: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.command.getEncodedResponse/parameters/sizeOnly。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to only return the size information (defaults to false).</pre>
    ///
    ///
    #[serde(rename = "sizeOnly")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub size_only: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for GetEncodedResponseParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetEncodedResponseParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.command.getEncodedResponse/parameters/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifier of the network request to get content for.</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: crate::network::types::RequestId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.command.getEncodedResponse/parameters/encoding。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The encoding to use.</pre>
                    ///
                    ///
                    #[serde(rename = "encoding")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub encoding: GetEncodedResponseParamsEncoding,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.command.getEncodedResponse/parameters/quality。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The quality of the encoding (0-1). (defaults to 1)</pre>
                    ///
                    ///
                    #[serde(rename = "quality")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub quality: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.command.getEncodedResponse/parameters/sizeOnly。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to only return the size information (defaults to false).</pre>
                    ///
                    ///
                    #[serde(rename = "sizeOnly")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub size_only: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetEncodedResponseParams {
                    request_id: fields.request_id,
                    encoding: fields.encoding,
                    quality: fields.quality,
                    size_only: fields.size_only,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Audits.command.getEncodedResponse。
///
///协议原文：
///
///<pre>Returns the response body and size if it were re-encoded with the specified settings. Only
///applies to images.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetEncodedResponse;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Audits.command.getEncodedResponse/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetEncodedResponseResponse {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.command.getEncodedResponse/returns/body。
    ///
    ///协议原文：
    ///
    ///<pre>The encoded body as a base64 string. Omitted if sizeOnly is true. (Encoded as a base64 string when passed over JSON)</pre>
    ///
    ///
    #[serde(rename = "body")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub body: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.command.getEncodedResponse/returns/originalSize。
    ///
    ///协议原文：
    ///
    ///<pre>Size before re-encoding.</pre>
    ///
    ///
    #[serde(rename = "originalSize")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub original_size: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.command.getEncodedResponse/returns/encodedSize。
    ///
    ///协议原文：
    ///
    ///<pre>Size after re-encoding.</pre>
    ///
    ///
    #[serde(rename = "encodedSize")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub encoded_size: i64,
}
impl<'de> serde::Deserialize<'de> for GetEncodedResponseResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetEncodedResponseResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Audits.command.getEncodedResponse/returns/body。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The encoded body as a base64 string. Omitted if sizeOnly is true. (Encoded as a base64 string when passed over JSON)</pre>
                    ///
                    ///
                    #[serde(rename = "body")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub body: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.command.getEncodedResponse/returns/originalSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Size before re-encoding.</pre>
                    ///
                    ///
                    #[serde(rename = "originalSize")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub original_size: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Audits.command.getEncodedResponse/returns/encodedSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Size after re-encoding.</pre>
                    ///
                    ///
                    #[serde(rename = "encodedSize")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub encoded_size: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetEncodedResponseResponse {
                    body: fields.body,
                    original_size: fields.original_size,
                    encoded_size: fields.encoded_size,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetEncodedResponse {
    type Params = GetEncodedResponseParams;
    type Response = GetEncodedResponseResponse;
    const METHOD: &'static str = "Audits.getEncodedResponse";
}
