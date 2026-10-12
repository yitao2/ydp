///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：FileSystem.type.BucketFileSystemLocator。
#[derive(Debug, Clone, serde::Serialize)]
pub struct BucketFileSystemLocator {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FileSystem.type.BucketFileSystemLocator/properties/storageKey。
    ///
    ///协议原文：
    ///
    ///<pre>Storage key</pre>
    ///
    ///
    #[serde(rename = "storageKey")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub storage_key: crate::storage::types::SerializedStorageKey,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：FileSystem.type.BucketFileSystemLocator/properties/bucketName。
    ///
    ///协议原文：
    ///
    ///<pre>Bucket name. Not passing a `bucketName` will retrieve the default Bucket. (https://developer.mozilla.org/en-US/docs/Web/API/Storage_API#storage_buckets)</pre>
    ///
    ///
    #[serde(rename = "bucketName")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub bucket_name: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FileSystem.type.BucketFileSystemLocator/properties/pathComponents。
    ///
    ///协议原文：
    ///
    ///<pre>Path to the directory using each path component as an array item.</pre>
    ///
    ///
    #[serde(rename = "pathComponents")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub path_components: Vec<String>,
}
impl<'de> serde::Deserialize<'de> for BucketFileSystemLocator {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BucketFileSystemLocator;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FileSystem.type.BucketFileSystemLocator/properties/storageKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Storage key</pre>
                    ///
                    ///
                    #[serde(rename = "storageKey")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub storage_key: crate::storage::types::SerializedStorageKey,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：FileSystem.type.BucketFileSystemLocator/properties/bucketName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Bucket name. Not passing a `bucketName` will retrieve the default Bucket. (https://developer.mozilla.org/en-US/docs/Web/API/Storage_API#storage_buckets)</pre>
                    ///
                    ///
                    #[serde(rename = "bucketName")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub bucket_name: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FileSystem.type.BucketFileSystemLocator/properties/pathComponents。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Path to the directory using each path component as an array item.</pre>
                    ///
                    ///
                    #[serde(rename = "pathComponents")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub path_components: Vec<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(BucketFileSystemLocator {
                    storage_key: fields.storage_key,
                    bucket_name: fields.bucket_name,
                    path_components: fields.path_components,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：FileSystem.type.Directory。
#[derive(Debug, Clone, serde::Serialize)]
pub struct Directory {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FileSystem.type.Directory/properties/name。
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FileSystem.type.Directory/properties/nestedDirectories。
    #[serde(rename = "nestedDirectories")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub nested_directories: Vec<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FileSystem.type.Directory/properties/nestedFiles。
    ///
    ///协议原文：
    ///
    ///<pre>Files that are directly nested under this directory.</pre>
    ///
    ///
    #[serde(rename = "nestedFiles")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub nested_files: Vec<crate::file_system::types::File>,
}
impl<'de> serde::Deserialize<'de> for Directory {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Directory;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FileSystem.type.Directory/properties/name。
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FileSystem.type.Directory/properties/nestedDirectories。
                    #[serde(rename = "nestedDirectories")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub nested_directories: Vec<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FileSystem.type.Directory/properties/nestedFiles。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Files that are directly nested under this directory.</pre>
                    ///
                    ///
                    #[serde(rename = "nestedFiles")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub nested_files: Vec<crate::file_system::types::File>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Directory {
                    name: fields.name,
                    nested_directories: fields.nested_directories,
                    nested_files: fields.nested_files,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：FileSystem.type.File。
#[derive(Debug, Clone, serde::Serialize)]
pub struct File {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FileSystem.type.File/properties/name。
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FileSystem.type.File/properties/lastModified。
    ///
    ///协议原文：
    ///
    ///<pre>Timestamp</pre>
    ///
    ///
    #[serde(rename = "lastModified")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub last_modified: crate::network::types::TimeSinceEpoch,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FileSystem.type.File/properties/size。
    ///
    ///协议原文：
    ///
    ///<pre>Size in bytes</pre>
    ///
    ///
    #[serde(rename = "size")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub size: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FileSystem.type.File/properties/type。
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: String,
}
impl<'de> serde::Deserialize<'de> for File {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = File;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FileSystem.type.File/properties/name。
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FileSystem.type.File/properties/lastModified。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Timestamp</pre>
                    ///
                    ///
                    #[serde(rename = "lastModified")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub last_modified: crate::network::types::TimeSinceEpoch,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FileSystem.type.File/properties/size。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Size in bytes</pre>
                    ///
                    ///
                    #[serde(rename = "size")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub size: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FileSystem.type.File/properties/type。
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(File {
                    name: fields.name,
                    last_modified: fields.last_modified,
                    size: fields.size,
                    r#type: fields.r#type,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
