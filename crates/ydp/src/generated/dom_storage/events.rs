///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMStorage.event.domStorageItemAdded/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DomStorageItemAddedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.event.domStorageItemAdded/parameters/storageId。
    #[serde(rename = "storageId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_id: crate::dom_storage::types::StorageId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.event.domStorageItemAdded/parameters/key。
    #[serde(rename = "key")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub key: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.event.domStorageItemAdded/parameters/newValue。
    #[serde(rename = "newValue")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub new_value: String,
}
impl<'de> serde::Deserialize<'de> for DomStorageItemAddedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DomStorageItemAddedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.event.domStorageItemAdded/parameters/storageId。
                    #[serde(rename = "storageId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_id: crate::dom_storage::types::StorageId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.event.domStorageItemAdded/parameters/key。
                    #[serde(rename = "key")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub key: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.event.domStorageItemAdded/parameters/newValue。
                    #[serde(rename = "newValue")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub new_value: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DomStorageItemAddedParams {
                    storage_id: fields.storage_id,
                    key: fields.key,
                    new_value: fields.new_value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOMStorage.event.domStorageItemAdded。
#[derive(Debug, Clone, Copy)]
pub struct DomStorageItemAdded;
impl ydp_core::Event for DomStorageItemAdded {
    type Params = DomStorageItemAddedParams;
    const METHOD: &'static str = "DOMStorage.domStorageItemAdded";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMStorage.event.domStorageItemRemoved/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DomStorageItemRemovedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.event.domStorageItemRemoved/parameters/storageId。
    #[serde(rename = "storageId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_id: crate::dom_storage::types::StorageId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.event.domStorageItemRemoved/parameters/key。
    #[serde(rename = "key")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub key: String,
}
impl<'de> serde::Deserialize<'de> for DomStorageItemRemovedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DomStorageItemRemovedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.event.domStorageItemRemoved/parameters/storageId。
                    #[serde(rename = "storageId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_id: crate::dom_storage::types::StorageId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.event.domStorageItemRemoved/parameters/key。
                    #[serde(rename = "key")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub key: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DomStorageItemRemovedParams {
                    storage_id: fields.storage_id,
                    key: fields.key,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOMStorage.event.domStorageItemRemoved。
#[derive(Debug, Clone, Copy)]
pub struct DomStorageItemRemoved;
impl ydp_core::Event for DomStorageItemRemoved {
    type Params = DomStorageItemRemovedParams;
    const METHOD: &'static str = "DOMStorage.domStorageItemRemoved";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMStorage.event.domStorageItemUpdated/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DomStorageItemUpdatedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.event.domStorageItemUpdated/parameters/storageId。
    #[serde(rename = "storageId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_id: crate::dom_storage::types::StorageId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.event.domStorageItemUpdated/parameters/key。
    #[serde(rename = "key")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub key: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.event.domStorageItemUpdated/parameters/oldValue。
    #[serde(rename = "oldValue")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub old_value: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.event.domStorageItemUpdated/parameters/newValue。
    #[serde(rename = "newValue")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub new_value: String,
}
impl<'de> serde::Deserialize<'de> for DomStorageItemUpdatedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DomStorageItemUpdatedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.event.domStorageItemUpdated/parameters/storageId。
                    #[serde(rename = "storageId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_id: crate::dom_storage::types::StorageId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.event.domStorageItemUpdated/parameters/key。
                    #[serde(rename = "key")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub key: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.event.domStorageItemUpdated/parameters/oldValue。
                    #[serde(rename = "oldValue")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub old_value: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.event.domStorageItemUpdated/parameters/newValue。
                    #[serde(rename = "newValue")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub new_value: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DomStorageItemUpdatedParams {
                    storage_id: fields.storage_id,
                    key: fields.key,
                    old_value: fields.old_value,
                    new_value: fields.new_value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOMStorage.event.domStorageItemUpdated。
#[derive(Debug, Clone, Copy)]
pub struct DomStorageItemUpdated;
impl ydp_core::Event for DomStorageItemUpdated {
    type Params = DomStorageItemUpdatedParams;
    const METHOD: &'static str = "DOMStorage.domStorageItemUpdated";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DOMStorage.event.domStorageItemsCleared/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DomStorageItemsClearedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.event.domStorageItemsCleared/parameters/storageId。
    #[serde(rename = "storageId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_id: crate::dom_storage::types::StorageId,
}
impl<'de> serde::Deserialize<'de> for DomStorageItemsClearedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DomStorageItemsClearedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DOMStorage.event.domStorageItemsCleared/parameters/storageId。
                    #[serde(rename = "storageId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_id: crate::dom_storage::types::StorageId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DomStorageItemsClearedParams {
                    storage_id: fields.storage_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DOMStorage.event.domStorageItemsCleared。
#[derive(Debug, Clone, Copy)]
pub struct DomStorageItemsCleared;
impl ydp_core::Event for DomStorageItemsCleared {
    type Params = DomStorageItemsClearedParams;
    const METHOD: &'static str = "DOMStorage.domStorageItemsCleared";
}
