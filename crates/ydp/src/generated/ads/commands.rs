///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Ads.command.getAdMetrics/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetAdMetricsParams {}
impl<'de> serde::Deserialize<'de> for GetAdMetricsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetAdMetricsParams;
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
                Ok(GetAdMetricsParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Ads.command.getAdMetrics。
///
///协议原文：
///
///<pre>Retrieves ad metrics for the current page.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetAdMetrics;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Ads.command.getAdMetrics/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetAdMetricsResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.command.getAdMetrics/returns/metrics。
    #[serde(rename = "metrics")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub metrics: crate::ads::types::AdMetrics,
}
impl<'de> serde::Deserialize<'de> for GetAdMetricsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetAdMetricsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.command.getAdMetrics/returns/metrics。
                    #[serde(rename = "metrics")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub metrics: crate::ads::types::AdMetrics,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetAdMetricsResponse {
                    metrics: fields.metrics,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetAdMetrics {
    type Params = GetAdMetricsParams;
    type Response = GetAdMetricsResponse;
    const METHOD: &'static str = "Ads.getAdMetrics";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Ads.command.getAdScripts/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetAdScriptsParams {}
impl<'de> serde::Deserialize<'de> for GetAdScriptsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetAdScriptsParams;
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
                Ok(GetAdScriptsParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Ads.command.getAdScripts。
///
///协议原文：
///
///<pre>Retrieves ad scripts for the current page. To minimize payload size, this
///only returns the newly tracked ad scripts since the last call to
///getAdScripts (i.e., the delta).</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetAdScripts;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Ads.command.getAdScripts/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetAdScriptsResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.command.getAdScripts/returns/newScripts。
    #[serde(rename = "newScripts")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub new_scripts: Vec<crate::ads::types::AdScript>,
}
impl<'de> serde::Deserialize<'de> for GetAdScriptsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetAdScriptsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.command.getAdScripts/returns/newScripts。
                    #[serde(rename = "newScripts")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub new_scripts: Vec<crate::ads::types::AdScript>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetAdScriptsResponse {
                    new_scripts: fields.new_scripts,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetAdScripts {
    type Params = GetAdScriptsParams;
    type Response = GetAdScriptsResponse;
    const METHOD: &'static str = "Ads.getAdScripts";
}
