///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：FedCm.event.dialogClosed/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DialogClosedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.event.dialogClosed/parameters/dialogId。
    #[serde(rename = "dialogId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub dialog_id: String,
}
impl<'de> serde::Deserialize<'de> for DialogClosedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DialogClosedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.event.dialogClosed/parameters/dialogId。
                    #[serde(rename = "dialogId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub dialog_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DialogClosedParams {
                    dialog_id: fields.dialog_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：FedCm.event.dialogClosed。
///
///协议原文：
///
///<pre>Triggered when a dialog is closed, either by user action, JS abort,
///or a command below.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct DialogClosed;
impl ydp_core::Event for DialogClosed {
    type Params = DialogClosedParams;
    const METHOD: &'static str = "FedCm.dialogClosed";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：FedCm.event.dialogShown/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DialogShownParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.event.dialogShown/parameters/dialogId。
    #[serde(rename = "dialogId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub dialog_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.event.dialogShown/parameters/dialogType。
    #[serde(rename = "dialogType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub dialog_type: crate::fed_cm::types::DialogType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.event.dialogShown/parameters/accounts。
    #[serde(rename = "accounts")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub accounts: Vec<crate::fed_cm::types::Account>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.event.dialogShown/parameters/title。
    ///
    ///协议原文：
    ///
    ///<pre>These exist primarily so that the caller can verify the
    ///RP context was used appropriately.</pre>
    ///
    ///
    #[serde(rename = "title")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub title: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：FedCm.event.dialogShown/parameters/subtitle。
    #[serde(rename = "subtitle")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub subtitle: Option<String>,
}
impl<'de> serde::Deserialize<'de> for DialogShownParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DialogShownParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.event.dialogShown/parameters/dialogId。
                    #[serde(rename = "dialogId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub dialog_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.event.dialogShown/parameters/dialogType。
                    #[serde(rename = "dialogType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub dialog_type: crate::fed_cm::types::DialogType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.event.dialogShown/parameters/accounts。
                    #[serde(rename = "accounts")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub accounts: Vec<crate::fed_cm::types::Account>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FedCm.event.dialogShown/parameters/title。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>These exist primarily so that the caller can verify the
                    ///RP context was used appropriately.</pre>
                    ///
                    ///
                    #[serde(rename = "title")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub title: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：FedCm.event.dialogShown/parameters/subtitle。
                    #[serde(rename = "subtitle")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub subtitle: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DialogShownParams {
                    dialog_id: fields.dialog_id,
                    dialog_type: fields.dialog_type,
                    accounts: fields.accounts,
                    title: fields.title,
                    subtitle: fields.subtitle,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：FedCm.event.dialogShown。
#[derive(Debug, Clone, Copy)]
pub struct DialogShown;
impl ydp_core::Event for DialogShown {
    type Params = DialogShownParams;
    const METHOD: &'static str = "FedCm.dialogShown";
}
