///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SystemInfo.command.getFeatureState/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetFeatureStateParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.command.getFeatureState/parameters/featureState。
    #[serde(rename = "featureState")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub feature_state: String,
}
impl<'de> serde::Deserialize<'de> for GetFeatureStateParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetFeatureStateParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.command.getFeatureState/parameters/featureState。
                    #[serde(rename = "featureState")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub feature_state: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetFeatureStateParams {
                    feature_state: fields.feature_state,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SystemInfo.command.getFeatureState。
///
///协议原文：
///
///<pre>Returns information about the feature state.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetFeatureState;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SystemInfo.command.getFeatureState/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetFeatureStateResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.command.getFeatureState/returns/featureEnabled。
    #[serde(rename = "featureEnabled")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub feature_enabled: bool,
}
impl<'de> serde::Deserialize<'de> for GetFeatureStateResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetFeatureStateResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.command.getFeatureState/returns/featureEnabled。
                    #[serde(rename = "featureEnabled")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub feature_enabled: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetFeatureStateResponse {
                    feature_enabled: fields.feature_enabled,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetFeatureState {
    type Params = GetFeatureStateParams;
    type Response = GetFeatureStateResponse;
    const METHOD: &'static str = "SystemInfo.getFeatureState";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SystemInfo.command.getInfo/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetInfoParams {}
impl<'de> serde::Deserialize<'de> for GetInfoParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetInfoParams;
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
                Ok(GetInfoParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SystemInfo.command.getInfo。
///
///协议原文：
///
///<pre>Returns information about the system.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetInfo;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SystemInfo.command.getInfo/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetInfoResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.command.getInfo/returns/gpu。
    ///
    ///协议原文：
    ///
    ///<pre>Information about the GPUs on the system.</pre>
    ///
    ///
    #[serde(rename = "gpu")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub gpu: crate::system_info::types::GpuInfo,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.command.getInfo/returns/modelName。
    ///
    ///协议原文：
    ///
    ///<pre>A platform-dependent description of the model of the machine. On Mac OS, this is, for
    ///example, 'MacBookPro'. Will be the empty string if not supported.</pre>
    ///
    ///
    #[serde(rename = "modelName")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub model_name: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.command.getInfo/returns/modelVersion。
    ///
    ///协议原文：
    ///
    ///<pre>A platform-dependent description of the version of the machine. On Mac OS, this is, for
    ///example, '10.1'. Will be the empty string if not supported.</pre>
    ///
    ///
    #[serde(rename = "modelVersion")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub model_version: String,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.command.getInfo/returns/commandLine。
    ///
    ///协议原文：
    ///
    ///<pre>The command line string used to launch the browser. Will be the empty string if not
    ///supported.</pre>
    ///
    ///
    #[serde(rename = "commandLine")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub command_line: String,
}
impl<'de> serde::Deserialize<'de> for GetInfoResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetInfoResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.command.getInfo/returns/gpu。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Information about the GPUs on the system.</pre>
                    ///
                    ///
                    #[serde(rename = "gpu")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub gpu: crate::system_info::types::GpuInfo,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.command.getInfo/returns/modelName。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A platform-dependent description of the model of the machine. On Mac OS, this is, for
                    ///example, 'MacBookPro'. Will be the empty string if not supported.</pre>
                    ///
                    ///
                    #[serde(rename = "modelName")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub model_name: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.command.getInfo/returns/modelVersion。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>A platform-dependent description of the version of the machine. On Mac OS, this is, for
                    ///example, '10.1'. Will be the empty string if not supported.</pre>
                    ///
                    ///
                    #[serde(rename = "modelVersion")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub model_version: String,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.command.getInfo/returns/commandLine。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The command line string used to launch the browser. Will be the empty string if not
                    ///supported.</pre>
                    ///
                    ///
                    #[serde(rename = "commandLine")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub command_line: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetInfoResponse {
                    gpu: fields.gpu,
                    model_name: fields.model_name,
                    model_version: fields.model_version,
                    command_line: fields.command_line,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetInfo {
    type Params = GetInfoParams;
    type Response = GetInfoResponse;
    const METHOD: &'static str = "SystemInfo.getInfo";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SystemInfo.command.getProcessInfo/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetProcessInfoParams {}
impl<'de> serde::Deserialize<'de> for GetProcessInfoParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetProcessInfoParams;
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
                Ok(GetProcessInfoParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：SystemInfo.command.getProcessInfo。
///
///协议原文：
///
///<pre>Returns information about all running processes.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct GetProcessInfo;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：SystemInfo.command.getProcessInfo/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetProcessInfoResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.command.getProcessInfo/returns/processInfo。
    ///
    ///协议原文：
    ///
    ///<pre>An array of process info blocks.</pre>
    ///
    ///
    #[serde(rename = "processInfo")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub process_info: Vec<crate::system_info::types::ProcessInfo>,
}
impl<'de> serde::Deserialize<'de> for GetProcessInfoResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetProcessInfoResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：SystemInfo.command.getProcessInfo/returns/processInfo。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>An array of process info blocks.</pre>
                    ///
                    ///
                    #[serde(rename = "processInfo")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub process_info: Vec<crate::system_info::types::ProcessInfo>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetProcessInfoResponse {
                    process_info: fields.process_info,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetProcessInfo {
    type Params = GetProcessInfoParams;
    type Response = GetProcessInfoResponse;
    const METHOD: &'static str = "SystemInfo.getProcessInfo";
}
