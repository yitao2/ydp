///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DeviceOrientation.command.clearDeviceOrientationOverride/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ClearDeviceOrientationOverrideParams {}
impl<'de> serde::Deserialize<'de> for ClearDeviceOrientationOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearDeviceOrientationOverrideParams;
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
                Ok(ClearDeviceOrientationOverrideParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DeviceOrientation.command.clearDeviceOrientationOverride。
///
///协议原文：
///
///<pre>Clears the overridden Device Orientation.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ClearDeviceOrientationOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DeviceOrientation.command.clearDeviceOrientationOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearDeviceOrientationOverrideResponse {}
impl<'de> serde::Deserialize<'de> for ClearDeviceOrientationOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearDeviceOrientationOverrideResponse;
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
                Ok(ClearDeviceOrientationOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ClearDeviceOrientationOverride {
    type Params = ClearDeviceOrientationOverrideParams;
    type Response = ClearDeviceOrientationOverrideResponse;
    const METHOD: &'static str = "DeviceOrientation.clearDeviceOrientationOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DeviceOrientation.command.setDeviceOrientationOverride/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDeviceOrientationOverrideParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DeviceOrientation.command.setDeviceOrientationOverride/parameters/alpha。
    ///
    ///协议原文：
    ///
    ///<pre>Mock alpha</pre>
    ///
    ///
    #[serde(rename = "alpha")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub alpha: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DeviceOrientation.command.setDeviceOrientationOverride/parameters/beta。
    ///
    ///协议原文：
    ///
    ///<pre>Mock beta</pre>
    ///
    ///
    #[serde(rename = "beta")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub beta: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：DeviceOrientation.command.setDeviceOrientationOverride/parameters/gamma。
    ///
    ///协议原文：
    ///
    ///<pre>Mock gamma</pre>
    ///
    ///
    #[serde(rename = "gamma")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub gamma: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for SetDeviceOrientationOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDeviceOrientationOverrideParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DeviceOrientation.command.setDeviceOrientationOverride/parameters/alpha。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Mock alpha</pre>
                    ///
                    ///
                    #[serde(rename = "alpha")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub alpha: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DeviceOrientation.command.setDeviceOrientationOverride/parameters/beta。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Mock beta</pre>
                    ///
                    ///
                    #[serde(rename = "beta")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub beta: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：DeviceOrientation.command.setDeviceOrientationOverride/parameters/gamma。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Mock gamma</pre>
                    ///
                    ///
                    #[serde(rename = "gamma")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub gamma: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetDeviceOrientationOverrideParams {
                    alpha: fields.alpha,
                    beta: fields.beta,
                    gamma: fields.gamma,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：DeviceOrientation.command.setDeviceOrientationOverride。
///
///协议原文：
///
///<pre>Overrides the Device Orientation.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetDeviceOrientationOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：DeviceOrientation.command.setDeviceOrientationOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDeviceOrientationOverrideResponse {}
impl<'de> serde::Deserialize<'de> for SetDeviceOrientationOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDeviceOrientationOverrideResponse;
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
                Ok(SetDeviceOrientationOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetDeviceOrientationOverride {
    type Params = SetDeviceOrientationOverrideParams;
    type Response = SetDeviceOrientationOverrideResponse;
    const METHOD: &'static str = "DeviceOrientation.setDeviceOrientationOverride";
}
