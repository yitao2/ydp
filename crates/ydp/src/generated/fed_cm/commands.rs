///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：FedCm.command.clickDialogButton/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClickDialogButtonParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.command.clickDialogButton/parameters/dialogId。
    #[serde(rename = "dialogId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub dialog_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.command.clickDialogButton/parameters/dialogButton。
    #[serde(rename = "dialogButton")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub dialog_button: crate::fed_cm::types::DialogButton,
}
impl<'de> serde::Deserialize<'de> for ClickDialogButtonParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClickDialogButtonParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.command.clickDialogButton/parameters/dialogId。
                    #[serde(rename = "dialogId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub dialog_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.command.clickDialogButton/parameters/dialogButton。
                    #[serde(rename = "dialogButton")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub dialog_button: crate::fed_cm::types::DialogButton,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ClickDialogButtonParams {
                    dialog_id: fields.dialog_id,
                    dialog_button: fields.dialog_button,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：FedCm.command.clickDialogButton。
#[derive(Debug, Clone, Copy)]
pub struct ClickDialogButton;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：FedCm.command.clickDialogButton/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClickDialogButtonResponse {}
impl<'de> serde::Deserialize<'de> for ClickDialogButtonResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClickDialogButtonResponse;
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
                Ok(ClickDialogButtonResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ClickDialogButton {
    type Params = ClickDialogButtonParams;
    type Response = ClickDialogButtonResponse;
    const METHOD: &'static str = "FedCm.clickDialogButton";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：FedCm.command.disable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：FedCm.command.disable。
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：FedCm.command.disable/returns。
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
    const METHOD: &'static str = "FedCm.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：FedCm.command.dismissDialog/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DismissDialogParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.command.dismissDialog/parameters/dialogId。
    #[serde(rename = "dialogId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub dialog_id: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：FedCm.command.dismissDialog/parameters/triggerCooldown。
    #[serde(rename = "triggerCooldown")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_cooldown: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for DismissDialogParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DismissDialogParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.command.dismissDialog/parameters/dialogId。
                    #[serde(rename = "dialogId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub dialog_id: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：FedCm.command.dismissDialog/parameters/triggerCooldown。
                    #[serde(rename = "triggerCooldown")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub trigger_cooldown: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DismissDialogParams {
                    dialog_id: fields.dialog_id,
                    trigger_cooldown: fields.trigger_cooldown,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：FedCm.command.dismissDialog。
#[derive(Debug, Clone, Copy)]
pub struct DismissDialog;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：FedCm.command.dismissDialog/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DismissDialogResponse {}
impl<'de> serde::Deserialize<'de> for DismissDialogResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DismissDialogResponse;
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
                Ok(DismissDialogResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for DismissDialog {
    type Params = DismissDialogParams;
    type Response = DismissDialogResponse;
    const METHOD: &'static str = "FedCm.dismissDialog";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：FedCm.command.enable/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct EnableParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：FedCm.command.enable/parameters/disableRejectionDelay。
    ///
    ///协议原文：
    ///
    ///<pre>Allows callers to disable the promise rejection delay that would
    ///normally happen, if this is unimportant to what's being tested.
    ///(step 4 of https://fedidcg.github.io/FedCM/#browser-api-rp-sign-in)</pre>
    ///
    ///
    #[serde(rename = "disableRejectionDelay")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_rejection_delay: Option<bool>,
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
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：FedCm.command.enable/parameters/disableRejectionDelay。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Allows callers to disable the promise rejection delay that would
                    ///normally happen, if this is unimportant to what's being tested.
                    ///(step 4 of https://fedidcg.github.io/FedCM/#browser-api-rp-sign-in)</pre>
                    ///
                    ///
                    #[serde(rename = "disableRejectionDelay")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub disable_rejection_delay: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EnableParams {
                    disable_rejection_delay: fields.disable_rejection_delay,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：FedCm.command.enable。
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：FedCm.command.enable/returns。
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
    const METHOD: &'static str = "FedCm.enable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：FedCm.command.openUrl/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct OpenUrlParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.command.openUrl/parameters/dialogId。
    #[serde(rename = "dialogId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub dialog_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.command.openUrl/parameters/accountIndex。
    #[serde(rename = "accountIndex")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub account_index: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.command.openUrl/parameters/accountUrlType。
    #[serde(rename = "accountUrlType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub account_url_type: crate::fed_cm::types::AccountUrlType,
}
impl<'de> serde::Deserialize<'de> for OpenUrlParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = OpenUrlParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.command.openUrl/parameters/dialogId。
                    #[serde(rename = "dialogId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub dialog_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.command.openUrl/parameters/accountIndex。
                    #[serde(rename = "accountIndex")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub account_index: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.command.openUrl/parameters/accountUrlType。
                    #[serde(rename = "accountUrlType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub account_url_type: crate::fed_cm::types::AccountUrlType,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(OpenUrlParams {
                    dialog_id: fields.dialog_id,
                    account_index: fields.account_index,
                    account_url_type: fields.account_url_type,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：FedCm.command.openUrl。
#[derive(Debug, Clone, Copy)]
pub struct OpenUrl;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：FedCm.command.openUrl/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct OpenUrlResponse {}
impl<'de> serde::Deserialize<'de> for OpenUrlResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = OpenUrlResponse;
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
                Ok(OpenUrlResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for OpenUrl {
    type Params = OpenUrlParams;
    type Response = OpenUrlResponse;
    const METHOD: &'static str = "FedCm.openUrl";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：FedCm.command.resetCooldown/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ResetCooldownParams {}
impl<'de> serde::Deserialize<'de> for ResetCooldownParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResetCooldownParams;
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
                Ok(ResetCooldownParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：FedCm.command.resetCooldown。
///
///协议原文：
///
///<pre>Resets the cooldown time, if any, to allow the next FedCM call to show
///a dialog even if one was recently dismissed by the user.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ResetCooldown;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：FedCm.command.resetCooldown/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ResetCooldownResponse {}
impl<'de> serde::Deserialize<'de> for ResetCooldownResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResetCooldownResponse;
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
                Ok(ResetCooldownResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ResetCooldown {
    type Params = ResetCooldownParams;
    type Response = ResetCooldownResponse;
    const METHOD: &'static str = "FedCm.resetCooldown";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：FedCm.command.selectAccount/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SelectAccountParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.command.selectAccount/parameters/dialogId。
    #[serde(rename = "dialogId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub dialog_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.command.selectAccount/parameters/accountIndex。
    #[serde(rename = "accountIndex")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub account_index: i64,
}
impl<'de> serde::Deserialize<'de> for SelectAccountParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SelectAccountParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.command.selectAccount/parameters/dialogId。
                    #[serde(rename = "dialogId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub dialog_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.command.selectAccount/parameters/accountIndex。
                    #[serde(rename = "accountIndex")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub account_index: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SelectAccountParams {
                    dialog_id: fields.dialog_id,
                    account_index: fields.account_index,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：FedCm.command.selectAccount。
#[derive(Debug, Clone, Copy)]
pub struct SelectAccount;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：FedCm.command.selectAccount/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SelectAccountResponse {}
impl<'de> serde::Deserialize<'de> for SelectAccountResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SelectAccountResponse;
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
                Ok(SelectAccountResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SelectAccount {
    type Params = SelectAccountParams;
    type Response = SelectAccountResponse;
    const METHOD: &'static str = "FedCm.selectAccount";
}
