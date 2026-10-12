///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.AdAncestry。
///
///协议原文：
///
///<pre>Encapsulates the script ancestry and the root script filter list rule that
///caused the resource or element to be labeled as an ad.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AdAncestry {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.AdAncestry/properties/ancestryChain。
    ///
    ///协议原文：
    ///
    ///<pre>A chain of `AdScriptIdentifier`s representing the ancestry of an ad
    ///script that led to the creation of a resource or element. The chain is
    ///ordered from the script itself (lowest level) up to its root ancestor
    ///that was flagged by a filter list.</pre>
    ///
    ///
    #[serde(rename = "ancestryChain")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub ancestry_chain: Vec<crate::network::types::AdScriptIdentifier>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.AdAncestry/properties/rootScriptFilterlistRule。
    ///
    ///协议原文：
    ///
    ///<pre>The filter list rule that caused the root (last) script in
    ///`ancestryChain` to be tagged as an ad.</pre>
    ///
    ///
    #[serde(rename = "rootScriptFilterlistRule")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub root_script_filterlist_rule: Option<String>,
}
impl<'de> serde::Deserialize<'de> for AdAncestry {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AdAncestry;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.AdAncestry/properties/ancestryChain。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A chain of `AdScriptIdentifier`s representing the ancestry of an ad
                    ///script that led to the creation of a resource or element. The chain is
                    ///ordered from the script itself (lowest level) up to its root ancestor
                    ///that was flagged by a filter list.</pre>
                    ///
                    ///
                    #[serde(rename = "ancestryChain")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub ancestry_chain: Vec<crate::network::types::AdScriptIdentifier>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.AdAncestry/properties/rootScriptFilterlistRule。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The filter list rule that caused the root (last) script in
                    ///`ancestryChain` to be tagged as an ad.</pre>
                    ///
                    ///
                    #[serde(rename = "rootScriptFilterlistRule")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub root_script_filterlist_rule: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AdAncestry {
                    ancestry_chain: fields.ancestry_chain,
                    root_script_filterlist_rule: fields.root_script_filterlist_rule,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.AdProvenance。
///
///协议原文：
///
///<pre>Represents the provenance of an ad resource or element. Only one of
///`filterlistRule` or `adScriptAncestry` can be set. If `filterlistRule`
///is provided, the resource URL directly matches a filter list rule. If
///`adScriptAncestry` is provided, an ad script initiated the resource fetch or
///appended the element to the DOM. If neither is provided, the entity is
///known to be an ad, but provenance tracking information is unavailable.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AdProvenance {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.AdProvenance/properties/filterlistRule。
    ///
    ///协议原文：
    ///
    ///<pre>The filterlist rule that matched, if any.</pre>
    ///
    ///
    #[serde(rename = "filterlistRule")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub filterlist_rule: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.AdProvenance/properties/adScriptAncestry。
    ///
    ///协议原文：
    ///
    ///<pre>The script ancestry that created the ad, if any.
    ///Note: depending on the context, this may represent the full ancestry up
    ///to the root script, or it may contain only one script representing the
    ///immediate ancestor.</pre>
    ///
    ///
    #[serde(rename = "adScriptAncestry")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub ad_script_ancestry: Option<crate::network::types::AdAncestry>,
}
impl<'de> serde::Deserialize<'de> for AdProvenance {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AdProvenance;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.AdProvenance/properties/filterlistRule。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The filterlist rule that matched, if any.</pre>
                    ///
                    ///
                    #[serde(rename = "filterlistRule")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub filterlist_rule: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.AdProvenance/properties/adScriptAncestry。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The script ancestry that created the ad, if any.
                    ///Note: depending on the context, this may represent the full ancestry up
                    ///to the root script, or it may contain only one script representing the
                    ///immediate ancestor.</pre>
                    ///
                    ///
                    #[serde(rename = "adScriptAncestry")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub ad_script_ancestry: Option<crate::network::types::AdAncestry>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AdProvenance {
                    filterlist_rule: fields.filterlist_rule,
                    ad_script_ancestry: fields.ad_script_ancestry,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.AdScriptIdentifier。
///
///协议原文：
///
///<pre>Identifies the script on the stack that caused a resource or element to be
///labeled as an ad. For resources, this indicates the context that triggered
///the fetch. For elements, this indicates the context that caused the element
///to be appended to the DOM.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AdScriptIdentifier {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.AdScriptIdentifier/properties/scriptId。
    ///
    ///协议原文：
    ///
    ///<pre>The script's V8 identifier.</pre>
    ///
    ///
    #[serde(rename = "scriptId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub script_id: crate::runtime::types::ScriptId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.AdScriptIdentifier/properties/debuggerId。
    ///
    ///协议原文：
    ///
    ///<pre>V8's debugging ID for the v8::Context.</pre>
    ///
    ///
    #[serde(rename = "debuggerId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub debugger_id: crate::runtime::types::UniqueDebuggerId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.AdScriptIdentifier/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>The script's url (or generated name based on id if inline script).</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
}
impl<'de> serde::Deserialize<'de> for AdScriptIdentifier {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AdScriptIdentifier;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.AdScriptIdentifier/properties/scriptId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The script's V8 identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "scriptId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub script_id: crate::runtime::types::ScriptId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.AdScriptIdentifier/properties/debuggerId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>V8's debugging ID for the v8::Context.</pre>
                    ///
                    ///
                    #[serde(rename = "debuggerId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub debugger_id: crate::runtime::types::UniqueDebuggerId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.AdScriptIdentifier/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The script's url (or generated name based on id if inline script).</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AdScriptIdentifier {
                    script_id: fields.script_id,
                    debugger_id: fields.debugger_id,
                    name: fields.name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.AlternateProtocolUsage。
///
///协议原文：
///
///<pre>The reason why Chrome uses a specific transport protocol for HTTP semantics.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AlternateProtocolUsage {
    ///协议值 "alternativeJobWonWithoutRace"。
    AlternativeJobWonWithoutRace,
    ///协议值 "alternativeJobWonRace"。
    AlternativeJobWonRace,
    ///协议值 "mainJobWonRace"。
    MainJobWonRace,
    ///协议值 "mappingMissing"。
    MappingMissing,
    ///协议值 "broken"。
    Broken,
    ///协议值 "dnsAlpnH3JobWonWithoutRace"。
    DnsAlpnH3JobWonWithoutRace,
    ///协议值 "dnsAlpnH3JobWonRace"。
    DnsAlpnH3JobWonRace,
    ///协议值 "unspecifiedReason"。
    UnspecifiedReason,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl AlternateProtocolUsage {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::AlternateProtocolUsage::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::AlternativeJobWonWithoutRace => "alternativeJobWonWithoutRace",
            Self::AlternativeJobWonRace => "alternativeJobWonRace",
            Self::MainJobWonRace => "mainJobWonRace",
            Self::MappingMissing => "mappingMissing",
            Self::Broken => "broken",
            Self::DnsAlpnH3JobWonWithoutRace => "dnsAlpnH3JobWonWithoutRace",
            Self::DnsAlpnH3JobWonRace => "dnsAlpnH3JobWonRace",
            Self::UnspecifiedReason => "unspecifiedReason",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for AlternateProtocolUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for AlternateProtocolUsage {
    fn from(value: String) -> Self {
        match value.as_str() {
            "alternativeJobWonWithoutRace" => AlternateProtocolUsage::AlternativeJobWonWithoutRace,
            "alternativeJobWonRace" => AlternateProtocolUsage::AlternativeJobWonRace,
            "mainJobWonRace" => AlternateProtocolUsage::MainJobWonRace,
            "mappingMissing" => AlternateProtocolUsage::MappingMissing,
            "broken" => AlternateProtocolUsage::Broken,
            "dnsAlpnH3JobWonWithoutRace" => AlternateProtocolUsage::DnsAlpnH3JobWonWithoutRace,
            "dnsAlpnH3JobWonRace" => AlternateProtocolUsage::DnsAlpnH3JobWonRace,
            "unspecifiedReason" => AlternateProtocolUsage::UnspecifiedReason,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for AlternateProtocolUsage {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AlternateProtocolUsage {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AlternateProtocolUsage;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "alternativeJobWonWithoutRace" => {
                        AlternateProtocolUsage::AlternativeJobWonWithoutRace
                    }
                    "alternativeJobWonRace" => AlternateProtocolUsage::AlternativeJobWonRace,
                    "mainJobWonRace" => AlternateProtocolUsage::MainJobWonRace,
                    "mappingMissing" => AlternateProtocolUsage::MappingMissing,
                    "broken" => AlternateProtocolUsage::Broken,
                    "dnsAlpnH3JobWonWithoutRace" => {
                        AlternateProtocolUsage::DnsAlpnH3JobWonWithoutRace
                    }
                    "dnsAlpnH3JobWonRace" => AlternateProtocolUsage::DnsAlpnH3JobWonRace,
                    "unspecifiedReason" => AlternateProtocolUsage::UnspecifiedReason,
                    _ => AlternateProtocolUsage::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(AlternateProtocolUsage::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.AssociatedCookie。
///
///协议原文：
///
///<pre>A cookie associated with the request which may or may not be sent with it.
///Includes the cookies itself and reasons for blocking or exemption.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AssociatedCookie {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.AssociatedCookie/properties/cookie。
    ///
    ///协议原文：
    ///
    ///<pre>The cookie object representing the cookie which was not sent.</pre>
    ///
    ///
    #[serde(rename = "cookie")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cookie: crate::network::types::Cookie,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.AssociatedCookie/properties/blockedReasons。
    ///
    ///协议原文：
    ///
    ///<pre>The reason(s) the cookie was blocked. If empty means the cookie is included.</pre>
    ///
    ///
    #[serde(rename = "blockedReasons")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub blocked_reasons: Vec<crate::network::types::CookieBlockedReason>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.AssociatedCookie/properties/exemptionReason。
    ///
    ///协议原文：
    ///
    ///<pre>The reason the cookie should have been blocked by 3PCD but is exempted. A cookie could
    ///only have at most one exemption reason.</pre>
    ///
    ///
    #[serde(rename = "exemptionReason")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub exemption_reason: Option<crate::network::types::CookieExemptionReason>,
}
impl<'de> serde::Deserialize<'de> for AssociatedCookie {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AssociatedCookie;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.AssociatedCookie/properties/cookie。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The cookie object representing the cookie which was not sent.</pre>
                    ///
                    ///
                    #[serde(rename = "cookie")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cookie: crate::network::types::Cookie,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.AssociatedCookie/properties/blockedReasons。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The reason(s) the cookie was blocked. If empty means the cookie is included.</pre>
                    ///
                    ///
                    #[serde(rename = "blockedReasons")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub blocked_reasons: Vec<crate::network::types::CookieBlockedReason>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.AssociatedCookie/properties/exemptionReason。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The reason the cookie should have been blocked by 3PCD but is exempted. A cookie could
                    ///only have at most one exemption reason.</pre>
                    ///
                    ///
                    #[serde(rename = "exemptionReason")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub exemption_reason: Option<crate::network::types::CookieExemptionReason>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AssociatedCookie {
                    cookie: fields.cookie,
                    blocked_reasons: fields.blocked_reasons,
                    exemption_reason: fields.exemption_reason,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.AuthChallenge/properties/source。
///
///协议原文：
///
///<pre>Source of the authentication challenge.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AuthChallengeSource {
    ///协议值 "Server"。
    Server,
    ///协议值 "Proxy"。
    Proxy,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl AuthChallengeSource {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::AuthChallengeSource::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Server => "Server",
            Self::Proxy => "Proxy",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for AuthChallengeSource {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for AuthChallengeSource {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Server" => AuthChallengeSource::Server,
            "Proxy" => AuthChallengeSource::Proxy,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for AuthChallengeSource {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AuthChallengeSource {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AuthChallengeSource;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Server" => AuthChallengeSource::Server,
                    "Proxy" => AuthChallengeSource::Proxy,
                    _ => AuthChallengeSource::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(AuthChallengeSource::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.AuthChallenge。
///
///协议原文：
///
///<pre>Authorization challenge for HTTP status code 401 or 407.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AuthChallenge {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.AuthChallenge/properties/source。
    ///
    ///协议原文：
    ///
    ///<pre>Source of the authentication challenge.</pre>
    ///
    ///
    #[serde(rename = "source")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub source: Option<AuthChallengeSource>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.AuthChallenge/properties/origin。
    ///
    ///协议原文：
    ///
    ///<pre>Origin of the challenger.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.AuthChallenge/properties/scheme。
    ///
    ///协议原文：
    ///
    ///<pre>The authentication scheme used, such as basic or digest</pre>
    ///
    ///
    #[serde(rename = "scheme")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub scheme: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.AuthChallenge/properties/realm。
    ///
    ///协议原文：
    ///
    ///<pre>The realm of the challenge. May be empty.</pre>
    ///
    ///
    #[serde(rename = "realm")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub realm: String,
}
impl<'de> serde::Deserialize<'de> for AuthChallenge {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AuthChallenge;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.AuthChallenge/properties/source。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Source of the authentication challenge.</pre>
                    ///
                    ///
                    #[serde(rename = "source")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub source: Option<AuthChallengeSource>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.AuthChallenge/properties/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Origin of the challenger.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.AuthChallenge/properties/scheme。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The authentication scheme used, such as basic or digest</pre>
                    ///
                    ///
                    #[serde(rename = "scheme")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub scheme: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.AuthChallenge/properties/realm。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The realm of the challenge. May be empty.</pre>
                    ///
                    ///
                    #[serde(rename = "realm")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub realm: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AuthChallenge {
                    source: fields.source,
                    origin: fields.origin,
                    scheme: fields.scheme,
                    realm: fields.realm,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.AuthChallengeResponse/properties/response。
///
///协议原文：
///
///<pre>The decision on what to do in response to the authorization challenge.  Default means
///deferring to the default behavior of the net stack, which will likely either the Cancel
///authentication or display a popup dialog box.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AuthChallengeResponseResponse {
    ///协议值 "Default"。
    Default,
    ///协议值 "CancelAuth"。
    CancelAuth,
    ///协议值 "ProvideCredentials"。
    ProvideCredentials,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl AuthChallengeResponseResponse {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::AuthChallengeResponseResponse::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Default => "Default",
            Self::CancelAuth => "CancelAuth",
            Self::ProvideCredentials => "ProvideCredentials",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for AuthChallengeResponseResponse {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for AuthChallengeResponseResponse {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Default" => AuthChallengeResponseResponse::Default,
            "CancelAuth" => AuthChallengeResponseResponse::CancelAuth,
            "ProvideCredentials" => AuthChallengeResponseResponse::ProvideCredentials,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for AuthChallengeResponseResponse {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AuthChallengeResponseResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AuthChallengeResponseResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Default" => AuthChallengeResponseResponse::Default,
                    "CancelAuth" => AuthChallengeResponseResponse::CancelAuth,
                    "ProvideCredentials" => AuthChallengeResponseResponse::ProvideCredentials,
                    _ => AuthChallengeResponseResponse::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(AuthChallengeResponseResponse::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.AuthChallengeResponse。
///
///协议原文：
///
///<pre>Response to an AuthChallenge.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AuthChallengeResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.AuthChallengeResponse/properties/response。
    ///
    ///协议原文：
    ///
    ///<pre>The decision on what to do in response to the authorization challenge.  Default means
    ///deferring to the default behavior of the net stack, which will likely either the Cancel
    ///authentication or display a popup dialog box.</pre>
    ///
    ///
    #[serde(rename = "response")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub response: AuthChallengeResponseResponse,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.AuthChallengeResponse/properties/username。
    ///
    ///协议原文：
    ///
    ///<pre>The username to provide, possibly empty. Should only be set if response is
    ///ProvideCredentials.</pre>
    ///
    ///
    #[serde(rename = "username")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub username: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.AuthChallengeResponse/properties/password。
    ///
    ///协议原文：
    ///
    ///<pre>The password to provide, possibly empty. Should only be set if response is
    ///ProvideCredentials.</pre>
    ///
    ///
    #[serde(rename = "password")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub password: Option<String>,
}
impl<'de> serde::Deserialize<'de> for AuthChallengeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AuthChallengeResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.AuthChallengeResponse/properties/response。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The decision on what to do in response to the authorization challenge.  Default means
                    ///deferring to the default behavior of the net stack, which will likely either the Cancel
                    ///authentication or display a popup dialog box.</pre>
                    ///
                    ///
                    #[serde(rename = "response")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub response: AuthChallengeResponseResponse,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.AuthChallengeResponse/properties/username。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The username to provide, possibly empty. Should only be set if response is
                    ///ProvideCredentials.</pre>
                    ///
                    ///
                    #[serde(rename = "username")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub username: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.AuthChallengeResponse/properties/password。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The password to provide, possibly empty. Should only be set if response is
                    ///ProvideCredentials.</pre>
                    ///
                    ///
                    #[serde(rename = "password")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub password: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AuthChallengeResponse {
                    response: fields.response,
                    username: fields.username,
                    password: fields.password,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.BlockPattern。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct BlockPattern {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.BlockPattern/properties/urlPattern。
    ///
    ///协议原文：
    ///
    ///<pre>URL pattern to match. Patterns use the URLPattern constructor string syntax
    ///(https://urlpattern.spec.whatwg.org/) and must be absolute. Example: `*://*:*/*.css`.</pre>
    ///
    ///
    #[serde(rename = "urlPattern")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url_pattern: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.BlockPattern/properties/block。
    ///
    ///协议原文：
    ///
    ///<pre>Whether or not to block the pattern. If false, a matching request will not be blocked even if it matches a later
    ///`BlockPattern`.</pre>
    ///
    ///
    #[serde(rename = "block")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub block: bool,
}
impl<'de> serde::Deserialize<'de> for BlockPattern {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BlockPattern;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.BlockPattern/properties/urlPattern。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>URL pattern to match. Patterns use the URLPattern constructor string syntax
                    ///(https://urlpattern.spec.whatwg.org/) and must be absolute. Example: `*://*:*/*.css`.</pre>
                    ///
                    ///
                    #[serde(rename = "urlPattern")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url_pattern: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.BlockPattern/properties/block。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether or not to block the pattern. If false, a matching request will not be blocked even if it matches a later
                    ///`BlockPattern`.</pre>
                    ///
                    ///
                    #[serde(rename = "block")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub block: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(BlockPattern {
                    url_pattern: fields.url_pattern,
                    block: fields.block,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.BlockedReason。
///
///协议原文：
///
///<pre>The reason why request was blocked.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum BlockedReason {
    ///协议值 "other"。
    Other,
    ///协议值 "csp"。
    Csp,
    ///协议值 "mixed-content"。
    MixedContent,
    ///协议值 "origin"。
    Origin,
    ///协议值 "inspector"。
    Inspector,
    ///协议值 "integrity"。
    Integrity,
    ///协议值 "subresource-filter"。
    SubresourceFilter,
    ///协议值 "content-type"。
    ContentType,
    ///协议值 "coep-frame-resource-needs-coep-header"。
    CoepFrameResourceNeedsCoepHeader,
    ///协议值 "coop-sandboxed-iframe-cannot-navigate-to-coop-page"。
    CoopSandboxedIframeCannotNavigateToCoopPage,
    ///协议值 "corp-not-same-origin"。
    CorpNotSameOrigin,
    ///协议值 "corp-not-same-origin-after-defaulted-to-same-origin-by-coep"。
    CorpNotSameOriginAfterDefaultedToSameOriginByCoep,
    ///协议值 "corp-not-same-origin-after-defaulted-to-same-origin-by-dip"。
    CorpNotSameOriginAfterDefaultedToSameOriginByDip,
    ///协议值 "corp-not-same-origin-after-defaulted-to-same-origin-by-coep-and-dip"。
    CorpNotSameOriginAfterDefaultedToSameOriginByCoepAndDip,
    ///协议值 "corp-not-same-site"。
    CorpNotSameSite,
    ///协议值 "sri-message-signature-mismatch"。
    SriMessageSignatureMismatch,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl BlockedReason {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::BlockedReason::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Other => "other",
            Self::Csp => "csp",
            Self::MixedContent => "mixed-content",
            Self::Origin => "origin",
            Self::Inspector => "inspector",
            Self::Integrity => "integrity",
            Self::SubresourceFilter => "subresource-filter",
            Self::ContentType => "content-type",
            Self::CoepFrameResourceNeedsCoepHeader => "coep-frame-resource-needs-coep-header",
            Self::CoopSandboxedIframeCannotNavigateToCoopPage => {
                "coop-sandboxed-iframe-cannot-navigate-to-coop-page"
            }
            Self::CorpNotSameOrigin => "corp-not-same-origin",
            Self::CorpNotSameOriginAfterDefaultedToSameOriginByCoep => {
                "corp-not-same-origin-after-defaulted-to-same-origin-by-coep"
            }
            Self::CorpNotSameOriginAfterDefaultedToSameOriginByDip => {
                "corp-not-same-origin-after-defaulted-to-same-origin-by-dip"
            }
            Self::CorpNotSameOriginAfterDefaultedToSameOriginByCoepAndDip => {
                "corp-not-same-origin-after-defaulted-to-same-origin-by-coep-and-dip"
            }
            Self::CorpNotSameSite => "corp-not-same-site",
            Self::SriMessageSignatureMismatch => "sri-message-signature-mismatch",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for BlockedReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for BlockedReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "other" => BlockedReason::Other,
            "csp" => BlockedReason::Csp,
            "mixed-content" => BlockedReason::MixedContent,
            "origin" => BlockedReason::Origin,
            "inspector" => BlockedReason::Inspector,
            "integrity" => BlockedReason::Integrity,
            "subresource-filter" => BlockedReason::SubresourceFilter,
            "content-type" => BlockedReason::ContentType,
            "coep-frame-resource-needs-coep-header" => {
                BlockedReason::CoepFrameResourceNeedsCoepHeader
            }
            "coop-sandboxed-iframe-cannot-navigate-to-coop-page" => {
                BlockedReason::CoopSandboxedIframeCannotNavigateToCoopPage
            }
            "corp-not-same-origin" => BlockedReason::CorpNotSameOrigin,
            "corp-not-same-origin-after-defaulted-to-same-origin-by-coep" => {
                BlockedReason::CorpNotSameOriginAfterDefaultedToSameOriginByCoep
            }
            "corp-not-same-origin-after-defaulted-to-same-origin-by-dip" => {
                BlockedReason::CorpNotSameOriginAfterDefaultedToSameOriginByDip
            }
            "corp-not-same-origin-after-defaulted-to-same-origin-by-coep-and-dip" => {
                BlockedReason::CorpNotSameOriginAfterDefaultedToSameOriginByCoepAndDip
            }
            "corp-not-same-site" => BlockedReason::CorpNotSameSite,
            "sri-message-signature-mismatch" => BlockedReason::SriMessageSignatureMismatch,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for BlockedReason {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BlockedReason {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BlockedReason;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "other" => BlockedReason::Other,
                    "csp" => BlockedReason::Csp,
                    "mixed-content" => BlockedReason::MixedContent,
                    "origin" => BlockedReason::Origin,
                    "inspector" => BlockedReason::Inspector,
                    "integrity" => BlockedReason::Integrity,
                    "subresource-filter" => BlockedReason::SubresourceFilter,
                    "content-type" => BlockedReason::ContentType,
                    "coep-frame-resource-needs-coep-header" => {
                        BlockedReason::CoepFrameResourceNeedsCoepHeader
                    }
                    "coop-sandboxed-iframe-cannot-navigate-to-coop-page" => {
                        BlockedReason::CoopSandboxedIframeCannotNavigateToCoopPage
                    }
                    "corp-not-same-origin" => BlockedReason::CorpNotSameOrigin,
                    "corp-not-same-origin-after-defaulted-to-same-origin-by-coep" => {
                        BlockedReason::CorpNotSameOriginAfterDefaultedToSameOriginByCoep
                    }
                    "corp-not-same-origin-after-defaulted-to-same-origin-by-dip" => {
                        BlockedReason::CorpNotSameOriginAfterDefaultedToSameOriginByDip
                    }
                    "corp-not-same-origin-after-defaulted-to-same-origin-by-coep-and-dip" => {
                        BlockedReason::CorpNotSameOriginAfterDefaultedToSameOriginByCoepAndDip
                    }
                    "corp-not-same-site" => BlockedReason::CorpNotSameSite,
                    "sri-message-signature-mismatch" => BlockedReason::SriMessageSignatureMismatch,
                    _ => BlockedReason::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(BlockedReason::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.BlockedSetCookieWithReason。
///
///协议原文：
///
///<pre>A cookie which was not stored from a response with the corresponding reason.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct BlockedSetCookieWithReason {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.BlockedSetCookieWithReason/properties/blockedReasons。
    ///
    ///协议原文：
    ///
    ///<pre>The reason(s) this cookie was blocked.</pre>
    ///
    ///
    #[serde(rename = "blockedReasons")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub blocked_reasons: Vec<crate::network::types::SetCookieBlockedReason>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.BlockedSetCookieWithReason/properties/cookieLine。
    ///
    ///协议原文：
    ///
    ///<pre>The string representing this individual cookie as it would appear in the header.
    ///This is not the entire "cookie" or "set-cookie" header which could have multiple cookies.</pre>
    ///
    ///
    #[serde(rename = "cookieLine")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cookie_line: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.BlockedSetCookieWithReason/properties/cookie。
    ///
    ///协议原文：
    ///
    ///<pre>The cookie object which represents the cookie which was not stored. It is optional because
    ///sometimes complete cookie information is not available, such as in the case of parsing
    ///errors.</pre>
    ///
    ///
    #[serde(rename = "cookie")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub cookie: Option<crate::network::types::Cookie>,
}
impl<'de> serde::Deserialize<'de> for BlockedSetCookieWithReason {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BlockedSetCookieWithReason;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.BlockedSetCookieWithReason/properties/blockedReasons。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The reason(s) this cookie was blocked.</pre>
                    ///
                    ///
                    #[serde(rename = "blockedReasons")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub blocked_reasons: Vec<crate::network::types::SetCookieBlockedReason>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.BlockedSetCookieWithReason/properties/cookieLine。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The string representing this individual cookie as it would appear in the header.
                    ///This is not the entire "cookie" or "set-cookie" header which could have multiple cookies.</pre>
                    ///
                    ///
                    #[serde(rename = "cookieLine")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cookie_line: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.BlockedSetCookieWithReason/properties/cookie。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The cookie object which represents the cookie which was not stored. It is optional because
                    ///sometimes complete cookie information is not available, such as in the case of parsing
                    ///errors.</pre>
                    ///
                    ///
                    #[serde(rename = "cookie")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub cookie: Option<crate::network::types::Cookie>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(BlockedSetCookieWithReason {
                    blocked_reasons: fields.blocked_reasons,
                    cookie_line: fields.cookie_line,
                    cookie: fields.cookie,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.CachedResource。
///
///协议原文：
///
///<pre>Information about the cached resource.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CachedResource {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.CachedResource/properties/url。
    ///
    ///协议原文：
    ///
    ///<pre>Resource URL. This is the url of the original network request.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.CachedResource/properties/type。
    ///
    ///协议原文：
    ///
    ///<pre>Type of this resource.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: crate::network::types::ResourceType,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CachedResource/properties/response。
    ///
    ///协议原文：
    ///
    ///<pre>Cached response data.</pre>
    ///
    ///
    #[serde(rename = "response")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub response: Option<crate::network::types::Response>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.CachedResource/properties/bodySize。
    ///
    ///协议原文：
    ///
    ///<pre>Cached response body size.</pre>
    ///
    ///
    #[serde(rename = "bodySize")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub body_size: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for CachedResource {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CachedResource;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.CachedResource/properties/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Resource URL. This is the url of the original network request.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.CachedResource/properties/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Type of this resource.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: crate::network::types::ResourceType,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CachedResource/properties/response。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cached response data.</pre>
                    ///
                    ///
                    #[serde(rename = "response")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub response: Option<crate::network::types::Response>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.CachedResource/properties/bodySize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cached response body size.</pre>
                    ///
                    ///
                    #[serde(rename = "bodySize")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub body_size: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CachedResource {
                    url: fields.url,
                    r#type: fields.r#type,
                    response: fields.response,
                    body_size: fields.body_size,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.CertificateTransparencyCompliance。
///
///协议原文：
///
///<pre>Whether the request complied with Certificate Transparency policy.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CertificateTransparencyCompliance {
    ///协议值 "unknown"。
    KnownUnknown,
    ///协议值 "not-compliant"。
    NotCompliant,
    ///协议值 "compliant"。
    Compliant,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl CertificateTransparencyCompliance {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::CertificateTransparencyCompliance::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::KnownUnknown => "unknown",
            Self::NotCompliant => "not-compliant",
            Self::Compliant => "compliant",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for CertificateTransparencyCompliance {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for CertificateTransparencyCompliance {
    fn from(value: String) -> Self {
        match value.as_str() {
            "unknown" => CertificateTransparencyCompliance::KnownUnknown,
            "not-compliant" => CertificateTransparencyCompliance::NotCompliant,
            "compliant" => CertificateTransparencyCompliance::Compliant,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for CertificateTransparencyCompliance {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CertificateTransparencyCompliance {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CertificateTransparencyCompliance;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "unknown" => CertificateTransparencyCompliance::KnownUnknown,
                    "not-compliant" => CertificateTransparencyCompliance::NotCompliant,
                    "compliant" => CertificateTransparencyCompliance::Compliant,
                    _ => CertificateTransparencyCompliance::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(CertificateTransparencyCompliance::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.ChallengeEventDetails/properties/challengeResult。
///
///协议原文：
///
///<pre>The result of a challenge.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ChallengeEventDetailsChallengeResult {
    ///协议值 "Success"。
    Success,
    ///协议值 "NoSessionId"。
    NoSessionId,
    ///协议值 "NoSessionMatch"。
    NoSessionMatch,
    ///协议值 "CantSetBoundCookie"。
    CantSetBoundCookie,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ChallengeEventDetailsChallengeResult {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::ChallengeEventDetailsChallengeResult::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Success => "Success",
            Self::NoSessionId => "NoSessionId",
            Self::NoSessionMatch => "NoSessionMatch",
            Self::CantSetBoundCookie => "CantSetBoundCookie",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ChallengeEventDetailsChallengeResult {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ChallengeEventDetailsChallengeResult {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Success" => ChallengeEventDetailsChallengeResult::Success,
            "NoSessionId" => ChallengeEventDetailsChallengeResult::NoSessionId,
            "NoSessionMatch" => ChallengeEventDetailsChallengeResult::NoSessionMatch,
            "CantSetBoundCookie" => ChallengeEventDetailsChallengeResult::CantSetBoundCookie,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ChallengeEventDetailsChallengeResult {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ChallengeEventDetailsChallengeResult {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ChallengeEventDetailsChallengeResult;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Success" => ChallengeEventDetailsChallengeResult::Success,
                    "NoSessionId" => ChallengeEventDetailsChallengeResult::NoSessionId,
                    "NoSessionMatch" => ChallengeEventDetailsChallengeResult::NoSessionMatch,
                    "CantSetBoundCookie" => {
                        ChallengeEventDetailsChallengeResult::CantSetBoundCookie
                    }
                    _ => ChallengeEventDetailsChallengeResult::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ChallengeEventDetailsChallengeResult::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.ChallengeEventDetails。
///
///协议原文：
///
///<pre>Session event details specific to challenges.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ChallengeEventDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ChallengeEventDetails/properties/challengeResult。
    ///
    ///协议原文：
    ///
    ///<pre>The result of a challenge.</pre>
    ///
    ///
    #[serde(rename = "challengeResult")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub challenge_result: ChallengeEventDetailsChallengeResult,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ChallengeEventDetails/properties/challenge。
    ///
    ///协议原文：
    ///
    ///<pre>The challenge set.</pre>
    ///
    ///
    #[serde(rename = "challenge")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub challenge: String,
}
impl<'de> serde::Deserialize<'de> for ChallengeEventDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ChallengeEventDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ChallengeEventDetails/properties/challengeResult。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The result of a challenge.</pre>
                    ///
                    ///
                    #[serde(rename = "challengeResult")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub challenge_result: ChallengeEventDetailsChallengeResult,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ChallengeEventDetails/properties/challenge。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The challenge set.</pre>
                    ///
                    ///
                    #[serde(rename = "challenge")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub challenge: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ChallengeEventDetails {
                    challenge_result: fields.challenge_result,
                    challenge: fields.challenge,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.ClientSecurityState。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClientSecurityState {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ClientSecurityState/properties/initiatorIsSecureContext。
    #[serde(rename = "initiatorIsSecureContext")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub initiator_is_secure_context: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ClientSecurityState/properties/initiatorIPAddressSpace。
    #[serde(rename = "initiatorIPAddressSpace")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub initiator_ip_address_space: crate::network::types::IpAddressSpace,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ClientSecurityState/properties/localNetworkAccessRequestPolicy。
    #[serde(rename = "localNetworkAccessRequestPolicy")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub local_network_access_request_policy: crate::network::types::LocalNetworkAccessRequestPolicy,
}
impl<'de> serde::Deserialize<'de> for ClientSecurityState {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClientSecurityState;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ClientSecurityState/properties/initiatorIsSecureContext。
                    #[serde(rename = "initiatorIsSecureContext")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub initiator_is_secure_context: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ClientSecurityState/properties/initiatorIPAddressSpace。
                    #[serde(rename = "initiatorIPAddressSpace")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub initiator_ip_address_space: crate::network::types::IpAddressSpace,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ClientSecurityState/properties/localNetworkAccessRequestPolicy。
                    #[serde(rename = "localNetworkAccessRequestPolicy")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub local_network_access_request_policy:
                        crate::network::types::LocalNetworkAccessRequestPolicy,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ClientSecurityState {
                    initiator_is_secure_context: fields.initiator_is_secure_context,
                    initiator_ip_address_space: fields.initiator_ip_address_space,
                    local_network_access_request_policy: fields.local_network_access_request_policy,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.ConnectTiming。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ConnectTiming {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ConnectTiming/properties/requestTime。
    ///
    ///协议原文：
    ///
    ///<pre>Timing's requestTime is a baseline in seconds, while the other numbers are ticks in
    ///milliseconds relatively to this requestTime. Matches ResourceTiming's requestTime for
    ///the same request (but not for redirected requests).</pre>
    ///
    ///
    #[serde(rename = "requestTime")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_time: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for ConnectTiming {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ConnectTiming;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ConnectTiming/properties/requestTime。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Timing's requestTime is a baseline in seconds, while the other numbers are ticks in
                    ///milliseconds relatively to this requestTime. Matches ResourceTiming's requestTime for
                    ///the same request (but not for redirected requests).</pre>
                    ///
                    ///
                    #[serde(rename = "requestTime")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_time: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ConnectTiming {
                    request_time: fields.request_time,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.ConnectionType。
///
///协议原文：
///
///<pre>The underlying connection technology that the browser is supposedly using.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConnectionType {
    ///协议值 "none"。
    None,
    ///协议值 "cellular2g"。
    Cellular2g,
    ///协议值 "cellular3g"。
    Cellular3g,
    ///协议值 "cellular4g"。
    Cellular4g,
    ///协议值 "bluetooth"。
    Bluetooth,
    ///协议值 "ethernet"。
    Ethernet,
    ///协议值 "wifi"。
    Wifi,
    ///协议值 "wimax"。
    Wimax,
    ///协议值 "other"。
    Other,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ConnectionType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::ConnectionType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::None => "none",
            Self::Cellular2g => "cellular2g",
            Self::Cellular3g => "cellular3g",
            Self::Cellular4g => "cellular4g",
            Self::Bluetooth => "bluetooth",
            Self::Ethernet => "ethernet",
            Self::Wifi => "wifi",
            Self::Wimax => "wimax",
            Self::Other => "other",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ConnectionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ConnectionType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "none" => ConnectionType::None,
            "cellular2g" => ConnectionType::Cellular2g,
            "cellular3g" => ConnectionType::Cellular3g,
            "cellular4g" => ConnectionType::Cellular4g,
            "bluetooth" => ConnectionType::Bluetooth,
            "ethernet" => ConnectionType::Ethernet,
            "wifi" => ConnectionType::Wifi,
            "wimax" => ConnectionType::Wimax,
            "other" => ConnectionType::Other,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ConnectionType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ConnectionType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ConnectionType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "none" => ConnectionType::None,
                    "cellular2g" => ConnectionType::Cellular2g,
                    "cellular3g" => ConnectionType::Cellular3g,
                    "cellular4g" => ConnectionType::Cellular4g,
                    "bluetooth" => ConnectionType::Bluetooth,
                    "ethernet" => ConnectionType::Ethernet,
                    "wifi" => ConnectionType::Wifi,
                    "wimax" => ConnectionType::Wimax,
                    "other" => ConnectionType::Other,
                    _ => ConnectionType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ConnectionType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.ContentSecurityPolicySource。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ContentSecurityPolicySource {
    ///协议值 "HTTP"。
    Http,
    ///协议值 "Meta"。
    Meta,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ContentSecurityPolicySource {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::ContentSecurityPolicySource::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Http => "HTTP",
            Self::Meta => "Meta",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ContentSecurityPolicySource {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ContentSecurityPolicySource {
    fn from(value: String) -> Self {
        match value.as_str() {
            "HTTP" => ContentSecurityPolicySource::Http,
            "Meta" => ContentSecurityPolicySource::Meta,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ContentSecurityPolicySource {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ContentSecurityPolicySource {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContentSecurityPolicySource;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "HTTP" => ContentSecurityPolicySource::Http,
                    "Meta" => ContentSecurityPolicySource::Meta,
                    _ => ContentSecurityPolicySource::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ContentSecurityPolicySource::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.ContentSecurityPolicyStatus。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ContentSecurityPolicyStatus {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ContentSecurityPolicyStatus/properties/effectiveDirectives。
    #[serde(rename = "effectiveDirectives")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub effective_directives: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ContentSecurityPolicyStatus/properties/isEnforced。
    #[serde(rename = "isEnforced")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub is_enforced: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ContentSecurityPolicyStatus/properties/source。
    #[serde(rename = "source")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub source: crate::network::types::ContentSecurityPolicySource,
}
impl<'de> serde::Deserialize<'de> for ContentSecurityPolicyStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContentSecurityPolicyStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ContentSecurityPolicyStatus/properties/effectiveDirectives。
                    #[serde(rename = "effectiveDirectives")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub effective_directives: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ContentSecurityPolicyStatus/properties/isEnforced。
                    #[serde(rename = "isEnforced")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub is_enforced: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ContentSecurityPolicyStatus/properties/source。
                    #[serde(rename = "source")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub source: crate::network::types::ContentSecurityPolicySource,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ContentSecurityPolicyStatus {
                    effective_directives: fields.effective_directives,
                    is_enforced: fields.is_enforced,
                    source: fields.source,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.Cookie。
///
///协议原文：
///
///<pre>Cookie object</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct Cookie {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Cookie/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Cookie/properties/value。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie value.</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Cookie/properties/domain。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie domain.</pre>
    ///
    ///
    #[serde(rename = "domain")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub domain: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Cookie/properties/path。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie path.</pre>
    ///
    ///
    #[serde(rename = "path")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub path: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Cookie/properties/expires。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie expiration date as the number of seconds since the UNIX epoch.
    ///The value is set to -1 if the expiry date is not set.
    ///The value can be null for values that cannot be represented in
    ///JSON (±Inf).</pre>
    ///
    ///
    #[serde(rename = "expires")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub expires: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Cookie/properties/size。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie size.</pre>
    ///
    ///
    #[serde(rename = "size")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub size: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Cookie/properties/httpOnly。
    ///
    ///协议原文：
    ///
    ///<pre>True if cookie is http-only.</pre>
    ///
    ///
    #[serde(rename = "httpOnly")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub http_only: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Cookie/properties/secure。
    ///
    ///协议原文：
    ///
    ///<pre>True if cookie is secure.</pre>
    ///
    ///
    #[serde(rename = "secure")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub secure: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Cookie/properties/session。
    ///
    ///协议原文：
    ///
    ///<pre>True in case of session cookie.</pre>
    ///
    ///
    #[serde(rename = "session")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub session: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Cookie/properties/sameSite。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie SameSite type.</pre>
    ///
    ///
    #[serde(rename = "sameSite")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub same_site: Option<crate::network::types::CookieSameSite>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Cookie/properties/priority。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie Priority</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "priority")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub priority: crate::network::types::CookiePriority,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Cookie/properties/sourceScheme。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie source scheme type.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "sourceScheme")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub source_scheme: crate::network::types::CookieSourceScheme,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Cookie/properties/sourcePort。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie source port. Valid values are {-1, [1, 65535]}, -1 indicates an unspecified port.
    ///An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.
    ///This is a temporary ability and it will be removed in the future.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "sourcePort")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub source_port: i64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Cookie/properties/partitionKey。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie partition key.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "partitionKey")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub partition_key: Option<crate::network::types::CookiePartitionKey>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Cookie/properties/partitionKeyOpaque。
    ///
    ///协议原文：
    ///
    ///<pre>True if cookie partition key is opaque.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "partitionKeyOpaque")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub partition_key_opaque: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for Cookie {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Cookie;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Cookie/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Cookie/properties/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie value.</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Cookie/properties/domain。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie domain.</pre>
                    ///
                    ///
                    #[serde(rename = "domain")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub domain: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Cookie/properties/path。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie path.</pre>
                    ///
                    ///
                    #[serde(rename = "path")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub path: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Cookie/properties/expires。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie expiration date as the number of seconds since the UNIX epoch.
                    ///The value is set to -1 if the expiry date is not set.
                    ///The value can be null for values that cannot be represented in
                    ///JSON (±Inf).</pre>
                    ///
                    ///
                    #[serde(rename = "expires")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub expires: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Cookie/properties/size。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie size.</pre>
                    ///
                    ///
                    #[serde(rename = "size")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub size: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Cookie/properties/httpOnly。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if cookie is http-only.</pre>
                    ///
                    ///
                    #[serde(rename = "httpOnly")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub http_only: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Cookie/properties/secure。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if cookie is secure.</pre>
                    ///
                    ///
                    #[serde(rename = "secure")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub secure: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Cookie/properties/session。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True in case of session cookie.</pre>
                    ///
                    ///
                    #[serde(rename = "session")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub session: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Cookie/properties/sameSite。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie SameSite type.</pre>
                    ///
                    ///
                    #[serde(rename = "sameSite")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub same_site: Option<crate::network::types::CookieSameSite>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Cookie/properties/priority。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie Priority</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "priority")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub priority: crate::network::types::CookiePriority,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Cookie/properties/sourceScheme。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie source scheme type.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "sourceScheme")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub source_scheme: crate::network::types::CookieSourceScheme,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Cookie/properties/sourcePort。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie source port. Valid values are {-1, [1, 65535]}, -1 indicates an unspecified port.
                    ///An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.
                    ///This is a temporary ability and it will be removed in the future.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "sourcePort")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub source_port: i64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Cookie/properties/partitionKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie partition key.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "partitionKey")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub partition_key: Option<crate::network::types::CookiePartitionKey>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Cookie/properties/partitionKeyOpaque。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if cookie partition key is opaque.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "partitionKeyOpaque")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub partition_key_opaque: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Cookie {
                    name: fields.name,
                    value: fields.value,
                    domain: fields.domain,
                    path: fields.path,
                    expires: fields.expires,
                    size: fields.size,
                    http_only: fields.http_only,
                    secure: fields.secure,
                    session: fields.session,
                    same_site: fields.same_site,
                    priority: fields.priority,
                    source_scheme: fields.source_scheme,
                    source_port: fields.source_port,
                    partition_key: fields.partition_key,
                    partition_key_opaque: fields.partition_key_opaque,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.CookieBlockedReason。
///
///协议原文：
///
///<pre>Types of reasons why a cookie may not be sent with a request.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CookieBlockedReason {
    ///协议值 "SecureOnly"。
    SecureOnly,
    ///协议值 "NotOnPath"。
    NotOnPath,
    ///协议值 "DomainMismatch"。
    DomainMismatch,
    ///协议值 "SameSiteStrict"。
    SameSiteStrict,
    ///协议值 "SameSiteLax"。
    SameSiteLax,
    ///协议值 "SameSiteUnspecifiedTreatedAsLax"。
    SameSiteUnspecifiedTreatedAsLax,
    ///协议值 "SameSiteNoneInsecure"。
    SameSiteNoneInsecure,
    ///协议值 "UserPreferences"。
    UserPreferences,
    ///协议值 "ThirdPartyPhaseout"。
    ThirdPartyPhaseout,
    ///协议值 "ThirdPartyBlockedInFirstPartySet"。
    ThirdPartyBlockedInFirstPartySet,
    ///协议值 "UnknownError"。
    UnknownError,
    ///协议值 "SchemefulSameSiteStrict"。
    SchemefulSameSiteStrict,
    ///协议值 "SchemefulSameSiteLax"。
    SchemefulSameSiteLax,
    ///协议值 "SchemefulSameSiteUnspecifiedTreatedAsLax"。
    SchemefulSameSiteUnspecifiedTreatedAsLax,
    ///协议值 "NameValuePairExceedsMaxSize"。
    NameValuePairExceedsMaxSize,
    ///协议值 "PortMismatch"。
    PortMismatch,
    ///协议值 "SchemeMismatch"。
    SchemeMismatch,
    ///协议值 "AnonymousContext"。
    AnonymousContext,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl CookieBlockedReason {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::CookieBlockedReason::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::SecureOnly => "SecureOnly",
            Self::NotOnPath => "NotOnPath",
            Self::DomainMismatch => "DomainMismatch",
            Self::SameSiteStrict => "SameSiteStrict",
            Self::SameSiteLax => "SameSiteLax",
            Self::SameSiteUnspecifiedTreatedAsLax => "SameSiteUnspecifiedTreatedAsLax",
            Self::SameSiteNoneInsecure => "SameSiteNoneInsecure",
            Self::UserPreferences => "UserPreferences",
            Self::ThirdPartyPhaseout => "ThirdPartyPhaseout",
            Self::ThirdPartyBlockedInFirstPartySet => "ThirdPartyBlockedInFirstPartySet",
            Self::UnknownError => "UnknownError",
            Self::SchemefulSameSiteStrict => "SchemefulSameSiteStrict",
            Self::SchemefulSameSiteLax => "SchemefulSameSiteLax",
            Self::SchemefulSameSiteUnspecifiedTreatedAsLax => {
                "SchemefulSameSiteUnspecifiedTreatedAsLax"
            }
            Self::NameValuePairExceedsMaxSize => "NameValuePairExceedsMaxSize",
            Self::PortMismatch => "PortMismatch",
            Self::SchemeMismatch => "SchemeMismatch",
            Self::AnonymousContext => "AnonymousContext",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for CookieBlockedReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for CookieBlockedReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "SecureOnly" => CookieBlockedReason::SecureOnly,
            "NotOnPath" => CookieBlockedReason::NotOnPath,
            "DomainMismatch" => CookieBlockedReason::DomainMismatch,
            "SameSiteStrict" => CookieBlockedReason::SameSiteStrict,
            "SameSiteLax" => CookieBlockedReason::SameSiteLax,
            "SameSiteUnspecifiedTreatedAsLax" => {
                CookieBlockedReason::SameSiteUnspecifiedTreatedAsLax
            }
            "SameSiteNoneInsecure" => CookieBlockedReason::SameSiteNoneInsecure,
            "UserPreferences" => CookieBlockedReason::UserPreferences,
            "ThirdPartyPhaseout" => CookieBlockedReason::ThirdPartyPhaseout,
            "ThirdPartyBlockedInFirstPartySet" => {
                CookieBlockedReason::ThirdPartyBlockedInFirstPartySet
            }
            "UnknownError" => CookieBlockedReason::UnknownError,
            "SchemefulSameSiteStrict" => CookieBlockedReason::SchemefulSameSiteStrict,
            "SchemefulSameSiteLax" => CookieBlockedReason::SchemefulSameSiteLax,
            "SchemefulSameSiteUnspecifiedTreatedAsLax" => {
                CookieBlockedReason::SchemefulSameSiteUnspecifiedTreatedAsLax
            }
            "NameValuePairExceedsMaxSize" => CookieBlockedReason::NameValuePairExceedsMaxSize,
            "PortMismatch" => CookieBlockedReason::PortMismatch,
            "SchemeMismatch" => CookieBlockedReason::SchemeMismatch,
            "AnonymousContext" => CookieBlockedReason::AnonymousContext,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for CookieBlockedReason {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CookieBlockedReason {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CookieBlockedReason;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "SecureOnly" => CookieBlockedReason::SecureOnly,
                    "NotOnPath" => CookieBlockedReason::NotOnPath,
                    "DomainMismatch" => CookieBlockedReason::DomainMismatch,
                    "SameSiteStrict" => CookieBlockedReason::SameSiteStrict,
                    "SameSiteLax" => CookieBlockedReason::SameSiteLax,
                    "SameSiteUnspecifiedTreatedAsLax" => {
                        CookieBlockedReason::SameSiteUnspecifiedTreatedAsLax
                    }
                    "SameSiteNoneInsecure" => CookieBlockedReason::SameSiteNoneInsecure,
                    "UserPreferences" => CookieBlockedReason::UserPreferences,
                    "ThirdPartyPhaseout" => CookieBlockedReason::ThirdPartyPhaseout,
                    "ThirdPartyBlockedInFirstPartySet" => {
                        CookieBlockedReason::ThirdPartyBlockedInFirstPartySet
                    }
                    "UnknownError" => CookieBlockedReason::UnknownError,
                    "SchemefulSameSiteStrict" => CookieBlockedReason::SchemefulSameSiteStrict,
                    "SchemefulSameSiteLax" => CookieBlockedReason::SchemefulSameSiteLax,
                    "SchemefulSameSiteUnspecifiedTreatedAsLax" => {
                        CookieBlockedReason::SchemefulSameSiteUnspecifiedTreatedAsLax
                    }
                    "NameValuePairExceedsMaxSize" => {
                        CookieBlockedReason::NameValuePairExceedsMaxSize
                    }
                    "PortMismatch" => CookieBlockedReason::PortMismatch,
                    "SchemeMismatch" => CookieBlockedReason::SchemeMismatch,
                    "AnonymousContext" => CookieBlockedReason::AnonymousContext,
                    _ => CookieBlockedReason::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(CookieBlockedReason::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.CookieExemptionReason。
///
///协议原文：
///
///<pre>Types of reasons why a cookie should have been blocked by 3PCD but is exempted for the request.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CookieExemptionReason {
    ///协议值 "None"。
    None,
    ///协议值 "UserSetting"。
    UserSetting,
    ///协议值 "EnterprisePolicy"。
    EnterprisePolicy,
    ///协议值 "StorageAccess"。
    StorageAccess,
    ///协议值 "TopLevelStorageAccess"。
    TopLevelStorageAccess,
    ///协议值 "Scheme"。
    Scheme,
    ///协议值 "SameSiteNoneCookiesInSandbox"。
    SameSiteNoneCookiesInSandbox,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl CookieExemptionReason {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::CookieExemptionReason::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::None => "None",
            Self::UserSetting => "UserSetting",
            Self::EnterprisePolicy => "EnterprisePolicy",
            Self::StorageAccess => "StorageAccess",
            Self::TopLevelStorageAccess => "TopLevelStorageAccess",
            Self::Scheme => "Scheme",
            Self::SameSiteNoneCookiesInSandbox => "SameSiteNoneCookiesInSandbox",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for CookieExemptionReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for CookieExemptionReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "None" => CookieExemptionReason::None,
            "UserSetting" => CookieExemptionReason::UserSetting,
            "EnterprisePolicy" => CookieExemptionReason::EnterprisePolicy,
            "StorageAccess" => CookieExemptionReason::StorageAccess,
            "TopLevelStorageAccess" => CookieExemptionReason::TopLevelStorageAccess,
            "Scheme" => CookieExemptionReason::Scheme,
            "SameSiteNoneCookiesInSandbox" => CookieExemptionReason::SameSiteNoneCookiesInSandbox,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for CookieExemptionReason {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CookieExemptionReason {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CookieExemptionReason;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "None" => CookieExemptionReason::None,
                    "UserSetting" => CookieExemptionReason::UserSetting,
                    "EnterprisePolicy" => CookieExemptionReason::EnterprisePolicy,
                    "StorageAccess" => CookieExemptionReason::StorageAccess,
                    "TopLevelStorageAccess" => CookieExemptionReason::TopLevelStorageAccess,
                    "Scheme" => CookieExemptionReason::Scheme,
                    "SameSiteNoneCookiesInSandbox" => {
                        CookieExemptionReason::SameSiteNoneCookiesInSandbox
                    }
                    _ => CookieExemptionReason::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(CookieExemptionReason::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.CookieParam。
///
///协议原文：
///
///<pre>Cookie parameter object</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct CookieParam {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.CookieParam/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie name.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.CookieParam/properties/value。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie value.</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CookieParam/properties/url。
    ///
    ///协议原文：
    ///
    ///<pre>The request-URI to associate with the setting of the cookie. This value can affect the
    ///default domain, path, source port, and source scheme values of the created cookie.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CookieParam/properties/domain。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie domain.</pre>
    ///
    ///
    #[serde(rename = "domain")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub domain: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CookieParam/properties/path。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie path.</pre>
    ///
    ///
    #[serde(rename = "path")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub path: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CookieParam/properties/secure。
    ///
    ///协议原文：
    ///
    ///<pre>True if cookie is secure.</pre>
    ///
    ///
    #[serde(rename = "secure")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub secure: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CookieParam/properties/httpOnly。
    ///
    ///协议原文：
    ///
    ///<pre>True if cookie is http-only.</pre>
    ///
    ///
    #[serde(rename = "httpOnly")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub http_only: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CookieParam/properties/sameSite。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie SameSite type.</pre>
    ///
    ///
    #[serde(rename = "sameSite")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub same_site: Option<crate::network::types::CookieSameSite>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CookieParam/properties/expires。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie expiration date, session cookie if not set</pre>
    ///
    ///
    #[serde(rename = "expires")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub expires: Option<crate::network::types::TimeSinceEpoch>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CookieParam/properties/priority。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie Priority.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "priority")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub priority: Option<crate::network::types::CookiePriority>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CookieParam/properties/sourceScheme。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie source scheme type.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "sourceScheme")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_scheme: Option<crate::network::types::CookieSourceScheme>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CookieParam/properties/sourcePort。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie source port. Valid values are {-1, [1, 65535]}, -1 indicates an unspecified port.
    ///An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.
    ///This is a temporary ability and it will be removed in the future.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "sourcePort")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_port: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CookieParam/properties/partitionKey。
    ///
    ///协议原文：
    ///
    ///<pre>Cookie partition key. If not set, the cookie will be set as not partitioned.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "partitionKey")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub partition_key: Option<crate::network::types::CookiePartitionKey>,
}
impl<'de> serde::Deserialize<'de> for CookieParam {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CookieParam;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.CookieParam/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie name.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.CookieParam/properties/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie value.</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CookieParam/properties/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The request-URI to associate with the setting of the cookie. This value can affect the
                    ///default domain, path, source port, and source scheme values of the created cookie.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub url: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CookieParam/properties/domain。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie domain.</pre>
                    ///
                    ///
                    #[serde(rename = "domain")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub domain: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CookieParam/properties/path。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie path.</pre>
                    ///
                    ///
                    #[serde(rename = "path")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub path: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CookieParam/properties/secure。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if cookie is secure.</pre>
                    ///
                    ///
                    #[serde(rename = "secure")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub secure: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CookieParam/properties/httpOnly。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if cookie is http-only.</pre>
                    ///
                    ///
                    #[serde(rename = "httpOnly")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub http_only: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CookieParam/properties/sameSite。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie SameSite type.</pre>
                    ///
                    ///
                    #[serde(rename = "sameSite")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub same_site: Option<crate::network::types::CookieSameSite>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CookieParam/properties/expires。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie expiration date, session cookie if not set</pre>
                    ///
                    ///
                    #[serde(rename = "expires")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub expires: Option<crate::network::types::TimeSinceEpoch>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CookieParam/properties/priority。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie Priority.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "priority")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub priority: Option<crate::network::types::CookiePriority>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CookieParam/properties/sourceScheme。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie source scheme type.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "sourceScheme")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub source_scheme: Option<crate::network::types::CookieSourceScheme>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CookieParam/properties/sourcePort。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie source port. Valid values are {-1, [1, 65535]}, -1 indicates an unspecified port.
                    ///An unspecified port value allows protocol clients to emulate legacy cookie scope for the port.
                    ///This is a temporary ability and it will be removed in the future.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "sourcePort")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub source_port: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CookieParam/properties/partitionKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cookie partition key. If not set, the cookie will be set as not partitioned.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "partitionKey")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub partition_key: Option<crate::network::types::CookiePartitionKey>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CookieParam {
                    name: fields.name,
                    value: fields.value,
                    url: fields.url,
                    domain: fields.domain,
                    path: fields.path,
                    secure: fields.secure,
                    http_only: fields.http_only,
                    same_site: fields.same_site,
                    expires: fields.expires,
                    priority: fields.priority,
                    source_scheme: fields.source_scheme,
                    source_port: fields.source_port,
                    partition_key: fields.partition_key,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.CookiePartitionKey。
///
///协议原文：
///
///<pre>cookiePartitionKey object
///The representation of the components of the key that are created by the cookiePartitionKey class contained in net/cookies/cookie_partition_key.h.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CookiePartitionKey {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.CookiePartitionKey/properties/topLevelSite。
    ///
    ///协议原文：
    ///
    ///<pre>The site of the top-level URL the browser was visiting at the start
    ///of the request to the endpoint that set the cookie.</pre>
    ///
    ///
    #[serde(rename = "topLevelSite")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub top_level_site: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.CookiePartitionKey/properties/hasCrossSiteAncestor。
    ///
    ///协议原文：
    ///
    ///<pre>Indicates if the cookie has any ancestors that are cross-site to the topLevelSite.</pre>
    ///
    ///
    #[serde(rename = "hasCrossSiteAncestor")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub has_cross_site_ancestor: bool,
}
impl<'de> serde::Deserialize<'de> for CookiePartitionKey {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CookiePartitionKey;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.CookiePartitionKey/properties/topLevelSite。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The site of the top-level URL the browser was visiting at the start
                    ///of the request to the endpoint that set the cookie.</pre>
                    ///
                    ///
                    #[serde(rename = "topLevelSite")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub top_level_site: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.CookiePartitionKey/properties/hasCrossSiteAncestor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Indicates if the cookie has any ancestors that are cross-site to the topLevelSite.</pre>
                    ///
                    ///
                    #[serde(rename = "hasCrossSiteAncestor")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub has_cross_site_ancestor: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CookiePartitionKey {
                    top_level_site: fields.top_level_site,
                    has_cross_site_ancestor: fields.has_cross_site_ancestor,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.CookiePriority。
///
///协议原文：
///
///<pre>Represents the cookie's 'Priority' status:
///https://tools.ietf.org/html/draft-west-cookie-priority-00</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CookiePriority {
    ///协议值 "Low"。
    Low,
    ///协议值 "Medium"。
    Medium,
    ///协议值 "High"。
    High,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl CookiePriority {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::CookiePriority::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for CookiePriority {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for CookiePriority {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Low" => CookiePriority::Low,
            "Medium" => CookiePriority::Medium,
            "High" => CookiePriority::High,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for CookiePriority {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CookiePriority {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CookiePriority;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Low" => CookiePriority::Low,
                    "Medium" => CookiePriority::Medium,
                    "High" => CookiePriority::High,
                    _ => CookiePriority::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(CookiePriority::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.CookieSameSite。
///
///协议原文：
///
///<pre>Represents the cookie's 'SameSite' status:
///https://tools.ietf.org/html/draft-west-first-party-cookies</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CookieSameSite {
    ///协议值 "Strict"。
    Strict,
    ///协议值 "Lax"。
    Lax,
    ///协议值 "None"。
    None,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl CookieSameSite {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::CookieSameSite::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Strict => "Strict",
            Self::Lax => "Lax",
            Self::None => "None",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for CookieSameSite {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for CookieSameSite {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Strict" => CookieSameSite::Strict,
            "Lax" => CookieSameSite::Lax,
            "None" => CookieSameSite::None,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for CookieSameSite {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CookieSameSite {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CookieSameSite;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Strict" => CookieSameSite::Strict,
                    "Lax" => CookieSameSite::Lax,
                    "None" => CookieSameSite::None,
                    _ => CookieSameSite::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(CookieSameSite::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.CookieSourceScheme。
///
///协议原文：
///
///<pre>Represents the source scheme of the origin that originally set the cookie.
///A value of "Unset" allows protocol clients to emulate legacy cookie scope for the scheme.
///This is a temporary ability and it will be removed in the future.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CookieSourceScheme {
    ///协议值 "Unset"。
    Unset,
    ///协议值 "NonSecure"。
    NonSecure,
    ///协议值 "Secure"。
    Secure,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl CookieSourceScheme {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::CookieSourceScheme::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Unset => "Unset",
            Self::NonSecure => "NonSecure",
            Self::Secure => "Secure",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for CookieSourceScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for CookieSourceScheme {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Unset" => CookieSourceScheme::Unset,
            "NonSecure" => CookieSourceScheme::NonSecure,
            "Secure" => CookieSourceScheme::Secure,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for CookieSourceScheme {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CookieSourceScheme {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CookieSourceScheme;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Unset" => CookieSourceScheme::Unset,
                    "NonSecure" => CookieSourceScheme::NonSecure,
                    "Secure" => CookieSourceScheme::Secure,
                    _ => CookieSourceScheme::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(CookieSourceScheme::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.CorsError。
///
///协议原文：
///
///<pre>The reason why request was blocked.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CorsError {
    ///协议值 "DisallowedByMode"。
    DisallowedByMode,
    ///协议值 "InvalidResponse"。
    InvalidResponse,
    ///协议值 "WildcardOriginNotAllowed"。
    WildcardOriginNotAllowed,
    ///协议值 "MissingAllowOriginHeader"。
    MissingAllowOriginHeader,
    ///协议值 "MultipleAllowOriginValues"。
    MultipleAllowOriginValues,
    ///协议值 "InvalidAllowOriginValue"。
    InvalidAllowOriginValue,
    ///协议值 "AllowOriginMismatch"。
    AllowOriginMismatch,
    ///协议值 "InvalidAllowCredentials"。
    InvalidAllowCredentials,
    ///协议值 "CorsDisabledScheme"。
    CorsDisabledScheme,
    ///协议值 "PreflightInvalidStatus"。
    PreflightInvalidStatus,
    ///协议值 "PreflightDisallowedRedirect"。
    PreflightDisallowedRedirect,
    ///协议值 "PreflightWildcardOriginNotAllowed"。
    PreflightWildcardOriginNotAllowed,
    ///协议值 "PreflightMissingAllowOriginHeader"。
    PreflightMissingAllowOriginHeader,
    ///协议值 "PreflightMultipleAllowOriginValues"。
    PreflightMultipleAllowOriginValues,
    ///协议值 "PreflightInvalidAllowOriginValue"。
    PreflightInvalidAllowOriginValue,
    ///协议值 "PreflightAllowOriginMismatch"。
    PreflightAllowOriginMismatch,
    ///协议值 "PreflightInvalidAllowCredentials"。
    PreflightInvalidAllowCredentials,
    ///协议值 "PreflightMissingAllowExternal"。
    PreflightMissingAllowExternal,
    ///协议值 "PreflightInvalidAllowExternal"。
    PreflightInvalidAllowExternal,
    ///协议值 "InvalidAllowMethodsPreflightResponse"。
    InvalidAllowMethodsPreflightResponse,
    ///协议值 "InvalidAllowHeadersPreflightResponse"。
    InvalidAllowHeadersPreflightResponse,
    ///协议值 "MethodDisallowedByPreflightResponse"。
    MethodDisallowedByPreflightResponse,
    ///协议值 "HeaderDisallowedByPreflightResponse"。
    HeaderDisallowedByPreflightResponse,
    ///协议值 "RedirectContainsCredentials"。
    RedirectContainsCredentials,
    ///协议值 "InsecureLocalNetwork"。
    InsecureLocalNetwork,
    ///协议值 "InvalidLocalNetworkAccess"。
    InvalidLocalNetworkAccess,
    ///协议值 "NoCorsRedirectModeNotFollow"。
    NoCorsRedirectModeNotFollow,
    ///协议值 "LocalNetworkAccessPermissionDenied"。
    LocalNetworkAccessPermissionDenied,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl CorsError {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::CorsError::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::DisallowedByMode => "DisallowedByMode",
            Self::InvalidResponse => "InvalidResponse",
            Self::WildcardOriginNotAllowed => "WildcardOriginNotAllowed",
            Self::MissingAllowOriginHeader => "MissingAllowOriginHeader",
            Self::MultipleAllowOriginValues => "MultipleAllowOriginValues",
            Self::InvalidAllowOriginValue => "InvalidAllowOriginValue",
            Self::AllowOriginMismatch => "AllowOriginMismatch",
            Self::InvalidAllowCredentials => "InvalidAllowCredentials",
            Self::CorsDisabledScheme => "CorsDisabledScheme",
            Self::PreflightInvalidStatus => "PreflightInvalidStatus",
            Self::PreflightDisallowedRedirect => "PreflightDisallowedRedirect",
            Self::PreflightWildcardOriginNotAllowed => "PreflightWildcardOriginNotAllowed",
            Self::PreflightMissingAllowOriginHeader => "PreflightMissingAllowOriginHeader",
            Self::PreflightMultipleAllowOriginValues => "PreflightMultipleAllowOriginValues",
            Self::PreflightInvalidAllowOriginValue => "PreflightInvalidAllowOriginValue",
            Self::PreflightAllowOriginMismatch => "PreflightAllowOriginMismatch",
            Self::PreflightInvalidAllowCredentials => "PreflightInvalidAllowCredentials",
            Self::PreflightMissingAllowExternal => "PreflightMissingAllowExternal",
            Self::PreflightInvalidAllowExternal => "PreflightInvalidAllowExternal",
            Self::InvalidAllowMethodsPreflightResponse => "InvalidAllowMethodsPreflightResponse",
            Self::InvalidAllowHeadersPreflightResponse => "InvalidAllowHeadersPreflightResponse",
            Self::MethodDisallowedByPreflightResponse => "MethodDisallowedByPreflightResponse",
            Self::HeaderDisallowedByPreflightResponse => "HeaderDisallowedByPreflightResponse",
            Self::RedirectContainsCredentials => "RedirectContainsCredentials",
            Self::InsecureLocalNetwork => "InsecureLocalNetwork",
            Self::InvalidLocalNetworkAccess => "InvalidLocalNetworkAccess",
            Self::NoCorsRedirectModeNotFollow => "NoCorsRedirectModeNotFollow",
            Self::LocalNetworkAccessPermissionDenied => "LocalNetworkAccessPermissionDenied",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for CorsError {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for CorsError {
    fn from(value: String) -> Self {
        match value.as_str() {
            "DisallowedByMode" => CorsError::DisallowedByMode,
            "InvalidResponse" => CorsError::InvalidResponse,
            "WildcardOriginNotAllowed" => CorsError::WildcardOriginNotAllowed,
            "MissingAllowOriginHeader" => CorsError::MissingAllowOriginHeader,
            "MultipleAllowOriginValues" => CorsError::MultipleAllowOriginValues,
            "InvalidAllowOriginValue" => CorsError::InvalidAllowOriginValue,
            "AllowOriginMismatch" => CorsError::AllowOriginMismatch,
            "InvalidAllowCredentials" => CorsError::InvalidAllowCredentials,
            "CorsDisabledScheme" => CorsError::CorsDisabledScheme,
            "PreflightInvalidStatus" => CorsError::PreflightInvalidStatus,
            "PreflightDisallowedRedirect" => CorsError::PreflightDisallowedRedirect,
            "PreflightWildcardOriginNotAllowed" => CorsError::PreflightWildcardOriginNotAllowed,
            "PreflightMissingAllowOriginHeader" => CorsError::PreflightMissingAllowOriginHeader,
            "PreflightMultipleAllowOriginValues" => CorsError::PreflightMultipleAllowOriginValues,
            "PreflightInvalidAllowOriginValue" => CorsError::PreflightInvalidAllowOriginValue,
            "PreflightAllowOriginMismatch" => CorsError::PreflightAllowOriginMismatch,
            "PreflightInvalidAllowCredentials" => CorsError::PreflightInvalidAllowCredentials,
            "PreflightMissingAllowExternal" => CorsError::PreflightMissingAllowExternal,
            "PreflightInvalidAllowExternal" => CorsError::PreflightInvalidAllowExternal,
            "InvalidAllowMethodsPreflightResponse" => {
                CorsError::InvalidAllowMethodsPreflightResponse
            }
            "InvalidAllowHeadersPreflightResponse" => {
                CorsError::InvalidAllowHeadersPreflightResponse
            }
            "MethodDisallowedByPreflightResponse" => CorsError::MethodDisallowedByPreflightResponse,
            "HeaderDisallowedByPreflightResponse" => CorsError::HeaderDisallowedByPreflightResponse,
            "RedirectContainsCredentials" => CorsError::RedirectContainsCredentials,
            "InsecureLocalNetwork" => CorsError::InsecureLocalNetwork,
            "InvalidLocalNetworkAccess" => CorsError::InvalidLocalNetworkAccess,
            "NoCorsRedirectModeNotFollow" => CorsError::NoCorsRedirectModeNotFollow,
            "LocalNetworkAccessPermissionDenied" => CorsError::LocalNetworkAccessPermissionDenied,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for CorsError {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CorsError {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CorsError;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "DisallowedByMode" => CorsError::DisallowedByMode,
                    "InvalidResponse" => CorsError::InvalidResponse,
                    "WildcardOriginNotAllowed" => CorsError::WildcardOriginNotAllowed,
                    "MissingAllowOriginHeader" => CorsError::MissingAllowOriginHeader,
                    "MultipleAllowOriginValues" => CorsError::MultipleAllowOriginValues,
                    "InvalidAllowOriginValue" => CorsError::InvalidAllowOriginValue,
                    "AllowOriginMismatch" => CorsError::AllowOriginMismatch,
                    "InvalidAllowCredentials" => CorsError::InvalidAllowCredentials,
                    "CorsDisabledScheme" => CorsError::CorsDisabledScheme,
                    "PreflightInvalidStatus" => CorsError::PreflightInvalidStatus,
                    "PreflightDisallowedRedirect" => CorsError::PreflightDisallowedRedirect,
                    "PreflightWildcardOriginNotAllowed" => {
                        CorsError::PreflightWildcardOriginNotAllowed
                    }
                    "PreflightMissingAllowOriginHeader" => {
                        CorsError::PreflightMissingAllowOriginHeader
                    }
                    "PreflightMultipleAllowOriginValues" => {
                        CorsError::PreflightMultipleAllowOriginValues
                    }
                    "PreflightInvalidAllowOriginValue" => {
                        CorsError::PreflightInvalidAllowOriginValue
                    }
                    "PreflightAllowOriginMismatch" => CorsError::PreflightAllowOriginMismatch,
                    "PreflightInvalidAllowCredentials" => {
                        CorsError::PreflightInvalidAllowCredentials
                    }
                    "PreflightMissingAllowExternal" => CorsError::PreflightMissingAllowExternal,
                    "PreflightInvalidAllowExternal" => CorsError::PreflightInvalidAllowExternal,
                    "InvalidAllowMethodsPreflightResponse" => {
                        CorsError::InvalidAllowMethodsPreflightResponse
                    }
                    "InvalidAllowHeadersPreflightResponse" => {
                        CorsError::InvalidAllowHeadersPreflightResponse
                    }
                    "MethodDisallowedByPreflightResponse" => {
                        CorsError::MethodDisallowedByPreflightResponse
                    }
                    "HeaderDisallowedByPreflightResponse" => {
                        CorsError::HeaderDisallowedByPreflightResponse
                    }
                    "RedirectContainsCredentials" => CorsError::RedirectContainsCredentials,
                    "InsecureLocalNetwork" => CorsError::InsecureLocalNetwork,
                    "InvalidLocalNetworkAccess" => CorsError::InvalidLocalNetworkAccess,
                    "NoCorsRedirectModeNotFollow" => CorsError::NoCorsRedirectModeNotFollow,
                    "LocalNetworkAccessPermissionDenied" => {
                        CorsError::LocalNetworkAccessPermissionDenied
                    }
                    _ => CorsError::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(CorsError::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.CorsErrorStatus。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CorsErrorStatus {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.CorsErrorStatus/properties/corsError。
    #[serde(rename = "corsError")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cors_error: crate::network::types::CorsError,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.CorsErrorStatus/properties/failedParameter。
    #[serde(rename = "failedParameter")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub failed_parameter: String,
}
impl<'de> serde::Deserialize<'de> for CorsErrorStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CorsErrorStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.CorsErrorStatus/properties/corsError。
                    #[serde(rename = "corsError")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cors_error: crate::network::types::CorsError,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.CorsErrorStatus/properties/failedParameter。
                    #[serde(rename = "failedParameter")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub failed_parameter: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CorsErrorStatus {
                    cors_error: fields.cors_error,
                    failed_parameter: fields.failed_parameter,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.CreationEventDetails。
///
///协议原文：
///
///<pre>Session event details specific to creation.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CreationEventDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.CreationEventDetails/properties/fetchResult。
    ///
    ///协议原文：
    ///
    ///<pre>The result of the fetch attempt.</pre>
    ///
    ///
    #[serde(rename = "fetchResult")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub fetch_result: crate::network::types::DeviceBoundSessionFetchResult,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CreationEventDetails/properties/newSession。
    ///
    ///协议原文：
    ///
    ///<pre>The session if there was a newly created session. This is populated for
    ///all successful creation events.</pre>
    ///
    ///
    #[serde(rename = "newSession")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub new_session: Option<crate::network::types::DeviceBoundSession>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CreationEventDetails/properties/failedRequest。
    ///
    ///协议原文：
    ///
    ///<pre>Details about a failed device bound session network request if there was
    ///one.</pre>
    ///
    ///
    #[serde(rename = "failedRequest")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub failed_request: Option<crate::network::types::DeviceBoundSessionFailedRequest>,
}
impl<'de> serde::Deserialize<'de> for CreationEventDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CreationEventDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.CreationEventDetails/properties/fetchResult。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The result of the fetch attempt.</pre>
                    ///
                    ///
                    #[serde(rename = "fetchResult")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub fetch_result: crate::network::types::DeviceBoundSessionFetchResult,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CreationEventDetails/properties/newSession。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The session if there was a newly created session. This is populated for
                    ///all successful creation events.</pre>
                    ///
                    ///
                    #[serde(rename = "newSession")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub new_session: Option<crate::network::types::DeviceBoundSession>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CreationEventDetails/properties/failedRequest。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Details about a failed device bound session network request if there was
                    ///one.</pre>
                    ///
                    ///
                    #[serde(rename = "failedRequest")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub failed_request:
                        Option<crate::network::types::DeviceBoundSessionFailedRequest>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CreationEventDetails {
                    fetch_result: fields.fetch_result,
                    new_session: fields.new_session,
                    failed_request: fields.failed_request,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.CrossOriginEmbedderPolicyStatus。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CrossOriginEmbedderPolicyStatus {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.CrossOriginEmbedderPolicyStatus/properties/value。
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: crate::network::types::CrossOriginEmbedderPolicyValue,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.CrossOriginEmbedderPolicyStatus/properties/reportOnlyValue。
    #[serde(rename = "reportOnlyValue")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub report_only_value: crate::network::types::CrossOriginEmbedderPolicyValue,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CrossOriginEmbedderPolicyStatus/properties/reportingEndpoint。
    #[serde(rename = "reportingEndpoint")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub reporting_endpoint: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CrossOriginEmbedderPolicyStatus/properties/reportOnlyReportingEndpoint。
    #[serde(rename = "reportOnlyReportingEndpoint")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub report_only_reporting_endpoint: Option<String>,
}
impl<'de> serde::Deserialize<'de> for CrossOriginEmbedderPolicyStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CrossOriginEmbedderPolicyStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.CrossOriginEmbedderPolicyStatus/properties/value。
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: crate::network::types::CrossOriginEmbedderPolicyValue,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.CrossOriginEmbedderPolicyStatus/properties/reportOnlyValue。
                    #[serde(rename = "reportOnlyValue")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub report_only_value: crate::network::types::CrossOriginEmbedderPolicyValue,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CrossOriginEmbedderPolicyStatus/properties/reportingEndpoint。
                    #[serde(rename = "reportingEndpoint")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub reporting_endpoint: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CrossOriginEmbedderPolicyStatus/properties/reportOnlyReportingEndpoint。
                    #[serde(rename = "reportOnlyReportingEndpoint")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub report_only_reporting_endpoint: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CrossOriginEmbedderPolicyStatus {
                    value: fields.value,
                    report_only_value: fields.report_only_value,
                    reporting_endpoint: fields.reporting_endpoint,
                    report_only_reporting_endpoint: fields.report_only_reporting_endpoint,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.CrossOriginEmbedderPolicyValue。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CrossOriginEmbedderPolicyValue {
    ///协议值 "None"。
    None,
    ///协议值 "Credentialless"。
    Credentialless,
    ///协议值 "RequireCorp"。
    RequireCorp,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl CrossOriginEmbedderPolicyValue {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::CrossOriginEmbedderPolicyValue::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::None => "None",
            Self::Credentialless => "Credentialless",
            Self::RequireCorp => "RequireCorp",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for CrossOriginEmbedderPolicyValue {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for CrossOriginEmbedderPolicyValue {
    fn from(value: String) -> Self {
        match value.as_str() {
            "None" => CrossOriginEmbedderPolicyValue::None,
            "Credentialless" => CrossOriginEmbedderPolicyValue::Credentialless,
            "RequireCorp" => CrossOriginEmbedderPolicyValue::RequireCorp,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for CrossOriginEmbedderPolicyValue {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CrossOriginEmbedderPolicyValue {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CrossOriginEmbedderPolicyValue;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "None" => CrossOriginEmbedderPolicyValue::None,
                    "Credentialless" => CrossOriginEmbedderPolicyValue::Credentialless,
                    "RequireCorp" => CrossOriginEmbedderPolicyValue::RequireCorp,
                    _ => CrossOriginEmbedderPolicyValue::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(CrossOriginEmbedderPolicyValue::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.CrossOriginOpenerPolicyStatus。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CrossOriginOpenerPolicyStatus {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.CrossOriginOpenerPolicyStatus/properties/value。
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: crate::network::types::CrossOriginOpenerPolicyValue,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.CrossOriginOpenerPolicyStatus/properties/reportOnlyValue。
    #[serde(rename = "reportOnlyValue")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub report_only_value: crate::network::types::CrossOriginOpenerPolicyValue,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CrossOriginOpenerPolicyStatus/properties/reportingEndpoint。
    #[serde(rename = "reportingEndpoint")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub reporting_endpoint: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CrossOriginOpenerPolicyStatus/properties/reportOnlyReportingEndpoint。
    #[serde(rename = "reportOnlyReportingEndpoint")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub report_only_reporting_endpoint: Option<String>,
}
impl<'de> serde::Deserialize<'de> for CrossOriginOpenerPolicyStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CrossOriginOpenerPolicyStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.CrossOriginOpenerPolicyStatus/properties/value。
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: crate::network::types::CrossOriginOpenerPolicyValue,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.CrossOriginOpenerPolicyStatus/properties/reportOnlyValue。
                    #[serde(rename = "reportOnlyValue")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub report_only_value: crate::network::types::CrossOriginOpenerPolicyValue,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CrossOriginOpenerPolicyStatus/properties/reportingEndpoint。
                    #[serde(rename = "reportingEndpoint")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub reporting_endpoint: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.CrossOriginOpenerPolicyStatus/properties/reportOnlyReportingEndpoint。
                    #[serde(rename = "reportOnlyReportingEndpoint")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub report_only_reporting_endpoint: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CrossOriginOpenerPolicyStatus {
                    value: fields.value,
                    report_only_value: fields.report_only_value,
                    reporting_endpoint: fields.reporting_endpoint,
                    report_only_reporting_endpoint: fields.report_only_reporting_endpoint,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.CrossOriginOpenerPolicyValue。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CrossOriginOpenerPolicyValue {
    ///协议值 "SameOrigin"。
    SameOrigin,
    ///协议值 "SameOriginAllowPopups"。
    SameOriginAllowPopups,
    ///协议值 "RestrictProperties"。
    RestrictProperties,
    ///协议值 "UnsafeNone"。
    UnsafeNone,
    ///协议值 "SameOriginPlusCoep"。
    SameOriginPlusCoep,
    ///协议值 "RestrictPropertiesPlusCoep"。
    RestrictPropertiesPlusCoep,
    ///协议值 "NoopenerAllowPopups"。
    NoopenerAllowPopups,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl CrossOriginOpenerPolicyValue {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::CrossOriginOpenerPolicyValue::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::SameOrigin => "SameOrigin",
            Self::SameOriginAllowPopups => "SameOriginAllowPopups",
            Self::RestrictProperties => "RestrictProperties",
            Self::UnsafeNone => "UnsafeNone",
            Self::SameOriginPlusCoep => "SameOriginPlusCoep",
            Self::RestrictPropertiesPlusCoep => "RestrictPropertiesPlusCoep",
            Self::NoopenerAllowPopups => "NoopenerAllowPopups",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for CrossOriginOpenerPolicyValue {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for CrossOriginOpenerPolicyValue {
    fn from(value: String) -> Self {
        match value.as_str() {
            "SameOrigin" => CrossOriginOpenerPolicyValue::SameOrigin,
            "SameOriginAllowPopups" => CrossOriginOpenerPolicyValue::SameOriginAllowPopups,
            "RestrictProperties" => CrossOriginOpenerPolicyValue::RestrictProperties,
            "UnsafeNone" => CrossOriginOpenerPolicyValue::UnsafeNone,
            "SameOriginPlusCoep" => CrossOriginOpenerPolicyValue::SameOriginPlusCoep,
            "RestrictPropertiesPlusCoep" => {
                CrossOriginOpenerPolicyValue::RestrictPropertiesPlusCoep
            }
            "NoopenerAllowPopups" => CrossOriginOpenerPolicyValue::NoopenerAllowPopups,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for CrossOriginOpenerPolicyValue {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CrossOriginOpenerPolicyValue {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CrossOriginOpenerPolicyValue;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "SameOrigin" => CrossOriginOpenerPolicyValue::SameOrigin,
                    "SameOriginAllowPopups" => CrossOriginOpenerPolicyValue::SameOriginAllowPopups,
                    "RestrictProperties" => CrossOriginOpenerPolicyValue::RestrictProperties,
                    "UnsafeNone" => CrossOriginOpenerPolicyValue::UnsafeNone,
                    "SameOriginPlusCoep" => CrossOriginOpenerPolicyValue::SameOriginPlusCoep,
                    "RestrictPropertiesPlusCoep" => {
                        CrossOriginOpenerPolicyValue::RestrictPropertiesPlusCoep
                    }
                    "NoopenerAllowPopups" => CrossOriginOpenerPolicyValue::NoopenerAllowPopups,
                    _ => CrossOriginOpenerPolicyValue::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(CrossOriginOpenerPolicyValue::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.DeviceBoundSession。
///
///协议原文：
///
///<pre>A device bound session.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeviceBoundSession {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSession/properties/key。
    ///
    ///协议原文：
    ///
    ///<pre>The site and session ID of the session.</pre>
    ///
    ///
    #[serde(rename = "key")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub key: crate::network::types::DeviceBoundSessionKey,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSession/properties/refreshUrl。
    ///
    ///协议原文：
    ///
    ///<pre>See comments on `net::device_bound_sessions::Session::refresh_url_`.</pre>
    ///
    ///
    #[serde(rename = "refreshUrl")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub refresh_url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSession/properties/inclusionRules。
    ///
    ///协议原文：
    ///
    ///<pre>See comments on `net::device_bound_sessions::Session::inclusion_rules_`.</pre>
    ///
    ///
    #[serde(rename = "inclusionRules")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub inclusion_rules: crate::network::types::DeviceBoundSessionInclusionRules,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSession/properties/cookieCravings。
    ///
    ///协议原文：
    ///
    ///<pre>See comments on `net::device_bound_sessions::Session::cookie_cravings_`.</pre>
    ///
    ///
    #[serde(rename = "cookieCravings")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cookie_cravings: Vec<crate::network::types::DeviceBoundSessionCookieCraving>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSession/properties/expiryDate。
    ///
    ///协议原文：
    ///
    ///<pre>See comments on `net::device_bound_sessions::Session::expiry_date_`.</pre>
    ///
    ///
    #[serde(rename = "expiryDate")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub expiry_date: crate::network::types::TimeSinceEpoch,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DeviceBoundSession/properties/cachedChallenge。
    ///
    ///协议原文：
    ///
    ///<pre>See comments on `net::device_bound_sessions::Session::cached_challenge__`.</pre>
    ///
    ///
    #[serde(rename = "cachedChallenge")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub cached_challenge: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSession/properties/allowedRefreshInitiators。
    ///
    ///协议原文：
    ///
    ///<pre>See comments on `net::device_bound_sessions::Session::allowed_refresh_initiators_`.</pre>
    ///
    ///
    #[serde(rename = "allowedRefreshInitiators")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub allowed_refresh_initiators: Vec<String>,
}
impl<'de> serde::Deserialize<'de> for DeviceBoundSession {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceBoundSession;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSession/properties/key。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The site and session ID of the session.</pre>
                    ///
                    ///
                    #[serde(rename = "key")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub key: crate::network::types::DeviceBoundSessionKey,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSession/properties/refreshUrl。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>See comments on `net::device_bound_sessions::Session::refresh_url_`.</pre>
                    ///
                    ///
                    #[serde(rename = "refreshUrl")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub refresh_url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSession/properties/inclusionRules。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>See comments on `net::device_bound_sessions::Session::inclusion_rules_`.</pre>
                    ///
                    ///
                    #[serde(rename = "inclusionRules")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub inclusion_rules: crate::network::types::DeviceBoundSessionInclusionRules,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSession/properties/cookieCravings。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>See comments on `net::device_bound_sessions::Session::cookie_cravings_`.</pre>
                    ///
                    ///
                    #[serde(rename = "cookieCravings")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cookie_cravings:
                        Vec<crate::network::types::DeviceBoundSessionCookieCraving>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSession/properties/expiryDate。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>See comments on `net::device_bound_sessions::Session::expiry_date_`.</pre>
                    ///
                    ///
                    #[serde(rename = "expiryDate")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub expiry_date: crate::network::types::TimeSinceEpoch,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DeviceBoundSession/properties/cachedChallenge。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>See comments on `net::device_bound_sessions::Session::cached_challenge__`.</pre>
                    ///
                    ///
                    #[serde(rename = "cachedChallenge")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub cached_challenge: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSession/properties/allowedRefreshInitiators。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>See comments on `net::device_bound_sessions::Session::allowed_refresh_initiators_`.</pre>
                    ///
                    ///
                    #[serde(rename = "allowedRefreshInitiators")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub allowed_refresh_initiators: Vec<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DeviceBoundSession {
                    key: fields.key,
                    refresh_url: fields.refresh_url,
                    inclusion_rules: fields.inclusion_rules,
                    cookie_cravings: fields.cookie_cravings,
                    expiry_date: fields.expiry_date,
                    cached_challenge: fields.cached_challenge,
                    allowed_refresh_initiators: fields.allowed_refresh_initiators,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.DeviceBoundSessionCookieCraving。
///
///协议原文：
///
///<pre>A device bound session's cookie craving.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeviceBoundSessionCookieCraving {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionCookieCraving/properties/name。
    ///
    ///协议原文：
    ///
    ///<pre>The name of the craving.</pre>
    ///
    ///
    #[serde(rename = "name")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionCookieCraving/properties/domain。
    ///
    ///协议原文：
    ///
    ///<pre>The domain of the craving.</pre>
    ///
    ///
    #[serde(rename = "domain")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub domain: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionCookieCraving/properties/path。
    ///
    ///协议原文：
    ///
    ///<pre>The path of the craving.</pre>
    ///
    ///
    #[serde(rename = "path")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub path: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionCookieCraving/properties/secure。
    ///
    ///协议原文：
    ///
    ///<pre>The `Secure` attribute of the craving attributes.</pre>
    ///
    ///
    #[serde(rename = "secure")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub secure: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionCookieCraving/properties/httpOnly。
    ///
    ///协议原文：
    ///
    ///<pre>The `HttpOnly` attribute of the craving attributes.</pre>
    ///
    ///
    #[serde(rename = "httpOnly")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub http_only: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DeviceBoundSessionCookieCraving/properties/sameSite。
    ///
    ///协议原文：
    ///
    ///<pre>The `SameSite` attribute of the craving attributes.</pre>
    ///
    ///
    #[serde(rename = "sameSite")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub same_site: Option<crate::network::types::CookieSameSite>,
}
impl<'de> serde::Deserialize<'de> for DeviceBoundSessionCookieCraving {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceBoundSessionCookieCraving;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionCookieCraving/properties/name。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The name of the craving.</pre>
                    ///
                    ///
                    #[serde(rename = "name")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionCookieCraving/properties/domain。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The domain of the craving.</pre>
                    ///
                    ///
                    #[serde(rename = "domain")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub domain: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionCookieCraving/properties/path。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The path of the craving.</pre>
                    ///
                    ///
                    #[serde(rename = "path")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub path: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionCookieCraving/properties/secure。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The `Secure` attribute of the craving attributes.</pre>
                    ///
                    ///
                    #[serde(rename = "secure")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub secure: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionCookieCraving/properties/httpOnly。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The `HttpOnly` attribute of the craving attributes.</pre>
                    ///
                    ///
                    #[serde(rename = "httpOnly")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub http_only: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DeviceBoundSessionCookieCraving/properties/sameSite。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The `SameSite` attribute of the craving attributes.</pre>
                    ///
                    ///
                    #[serde(rename = "sameSite")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub same_site: Option<crate::network::types::CookieSameSite>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DeviceBoundSessionCookieCraving {
                    name: fields.name,
                    domain: fields.domain,
                    path: fields.path,
                    secure: fields.secure,
                    http_only: fields.http_only,
                    same_site: fields.same_site,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Network.type.DeviceBoundSessionEventId。
///
///协议原文：
///
///<pre>A unique identifier for a device bound session event.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct DeviceBoundSessionEventId(String);
impl DeviceBoundSessionEventId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::network::types::DeviceBoundSessionEventId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::network::types::DeviceBoundSessionEventId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::network::types::DeviceBoundSessionEventId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::network::types::DeviceBoundSessionEventId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::network::types::DeviceBoundSessionEventId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.DeviceBoundSessionFailedRequest。
///
///协议原文：
///
///<pre>Details about a failed device bound session network request.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeviceBoundSessionFailedRequest {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionFailedRequest/properties/requestUrl。
    ///
    ///协议原文：
    ///
    ///<pre>The failed request URL.</pre>
    ///
    ///
    #[serde(rename = "requestUrl")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_url: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DeviceBoundSessionFailedRequest/properties/netError。
    ///
    ///协议原文：
    ///
    ///<pre>The net error of the response if it was not OK.</pre>
    ///
    ///
    #[serde(rename = "netError")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub net_error: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DeviceBoundSessionFailedRequest/properties/responseError。
    ///
    ///协议原文：
    ///
    ///<pre>The response code if the net error was OK and the response code was not
    ///200.</pre>
    ///
    ///
    #[serde(rename = "responseError")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_error: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DeviceBoundSessionFailedRequest/properties/responseErrorBody。
    ///
    ///协议原文：
    ///
    ///<pre>The body of the response if the net error was OK, the response code was
    ///not 200, and the response body was not empty.</pre>
    ///
    ///
    #[serde(rename = "responseErrorBody")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_error_body: Option<String>,
}
impl<'de> serde::Deserialize<'de> for DeviceBoundSessionFailedRequest {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceBoundSessionFailedRequest;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionFailedRequest/properties/requestUrl。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The failed request URL.</pre>
                    ///
                    ///
                    #[serde(rename = "requestUrl")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_url: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DeviceBoundSessionFailedRequest/properties/netError。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The net error of the response if it was not OK.</pre>
                    ///
                    ///
                    #[serde(rename = "netError")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub net_error: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DeviceBoundSessionFailedRequest/properties/responseError。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The response code if the net error was OK and the response code was not
                    ///200.</pre>
                    ///
                    ///
                    #[serde(rename = "responseError")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub response_error: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DeviceBoundSessionFailedRequest/properties/responseErrorBody。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The body of the response if the net error was OK, the response code was
                    ///not 200, and the response body was not empty.</pre>
                    ///
                    ///
                    #[serde(rename = "responseErrorBody")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub response_error_body: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DeviceBoundSessionFailedRequest {
                    request_url: fields.request_url,
                    net_error: fields.net_error,
                    response_error: fields.response_error,
                    response_error_body: fields.response_error_body,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.DeviceBoundSessionFetchResult。
///
///协议原文：
///
///<pre>A fetch result for a device bound session creation or refresh.
///LINT.IfChange(DeviceBoundSessionFetchResult)</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DeviceBoundSessionFetchResult {
    ///协议值 "Success"。
    Success,
    ///协议值 "SigningKeyGenerationError"。
    SigningKeyGenerationError,
    ///协议值 "AttestationKeyGenerationError"。
    AttestationKeyGenerationError,
    ///协议值 "SigningError"。
    SigningError,
    ///协议值 "TransientSigningError"。
    TransientSigningError,
    ///协议值 "ServerRequestedTermination"。
    ServerRequestedTermination,
    ///协议值 "InvalidSessionId"。
    InvalidSessionId,
    ///协议值 "InvalidChallenge"。
    InvalidChallenge,
    ///协议值 "TooManyChallenges"。
    TooManyChallenges,
    ///协议值 "InvalidFetcherUrl"。
    InvalidFetcherUrl,
    ///协议值 "InvalidRefreshUrl"。
    InvalidRefreshUrl,
    ///协议值 "TransientHttpError"。
    TransientHttpError,
    ///协议值 "ScopeOriginSameSiteMismatch"。
    ScopeOriginSameSiteMismatch,
    ///协议值 "RefreshUrlSameSiteMismatch"。
    RefreshUrlSameSiteMismatch,
    ///协议值 "MismatchedSessionId"。
    MismatchedSessionId,
    ///协议值 "MissingScope"。
    MissingScope,
    ///协议值 "NoCredentials"。
    NoCredentials,
    ///协议值 "SubdomainRegistrationWellKnownUnavailable"。
    SubdomainRegistrationWellKnownUnavailable,
    ///协议值 "SubdomainRegistrationUnauthorized"。
    SubdomainRegistrationUnauthorized,
    ///协议值 "SubdomainRegistrationWellKnownMalformed"。
    SubdomainRegistrationWellKnownMalformed,
    ///协议值 "SessionProviderWellKnownUnavailable"。
    SessionProviderWellKnownUnavailable,
    ///协议值 "RelyingPartyWellKnownUnavailable"。
    RelyingPartyWellKnownUnavailable,
    ///协议值 "FederatedKeyThumbprintMismatch"。
    FederatedKeyThumbprintMismatch,
    ///协议值 "InvalidFederatedSessionUrl"。
    InvalidFederatedSessionUrl,
    ///协议值 "InvalidFederatedKey"。
    InvalidFederatedKey,
    ///协议值 "TooManyRelyingOriginLabels"。
    TooManyRelyingOriginLabels,
    ///协议值 "BoundCookieSetForbidden"。
    BoundCookieSetForbidden,
    ///协议值 "NetError"。
    NetError,
    ///协议值 "ProxyError"。
    ProxyError,
    ///协议值 "EmptySessionConfig"。
    EmptySessionConfig,
    ///协议值 "InvalidCredentialsConfig"。
    InvalidCredentialsConfig,
    ///协议值 "InvalidCredentialsType"。
    InvalidCredentialsType,
    ///协议值 "InvalidCredentialsEmptyName"。
    InvalidCredentialsEmptyName,
    ///协议值 "InvalidCredentialsCookie"。
    InvalidCredentialsCookie,
    ///协议值 "PersistentHttpError"。
    PersistentHttpError,
    ///协议值 "RegistrationAttemptedChallenge"。
    RegistrationAttemptedChallenge,
    ///协议值 "InvalidScopeOrigin"。
    InvalidScopeOrigin,
    ///协议值 "ScopeOriginContainsPath"。
    ScopeOriginContainsPath,
    ///协议值 "RefreshInitiatorNotString"。
    RefreshInitiatorNotString,
    ///协议值 "RefreshInitiatorInvalidHostPattern"。
    RefreshInitiatorInvalidHostPattern,
    ///协议值 "InvalidScopeSpecification"。
    InvalidScopeSpecification,
    ///协议值 "MissingScopeSpecificationType"。
    MissingScopeSpecificationType,
    ///协议值 "EmptyScopeSpecificationDomain"。
    EmptyScopeSpecificationDomain,
    ///协议值 "EmptyScopeSpecificationPath"。
    EmptyScopeSpecificationPath,
    ///协议值 "InvalidScopeSpecificationType"。
    InvalidScopeSpecificationType,
    ///协议值 "InvalidScopeIncludeSite"。
    InvalidScopeIncludeSite,
    ///协议值 "MissingScopeIncludeSite"。
    MissingScopeIncludeSite,
    ///协议值 "FederatedNotAuthorizedByProvider"。
    FederatedNotAuthorizedByProvider,
    ///协议值 "FederatedNotAuthorizedByRelyingParty"。
    FederatedNotAuthorizedByRelyingParty,
    ///协议值 "SessionProviderWellKnownMalformed"。
    SessionProviderWellKnownMalformed,
    ///协议值 "SessionProviderWellKnownHasProviderOrigin"。
    SessionProviderWellKnownHasProviderOrigin,
    ///协议值 "RelyingPartyWellKnownMalformed"。
    RelyingPartyWellKnownMalformed,
    ///协议值 "RelyingPartyWellKnownHasRelyingOrigins"。
    RelyingPartyWellKnownHasRelyingOrigins,
    ///协议值 "InvalidFederatedSessionProviderSessionMissing"。
    InvalidFederatedSessionProviderSessionMissing,
    ///协议值 "InvalidFederatedSessionWrongProviderOrigin"。
    InvalidFederatedSessionWrongProviderOrigin,
    ///协议值 "InvalidCredentialsCookieCreationTime"。
    InvalidCredentialsCookieCreationTime,
    ///协议值 "InvalidCredentialsCookieName"。
    InvalidCredentialsCookieName,
    ///协议值 "InvalidCredentialsCookieParsing"。
    InvalidCredentialsCookieParsing,
    ///协议值 "InvalidCredentialsCookieUnpermittedAttribute"。
    InvalidCredentialsCookieUnpermittedAttribute,
    ///协议值 "InvalidCredentialsCookieInvalidDomain"。
    InvalidCredentialsCookieInvalidDomain,
    ///协议值 "InvalidCredentialsCookiePrefix"。
    InvalidCredentialsCookiePrefix,
    ///协议值 "InvalidScopeRulePath"。
    InvalidScopeRulePath,
    ///协议值 "InvalidScopeRuleHostPattern"。
    InvalidScopeRuleHostPattern,
    ///协议值 "ScopeRuleOriginScopedHostPatternMismatch"。
    ScopeRuleOriginScopedHostPatternMismatch,
    ///协议值 "ScopeRuleSiteScopedHostPatternMismatch"。
    ScopeRuleSiteScopedHostPatternMismatch,
    ///协议值 "SigningQuotaExceeded"。
    SigningQuotaExceeded,
    ///协议值 "InvalidConfigJson"。
    InvalidConfigJson,
    ///协议值 "InvalidFederatedSessionProviderFailedToRestoreKey"。
    InvalidFederatedSessionProviderFailedToRestoreKey,
    ///协议值 "FailedToUnwrapKey"。
    FailedToUnwrapKey,
    ///协议值 "SessionDeletedDuringRefresh"。
    SessionDeletedDuringRefresh,
    ///协议值 "CrossOriginRegistrationSiteNotIncluded"。
    CrossOriginRegistrationSiteNotIncluded,
    ///协议值 "InvalidPreProvisionedKeyInitiatorMissing"。
    InvalidPreProvisionedKeyInitiatorMissing,
    ///协议值 "PreProvisionedKeyAccessNotGranted"。
    PreProvisionedKeyAccessNotGranted,
    ///协议值 "PreProvisionedKeyNotFound"。
    PreProvisionedKeyNotFound,
    ///协议值 "AttestationCertificationError"。
    AttestationCertificationError,
    ///协议值 "AttestationSigningError"。
    AttestationSigningError,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl DeviceBoundSessionFetchResult {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::DeviceBoundSessionFetchResult::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Success => "Success",
            Self::SigningKeyGenerationError => "SigningKeyGenerationError",
            Self::AttestationKeyGenerationError => "AttestationKeyGenerationError",
            Self::SigningError => "SigningError",
            Self::TransientSigningError => "TransientSigningError",
            Self::ServerRequestedTermination => "ServerRequestedTermination",
            Self::InvalidSessionId => "InvalidSessionId",
            Self::InvalidChallenge => "InvalidChallenge",
            Self::TooManyChallenges => "TooManyChallenges",
            Self::InvalidFetcherUrl => "InvalidFetcherUrl",
            Self::InvalidRefreshUrl => "InvalidRefreshUrl",
            Self::TransientHttpError => "TransientHttpError",
            Self::ScopeOriginSameSiteMismatch => "ScopeOriginSameSiteMismatch",
            Self::RefreshUrlSameSiteMismatch => "RefreshUrlSameSiteMismatch",
            Self::MismatchedSessionId => "MismatchedSessionId",
            Self::MissingScope => "MissingScope",
            Self::NoCredentials => "NoCredentials",
            Self::SubdomainRegistrationWellKnownUnavailable => {
                "SubdomainRegistrationWellKnownUnavailable"
            }
            Self::SubdomainRegistrationUnauthorized => "SubdomainRegistrationUnauthorized",
            Self::SubdomainRegistrationWellKnownMalformed => {
                "SubdomainRegistrationWellKnownMalformed"
            }
            Self::SessionProviderWellKnownUnavailable => "SessionProviderWellKnownUnavailable",
            Self::RelyingPartyWellKnownUnavailable => "RelyingPartyWellKnownUnavailable",
            Self::FederatedKeyThumbprintMismatch => "FederatedKeyThumbprintMismatch",
            Self::InvalidFederatedSessionUrl => "InvalidFederatedSessionUrl",
            Self::InvalidFederatedKey => "InvalidFederatedKey",
            Self::TooManyRelyingOriginLabels => "TooManyRelyingOriginLabels",
            Self::BoundCookieSetForbidden => "BoundCookieSetForbidden",
            Self::NetError => "NetError",
            Self::ProxyError => "ProxyError",
            Self::EmptySessionConfig => "EmptySessionConfig",
            Self::InvalidCredentialsConfig => "InvalidCredentialsConfig",
            Self::InvalidCredentialsType => "InvalidCredentialsType",
            Self::InvalidCredentialsEmptyName => "InvalidCredentialsEmptyName",
            Self::InvalidCredentialsCookie => "InvalidCredentialsCookie",
            Self::PersistentHttpError => "PersistentHttpError",
            Self::RegistrationAttemptedChallenge => "RegistrationAttemptedChallenge",
            Self::InvalidScopeOrigin => "InvalidScopeOrigin",
            Self::ScopeOriginContainsPath => "ScopeOriginContainsPath",
            Self::RefreshInitiatorNotString => "RefreshInitiatorNotString",
            Self::RefreshInitiatorInvalidHostPattern => "RefreshInitiatorInvalidHostPattern",
            Self::InvalidScopeSpecification => "InvalidScopeSpecification",
            Self::MissingScopeSpecificationType => "MissingScopeSpecificationType",
            Self::EmptyScopeSpecificationDomain => "EmptyScopeSpecificationDomain",
            Self::EmptyScopeSpecificationPath => "EmptyScopeSpecificationPath",
            Self::InvalidScopeSpecificationType => "InvalidScopeSpecificationType",
            Self::InvalidScopeIncludeSite => "InvalidScopeIncludeSite",
            Self::MissingScopeIncludeSite => "MissingScopeIncludeSite",
            Self::FederatedNotAuthorizedByProvider => "FederatedNotAuthorizedByProvider",
            Self::FederatedNotAuthorizedByRelyingParty => "FederatedNotAuthorizedByRelyingParty",
            Self::SessionProviderWellKnownMalformed => "SessionProviderWellKnownMalformed",
            Self::SessionProviderWellKnownHasProviderOrigin => {
                "SessionProviderWellKnownHasProviderOrigin"
            }
            Self::RelyingPartyWellKnownMalformed => "RelyingPartyWellKnownMalformed",
            Self::RelyingPartyWellKnownHasRelyingOrigins => {
                "RelyingPartyWellKnownHasRelyingOrigins"
            }
            Self::InvalidFederatedSessionProviderSessionMissing => {
                "InvalidFederatedSessionProviderSessionMissing"
            }
            Self::InvalidFederatedSessionWrongProviderOrigin => {
                "InvalidFederatedSessionWrongProviderOrigin"
            }
            Self::InvalidCredentialsCookieCreationTime => "InvalidCredentialsCookieCreationTime",
            Self::InvalidCredentialsCookieName => "InvalidCredentialsCookieName",
            Self::InvalidCredentialsCookieParsing => "InvalidCredentialsCookieParsing",
            Self::InvalidCredentialsCookieUnpermittedAttribute => {
                "InvalidCredentialsCookieUnpermittedAttribute"
            }
            Self::InvalidCredentialsCookieInvalidDomain => "InvalidCredentialsCookieInvalidDomain",
            Self::InvalidCredentialsCookiePrefix => "InvalidCredentialsCookiePrefix",
            Self::InvalidScopeRulePath => "InvalidScopeRulePath",
            Self::InvalidScopeRuleHostPattern => "InvalidScopeRuleHostPattern",
            Self::ScopeRuleOriginScopedHostPatternMismatch => {
                "ScopeRuleOriginScopedHostPatternMismatch"
            }
            Self::ScopeRuleSiteScopedHostPatternMismatch => {
                "ScopeRuleSiteScopedHostPatternMismatch"
            }
            Self::SigningQuotaExceeded => "SigningQuotaExceeded",
            Self::InvalidConfigJson => "InvalidConfigJson",
            Self::InvalidFederatedSessionProviderFailedToRestoreKey => {
                "InvalidFederatedSessionProviderFailedToRestoreKey"
            }
            Self::FailedToUnwrapKey => "FailedToUnwrapKey",
            Self::SessionDeletedDuringRefresh => "SessionDeletedDuringRefresh",
            Self::CrossOriginRegistrationSiteNotIncluded => {
                "CrossOriginRegistrationSiteNotIncluded"
            }
            Self::InvalidPreProvisionedKeyInitiatorMissing => {
                "InvalidPreProvisionedKeyInitiatorMissing"
            }
            Self::PreProvisionedKeyAccessNotGranted => "PreProvisionedKeyAccessNotGranted",
            Self::PreProvisionedKeyNotFound => "PreProvisionedKeyNotFound",
            Self::AttestationCertificationError => "AttestationCertificationError",
            Self::AttestationSigningError => "AttestationSigningError",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for DeviceBoundSessionFetchResult {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for DeviceBoundSessionFetchResult {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Success" => DeviceBoundSessionFetchResult::Success,
            "SigningKeyGenerationError" => DeviceBoundSessionFetchResult::SigningKeyGenerationError,
            "AttestationKeyGenerationError" => {
                DeviceBoundSessionFetchResult::AttestationKeyGenerationError
            }
            "SigningError" => DeviceBoundSessionFetchResult::SigningError,
            "TransientSigningError" => DeviceBoundSessionFetchResult::TransientSigningError,
            "ServerRequestedTermination" => {
                DeviceBoundSessionFetchResult::ServerRequestedTermination
            }
            "InvalidSessionId" => DeviceBoundSessionFetchResult::InvalidSessionId,
            "InvalidChallenge" => DeviceBoundSessionFetchResult::InvalidChallenge,
            "TooManyChallenges" => DeviceBoundSessionFetchResult::TooManyChallenges,
            "InvalidFetcherUrl" => DeviceBoundSessionFetchResult::InvalidFetcherUrl,
            "InvalidRefreshUrl" => DeviceBoundSessionFetchResult::InvalidRefreshUrl,
            "TransientHttpError" => DeviceBoundSessionFetchResult::TransientHttpError,
            "ScopeOriginSameSiteMismatch" => {
                DeviceBoundSessionFetchResult::ScopeOriginSameSiteMismatch
            }
            "RefreshUrlSameSiteMismatch" => {
                DeviceBoundSessionFetchResult::RefreshUrlSameSiteMismatch
            }
            "MismatchedSessionId" => DeviceBoundSessionFetchResult::MismatchedSessionId,
            "MissingScope" => DeviceBoundSessionFetchResult::MissingScope,
            "NoCredentials" => DeviceBoundSessionFetchResult::NoCredentials,
            "SubdomainRegistrationWellKnownUnavailable" => {
                DeviceBoundSessionFetchResult::SubdomainRegistrationWellKnownUnavailable
            }
            "SubdomainRegistrationUnauthorized" => {
                DeviceBoundSessionFetchResult::SubdomainRegistrationUnauthorized
            }
            "SubdomainRegistrationWellKnownMalformed" => {
                DeviceBoundSessionFetchResult::SubdomainRegistrationWellKnownMalformed
            }
            "SessionProviderWellKnownUnavailable" => {
                DeviceBoundSessionFetchResult::SessionProviderWellKnownUnavailable
            }
            "RelyingPartyWellKnownUnavailable" => {
                DeviceBoundSessionFetchResult::RelyingPartyWellKnownUnavailable
            }
            "FederatedKeyThumbprintMismatch" => {
                DeviceBoundSessionFetchResult::FederatedKeyThumbprintMismatch
            }
            "InvalidFederatedSessionUrl" => {
                DeviceBoundSessionFetchResult::InvalidFederatedSessionUrl
            }
            "InvalidFederatedKey" => DeviceBoundSessionFetchResult::InvalidFederatedKey,
            "TooManyRelyingOriginLabels" => {
                DeviceBoundSessionFetchResult::TooManyRelyingOriginLabels
            }
            "BoundCookieSetForbidden" => DeviceBoundSessionFetchResult::BoundCookieSetForbidden,
            "NetError" => DeviceBoundSessionFetchResult::NetError,
            "ProxyError" => DeviceBoundSessionFetchResult::ProxyError,
            "EmptySessionConfig" => DeviceBoundSessionFetchResult::EmptySessionConfig,
            "InvalidCredentialsConfig" => DeviceBoundSessionFetchResult::InvalidCredentialsConfig,
            "InvalidCredentialsType" => DeviceBoundSessionFetchResult::InvalidCredentialsType,
            "InvalidCredentialsEmptyName" => {
                DeviceBoundSessionFetchResult::InvalidCredentialsEmptyName
            }
            "InvalidCredentialsCookie" => DeviceBoundSessionFetchResult::InvalidCredentialsCookie,
            "PersistentHttpError" => DeviceBoundSessionFetchResult::PersistentHttpError,
            "RegistrationAttemptedChallenge" => {
                DeviceBoundSessionFetchResult::RegistrationAttemptedChallenge
            }
            "InvalidScopeOrigin" => DeviceBoundSessionFetchResult::InvalidScopeOrigin,
            "ScopeOriginContainsPath" => DeviceBoundSessionFetchResult::ScopeOriginContainsPath,
            "RefreshInitiatorNotString" => DeviceBoundSessionFetchResult::RefreshInitiatorNotString,
            "RefreshInitiatorInvalidHostPattern" => {
                DeviceBoundSessionFetchResult::RefreshInitiatorInvalidHostPattern
            }
            "InvalidScopeSpecification" => DeviceBoundSessionFetchResult::InvalidScopeSpecification,
            "MissingScopeSpecificationType" => {
                DeviceBoundSessionFetchResult::MissingScopeSpecificationType
            }
            "EmptyScopeSpecificationDomain" => {
                DeviceBoundSessionFetchResult::EmptyScopeSpecificationDomain
            }
            "EmptyScopeSpecificationPath" => {
                DeviceBoundSessionFetchResult::EmptyScopeSpecificationPath
            }
            "InvalidScopeSpecificationType" => {
                DeviceBoundSessionFetchResult::InvalidScopeSpecificationType
            }
            "InvalidScopeIncludeSite" => DeviceBoundSessionFetchResult::InvalidScopeIncludeSite,
            "MissingScopeIncludeSite" => DeviceBoundSessionFetchResult::MissingScopeIncludeSite,
            "FederatedNotAuthorizedByProvider" => {
                DeviceBoundSessionFetchResult::FederatedNotAuthorizedByProvider
            }
            "FederatedNotAuthorizedByRelyingParty" => {
                DeviceBoundSessionFetchResult::FederatedNotAuthorizedByRelyingParty
            }
            "SessionProviderWellKnownMalformed" => {
                DeviceBoundSessionFetchResult::SessionProviderWellKnownMalformed
            }
            "SessionProviderWellKnownHasProviderOrigin" => {
                DeviceBoundSessionFetchResult::SessionProviderWellKnownHasProviderOrigin
            }
            "RelyingPartyWellKnownMalformed" => {
                DeviceBoundSessionFetchResult::RelyingPartyWellKnownMalformed
            }
            "RelyingPartyWellKnownHasRelyingOrigins" => {
                DeviceBoundSessionFetchResult::RelyingPartyWellKnownHasRelyingOrigins
            }
            "InvalidFederatedSessionProviderSessionMissing" => {
                DeviceBoundSessionFetchResult::InvalidFederatedSessionProviderSessionMissing
            }
            "InvalidFederatedSessionWrongProviderOrigin" => {
                DeviceBoundSessionFetchResult::InvalidFederatedSessionWrongProviderOrigin
            }
            "InvalidCredentialsCookieCreationTime" => {
                DeviceBoundSessionFetchResult::InvalidCredentialsCookieCreationTime
            }
            "InvalidCredentialsCookieName" => {
                DeviceBoundSessionFetchResult::InvalidCredentialsCookieName
            }
            "InvalidCredentialsCookieParsing" => {
                DeviceBoundSessionFetchResult::InvalidCredentialsCookieParsing
            }
            "InvalidCredentialsCookieUnpermittedAttribute" => {
                DeviceBoundSessionFetchResult::InvalidCredentialsCookieUnpermittedAttribute
            }
            "InvalidCredentialsCookieInvalidDomain" => {
                DeviceBoundSessionFetchResult::InvalidCredentialsCookieInvalidDomain
            }
            "InvalidCredentialsCookiePrefix" => {
                DeviceBoundSessionFetchResult::InvalidCredentialsCookiePrefix
            }
            "InvalidScopeRulePath" => DeviceBoundSessionFetchResult::InvalidScopeRulePath,
            "InvalidScopeRuleHostPattern" => {
                DeviceBoundSessionFetchResult::InvalidScopeRuleHostPattern
            }
            "ScopeRuleOriginScopedHostPatternMismatch" => {
                DeviceBoundSessionFetchResult::ScopeRuleOriginScopedHostPatternMismatch
            }
            "ScopeRuleSiteScopedHostPatternMismatch" => {
                DeviceBoundSessionFetchResult::ScopeRuleSiteScopedHostPatternMismatch
            }
            "SigningQuotaExceeded" => DeviceBoundSessionFetchResult::SigningQuotaExceeded,
            "InvalidConfigJson" => DeviceBoundSessionFetchResult::InvalidConfigJson,
            "InvalidFederatedSessionProviderFailedToRestoreKey" => {
                DeviceBoundSessionFetchResult::InvalidFederatedSessionProviderFailedToRestoreKey
            }
            "FailedToUnwrapKey" => DeviceBoundSessionFetchResult::FailedToUnwrapKey,
            "SessionDeletedDuringRefresh" => {
                DeviceBoundSessionFetchResult::SessionDeletedDuringRefresh
            }
            "CrossOriginRegistrationSiteNotIncluded" => {
                DeviceBoundSessionFetchResult::CrossOriginRegistrationSiteNotIncluded
            }
            "InvalidPreProvisionedKeyInitiatorMissing" => {
                DeviceBoundSessionFetchResult::InvalidPreProvisionedKeyInitiatorMissing
            }
            "PreProvisionedKeyAccessNotGranted" => {
                DeviceBoundSessionFetchResult::PreProvisionedKeyAccessNotGranted
            }
            "PreProvisionedKeyNotFound" => DeviceBoundSessionFetchResult::PreProvisionedKeyNotFound,
            "AttestationCertificationError" => {
                DeviceBoundSessionFetchResult::AttestationCertificationError
            }
            "AttestationSigningError" => DeviceBoundSessionFetchResult::AttestationSigningError,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for DeviceBoundSessionFetchResult {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeviceBoundSessionFetchResult {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceBoundSessionFetchResult;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(
                    match v {
                        "Success" => DeviceBoundSessionFetchResult::Success,
                        "SigningKeyGenerationError" => {
                            DeviceBoundSessionFetchResult::SigningKeyGenerationError
                        }
                        "AttestationKeyGenerationError" => {
                            DeviceBoundSessionFetchResult::AttestationKeyGenerationError
                        }
                        "SigningError" => DeviceBoundSessionFetchResult::SigningError,
                        "TransientSigningError" => {
                            DeviceBoundSessionFetchResult::TransientSigningError
                        }
                        "ServerRequestedTermination" => {
                            DeviceBoundSessionFetchResult::ServerRequestedTermination
                        }
                        "InvalidSessionId" => {
                            DeviceBoundSessionFetchResult::InvalidSessionId
                        }
                        "InvalidChallenge" => {
                            DeviceBoundSessionFetchResult::InvalidChallenge
                        }
                        "TooManyChallenges" => {
                            DeviceBoundSessionFetchResult::TooManyChallenges
                        }
                        "InvalidFetcherUrl" => {
                            DeviceBoundSessionFetchResult::InvalidFetcherUrl
                        }
                        "InvalidRefreshUrl" => {
                            DeviceBoundSessionFetchResult::InvalidRefreshUrl
                        }
                        "TransientHttpError" => {
                            DeviceBoundSessionFetchResult::TransientHttpError
                        }
                        "ScopeOriginSameSiteMismatch" => {
                            DeviceBoundSessionFetchResult::ScopeOriginSameSiteMismatch
                        }
                        "RefreshUrlSameSiteMismatch" => {
                            DeviceBoundSessionFetchResult::RefreshUrlSameSiteMismatch
                        }
                        "MismatchedSessionId" => {
                            DeviceBoundSessionFetchResult::MismatchedSessionId
                        }
                        "MissingScope" => DeviceBoundSessionFetchResult::MissingScope,
                        "NoCredentials" => DeviceBoundSessionFetchResult::NoCredentials,
                        "SubdomainRegistrationWellKnownUnavailable" => {
                            DeviceBoundSessionFetchResult::SubdomainRegistrationWellKnownUnavailable
                        }
                        "SubdomainRegistrationUnauthorized" => {
                            DeviceBoundSessionFetchResult::SubdomainRegistrationUnauthorized
                        }
                        "SubdomainRegistrationWellKnownMalformed" => {
                            DeviceBoundSessionFetchResult::SubdomainRegistrationWellKnownMalformed
                        }
                        "SessionProviderWellKnownUnavailable" => {
                            DeviceBoundSessionFetchResult::SessionProviderWellKnownUnavailable
                        }
                        "RelyingPartyWellKnownUnavailable" => {
                            DeviceBoundSessionFetchResult::RelyingPartyWellKnownUnavailable
                        }
                        "FederatedKeyThumbprintMismatch" => {
                            DeviceBoundSessionFetchResult::FederatedKeyThumbprintMismatch
                        }
                        "InvalidFederatedSessionUrl" => {
                            DeviceBoundSessionFetchResult::InvalidFederatedSessionUrl
                        }
                        "InvalidFederatedKey" => {
                            DeviceBoundSessionFetchResult::InvalidFederatedKey
                        }
                        "TooManyRelyingOriginLabels" => {
                            DeviceBoundSessionFetchResult::TooManyRelyingOriginLabels
                        }
                        "BoundCookieSetForbidden" => {
                            DeviceBoundSessionFetchResult::BoundCookieSetForbidden
                        }
                        "NetError" => DeviceBoundSessionFetchResult::NetError,
                        "ProxyError" => DeviceBoundSessionFetchResult::ProxyError,
                        "EmptySessionConfig" => {
                            DeviceBoundSessionFetchResult::EmptySessionConfig
                        }
                        "InvalidCredentialsConfig" => {
                            DeviceBoundSessionFetchResult::InvalidCredentialsConfig
                        }
                        "InvalidCredentialsType" => {
                            DeviceBoundSessionFetchResult::InvalidCredentialsType
                        }
                        "InvalidCredentialsEmptyName" => {
                            DeviceBoundSessionFetchResult::InvalidCredentialsEmptyName
                        }
                        "InvalidCredentialsCookie" => {
                            DeviceBoundSessionFetchResult::InvalidCredentialsCookie
                        }
                        "PersistentHttpError" => {
                            DeviceBoundSessionFetchResult::PersistentHttpError
                        }
                        "RegistrationAttemptedChallenge" => {
                            DeviceBoundSessionFetchResult::RegistrationAttemptedChallenge
                        }
                        "InvalidScopeOrigin" => {
                            DeviceBoundSessionFetchResult::InvalidScopeOrigin
                        }
                        "ScopeOriginContainsPath" => {
                            DeviceBoundSessionFetchResult::ScopeOriginContainsPath
                        }
                        "RefreshInitiatorNotString" => {
                            DeviceBoundSessionFetchResult::RefreshInitiatorNotString
                        }
                        "RefreshInitiatorInvalidHostPattern" => {
                            DeviceBoundSessionFetchResult::RefreshInitiatorInvalidHostPattern
                        }
                        "InvalidScopeSpecification" => {
                            DeviceBoundSessionFetchResult::InvalidScopeSpecification
                        }
                        "MissingScopeSpecificationType" => {
                            DeviceBoundSessionFetchResult::MissingScopeSpecificationType
                        }
                        "EmptyScopeSpecificationDomain" => {
                            DeviceBoundSessionFetchResult::EmptyScopeSpecificationDomain
                        }
                        "EmptyScopeSpecificationPath" => {
                            DeviceBoundSessionFetchResult::EmptyScopeSpecificationPath
                        }
                        "InvalidScopeSpecificationType" => {
                            DeviceBoundSessionFetchResult::InvalidScopeSpecificationType
                        }
                        "InvalidScopeIncludeSite" => {
                            DeviceBoundSessionFetchResult::InvalidScopeIncludeSite
                        }
                        "MissingScopeIncludeSite" => {
                            DeviceBoundSessionFetchResult::MissingScopeIncludeSite
                        }
                        "FederatedNotAuthorizedByProvider" => {
                            DeviceBoundSessionFetchResult::FederatedNotAuthorizedByProvider
                        }
                        "FederatedNotAuthorizedByRelyingParty" => {
                            DeviceBoundSessionFetchResult::FederatedNotAuthorizedByRelyingParty
                        }
                        "SessionProviderWellKnownMalformed" => {
                            DeviceBoundSessionFetchResult::SessionProviderWellKnownMalformed
                        }
                        "SessionProviderWellKnownHasProviderOrigin" => {
                            DeviceBoundSessionFetchResult::SessionProviderWellKnownHasProviderOrigin
                        }
                        "RelyingPartyWellKnownMalformed" => {
                            DeviceBoundSessionFetchResult::RelyingPartyWellKnownMalformed
                        }
                        "RelyingPartyWellKnownHasRelyingOrigins" => {
                            DeviceBoundSessionFetchResult::RelyingPartyWellKnownHasRelyingOrigins
                        }
                        "InvalidFederatedSessionProviderSessionMissing" => {
                            DeviceBoundSessionFetchResult::InvalidFederatedSessionProviderSessionMissing
                        }
                        "InvalidFederatedSessionWrongProviderOrigin" => {
                            DeviceBoundSessionFetchResult::InvalidFederatedSessionWrongProviderOrigin
                        }
                        "InvalidCredentialsCookieCreationTime" => {
                            DeviceBoundSessionFetchResult::InvalidCredentialsCookieCreationTime
                        }
                        "InvalidCredentialsCookieName" => {
                            DeviceBoundSessionFetchResult::InvalidCredentialsCookieName
                        }
                        "InvalidCredentialsCookieParsing" => {
                            DeviceBoundSessionFetchResult::InvalidCredentialsCookieParsing
                        }
                        "InvalidCredentialsCookieUnpermittedAttribute" => {
                            DeviceBoundSessionFetchResult::InvalidCredentialsCookieUnpermittedAttribute
                        }
                        "InvalidCredentialsCookieInvalidDomain" => {
                            DeviceBoundSessionFetchResult::InvalidCredentialsCookieInvalidDomain
                        }
                        "InvalidCredentialsCookiePrefix" => {
                            DeviceBoundSessionFetchResult::InvalidCredentialsCookiePrefix
                        }
                        "InvalidScopeRulePath" => {
                            DeviceBoundSessionFetchResult::InvalidScopeRulePath
                        }
                        "InvalidScopeRuleHostPattern" => {
                            DeviceBoundSessionFetchResult::InvalidScopeRuleHostPattern
                        }
                        "ScopeRuleOriginScopedHostPatternMismatch" => {
                            DeviceBoundSessionFetchResult::ScopeRuleOriginScopedHostPatternMismatch
                        }
                        "ScopeRuleSiteScopedHostPatternMismatch" => {
                            DeviceBoundSessionFetchResult::ScopeRuleSiteScopedHostPatternMismatch
                        }
                        "SigningQuotaExceeded" => {
                            DeviceBoundSessionFetchResult::SigningQuotaExceeded
                        }
                        "InvalidConfigJson" => {
                            DeviceBoundSessionFetchResult::InvalidConfigJson
                        }
                        "InvalidFederatedSessionProviderFailedToRestoreKey" => {
                            DeviceBoundSessionFetchResult::InvalidFederatedSessionProviderFailedToRestoreKey
                        }
                        "FailedToUnwrapKey" => {
                            DeviceBoundSessionFetchResult::FailedToUnwrapKey
                        }
                        "SessionDeletedDuringRefresh" => {
                            DeviceBoundSessionFetchResult::SessionDeletedDuringRefresh
                        }
                        "CrossOriginRegistrationSiteNotIncluded" => {
                            DeviceBoundSessionFetchResult::CrossOriginRegistrationSiteNotIncluded
                        }
                        "InvalidPreProvisionedKeyInitiatorMissing" => {
                            DeviceBoundSessionFetchResult::InvalidPreProvisionedKeyInitiatorMissing
                        }
                        "PreProvisionedKeyAccessNotGranted" => {
                            DeviceBoundSessionFetchResult::PreProvisionedKeyAccessNotGranted
                        }
                        "PreProvisionedKeyNotFound" => {
                            DeviceBoundSessionFetchResult::PreProvisionedKeyNotFound
                        }
                        "AttestationCertificationError" => {
                            DeviceBoundSessionFetchResult::AttestationCertificationError
                        }
                        "AttestationSigningError" => {
                            DeviceBoundSessionFetchResult::AttestationSigningError
                        }
                        _ => DeviceBoundSessionFetchResult::Unknown(v.to_owned()),
                    },
                )
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(DeviceBoundSessionFetchResult::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.DeviceBoundSessionInclusionRules。
///
///协议原文：
///
///<pre>A device bound session's inclusion rules.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeviceBoundSessionInclusionRules {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionInclusionRules/properties/origin。
    ///
    ///协议原文：
    ///
    ///<pre>See comments on `net::device_bound_sessions::SessionInclusionRules::origin_`.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionInclusionRules/properties/includeSite。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the whole site is included. See comments on
    ///`net::device_bound_sessions::SessionInclusionRules::include_site_` for more
    ///details; this boolean is true if that value is populated.</pre>
    ///
    ///
    #[serde(rename = "includeSite")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub include_site: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionInclusionRules/properties/urlRules。
    ///
    ///协议原文：
    ///
    ///<pre>See comments on `net::device_bound_sessions::SessionInclusionRules::url_rules_`.</pre>
    ///
    ///
    #[serde(rename = "urlRules")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url_rules: Vec<crate::network::types::DeviceBoundSessionUrlRule>,
}
impl<'de> serde::Deserialize<'de> for DeviceBoundSessionInclusionRules {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceBoundSessionInclusionRules;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionInclusionRules/properties/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>See comments on `net::device_bound_sessions::SessionInclusionRules::origin_`.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionInclusionRules/properties/includeSite。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the whole site is included. See comments on
                    ///`net::device_bound_sessions::SessionInclusionRules::include_site_` for more
                    ///details; this boolean is true if that value is populated.</pre>
                    ///
                    ///
                    #[serde(rename = "includeSite")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub include_site: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionInclusionRules/properties/urlRules。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>See comments on `net::device_bound_sessions::SessionInclusionRules::url_rules_`.</pre>
                    ///
                    ///
                    #[serde(rename = "urlRules")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url_rules: Vec<crate::network::types::DeviceBoundSessionUrlRule>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DeviceBoundSessionInclusionRules {
                    origin: fields.origin,
                    include_site: fields.include_site,
                    url_rules: fields.url_rules,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.DeviceBoundSessionKey。
///
///协议原文：
///
///<pre>Unique identifier for a device bound session.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeviceBoundSessionKey {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionKey/properties/site。
    ///
    ///协议原文：
    ///
    ///<pre>The site the session is set up for.</pre>
    ///
    ///
    #[serde(rename = "site")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub site: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionKey/properties/id。
    ///
    ///协议原文：
    ///
    ///<pre>The id of the session.</pre>
    ///
    ///
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: String,
}
impl<'de> serde::Deserialize<'de> for DeviceBoundSessionKey {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceBoundSessionKey;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionKey/properties/site。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The site the session is set up for.</pre>
                    ///
                    ///
                    #[serde(rename = "site")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub site: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionKey/properties/id。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The id of the session.</pre>
                    ///
                    ///
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DeviceBoundSessionKey {
                    site: fields.site,
                    id: fields.id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.DeviceBoundSessionUrlRule/properties/ruleType。
///
///协议原文：
///
///<pre>See comments on `net::device_bound_sessions::SessionInclusionRules::UrlRule::rule_type`.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DeviceBoundSessionUrlRuleRuleType {
    ///协议值 "Exclude"。
    Exclude,
    ///协议值 "Include"。
    Include,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl DeviceBoundSessionUrlRuleRuleType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::DeviceBoundSessionUrlRuleRuleType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Exclude => "Exclude",
            Self::Include => "Include",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for DeviceBoundSessionUrlRuleRuleType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for DeviceBoundSessionUrlRuleRuleType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Exclude" => DeviceBoundSessionUrlRuleRuleType::Exclude,
            "Include" => DeviceBoundSessionUrlRuleRuleType::Include,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for DeviceBoundSessionUrlRuleRuleType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeviceBoundSessionUrlRuleRuleType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceBoundSessionUrlRuleRuleType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Exclude" => DeviceBoundSessionUrlRuleRuleType::Exclude,
                    "Include" => DeviceBoundSessionUrlRuleRuleType::Include,
                    _ => DeviceBoundSessionUrlRuleRuleType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(DeviceBoundSessionUrlRuleRuleType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.DeviceBoundSessionUrlRule。
///
///协议原文：
///
///<pre>A device bound session's inclusion URL rule.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeviceBoundSessionUrlRule {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionUrlRule/properties/ruleType。
    ///
    ///协议原文：
    ///
    ///<pre>See comments on `net::device_bound_sessions::SessionInclusionRules::UrlRule::rule_type`.</pre>
    ///
    ///
    #[serde(rename = "ruleType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub rule_type: DeviceBoundSessionUrlRuleRuleType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionUrlRule/properties/hostPattern。
    ///
    ///协议原文：
    ///
    ///<pre>See comments on `net::device_bound_sessions::SessionInclusionRules::UrlRule::host_pattern`.</pre>
    ///
    ///
    #[serde(rename = "hostPattern")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub host_pattern: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionUrlRule/properties/pathPrefix。
    ///
    ///协议原文：
    ///
    ///<pre>See comments on `net::device_bound_sessions::SessionInclusionRules::UrlRule::path_prefix`.</pre>
    ///
    ///
    #[serde(rename = "pathPrefix")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub path_prefix: String,
}
impl<'de> serde::Deserialize<'de> for DeviceBoundSessionUrlRule {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceBoundSessionUrlRule;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionUrlRule/properties/ruleType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>See comments on `net::device_bound_sessions::SessionInclusionRules::UrlRule::rule_type`.</pre>
                    ///
                    ///
                    #[serde(rename = "ruleType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub rule_type: DeviceBoundSessionUrlRuleRuleType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionUrlRule/properties/hostPattern。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>See comments on `net::device_bound_sessions::SessionInclusionRules::UrlRule::host_pattern`.</pre>
                    ///
                    ///
                    #[serde(rename = "hostPattern")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub host_pattern: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionUrlRule/properties/pathPrefix。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>See comments on `net::device_bound_sessions::SessionInclusionRules::UrlRule::path_prefix`.</pre>
                    ///
                    ///
                    #[serde(rename = "pathPrefix")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub path_prefix: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DeviceBoundSessionUrlRule {
                    rule_type: fields.rule_type,
                    host_pattern: fields.host_pattern,
                    path_prefix: fields.path_prefix,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.DeviceBoundSessionWithUsage/properties/usage。
///
///协议原文：
///
///<pre>How the session was used (or not used).</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DeviceBoundSessionWithUsageUsage {
    ///协议值 "NotInScope"。
    NotInScope,
    ///协议值 "InScopeRefreshNotYetNeeded"。
    InScopeRefreshNotYetNeeded,
    ///协议值 "InScopeRefreshNotAllowed"。
    InScopeRefreshNotAllowed,
    ///协议值 "ProactiveRefreshNotPossible"。
    ProactiveRefreshNotPossible,
    ///协议值 "ProactiveRefreshAttempted"。
    ProactiveRefreshAttempted,
    ///协议值 "Deferred"。
    Deferred,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl DeviceBoundSessionWithUsageUsage {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::DeviceBoundSessionWithUsageUsage::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::NotInScope => "NotInScope",
            Self::InScopeRefreshNotYetNeeded => "InScopeRefreshNotYetNeeded",
            Self::InScopeRefreshNotAllowed => "InScopeRefreshNotAllowed",
            Self::ProactiveRefreshNotPossible => "ProactiveRefreshNotPossible",
            Self::ProactiveRefreshAttempted => "ProactiveRefreshAttempted",
            Self::Deferred => "Deferred",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for DeviceBoundSessionWithUsageUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for DeviceBoundSessionWithUsageUsage {
    fn from(value: String) -> Self {
        match value.as_str() {
            "NotInScope" => DeviceBoundSessionWithUsageUsage::NotInScope,
            "InScopeRefreshNotYetNeeded" => {
                DeviceBoundSessionWithUsageUsage::InScopeRefreshNotYetNeeded
            }
            "InScopeRefreshNotAllowed" => {
                DeviceBoundSessionWithUsageUsage::InScopeRefreshNotAllowed
            }
            "ProactiveRefreshNotPossible" => {
                DeviceBoundSessionWithUsageUsage::ProactiveRefreshNotPossible
            }
            "ProactiveRefreshAttempted" => {
                DeviceBoundSessionWithUsageUsage::ProactiveRefreshAttempted
            }
            "Deferred" => DeviceBoundSessionWithUsageUsage::Deferred,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for DeviceBoundSessionWithUsageUsage {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeviceBoundSessionWithUsageUsage {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceBoundSessionWithUsageUsage;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "NotInScope" => DeviceBoundSessionWithUsageUsage::NotInScope,
                    "InScopeRefreshNotYetNeeded" => {
                        DeviceBoundSessionWithUsageUsage::InScopeRefreshNotYetNeeded
                    }
                    "InScopeRefreshNotAllowed" => {
                        DeviceBoundSessionWithUsageUsage::InScopeRefreshNotAllowed
                    }
                    "ProactiveRefreshNotPossible" => {
                        DeviceBoundSessionWithUsageUsage::ProactiveRefreshNotPossible
                    }
                    "ProactiveRefreshAttempted" => {
                        DeviceBoundSessionWithUsageUsage::ProactiveRefreshAttempted
                    }
                    "Deferred" => DeviceBoundSessionWithUsageUsage::Deferred,
                    _ => DeviceBoundSessionWithUsageUsage::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(DeviceBoundSessionWithUsageUsage::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.DeviceBoundSessionWithUsage。
///
///协议原文：
///
///<pre>How a device bound session was used during a request.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeviceBoundSessionWithUsage {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionWithUsage/properties/sessionKey。
    ///
    ///协议原文：
    ///
    ///<pre>The key for the session.</pre>
    ///
    ///
    #[serde(rename = "sessionKey")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub session_key: crate::network::types::DeviceBoundSessionKey,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionWithUsage/properties/usage。
    ///
    ///协议原文：
    ///
    ///<pre>How the session was used (or not used).</pre>
    ///
    ///
    #[serde(rename = "usage")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub usage: DeviceBoundSessionWithUsageUsage,
}
impl<'de> serde::Deserialize<'de> for DeviceBoundSessionWithUsage {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceBoundSessionWithUsage;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionWithUsage/properties/sessionKey。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The key for the session.</pre>
                    ///
                    ///
                    #[serde(rename = "sessionKey")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub session_key: crate::network::types::DeviceBoundSessionKey,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DeviceBoundSessionWithUsage/properties/usage。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>How the session was used (or not used).</pre>
                    ///
                    ///
                    #[serde(rename = "usage")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub usage: DeviceBoundSessionWithUsageUsage,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DeviceBoundSessionWithUsage {
                    session_key: fields.session_key,
                    usage: fields.usage,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.DirectSocketDnsQueryType。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DirectSocketDnsQueryType {
    ///协议值 "ipv4"。
    Ipv4,
    ///协议值 "ipv6"。
    Ipv6,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl DirectSocketDnsQueryType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::DirectSocketDnsQueryType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ipv4 => "ipv4",
            Self::Ipv6 => "ipv6",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for DirectSocketDnsQueryType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for DirectSocketDnsQueryType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ipv4" => DirectSocketDnsQueryType::Ipv4,
            "ipv6" => DirectSocketDnsQueryType::Ipv6,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for DirectSocketDnsQueryType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DirectSocketDnsQueryType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DirectSocketDnsQueryType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "ipv4" => DirectSocketDnsQueryType::Ipv4,
                    "ipv6" => DirectSocketDnsQueryType::Ipv6,
                    _ => DirectSocketDnsQueryType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(DirectSocketDnsQueryType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.DirectTCPSocketOptions。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DirectTcpSocketOptions {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DirectTCPSocketOptions/properties/noDelay。
    ///
    ///协议原文：
    ///
    ///<pre>TCP_NODELAY option</pre>
    ///
    ///
    #[serde(rename = "noDelay")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub no_delay: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectTCPSocketOptions/properties/keepAliveDelay。
    ///
    ///协议原文：
    ///
    ///<pre>Expected to be unsigned integer.</pre>
    ///
    ///
    #[serde(rename = "keepAliveDelay")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub keep_alive_delay: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectTCPSocketOptions/properties/sendBufferSize。
    ///
    ///协议原文：
    ///
    ///<pre>Expected to be unsigned integer.</pre>
    ///
    ///
    #[serde(rename = "sendBufferSize")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub send_buffer_size: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectTCPSocketOptions/properties/receiveBufferSize。
    ///
    ///协议原文：
    ///
    ///<pre>Expected to be unsigned integer.</pre>
    ///
    ///
    #[serde(rename = "receiveBufferSize")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub receive_buffer_size: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectTCPSocketOptions/properties/dnsQueryType。
    #[serde(rename = "dnsQueryType")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub dns_query_type: Option<crate::network::types::DirectSocketDnsQueryType>,
}
impl<'de> serde::Deserialize<'de> for DirectTcpSocketOptions {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DirectTcpSocketOptions;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DirectTCPSocketOptions/properties/noDelay。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>TCP_NODELAY option</pre>
                    ///
                    ///
                    #[serde(rename = "noDelay")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub no_delay: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectTCPSocketOptions/properties/keepAliveDelay。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Expected to be unsigned integer.</pre>
                    ///
                    ///
                    #[serde(rename = "keepAliveDelay")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub keep_alive_delay: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectTCPSocketOptions/properties/sendBufferSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Expected to be unsigned integer.</pre>
                    ///
                    ///
                    #[serde(rename = "sendBufferSize")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub send_buffer_size: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectTCPSocketOptions/properties/receiveBufferSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Expected to be unsigned integer.</pre>
                    ///
                    ///
                    #[serde(rename = "receiveBufferSize")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub receive_buffer_size: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectTCPSocketOptions/properties/dnsQueryType。
                    #[serde(rename = "dnsQueryType")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub dns_query_type: Option<crate::network::types::DirectSocketDnsQueryType>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DirectTcpSocketOptions {
                    no_delay: fields.no_delay,
                    keep_alive_delay: fields.keep_alive_delay,
                    send_buffer_size: fields.send_buffer_size,
                    receive_buffer_size: fields.receive_buffer_size,
                    dns_query_type: fields.dns_query_type,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.DirectUDPMessage。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DirectUdpMessage {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DirectUDPMessage/properties/data。
    #[serde(rename = "data")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub data: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectUDPMessage/properties/remoteAddr。
    ///
    ///协议原文：
    ///
    ///<pre>Null for connected mode.</pre>
    ///
    ///
    #[serde(rename = "remoteAddr")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_addr: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectUDPMessage/properties/remotePort。
    ///
    ///协议原文：
    ///
    ///<pre>Null for connected mode.
    ///Expected to be unsigned integer.</pre>
    ///
    ///
    #[serde(rename = "remotePort")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_port: Option<i64>,
}
impl<'de> serde::Deserialize<'de> for DirectUdpMessage {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DirectUdpMessage;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.DirectUDPMessage/properties/data。
                    #[serde(rename = "data")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub data: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectUDPMessage/properties/remoteAddr。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Null for connected mode.</pre>
                    ///
                    ///
                    #[serde(rename = "remoteAddr")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub remote_addr: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectUDPMessage/properties/remotePort。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Null for connected mode.
                    ///Expected to be unsigned integer.</pre>
                    ///
                    ///
                    #[serde(rename = "remotePort")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub remote_port: Option<i64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DirectUdpMessage {
                    data: fields.data,
                    remote_addr: fields.remote_addr,
                    remote_port: fields.remote_port,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.DirectUDPSocketOptions。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct DirectUdpSocketOptions {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectUDPSocketOptions/properties/remoteAddr。
    #[serde(rename = "remoteAddr")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_addr: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectUDPSocketOptions/properties/remotePort。
    ///
    ///协议原文：
    ///
    ///<pre>Unsigned int 16.</pre>
    ///
    ///
    #[serde(rename = "remotePort")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_port: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectUDPSocketOptions/properties/localAddr。
    #[serde(rename = "localAddr")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub local_addr: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectUDPSocketOptions/properties/localPort。
    ///
    ///协议原文：
    ///
    ///<pre>Unsigned int 16.</pre>
    ///
    ///
    #[serde(rename = "localPort")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub local_port: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectUDPSocketOptions/properties/dnsQueryType。
    #[serde(rename = "dnsQueryType")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub dns_query_type: Option<crate::network::types::DirectSocketDnsQueryType>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectUDPSocketOptions/properties/sendBufferSize。
    ///
    ///协议原文：
    ///
    ///<pre>Expected to be unsigned integer.</pre>
    ///
    ///
    #[serde(rename = "sendBufferSize")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub send_buffer_size: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectUDPSocketOptions/properties/receiveBufferSize。
    ///
    ///协议原文：
    ///
    ///<pre>Expected to be unsigned integer.</pre>
    ///
    ///
    #[serde(rename = "receiveBufferSize")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub receive_buffer_size: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectUDPSocketOptions/properties/multicastLoopback。
    #[serde(rename = "multicastLoopback")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub multicast_loopback: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectUDPSocketOptions/properties/multicastTimeToLive。
    ///
    ///协议原文：
    ///
    ///<pre>Unsigned int 8.</pre>
    ///
    ///
    #[serde(rename = "multicastTimeToLive")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub multicast_time_to_live: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectUDPSocketOptions/properties/multicastAllowAddressSharing。
    #[serde(rename = "multicastAllowAddressSharing")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub multicast_allow_address_sharing: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for DirectUdpSocketOptions {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DirectUdpSocketOptions;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectUDPSocketOptions/properties/remoteAddr。
                    #[serde(rename = "remoteAddr")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub remote_addr: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectUDPSocketOptions/properties/remotePort。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Unsigned int 16.</pre>
                    ///
                    ///
                    #[serde(rename = "remotePort")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub remote_port: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectUDPSocketOptions/properties/localAddr。
                    #[serde(rename = "localAddr")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub local_addr: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectUDPSocketOptions/properties/localPort。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Unsigned int 16.</pre>
                    ///
                    ///
                    #[serde(rename = "localPort")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub local_port: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectUDPSocketOptions/properties/dnsQueryType。
                    #[serde(rename = "dnsQueryType")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub dns_query_type: Option<crate::network::types::DirectSocketDnsQueryType>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectUDPSocketOptions/properties/sendBufferSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Expected to be unsigned integer.</pre>
                    ///
                    ///
                    #[serde(rename = "sendBufferSize")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub send_buffer_size: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectUDPSocketOptions/properties/receiveBufferSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Expected to be unsigned integer.</pre>
                    ///
                    ///
                    #[serde(rename = "receiveBufferSize")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub receive_buffer_size: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectUDPSocketOptions/properties/multicastLoopback。
                    #[serde(rename = "multicastLoopback")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub multicast_loopback: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectUDPSocketOptions/properties/multicastTimeToLive。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Unsigned int 8.</pre>
                    ///
                    ///
                    #[serde(rename = "multicastTimeToLive")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub multicast_time_to_live: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.DirectUDPSocketOptions/properties/multicastAllowAddressSharing。
                    #[serde(rename = "multicastAllowAddressSharing")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub multicast_allow_address_sharing: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(DirectUdpSocketOptions {
                    remote_addr: fields.remote_addr,
                    remote_port: fields.remote_port,
                    local_addr: fields.local_addr,
                    local_port: fields.local_port,
                    dns_query_type: fields.dns_query_type,
                    send_buffer_size: fields.send_buffer_size,
                    receive_buffer_size: fields.receive_buffer_size,
                    multicast_loopback: fields.multicast_loopback,
                    multicast_time_to_live: fields.multicast_time_to_live,
                    multicast_allow_address_sharing: fields.multicast_allow_address_sharing,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.ErrorReason。
///
///协议原文：
///
///<pre>Network level fetch failure reason.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorReason {
    ///协议值 "Failed"。
    Failed,
    ///协议值 "Aborted"。
    Aborted,
    ///协议值 "TimedOut"。
    TimedOut,
    ///协议值 "AccessDenied"。
    AccessDenied,
    ///协议值 "ConnectionClosed"。
    ConnectionClosed,
    ///协议值 "ConnectionReset"。
    ConnectionReset,
    ///协议值 "ConnectionRefused"。
    ConnectionRefused,
    ///协议值 "ConnectionAborted"。
    ConnectionAborted,
    ///协议值 "ConnectionFailed"。
    ConnectionFailed,
    ///协议值 "NameNotResolved"。
    NameNotResolved,
    ///协议值 "InternetDisconnected"。
    InternetDisconnected,
    ///协议值 "AddressUnreachable"。
    AddressUnreachable,
    ///协议值 "BlockedByClient"。
    BlockedByClient,
    ///协议值 "BlockedByResponse"。
    BlockedByResponse,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ErrorReason {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::ErrorReason::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Failed => "Failed",
            Self::Aborted => "Aborted",
            Self::TimedOut => "TimedOut",
            Self::AccessDenied => "AccessDenied",
            Self::ConnectionClosed => "ConnectionClosed",
            Self::ConnectionReset => "ConnectionReset",
            Self::ConnectionRefused => "ConnectionRefused",
            Self::ConnectionAborted => "ConnectionAborted",
            Self::ConnectionFailed => "ConnectionFailed",
            Self::NameNotResolved => "NameNotResolved",
            Self::InternetDisconnected => "InternetDisconnected",
            Self::AddressUnreachable => "AddressUnreachable",
            Self::BlockedByClient => "BlockedByClient",
            Self::BlockedByResponse => "BlockedByResponse",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ErrorReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ErrorReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Failed" => ErrorReason::Failed,
            "Aborted" => ErrorReason::Aborted,
            "TimedOut" => ErrorReason::TimedOut,
            "AccessDenied" => ErrorReason::AccessDenied,
            "ConnectionClosed" => ErrorReason::ConnectionClosed,
            "ConnectionReset" => ErrorReason::ConnectionReset,
            "ConnectionRefused" => ErrorReason::ConnectionRefused,
            "ConnectionAborted" => ErrorReason::ConnectionAborted,
            "ConnectionFailed" => ErrorReason::ConnectionFailed,
            "NameNotResolved" => ErrorReason::NameNotResolved,
            "InternetDisconnected" => ErrorReason::InternetDisconnected,
            "AddressUnreachable" => ErrorReason::AddressUnreachable,
            "BlockedByClient" => ErrorReason::BlockedByClient,
            "BlockedByResponse" => ErrorReason::BlockedByResponse,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ErrorReason {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ErrorReason {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ErrorReason;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Failed" => ErrorReason::Failed,
                    "Aborted" => ErrorReason::Aborted,
                    "TimedOut" => ErrorReason::TimedOut,
                    "AccessDenied" => ErrorReason::AccessDenied,
                    "ConnectionClosed" => ErrorReason::ConnectionClosed,
                    "ConnectionReset" => ErrorReason::ConnectionReset,
                    "ConnectionRefused" => ErrorReason::ConnectionRefused,
                    "ConnectionAborted" => ErrorReason::ConnectionAborted,
                    "ConnectionFailed" => ErrorReason::ConnectionFailed,
                    "NameNotResolved" => ErrorReason::NameNotResolved,
                    "InternetDisconnected" => ErrorReason::InternetDisconnected,
                    "AddressUnreachable" => ErrorReason::AddressUnreachable,
                    "BlockedByClient" => ErrorReason::BlockedByClient,
                    "BlockedByResponse" => ErrorReason::BlockedByResponse,
                    _ => ErrorReason::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ErrorReason::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.ExemptedSetCookieWithReason。
///
///协议原文：
///
///<pre>A cookie should have been blocked by 3PCD but is exempted and stored from a response with the
///corresponding reason. A cookie could only have at most one exemption reason.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ExemptedSetCookieWithReason {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ExemptedSetCookieWithReason/properties/exemptionReason。
    ///
    ///协议原文：
    ///
    ///<pre>The reason the cookie was exempted.</pre>
    ///
    ///
    #[serde(rename = "exemptionReason")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub exemption_reason: crate::network::types::CookieExemptionReason,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ExemptedSetCookieWithReason/properties/cookieLine。
    ///
    ///协议原文：
    ///
    ///<pre>The string representing this individual cookie as it would appear in the header.</pre>
    ///
    ///
    #[serde(rename = "cookieLine")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cookie_line: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ExemptedSetCookieWithReason/properties/cookie。
    ///
    ///协议原文：
    ///
    ///<pre>The cookie object representing the cookie.</pre>
    ///
    ///
    #[serde(rename = "cookie")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cookie: crate::network::types::Cookie,
}
impl<'de> serde::Deserialize<'de> for ExemptedSetCookieWithReason {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ExemptedSetCookieWithReason;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ExemptedSetCookieWithReason/properties/exemptionReason。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The reason the cookie was exempted.</pre>
                    ///
                    ///
                    #[serde(rename = "exemptionReason")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub exemption_reason: crate::network::types::CookieExemptionReason,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ExemptedSetCookieWithReason/properties/cookieLine。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The string representing this individual cookie as it would appear in the header.</pre>
                    ///
                    ///
                    #[serde(rename = "cookieLine")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cookie_line: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ExemptedSetCookieWithReason/properties/cookie。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The cookie object representing the cookie.</pre>
                    ///
                    ///
                    #[serde(rename = "cookie")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cookie: crate::network::types::Cookie,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ExemptedSetCookieWithReason {
                    exemption_reason: fields.exemption_reason,
                    cookie_line: fields.cookie_line,
                    cookie: fields.cookie,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Network.type.Headers。
///
///协议原文：
///
///<pre>Request / response headers as keys / values of JSON object.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Headers(std::collections::BTreeMap<String, serde_json::Value>);
impl Headers {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::network::types::Headers = serde_json::from_str("{}").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::network::types::Headers::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("{}").unwrap());
    ///```
    pub fn new(value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::network::types::Headers = serde_json::from_str("{}").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::network::types::Headers::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("{}").unwrap());
    ///```
    pub fn into_inner(self) -> std::collections::BTreeMap<String, serde_json::Value> {
        self.0
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.IPAddressSpace。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum IpAddressSpace {
    ///协议值 "Loopback"。
    Loopback,
    ///协议值 "Local"。
    Local,
    ///协议值 "Public"。
    Public,
    ///协议值 "Unknown"。
    KnownUnknown,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl IpAddressSpace {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::IpAddressSpace::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Loopback => "Loopback",
            Self::Local => "Local",
            Self::Public => "Public",
            Self::KnownUnknown => "Unknown",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for IpAddressSpace {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for IpAddressSpace {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Loopback" => IpAddressSpace::Loopback,
            "Local" => IpAddressSpace::Local,
            "Public" => IpAddressSpace::Public,
            "Unknown" => IpAddressSpace::KnownUnknown,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for IpAddressSpace {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IpAddressSpace {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = IpAddressSpace;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Loopback" => IpAddressSpace::Loopback,
                    "Local" => IpAddressSpace::Local,
                    "Public" => IpAddressSpace::Public,
                    "Unknown" => IpAddressSpace::KnownUnknown,
                    _ => IpAddressSpace::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(IpAddressSpace::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.Initiator/properties/type。
///
///协议原文：
///
///<pre>Type of this initiator.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum InitiatorType {
    ///协议值 "parser"。
    Parser,
    ///协议值 "script"。
    Script,
    ///协议值 "preload"。
    Preload,
    ///协议值 "SignedExchange"。
    SignedExchange,
    ///协议值 "preflight"。
    Preflight,
    ///协议值 "FedCM"。
    FedCm,
    ///协议值 "other"。
    Other,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl InitiatorType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::InitiatorType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Parser => "parser",
            Self::Script => "script",
            Self::Preload => "preload",
            Self::SignedExchange => "SignedExchange",
            Self::Preflight => "preflight",
            Self::FedCm => "FedCM",
            Self::Other => "other",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for InitiatorType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for InitiatorType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "parser" => InitiatorType::Parser,
            "script" => InitiatorType::Script,
            "preload" => InitiatorType::Preload,
            "SignedExchange" => InitiatorType::SignedExchange,
            "preflight" => InitiatorType::Preflight,
            "FedCM" => InitiatorType::FedCm,
            "other" => InitiatorType::Other,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for InitiatorType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InitiatorType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = InitiatorType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "parser" => InitiatorType::Parser,
                    "script" => InitiatorType::Script,
                    "preload" => InitiatorType::Preload,
                    "SignedExchange" => InitiatorType::SignedExchange,
                    "preflight" => InitiatorType::Preflight,
                    "FedCM" => InitiatorType::FedCm,
                    "other" => InitiatorType::Other,
                    _ => InitiatorType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(InitiatorType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.Initiator。
///
///协议原文：
///
///<pre>Information about the request initiator.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct Initiator {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Initiator/properties/type。
    ///
    ///协议原文：
    ///
    ///<pre>Type of this initiator.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: InitiatorType,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Initiator/properties/stack。
    ///
    ///协议原文：
    ///
    ///<pre>Initiator JavaScript stack trace, set for Script only.
    ///Requires the Debugger domain to be enabled.</pre>
    ///
    ///
    #[serde(rename = "stack")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub stack: Option<crate::runtime::types::StackTrace>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Initiator/properties/url。
    ///
    ///协议原文：
    ///
    ///<pre>Initiator URL, set for Parser type or for Script type (when script is importing module) or for SignedExchange type.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Initiator/properties/lineNumber。
    ///
    ///协议原文：
    ///
    ///<pre>Initiator line number, set for Parser type or for Script type (when script is importing
    ///module) (0-based).</pre>
    ///
    ///
    #[serde(rename = "lineNumber")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub line_number: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Initiator/properties/columnNumber。
    ///
    ///协议原文：
    ///
    ///<pre>Initiator column number, set for Parser type or for Script type (when script is importing
    ///module) (0-based).</pre>
    ///
    ///
    #[serde(rename = "columnNumber")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub column_number: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Initiator/properties/requestId。
    ///
    ///协议原文：
    ///
    ///<pre>Set if another request triggered this request (e.g. preflight).</pre>
    ///
    ///
    #[serde(rename = "requestId")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub request_id: Option<crate::network::types::RequestId>,
}
impl<'de> serde::Deserialize<'de> for Initiator {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Initiator;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Initiator/properties/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Type of this initiator.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: InitiatorType,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Initiator/properties/stack。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Initiator JavaScript stack trace, set for Script only.
                    ///Requires the Debugger domain to be enabled.</pre>
                    ///
                    ///
                    #[serde(rename = "stack")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub stack: Option<crate::runtime::types::StackTrace>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Initiator/properties/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Initiator URL, set for Parser type or for Script type (when script is importing module) or for SignedExchange type.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub url: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Initiator/properties/lineNumber。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Initiator line number, set for Parser type or for Script type (when script is importing
                    ///module) (0-based).</pre>
                    ///
                    ///
                    #[serde(rename = "lineNumber")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub line_number: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Initiator/properties/columnNumber。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Initiator column number, set for Parser type or for Script type (when script is importing
                    ///module) (0-based).</pre>
                    ///
                    ///
                    #[serde(rename = "columnNumber")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub column_number: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Initiator/properties/requestId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Set if another request triggered this request (e.g. preflight).</pre>
                    ///
                    ///
                    #[serde(rename = "requestId")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub request_id: Option<crate::network::types::RequestId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Initiator {
                    r#type: fields.r#type,
                    stack: fields.stack,
                    url: fields.url,
                    line_number: fields.line_number,
                    column_number: fields.column_number,
                    request_id: fields.request_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.LoadNetworkResourceOptions。
///
///协议原文：
///
///<pre>An options object that may be extended later to better support CORS,
///CORB and streaming.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct LoadNetworkResourceOptions {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.LoadNetworkResourceOptions/properties/disableCache。
    #[serde(rename = "disableCache")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub disable_cache: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.LoadNetworkResourceOptions/properties/includeCredentials。
    #[serde(rename = "includeCredentials")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub include_credentials: bool,
}
impl<'de> serde::Deserialize<'de> for LoadNetworkResourceOptions {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LoadNetworkResourceOptions;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.LoadNetworkResourceOptions/properties/disableCache。
                    #[serde(rename = "disableCache")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub disable_cache: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.LoadNetworkResourceOptions/properties/includeCredentials。
                    #[serde(rename = "includeCredentials")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub include_credentials: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LoadNetworkResourceOptions {
                    disable_cache: fields.disable_cache,
                    include_credentials: fields.include_credentials,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.LoadNetworkResourcePageResult。
///
///协议原文：
///
///<pre>An object providing the result of a network resource load.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct LoadNetworkResourcePageResult {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.LoadNetworkResourcePageResult/properties/success。
    #[serde(rename = "success")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub success: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.LoadNetworkResourcePageResult/properties/netError。
    ///
    ///协议原文：
    ///
    ///<pre>Optional values used for error reporting.</pre>
    ///
    ///
    #[serde(rename = "netError")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub net_error: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.LoadNetworkResourcePageResult/properties/netErrorName。
    #[serde(rename = "netErrorName")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub net_error_name: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.LoadNetworkResourcePageResult/properties/httpStatusCode。
    #[serde(rename = "httpStatusCode")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub http_status_code: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.LoadNetworkResourcePageResult/properties/stream。
    ///
    ///协议原文：
    ///
    ///<pre>If successful, one of the following two fields holds the result.</pre>
    ///
    ///
    #[serde(rename = "stream")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub stream: Option<crate::io::types::StreamHandle>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.LoadNetworkResourcePageResult/properties/headers。
    ///
    ///协议原文：
    ///
    ///<pre>Response headers.</pre>
    ///
    ///
    #[serde(rename = "headers")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub headers: Option<crate::network::types::Headers>,
}
impl<'de> serde::Deserialize<'de> for LoadNetworkResourcePageResult {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LoadNetworkResourcePageResult;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.LoadNetworkResourcePageResult/properties/success。
                    #[serde(rename = "success")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub success: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.LoadNetworkResourcePageResult/properties/netError。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Optional values used for error reporting.</pre>
                    ///
                    ///
                    #[serde(rename = "netError")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub net_error: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.LoadNetworkResourcePageResult/properties/netErrorName。
                    #[serde(rename = "netErrorName")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub net_error_name: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.LoadNetworkResourcePageResult/properties/httpStatusCode。
                    #[serde(rename = "httpStatusCode")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub http_status_code: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.LoadNetworkResourcePageResult/properties/stream。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If successful, one of the following two fields holds the result.</pre>
                    ///
                    ///
                    #[serde(rename = "stream")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub stream: Option<crate::io::types::StreamHandle>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.LoadNetworkResourcePageResult/properties/headers。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Response headers.</pre>
                    ///
                    ///
                    #[serde(rename = "headers")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub headers: Option<crate::network::types::Headers>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(LoadNetworkResourcePageResult {
                    success: fields.success,
                    net_error: fields.net_error,
                    net_error_name: fields.net_error_name,
                    http_status_code: fields.http_status_code,
                    stream: fields.stream,
                    headers: fields.headers,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Network.type.LoaderId。
///
///协议原文：
///
///<pre>Unique loader identifier.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct LoaderId(String);
impl LoaderId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::network::types::LoaderId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::network::types::LoaderId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::network::types::LoaderId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::network::types::LoaderId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::network::types::LoaderId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.LocalNetworkAccessRequestPolicy。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum LocalNetworkAccessRequestPolicy {
    ///协议值 "Allow"。
    Allow,
    ///协议值 "BlockFromInsecureToMorePrivate"。
    BlockFromInsecureToMorePrivate,
    ///协议值 "WarnFromInsecureToMorePrivate"。
    WarnFromInsecureToMorePrivate,
    ///协议值 "PermissionBlock"。
    PermissionBlock,
    ///协议值 "PermissionWarn"。
    PermissionWarn,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl LocalNetworkAccessRequestPolicy {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::LocalNetworkAccessRequestPolicy::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Allow => "Allow",
            Self::BlockFromInsecureToMorePrivate => "BlockFromInsecureToMorePrivate",
            Self::WarnFromInsecureToMorePrivate => "WarnFromInsecureToMorePrivate",
            Self::PermissionBlock => "PermissionBlock",
            Self::PermissionWarn => "PermissionWarn",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for LocalNetworkAccessRequestPolicy {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for LocalNetworkAccessRequestPolicy {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Allow" => LocalNetworkAccessRequestPolicy::Allow,
            "BlockFromInsecureToMorePrivate" => {
                LocalNetworkAccessRequestPolicy::BlockFromInsecureToMorePrivate
            }
            "WarnFromInsecureToMorePrivate" => {
                LocalNetworkAccessRequestPolicy::WarnFromInsecureToMorePrivate
            }
            "PermissionBlock" => LocalNetworkAccessRequestPolicy::PermissionBlock,
            "PermissionWarn" => LocalNetworkAccessRequestPolicy::PermissionWarn,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for LocalNetworkAccessRequestPolicy {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LocalNetworkAccessRequestPolicy {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LocalNetworkAccessRequestPolicy;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Allow" => LocalNetworkAccessRequestPolicy::Allow,
                    "BlockFromInsecureToMorePrivate" => {
                        LocalNetworkAccessRequestPolicy::BlockFromInsecureToMorePrivate
                    }
                    "WarnFromInsecureToMorePrivate" => {
                        LocalNetworkAccessRequestPolicy::WarnFromInsecureToMorePrivate
                    }
                    "PermissionBlock" => LocalNetworkAccessRequestPolicy::PermissionBlock,
                    "PermissionWarn" => LocalNetworkAccessRequestPolicy::PermissionWarn,
                    _ => LocalNetworkAccessRequestPolicy::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(LocalNetworkAccessRequestPolicy::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Network.type.MonotonicTime。
///
///协议原文：
///
///<pre>Monotonically increasing time in seconds since an arbitrary point in the past.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct MonotonicTime(ydp_core::FiniteF64);
impl MonotonicTime {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::network::types::MonotonicTime = serde_json::from_str("0.0").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::network::types::MonotonicTime::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("0.0").unwrap());
    ///```
    pub fn new(value: ydp_core::FiniteF64) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::network::types::MonotonicTime = serde_json::from_str("0.0").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::network::types::MonotonicTime::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("0.0").unwrap());
    ///```
    pub fn into_inner(self) -> ydp_core::FiniteF64 {
        self.0
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.NetworkConditions。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct NetworkConditions {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.NetworkConditions/properties/urlPattern。
    ///
    ///协议原文：
    ///
    ///<pre>Only matching requests will be affected by these conditions. Patterns use the URLPattern constructor string
    ///syntax (https://urlpattern.spec.whatwg.org/) and must be absolute. If the pattern is empty, all requests are
    ///matched (including p2p connections).</pre>
    ///
    ///
    #[serde(rename = "urlPattern")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url_pattern: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.NetworkConditions/properties/latency。
    ///
    ///协议原文：
    ///
    ///<pre>Minimum latency from request sent to response headers received (ms).</pre>
    ///
    ///
    #[serde(rename = "latency")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub latency: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.NetworkConditions/properties/downloadThroughput。
    ///
    ///协议原文：
    ///
    ///<pre>Maximal aggregated download throughput (bytes/sec). -1 disables download throttling.</pre>
    ///
    ///
    #[serde(rename = "downloadThroughput")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub download_throughput: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.NetworkConditions/properties/uploadThroughput。
    ///
    ///协议原文：
    ///
    ///<pre>Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling.</pre>
    ///
    ///
    #[serde(rename = "uploadThroughput")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub upload_throughput: ydp_core::FiniteF64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.NetworkConditions/properties/connectionType。
    ///
    ///协议原文：
    ///
    ///<pre>Connection type if known.</pre>
    ///
    ///
    #[serde(rename = "connectionType")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub connection_type: Option<crate::network::types::ConnectionType>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.NetworkConditions/properties/packetLoss。
    ///
    ///协议原文：
    ///
    ///<pre>WebRTC packet loss (percent, 0-100). 0 disables packet loss emulation, 100 drops all the packets.</pre>
    ///
    ///
    #[serde(rename = "packetLoss")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub packet_loss: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.NetworkConditions/properties/packetQueueLength。
    ///
    ///协议原文：
    ///
    ///<pre>WebRTC packet queue length (packet). 0 removes any queue length limitations.</pre>
    ///
    ///
    #[serde(rename = "packetQueueLength")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub packet_queue_length: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.NetworkConditions/properties/packetReordering。
    ///
    ///协议原文：
    ///
    ///<pre>WebRTC packetReordering feature.</pre>
    ///
    ///
    #[serde(rename = "packetReordering")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub packet_reordering: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.NetworkConditions/properties/offline。
    ///
    ///协议原文：
    ///
    ///<pre>True to emulate internet disconnection.</pre>
    ///
    ///
    #[serde(rename = "offline")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub offline: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for NetworkConditions {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = NetworkConditions;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.NetworkConditions/properties/urlPattern。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Only matching requests will be affected by these conditions. Patterns use the URLPattern constructor string
                    ///syntax (https://urlpattern.spec.whatwg.org/) and must be absolute. If the pattern is empty, all requests are
                    ///matched (including p2p connections).</pre>
                    ///
                    ///
                    #[serde(rename = "urlPattern")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url_pattern: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.NetworkConditions/properties/latency。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Minimum latency from request sent to response headers received (ms).</pre>
                    ///
                    ///
                    #[serde(rename = "latency")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub latency: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.NetworkConditions/properties/downloadThroughput。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Maximal aggregated download throughput (bytes/sec). -1 disables download throttling.</pre>
                    ///
                    ///
                    #[serde(rename = "downloadThroughput")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub download_throughput: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.NetworkConditions/properties/uploadThroughput。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Maximal aggregated upload throughput (bytes/sec).  -1 disables upload throttling.</pre>
                    ///
                    ///
                    #[serde(rename = "uploadThroughput")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub upload_throughput: ydp_core::FiniteF64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.NetworkConditions/properties/connectionType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Connection type if known.</pre>
                    ///
                    ///
                    #[serde(rename = "connectionType")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub connection_type: Option<crate::network::types::ConnectionType>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.NetworkConditions/properties/packetLoss。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>WebRTC packet loss (percent, 0-100). 0 disables packet loss emulation, 100 drops all the packets.</pre>
                    ///
                    ///
                    #[serde(rename = "packetLoss")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub packet_loss: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.NetworkConditions/properties/packetQueueLength。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>WebRTC packet queue length (packet). 0 removes any queue length limitations.</pre>
                    ///
                    ///
                    #[serde(rename = "packetQueueLength")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub packet_queue_length: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.NetworkConditions/properties/packetReordering。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>WebRTC packetReordering feature.</pre>
                    ///
                    ///
                    #[serde(rename = "packetReordering")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub packet_reordering: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.NetworkConditions/properties/offline。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True to emulate internet disconnection.</pre>
                    ///
                    ///
                    #[serde(rename = "offline")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub offline: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(NetworkConditions {
                    url_pattern: fields.url_pattern,
                    latency: fields.latency,
                    download_throughput: fields.download_throughput,
                    upload_throughput: fields.upload_throughput,
                    connection_type: fields.connection_type,
                    packet_loss: fields.packet_loss,
                    packet_queue_length: fields.packet_queue_length,
                    packet_reordering: fields.packet_reordering,
                    offline: fields.offline,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.PostDataEntry。
///
///协议原文：
///
///<pre>Post data entry for HTTP request</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct PostDataEntry {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.PostDataEntry/properties/bytes。
    #[serde(rename = "bytes")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub bytes: Option<String>,
}
impl<'de> serde::Deserialize<'de> for PostDataEntry {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PostDataEntry;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.PostDataEntry/properties/bytes。
                    #[serde(rename = "bytes")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub bytes: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(PostDataEntry {
                    bytes: fields.bytes,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.RefreshEventDetails/properties/refreshResult。
///
///协议原文：
///
///<pre>The result of a refresh.
///LINT.IfChange(DeviceBoundSessionRefreshResult)</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RefreshEventDetailsRefreshResult {
    ///协议值 "Refreshed"。
    Refreshed,
    ///协议值 "InitializedService"。
    InitializedService,
    ///协议值 "Unreachable"。
    Unreachable,
    ///协议值 "ServerError"。
    ServerError,
    ///协议值 "FatalError"。
    FatalError,
    ///协议值 "SigningQuotaExceeded"。
    SigningQuotaExceeded,
    ///协议值 "RefreshedAsWaiter"。
    RefreshedAsWaiter,
    ///协议值 "TransientSigningError"。
    TransientSigningError,
    ///协议值 "InScopeRefreshNotYetNeeded"。
    InScopeRefreshNotYetNeeded,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl RefreshEventDetailsRefreshResult {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::RefreshEventDetailsRefreshResult::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Refreshed => "Refreshed",
            Self::InitializedService => "InitializedService",
            Self::Unreachable => "Unreachable",
            Self::ServerError => "ServerError",
            Self::FatalError => "FatalError",
            Self::SigningQuotaExceeded => "SigningQuotaExceeded",
            Self::RefreshedAsWaiter => "RefreshedAsWaiter",
            Self::TransientSigningError => "TransientSigningError",
            Self::InScopeRefreshNotYetNeeded => "InScopeRefreshNotYetNeeded",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for RefreshEventDetailsRefreshResult {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for RefreshEventDetailsRefreshResult {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Refreshed" => RefreshEventDetailsRefreshResult::Refreshed,
            "InitializedService" => RefreshEventDetailsRefreshResult::InitializedService,
            "Unreachable" => RefreshEventDetailsRefreshResult::Unreachable,
            "ServerError" => RefreshEventDetailsRefreshResult::ServerError,
            "FatalError" => RefreshEventDetailsRefreshResult::FatalError,
            "SigningQuotaExceeded" => RefreshEventDetailsRefreshResult::SigningQuotaExceeded,
            "RefreshedAsWaiter" => RefreshEventDetailsRefreshResult::RefreshedAsWaiter,
            "TransientSigningError" => RefreshEventDetailsRefreshResult::TransientSigningError,
            "InScopeRefreshNotYetNeeded" => {
                RefreshEventDetailsRefreshResult::InScopeRefreshNotYetNeeded
            }
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for RefreshEventDetailsRefreshResult {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RefreshEventDetailsRefreshResult {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RefreshEventDetailsRefreshResult;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Refreshed" => RefreshEventDetailsRefreshResult::Refreshed,
                    "InitializedService" => RefreshEventDetailsRefreshResult::InitializedService,
                    "Unreachable" => RefreshEventDetailsRefreshResult::Unreachable,
                    "ServerError" => RefreshEventDetailsRefreshResult::ServerError,
                    "FatalError" => RefreshEventDetailsRefreshResult::FatalError,
                    "SigningQuotaExceeded" => {
                        RefreshEventDetailsRefreshResult::SigningQuotaExceeded
                    }
                    "RefreshedAsWaiter" => RefreshEventDetailsRefreshResult::RefreshedAsWaiter,
                    "TransientSigningError" => {
                        RefreshEventDetailsRefreshResult::TransientSigningError
                    }
                    "InScopeRefreshNotYetNeeded" => {
                        RefreshEventDetailsRefreshResult::InScopeRefreshNotYetNeeded
                    }
                    _ => RefreshEventDetailsRefreshResult::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(RefreshEventDetailsRefreshResult::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.RefreshEventDetails。
///
///协议原文：
///
///<pre>Session event details specific to refresh.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RefreshEventDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.RefreshEventDetails/properties/refreshResult。
    ///
    ///协议原文：
    ///
    ///<pre>The result of a refresh.
    ///LINT.IfChange(DeviceBoundSessionRefreshResult)</pre>
    ///
    ///
    #[serde(rename = "refreshResult")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub refresh_result: RefreshEventDetailsRefreshResult,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.RefreshEventDetails/properties/fetchResult。
    ///
    ///协议原文：
    ///
    ///<pre>LINT.ThenChange(//net/device_bound_sessions/refresh_result.h:DeviceBoundSessionRefreshResult,//content/browser/devtools/protocol/network_handler.cc:DeviceBoundSessionRefreshResult)
    ///If there was a fetch attempt, the result of that.</pre>
    ///
    ///
    #[serde(rename = "fetchResult")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub fetch_result: Option<crate::network::types::DeviceBoundSessionFetchResult>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.RefreshEventDetails/properties/newSession。
    ///
    ///协议原文：
    ///
    ///<pre>The session display if there was a newly created session. This is populated
    ///for any refresh event that modifies the session config.</pre>
    ///
    ///
    #[serde(rename = "newSession")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub new_session: Option<crate::network::types::DeviceBoundSession>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.RefreshEventDetails/properties/wasFullyProactiveRefresh。
    ///
    ///协议原文：
    ///
    ///<pre>See comments on `net::device_bound_sessions::RefreshEventResult::was_fully_proactive_refresh`.</pre>
    ///
    ///
    #[serde(rename = "wasFullyProactiveRefresh")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub was_fully_proactive_refresh: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.RefreshEventDetails/properties/failedRequest。
    ///
    ///协议原文：
    ///
    ///<pre>Details about a failed device bound session network request if there was
    ///one.</pre>
    ///
    ///
    #[serde(rename = "failedRequest")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub failed_request: Option<crate::network::types::DeviceBoundSessionFailedRequest>,
}
impl<'de> serde::Deserialize<'de> for RefreshEventDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RefreshEventDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.RefreshEventDetails/properties/refreshResult。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The result of a refresh.
                    ///LINT.IfChange(DeviceBoundSessionRefreshResult)</pre>
                    ///
                    ///
                    #[serde(rename = "refreshResult")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub refresh_result: RefreshEventDetailsRefreshResult,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.RefreshEventDetails/properties/fetchResult。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>LINT.ThenChange(//net/device_bound_sessions/refresh_result.h:DeviceBoundSessionRefreshResult,//content/browser/devtools/protocol/network_handler.cc:DeviceBoundSessionRefreshResult)
                    ///If there was a fetch attempt, the result of that.</pre>
                    ///
                    ///
                    #[serde(rename = "fetchResult")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub fetch_result: Option<crate::network::types::DeviceBoundSessionFetchResult>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.RefreshEventDetails/properties/newSession。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The session display if there was a newly created session. This is populated
                    ///for any refresh event that modifies the session config.</pre>
                    ///
                    ///
                    #[serde(rename = "newSession")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub new_session: Option<crate::network::types::DeviceBoundSession>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.RefreshEventDetails/properties/wasFullyProactiveRefresh。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>See comments on `net::device_bound_sessions::RefreshEventResult::was_fully_proactive_refresh`.</pre>
                    ///
                    ///
                    #[serde(rename = "wasFullyProactiveRefresh")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub was_fully_proactive_refresh: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.RefreshEventDetails/properties/failedRequest。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Details about a failed device bound session network request if there was
                    ///one.</pre>
                    ///
                    ///
                    #[serde(rename = "failedRequest")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub failed_request:
                        Option<crate::network::types::DeviceBoundSessionFailedRequest>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RefreshEventDetails {
                    refresh_result: fields.refresh_result,
                    fetch_result: fields.fetch_result,
                    new_session: fields.new_session,
                    was_fully_proactive_refresh: fields.was_fully_proactive_refresh,
                    failed_request: fields.failed_request,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.RenderBlockingBehavior。
///
///协议原文：
///
///<pre>The render-blocking behavior of a resource request.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RenderBlockingBehavior {
    ///协议值 "Blocking"。
    Blocking,
    ///协议值 "InBodyParserBlocking"。
    InBodyParserBlocking,
    ///协议值 "NonBlocking"。
    NonBlocking,
    ///协议值 "NonBlockingDynamic"。
    NonBlockingDynamic,
    ///协议值 "PotentiallyBlocking"。
    PotentiallyBlocking,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl RenderBlockingBehavior {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::RenderBlockingBehavior::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Blocking => "Blocking",
            Self::InBodyParserBlocking => "InBodyParserBlocking",
            Self::NonBlocking => "NonBlocking",
            Self::NonBlockingDynamic => "NonBlockingDynamic",
            Self::PotentiallyBlocking => "PotentiallyBlocking",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for RenderBlockingBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for RenderBlockingBehavior {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Blocking" => RenderBlockingBehavior::Blocking,
            "InBodyParserBlocking" => RenderBlockingBehavior::InBodyParserBlocking,
            "NonBlocking" => RenderBlockingBehavior::NonBlocking,
            "NonBlockingDynamic" => RenderBlockingBehavior::NonBlockingDynamic,
            "PotentiallyBlocking" => RenderBlockingBehavior::PotentiallyBlocking,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for RenderBlockingBehavior {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RenderBlockingBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RenderBlockingBehavior;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Blocking" => RenderBlockingBehavior::Blocking,
                    "InBodyParserBlocking" => RenderBlockingBehavior::InBodyParserBlocking,
                    "NonBlocking" => RenderBlockingBehavior::NonBlocking,
                    "NonBlockingDynamic" => RenderBlockingBehavior::NonBlockingDynamic,
                    "PotentiallyBlocking" => RenderBlockingBehavior::PotentiallyBlocking,
                    _ => RenderBlockingBehavior::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(RenderBlockingBehavior::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Network.type.ReportId。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ReportId(String);
impl ReportId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::network::types::ReportId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::network::types::ReportId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::network::types::ReportId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::network::types::ReportId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::network::types::ReportId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.ReportStatus。
///
///协议原文：
///
///<pre>The status of a Reporting API report.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ReportStatus {
    ///协议值 "Queued"。
    Queued,
    ///协议值 "Pending"。
    Pending,
    ///协议值 "MarkedForRemoval"。
    MarkedForRemoval,
    ///协议值 "Success"。
    Success,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ReportStatus {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::ReportStatus::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Queued => "Queued",
            Self::Pending => "Pending",
            Self::MarkedForRemoval => "MarkedForRemoval",
            Self::Success => "Success",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ReportStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ReportStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Queued" => ReportStatus::Queued,
            "Pending" => ReportStatus::Pending,
            "MarkedForRemoval" => ReportStatus::MarkedForRemoval,
            "Success" => ReportStatus::Success,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ReportStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReportStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReportStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Queued" => ReportStatus::Queued,
                    "Pending" => ReportStatus::Pending,
                    "MarkedForRemoval" => ReportStatus::MarkedForRemoval,
                    "Success" => ReportStatus::Success,
                    _ => ReportStatus::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ReportStatus::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.ReportingApiEndpoint。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReportingApiEndpoint {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ReportingApiEndpoint/properties/url。
    ///
    ///协议原文：
    ///
    ///<pre>The URL of the endpoint to which reports may be delivered.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ReportingApiEndpoint/properties/groupName。
    ///
    ///协议原文：
    ///
    ///<pre>Name of the endpoint group.</pre>
    ///
    ///
    #[serde(rename = "groupName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub group_name: String,
}
impl<'de> serde::Deserialize<'de> for ReportingApiEndpoint {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReportingApiEndpoint;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ReportingApiEndpoint/properties/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The URL of the endpoint to which reports may be delivered.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ReportingApiEndpoint/properties/groupName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Name of the endpoint group.</pre>
                    ///
                    ///
                    #[serde(rename = "groupName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub group_name: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReportingApiEndpoint {
                    url: fields.url,
                    group_name: fields.group_name,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.ReportingApiReport。
///
///协议原文：
///
///<pre>An object representing a report generated by the Reporting API.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ReportingApiReport {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ReportingApiReport/properties/id。
    #[serde(rename = "id")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub id: crate::network::types::ReportId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ReportingApiReport/properties/initiatorUrl。
    ///
    ///协议原文：
    ///
    ///<pre>The URL of the document that triggered the report.</pre>
    ///
    ///
    #[serde(rename = "initiatorUrl")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub initiator_url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ReportingApiReport/properties/destination。
    ///
    ///协议原文：
    ///
    ///<pre>The name of the endpoint group that should be used to deliver the report.</pre>
    ///
    ///
    #[serde(rename = "destination")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub destination: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ReportingApiReport/properties/type。
    ///
    ///协议原文：
    ///
    ///<pre>The type of the report (specifies the set of data that is contained in the report body).</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ReportingApiReport/properties/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>When the report was generated.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: crate::network::types::TimeSinceEpoch,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ReportingApiReport/properties/depth。
    ///
    ///协议原文：
    ///
    ///<pre>How many uploads deep the related request was.</pre>
    ///
    ///
    #[serde(rename = "depth")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub depth: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ReportingApiReport/properties/completedAttempts。
    ///
    ///协议原文：
    ///
    ///<pre>The number of delivery attempts made so far, not including an active attempt.</pre>
    ///
    ///
    #[serde(rename = "completedAttempts")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub completed_attempts: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ReportingApiReport/properties/body。
    #[serde(rename = "body")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub body: std::collections::BTreeMap<String, serde_json::Value>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ReportingApiReport/properties/status。
    #[serde(rename = "status")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub status: crate::network::types::ReportStatus,
}
impl<'de> serde::Deserialize<'de> for ReportingApiReport {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ReportingApiReport;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ReportingApiReport/properties/id。
                    #[serde(rename = "id")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub id: crate::network::types::ReportId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ReportingApiReport/properties/initiatorUrl。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The URL of the document that triggered the report.</pre>
                    ///
                    ///
                    #[serde(rename = "initiatorUrl")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub initiator_url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ReportingApiReport/properties/destination。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The name of the endpoint group that should be used to deliver the report.</pre>
                    ///
                    ///
                    #[serde(rename = "destination")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub destination: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ReportingApiReport/properties/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The type of the report (specifies the set of data that is contained in the report body).</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ReportingApiReport/properties/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>When the report was generated.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: crate::network::types::TimeSinceEpoch,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ReportingApiReport/properties/depth。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>How many uploads deep the related request was.</pre>
                    ///
                    ///
                    #[serde(rename = "depth")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub depth: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ReportingApiReport/properties/completedAttempts。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The number of delivery attempts made so far, not including an active attempt.</pre>
                    ///
                    ///
                    #[serde(rename = "completedAttempts")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub completed_attempts: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ReportingApiReport/properties/body。
                    #[serde(rename = "body")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub body: std::collections::BTreeMap<String, serde_json::Value>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ReportingApiReport/properties/status。
                    #[serde(rename = "status")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub status: crate::network::types::ReportStatus,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ReportingApiReport {
                    id: fields.id,
                    initiator_url: fields.initiator_url,
                    destination: fields.destination,
                    r#type: fields.r#type,
                    timestamp: fields.timestamp,
                    depth: fields.depth,
                    completed_attempts: fields.completed_attempts,
                    body: fields.body,
                    status: fields.status,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.Request/properties/referrerPolicy。
///
///协议原文：
///
///<pre>The referrer policy of the request, as defined in https://www.w3.org/TR/referrer-policy/</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RequestReferrerPolicy {
    ///协议值 "unsafe-url"。
    UnsafeUrl,
    ///协议值 "no-referrer-when-downgrade"。
    NoReferrerWhenDowngrade,
    ///协议值 "no-referrer"。
    NoReferrer,
    ///协议值 "origin"。
    Origin,
    ///协议值 "origin-when-cross-origin"。
    OriginWhenCrossOrigin,
    ///协议值 "same-origin"。
    SameOrigin,
    ///协议值 "strict-origin"。
    StrictOrigin,
    ///协议值 "strict-origin-when-cross-origin"。
    StrictOriginWhenCrossOrigin,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl RequestReferrerPolicy {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::RequestReferrerPolicy::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::UnsafeUrl => "unsafe-url",
            Self::NoReferrerWhenDowngrade => "no-referrer-when-downgrade",
            Self::NoReferrer => "no-referrer",
            Self::Origin => "origin",
            Self::OriginWhenCrossOrigin => "origin-when-cross-origin",
            Self::SameOrigin => "same-origin",
            Self::StrictOrigin => "strict-origin",
            Self::StrictOriginWhenCrossOrigin => "strict-origin-when-cross-origin",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for RequestReferrerPolicy {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for RequestReferrerPolicy {
    fn from(value: String) -> Self {
        match value.as_str() {
            "unsafe-url" => RequestReferrerPolicy::UnsafeUrl,
            "no-referrer-when-downgrade" => RequestReferrerPolicy::NoReferrerWhenDowngrade,
            "no-referrer" => RequestReferrerPolicy::NoReferrer,
            "origin" => RequestReferrerPolicy::Origin,
            "origin-when-cross-origin" => RequestReferrerPolicy::OriginWhenCrossOrigin,
            "same-origin" => RequestReferrerPolicy::SameOrigin,
            "strict-origin" => RequestReferrerPolicy::StrictOrigin,
            "strict-origin-when-cross-origin" => RequestReferrerPolicy::StrictOriginWhenCrossOrigin,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for RequestReferrerPolicy {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RequestReferrerPolicy {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RequestReferrerPolicy;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "unsafe-url" => RequestReferrerPolicy::UnsafeUrl,
                    "no-referrer-when-downgrade" => RequestReferrerPolicy::NoReferrerWhenDowngrade,
                    "no-referrer" => RequestReferrerPolicy::NoReferrer,
                    "origin" => RequestReferrerPolicy::Origin,
                    "origin-when-cross-origin" => RequestReferrerPolicy::OriginWhenCrossOrigin,
                    "same-origin" => RequestReferrerPolicy::SameOrigin,
                    "strict-origin" => RequestReferrerPolicy::StrictOrigin,
                    "strict-origin-when-cross-origin" => {
                        RequestReferrerPolicy::StrictOriginWhenCrossOrigin
                    }
                    _ => RequestReferrerPolicy::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(RequestReferrerPolicy::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.Request。
///
///协议原文：
///
///<pre>HTTP request data.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct Request {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Request/properties/url。
    ///
    ///协议原文：
    ///
    ///<pre>Request URL (without fragment).</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Request/properties/urlFragment。
    ///
    ///协议原文：
    ///
    ///<pre>Fragment of the requested URL starting with hash, if present.</pre>
    ///
    ///
    #[serde(rename = "urlFragment")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub url_fragment: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Request/properties/method。
    ///
    ///协议原文：
    ///
    ///<pre>HTTP request method.</pre>
    ///
    ///
    #[serde(rename = "method")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub method: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Request/properties/headers。
    ///
    ///协议原文：
    ///
    ///<pre>HTTP request headers.</pre>
    ///
    ///
    #[serde(rename = "headers")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub headers: crate::network::types::Headers,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Request/properties/postData。
    ///
    ///协议原文：
    ///
    ///<pre>HTTP POST request data.
    ///Use postDataEntries instead.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "postData")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub post_data: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Request/properties/hasPostData。
    ///
    ///协议原文：
    ///
    ///<pre>True when the request has POST data. Note that postData might still be omitted when this flag is true when the data is too long.</pre>
    ///
    ///
    #[serde(rename = "hasPostData")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_post_data: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Request/properties/postDataEntries。
    ///
    ///协议原文：
    ///
    ///<pre>Request body elements (post data broken into individual entries).</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "postDataEntries")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub post_data_entries: Option<Vec<crate::network::types::PostDataEntry>>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Request/properties/mixedContentType。
    ///
    ///协议原文：
    ///
    ///<pre>The mixed content type of the request.</pre>
    ///
    ///
    #[serde(rename = "mixedContentType")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub mixed_content_type: Option<crate::security::types::MixedContentType>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Request/properties/initialPriority。
    ///
    ///协议原文：
    ///
    ///<pre>Priority of the resource request at the time request is sent.</pre>
    ///
    ///
    #[serde(rename = "initialPriority")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub initial_priority: crate::network::types::ResourcePriority,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Request/properties/referrerPolicy。
    ///
    ///协议原文：
    ///
    ///<pre>The referrer policy of the request, as defined in https://www.w3.org/TR/referrer-policy/</pre>
    ///
    ///
    #[serde(rename = "referrerPolicy")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub referrer_policy: RequestReferrerPolicy,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Request/properties/isLinkPreload。
    ///
    ///协议原文：
    ///
    ///<pre>Whether is loaded via link preload.</pre>
    ///
    ///
    #[serde(rename = "isLinkPreload")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_link_preload: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Request/properties/trustTokenParams。
    ///
    ///协议原文：
    ///
    ///<pre>Set for requests when the TrustToken API is used. Contains the parameters
    ///passed by the developer (e.g. via "fetch") as understood by the backend.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "trustTokenParams")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub trust_token_params: Option<crate::network::types::TrustTokenParams>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Request/properties/isSameSite。
    ///
    ///协议原文：
    ///
    ///<pre>True if this resource request is considered to be the 'same site' as the
    ///request corresponding to the main frame.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "isSameSite")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_same_site: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Request/properties/isAdRelated。
    ///
    ///协议原文：
    ///
    ///<pre>True when the resource request is ad-related.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "isAdRelated")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_ad_related: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for Request {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Request;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Request/properties/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request URL (without fragment).</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Request/properties/urlFragment。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Fragment of the requested URL starting with hash, if present.</pre>
                    ///
                    ///
                    #[serde(rename = "urlFragment")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub url_fragment: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Request/properties/method。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>HTTP request method.</pre>
                    ///
                    ///
                    #[serde(rename = "method")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub method: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Request/properties/headers。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>HTTP request headers.</pre>
                    ///
                    ///
                    #[serde(rename = "headers")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub headers: crate::network::types::Headers,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Request/properties/postData。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>HTTP POST request data.
                    ///Use postDataEntries instead.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "postData")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub post_data: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Request/properties/hasPostData。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True when the request has POST data. Note that postData might still be omitted when this flag is true when the data is too long.</pre>
                    ///
                    ///
                    #[serde(rename = "hasPostData")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub has_post_data: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Request/properties/postDataEntries。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Request body elements (post data broken into individual entries).</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "postDataEntries")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub post_data_entries: Option<Vec<crate::network::types::PostDataEntry>>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Request/properties/mixedContentType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The mixed content type of the request.</pre>
                    ///
                    ///
                    #[serde(rename = "mixedContentType")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub mixed_content_type: Option<crate::security::types::MixedContentType>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Request/properties/initialPriority。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Priority of the resource request at the time request is sent.</pre>
                    ///
                    ///
                    #[serde(rename = "initialPriority")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub initial_priority: crate::network::types::ResourcePriority,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Request/properties/referrerPolicy。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The referrer policy of the request, as defined in https://www.w3.org/TR/referrer-policy/</pre>
                    ///
                    ///
                    #[serde(rename = "referrerPolicy")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub referrer_policy: RequestReferrerPolicy,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Request/properties/isLinkPreload。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether is loaded via link preload.</pre>
                    ///
                    ///
                    #[serde(rename = "isLinkPreload")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub is_link_preload: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Request/properties/trustTokenParams。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Set for requests when the TrustToken API is used. Contains the parameters
                    ///passed by the developer (e.g. via "fetch") as understood by the backend.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "trustTokenParams")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub trust_token_params: Option<crate::network::types::TrustTokenParams>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Request/properties/isSameSite。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if this resource request is considered to be the 'same site' as the
                    ///request corresponding to the main frame.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "isSameSite")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub is_same_site: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Request/properties/isAdRelated。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True when the resource request is ad-related.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "isAdRelated")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub is_ad_related: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Request {
                    url: fields.url,
                    url_fragment: fields.url_fragment,
                    method: fields.method,
                    headers: fields.headers,
                    post_data: fields.post_data,
                    has_post_data: fields.has_post_data,
                    post_data_entries: fields.post_data_entries,
                    mixed_content_type: fields.mixed_content_type,
                    initial_priority: fields.initial_priority,
                    referrer_policy: fields.referrer_policy,
                    is_link_preload: fields.is_link_preload,
                    trust_token_params: fields.trust_token_params,
                    is_same_site: fields.is_same_site,
                    is_ad_related: fields.is_ad_related,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Network.type.RequestId。
///
///协议原文：
///
///<pre>Unique network request identifier.
///Note that this does not identify individual HTTP requests that are part of
///a network request.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct RequestId(String);
impl RequestId {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::network::types::RequestId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::network::types::RequestId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn new(value: String) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::network::types::RequestId = serde_json::from_str("\"\"").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::network::types::RequestId::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("\"\"").unwrap());
    ///```
    pub fn into_inner(self) -> String {
        self.0
    }
    ///借用字符串；返回值不长于包装的生命周期。
    ///
    ///```
    ///let value = ydp::network::types::RequestId::new(String::from("示例"));
    ///assert_eq!(value.as_str(), "示例");
    ///```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.ResourcePriority。
///
///协议原文：
///
///<pre>Loading priority of a resource request.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ResourcePriority {
    ///协议值 "VeryLow"。
    VeryLow,
    ///协议值 "Low"。
    Low,
    ///协议值 "Medium"。
    Medium,
    ///协议值 "High"。
    High,
    ///协议值 "VeryHigh"。
    VeryHigh,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ResourcePriority {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::ResourcePriority::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::VeryLow => "VeryLow",
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::VeryHigh => "VeryHigh",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ResourcePriority {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ResourcePriority {
    fn from(value: String) -> Self {
        match value.as_str() {
            "VeryLow" => ResourcePriority::VeryLow,
            "Low" => ResourcePriority::Low,
            "Medium" => ResourcePriority::Medium,
            "High" => ResourcePriority::High,
            "VeryHigh" => ResourcePriority::VeryHigh,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ResourcePriority {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ResourcePriority {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResourcePriority;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "VeryLow" => ResourcePriority::VeryLow,
                    "Low" => ResourcePriority::Low,
                    "Medium" => ResourcePriority::Medium,
                    "High" => ResourcePriority::High,
                    "VeryHigh" => ResourcePriority::VeryHigh,
                    _ => ResourcePriority::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ResourcePriority::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.ResourceTiming。
///
///协议原文：
///
///<pre>Timing information for the request.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct ResourceTiming {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/requestTime。
    ///
    ///协议原文：
    ///
    ///<pre>Timing's requestTime is a baseline in seconds, while the other numbers are ticks in
    ///milliseconds relatively to this requestTime.</pre>
    ///
    ///
    #[serde(rename = "requestTime")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_time: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/proxyStart。
    ///
    ///协议原文：
    ///
    ///<pre>Started resolving proxy.</pre>
    ///
    ///
    #[serde(rename = "proxyStart")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub proxy_start: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/proxyEnd。
    ///
    ///协议原文：
    ///
    ///<pre>Finished resolving proxy.</pre>
    ///
    ///
    #[serde(rename = "proxyEnd")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub proxy_end: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/dnsStart。
    ///
    ///协议原文：
    ///
    ///<pre>Started DNS address resolve.</pre>
    ///
    ///
    #[serde(rename = "dnsStart")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub dns_start: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/dnsEnd。
    ///
    ///协议原文：
    ///
    ///<pre>Finished DNS address resolve.</pre>
    ///
    ///
    #[serde(rename = "dnsEnd")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub dns_end: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/connectStart。
    ///
    ///协议原文：
    ///
    ///<pre>Started connecting to the remote host.</pre>
    ///
    ///
    #[serde(rename = "connectStart")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub connect_start: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/connectEnd。
    ///
    ///协议原文：
    ///
    ///<pre>Connected to the remote host.</pre>
    ///
    ///
    #[serde(rename = "connectEnd")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub connect_end: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/sslStart。
    ///
    ///协议原文：
    ///
    ///<pre>Started SSL handshake.</pre>
    ///
    ///
    #[serde(rename = "sslStart")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub ssl_start: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/sslEnd。
    ///
    ///协议原文：
    ///
    ///<pre>Finished SSL handshake.</pre>
    ///
    ///
    #[serde(rename = "sslEnd")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub ssl_end: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/workerStart。
    ///
    ///协议原文：
    ///
    ///<pre>Started running ServiceWorker.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "workerStart")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub worker_start: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/workerReady。
    ///
    ///协议原文：
    ///
    ///<pre>Finished Starting ServiceWorker.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "workerReady")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub worker_ready: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/workerFetchStart。
    ///
    ///协议原文：
    ///
    ///<pre>Started fetch event.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "workerFetchStart")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub worker_fetch_start: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/workerRespondWithSettled。
    ///
    ///协议原文：
    ///
    ///<pre>Settled fetch event respondWith promise.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "workerRespondWithSettled")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub worker_respond_with_settled: ydp_core::FiniteF64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.ResourceTiming/properties/workerRouterEvaluationStart。
    ///
    ///协议原文：
    ///
    ///<pre>Started ServiceWorker static routing source evaluation.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "workerRouterEvaluationStart")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub worker_router_evaluation_start: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.ResourceTiming/properties/workerCacheLookupStart。
    ///
    ///协议原文：
    ///
    ///<pre>Started cache lookup when the source was evaluated to `cache`.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "workerCacheLookupStart")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub worker_cache_lookup_start: Option<ydp_core::FiniteF64>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/sendStart。
    ///
    ///协议原文：
    ///
    ///<pre>Started sending request.</pre>
    ///
    ///
    #[serde(rename = "sendStart")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub send_start: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/sendEnd。
    ///
    ///协议原文：
    ///
    ///<pre>Finished sending request.</pre>
    ///
    ///
    #[serde(rename = "sendEnd")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub send_end: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/pushStart。
    ///
    ///协议原文：
    ///
    ///<pre>Time the server started pushing request.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "pushStart")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub push_start: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/pushEnd。
    ///
    ///协议原文：
    ///
    ///<pre>Time the server finished pushing request.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "pushEnd")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub push_end: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/receiveHeadersStart。
    ///
    ///协议原文：
    ///
    ///<pre>Started receiving response headers.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "receiveHeadersStart")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub receive_headers_start: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/receiveHeadersEnd。
    ///
    ///协议原文：
    ///
    ///<pre>Finished receiving response headers.</pre>
    ///
    ///
    #[serde(rename = "receiveHeadersEnd")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub receive_headers_end: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for ResourceTiming {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResourceTiming;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/requestTime。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Timing's requestTime is a baseline in seconds, while the other numbers are ticks in
                    ///milliseconds relatively to this requestTime.</pre>
                    ///
                    ///
                    #[serde(rename = "requestTime")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_time: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/proxyStart。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Started resolving proxy.</pre>
                    ///
                    ///
                    #[serde(rename = "proxyStart")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub proxy_start: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/proxyEnd。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Finished resolving proxy.</pre>
                    ///
                    ///
                    #[serde(rename = "proxyEnd")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub proxy_end: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/dnsStart。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Started DNS address resolve.</pre>
                    ///
                    ///
                    #[serde(rename = "dnsStart")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub dns_start: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/dnsEnd。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Finished DNS address resolve.</pre>
                    ///
                    ///
                    #[serde(rename = "dnsEnd")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub dns_end: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/connectStart。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Started connecting to the remote host.</pre>
                    ///
                    ///
                    #[serde(rename = "connectStart")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub connect_start: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/connectEnd。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Connected to the remote host.</pre>
                    ///
                    ///
                    #[serde(rename = "connectEnd")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub connect_end: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/sslStart。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Started SSL handshake.</pre>
                    ///
                    ///
                    #[serde(rename = "sslStart")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub ssl_start: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/sslEnd。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Finished SSL handshake.</pre>
                    ///
                    ///
                    #[serde(rename = "sslEnd")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub ssl_end: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/workerStart。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Started running ServiceWorker.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "workerStart")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub worker_start: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/workerReady。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Finished Starting ServiceWorker.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "workerReady")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub worker_ready: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/workerFetchStart。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Started fetch event.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "workerFetchStart")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub worker_fetch_start: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/workerRespondWithSettled。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Settled fetch event respondWith promise.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "workerRespondWithSettled")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub worker_respond_with_settled: ydp_core::FiniteF64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.ResourceTiming/properties/workerRouterEvaluationStart。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Started ServiceWorker static routing source evaluation.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "workerRouterEvaluationStart")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub worker_router_evaluation_start: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.ResourceTiming/properties/workerCacheLookupStart。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Started cache lookup when the source was evaluated to `cache`.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "workerCacheLookupStart")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub worker_cache_lookup_start: Option<ydp_core::FiniteF64>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/sendStart。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Started sending request.</pre>
                    ///
                    ///
                    #[serde(rename = "sendStart")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub send_start: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/sendEnd。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Finished sending request.</pre>
                    ///
                    ///
                    #[serde(rename = "sendEnd")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub send_end: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/pushStart。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Time the server started pushing request.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "pushStart")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub push_start: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/pushEnd。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Time the server finished pushing request.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "pushEnd")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub push_end: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/receiveHeadersStart。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Started receiving response headers.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "receiveHeadersStart")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub receive_headers_start: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.ResourceTiming/properties/receiveHeadersEnd。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Finished receiving response headers.</pre>
                    ///
                    ///
                    #[serde(rename = "receiveHeadersEnd")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub receive_headers_end: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ResourceTiming {
                    request_time: fields.request_time,
                    proxy_start: fields.proxy_start,
                    proxy_end: fields.proxy_end,
                    dns_start: fields.dns_start,
                    dns_end: fields.dns_end,
                    connect_start: fields.connect_start,
                    connect_end: fields.connect_end,
                    ssl_start: fields.ssl_start,
                    ssl_end: fields.ssl_end,
                    worker_start: fields.worker_start,
                    worker_ready: fields.worker_ready,
                    worker_fetch_start: fields.worker_fetch_start,
                    worker_respond_with_settled: fields.worker_respond_with_settled,
                    worker_router_evaluation_start: fields.worker_router_evaluation_start,
                    worker_cache_lookup_start: fields.worker_cache_lookup_start,
                    send_start: fields.send_start,
                    send_end: fields.send_end,
                    push_start: fields.push_start,
                    push_end: fields.push_end,
                    receive_headers_start: fields.receive_headers_start,
                    receive_headers_end: fields.receive_headers_end,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.ResourceType。
///
///协议原文：
///
///<pre>Resource type as it was perceived by the rendering engine.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ResourceType {
    ///协议值 "Document"。
    Document,
    ///协议值 "Stylesheet"。
    Stylesheet,
    ///协议值 "Image"。
    Image,
    ///协议值 "Media"。
    Media,
    ///协议值 "Font"。
    Font,
    ///协议值 "Script"。
    Script,
    ///协议值 "TextTrack"。
    TextTrack,
    ///协议值 "XHR"。
    Xhr,
    ///协议值 "Fetch"。
    Fetch,
    ///协议值 "Prefetch"。
    Prefetch,
    ///协议值 "EventSource"。
    EventSource,
    ///协议值 "WebSocket"。
    WebSocket,
    ///协议值 "Manifest"。
    Manifest,
    ///协议值 "SignedExchange"。
    SignedExchange,
    ///协议值 "Ping"。
    Ping,
    ///协议值 "CSPViolationReport"。
    CspViolationReport,
    ///协议值 "Preflight"。
    Preflight,
    ///协议值 "FedCM"。
    FedCm,
    ///协议值 "Other"。
    Other,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ResourceType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::ResourceType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Document => "Document",
            Self::Stylesheet => "Stylesheet",
            Self::Image => "Image",
            Self::Media => "Media",
            Self::Font => "Font",
            Self::Script => "Script",
            Self::TextTrack => "TextTrack",
            Self::Xhr => "XHR",
            Self::Fetch => "Fetch",
            Self::Prefetch => "Prefetch",
            Self::EventSource => "EventSource",
            Self::WebSocket => "WebSocket",
            Self::Manifest => "Manifest",
            Self::SignedExchange => "SignedExchange",
            Self::Ping => "Ping",
            Self::CspViolationReport => "CSPViolationReport",
            Self::Preflight => "Preflight",
            Self::FedCm => "FedCM",
            Self::Other => "Other",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ResourceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ResourceType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Document" => ResourceType::Document,
            "Stylesheet" => ResourceType::Stylesheet,
            "Image" => ResourceType::Image,
            "Media" => ResourceType::Media,
            "Font" => ResourceType::Font,
            "Script" => ResourceType::Script,
            "TextTrack" => ResourceType::TextTrack,
            "XHR" => ResourceType::Xhr,
            "Fetch" => ResourceType::Fetch,
            "Prefetch" => ResourceType::Prefetch,
            "EventSource" => ResourceType::EventSource,
            "WebSocket" => ResourceType::WebSocket,
            "Manifest" => ResourceType::Manifest,
            "SignedExchange" => ResourceType::SignedExchange,
            "Ping" => ResourceType::Ping,
            "CSPViolationReport" => ResourceType::CspViolationReport,
            "Preflight" => ResourceType::Preflight,
            "FedCM" => ResourceType::FedCm,
            "Other" => ResourceType::Other,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ResourceType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ResourceType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResourceType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Document" => ResourceType::Document,
                    "Stylesheet" => ResourceType::Stylesheet,
                    "Image" => ResourceType::Image,
                    "Media" => ResourceType::Media,
                    "Font" => ResourceType::Font,
                    "Script" => ResourceType::Script,
                    "TextTrack" => ResourceType::TextTrack,
                    "XHR" => ResourceType::Xhr,
                    "Fetch" => ResourceType::Fetch,
                    "Prefetch" => ResourceType::Prefetch,
                    "EventSource" => ResourceType::EventSource,
                    "WebSocket" => ResourceType::WebSocket,
                    "Manifest" => ResourceType::Manifest,
                    "SignedExchange" => ResourceType::SignedExchange,
                    "Ping" => ResourceType::Ping,
                    "CSPViolationReport" => ResourceType::CspViolationReport,
                    "Preflight" => ResourceType::Preflight,
                    "FedCM" => ResourceType::FedCm,
                    "Other" => ResourceType::Other,
                    _ => ResourceType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ResourceType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.Response。
///
///协议原文：
///
///<pre>HTTP response data.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct Response {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Response/properties/url。
    ///
    ///协议原文：
    ///
    ///<pre>Response URL. This URL can be different from CachedResource.url in case of redirect.</pre>
    ///
    ///
    #[serde(rename = "url")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Response/properties/status。
    ///
    ///协议原文：
    ///
    ///<pre>HTTP response status code.</pre>
    ///
    ///
    #[serde(rename = "status")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub status: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Response/properties/statusText。
    ///
    ///协议原文：
    ///
    ///<pre>HTTP response status text.</pre>
    ///
    ///
    #[serde(rename = "statusText")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub status_text: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Response/properties/headers。
    ///
    ///协议原文：
    ///
    ///<pre>HTTP response headers.</pre>
    ///
    ///
    #[serde(rename = "headers")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub headers: crate::network::types::Headers,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/headersText。
    ///
    ///协议原文：
    ///
    ///<pre>HTTP response headers text. This has been replaced by the headers in Network.responseReceivedExtraInfo.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "headersText")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub headers_text: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Response/properties/mimeType。
    ///
    ///协议原文：
    ///
    ///<pre>Resource mimeType as determined by the browser.</pre>
    ///
    ///
    #[serde(rename = "mimeType")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub mime_type: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Response/properties/charset。
    ///
    ///协议原文：
    ///
    ///<pre>Resource charset as determined by the browser (if applicable).</pre>
    ///
    ///
    #[serde(rename = "charset")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub charset: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/requestHeaders。
    ///
    ///协议原文：
    ///
    ///<pre>Refined HTTP request headers that were actually transmitted over the network.</pre>
    ///
    ///
    #[serde(rename = "requestHeaders")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub request_headers: Option<crate::network::types::Headers>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/requestHeadersText。
    ///
    ///协议原文：
    ///
    ///<pre>HTTP request headers text. This has been replaced by the headers in Network.requestWillBeSentExtraInfo.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "requestHeadersText")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub request_headers_text: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Response/properties/connectionReused。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies whether physical connection was actually reused for this request.</pre>
    ///
    ///
    #[serde(rename = "connectionReused")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub connection_reused: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Response/properties/connectionId。
    ///
    ///协议原文：
    ///
    ///<pre>Physical connection id that was actually used for this request.</pre>
    ///
    ///
    #[serde(rename = "connectionId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub connection_id: ydp_core::FiniteF64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/remoteIPAddress。
    ///
    ///协议原文：
    ///
    ///<pre>Remote IP address.</pre>
    ///
    ///
    #[serde(rename = "remoteIPAddress")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_ip_address: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/remotePort。
    ///
    ///协议原文：
    ///
    ///<pre>Remote port.</pre>
    ///
    ///
    #[serde(rename = "remotePort")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_port: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/fromDiskCache。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies that the request was served from the disk cache.</pre>
    ///
    ///
    #[serde(rename = "fromDiskCache")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub from_disk_cache: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/fromServiceWorker。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies that the request was served from the ServiceWorker.</pre>
    ///
    ///
    #[serde(rename = "fromServiceWorker")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub from_service_worker: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/fromPrefetchCache。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies that the request was served from the prefetch cache.</pre>
    ///
    ///
    #[serde(rename = "fromPrefetchCache")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub from_prefetch_cache: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/fromEarlyHints。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies that the request was served from the prefetch cache.</pre>
    ///
    ///
    #[serde(rename = "fromEarlyHints")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub from_early_hints: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/serviceWorkerRouterInfo。
    ///
    ///协议原文：
    ///
    ///<pre>Information about how ServiceWorker Static Router API was used. If this
    ///field is set with `matchedSourceType` field, a matching rule is found.
    ///If this field is set without `matchedSource`, no matching rule is found.
    ///Otherwise, the API is not used.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "serviceWorkerRouterInfo")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_worker_router_info: Option<crate::network::types::ServiceWorkerRouterInfo>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Response/properties/encodedDataLength。
    ///
    ///协议原文：
    ///
    ///<pre>Total number of bytes received for this request so far.</pre>
    ///
    ///
    #[serde(rename = "encodedDataLength")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub encoded_data_length: ydp_core::FiniteF64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/timing。
    ///
    ///协议原文：
    ///
    ///<pre>Timing information for the given request.</pre>
    ///
    ///
    #[serde(rename = "timing")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub timing: Option<crate::network::types::ResourceTiming>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/serviceWorkerResponseSource。
    ///
    ///协议原文：
    ///
    ///<pre>Response source of response from ServiceWorker.</pre>
    ///
    ///
    #[serde(rename = "serviceWorkerResponseSource")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_worker_response_source: Option<crate::network::types::ServiceWorkerResponseSource>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/responseTime。
    ///
    ///协议原文：
    ///
    ///<pre>The time at which the returned response was generated.</pre>
    ///
    ///
    #[serde(rename = "responseTime")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_time: Option<crate::network::types::TimeSinceEpoch>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/cacheStorageCacheName。
    ///
    ///协议原文：
    ///
    ///<pre>Cache Storage Cache Name.</pre>
    ///
    ///
    #[serde(rename = "cacheStorageCacheName")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub cache_storage_cache_name: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/protocol。
    ///
    ///协议原文：
    ///
    ///<pre>Protocol used to fetch this request.</pre>
    ///
    ///
    #[serde(rename = "protocol")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub protocol: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/alternateProtocolUsage。
    ///
    ///协议原文：
    ///
    ///<pre>The reason why Chrome uses a specific transport protocol for HTTP semantics.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "alternateProtocolUsage")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub alternate_protocol_usage: Option<crate::network::types::AlternateProtocolUsage>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Response/properties/securityState。
    ///
    ///协议原文：
    ///
    ///<pre>Security state of the request resource.</pre>
    ///
    ///
    #[serde(rename = "securityState")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub security_state: crate::security::types::SecurityState,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/securityDetails。
    ///
    ///协议原文：
    ///
    ///<pre>Security details for the request.</pre>
    ///
    ///
    #[serde(rename = "securityDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub security_details: Option<crate::network::types::SecurityDetails>,
}
impl<'de> serde::Deserialize<'de> for Response {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Response;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Response/properties/url。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Response URL. This URL can be different from CachedResource.url in case of redirect.</pre>
                    ///
                    ///
                    #[serde(rename = "url")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Response/properties/status。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>HTTP response status code.</pre>
                    ///
                    ///
                    #[serde(rename = "status")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub status: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Response/properties/statusText。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>HTTP response status text.</pre>
                    ///
                    ///
                    #[serde(rename = "statusText")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub status_text: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Response/properties/headers。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>HTTP response headers.</pre>
                    ///
                    ///
                    #[serde(rename = "headers")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub headers: crate::network::types::Headers,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/headersText。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>HTTP response headers text. This has been replaced by the headers in Network.responseReceivedExtraInfo.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "headersText")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub headers_text: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Response/properties/mimeType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Resource mimeType as determined by the browser.</pre>
                    ///
                    ///
                    #[serde(rename = "mimeType")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub mime_type: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Response/properties/charset。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Resource charset as determined by the browser (if applicable).</pre>
                    ///
                    ///
                    #[serde(rename = "charset")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub charset: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/requestHeaders。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Refined HTTP request headers that were actually transmitted over the network.</pre>
                    ///
                    ///
                    #[serde(rename = "requestHeaders")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub request_headers: Option<crate::network::types::Headers>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/requestHeadersText。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>HTTP request headers text. This has been replaced by the headers in Network.requestWillBeSentExtraInfo.</pre>
                    ///
                    ///此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "requestHeadersText")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub request_headers_text: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Response/properties/connectionReused。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies whether physical connection was actually reused for this request.</pre>
                    ///
                    ///
                    #[serde(rename = "connectionReused")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub connection_reused: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Response/properties/connectionId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Physical connection id that was actually used for this request.</pre>
                    ///
                    ///
                    #[serde(rename = "connectionId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub connection_id: ydp_core::FiniteF64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/remoteIPAddress。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Remote IP address.</pre>
                    ///
                    ///
                    #[serde(rename = "remoteIPAddress")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub remote_ip_address: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/remotePort。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Remote port.</pre>
                    ///
                    ///
                    #[serde(rename = "remotePort")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub remote_port: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/fromDiskCache。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies that the request was served from the disk cache.</pre>
                    ///
                    ///
                    #[serde(rename = "fromDiskCache")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub from_disk_cache: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/fromServiceWorker。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies that the request was served from the ServiceWorker.</pre>
                    ///
                    ///
                    #[serde(rename = "fromServiceWorker")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub from_service_worker: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/fromPrefetchCache。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies that the request was served from the prefetch cache.</pre>
                    ///
                    ///
                    #[serde(rename = "fromPrefetchCache")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub from_prefetch_cache: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/fromEarlyHints。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies that the request was served from the prefetch cache.</pre>
                    ///
                    ///
                    #[serde(rename = "fromEarlyHints")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub from_early_hints: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/serviceWorkerRouterInfo。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Information about how ServiceWorker Static Router API was used. If this
                    ///field is set with `matchedSourceType` field, a matching rule is found.
                    ///If this field is set without `matchedSource`, no matching rule is found.
                    ///Otherwise, the API is not used.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "serviceWorkerRouterInfo")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub service_worker_router_info:
                        Option<crate::network::types::ServiceWorkerRouterInfo>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Response/properties/encodedDataLength。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Total number of bytes received for this request so far.</pre>
                    ///
                    ///
                    #[serde(rename = "encodedDataLength")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub encoded_data_length: ydp_core::FiniteF64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/timing。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Timing information for the given request.</pre>
                    ///
                    ///
                    #[serde(rename = "timing")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub timing: Option<crate::network::types::ResourceTiming>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/serviceWorkerResponseSource。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Response source of response from ServiceWorker.</pre>
                    ///
                    ///
                    #[serde(rename = "serviceWorkerResponseSource")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub service_worker_response_source:
                        Option<crate::network::types::ServiceWorkerResponseSource>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/responseTime。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The time at which the returned response was generated.</pre>
                    ///
                    ///
                    #[serde(rename = "responseTime")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub response_time: Option<crate::network::types::TimeSinceEpoch>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/cacheStorageCacheName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cache Storage Cache Name.</pre>
                    ///
                    ///
                    #[serde(rename = "cacheStorageCacheName")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub cache_storage_cache_name: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/protocol。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Protocol used to fetch this request.</pre>
                    ///
                    ///
                    #[serde(rename = "protocol")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub protocol: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/alternateProtocolUsage。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The reason why Chrome uses a specific transport protocol for HTTP semantics.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "alternateProtocolUsage")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub alternate_protocol_usage:
                        Option<crate::network::types::AlternateProtocolUsage>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.Response/properties/securityState。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Security state of the request resource.</pre>
                    ///
                    ///
                    #[serde(rename = "securityState")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub security_state: crate::security::types::SecurityState,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.Response/properties/securityDetails。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Security details for the request.</pre>
                    ///
                    ///
                    #[serde(rename = "securityDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub security_details: Option<crate::network::types::SecurityDetails>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(Response {
                    url: fields.url,
                    status: fields.status,
                    status_text: fields.status_text,
                    headers: fields.headers,
                    headers_text: fields.headers_text,
                    mime_type: fields.mime_type,
                    charset: fields.charset,
                    request_headers: fields.request_headers,
                    request_headers_text: fields.request_headers_text,
                    connection_reused: fields.connection_reused,
                    connection_id: fields.connection_id,
                    remote_ip_address: fields.remote_ip_address,
                    remote_port: fields.remote_port,
                    from_disk_cache: fields.from_disk_cache,
                    from_service_worker: fields.from_service_worker,
                    from_prefetch_cache: fields.from_prefetch_cache,
                    from_early_hints: fields.from_early_hints,
                    service_worker_router_info: fields.service_worker_router_info,
                    encoded_data_length: fields.encoded_data_length,
                    timing: fields.timing,
                    service_worker_response_source: fields.service_worker_response_source,
                    response_time: fields.response_time,
                    cache_storage_cache_name: fields.cache_storage_cache_name,
                    protocol: fields.protocol,
                    alternate_protocol_usage: fields.alternate_protocol_usage,
                    security_state: fields.security_state,
                    security_details: fields.security_details,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.SecurityDetails。
///
///协议原文：
///
///<pre>Security details about a request.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct SecurityDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SecurityDetails/properties/protocol。
    ///
    ///协议原文：
    ///
    ///<pre>Protocol name (e.g. "TLS 1.2" or "QUIC").</pre>
    ///
    ///
    #[serde(rename = "protocol")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub protocol: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SecurityDetails/properties/keyExchange。
    ///
    ///协议原文：
    ///
    ///<pre>Key Exchange used by the connection, or the empty string if not applicable.</pre>
    ///
    ///
    #[serde(rename = "keyExchange")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub key_exchange: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.SecurityDetails/properties/keyExchangeGroup。
    ///
    ///协议原文：
    ///
    ///<pre>(EC)DH group used by the connection, if applicable.</pre>
    ///
    ///
    #[serde(rename = "keyExchangeGroup")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub key_exchange_group: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SecurityDetails/properties/cipher。
    ///
    ///协议原文：
    ///
    ///<pre>Cipher name.</pre>
    ///
    ///
    #[serde(rename = "cipher")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cipher: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.SecurityDetails/properties/mac。
    ///
    ///协议原文：
    ///
    ///<pre>TLS MAC. Note that AEAD ciphers do not have separate MACs.</pre>
    ///
    ///
    #[serde(rename = "mac")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub mac: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SecurityDetails/properties/certificateId。
    ///
    ///协议原文：
    ///
    ///<pre>Certificate ID value.</pre>
    ///
    ///
    #[serde(rename = "certificateId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub certificate_id: crate::security::types::CertificateId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SecurityDetails/properties/subjectName。
    ///
    ///协议原文：
    ///
    ///<pre>Certificate subject name.</pre>
    ///
    ///
    #[serde(rename = "subjectName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub subject_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SecurityDetails/properties/sanList。
    ///
    ///协议原文：
    ///
    ///<pre>Subject Alternative Name (SAN) DNS names and IP addresses.</pre>
    ///
    ///
    #[serde(rename = "sanList")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub san_list: Vec<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SecurityDetails/properties/issuer。
    ///
    ///协议原文：
    ///
    ///<pre>Name of the issuing CA.</pre>
    ///
    ///
    #[serde(rename = "issuer")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub issuer: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SecurityDetails/properties/validFrom。
    ///
    ///协议原文：
    ///
    ///<pre>Certificate valid from date.</pre>
    ///
    ///
    #[serde(rename = "validFrom")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub valid_from: crate::network::types::TimeSinceEpoch,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SecurityDetails/properties/validTo。
    ///
    ///协议原文：
    ///
    ///<pre>Certificate valid to (expiration) date</pre>
    ///
    ///
    #[serde(rename = "validTo")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub valid_to: crate::network::types::TimeSinceEpoch,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SecurityDetails/properties/signedCertificateTimestampList。
    ///
    ///协议原文：
    ///
    ///<pre>List of signed certificate timestamps (SCTs).</pre>
    ///
    ///
    #[serde(rename = "signedCertificateTimestampList")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub signed_certificate_timestamp_list: Vec<crate::network::types::SignedCertificateTimestamp>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SecurityDetails/properties/certificateTransparencyCompliance。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the request complied with Certificate Transparency policy</pre>
    ///
    ///
    #[serde(rename = "certificateTransparencyCompliance")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub certificate_transparency_compliance:
        crate::network::types::CertificateTransparencyCompliance,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.SecurityDetails/properties/serverSignatureAlgorithm。
    ///
    ///协议原文：
    ///
    ///<pre>The signature algorithm used by the server in the TLS server signature,
    ///represented as a TLS SignatureScheme code point. Omitted if not
    ///applicable or not known.</pre>
    ///
    ///
    #[serde(rename = "serverSignatureAlgorithm")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub server_signature_algorithm: Option<i64>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SecurityDetails/properties/encryptedClientHello。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the connection used Encrypted ClientHello</pre>
    ///
    ///
    #[serde(rename = "encryptedClientHello")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub encrypted_client_hello: bool,
}
impl<'de> serde::Deserialize<'de> for SecurityDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SecurityDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SecurityDetails/properties/protocol。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Protocol name (e.g. "TLS 1.2" or "QUIC").</pre>
                    ///
                    ///
                    #[serde(rename = "protocol")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub protocol: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SecurityDetails/properties/keyExchange。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Key Exchange used by the connection, or the empty string if not applicable.</pre>
                    ///
                    ///
                    #[serde(rename = "keyExchange")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub key_exchange: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.SecurityDetails/properties/keyExchangeGroup。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>(EC)DH group used by the connection, if applicable.</pre>
                    ///
                    ///
                    #[serde(rename = "keyExchangeGroup")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub key_exchange_group: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SecurityDetails/properties/cipher。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Cipher name.</pre>
                    ///
                    ///
                    #[serde(rename = "cipher")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cipher: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.SecurityDetails/properties/mac。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>TLS MAC. Note that AEAD ciphers do not have separate MACs.</pre>
                    ///
                    ///
                    #[serde(rename = "mac")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub mac: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SecurityDetails/properties/certificateId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Certificate ID value.</pre>
                    ///
                    ///
                    #[serde(rename = "certificateId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub certificate_id: crate::security::types::CertificateId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SecurityDetails/properties/subjectName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Certificate subject name.</pre>
                    ///
                    ///
                    #[serde(rename = "subjectName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub subject_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SecurityDetails/properties/sanList。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Subject Alternative Name (SAN) DNS names and IP addresses.</pre>
                    ///
                    ///
                    #[serde(rename = "sanList")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub san_list: Vec<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SecurityDetails/properties/issuer。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Name of the issuing CA.</pre>
                    ///
                    ///
                    #[serde(rename = "issuer")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub issuer: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SecurityDetails/properties/validFrom。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Certificate valid from date.</pre>
                    ///
                    ///
                    #[serde(rename = "validFrom")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub valid_from: crate::network::types::TimeSinceEpoch,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SecurityDetails/properties/validTo。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Certificate valid to (expiration) date</pre>
                    ///
                    ///
                    #[serde(rename = "validTo")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub valid_to: crate::network::types::TimeSinceEpoch,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SecurityDetails/properties/signedCertificateTimestampList。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>List of signed certificate timestamps (SCTs).</pre>
                    ///
                    ///
                    #[serde(rename = "signedCertificateTimestampList")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub signed_certificate_timestamp_list:
                        Vec<crate::network::types::SignedCertificateTimestamp>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SecurityDetails/properties/certificateTransparencyCompliance。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the request complied with Certificate Transparency policy</pre>
                    ///
                    ///
                    #[serde(rename = "certificateTransparencyCompliance")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub certificate_transparency_compliance:
                        crate::network::types::CertificateTransparencyCompliance,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.SecurityDetails/properties/serverSignatureAlgorithm。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The signature algorithm used by the server in the TLS server signature,
                    ///represented as a TLS SignatureScheme code point. Omitted if not
                    ///applicable or not known.</pre>
                    ///
                    ///
                    #[serde(rename = "serverSignatureAlgorithm")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub server_signature_algorithm: Option<i64>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SecurityDetails/properties/encryptedClientHello。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the connection used Encrypted ClientHello</pre>
                    ///
                    ///
                    #[serde(rename = "encryptedClientHello")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub encrypted_client_hello: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SecurityDetails {
                    protocol: fields.protocol,
                    key_exchange: fields.key_exchange,
                    key_exchange_group: fields.key_exchange_group,
                    cipher: fields.cipher,
                    mac: fields.mac,
                    certificate_id: fields.certificate_id,
                    subject_name: fields.subject_name,
                    san_list: fields.san_list,
                    issuer: fields.issuer,
                    valid_from: fields.valid_from,
                    valid_to: fields.valid_to,
                    signed_certificate_timestamp_list: fields.signed_certificate_timestamp_list,
                    certificate_transparency_compliance: fields.certificate_transparency_compliance,
                    server_signature_algorithm: fields.server_signature_algorithm,
                    encrypted_client_hello: fields.encrypted_client_hello,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.SecurityIsolationStatus。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SecurityIsolationStatus {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.SecurityIsolationStatus/properties/coop。
    #[serde(rename = "coop")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub coop: Option<crate::network::types::CrossOriginOpenerPolicyStatus>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.SecurityIsolationStatus/properties/coep。
    #[serde(rename = "coep")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub coep: Option<crate::network::types::CrossOriginEmbedderPolicyStatus>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.SecurityIsolationStatus/properties/csp。
    #[serde(rename = "csp")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub csp: Option<Vec<crate::network::types::ContentSecurityPolicyStatus>>,
}
impl<'de> serde::Deserialize<'de> for SecurityIsolationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SecurityIsolationStatus;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.SecurityIsolationStatus/properties/coop。
                    #[serde(rename = "coop")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub coop: Option<crate::network::types::CrossOriginOpenerPolicyStatus>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.SecurityIsolationStatus/properties/coep。
                    #[serde(rename = "coep")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub coep: Option<crate::network::types::CrossOriginEmbedderPolicyStatus>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.SecurityIsolationStatus/properties/csp。
                    #[serde(rename = "csp")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub csp: Option<Vec<crate::network::types::ContentSecurityPolicyStatus>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SecurityIsolationStatus {
                    coop: fields.coop,
                    coep: fields.coep,
                    csp: fields.csp,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.ServiceWorkerResponseSource。
///
///协议原文：
///
///<pre>Source of serviceworker response.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ServiceWorkerResponseSource {
    ///协议值 "cache-storage"。
    CacheStorage,
    ///协议值 "http-cache"。
    HttpCache,
    ///协议值 "fallback-code"。
    FallbackCode,
    ///协议值 "network"。
    Network,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ServiceWorkerResponseSource {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::ServiceWorkerResponseSource::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::CacheStorage => "cache-storage",
            Self::HttpCache => "http-cache",
            Self::FallbackCode => "fallback-code",
            Self::Network => "network",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ServiceWorkerResponseSource {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ServiceWorkerResponseSource {
    fn from(value: String) -> Self {
        match value.as_str() {
            "cache-storage" => ServiceWorkerResponseSource::CacheStorage,
            "http-cache" => ServiceWorkerResponseSource::HttpCache,
            "fallback-code" => ServiceWorkerResponseSource::FallbackCode,
            "network" => ServiceWorkerResponseSource::Network,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ServiceWorkerResponseSource {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ServiceWorkerResponseSource {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ServiceWorkerResponseSource;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "cache-storage" => ServiceWorkerResponseSource::CacheStorage,
                    "http-cache" => ServiceWorkerResponseSource::HttpCache,
                    "fallback-code" => ServiceWorkerResponseSource::FallbackCode,
                    "network" => ServiceWorkerResponseSource::Network,
                    _ => ServiceWorkerResponseSource::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ServiceWorkerResponseSource::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.ServiceWorkerRouterInfo。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ServiceWorkerRouterInfo {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.ServiceWorkerRouterInfo/properties/ruleIdMatched。
    ///
    ///协议原文：
    ///
    ///<pre>ID of the rule matched. If there is a matched rule, this field will
    ///be set, otherwiser no value will be set.</pre>
    ///
    ///
    #[serde(rename = "ruleIdMatched")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub rule_id_matched: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.ServiceWorkerRouterInfo/properties/matchedSourceType。
    ///
    ///协议原文：
    ///
    ///<pre>The router source of the matched rule. If there is a matched rule, this
    ///field will be set, otherwise no value will be set.</pre>
    ///
    ///
    #[serde(rename = "matchedSourceType")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub matched_source_type: Option<crate::network::types::ServiceWorkerRouterSource>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.ServiceWorkerRouterInfo/properties/actualSourceType。
    ///
    ///协议原文：
    ///
    ///<pre>The actual router source used.</pre>
    ///
    ///
    #[serde(rename = "actualSourceType")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub actual_source_type: Option<crate::network::types::ServiceWorkerRouterSource>,
}
impl<'de> serde::Deserialize<'de> for ServiceWorkerRouterInfo {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ServiceWorkerRouterInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.ServiceWorkerRouterInfo/properties/ruleIdMatched。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>ID of the rule matched. If there is a matched rule, this field will
                    ///be set, otherwiser no value will be set.</pre>
                    ///
                    ///
                    #[serde(rename = "ruleIdMatched")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub rule_id_matched: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.ServiceWorkerRouterInfo/properties/matchedSourceType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The router source of the matched rule. If there is a matched rule, this
                    ///field will be set, otherwise no value will be set.</pre>
                    ///
                    ///
                    #[serde(rename = "matchedSourceType")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub matched_source_type:
                        Option<crate::network::types::ServiceWorkerRouterSource>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.ServiceWorkerRouterInfo/properties/actualSourceType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The actual router source used.</pre>
                    ///
                    ///
                    #[serde(rename = "actualSourceType")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub actual_source_type:
                        Option<crate::network::types::ServiceWorkerRouterSource>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ServiceWorkerRouterInfo {
                    rule_id_matched: fields.rule_id_matched,
                    matched_source_type: fields.matched_source_type,
                    actual_source_type: fields.actual_source_type,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.ServiceWorkerRouterSource。
///
///协议原文：
///
///<pre>Source of service worker router.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ServiceWorkerRouterSource {
    ///协议值 "network"。
    Network,
    ///协议值 "cache"。
    Cache,
    ///协议值 "fetch-event"。
    FetchEvent,
    ///协议值 "race-network-and-fetch-handler"。
    RaceNetworkAndFetchHandler,
    ///协议值 "race-network-and-cache"。
    RaceNetworkAndCache,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl ServiceWorkerRouterSource {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::ServiceWorkerRouterSource::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Network => "network",
            Self::Cache => "cache",
            Self::FetchEvent => "fetch-event",
            Self::RaceNetworkAndFetchHandler => "race-network-and-fetch-handler",
            Self::RaceNetworkAndCache => "race-network-and-cache",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for ServiceWorkerRouterSource {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for ServiceWorkerRouterSource {
    fn from(value: String) -> Self {
        match value.as_str() {
            "network" => ServiceWorkerRouterSource::Network,
            "cache" => ServiceWorkerRouterSource::Cache,
            "fetch-event" => ServiceWorkerRouterSource::FetchEvent,
            "race-network-and-fetch-handler" => {
                ServiceWorkerRouterSource::RaceNetworkAndFetchHandler
            }
            "race-network-and-cache" => ServiceWorkerRouterSource::RaceNetworkAndCache,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for ServiceWorkerRouterSource {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ServiceWorkerRouterSource {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ServiceWorkerRouterSource;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "network" => ServiceWorkerRouterSource::Network,
                    "cache" => ServiceWorkerRouterSource::Cache,
                    "fetch-event" => ServiceWorkerRouterSource::FetchEvent,
                    "race-network-and-fetch-handler" => {
                        ServiceWorkerRouterSource::RaceNetworkAndFetchHandler
                    }
                    "race-network-and-cache" => ServiceWorkerRouterSource::RaceNetworkAndCache,
                    _ => ServiceWorkerRouterSource::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(ServiceWorkerRouterSource::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.SetCookieBlockedReason。
///
///协议原文：
///
///<pre>Types of reasons why a cookie may not be stored from a response.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SetCookieBlockedReason {
    ///协议值 "SecureOnly"。
    SecureOnly,
    ///协议值 "SameSiteStrict"。
    SameSiteStrict,
    ///协议值 "SameSiteLax"。
    SameSiteLax,
    ///协议值 "SameSiteUnspecifiedTreatedAsLax"。
    SameSiteUnspecifiedTreatedAsLax,
    ///协议值 "SameSiteNoneInsecure"。
    SameSiteNoneInsecure,
    ///协议值 "UserPreferences"。
    UserPreferences,
    ///协议值 "ThirdPartyPhaseout"。
    ThirdPartyPhaseout,
    ///协议值 "ThirdPartyBlockedInFirstPartySet"。
    ThirdPartyBlockedInFirstPartySet,
    ///协议值 "SyntaxError"。
    SyntaxError,
    ///协议值 "SchemeNotSupported"。
    SchemeNotSupported,
    ///协议值 "OverwriteSecure"。
    OverwriteSecure,
    ///协议值 "InvalidDomain"。
    InvalidDomain,
    ///协议值 "InvalidPrefix"。
    InvalidPrefix,
    ///协议值 "UnknownError"。
    UnknownError,
    ///协议值 "SchemefulSameSiteStrict"。
    SchemefulSameSiteStrict,
    ///协议值 "SchemefulSameSiteLax"。
    SchemefulSameSiteLax,
    ///协议值 "SchemefulSameSiteUnspecifiedTreatedAsLax"。
    SchemefulSameSiteUnspecifiedTreatedAsLax,
    ///协议值 "NameValuePairExceedsMaxSize"。
    NameValuePairExceedsMaxSize,
    ///协议值 "DisallowedCharacter"。
    DisallowedCharacter,
    ///协议值 "NoCookieContent"。
    NoCookieContent,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl SetCookieBlockedReason {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::SetCookieBlockedReason::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::SecureOnly => "SecureOnly",
            Self::SameSiteStrict => "SameSiteStrict",
            Self::SameSiteLax => "SameSiteLax",
            Self::SameSiteUnspecifiedTreatedAsLax => "SameSiteUnspecifiedTreatedAsLax",
            Self::SameSiteNoneInsecure => "SameSiteNoneInsecure",
            Self::UserPreferences => "UserPreferences",
            Self::ThirdPartyPhaseout => "ThirdPartyPhaseout",
            Self::ThirdPartyBlockedInFirstPartySet => "ThirdPartyBlockedInFirstPartySet",
            Self::SyntaxError => "SyntaxError",
            Self::SchemeNotSupported => "SchemeNotSupported",
            Self::OverwriteSecure => "OverwriteSecure",
            Self::InvalidDomain => "InvalidDomain",
            Self::InvalidPrefix => "InvalidPrefix",
            Self::UnknownError => "UnknownError",
            Self::SchemefulSameSiteStrict => "SchemefulSameSiteStrict",
            Self::SchemefulSameSiteLax => "SchemefulSameSiteLax",
            Self::SchemefulSameSiteUnspecifiedTreatedAsLax => {
                "SchemefulSameSiteUnspecifiedTreatedAsLax"
            }
            Self::NameValuePairExceedsMaxSize => "NameValuePairExceedsMaxSize",
            Self::DisallowedCharacter => "DisallowedCharacter",
            Self::NoCookieContent => "NoCookieContent",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for SetCookieBlockedReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for SetCookieBlockedReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "SecureOnly" => SetCookieBlockedReason::SecureOnly,
            "SameSiteStrict" => SetCookieBlockedReason::SameSiteStrict,
            "SameSiteLax" => SetCookieBlockedReason::SameSiteLax,
            "SameSiteUnspecifiedTreatedAsLax" => {
                SetCookieBlockedReason::SameSiteUnspecifiedTreatedAsLax
            }
            "SameSiteNoneInsecure" => SetCookieBlockedReason::SameSiteNoneInsecure,
            "UserPreferences" => SetCookieBlockedReason::UserPreferences,
            "ThirdPartyPhaseout" => SetCookieBlockedReason::ThirdPartyPhaseout,
            "ThirdPartyBlockedInFirstPartySet" => {
                SetCookieBlockedReason::ThirdPartyBlockedInFirstPartySet
            }
            "SyntaxError" => SetCookieBlockedReason::SyntaxError,
            "SchemeNotSupported" => SetCookieBlockedReason::SchemeNotSupported,
            "OverwriteSecure" => SetCookieBlockedReason::OverwriteSecure,
            "InvalidDomain" => SetCookieBlockedReason::InvalidDomain,
            "InvalidPrefix" => SetCookieBlockedReason::InvalidPrefix,
            "UnknownError" => SetCookieBlockedReason::UnknownError,
            "SchemefulSameSiteStrict" => SetCookieBlockedReason::SchemefulSameSiteStrict,
            "SchemefulSameSiteLax" => SetCookieBlockedReason::SchemefulSameSiteLax,
            "SchemefulSameSiteUnspecifiedTreatedAsLax" => {
                SetCookieBlockedReason::SchemefulSameSiteUnspecifiedTreatedAsLax
            }
            "NameValuePairExceedsMaxSize" => SetCookieBlockedReason::NameValuePairExceedsMaxSize,
            "DisallowedCharacter" => SetCookieBlockedReason::DisallowedCharacter,
            "NoCookieContent" => SetCookieBlockedReason::NoCookieContent,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for SetCookieBlockedReason {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetCookieBlockedReason {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetCookieBlockedReason;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "SecureOnly" => SetCookieBlockedReason::SecureOnly,
                    "SameSiteStrict" => SetCookieBlockedReason::SameSiteStrict,
                    "SameSiteLax" => SetCookieBlockedReason::SameSiteLax,
                    "SameSiteUnspecifiedTreatedAsLax" => {
                        SetCookieBlockedReason::SameSiteUnspecifiedTreatedAsLax
                    }
                    "SameSiteNoneInsecure" => SetCookieBlockedReason::SameSiteNoneInsecure,
                    "UserPreferences" => SetCookieBlockedReason::UserPreferences,
                    "ThirdPartyPhaseout" => SetCookieBlockedReason::ThirdPartyPhaseout,
                    "ThirdPartyBlockedInFirstPartySet" => {
                        SetCookieBlockedReason::ThirdPartyBlockedInFirstPartySet
                    }
                    "SyntaxError" => SetCookieBlockedReason::SyntaxError,
                    "SchemeNotSupported" => SetCookieBlockedReason::SchemeNotSupported,
                    "OverwriteSecure" => SetCookieBlockedReason::OverwriteSecure,
                    "InvalidDomain" => SetCookieBlockedReason::InvalidDomain,
                    "InvalidPrefix" => SetCookieBlockedReason::InvalidPrefix,
                    "UnknownError" => SetCookieBlockedReason::UnknownError,
                    "SchemefulSameSiteStrict" => SetCookieBlockedReason::SchemefulSameSiteStrict,
                    "SchemefulSameSiteLax" => SetCookieBlockedReason::SchemefulSameSiteLax,
                    "SchemefulSameSiteUnspecifiedTreatedAsLax" => {
                        SetCookieBlockedReason::SchemefulSameSiteUnspecifiedTreatedAsLax
                    }
                    "NameValuePairExceedsMaxSize" => {
                        SetCookieBlockedReason::NameValuePairExceedsMaxSize
                    }
                    "DisallowedCharacter" => SetCookieBlockedReason::DisallowedCharacter,
                    "NoCookieContent" => SetCookieBlockedReason::NoCookieContent,
                    _ => SetCookieBlockedReason::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(SetCookieBlockedReason::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.SignedCertificateTimestamp。
///
///协议原文：
///
///<pre>Details of a signed certificate timestamp (SCT).</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct SignedCertificateTimestamp {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedCertificateTimestamp/properties/status。
    ///
    ///协议原文：
    ///
    ///<pre>Validation status.</pre>
    ///
    ///
    #[serde(rename = "status")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub status: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedCertificateTimestamp/properties/origin。
    ///
    ///协议原文：
    ///
    ///<pre>Origin.</pre>
    ///
    ///
    #[serde(rename = "origin")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub origin: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedCertificateTimestamp/properties/logDescription。
    ///
    ///协议原文：
    ///
    ///<pre>Log name / description.</pre>
    ///
    ///
    #[serde(rename = "logDescription")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub log_description: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedCertificateTimestamp/properties/logId。
    ///
    ///协议原文：
    ///
    ///<pre>Log ID.</pre>
    ///
    ///
    #[serde(rename = "logId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub log_id: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedCertificateTimestamp/properties/timestamp。
    ///
    ///协议原文：
    ///
    ///<pre>Issuance date. Unlike TimeSinceEpoch, this contains the number of
    ///milliseconds since January 1, 1970, UTC, not the number of seconds.</pre>
    ///
    ///
    #[serde(rename = "timestamp")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timestamp: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedCertificateTimestamp/properties/hashAlgorithm。
    ///
    ///协议原文：
    ///
    ///<pre>Hash algorithm.</pre>
    ///
    ///
    #[serde(rename = "hashAlgorithm")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub hash_algorithm: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedCertificateTimestamp/properties/signatureAlgorithm。
    ///
    ///协议原文：
    ///
    ///<pre>Signature algorithm.</pre>
    ///
    ///
    #[serde(rename = "signatureAlgorithm")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub signature_algorithm: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedCertificateTimestamp/properties/signatureData。
    ///
    ///协议原文：
    ///
    ///<pre>Signature data.</pre>
    ///
    ///
    #[serde(rename = "signatureData")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub signature_data: String,
}
impl<'de> serde::Deserialize<'de> for SignedCertificateTimestamp {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SignedCertificateTimestamp;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedCertificateTimestamp/properties/status。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Validation status.</pre>
                    ///
                    ///
                    #[serde(rename = "status")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub status: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedCertificateTimestamp/properties/origin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Origin.</pre>
                    ///
                    ///
                    #[serde(rename = "origin")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub origin: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedCertificateTimestamp/properties/logDescription。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Log name / description.</pre>
                    ///
                    ///
                    #[serde(rename = "logDescription")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub log_description: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedCertificateTimestamp/properties/logId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Log ID.</pre>
                    ///
                    ///
                    #[serde(rename = "logId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub log_id: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedCertificateTimestamp/properties/timestamp。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Issuance date. Unlike TimeSinceEpoch, this contains the number of
                    ///milliseconds since January 1, 1970, UTC, not the number of seconds.</pre>
                    ///
                    ///
                    #[serde(rename = "timestamp")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timestamp: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedCertificateTimestamp/properties/hashAlgorithm。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Hash algorithm.</pre>
                    ///
                    ///
                    #[serde(rename = "hashAlgorithm")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub hash_algorithm: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedCertificateTimestamp/properties/signatureAlgorithm。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Signature algorithm.</pre>
                    ///
                    ///
                    #[serde(rename = "signatureAlgorithm")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub signature_algorithm: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedCertificateTimestamp/properties/signatureData。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Signature data.</pre>
                    ///
                    ///
                    #[serde(rename = "signatureData")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub signature_data: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SignedCertificateTimestamp {
                    status: fields.status,
                    origin: fields.origin,
                    log_description: fields.log_description,
                    log_id: fields.log_id,
                    timestamp: fields.timestamp,
                    hash_algorithm: fields.hash_algorithm,
                    signature_algorithm: fields.signature_algorithm,
                    signature_data: fields.signature_data,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.SignedExchangeError。
///
///协议原文：
///
///<pre>Information about a signed exchange response.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SignedExchangeError {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedExchangeError/properties/message。
    ///
    ///协议原文：
    ///
    ///<pre>Error message.</pre>
    ///
    ///
    #[serde(rename = "message")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub message: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.SignedExchangeError/properties/signatureIndex。
    ///
    ///协议原文：
    ///
    ///<pre>The index of the signature which caused the error.</pre>
    ///
    ///
    #[serde(rename = "signatureIndex")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub signature_index: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.SignedExchangeError/properties/errorField。
    ///
    ///协议原文：
    ///
    ///<pre>The field which caused the error.</pre>
    ///
    ///
    #[serde(rename = "errorField")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub error_field: Option<crate::network::types::SignedExchangeErrorField>,
}
impl<'de> serde::Deserialize<'de> for SignedExchangeError {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SignedExchangeError;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedExchangeError/properties/message。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Error message.</pre>
                    ///
                    ///
                    #[serde(rename = "message")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub message: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.SignedExchangeError/properties/signatureIndex。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The index of the signature which caused the error.</pre>
                    ///
                    ///
                    #[serde(rename = "signatureIndex")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub signature_index: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.SignedExchangeError/properties/errorField。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The field which caused the error.</pre>
                    ///
                    ///
                    #[serde(rename = "errorField")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub error_field: Option<crate::network::types::SignedExchangeErrorField>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SignedExchangeError {
                    message: fields.message,
                    signature_index: fields.signature_index,
                    error_field: fields.error_field,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.SignedExchangeErrorField。
///
///协议原文：
///
///<pre>Field type for a signed exchange related error.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SignedExchangeErrorField {
    ///协议值 "signatureSig"。
    SignatureSig,
    ///协议值 "signatureIntegrity"。
    SignatureIntegrity,
    ///协议值 "signatureCertUrl"。
    SignatureCertUrl,
    ///协议值 "signatureCertSha256"。
    SignatureCertSha256,
    ///协议值 "signatureValidityUrl"。
    SignatureValidityUrl,
    ///协议值 "signatureTimestamps"。
    SignatureTimestamps,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl SignedExchangeErrorField {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::SignedExchangeErrorField::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::SignatureSig => "signatureSig",
            Self::SignatureIntegrity => "signatureIntegrity",
            Self::SignatureCertUrl => "signatureCertUrl",
            Self::SignatureCertSha256 => "signatureCertSha256",
            Self::SignatureValidityUrl => "signatureValidityUrl",
            Self::SignatureTimestamps => "signatureTimestamps",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for SignedExchangeErrorField {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for SignedExchangeErrorField {
    fn from(value: String) -> Self {
        match value.as_str() {
            "signatureSig" => SignedExchangeErrorField::SignatureSig,
            "signatureIntegrity" => SignedExchangeErrorField::SignatureIntegrity,
            "signatureCertUrl" => SignedExchangeErrorField::SignatureCertUrl,
            "signatureCertSha256" => SignedExchangeErrorField::SignatureCertSha256,
            "signatureValidityUrl" => SignedExchangeErrorField::SignatureValidityUrl,
            "signatureTimestamps" => SignedExchangeErrorField::SignatureTimestamps,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for SignedExchangeErrorField {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SignedExchangeErrorField {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SignedExchangeErrorField;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "signatureSig" => SignedExchangeErrorField::SignatureSig,
                    "signatureIntegrity" => SignedExchangeErrorField::SignatureIntegrity,
                    "signatureCertUrl" => SignedExchangeErrorField::SignatureCertUrl,
                    "signatureCertSha256" => SignedExchangeErrorField::SignatureCertSha256,
                    "signatureValidityUrl" => SignedExchangeErrorField::SignatureValidityUrl,
                    "signatureTimestamps" => SignedExchangeErrorField::SignatureTimestamps,
                    _ => SignedExchangeErrorField::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(SignedExchangeErrorField::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.SignedExchangeHeader。
///
///协议原文：
///
///<pre>Information about a signed exchange header.
///https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#cbor-representation</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SignedExchangeHeader {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedExchangeHeader/properties/requestUrl。
    ///
    ///协议原文：
    ///
    ///<pre>Signed exchange request URL.</pre>
    ///
    ///
    #[serde(rename = "requestUrl")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub request_url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedExchangeHeader/properties/responseCode。
    ///
    ///协议原文：
    ///
    ///<pre>Signed exchange response code.</pre>
    ///
    ///
    #[serde(rename = "responseCode")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub response_code: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedExchangeHeader/properties/responseHeaders。
    ///
    ///协议原文：
    ///
    ///<pre>Signed exchange response headers.</pre>
    ///
    ///
    #[serde(rename = "responseHeaders")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub response_headers: crate::network::types::Headers,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedExchangeHeader/properties/signatures。
    ///
    ///协议原文：
    ///
    ///<pre>Signed exchange response signature.</pre>
    ///
    ///
    #[serde(rename = "signatures")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub signatures: Vec<crate::network::types::SignedExchangeSignature>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedExchangeHeader/properties/headerIntegrity。
    ///
    ///协议原文：
    ///
    ///<pre>Signed exchange header integrity hash in the form of `sha256-&lt;base64-hash-value&gt;`.</pre>
    ///
    ///
    #[serde(rename = "headerIntegrity")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub header_integrity: String,
}
impl<'de> serde::Deserialize<'de> for SignedExchangeHeader {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SignedExchangeHeader;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedExchangeHeader/properties/requestUrl。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Signed exchange request URL.</pre>
                    ///
                    ///
                    #[serde(rename = "requestUrl")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub request_url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedExchangeHeader/properties/responseCode。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Signed exchange response code.</pre>
                    ///
                    ///
                    #[serde(rename = "responseCode")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub response_code: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedExchangeHeader/properties/responseHeaders。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Signed exchange response headers.</pre>
                    ///
                    ///
                    #[serde(rename = "responseHeaders")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub response_headers: crate::network::types::Headers,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedExchangeHeader/properties/signatures。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Signed exchange response signature.</pre>
                    ///
                    ///
                    #[serde(rename = "signatures")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub signatures: Vec<crate::network::types::SignedExchangeSignature>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedExchangeHeader/properties/headerIntegrity。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Signed exchange header integrity hash in the form of `sha256-&lt;base64-hash-value&gt;`.</pre>
                    ///
                    ///
                    #[serde(rename = "headerIntegrity")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub header_integrity: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SignedExchangeHeader {
                    request_url: fields.request_url,
                    response_code: fields.response_code,
                    response_headers: fields.response_headers,
                    signatures: fields.signatures,
                    header_integrity: fields.header_integrity,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.SignedExchangeInfo。
///
///协议原文：
///
///<pre>Information about a signed exchange response.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SignedExchangeInfo {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedExchangeInfo/properties/outerResponse。
    ///
    ///协议原文：
    ///
    ///<pre>The outer response of signed HTTP exchange which was received from network.</pre>
    ///
    ///
    #[serde(rename = "outerResponse")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub outer_response: crate::network::types::Response,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedExchangeInfo/properties/hasExtraInfo。
    ///
    ///协议原文：
    ///
    ///<pre>Whether network response for the signed exchange was accompanied by
    ///extra headers.</pre>
    ///
    ///
    #[serde(rename = "hasExtraInfo")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub has_extra_info: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.SignedExchangeInfo/properties/header。
    ///
    ///协议原文：
    ///
    ///<pre>Information about the signed exchange header.</pre>
    ///
    ///
    #[serde(rename = "header")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub header: Option<crate::network::types::SignedExchangeHeader>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.SignedExchangeInfo/properties/securityDetails。
    ///
    ///协议原文：
    ///
    ///<pre>Security details for the signed exchange header.</pre>
    ///
    ///
    #[serde(rename = "securityDetails")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub security_details: Option<crate::network::types::SecurityDetails>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.SignedExchangeInfo/properties/errors。
    ///
    ///协议原文：
    ///
    ///<pre>Errors occurred while handling the signed exchange.</pre>
    ///
    ///
    #[serde(rename = "errors")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub errors: Option<Vec<crate::network::types::SignedExchangeError>>,
}
impl<'de> serde::Deserialize<'de> for SignedExchangeInfo {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SignedExchangeInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedExchangeInfo/properties/outerResponse。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The outer response of signed HTTP exchange which was received from network.</pre>
                    ///
                    ///
                    #[serde(rename = "outerResponse")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub outer_response: crate::network::types::Response,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedExchangeInfo/properties/hasExtraInfo。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether network response for the signed exchange was accompanied by
                    ///extra headers.</pre>
                    ///
                    ///
                    #[serde(rename = "hasExtraInfo")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub has_extra_info: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.SignedExchangeInfo/properties/header。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Information about the signed exchange header.</pre>
                    ///
                    ///
                    #[serde(rename = "header")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub header: Option<crate::network::types::SignedExchangeHeader>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.SignedExchangeInfo/properties/securityDetails。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Security details for the signed exchange header.</pre>
                    ///
                    ///
                    #[serde(rename = "securityDetails")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub security_details: Option<crate::network::types::SecurityDetails>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.SignedExchangeInfo/properties/errors。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Errors occurred while handling the signed exchange.</pre>
                    ///
                    ///
                    #[serde(rename = "errors")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub errors: Option<Vec<crate::network::types::SignedExchangeError>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SignedExchangeInfo {
                    outer_response: fields.outer_response,
                    has_extra_info: fields.has_extra_info,
                    header: fields.header,
                    security_details: fields.security_details,
                    errors: fields.errors,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.SignedExchangeSignature。
///
///协议原文：
///
///<pre>Information about a signed exchange signature.
///https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#rfc.section.3.1</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SignedExchangeSignature {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedExchangeSignature/properties/label。
    ///
    ///协议原文：
    ///
    ///<pre>Signed exchange signature label.</pre>
    ///
    ///
    #[serde(rename = "label")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub label: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedExchangeSignature/properties/signature。
    ///
    ///协议原文：
    ///
    ///<pre>The hex string of signed exchange signature.</pre>
    ///
    ///
    #[serde(rename = "signature")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub signature: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedExchangeSignature/properties/integrity。
    ///
    ///协议原文：
    ///
    ///<pre>Signed exchange signature integrity.</pre>
    ///
    ///
    #[serde(rename = "integrity")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub integrity: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.SignedExchangeSignature/properties/certUrl。
    ///
    ///协议原文：
    ///
    ///<pre>Signed exchange signature cert Url.</pre>
    ///
    ///
    #[serde(rename = "certUrl")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub cert_url: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.SignedExchangeSignature/properties/certSha256。
    ///
    ///协议原文：
    ///
    ///<pre>The hex string of signed exchange signature cert sha256.</pre>
    ///
    ///
    #[serde(rename = "certSha256")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub cert_sha256: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedExchangeSignature/properties/validityUrl。
    ///
    ///协议原文：
    ///
    ///<pre>Signed exchange signature validity Url.</pre>
    ///
    ///
    #[serde(rename = "validityUrl")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub validity_url: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedExchangeSignature/properties/date。
    ///
    ///协议原文：
    ///
    ///<pre>Signed exchange signature date.</pre>
    ///
    ///
    #[serde(rename = "date")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub date: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedExchangeSignature/properties/expires。
    ///
    ///协议原文：
    ///
    ///<pre>Signed exchange signature expires.</pre>
    ///
    ///
    #[serde(rename = "expires")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub expires: i64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.SignedExchangeSignature/properties/certificates。
    ///
    ///协议原文：
    ///
    ///<pre>The encoded certificates.</pre>
    ///
    ///
    #[serde(rename = "certificates")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub certificates: Option<Vec<String>>,
}
impl<'de> serde::Deserialize<'de> for SignedExchangeSignature {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SignedExchangeSignature;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedExchangeSignature/properties/label。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Signed exchange signature label.</pre>
                    ///
                    ///
                    #[serde(rename = "label")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub label: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedExchangeSignature/properties/signature。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The hex string of signed exchange signature.</pre>
                    ///
                    ///
                    #[serde(rename = "signature")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub signature: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedExchangeSignature/properties/integrity。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Signed exchange signature integrity.</pre>
                    ///
                    ///
                    #[serde(rename = "integrity")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub integrity: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.SignedExchangeSignature/properties/certUrl。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Signed exchange signature cert Url.</pre>
                    ///
                    ///
                    #[serde(rename = "certUrl")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub cert_url: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.SignedExchangeSignature/properties/certSha256。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The hex string of signed exchange signature cert sha256.</pre>
                    ///
                    ///
                    #[serde(rename = "certSha256")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub cert_sha256: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedExchangeSignature/properties/validityUrl。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Signed exchange signature validity Url.</pre>
                    ///
                    ///
                    #[serde(rename = "validityUrl")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub validity_url: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedExchangeSignature/properties/date。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Signed exchange signature date.</pre>
                    ///
                    ///
                    #[serde(rename = "date")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub date: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.SignedExchangeSignature/properties/expires。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Signed exchange signature expires.</pre>
                    ///
                    ///
                    #[serde(rename = "expires")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub expires: i64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.SignedExchangeSignature/properties/certificates。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The encoded certificates.</pre>
                    ///
                    ///
                    #[serde(rename = "certificates")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub certificates: Option<Vec<String>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SignedExchangeSignature {
                    label: fields.label,
                    signature: fields.signature,
                    integrity: fields.integrity,
                    cert_url: fields.cert_url,
                    cert_sha256: fields.cert_sha256,
                    validity_url: fields.validity_url,
                    date: fields.date,
                    expires: fields.expires,
                    certificates: fields.certificates,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.TerminationEventDetails/properties/deletionReason。
///
///协议原文：
///
///<pre>The reason for a session being deleted.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TerminationEventDetailsDeletionReason {
    ///协议值 "Expired"。
    Expired,
    ///协议值 "FailedToRestoreKey"。
    FailedToRestoreKey,
    ///协议值 "FailedToUnwrapKey"。
    FailedToUnwrapKey,
    ///协议值 "StoragePartitionCleared"。
    StoragePartitionCleared,
    ///协议值 "ClearBrowsingData"。
    ClearBrowsingData,
    ///协议值 "ServerRequested"。
    ServerRequested,
    ///协议值 "InvalidSessionParams"。
    InvalidSessionParams,
    ///协议值 "RefreshFatalError"。
    RefreshFatalError,
    ///协议值 "DevTools"。
    DevTools,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl TerminationEventDetailsDeletionReason {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::TerminationEventDetailsDeletionReason::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Expired => "Expired",
            Self::FailedToRestoreKey => "FailedToRestoreKey",
            Self::FailedToUnwrapKey => "FailedToUnwrapKey",
            Self::StoragePartitionCleared => "StoragePartitionCleared",
            Self::ClearBrowsingData => "ClearBrowsingData",
            Self::ServerRequested => "ServerRequested",
            Self::InvalidSessionParams => "InvalidSessionParams",
            Self::RefreshFatalError => "RefreshFatalError",
            Self::DevTools => "DevTools",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for TerminationEventDetailsDeletionReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for TerminationEventDetailsDeletionReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Expired" => TerminationEventDetailsDeletionReason::Expired,
            "FailedToRestoreKey" => TerminationEventDetailsDeletionReason::FailedToRestoreKey,
            "FailedToUnwrapKey" => TerminationEventDetailsDeletionReason::FailedToUnwrapKey,
            "StoragePartitionCleared" => {
                TerminationEventDetailsDeletionReason::StoragePartitionCleared
            }
            "ClearBrowsingData" => TerminationEventDetailsDeletionReason::ClearBrowsingData,
            "ServerRequested" => TerminationEventDetailsDeletionReason::ServerRequested,
            "InvalidSessionParams" => TerminationEventDetailsDeletionReason::InvalidSessionParams,
            "RefreshFatalError" => TerminationEventDetailsDeletionReason::RefreshFatalError,
            "DevTools" => TerminationEventDetailsDeletionReason::DevTools,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for TerminationEventDetailsDeletionReason {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TerminationEventDetailsDeletionReason {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TerminationEventDetailsDeletionReason;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Expired" => TerminationEventDetailsDeletionReason::Expired,
                    "FailedToRestoreKey" => {
                        TerminationEventDetailsDeletionReason::FailedToRestoreKey
                    }
                    "FailedToUnwrapKey" => TerminationEventDetailsDeletionReason::FailedToUnwrapKey,
                    "StoragePartitionCleared" => {
                        TerminationEventDetailsDeletionReason::StoragePartitionCleared
                    }
                    "ClearBrowsingData" => TerminationEventDetailsDeletionReason::ClearBrowsingData,
                    "ServerRequested" => TerminationEventDetailsDeletionReason::ServerRequested,
                    "InvalidSessionParams" => {
                        TerminationEventDetailsDeletionReason::InvalidSessionParams
                    }
                    "RefreshFatalError" => TerminationEventDetailsDeletionReason::RefreshFatalError,
                    "DevTools" => TerminationEventDetailsDeletionReason::DevTools,
                    _ => TerminationEventDetailsDeletionReason::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(TerminationEventDetailsDeletionReason::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.TerminationEventDetails。
///
///协议原文：
///
///<pre>Session event details specific to termination.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TerminationEventDetails {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.TerminationEventDetails/properties/deletionReason。
    ///
    ///协议原文：
    ///
    ///<pre>The reason for a session being deleted.</pre>
    ///
    ///
    #[serde(rename = "deletionReason")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub deletion_reason: TerminationEventDetailsDeletionReason,
}
impl<'de> serde::Deserialize<'de> for TerminationEventDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TerminationEventDetails;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.TerminationEventDetails/properties/deletionReason。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The reason for a session being deleted.</pre>
                    ///
                    ///
                    #[serde(rename = "deletionReason")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub deletion_reason: TerminationEventDetailsDeletionReason,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TerminationEventDetails {
                    deletion_reason: fields.deletion_reason,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///命名协议值；保持独立 Rust 类型身份，Clone 按底层值复制。协议来源：Network.type.TimeSinceEpoch。
///
///协议原文：
///
///<pre>UTC time in seconds, counted from January 1, 1970.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct TimeSinceEpoch(ydp_core::FiniteF64);
impl TimeSinceEpoch {
    ///消耗底层值构造命名包装，不新增验证或额外分配。
    ///
    ///```
    ///let value: ydp::network::types::TimeSinceEpoch = serde_json::from_str("0.0").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::network::types::TimeSinceEpoch::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("0.0").unwrap());
    ///```
    pub fn new(value: ydp_core::FiniteF64) -> Self {
        Self(value)
    }
    ///消耗包装并移出底层拥有型值，不复制数据。
    ///
    ///```
    ///let value: ydp::network::types::TimeSinceEpoch = serde_json::from_str("0.0").unwrap();
    ///let inner = value.into_inner();
    ///let restored = ydp::network::types::TimeSinceEpoch::new(inner);
    ///assert_eq!(serde_json::to_value(restored).unwrap(), serde_json::from_str::<serde_json::Value>("0.0").unwrap());
    ///```
    pub fn into_inner(self) -> ydp_core::FiniteF64 {
        self.0
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.TrustTokenOperationType。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TrustTokenOperationType {
    ///协议值 "Issuance"。
    Issuance,
    ///协议值 "Redemption"。
    Redemption,
    ///协议值 "Signing"。
    Signing,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl TrustTokenOperationType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::TrustTokenOperationType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Issuance => "Issuance",
            Self::Redemption => "Redemption",
            Self::Signing => "Signing",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for TrustTokenOperationType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for TrustTokenOperationType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Issuance" => TrustTokenOperationType::Issuance,
            "Redemption" => TrustTokenOperationType::Redemption,
            "Signing" => TrustTokenOperationType::Signing,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for TrustTokenOperationType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TrustTokenOperationType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TrustTokenOperationType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "Issuance" => TrustTokenOperationType::Issuance,
                    "Redemption" => TrustTokenOperationType::Redemption,
                    "Signing" => TrustTokenOperationType::Signing,
                    _ => TrustTokenOperationType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(TrustTokenOperationType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Network.type.TrustTokenParams/properties/refreshPolicy。
///
///协议原文：
///
///<pre>Only set for "token-redemption" operation and determine whether
///to request a fresh SRR or use a still valid cached SRR.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TrustTokenParamsRefreshPolicy {
    ///协议值 "UseCached"。
    UseCached,
    ///协议值 "Refresh"。
    Refresh,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl TrustTokenParamsRefreshPolicy {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::network::types::TrustTokenParamsRefreshPolicy::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::UseCached => "UseCached",
            Self::Refresh => "Refresh",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for TrustTokenParamsRefreshPolicy {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for TrustTokenParamsRefreshPolicy {
    fn from(value: String) -> Self {
        match value.as_str() {
            "UseCached" => TrustTokenParamsRefreshPolicy::UseCached,
            "Refresh" => TrustTokenParamsRefreshPolicy::Refresh,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for TrustTokenParamsRefreshPolicy {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TrustTokenParamsRefreshPolicy {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TrustTokenParamsRefreshPolicy;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "UseCached" => TrustTokenParamsRefreshPolicy::UseCached,
                    "Refresh" => TrustTokenParamsRefreshPolicy::Refresh,
                    _ => TrustTokenParamsRefreshPolicy::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(TrustTokenParamsRefreshPolicy::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.TrustTokenParams。
///
///协议原文：
///
///<pre>Determines what type of Trust Token operation is executed and
///depending on the type, some additional parameters. The values
///are specified in third_party/blink/renderer/core/fetch/trust_token.idl.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, serde::Serialize)]
pub struct TrustTokenParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.TrustTokenParams/properties/operation。
    #[serde(rename = "operation")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub operation: crate::network::types::TrustTokenOperationType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.TrustTokenParams/properties/refreshPolicy。
    ///
    ///协议原文：
    ///
    ///<pre>Only set for "token-redemption" operation and determine whether
    ///to request a fresh SRR or use a still valid cached SRR.</pre>
    ///
    ///
    #[serde(rename = "refreshPolicy")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub refresh_policy: TrustTokenParamsRefreshPolicy,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.TrustTokenParams/properties/issuers。
    ///
    ///协议原文：
    ///
    ///<pre>Origins of issuers from whom to request tokens or redemption
    ///records.</pre>
    ///
    ///
    #[serde(rename = "issuers")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub issuers: Option<Vec<String>>,
}
impl<'de> serde::Deserialize<'de> for TrustTokenParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TrustTokenParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.TrustTokenParams/properties/operation。
                    #[serde(rename = "operation")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub operation: crate::network::types::TrustTokenOperationType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.TrustTokenParams/properties/refreshPolicy。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Only set for "token-redemption" operation and determine whether
                    ///to request a fresh SRR or use a still valid cached SRR.</pre>
                    ///
                    ///
                    #[serde(rename = "refreshPolicy")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub refresh_policy: TrustTokenParamsRefreshPolicy,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.TrustTokenParams/properties/issuers。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Origins of issuers from whom to request tokens or redemption
                    ///records.</pre>
                    ///
                    ///
                    #[serde(rename = "issuers")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub issuers: Option<Vec<String>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(TrustTokenParams {
                    operation: fields.operation,
                    refresh_policy: fields.refresh_policy,
                    issuers: fields.issuers,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.WebSocketFrame。
///
///协议原文：
///
///<pre>WebSocket message data. This represents an entire WebSocket message, not just a fragmented frame as the name suggests.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct WebSocketFrame {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.WebSocketFrame/properties/opcode。
    ///
    ///协议原文：
    ///
    ///<pre>WebSocket message opcode.</pre>
    ///
    ///
    #[serde(rename = "opcode")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub opcode: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.WebSocketFrame/properties/mask。
    ///
    ///协议原文：
    ///
    ///<pre>WebSocket message mask.</pre>
    ///
    ///
    #[serde(rename = "mask")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub mask: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.WebSocketFrame/properties/payloadData。
    ///
    ///协议原文：
    ///
    ///<pre>WebSocket message payload data.
    ///If the opcode is 1, this is a text message and payloadData is a UTF-8 string.
    ///If the opcode isn't 1, then payloadData is a base64 encoded string representing binary data.</pre>
    ///
    ///
    #[serde(rename = "payloadData")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub payload_data: String,
}
impl<'de> serde::Deserialize<'de> for WebSocketFrame {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WebSocketFrame;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.WebSocketFrame/properties/opcode。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>WebSocket message opcode.</pre>
                    ///
                    ///
                    #[serde(rename = "opcode")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub opcode: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.WebSocketFrame/properties/mask。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>WebSocket message mask.</pre>
                    ///
                    ///
                    #[serde(rename = "mask")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub mask: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.WebSocketFrame/properties/payloadData。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>WebSocket message payload data.
                    ///If the opcode is 1, this is a text message and payloadData is a UTF-8 string.
                    ///If the opcode isn't 1, then payloadData is a base64 encoded string representing binary data.</pre>
                    ///
                    ///
                    #[serde(rename = "payloadData")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub payload_data: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(WebSocketFrame {
                    opcode: fields.opcode,
                    mask: fields.mask,
                    payload_data: fields.payload_data,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.WebSocketRequest。
///
///协议原文：
///
///<pre>WebSocket request data.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct WebSocketRequest {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.WebSocketRequest/properties/headers。
    ///
    ///协议原文：
    ///
    ///<pre>HTTP request headers.</pre>
    ///
    ///
    #[serde(rename = "headers")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub headers: crate::network::types::Headers,
}
impl<'de> serde::Deserialize<'de> for WebSocketRequest {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WebSocketRequest;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.WebSocketRequest/properties/headers。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>HTTP request headers.</pre>
                    ///
                    ///
                    #[serde(rename = "headers")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub headers: crate::network::types::Headers,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(WebSocketRequest {
                    headers: fields.headers,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Network.type.WebSocketResponse。
///
///协议原文：
///
///<pre>WebSocket response data.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct WebSocketResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.WebSocketResponse/properties/status。
    ///
    ///协议原文：
    ///
    ///<pre>HTTP response status code.</pre>
    ///
    ///
    #[serde(rename = "status")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub status: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.WebSocketResponse/properties/statusText。
    ///
    ///协议原文：
    ///
    ///<pre>HTTP response status text.</pre>
    ///
    ///
    #[serde(rename = "statusText")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub status_text: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.WebSocketResponse/properties/headers。
    ///
    ///协议原文：
    ///
    ///<pre>HTTP response headers.</pre>
    ///
    ///
    #[serde(rename = "headers")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub headers: crate::network::types::Headers,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.WebSocketResponse/properties/headersText。
    ///
    ///协议原文：
    ///
    ///<pre>HTTP response headers text.</pre>
    ///
    ///
    #[serde(rename = "headersText")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub headers_text: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.WebSocketResponse/properties/requestHeaders。
    ///
    ///协议原文：
    ///
    ///<pre>HTTP request headers.</pre>
    ///
    ///
    #[serde(rename = "requestHeaders")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub request_headers: Option<crate::network::types::Headers>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.WebSocketResponse/properties/requestHeadersText。
    ///
    ///协议原文：
    ///
    ///<pre>HTTP request headers text.</pre>
    ///
    ///
    #[serde(rename = "requestHeadersText")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub request_headers_text: Option<String>,
}
impl<'de> serde::Deserialize<'de> for WebSocketResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WebSocketResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.WebSocketResponse/properties/status。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>HTTP response status code.</pre>
                    ///
                    ///
                    #[serde(rename = "status")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub status: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.WebSocketResponse/properties/statusText。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>HTTP response status text.</pre>
                    ///
                    ///
                    #[serde(rename = "statusText")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub status_text: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Network.type.WebSocketResponse/properties/headers。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>HTTP response headers.</pre>
                    ///
                    ///
                    #[serde(rename = "headers")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub headers: crate::network::types::Headers,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.WebSocketResponse/properties/headersText。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>HTTP response headers text.</pre>
                    ///
                    ///
                    #[serde(rename = "headersText")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub headers_text: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.WebSocketResponse/properties/requestHeaders。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>HTTP request headers.</pre>
                    ///
                    ///
                    #[serde(rename = "requestHeaders")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub request_headers: Option<crate::network::types::Headers>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Network.type.WebSocketResponse/properties/requestHeadersText。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>HTTP request headers text.</pre>
                    ///
                    ///
                    #[serde(rename = "requestHeadersText")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub request_headers_text: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(WebSocketResponse {
                    status: fields.status,
                    status_text: fields.status_text,
                    headers: fields.headers,
                    headers_text: fields.headers_text,
                    request_headers: fields.request_headers,
                    request_headers_text: fields.request_headers_text,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
