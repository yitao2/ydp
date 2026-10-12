///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Security.command.disable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Security.command.disable。
///
///协议原文：
///
///<pre>Disables tracking security state changes.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Security.command.disable/returns。
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
    const METHOD: &'static str = "Security.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Security.command.enable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Security.command.enable。
///
///协议原文：
///
///<pre>Enables tracking security state changes.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Security.command.enable/returns。
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
    const METHOD: &'static str = "Security.enable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Security.command.handleCertificateError/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct HandleCertificateErrorParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.command.handleCertificateError/parameters/eventId。
    ///
    ///协议原文：
    ///
    ///<pre>The ID of the event.</pre>
    ///
    ///
    #[serde(rename = "eventId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub event_id: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.command.handleCertificateError/parameters/action。
    ///
    ///协议原文：
    ///
    ///<pre>The action to take on the certificate error.</pre>
    ///
    ///
    #[serde(rename = "action")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub action: crate::security::types::CertificateErrorAction,
}
impl<'de> serde::Deserialize<'de> for HandleCertificateErrorParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HandleCertificateErrorParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.command.handleCertificateError/parameters/eventId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The ID of the event.</pre>
                    ///
                    ///
                    #[serde(rename = "eventId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub event_id: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.command.handleCertificateError/parameters/action。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The action to take on the certificate error.</pre>
                    ///
                    ///
                    #[serde(rename = "action")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub action: crate::security::types::CertificateErrorAction,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(HandleCertificateErrorParams {
                    event_id: fields.event_id,
                    action: fields.action,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Security.command.handleCertificateError。
///
///协议原文：
///
///<pre>Handles a certificate error that fired a certificateError event.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct HandleCertificateError;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Security.command.handleCertificateError/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct HandleCertificateErrorResponse {}
impl<'de> serde::Deserialize<'de> for HandleCertificateErrorResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HandleCertificateErrorResponse;
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
                Ok(HandleCertificateErrorResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for HandleCertificateError {
    type Params = HandleCertificateErrorParams;
    type Response = HandleCertificateErrorResponse;
    const METHOD: &'static str = "Security.handleCertificateError";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Security.command.setIgnoreCertificateErrors/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetIgnoreCertificateErrorsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.command.setIgnoreCertificateErrors/parameters/ignore。
    ///
    ///协议原文：
    ///
    ///<pre>If true, all certificate errors will be ignored.</pre>
    ///
    ///
    #[serde(rename = "ignore")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub ignore: bool,
}
impl<'de> serde::Deserialize<'de> for SetIgnoreCertificateErrorsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetIgnoreCertificateErrorsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.command.setIgnoreCertificateErrors/parameters/ignore。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, all certificate errors will be ignored.</pre>
                    ///
                    ///
                    #[serde(rename = "ignore")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub ignore: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetIgnoreCertificateErrorsParams {
                    ignore: fields.ignore,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Security.command.setIgnoreCertificateErrors。
///
///协议原文：
///
///<pre>Enable/disable whether all certificate errors should be ignored.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetIgnoreCertificateErrors;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Security.command.setIgnoreCertificateErrors/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetIgnoreCertificateErrorsResponse {}
impl<'de> serde::Deserialize<'de> for SetIgnoreCertificateErrorsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetIgnoreCertificateErrorsResponse;
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
                Ok(SetIgnoreCertificateErrorsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetIgnoreCertificateErrors {
    type Params = SetIgnoreCertificateErrorsParams;
    type Response = SetIgnoreCertificateErrorsResponse;
    const METHOD: &'static str = "Security.setIgnoreCertificateErrors";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Security.command.setOverrideCertificateErrors/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetOverrideCertificateErrorsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.command.setOverrideCertificateErrors/parameters/override。
    ///
    ///协议原文：
    ///
    ///<pre>If true, certificate errors will be overridden.</pre>
    ///
    ///
    #[serde(rename = "override")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#override: bool,
}
impl<'de> serde::Deserialize<'de> for SetOverrideCertificateErrorsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetOverrideCertificateErrorsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Security.command.setOverrideCertificateErrors/parameters/override。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If true, certificate errors will be overridden.</pre>
                    ///
                    ///
                    #[serde(rename = "override")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#override: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetOverrideCertificateErrorsParams {
                    r#override: fields.r#override,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Security.command.setOverrideCertificateErrors。
///
///协议原文：
///
///<pre>Enable/disable overriding certificate errors. If enabled, all certificate error events need to
///be handled by the DevTools client and should be answered with `handleCertificateError` commands.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct SetOverrideCertificateErrors;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Security.command.setOverrideCertificateErrors/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetOverrideCertificateErrorsResponse {}
impl<'de> serde::Deserialize<'de> for SetOverrideCertificateErrorsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetOverrideCertificateErrorsResponse;
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
                Ok(SetOverrideCertificateErrorsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetOverrideCertificateErrors {
    type Params = SetOverrideCertificateErrorsParams;
    type Response = SetOverrideCertificateErrorsResponse;
    const METHOD: &'static str = "Security.setOverrideCertificateErrors";
}
