///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Autofill.command.disable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Autofill.command.disable。
///
///协议原文：
///
///<pre>Disables autofill domain notifications.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Autofill.command.disable/returns。
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
    const METHOD: &'static str = "Autofill.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Autofill.command.enable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Autofill.command.enable。
///
///协议原文：
///
///<pre>Enables autofill domain notifications.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Autofill.command.enable/returns。
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
    const METHOD: &'static str = "Autofill.enable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Autofill.command.setAddresses/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetAddressesParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.command.setAddresses/parameters/addresses。
    #[serde(rename = "addresses")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub addresses: Vec<crate::autofill::types::Address>,
}
impl<'de> serde::Deserialize<'de> for SetAddressesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetAddressesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.command.setAddresses/parameters/addresses。
                    #[serde(rename = "addresses")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub addresses: Vec<crate::autofill::types::Address>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetAddressesParams {
                    addresses: fields.addresses,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Autofill.command.setAddresses。
///
///协议原文：
///
///<pre>Set addresses so that developers can verify their forms implementation.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetAddresses;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Autofill.command.setAddresses/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetAddressesResponse {}
impl<'de> serde::Deserialize<'de> for SetAddressesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetAddressesResponse;
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
                Ok(SetAddressesResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetAddresses {
    type Params = SetAddressesParams;
    type Response = SetAddressesResponse;
    const METHOD: &'static str = "Autofill.setAddresses";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Autofill.command.trigger/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TriggerParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.command.trigger/parameters/fieldId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifies a field that serves as an anchor for autofill.</pre>
    ///
    ///
    #[serde(rename = "fieldId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub field_id: crate::dom::types::BackendNodeId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Autofill.command.trigger/parameters/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>Identifies the frame that field belongs to.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub frame_id: Option<crate::page::types::FrameId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Autofill.command.trigger/parameters/card。
    ///
    ///协议原文：
    ///
    ///<pre>Credit card information to fill out the form. Credit card data is not saved.  Mutually exclusive with `address`.</pre>
    ///
    ///
    #[serde(rename = "card")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub card: Option<crate::autofill::types::CreditCard>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Autofill.command.trigger/parameters/address。
    ///
    ///协议原文：
    ///
    ///<pre>Address to fill out the form. Address data is not saved. Mutually exclusive with `card`.</pre>
    ///
    ///
    #[serde(rename = "address")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub address: Option<crate::autofill::types::Address>,
}
impl<'de> serde::Deserialize<'de> for TriggerParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TriggerParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.command.trigger/parameters/fieldId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifies a field that serves as an anchor for autofill.</pre>
                    ///
                    ///
                    #[serde(rename = "fieldId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub field_id: crate::dom::types::BackendNodeId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Autofill.command.trigger/parameters/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Identifies the frame that field belongs to.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub frame_id: Option<crate::page::types::FrameId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Autofill.command.trigger/parameters/card。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Credit card information to fill out the form. Credit card data is not saved.  Mutually exclusive with `address`.</pre>
                    ///
                    ///
                    #[serde(rename = "card")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub card: Option<crate::autofill::types::CreditCard>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Autofill.command.trigger/parameters/address。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Address to fill out the form. Address data is not saved. Mutually exclusive with `card`.</pre>
                    ///
                    ///
                    #[serde(rename = "address")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub address: Option<crate::autofill::types::Address>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TriggerParams {
                    field_id: fields.field_id,
                    frame_id: fields.frame_id,
                    card: fields.card,
                    address: fields.address,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Autofill.command.trigger。
///
///协议原文：
///
///<pre>Trigger autofill on a form identified by the fieldId.
///If the field and related form cannot be autofilled, returns an error.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Trigger;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Autofill.command.trigger/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TriggerResponse {}
impl<'de> serde::Deserialize<'de> for TriggerResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TriggerResponse;
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
                Ok(TriggerResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Trigger {
    type Params = TriggerParams;
    type Response = TriggerResponse;
    const METHOD: &'static str = "Autofill.trigger";
}
