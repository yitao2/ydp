///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.type.FilterEntry。
///
///协议原文：
///
///<pre>A filter used by target query/discovery/auto-attach operations.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct FilterEntry {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.type.FilterEntry/properties/exclude。
    ///
    ///协议原文：
    ///
    ///<pre>If set, causes exclusion of matching targets from the list.</pre>
    ///
    ///
    #[serde(rename = "exclude")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub exclude: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.type.FilterEntry/properties/type。
    ///
    ///协议原文：
    ///
    ///<pre>If not present, matches any type.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub r#type: Option<String>,
}
impl<'de> serde::Deserialize<'de> for FilterEntry {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = FilterEntry;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.type.FilterEntry/properties/exclude。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If set, causes exclusion of matching targets from the list.</pre>
                    ///
                    ///
                    #[serde(rename = "exclude")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub exclude: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.type.FilterEntry/properties/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If not present, matches any type.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub r#type: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(FilterEntry {
                    exclude: fields.exclude,
                    r#type: fields.r#type,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.type.RemoteLocation。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoteLocation {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.type.RemoteLocation/properties/host。
    #[serde(rename = "host")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub host: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.type.RemoteLocation/properties/port。
    #[serde(rename = "port")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub port: i64,
}
impl<'de> serde::Deserialize<'de> for RemoteLocation {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoteLocation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.type.RemoteLocation/properties/host。
                    #[serde(rename = "host")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub host: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.type.RemoteLocation/properties/port。
                    #[serde(rename = "port")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub port: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RemoteLocation {
                    host: fields.host,
                    port: fields.port,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Target.type.SessionID。
///
///协议原文：
///
///<pre>Unique identifier of attached debugging session.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SessionId(String);
impl SessionId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::target::types::SessionId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::target::types::SessionId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::target::types::SessionId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::target::types::SessionId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::target::types::SessionId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Target.type.TargetFilter。
///
///协议原文：
///
///<pre>The entries in TargetFilter are matched sequentially against targets and
///the first entry that matches determines if the target is included or not,
///depending on the value of `exclude` field in the entry.
///If filter is not specified, the one assumed is
///[{type: "browser", exclude: true}, {type: "tab", exclude: true}, {}]
///(i.e. include everything but `browser` and `tab`).</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct TargetFilter(Vec<crate::target::types::FilterEntry>);
impl TargetFilter {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::target::types::TargetFilter = serde_json::from_str("[]").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::target::types::TargetFilter::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("[]").unwrap());
    ///```
    pub fn new(value: Vec<crate::target::types::FilterEntry>) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::target::types::TargetFilter = serde_json::from_str("[]").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::target::types::TargetFilter::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("[]").unwrap());
    ///```
    pub fn into_inner(self) -> Vec<crate::target::types::FilterEntry> {
        self.0
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Target.type.TargetID。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct TargetId(String);
impl TargetId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::target::types::TargetId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::target::types::TargetId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::target::types::TargetId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::target::types::TargetId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::target::types::TargetId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Target.type.TargetInfo。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TargetInfo {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.type.TargetInfo/properties/targetId。
    #[serde(rename = "targetId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub target_id: crate::target::types::TargetId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.type.TargetInfo/properties/type。
    ///
    ///协议原文：
    ///
    ///<pre>List of types: https://source.chromium.org/chromium/chromium/src/+/main:content/browser/devtools/devtools_agent_host_impl.cc?ss=chromium&amp;q=f:devtools%20-f:out%20%22::kTypeTab%5B%5D%22</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.type.TargetInfo/properties/title。
    #[serde(rename = "title")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub title: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.type.TargetInfo/properties/url。
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.type.TargetInfo/properties/attached。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the target has an attached client.</pre>
    ///
    ///
    #[serde(rename = "attached")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub attached: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.type.TargetInfo/properties/parentId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the parent target, if any. For example, "iframe" target may have a "page" parent.</pre>
    ///
    ///
    #[serde(rename = "parentId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_id: Option<crate::target::types::TargetId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.type.TargetInfo/properties/openerId。
    ///
    ///协议原文：
    ///
    ///<pre>Opener target Id</pre>
    ///
    ///
    #[serde(rename = "openerId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub opener_id: Option<crate::target::types::TargetId>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.type.TargetInfo/properties/canAccessOpener。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the target has access to the originating window.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "canAccessOpener")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub can_access_opener: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.type.TargetInfo/properties/openerFrameId。
    ///
    ///协议原文：
    ///
    ///<pre>Frame id of originating window (is only set if target has an opener).</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "openerFrameId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub opener_frame_id: Option<crate::page::types::FrameId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.type.TargetInfo/properties/parentFrameId。
    ///
    ///协议原文：
    ///
    ///<pre>Id of the parent frame, present for "iframe" and "worker" targets. For nested workers,
    ///this is the "ancestor" frame that created the first worker in the nested chain.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "parentFrameId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_frame_id: Option<crate::page::types::FrameId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.type.TargetInfo/properties/browserContextId。此项为 experimental，端点可能不支持。
    #[serde(rename = "browserContextId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub browser_context_id: Option<crate::browser::types::BrowserContextId>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.type.TargetInfo/properties/subtype。
    ///
    ///协议原文：
    ///
    ///<pre>Provides additional details for specific target types. For example, for
    ///the type of "page", this may be set to "prerender".</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "subtype")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub subtype: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.type.TargetInfo/properties/embedderData。
    ///
    ///协议原文：
    ///
    ///<pre>Embedder-specific target metadata. This is only set for targets of
    ///type "tab".</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "embedderData")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub embedder_data: Option<std::collections::BTreeMap<String, serde_json::Value>>,
}
impl<'de> serde::Deserialize<'de> for TargetInfo {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TargetInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.type.TargetInfo/properties/targetId。
                    #[serde(rename = "targetId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub target_id: crate::target::types::TargetId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.type.TargetInfo/properties/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>List of types: https://source.chromium.org/chromium/chromium/src/+/main:content/browser/devtools/devtools_agent_host_impl.cc?ss=chromium&amp;q=f:devtools%20-f:out%20%22::kTypeTab%5B%5D%22</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.type.TargetInfo/properties/title。
                    #[serde(rename = "title")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub title: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.type.TargetInfo/properties/url。
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.type.TargetInfo/properties/attached。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the target has an attached client.</pre>
                    ///
                    ///
                    #[serde(rename = "attached")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub attached: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.type.TargetInfo/properties/parentId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the parent target, if any. For example, "iframe" target may have a "page" parent.</pre>
                    ///
                    ///
                    #[serde(rename = "parentId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub parent_id: Option<crate::target::types::TargetId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.type.TargetInfo/properties/openerId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Opener target Id</pre>
                    ///
                    ///
                    #[serde(rename = "openerId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub opener_id: Option<crate::target::types::TargetId>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Target.type.TargetInfo/properties/canAccessOpener。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the target has access to the originating window.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "canAccessOpener")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub can_access_opener: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.type.TargetInfo/properties/openerFrameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame id of originating window (is only set if target has an opener).</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "openerFrameId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub opener_frame_id: Option<crate::page::types::FrameId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.type.TargetInfo/properties/parentFrameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Id of the parent frame, present for "iframe" and "worker" targets. For nested workers,
                    ///this is the "ancestor" frame that created the first worker in the nested chain.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "parentFrameId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub parent_frame_id: Option<crate::page::types::FrameId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.type.TargetInfo/properties/browserContextId。此项为 experimental，端点可能不支持。
                    #[serde(rename = "browserContextId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub browser_context_id: Option<crate::browser::types::BrowserContextId>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.type.TargetInfo/properties/subtype。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Provides additional details for specific target types. For example, for
                    ///the type of "page", this may be set to "prerender".</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "subtype")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub subtype: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Target.type.TargetInfo/properties/embedderData。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Embedder-specific target metadata. This is only set for targets of
                    ///type "tab".</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "embedderData")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub embedder_data:
                        Option<std::collections::BTreeMap<String, serde_json::Value>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TargetInfo {
                    target_id: fields.target_id,
                    r#type: fields.r#type,
                    title: fields.title,
                    url: fields.url,
                    attached: fields.attached,
                    parent_id: fields.parent_id,
                    opener_id: fields.opener_id,
                    can_access_opener: fields.can_access_opener,
                    opener_frame_id: fields.opener_frame_id,
                    parent_frame_id: fields.parent_frame_id,
                    browser_context_id: fields.browser_context_id,
                    subtype: fields.subtype,
                    embedder_data: fields.embedder_data,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Target.type.WindowState。
///
///协议原文：
///
///<pre>The state of the target window.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum WindowState {
    ///协议值 "normal"。
    Normal,
    ///协议值 "minimized"。
    Minimized,
    ///协议值 "maximized"。
    Maximized,
    ///协议值 "fullscreen"。
    Fullscreen,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl WindowState {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::target::types::WindowState::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Normal => "normal",
            Self::Minimized => "minimized",
            Self::Maximized => "maximized",
            Self::Fullscreen => "fullscreen",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for WindowState {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for WindowState {
    fn from(value: String) -> Self {
        match value.as_str() {
            "normal" => WindowState::Normal,
            "minimized" => WindowState::Minimized,
            "maximized" => WindowState::Maximized,
            "fullscreen" => WindowState::Fullscreen,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for WindowState {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for WindowState {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WindowState;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "normal" => WindowState::Normal,
                    "minimized" => WindowState::Minimized,
                    "maximized" => WindowState::Maximized,
                    "fullscreen" => WindowState::Fullscreen,
                    _ => WindowState::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(WindowState::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
