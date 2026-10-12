///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Autofill.event.addressFormFilled/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddressFormFilledParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.event.addressFormFilled/parameters/filledFields。
    ///
    ///协议原文：
    ///
    ///<pre>Information about the fields that were filled</pre>
    ///
    ///
    #[serde(rename = "filledFields")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub filled_fields: Vec<crate::autofill::types::FilledField>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.event.addressFormFilled/parameters/addressUi。
    ///
    ///协议原文：
    ///
    ///<pre>An UI representation of the address used to fill the form.
    ///Consists of a 2D array where each child represents an address/profile line.</pre>
    ///
    ///
    #[serde(rename = "addressUi")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub address_ui: crate::autofill::types::AddressUi,
}
impl<'de> serde::Deserialize<'de> for AddressFormFilledParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddressFormFilledParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.event.addressFormFilled/parameters/filledFields。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Information about the fields that were filled</pre>
                    ///
                    ///
                    #[serde(rename = "filledFields")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub filled_fields: Vec<crate::autofill::types::FilledField>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.event.addressFormFilled/parameters/addressUi。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An UI representation of the address used to fill the form.
                    ///Consists of a 2D array where each child represents an address/profile line.</pre>
                    ///
                    ///
                    #[serde(rename = "addressUi")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub address_ui: crate::autofill::types::AddressUi,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AddressFormFilledParams {
                    filled_fields: fields.filled_fields,
                    address_ui: fields.address_ui,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Autofill.event.addressFormFilled。
///
///协议原文：
///
///<pre>Emitted when an address form is filled.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct AddressFormFilled;
impl ydp_core::Event for AddressFormFilled {
    type Params = AddressFormFilledParams;
    const METHOD: &'static str = "Autofill.addressFormFilled";
}
