///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Performance.command.disable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Performance.command.disable。
///
///协议原文：
///
///<pre>Disable collecting and reporting metrics.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Performance.command.disable/returns。
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
    const METHOD: &'static str = "Performance.disable";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Performance.command.enable/parameters/timeDomain。
///
///协议原文：
///
///<pre>Time domain to use for collecting and reporting duration metrics.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum EnableParamsTimeDomain {
    ///协议值 "timeTicks"。
    TimeTicks,
    ///协议值 "threadTicks"。
    ThreadTicks,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl EnableParamsTimeDomain {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::performance::commands::EnableParamsTimeDomain::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::TimeTicks => "timeTicks",
            Self::ThreadTicks => "threadTicks",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for EnableParamsTimeDomain {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for EnableParamsTimeDomain {
    fn from(value: String) -> Self {
        match value.as_str() {
            "timeTicks" => EnableParamsTimeDomain::TimeTicks,
            "threadTicks" => EnableParamsTimeDomain::ThreadTicks,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for EnableParamsTimeDomain {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for EnableParamsTimeDomain {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EnableParamsTimeDomain;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "timeTicks" => EnableParamsTimeDomain::TimeTicks,
                    "threadTicks" => EnableParamsTimeDomain::ThreadTicks,
                    _ => EnableParamsTimeDomain::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(EnableParamsTimeDomain::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Performance.command.enable/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct EnableParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Performance.command.enable/parameters/timeDomain。
    ///
    ///协议原文：
    ///
    ///<pre>Time domain to use for collecting and reporting duration metrics.</pre>
    ///
    ///
    #[serde(rename = "timeDomain")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub time_domain: Option<EnableParamsTimeDomain>,
}
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
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Performance.command.enable/parameters/timeDomain。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Time domain to use for collecting and reporting duration metrics.</pre>
                    ///
                    ///
                    #[serde(rename = "timeDomain")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub time_domain: Option<EnableParamsTimeDomain>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EnableParams {
                    time_domain: fields.time_domain,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Performance.command.enable。
///
///协议原文：
///
///<pre>Enable collecting and reporting metrics.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Performance.command.enable/returns。
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
    const METHOD: &'static str = "Performance.enable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Performance.command.getMetrics/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetMetricsParams {}
impl<'de> serde::Deserialize<'de> for GetMetricsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetMetricsParams;
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
                Ok(GetMetricsParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Performance.command.getMetrics。
///
///协议原文：
///
///<pre>Retrieve current values of run-time metrics.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetMetrics;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Performance.command.getMetrics/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetMetricsResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Performance.command.getMetrics/returns/metrics。
    ///
    ///协议原文：
    ///
    ///<pre>Current values for run-time metrics.</pre>
    ///
    ///
    #[serde(rename = "metrics")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub metrics: Vec<crate::performance::types::Metric>,
}
impl<'de> serde::Deserialize<'de> for GetMetricsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetMetricsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Performance.command.getMetrics/returns/metrics。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Current values for run-time metrics.</pre>
                    ///
                    ///
                    #[serde(rename = "metrics")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub metrics: Vec<crate::performance::types::Metric>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetMetricsResponse {
                    metrics: fields.metrics,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetMetrics {
    type Params = GetMetricsParams;
    type Response = GetMetricsResponse;
    const METHOD: &'static str = "Performance.getMetrics";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Performance.command.setTimeDomain/parameters/timeDomain。
///
///协议原文：
///
///<pre>Time domain</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SetTimeDomainParamsTimeDomain {
    ///协议值 "timeTicks"。
    TimeTicks,
    ///协议值 "threadTicks"。
    ThreadTicks,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl SetTimeDomainParamsTimeDomain {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::performance::commands::SetTimeDomainParamsTimeDomain::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::TimeTicks => "timeTicks",
            Self::ThreadTicks => "threadTicks",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for SetTimeDomainParamsTimeDomain {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for SetTimeDomainParamsTimeDomain {
    fn from(value: String) -> Self {
        match value.as_str() {
            "timeTicks" => SetTimeDomainParamsTimeDomain::TimeTicks,
            "threadTicks" => SetTimeDomainParamsTimeDomain::ThreadTicks,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for SetTimeDomainParamsTimeDomain {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetTimeDomainParamsTimeDomain {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetTimeDomainParamsTimeDomain;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "timeTicks" => SetTimeDomainParamsTimeDomain::TimeTicks,
                    "threadTicks" => SetTimeDomainParamsTimeDomain::ThreadTicks,
                    _ => SetTimeDomainParamsTimeDomain::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(SetTimeDomainParamsTimeDomain::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Performance.command.setTimeDomain/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetTimeDomainParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Performance.command.setTimeDomain/parameters/timeDomain。
    ///
    ///协议原文：
    ///
    ///<pre>Time domain</pre>
    ///
    ///
    #[serde(rename = "timeDomain")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub time_domain: SetTimeDomainParamsTimeDomain,
}
impl<'de> serde::Deserialize<'de> for SetTimeDomainParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetTimeDomainParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Performance.command.setTimeDomain/parameters/timeDomain。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Time domain</pre>
                    ///
                    ///
                    #[serde(rename = "timeDomain")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub time_domain: SetTimeDomainParamsTimeDomain,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetTimeDomainParams {
                    time_domain: fields.time_domain,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Performance.command.setTimeDomain。
///
///协议原文：
///
///<pre>Sets time domain to use for collecting and reporting duration metrics.
///Note that this must be called before enabling metrics collection. Calling
///this method while metrics collection is enabled returns an error.</pre>
///
///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct SetTimeDomain;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Performance.command.setTimeDomain/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetTimeDomainResponse {}
impl<'de> serde::Deserialize<'de> for SetTimeDomainResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetTimeDomainResponse;
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
                Ok(SetTimeDomainResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetTimeDomain {
    type Params = SetTimeDomainParams;
    type Response = SetTimeDomainResponse;
    const METHOD: &'static str = "Performance.setTimeDomain";
}
