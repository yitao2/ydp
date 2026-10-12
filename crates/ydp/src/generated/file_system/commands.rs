///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：FileSystem.command.getDirectory/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetDirectoryParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FileSystem.command.getDirectory/parameters/bucketFileSystemLocator。
    #[serde(rename = "bucketFileSystemLocator")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub bucket_file_system_locator: crate::file_system::types::BucketFileSystemLocator,
}
impl<'de> serde::Deserialize<'de> for GetDirectoryParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetDirectoryParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FileSystem.command.getDirectory/parameters/bucketFileSystemLocator。
                    #[serde(rename = "bucketFileSystemLocator")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub bucket_file_system_locator:
                        crate::file_system::types::BucketFileSystemLocator,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetDirectoryParams {
                    bucket_file_system_locator: fields.bucket_file_system_locator,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：FileSystem.command.getDirectory。
#[derive(Debug, Clone, Copy)]
pub struct GetDirectory;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：FileSystem.command.getDirectory/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetDirectoryResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：FileSystem.command.getDirectory/returns/directory。
    ///
    ///协议原文：
    ///
    ///<pre>Returns the directory object at the path.</pre>
    ///
    ///
    #[serde(rename = "directory")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub directory: crate::file_system::types::Directory,
}
impl<'de> serde::Deserialize<'de> for GetDirectoryResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetDirectoryResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：FileSystem.command.getDirectory/returns/directory。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Returns the directory object at the path.</pre>
                    ///
                    ///
                    #[serde(rename = "directory")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub directory: crate::file_system::types::Directory,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetDirectoryResponse {
                    directory: fields.directory,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetDirectory {
    type Params = GetDirectoryParams;
    type Response = GetDirectoryResponse;
    const METHOD: &'static str = "FileSystem.getDirectory";
}
