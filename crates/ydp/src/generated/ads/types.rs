///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Ads.type.AdFrameData。
///
///协议原文：
///
///<pre>Ad frame data.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct AdFrameData {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.type.AdFrameData/properties/frameId。
    ///
    ///协议原文：
    ///
    ///<pre>The DevTools frame token.</pre>
    ///
    ///
    #[serde(rename = "frameId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub frame_id: crate::page::types::FrameId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Ads.type.AdFrameData/properties/initialOrigin。
    ///
    ///协议原文：
    ///
    ///<pre>The initial origin of the frame. To minimize the payload size, this is
    ///only sent once per frame.</pre>
    ///
    ///
    #[serde(rename = "initialOrigin")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_origin: Option<String>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.type.AdFrameData/properties/networkBytes。
    ///
    ///协议原文：
    ///
    ///<pre>The network bytes of the frame.</pre>
    ///
    ///
    #[serde(rename = "networkBytes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub network_bytes: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.type.AdFrameData/properties/cpuTime。
    ///
    ///协议原文：
    ///
    ///<pre>The CPU time of the frame, in milliseconds.</pre>
    ///
    ///
    #[serde(rename = "cpuTime")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub cpu_time: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for AdFrameData {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AdFrameData;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.type.AdFrameData/properties/frameId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The DevTools frame token.</pre>
                    ///
                    ///
                    #[serde(rename = "frameId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub frame_id: crate::page::types::FrameId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Ads.type.AdFrameData/properties/initialOrigin。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The initial origin of the frame. To minimize the payload size, this is
                    ///only sent once per frame.</pre>
                    ///
                    ///
                    #[serde(rename = "initialOrigin")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub initial_origin: Option<String>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.type.AdFrameData/properties/networkBytes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The network bytes of the frame.</pre>
                    ///
                    ///
                    #[serde(rename = "networkBytes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub network_bytes: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.type.AdFrameData/properties/cpuTime。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The CPU time of the frame, in milliseconds.</pre>
                    ///
                    ///
                    #[serde(rename = "cpuTime")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub cpu_time: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AdFrameData {
                    frame_id: fields.frame_id,
                    initial_origin: fields.initial_origin,
                    network_bytes: fields.network_bytes,
                    cpu_time: fields.cpu_time,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Ads.type.AdMetrics。
///
///协议原文：
///
///<pre>Ad metrics for a page.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct AdMetrics {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.type.AdMetrics/properties/viewportAdDensityByArea。
    ///
    ///协议原文：
    ///
    ///<pre>The viewport ad density by area, represented as a percentage (an integer
    ///between 0 and 100).</pre>
    ///
    ///
    #[serde(rename = "viewportAdDensityByArea")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub viewport_ad_density_by_area: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.type.AdMetrics/properties/averageViewportAdDensityByArea。
    ///
    ///协议原文：
    ///
    ///<pre>The time-weighted average of the viewport ad density by area, measured
    ///across the duration of the page.</pre>
    ///
    ///
    #[serde(rename = "averageViewportAdDensityByArea")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub average_viewport_ad_density_by_area: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.type.AdMetrics/properties/viewportAdCount。
    ///
    ///协议原文：
    ///
    ///<pre>The number of ads currently visible within the viewport.</pre>
    ///
    ///
    #[serde(rename = "viewportAdCount")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub viewport_ad_count: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.type.AdMetrics/properties/averageViewportAdCount。
    ///
    ///协议原文：
    ///
    ///<pre>The time-weighted average of the viewport ad count, measured across the
    ///duration of the page.</pre>
    ///
    ///
    #[serde(rename = "averageViewportAdCount")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub average_viewport_ad_count: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.type.AdMetrics/properties/totalAdCpuTime。
    ///
    ///协议原文：
    ///
    ///<pre>The total ad CPU usage, in milliseconds.</pre>
    ///
    ///
    #[serde(rename = "totalAdCpuTime")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub total_ad_cpu_time: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.type.AdMetrics/properties/totalAdNetworkBytes。
    ///
    ///协议原文：
    ///
    ///<pre>The total ad network bytes.</pre>
    ///
    ///
    #[serde(rename = "totalAdNetworkBytes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub total_ad_network_bytes: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.type.AdMetrics/properties/updateAdFrames。
    ///
    ///协议原文：
    ///
    ///<pre>The list of ad frames that have been updated since the last event.</pre>
    ///
    ///
    #[serde(rename = "updateAdFrames")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub update_ad_frames: Vec<crate::ads::types::AdFrameData>,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.type.AdMetrics/properties/removeAdFrames。
    ///
    ///协议原文：
    ///
    ///<pre>The list of ad frame IDs that have been removed since the last event.</pre>
    ///
    ///
    #[serde(rename = "removeAdFrames")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub remove_ad_frames: Vec<crate::page::types::FrameId>,
}
impl<'de> serde::Deserialize<'de> for AdMetrics {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AdMetrics;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.type.AdMetrics/properties/viewportAdDensityByArea。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The viewport ad density by area, represented as a percentage (an integer
                    ///between 0 and 100).</pre>
                    ///
                    ///
                    #[serde(rename = "viewportAdDensityByArea")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub viewport_ad_density_by_area: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.type.AdMetrics/properties/averageViewportAdDensityByArea。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The time-weighted average of the viewport ad density by area, measured
                    ///across the duration of the page.</pre>
                    ///
                    ///
                    #[serde(rename = "averageViewportAdDensityByArea")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub average_viewport_ad_density_by_area: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.type.AdMetrics/properties/viewportAdCount。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The number of ads currently visible within the viewport.</pre>
                    ///
                    ///
                    #[serde(rename = "viewportAdCount")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub viewport_ad_count: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.type.AdMetrics/properties/averageViewportAdCount。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The time-weighted average of the viewport ad count, measured across the
                    ///duration of the page.</pre>
                    ///
                    ///
                    #[serde(rename = "averageViewportAdCount")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub average_viewport_ad_count: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.type.AdMetrics/properties/totalAdCpuTime。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The total ad CPU usage, in milliseconds.</pre>
                    ///
                    ///
                    #[serde(rename = "totalAdCpuTime")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub total_ad_cpu_time: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.type.AdMetrics/properties/totalAdNetworkBytes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The total ad network bytes.</pre>
                    ///
                    ///
                    #[serde(rename = "totalAdNetworkBytes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub total_ad_network_bytes: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.type.AdMetrics/properties/updateAdFrames。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The list of ad frames that have been updated since the last event.</pre>
                    ///
                    ///
                    #[serde(rename = "updateAdFrames")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub update_ad_frames: Vec<crate::ads::types::AdFrameData>,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.type.AdMetrics/properties/removeAdFrames。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The list of ad frame IDs that have been removed since the last event.</pre>
                    ///
                    ///
                    #[serde(rename = "removeAdFrames")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub remove_ad_frames: Vec<crate::page::types::FrameId>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AdMetrics {
                    viewport_ad_density_by_area: fields.viewport_ad_density_by_area,
                    average_viewport_ad_density_by_area: fields.average_viewport_ad_density_by_area,
                    viewport_ad_count: fields.viewport_ad_count,
                    average_viewport_ad_count: fields.average_viewport_ad_count,
                    total_ad_cpu_time: fields.total_ad_cpu_time,
                    total_ad_network_bytes: fields.total_ad_network_bytes,
                    update_ad_frames: fields.update_ad_frames,
                    remove_ad_frames: fields.remove_ad_frames,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Ads.type.AdScript。
///
///协议原文：
///
///<pre>An ad script.
///Note: when the script is a transitive ad script, we only fill in the
///immediate ancestor script in the provenance's adScriptAncestry field (as its
///first entry), rather than filling in the full ancestry. This saves work for
///the backend, and the frontend can reconstruct the full ancestry if
///necessary.</pre>
///
///
#[derive(Debug, Clone, serde::Serialize)]
pub struct AdScript {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.type.AdScript/properties/scriptId。
    ///
    ///协议原文：
    ///
    ///<pre>The script ID.</pre>
    ///
    ///
    #[serde(rename = "scriptId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub script_id: crate::runtime::types::ScriptId,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.type.AdScript/properties/provenance。
    ///
    ///协议原文：
    ///
    ///<pre>The ad provenance.</pre>
    ///
    ///
    #[serde(rename = "provenance")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub provenance: crate::network::types::AdProvenance,
}
impl<'de> serde::Deserialize<'de> for AdScript {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AdScript;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.type.AdScript/properties/scriptId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The script ID.</pre>
                    ///
                    ///
                    #[serde(rename = "scriptId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub script_id: crate::runtime::types::ScriptId,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Ads.type.AdScript/properties/provenance。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The ad provenance.</pre>
                    ///
                    ///
                    #[serde(rename = "provenance")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub provenance: crate::network::types::AdProvenance,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AdScript {
                    script_id: fields.script_id,
                    provenance: fields.provenance,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
