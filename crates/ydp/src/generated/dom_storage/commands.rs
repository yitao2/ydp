///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMStorage.command.clear/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.command.clear/parameters/storageId。
    #[serde(rename = "storageId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_id: crate::dom_storage::types::StorageId,
}
impl<'de> serde::Deserialize<'de> for ClearParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.command.clear/parameters/storageId。
                    #[serde(rename = "storageId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_id: crate::dom_storage::types::StorageId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ClearParams {
                    storage_id: fields.storage_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOMStorage.command.clear。
#[derive(Debug, Clone, Copy)]
pub struct Clear;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMStorage.command.clear/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearResponse {}
impl<'de> serde::Deserialize<'de> for ClearResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearResponse;
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
                Ok(ClearResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Clear {
    type Params = ClearParams;
    type Response = ClearResponse;
    const METHOD: &'static str = "DOMStorage.clear";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMStorage.command.disable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOMStorage.command.disable。
///
///协议原文：
///
///<pre>Disables storage tracking, prevents storage events from being sent to the client.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMStorage.command.disable/returns。
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
    const METHOD: &'static str = "DOMStorage.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMStorage.command.enable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOMStorage.command.enable。
///
///协议原文：
///
///<pre>Enables storage tracking, storage events will now be delivered to the client.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMStorage.command.enable/returns。
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
    const METHOD: &'static str = "DOMStorage.enable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMStorage.command.getDOMStorageItems/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetDomStorageItemsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.command.getDOMStorageItems/parameters/storageId。
    #[serde(rename = "storageId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_id: crate::dom_storage::types::StorageId,
}
impl<'de> serde::Deserialize<'de> for GetDomStorageItemsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetDomStorageItemsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.command.getDOMStorageItems/parameters/storageId。
                    #[serde(rename = "storageId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_id: crate::dom_storage::types::StorageId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetDomStorageItemsParams {
                    storage_id: fields.storage_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOMStorage.command.getDOMStorageItems。
#[derive(Debug, Clone, Copy)]
pub struct GetDomStorageItems;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMStorage.command.getDOMStorageItems/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetDomStorageItemsResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.command.getDOMStorageItems/returns/entries。
    #[serde(rename = "entries")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub entries: Vec<crate::dom_storage::types::Item>,
}
impl<'de> serde::Deserialize<'de> for GetDomStorageItemsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetDomStorageItemsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.command.getDOMStorageItems/returns/entries。
                    #[serde(rename = "entries")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub entries: Vec<crate::dom_storage::types::Item>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetDomStorageItemsResponse {
                    entries: fields.entries,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetDomStorageItems {
    type Params = GetDomStorageItemsParams;
    type Response = GetDomStorageItemsResponse;
    const METHOD: &'static str = "DOMStorage.getDOMStorageItems";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMStorage.command.removeDOMStorageItem/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveDomStorageItemParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.command.removeDOMStorageItem/parameters/storageId。
    #[serde(rename = "storageId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_id: crate::dom_storage::types::StorageId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.command.removeDOMStorageItem/parameters/key。
    #[serde(rename = "key")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub key: String,
}
impl<'de> serde::Deserialize<'de> for RemoveDomStorageItemParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveDomStorageItemParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.command.removeDOMStorageItem/parameters/storageId。
                    #[serde(rename = "storageId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_id: crate::dom_storage::types::StorageId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.command.removeDOMStorageItem/parameters/key。
                    #[serde(rename = "key")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub key: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RemoveDomStorageItemParams {
                    storage_id: fields.storage_id,
                    key: fields.key,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOMStorage.command.removeDOMStorageItem。
#[derive(Debug, Clone, Copy)]
pub struct RemoveDomStorageItem;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMStorage.command.removeDOMStorageItem/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveDomStorageItemResponse {}
impl<'de> serde::Deserialize<'de> for RemoveDomStorageItemResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveDomStorageItemResponse;
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
                Ok(RemoveDomStorageItemResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RemoveDomStorageItem {
    type Params = RemoveDomStorageItemParams;
    type Response = RemoveDomStorageItemResponse;
    const METHOD: &'static str = "DOMStorage.removeDOMStorageItem";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMStorage.command.setDOMStorageItem/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDomStorageItemParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.command.setDOMStorageItem/parameters/storageId。
    #[serde(rename = "storageId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_id: crate::dom_storage::types::StorageId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.command.setDOMStorageItem/parameters/key。
    #[serde(rename = "key")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub key: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.command.setDOMStorageItem/parameters/value。
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: String,
}
impl<'de> serde::Deserialize<'de> for SetDomStorageItemParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDomStorageItemParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.command.setDOMStorageItem/parameters/storageId。
                    #[serde(rename = "storageId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_id: crate::dom_storage::types::StorageId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.command.setDOMStorageItem/parameters/key。
                    #[serde(rename = "key")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub key: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.command.setDOMStorageItem/parameters/value。
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetDomStorageItemParams {
                    storage_id: fields.storage_id,
                    key: fields.key,
                    value: fields.value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOMStorage.command.setDOMStorageItem。
#[derive(Debug, Clone, Copy)]
pub struct SetDomStorageItem;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMStorage.command.setDOMStorageItem/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDomStorageItemResponse {}
impl<'de> serde::Deserialize<'de> for SetDomStorageItemResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDomStorageItemResponse;
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
                Ok(SetDomStorageItemResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetDomStorageItem {
    type Params = SetDomStorageItemParams;
    type Response = SetDomStorageItemResponse;
    const METHOD: &'static str = "DOMStorage.setDOMStorageItem";
}
