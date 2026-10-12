///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Autofill.type.Address。
#[derive(Debug, Clone, serde::Serialize)]
pub struct Address {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.Address/properties/fields。
    ///
    ///协议原文：
    ///
    ///<pre>fields and values defining an address.</pre>
    ///
    ///
    #[serde(rename = "fields")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub fields: Vec<crate::autofill::types::AddressField>,
}
impl<'de> serde::Deserialize<'de> for Address {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Address;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.Address/properties/fields。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>fields and values defining an address.</pre>
                    ///
                    ///
                    #[serde(rename = "fields")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub fields: Vec<crate::autofill::types::AddressField>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Address {
                    fields: fields.fields,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Autofill.type.AddressField。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddressField {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.AddressField/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>address field name, for example GIVEN_NAME.
    ///The full list of supported field names:
    ///https://source.chromium.org/chromium/chromium/src/+/main:components/autofill/core/browser/field_types.cc;l=38</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.AddressField/properties/value。
    ///
    ///协议原文：
    ///
    ///<pre>address field value, for example Jon Doe.</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: String,
}
impl<'de> serde::Deserialize<'de> for AddressField {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddressField;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.AddressField/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>address field name, for example GIVEN_NAME.
                    ///The full list of supported field names:
                    ///https://source.chromium.org/chromium/chromium/src/+/main:components/autofill/core/browser/field_types.cc;l=38</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.AddressField/properties/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>address field value, for example Jon Doe.</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AddressField {
                    name: fields.name,
                    value: fields.value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Autofill.type.AddressFields。
///
///协议原文：
///
///<pre>A list of address fields.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddressFields {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.AddressFields/properties/fields。
    #[serde(rename = "fields")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub fields: Vec<crate::autofill::types::AddressField>,
}
impl<'de> serde::Deserialize<'de> for AddressFields {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddressFields;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.AddressFields/properties/fields。
                    #[serde(rename = "fields")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub fields: Vec<crate::autofill::types::AddressField>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AddressFields {
                    fields: fields.fields,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Autofill.type.AddressUI。
///
///协议原文：
///
///<pre>Defines how an address can be displayed like in chrome://settings/addresses.
///Address UI is a two dimensional array, each inner array is an "address information line", and when rendered in a UI surface should be displayed as such.
///The following address UI for instance:
///[[{name: "GIVE_NAME", value: "Jon"}, {name: "FAMILY_NAME", value: "Doe"}], [{name: "CITY", value: "Munich"}, {name: "ZIP", value: "81456"}]]
///should allow the receiver to render:
///Jon Doe
///Munich 81456</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddressUi {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.AddressUI/properties/addressFields。
    ///
    ///协议原文：
    ///
    ///<pre>A two dimension array containing the representation of values from an address profile.</pre>
    ///
    ///
    #[serde(rename = "addressFields")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub address_fields: Vec<crate::autofill::types::AddressFields>,
}
impl<'de> serde::Deserialize<'de> for AddressUi {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddressUi;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.AddressUI/properties/addressFields。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A two dimension array containing the representation of values from an address profile.</pre>
                    ///
                    ///
                    #[serde(rename = "addressFields")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub address_fields: Vec<crate::autofill::types::AddressFields>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AddressUi {
                    address_fields: fields.address_fields,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Autofill.type.CreditCard。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CreditCard {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.CreditCard/properties/number。
    ///
    ///协议原文：
    ///
    ///<pre>16-digit credit card number.</pre>
    ///
    ///
    #[serde(rename = "number")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub number: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.CreditCard/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Name of the credit card owner.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.CreditCard/properties/expiryMonth。
    ///
    ///协议原文：
    ///
    ///<pre>2-digit expiry month.</pre>
    ///
    ///
    #[serde(rename = "expiryMonth")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub expiry_month: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.CreditCard/properties/expiryYear。
    ///
    ///协议原文：
    ///
    ///<pre>4-digit expiry year.</pre>
    ///
    ///
    #[serde(rename = "expiryYear")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub expiry_year: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.CreditCard/properties/cvc。
    ///
    ///协议原文：
    ///
    ///<pre>3-digit card verification code.</pre>
    ///
    ///
    #[serde(rename = "cvc")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cvc: String,
}
impl<'de> serde::Deserialize<'de> for CreditCard {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CreditCard;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.CreditCard/properties/number。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>16-digit credit card number.</pre>
                    ///
                    ///
                    #[serde(rename = "number")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub number: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.CreditCard/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Name of the credit card owner.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.CreditCard/properties/expiryMonth。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>2-digit expiry month.</pre>
                    ///
                    ///
                    #[serde(rename = "expiryMonth")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub expiry_month: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.CreditCard/properties/expiryYear。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>4-digit expiry year.</pre>
                    ///
                    ///
                    #[serde(rename = "expiryYear")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub expiry_year: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.CreditCard/properties/cvc。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>3-digit card verification code.</pre>
                    ///
                    ///
                    #[serde(rename = "cvc")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cvc: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CreditCard {
                    number: fields.number,
                    name: fields.name,
                    expiry_month: fields.expiry_month,
                    expiry_year: fields.expiry_year,
                    cvc: fields.cvc,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Autofill.type.FilledField。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FilledField {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.FilledField/properties/htmlType。
    ///
    ///协议原文：
    ///
    ///<pre>The type of the field, e.g text, password etc.</pre>
    ///
    ///
    #[serde(rename = "htmlType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub html_type: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.FilledField/properties/id。
    ///
    ///协议原文：
    ///
    ///<pre>the html id</pre>
    ///
    ///
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.FilledField/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>the html name</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.FilledField/properties/value。
    ///
    ///协议原文：
    ///
    ///<pre>the field value</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.FilledField/properties/autofillType。
    ///
    ///协议原文：
    ///
    ///<pre>The actual field type, e.g FAMILY_NAME</pre>
    ///
    ///
    #[serde(rename = "autofillType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub autofill_type: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.FilledField/properties/fillingStrategy。
    ///
    ///协议原文：
    ///
    ///<pre>The filling strategy</pre>
    ///
    ///
    #[serde(rename = "fillingStrategy")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub filling_strategy: crate::autofill::types::FillingStrategy,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.FilledField/properties/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>The frame the field belongs to</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.FilledField/properties/fieldId。
    ///
    ///协议原文：
    ///
    ///<pre>The form field's DOM node</pre>
    ///
    ///
    #[serde(rename = "fieldId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub field_id: crate::dom::types::BackendNodeId,
}
impl<'de> serde::Deserialize<'de> for FilledField {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FilledField;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.FilledField/properties/htmlType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The type of the field, e.g text, password etc.</pre>
                    ///
                    ///
                    #[serde(rename = "htmlType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub html_type: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.FilledField/properties/id。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>the html id</pre>
                    ///
                    ///
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.FilledField/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>the html name</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.FilledField/properties/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>the field value</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.FilledField/properties/autofillType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The actual field type, e.g FAMILY_NAME</pre>
                    ///
                    ///
                    #[serde(rename = "autofillType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub autofill_type: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.FilledField/properties/fillingStrategy。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The filling strategy</pre>
                    ///
                    ///
                    #[serde(rename = "fillingStrategy")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub filling_strategy: crate::autofill::types::FillingStrategy,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.FilledField/properties/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The frame the field belongs to</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Autofill.type.FilledField/properties/fieldId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The form field's DOM node</pre>
                    ///
                    ///
                    #[serde(rename = "fieldId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub field_id: crate::dom::types::BackendNodeId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FilledField {
                    html_type: fields.html_type,
                    id: fields.id,
                    name: fields.name,
                    value: fields.value,
                    autofill_type: fields.autofill_type,
                    filling_strategy: fields.filling_strategy,
                    frame_id: fields.frame_id,
                    field_id: fields.field_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Autofill.type.FillingStrategy。
///
///协议原文：
///
///<pre>Specified whether a filled field was done so by using the html autocomplete attribute or autofill heuristics.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FillingStrategy {
    ///协议值 "autocompleteAttribute"。
    AutocompleteAttribute,
    ///协议值 "autofillInferred"。
    AutofillInferred,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl FillingStrategy {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::autofill::types::FillingStrategy::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::AutocompleteAttribute => "autocompleteAttribute",
            Self::AutofillInferred => "autofillInferred",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for FillingStrategy {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for FillingStrategy {
    fn from(value: String) -> Self {
        match value.as_str() {
            "autocompleteAttribute" => FillingStrategy::AutocompleteAttribute,
            "autofillInferred" => FillingStrategy::AutofillInferred,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for FillingStrategy {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FillingStrategy {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FillingStrategy;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "autocompleteAttribute" => FillingStrategy::AutocompleteAttribute,
                    "autofillInferred" => FillingStrategy::AutofillInferred,
                    _ => FillingStrategy::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(FillingStrategy::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
