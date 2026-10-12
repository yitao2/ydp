///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IO.command.close/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CloseParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IO.command.close/parameters/handle。
    ///
    ///协议原文：
    ///
    ///<pre>Handle of the stream to close.</pre>
    ///
    ///
    #[serde(rename = "handle")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub handle: crate::io::types::StreamHandle,
}
impl<'de> serde::Deserialize<'de> for CloseParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CloseParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IO.command.close/parameters/handle。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Handle of the stream to close.</pre>
                    ///
                    ///
                    #[serde(rename = "handle")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub handle: crate::io::types::StreamHandle,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CloseParams {
                    handle: fields.handle,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：IO.command.close。
///
///协议原文：
///
///<pre>Close the stream, discard any temporary backing storage.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Close;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IO.command.close/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CloseResponse {}
impl<'de> serde::Deserialize<'de> for CloseResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CloseResponse;
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
                Ok(CloseResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Close {
    type Params = CloseParams;
    type Response = CloseResponse;
    const METHOD: &'static str = "IO.close";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IO.command.read/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReadParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IO.command.read/parameters/handle。
    ///
    ///协议原文：
    ///
    ///<pre>Handle of the stream to read.</pre>
    ///
    ///
    #[serde(rename = "handle")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub handle: crate::io::types::StreamHandle,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IO.command.read/parameters/offset。
    ///
    ///协议原文：
    ///
    ///<pre>Seek to the specified offset before reading (if not specified, proceed with offset
    ///following the last read). Some types of streams may only support sequential reads.</pre>
    ///
    ///
    #[serde(rename = "offset")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub offset: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IO.command.read/parameters/size。
    ///
    ///协议原文：
    ///
    ///<pre>Maximum number of bytes to read (left upon the agent discretion if not specified).</pre>
    ///
    ///
    #[serde(rename = "size")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub size: Option<i64>,
}
impl<'de> serde::Deserialize<'de> for ReadParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReadParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IO.command.read/parameters/handle。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Handle of the stream to read.</pre>
                    ///
                    ///
                    #[serde(rename = "handle")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub handle: crate::io::types::StreamHandle,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IO.command.read/parameters/offset。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Seek to the specified offset before reading (if not specified, proceed with offset
                    ///following the last read). Some types of streams may only support sequential reads.</pre>
                    ///
                    ///
                    #[serde(rename = "offset")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub offset: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IO.command.read/parameters/size。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Maximum number of bytes to read (left upon the agent discretion if not specified).</pre>
                    ///
                    ///
                    #[serde(rename = "size")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub size: Option<i64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReadParams {
                    handle: fields.handle,
                    offset: fields.offset,
                    size: fields.size,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：IO.command.read。
///
///协议原文：
///
///<pre>Read a chunk of the stream</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Read;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IO.command.read/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReadResponse {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IO.command.read/returns/base64Encoded。
    ///
    ///协议原文：
    ///
    ///<pre>Set if the data is base64-encoded</pre>
    ///
    ///
    #[serde(rename = "base64Encoded")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub base64_encoded: Option<bool>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IO.command.read/returns/data。
    ///
    ///协议原文：
    ///
    ///<pre>Data that were read.</pre>
    ///
    ///
    #[serde(rename = "data")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub data: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IO.command.read/returns/eof。
    ///
    ///协议原文：
    ///
    ///<pre>Set if the end-of-file condition occurred while reading.</pre>
    ///
    ///
    #[serde(rename = "eof")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub eof: bool,
}
impl<'de> serde::Deserialize<'de> for ReadResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReadResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：IO.command.read/returns/base64Encoded。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Set if the data is base64-encoded</pre>
                    ///
                    ///
                    #[serde(rename = "base64Encoded")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub base64_encoded: Option<bool>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IO.command.read/returns/data。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Data that were read.</pre>
                    ///
                    ///
                    #[serde(rename = "data")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub data: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IO.command.read/returns/eof。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Set if the end-of-file condition occurred while reading.</pre>
                    ///
                    ///
                    #[serde(rename = "eof")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub eof: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReadResponse {
                    base64_encoded: fields.base64_encoded,
                    data: fields.data,
                    eof: fields.eof,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for Read {
    type Params = ReadParams;
    type Response = ReadResponse;
    const METHOD: &'static str = "IO.read";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IO.command.resolveBlob/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ResolveBlobParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IO.command.resolveBlob/parameters/objectId。
    ///
    ///协议原文：
    ///
    ///<pre>Object id of a Blob object wrapper.</pre>
    ///
    ///
    #[serde(rename = "objectId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub object_id: crate::runtime::types::RemoteObjectId,
}
impl<'de> serde::Deserialize<'de> for ResolveBlobParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResolveBlobParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IO.command.resolveBlob/parameters/objectId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Object id of a Blob object wrapper.</pre>
                    ///
                    ///
                    #[serde(rename = "objectId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub object_id: crate::runtime::types::RemoteObjectId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ResolveBlobParams {
                    object_id: fields.object_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：IO.command.resolveBlob。
///
///协议原文：
///
///<pre>Return UUID of Blob object specified by a remote object id.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ResolveBlob;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：IO.command.resolveBlob/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ResolveBlobResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：IO.command.resolveBlob/returns/uuid。
    ///
    ///协议原文：
    ///
    ///<pre>UUID of the specified Blob.</pre>
    ///
    ///
    #[serde(rename = "uuid")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub uuid: String,
}
impl<'de> serde::Deserialize<'de> for ResolveBlobResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResolveBlobResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：IO.command.resolveBlob/returns/uuid。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>UUID of the specified Blob.</pre>
                    ///
                    ///
                    #[serde(rename = "uuid")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub uuid: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ResolveBlobResponse { uuid: fields.uuid })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ResolveBlob {
    type Params = ResolveBlobParams;
    type Response = ResolveBlobResponse;
    const METHOD: &'static str = "IO.resolveBlob";
}
