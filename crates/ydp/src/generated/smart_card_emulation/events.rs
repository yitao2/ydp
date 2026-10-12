///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.event.beginTransactionRequested/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct BeginTransactionRequestedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.beginTransactionRequested/parameters/requestId。
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.beginTransactionRequested/parameters/handle。
    #[serde(rename = "handle")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub handle: i64,
}
impl<'de> serde::Deserialize<'de> for BeginTransactionRequestedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BeginTransactionRequestedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.beginTransactionRequested/parameters/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.beginTransactionRequested/parameters/handle。
                    #[serde(rename = "handle")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub handle: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(BeginTransactionRequestedParams {
                    request_id: fields.request_id,
                    handle: fields.handle,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SmartCardEmulation.event.beginTransactionRequested。
///
///协议原文：
///
///<pre>Fired when |SCardBeginTransaction| is called.
///
///This maps to:
///PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gaddb835dce01a0da1d6ca02d33ee7d861
///Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardbegintransaction</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct BeginTransactionRequested;
impl ydp_core::Event for BeginTransactionRequested {
    type Params = BeginTransactionRequestedParams;
    const METHOD: &'static str = "SmartCardEmulation.beginTransactionRequested";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.event.cancelRequested/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CancelRequestedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.cancelRequested/parameters/requestId。
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.cancelRequested/parameters/contextId。
    #[serde(rename = "contextId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub context_id: i64,
}
impl<'de> serde::Deserialize<'de> for CancelRequestedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CancelRequestedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.cancelRequested/parameters/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.cancelRequested/parameters/contextId。
                    #[serde(rename = "contextId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub context_id: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CancelRequestedParams {
                    request_id: fields.request_id,
                    context_id: fields.context_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SmartCardEmulation.event.cancelRequested。
///
///协议原文：
///
///<pre>Fired when |SCardCancel| is called.
///
///This maps to:
///PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gaacbbc0c6d6c0cbbeb4f4debf6fbeeee6
///Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardcancel</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct CancelRequested;
impl ydp_core::Event for CancelRequested {
    type Params = CancelRequestedParams;
    const METHOD: &'static str = "SmartCardEmulation.cancelRequested";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.event.connectRequested/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ConnectRequestedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.connectRequested/parameters/requestId。
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.connectRequested/parameters/contextId。
    #[serde(rename = "contextId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub context_id: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.connectRequested/parameters/reader。
    #[serde(rename = "reader")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub reader: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.connectRequested/parameters/shareMode。
    #[serde(rename = "shareMode")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub share_mode: crate::smart_card_emulation::types::ShareMode,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.connectRequested/parameters/preferredProtocols。
    #[serde(rename = "preferredProtocols")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub preferred_protocols: crate::smart_card_emulation::types::ProtocolSet,
}
impl<'de> serde::Deserialize<'de> for ConnectRequestedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ConnectRequestedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.connectRequested/parameters/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.connectRequested/parameters/contextId。
                    #[serde(rename = "contextId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub context_id: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.connectRequested/parameters/reader。
                    #[serde(rename = "reader")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub reader: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.connectRequested/parameters/shareMode。
                    #[serde(rename = "shareMode")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub share_mode: crate::smart_card_emulation::types::ShareMode,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.connectRequested/parameters/preferredProtocols。
                    #[serde(rename = "preferredProtocols")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub preferred_protocols: crate::smart_card_emulation::types::ProtocolSet,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ConnectRequestedParams {
                    request_id: fields.request_id,
                    context_id: fields.context_id,
                    reader: fields.reader,
                    share_mode: fields.share_mode,
                    preferred_protocols: fields.preferred_protocols,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SmartCardEmulation.event.connectRequested。
///
///协议原文：
///
///<pre>Fired when |SCardConnect| is called.
///
///This maps to:
///PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga4e515829752e0a8dbc4d630696a8d6a5
///Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardconnecta</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ConnectRequested;
impl ydp_core::Event for ConnectRequested {
    type Params = ConnectRequestedParams;
    const METHOD: &'static str = "SmartCardEmulation.connectRequested";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.event.controlRequested/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ControlRequestedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.controlRequested/parameters/requestId。
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.controlRequested/parameters/handle。
    #[serde(rename = "handle")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub handle: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.controlRequested/parameters/controlCode。
    #[serde(rename = "controlCode")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub control_code: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.controlRequested/parameters/data。
    #[serde(rename = "data")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub data: String,
}
impl<'de> serde::Deserialize<'de> for ControlRequestedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ControlRequestedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.controlRequested/parameters/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.controlRequested/parameters/handle。
                    #[serde(rename = "handle")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub handle: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.controlRequested/parameters/controlCode。
                    #[serde(rename = "controlCode")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub control_code: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.controlRequested/parameters/data。
                    #[serde(rename = "data")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub data: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ControlRequestedParams {
                    request_id: fields.request_id,
                    handle: fields.handle,
                    control_code: fields.control_code,
                    data: fields.data,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SmartCardEmulation.event.controlRequested。
///
///协议原文：
///
///<pre>Fired when |SCardControl| is called.
///
///This maps to:
///PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gac3454d4657110fd7f753b2d3d8f4e32f
///Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardcontrol</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ControlRequested;
impl ydp_core::Event for ControlRequested {
    type Params = ControlRequestedParams;
    const METHOD: &'static str = "SmartCardEmulation.controlRequested";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.event.disconnectRequested/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DisconnectRequestedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.disconnectRequested/parameters/requestId。
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.disconnectRequested/parameters/handle。
    #[serde(rename = "handle")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub handle: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.disconnectRequested/parameters/disposition。
    #[serde(rename = "disposition")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub disposition: crate::smart_card_emulation::types::Disposition,
}
impl<'de> serde::Deserialize<'de> for DisconnectRequestedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DisconnectRequestedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.disconnectRequested/parameters/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.disconnectRequested/parameters/handle。
                    #[serde(rename = "handle")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub handle: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.disconnectRequested/parameters/disposition。
                    #[serde(rename = "disposition")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub disposition: crate::smart_card_emulation::types::Disposition,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DisconnectRequestedParams {
                    request_id: fields.request_id,
                    handle: fields.handle,
                    disposition: fields.disposition,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SmartCardEmulation.event.disconnectRequested。
///
///协议原文：
///
///<pre>Fired when |SCardDisconnect| is called.
///
///This maps to:
///PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga4be198045c73ec0deb79e66c0ca1738a
///Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scarddisconnect</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct DisconnectRequested;
impl ydp_core::Event for DisconnectRequested {
    type Params = DisconnectRequestedParams;
    const METHOD: &'static str = "SmartCardEmulation.disconnectRequested";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.event.endTransactionRequested/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EndTransactionRequestedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.endTransactionRequested/parameters/requestId。
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.endTransactionRequested/parameters/handle。
    #[serde(rename = "handle")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub handle: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.endTransactionRequested/parameters/disposition。
    #[serde(rename = "disposition")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub disposition: crate::smart_card_emulation::types::Disposition,
}
impl<'de> serde::Deserialize<'de> for EndTransactionRequestedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EndTransactionRequestedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.endTransactionRequested/parameters/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.endTransactionRequested/parameters/handle。
                    #[serde(rename = "handle")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub handle: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.endTransactionRequested/parameters/disposition。
                    #[serde(rename = "disposition")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub disposition: crate::smart_card_emulation::types::Disposition,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EndTransactionRequestedParams {
                    request_id: fields.request_id,
                    handle: fields.handle,
                    disposition: fields.disposition,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SmartCardEmulation.event.endTransactionRequested。
///
///协议原文：
///
///<pre>Fired when |SCardEndTransaction| is called.
///
///This maps to:
///PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gae8742473b404363e5c587f570d7e2f3b
///Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardendtransaction</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct EndTransactionRequested;
impl ydp_core::Event for EndTransactionRequested {
    type Params = EndTransactionRequestedParams;
    const METHOD: &'static str = "SmartCardEmulation.endTransactionRequested";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.event.establishContextRequested/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct EstablishContextRequestedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.establishContextRequested/parameters/requestId。
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: String,
}
impl<'de> serde::Deserialize<'de> for EstablishContextRequestedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EstablishContextRequestedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.establishContextRequested/parameters/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EstablishContextRequestedParams {
                    request_id: fields.request_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SmartCardEmulation.event.establishContextRequested。
///
///协议原文：
///
///<pre>Fired when |SCardEstablishContext| is called.
///
///This maps to:
///PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gaa1b8970169fd4883a6dc4a8f43f19b67
///Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardestablishcontext</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct EstablishContextRequested;
impl ydp_core::Event for EstablishContextRequested {
    type Params = EstablishContextRequestedParams;
    const METHOD: &'static str = "SmartCardEmulation.establishContextRequested";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.event.getAttribRequested/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetAttribRequestedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.getAttribRequested/parameters/requestId。
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.getAttribRequested/parameters/handle。
    #[serde(rename = "handle")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub handle: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.getAttribRequested/parameters/attribId。
    #[serde(rename = "attribId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub attrib_id: i64,
}
impl<'de> serde::Deserialize<'de> for GetAttribRequestedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetAttribRequestedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.getAttribRequested/parameters/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.getAttribRequested/parameters/handle。
                    #[serde(rename = "handle")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub handle: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.getAttribRequested/parameters/attribId。
                    #[serde(rename = "attribId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub attrib_id: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetAttribRequestedParams {
                    request_id: fields.request_id,
                    handle: fields.handle,
                    attrib_id: fields.attrib_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SmartCardEmulation.event.getAttribRequested。
///
///协议原文：
///
///<pre>Fired when |SCardGetAttrib| is called.
///
///This maps to:
///PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gaacfec51917255b7a25b94c5104961602
///Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardgetattrib</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetAttribRequested;
impl ydp_core::Event for GetAttribRequested {
    type Params = GetAttribRequestedParams;
    const METHOD: &'static str = "SmartCardEmulation.getAttribRequested";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.event.getStatusChangeRequested/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetStatusChangeRequestedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.getStatusChangeRequested/parameters/requestId。
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.getStatusChangeRequested/parameters/contextId。
    #[serde(rename = "contextId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub context_id: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.getStatusChangeRequested/parameters/readerStates。
    #[serde(rename = "readerStates")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub reader_states: Vec<crate::smart_card_emulation::types::ReaderStateIn>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.event.getStatusChangeRequested/parameters/timeout。
    ///
    ///协议原文：
    ///
    ///<pre>in milliseconds, if absent, it means "infinite"</pre>
    ///
    ///
    #[serde(rename = "timeout")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub timeout: Option<i64>,
}
impl<'de> serde::Deserialize<'de> for GetStatusChangeRequestedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetStatusChangeRequestedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.getStatusChangeRequested/parameters/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.getStatusChangeRequested/parameters/contextId。
                    #[serde(rename = "contextId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub context_id: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.getStatusChangeRequested/parameters/readerStates。
                    #[serde(rename = "readerStates")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub reader_states: Vec<crate::smart_card_emulation::types::ReaderStateIn>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.event.getStatusChangeRequested/parameters/timeout。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>in milliseconds, if absent, it means "infinite"</pre>
                    ///
                    ///
                    #[serde(rename = "timeout")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub timeout: Option<i64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetStatusChangeRequestedParams {
                    request_id: fields.request_id,
                    context_id: fields.context_id,
                    reader_states: fields.reader_states,
                    timeout: fields.timeout,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SmartCardEmulation.event.getStatusChangeRequested。
///
///协议原文：
///
///<pre>Fired when |SCardGetStatusChange| is called. Timeout is specified in milliseconds.
///
///This maps to:
///PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga33247d5d1257d59e55647c3bb717db24
///Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardgetstatuschangea</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetStatusChangeRequested;
impl ydp_core::Event for GetStatusChangeRequested {
    type Params = GetStatusChangeRequestedParams;
    const METHOD: &'static str = "SmartCardEmulation.getStatusChangeRequested";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.event.listReadersRequested/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ListReadersRequestedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.listReadersRequested/parameters/requestId。
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.listReadersRequested/parameters/contextId。
    #[serde(rename = "contextId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub context_id: i64,
}
impl<'de> serde::Deserialize<'de> for ListReadersRequestedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ListReadersRequestedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.listReadersRequested/parameters/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.listReadersRequested/parameters/contextId。
                    #[serde(rename = "contextId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub context_id: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ListReadersRequestedParams {
                    request_id: fields.request_id,
                    context_id: fields.context_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SmartCardEmulation.event.listReadersRequested。
///
///协议原文：
///
///<pre>Fired when |SCardListReaders| is called.
///
///This maps to:
///PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga93b07815789b3cf2629d439ecf20f0d9
///Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardlistreadersa</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ListReadersRequested;
impl ydp_core::Event for ListReadersRequested {
    type Params = ListReadersRequestedParams;
    const METHOD: &'static str = "SmartCardEmulation.listReadersRequested";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.event.releaseContextRequested/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReleaseContextRequestedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.releaseContextRequested/parameters/requestId。
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.releaseContextRequested/parameters/contextId。
    #[serde(rename = "contextId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub context_id: i64,
}
impl<'de> serde::Deserialize<'de> for ReleaseContextRequestedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReleaseContextRequestedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.releaseContextRequested/parameters/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.releaseContextRequested/parameters/contextId。
                    #[serde(rename = "contextId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub context_id: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReleaseContextRequestedParams {
                    request_id: fields.request_id,
                    context_id: fields.context_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SmartCardEmulation.event.releaseContextRequested。
///
///协议原文：
///
///<pre>Fired when |SCardReleaseContext| is called.
///
///This maps to:
///PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga6aabcba7744c5c9419fdd6404f73a934
///Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardreleasecontext</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ReleaseContextRequested;
impl ydp_core::Event for ReleaseContextRequested {
    type Params = ReleaseContextRequestedParams;
    const METHOD: &'static str = "SmartCardEmulation.releaseContextRequested";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.event.setAttribRequested/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetAttribRequestedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.setAttribRequested/parameters/requestId。
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.setAttribRequested/parameters/handle。
    #[serde(rename = "handle")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub handle: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.setAttribRequested/parameters/attribId。
    #[serde(rename = "attribId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub attrib_id: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.setAttribRequested/parameters/data。
    #[serde(rename = "data")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub data: String,
}
impl<'de> serde::Deserialize<'de> for SetAttribRequestedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetAttribRequestedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.setAttribRequested/parameters/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.setAttribRequested/parameters/handle。
                    #[serde(rename = "handle")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub handle: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.setAttribRequested/parameters/attribId。
                    #[serde(rename = "attribId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub attrib_id: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.setAttribRequested/parameters/data。
                    #[serde(rename = "data")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub data: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetAttribRequestedParams {
                    request_id: fields.request_id,
                    handle: fields.handle,
                    attrib_id: fields.attrib_id,
                    data: fields.data,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SmartCardEmulation.event.setAttribRequested。
///
///协议原文：
///
///<pre>Fired when |SCardSetAttrib| is called.
///
///This maps to:
///PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga060f0038a4ddfd5dd2b8fadf3c3a2e4f
///Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardsetattrib</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetAttribRequested;
impl ydp_core::Event for SetAttribRequested {
    type Params = SetAttribRequestedParams;
    const METHOD: &'static str = "SmartCardEmulation.setAttribRequested";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.event.statusRequested/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct StatusRequestedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.statusRequested/parameters/requestId。
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.statusRequested/parameters/handle。
    #[serde(rename = "handle")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub handle: i64,
}
impl<'de> serde::Deserialize<'de> for StatusRequestedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = StatusRequestedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.statusRequested/parameters/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.statusRequested/parameters/handle。
                    #[serde(rename = "handle")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub handle: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(StatusRequestedParams {
                    request_id: fields.request_id,
                    handle: fields.handle,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SmartCardEmulation.event.statusRequested。
///
///协议原文：
///
///<pre>Fired when |SCardStatus| is called.
///
///This maps to:
///PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gae49c3c894ad7ac12a5b896bde70d0382
///Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardstatusa</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct StatusRequested;
impl ydp_core::Event for StatusRequested {
    type Params = StatusRequestedParams;
    const METHOD: &'static str = "SmartCardEmulation.statusRequested";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SmartCardEmulation.event.transmitRequested/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TransmitRequestedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.transmitRequested/parameters/requestId。
    #[serde(rename = "requestId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.transmitRequested/parameters/handle。
    #[serde(rename = "handle")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub handle: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.transmitRequested/parameters/data。
    #[serde(rename = "data")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub data: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.event.transmitRequested/parameters/protocol。
    #[serde(rename = "protocol")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub protocol: Option<crate::smart_card_emulation::types::Protocol>,
}
impl<'de> serde::Deserialize<'de> for TransmitRequestedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TransmitRequestedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.transmitRequested/parameters/requestId。
                    #[serde(rename = "requestId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.transmitRequested/parameters/handle。
                    #[serde(rename = "handle")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub handle: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SmartCardEmulation.event.transmitRequested/parameters/data。
                    #[serde(rename = "data")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub data: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：SmartCardEmulation.event.transmitRequested/parameters/protocol。
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
                Ok(TransmitRequestedParams {
                    request_id: fields.request_id,
                    handle: fields.handle,
                    data: fields.data,
                    protocol: fields.protocol,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SmartCardEmulation.event.transmitRequested。
///
///协议原文：
///
///<pre>Fired when |SCardTransmit| is called.
///
///This maps to:
///PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga9a2d77242a271310269065e64633ab99
///Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardtransmit</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct TransmitRequested;
impl ydp_core::Event for TransmitRequested {
    type Params = TransmitRequestedParams;
    const METHOD: &'static str = "SmartCardEmulation.transmitRequested";
}
