///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.command.disable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SmartCardEmulation.command.disable。
///
///协议原文：
///
///<pre>Disables the |SmartCardEmulation| domain.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.command.disable/returns。
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
    const METHOD: &'static str = "SmartCardEmulation.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.command.enable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SmartCardEmulation.command.enable。
///
///协议原文：
///
///<pre>Enables the |SmartCardEmulation| domain.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.command.enable/returns。
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
    const METHOD: &'static str = "SmartCardEmulation.enable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.command.reportBeginTransactionResult/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReportBeginTransactionResultParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportBeginTransactionResult/parameters/requestId。
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportBeginTransactionResult/parameters/handle。
    #[serde(rename = "handle")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub handle: i64,
}
impl<'de> serde::Deserialize<'de> for ReportBeginTransactionResultParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReportBeginTransactionResultParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportBeginTransactionResult/parameters/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportBeginTransactionResult/parameters/handle。
                    #[serde(rename = "handle")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub handle: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReportBeginTransactionResultParams {
                    request_id: fields.request_id,
                    handle: fields.handle,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SmartCardEmulation.command.reportBeginTransactionResult。
///
///协议原文：
///
///<pre>Reports the result of a |SCardBeginTransaction| call.
///On success, this creates a new transaction object.
///
///This maps to:
///PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gaddb835dce01a0da1d6ca02d33ee7d861
///Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardbegintransaction</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ReportBeginTransactionResult;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.command.reportBeginTransactionResult/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReportBeginTransactionResultResponse {}
impl<'de> serde::Deserialize<'de> for ReportBeginTransactionResultResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReportBeginTransactionResultResponse;
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
                Ok(ReportBeginTransactionResultResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ReportBeginTransactionResult {
    type Params = ReportBeginTransactionResultParams;
    type Response = ReportBeginTransactionResultResponse;
    const METHOD: &'static str = "SmartCardEmulation.reportBeginTransactionResult";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.command.reportConnectResult/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReportConnectResultParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportConnectResult/parameters/requestId。
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportConnectResult/parameters/handle。
    #[serde(rename = "handle")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub handle: i64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.command.reportConnectResult/parameters/activeProtocol。
    #[serde(rename = "activeProtocol")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub active_protocol: Option<crate::smart_card_emulation::types::Protocol>,
}
impl<'de> serde::Deserialize<'de> for ReportConnectResultParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReportConnectResultParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportConnectResult/parameters/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportConnectResult/parameters/handle。
                    #[serde(rename = "handle")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub handle: i64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.command.reportConnectResult/parameters/activeProtocol。
                    #[serde(rename = "activeProtocol")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub active_protocol: Option<crate::smart_card_emulation::types::Protocol>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReportConnectResultParams {
                    request_id: fields.request_id,
                    handle: fields.handle,
                    active_protocol: fields.active_protocol,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SmartCardEmulation.command.reportConnectResult。
///
///协议原文：
///
///<pre>Reports the successful result of a |SCardConnect| call.
///
///This maps to:
///PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga4e515829752e0a8dbc4d630696a8d6a5
///Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardconnecta</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ReportConnectResult;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.command.reportConnectResult/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReportConnectResultResponse {}
impl<'de> serde::Deserialize<'de> for ReportConnectResultResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReportConnectResultResponse;
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
                Ok(ReportConnectResultResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ReportConnectResult {
    type Params = ReportConnectResultParams;
    type Response = ReportConnectResultResponse;
    const METHOD: &'static str = "SmartCardEmulation.reportConnectResult";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.command.reportDataResult/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReportDataResultParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportDataResult/parameters/requestId。
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportDataResult/parameters/data。
    #[serde(rename = "data")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub data: String,
}
impl<'de> serde::Deserialize<'de> for ReportDataResultParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReportDataResultParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportDataResult/parameters/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportDataResult/parameters/data。
                    #[serde(rename = "data")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub data: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReportDataResultParams {
                    request_id: fields.request_id,
                    data: fields.data,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SmartCardEmulation.command.reportDataResult。
///
///协议原文：
///
///<pre>Reports the successful result of a call that sends back data on success.
///Used for |SCardTransmit|, |SCardControl|, and |SCardGetAttrib|.
///
///This maps to:
///1. SCardTransmit
///   PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga9a2d77242a271310269065e64633ab99
///   Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardtransmit
///
///2. SCardControl
///   PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gac3454d4657110fd7f753b2d3d8f4e32f
///   Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardcontrol
///
///3. SCardGetAttrib
///   PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gaacfec51917255b7a25b94c5104961602
///   Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardgetattrib</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ReportDataResult;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.command.reportDataResult/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReportDataResultResponse {}
impl<'de> serde::Deserialize<'de> for ReportDataResultResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReportDataResultResponse;
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
                Ok(ReportDataResultResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ReportDataResult {
    type Params = ReportDataResultParams;
    type Response = ReportDataResultResponse;
    const METHOD: &'static str = "SmartCardEmulation.reportDataResult";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.command.reportError/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReportErrorParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportError/parameters/requestId。
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportError/parameters/resultCode。
    #[serde(rename = "resultCode")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub result_code: crate::smart_card_emulation::types::ResultCode,
}
impl<'de> serde::Deserialize<'de> for ReportErrorParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReportErrorParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportError/parameters/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportError/parameters/resultCode。
                    #[serde(rename = "resultCode")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub result_code: crate::smart_card_emulation::types::ResultCode,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReportErrorParams {
                    request_id: fields.request_id,
                    result_code: fields.result_code,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SmartCardEmulation.command.reportError。
///
///协议原文：
///
///<pre>Reports an error result for the given request.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ReportError;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.command.reportError/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReportErrorResponse {}
impl<'de> serde::Deserialize<'de> for ReportErrorResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReportErrorResponse;
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
                Ok(ReportErrorResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ReportError {
    type Params = ReportErrorParams;
    type Response = ReportErrorResponse;
    const METHOD: &'static str = "SmartCardEmulation.reportError";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.command.reportEstablishContextResult/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReportEstablishContextResultParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportEstablishContextResult/parameters/requestId。
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportEstablishContextResult/parameters/contextId。
    #[serde(rename = "contextId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub context_id: i64,
}
impl<'de> serde::Deserialize<'de> for ReportEstablishContextResultParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReportEstablishContextResultParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportEstablishContextResult/parameters/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportEstablishContextResult/parameters/contextId。
                    #[serde(rename = "contextId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub context_id: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReportEstablishContextResultParams {
                    request_id: fields.request_id,
                    context_id: fields.context_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SmartCardEmulation.command.reportEstablishContextResult。
///
///协议原文：
///
///<pre>Reports the successful result of a |SCardEstablishContext| call.
///
///This maps to:
///PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gaa1b8970169fd4883a6dc4a8f43f19b67
///Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardestablishcontext</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ReportEstablishContextResult;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.command.reportEstablishContextResult/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReportEstablishContextResultResponse {}
impl<'de> serde::Deserialize<'de> for ReportEstablishContextResultResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReportEstablishContextResultResponse;
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
                Ok(ReportEstablishContextResultResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ReportEstablishContextResult {
    type Params = ReportEstablishContextResultParams;
    type Response = ReportEstablishContextResultResponse;
    const METHOD: &'static str = "SmartCardEmulation.reportEstablishContextResult";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.command.reportGetStatusChangeResult/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReportGetStatusChangeResultParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportGetStatusChangeResult/parameters/requestId。
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportGetStatusChangeResult/parameters/readerStates。
    #[serde(rename = "readerStates")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub reader_states: Vec<crate::smart_card_emulation::types::ReaderStateOut>,
}
impl<'de> serde::Deserialize<'de> for ReportGetStatusChangeResultParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReportGetStatusChangeResultParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportGetStatusChangeResult/parameters/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportGetStatusChangeResult/parameters/readerStates。
                    #[serde(rename = "readerStates")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub reader_states: Vec<crate::smart_card_emulation::types::ReaderStateOut>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReportGetStatusChangeResultParams {
                    request_id: fields.request_id,
                    reader_states: fields.reader_states,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SmartCardEmulation.command.reportGetStatusChangeResult。
///
///协议原文：
///
///<pre>Reports the successful result of a |SCardGetStatusChange| call.
///
///This maps to:
///PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga33247d5d1257d59e55647c3bb717db24
///Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardgetstatuschangea</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ReportGetStatusChangeResult;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.command.reportGetStatusChangeResult/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReportGetStatusChangeResultResponse {}
impl<'de> serde::Deserialize<'de> for ReportGetStatusChangeResultResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReportGetStatusChangeResultResponse;
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
                Ok(ReportGetStatusChangeResultResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ReportGetStatusChangeResult {
    type Params = ReportGetStatusChangeResultParams;
    type Response = ReportGetStatusChangeResultResponse;
    const METHOD: &'static str = "SmartCardEmulation.reportGetStatusChangeResult";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.command.reportListReadersResult/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReportListReadersResultParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportListReadersResult/parameters/requestId。
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportListReadersResult/parameters/readers。
    #[serde(rename = "readers")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub readers: Vec<String>,
}
impl<'de> serde::Deserialize<'de> for ReportListReadersResultParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReportListReadersResultParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportListReadersResult/parameters/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportListReadersResult/parameters/readers。
                    #[serde(rename = "readers")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub readers: Vec<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReportListReadersResultParams {
                    request_id: fields.request_id,
                    readers: fields.readers,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SmartCardEmulation.command.reportListReadersResult。
///
///协议原文：
///
///<pre>Reports the successful result of a |SCardListReaders| call.
///
///This maps to:
///PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga93b07815789b3cf2629d439ecf20f0d9
///Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardlistreadersa</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ReportListReadersResult;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.command.reportListReadersResult/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReportListReadersResultResponse {}
impl<'de> serde::Deserialize<'de> for ReportListReadersResultResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReportListReadersResultResponse;
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
                Ok(ReportListReadersResultResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ReportListReadersResult {
    type Params = ReportListReadersResultParams;
    type Response = ReportListReadersResultResponse;
    const METHOD: &'static str = "SmartCardEmulation.reportListReadersResult";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.command.reportPlainResult/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReportPlainResultParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportPlainResult/parameters/requestId。
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: String,
}
impl<'de> serde::Deserialize<'de> for ReportPlainResultParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReportPlainResultParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportPlainResult/parameters/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReportPlainResultParams {
                    request_id: fields.request_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SmartCardEmulation.command.reportPlainResult。
///
///协议原文：
///
///<pre>Reports the successful result of a call that returns only a result code.
///Used for: |SCardCancel|, |SCardDisconnect|, |SCardSetAttrib|, |SCardEndTransaction|.
///
///This maps to:
///1. SCardCancel
///   PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gaacbbc0c6d6c0cbbeb4f4debf6fbeeee6
///   Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardcancel
///
///2. SCardDisconnect
///   PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga4be198045c73ec0deb79e66c0ca1738a
///   Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scarddisconnect
///
///3. SCardSetAttrib
///   PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga060f0038a4ddfd5dd2b8fadf3c3a2e4f
///   Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardsetattrib
///
///4. SCardEndTransaction
///   PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gae8742473b404363e5c587f570d7e2f3b
///   Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardendtransaction</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ReportPlainResult;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.command.reportPlainResult/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReportPlainResultResponse {}
impl<'de> serde::Deserialize<'de> for ReportPlainResultResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReportPlainResultResponse;
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
                Ok(ReportPlainResultResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ReportPlainResult {
    type Params = ReportPlainResultParams;
    type Response = ReportPlainResultResponse;
    const METHOD: &'static str = "SmartCardEmulation.reportPlainResult";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.command.reportReleaseContextResult/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReportReleaseContextResultParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportReleaseContextResult/parameters/requestId。
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: String,
}
impl<'de> serde::Deserialize<'de> for ReportReleaseContextResultParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReportReleaseContextResultParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportReleaseContextResult/parameters/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReportReleaseContextResultParams {
                    request_id: fields.request_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SmartCardEmulation.command.reportReleaseContextResult。
///
///协议原文：
///
///<pre>Reports the successful result of a |SCardReleaseContext| call.
///
///This maps to:
///PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga6aabcba7744c5c9419fdd6404f73a934
///Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardreleasecontext</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ReportReleaseContextResult;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.command.reportReleaseContextResult/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReportReleaseContextResultResponse {}
impl<'de> serde::Deserialize<'de> for ReportReleaseContextResultResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReportReleaseContextResultResponse;
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
                Ok(ReportReleaseContextResultResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ReportReleaseContextResult {
    type Params = ReportReleaseContextResultParams;
    type Response = ReportReleaseContextResultResponse;
    const METHOD: &'static str = "SmartCardEmulation.reportReleaseContextResult";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.command.reportStatusResult/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReportStatusResultParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportStatusResult/parameters/requestId。
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportStatusResult/parameters/readerName。
    #[serde(rename = "readerName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub reader_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportStatusResult/parameters/state。
    #[serde(rename = "state")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub state: crate::smart_card_emulation::types::ConnectionState,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportStatusResult/parameters/atr。
    #[serde(rename = "atr")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub atr: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.command.reportStatusResult/parameters/protocol。
    #[serde(rename = "protocol")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub protocol: Option<crate::smart_card_emulation::types::Protocol>,
}
impl<'de> serde::Deserialize<'de> for ReportStatusResultParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReportStatusResultParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportStatusResult/parameters/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportStatusResult/parameters/readerName。
                    #[serde(rename = "readerName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub reader_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportStatusResult/parameters/state。
                    #[serde(rename = "state")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub state: crate::smart_card_emulation::types::ConnectionState,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.command.reportStatusResult/parameters/atr。
                    #[serde(rename = "atr")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub atr: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.command.reportStatusResult/parameters/protocol。
                    #[serde(rename = "protocol")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub protocol: Option<crate::smart_card_emulation::types::Protocol>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReportStatusResultParams {
                    request_id: fields.request_id,
                    reader_name: fields.reader_name,
                    state: fields.state,
                    atr: fields.atr,
                    protocol: fields.protocol,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SmartCardEmulation.command.reportStatusResult。
///
///协议原文：
///
///<pre>Reports the successful result of a |SCardStatus| call.
///
///This maps to:
///PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gae49c3c894ad7ac12a5b896bde70d0382
///Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardstatusa</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ReportStatusResult;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.command.reportStatusResult/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReportStatusResultResponse {}
impl<'de> serde::Deserialize<'de> for ReportStatusResultResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReportStatusResultResponse;
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
                Ok(ReportStatusResultResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ReportStatusResult {
    type Params = ReportStatusResultParams;
    type Response = ReportStatusResultResponse;
    const METHOD: &'static str = "SmartCardEmulation.reportStatusResult";
}
