///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Extensions.command.clearStorageItems/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearStorageItemsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.clearStorageItems/parameters/id。
    ///
    ///协议原文：
    ///
    ///<pre>ID of extension.</pre>
    ///
    ///
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.clearStorageItems/parameters/storageArea。
    ///
    ///协议原文：
    ///
    ///<pre>StorageArea to remove data from.</pre>
    ///
    ///
    #[serde(rename = "storageArea")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_area: crate::extensions::types::StorageArea,
}
impl<'de> serde::Deserialize<'de> for ClearStorageItemsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearStorageItemsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.clearStorageItems/parameters/id。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>ID of extension.</pre>
                    ///
                    ///
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.clearStorageItems/parameters/storageArea。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>StorageArea to remove data from.</pre>
                    ///
                    ///
                    #[serde(rename = "storageArea")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_area: crate::extensions::types::StorageArea,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ClearStorageItemsParams {
                    id: fields.id,
                    storage_area: fields.storage_area,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Extensions.command.clearStorageItems。
///
///协议原文：
///
///<pre>Clears extension storage in the given `storageArea`.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ClearStorageItems;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Extensions.command.clearStorageItems/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearStorageItemsResponse {}
impl<'de> serde::Deserialize<'de> for ClearStorageItemsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearStorageItemsResponse;
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
                Ok(ClearStorageItemsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ClearStorageItems {
    type Params = ClearStorageItemsParams;
    type Response = ClearStorageItemsResponse;
    const METHOD: &'static str = "Extensions.clearStorageItems";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Extensions.command.getExtensions/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetExtensionsParams {}
impl<'de> serde::Deserialize<'de> for GetExtensionsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetExtensionsParams;
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
                Ok(GetExtensionsParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Extensions.command.getExtensions。
///
///协议原文：
///
///<pre>Gets a list of all unpacked extensions.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetExtensions;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Extensions.command.getExtensions/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetExtensionsResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.getExtensions/returns/extensions。
    #[serde(rename = "extensions")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub extensions: Vec<crate::extensions::types::ExtensionInfo>,
}
impl<'de> serde::Deserialize<'de> for GetExtensionsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetExtensionsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.getExtensions/returns/extensions。
                    #[serde(rename = "extensions")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub extensions: Vec<crate::extensions::types::ExtensionInfo>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetExtensionsResponse {
                    extensions: fields.extensions,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetExtensions {
    type Params = GetExtensionsParams;
    type Response = GetExtensionsResponse;
    const METHOD: &'static str = "Extensions.getExtensions";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Extensions.command.getStorageItems/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetStorageItemsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.getStorageItems/parameters/id。
    ///
    ///协议原文：
    ///
    ///<pre>ID of extension.</pre>
    ///
    ///
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.getStorageItems/parameters/storageArea。
    ///
    ///协议原文：
    ///
    ///<pre>StorageArea to retrieve data from.</pre>
    ///
    ///
    #[serde(rename = "storageArea")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_area: crate::extensions::types::StorageArea,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Extensions.command.getStorageItems/parameters/keys。
    ///
    ///协议原文：
    ///
    ///<pre>Keys to retrieve.</pre>
    ///
    ///
    #[serde(rename = "keys")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub keys: Option<Vec<String>>,
}
impl<'de> serde::Deserialize<'de> for GetStorageItemsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetStorageItemsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.getStorageItems/parameters/id。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>ID of extension.</pre>
                    ///
                    ///
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.getStorageItems/parameters/storageArea。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>StorageArea to retrieve data from.</pre>
                    ///
                    ///
                    #[serde(rename = "storageArea")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_area: crate::extensions::types::StorageArea,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Extensions.command.getStorageItems/parameters/keys。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Keys to retrieve.</pre>
                    ///
                    ///
                    #[serde(rename = "keys")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub keys: Option<Vec<String>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetStorageItemsParams {
                    id: fields.id,
                    storage_area: fields.storage_area,
                    keys: fields.keys,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Extensions.command.getStorageItems。
///
///协议原文：
///
///<pre>Gets data from extension storage in the given `storageArea`. If `keys` is
///specified, these are used to filter the result.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetStorageItems;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Extensions.command.getStorageItems/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetStorageItemsResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.getStorageItems/returns/data。
    #[serde(rename = "data")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub data: std::collections::BTreeMap<String, serde_json::Value>,
}
impl<'de> serde::Deserialize<'de> for GetStorageItemsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetStorageItemsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.getStorageItems/returns/data。
                    #[serde(rename = "data")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub data: std::collections::BTreeMap<String, serde_json::Value>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetStorageItemsResponse { data: fields.data })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetStorageItems {
    type Params = GetStorageItemsParams;
    type Response = GetStorageItemsResponse;
    const METHOD: &'static str = "Extensions.getStorageItems";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Extensions.command.loadUnpacked/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct LoadUnpackedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.loadUnpacked/parameters/path。
    ///
    ///协议原文：
    ///
    ///<pre>Absolute file path.</pre>
    ///
    ///
    #[serde(rename = "path")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub path: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Extensions.command.loadUnpacked/parameters/enableInIncognito。
    ///
    ///协议原文：
    ///
    ///<pre>Enable the extension in incognito</pre>
    ///
    ///
    #[serde(rename = "enableInIncognito")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_in_incognito: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for LoadUnpackedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LoadUnpackedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.loadUnpacked/parameters/path。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Absolute file path.</pre>
                    ///
                    ///
                    #[serde(rename = "path")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub path: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Extensions.command.loadUnpacked/parameters/enableInIncognito。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Enable the extension in incognito</pre>
                    ///
                    ///
                    #[serde(rename = "enableInIncognito")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub enable_in_incognito: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LoadUnpackedParams {
                    path: fields.path,
                    enable_in_incognito: fields.enable_in_incognito,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Extensions.command.loadUnpacked。
///
///协议原文：
///
///<pre>Installs an unpacked extension from the filesystem similar to
///--load-extension CLI flags. Returns extension ID once the extension
///has been installed.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct LoadUnpacked;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Extensions.command.loadUnpacked/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct LoadUnpackedResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.loadUnpacked/returns/id。
    ///
    ///协议原文：
    ///
    ///<pre>Extension id.</pre>
    ///
    ///
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: String,
}
impl<'de> serde::Deserialize<'de> for LoadUnpackedResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LoadUnpackedResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.loadUnpacked/returns/id。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Extension id.</pre>
                    ///
                    ///
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LoadUnpackedResponse { id: fields.id })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for LoadUnpacked {
    type Params = LoadUnpackedParams;
    type Response = LoadUnpackedResponse;
    const METHOD: &'static str = "Extensions.loadUnpacked";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Extensions.command.removeStorageItems/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveStorageItemsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.removeStorageItems/parameters/id。
    ///
    ///协议原文：
    ///
    ///<pre>ID of extension.</pre>
    ///
    ///
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.removeStorageItems/parameters/storageArea。
    ///
    ///协议原文：
    ///
    ///<pre>StorageArea to remove data from.</pre>
    ///
    ///
    #[serde(rename = "storageArea")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_area: crate::extensions::types::StorageArea,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.removeStorageItems/parameters/keys。
    ///
    ///协议原文：
    ///
    ///<pre>Keys to remove.</pre>
    ///
    ///
    #[serde(rename = "keys")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub keys: Vec<String>,
}
impl<'de> serde::Deserialize<'de> for RemoveStorageItemsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveStorageItemsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.removeStorageItems/parameters/id。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>ID of extension.</pre>
                    ///
                    ///
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.removeStorageItems/parameters/storageArea。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>StorageArea to remove data from.</pre>
                    ///
                    ///
                    #[serde(rename = "storageArea")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_area: crate::extensions::types::StorageArea,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.removeStorageItems/parameters/keys。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Keys to remove.</pre>
                    ///
                    ///
                    #[serde(rename = "keys")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub keys: Vec<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RemoveStorageItemsParams {
                    id: fields.id,
                    storage_area: fields.storage_area,
                    keys: fields.keys,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Extensions.command.removeStorageItems。
///
///协议原文：
///
///<pre>Removes `keys` from extension storage in the given `storageArea`.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct RemoveStorageItems;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Extensions.command.removeStorageItems/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveStorageItemsResponse {}
impl<'de> serde::Deserialize<'de> for RemoveStorageItemsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveStorageItemsResponse;
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
                Ok(RemoveStorageItemsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RemoveStorageItems {
    type Params = RemoveStorageItemsParams;
    type Response = RemoveStorageItemsResponse;
    const METHOD: &'static str = "Extensions.removeStorageItems";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Extensions.command.setStorageItems/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetStorageItemsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.setStorageItems/parameters/id。
    ///
    ///协议原文：
    ///
    ///<pre>ID of extension.</pre>
    ///
    ///
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.setStorageItems/parameters/storageArea。
    ///
    ///协议原文：
    ///
    ///<pre>StorageArea to set data in.</pre>
    ///
    ///
    #[serde(rename = "storageArea")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_area: crate::extensions::types::StorageArea,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.setStorageItems/parameters/values。
    ///
    ///协议原文：
    ///
    ///<pre>Values to set.</pre>
    ///
    ///
    #[serde(rename = "values")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub values: std::collections::BTreeMap<String, serde_json::Value>,
}
impl<'de> serde::Deserialize<'de> for SetStorageItemsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetStorageItemsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.setStorageItems/parameters/id。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>ID of extension.</pre>
                    ///
                    ///
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.setStorageItems/parameters/storageArea。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>StorageArea to set data in.</pre>
                    ///
                    ///
                    #[serde(rename = "storageArea")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_area: crate::extensions::types::StorageArea,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.setStorageItems/parameters/values。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Values to set.</pre>
                    ///
                    ///
                    #[serde(rename = "values")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub values: std::collections::BTreeMap<String, serde_json::Value>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetStorageItemsParams {
                    id: fields.id,
                    storage_area: fields.storage_area,
                    values: fields.values,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Extensions.command.setStorageItems。
///
///协议原文：
///
///<pre>Sets `values` in extension storage in the given `storageArea`. The provided `values`
///will be merged with existing values in the storage area.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetStorageItems;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Extensions.command.setStorageItems/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetStorageItemsResponse {}
impl<'de> serde::Deserialize<'de> for SetStorageItemsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetStorageItemsResponse;
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
                Ok(SetStorageItemsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetStorageItems {
    type Params = SetStorageItemsParams;
    type Response = SetStorageItemsResponse;
    const METHOD: &'static str = "Extensions.setStorageItems";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Extensions.command.triggerAction/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TriggerActionParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.triggerAction/parameters/id。
    ///
    ///协议原文：
    ///
    ///<pre>Extension id.</pre>
    ///
    ///
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.triggerAction/parameters/targetId。
    ///
    ///协议原文：
    ///
    ///<pre>A tab target ID to trigger the default extension action on.</pre>
    ///
    ///
    #[serde(rename = "targetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub target_id: String,
}
impl<'de> serde::Deserialize<'de> for TriggerActionParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TriggerActionParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.triggerAction/parameters/id。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Extension id.</pre>
                    ///
                    ///
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.triggerAction/parameters/targetId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A tab target ID to trigger the default extension action on.</pre>
                    ///
                    ///
                    #[serde(rename = "targetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub target_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TriggerActionParams {
                    id: fields.id,
                    target_id: fields.target_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Extensions.command.triggerAction。
///
///协议原文：
///
///<pre>Runs an extension default action.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct TriggerAction;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Extensions.command.triggerAction/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TriggerActionResponse {}
impl<'de> serde::Deserialize<'de> for TriggerActionResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TriggerActionResponse;
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
                Ok(TriggerActionResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for TriggerAction {
    type Params = TriggerActionParams;
    type Response = TriggerActionResponse;
    const METHOD: &'static str = "Extensions.triggerAction";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Extensions.command.uninstall/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct UninstallParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.uninstall/parameters/id。
    ///
    ///协议原文：
    ///
    ///<pre>Extension id.</pre>
    ///
    ///
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: String,
}
impl<'de> serde::Deserialize<'de> for UninstallParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UninstallParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Extensions.command.uninstall/parameters/id。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Extension id.</pre>
                    ///
                    ///
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(UninstallParams { id: fields.id })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Extensions.command.uninstall。
///
///协议原文：
///
///<pre>Uninstalls an unpacked extension (others not supported) from the profile.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Uninstall;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Extensions.command.uninstall/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct UninstallResponse {}
impl<'de> serde::Deserialize<'de> for UninstallResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UninstallResponse;
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
                Ok(UninstallResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Uninstall {
    type Params = UninstallParams;
    type Response = UninstallResponse;
    const METHOD: &'static str = "Extensions.uninstall";
}
