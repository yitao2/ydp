///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.event.screenOrientationLockChanged/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ScreenOrientationLockChangedParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.event.screenOrientationLockChanged/parameters/locked。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the screen orientation is currently locked.</pre>
    ///
    ///
    #[serde(rename = "locked")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub locked: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.event.screenOrientationLockChanged/parameters/orientation。
    ///
    ///协议原文：
    ///
    ///<pre>The orientation lock type requested by the page. Only set when locked is true.</pre>
    ///
    ///
    #[serde(rename = "orientation")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub orientation: Option<crate::emulation::types::ScreenOrientation>,
}
impl<'de> serde::Deserialize<'de> for ScreenOrientationLockChangedParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ScreenOrientationLockChangedParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.event.screenOrientationLockChanged/parameters/locked。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the screen orientation is currently locked.</pre>
                    ///
                    ///
                    #[serde(rename = "locked")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub locked: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.event.screenOrientationLockChanged/parameters/orientation。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The orientation lock type requested by the page. Only set when locked is true.</pre>
                    ///
                    ///
                    #[serde(rename = "orientation")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub orientation: Option<crate::emulation::types::ScreenOrientation>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(ScreenOrientationLockChangedParams {
                    locked: fields.locked,
                    orientation: fields.orientation,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.event.screenOrientationLockChanged。
///
///协议原文：
///
///<pre>Fired when a page calls screen.orientation.lock() or screen.orientation.unlock()
///while device emulation is enabled. This allows the DevTools frontend to update the
///emulated device orientation accordingly.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ScreenOrientationLockChanged;
impl ydp_core::Event for ScreenOrientationLockChanged {
    type Params = ScreenOrientationLockChangedParams;
    const METHOD: &'static str = "Emulation.screenOrientationLockChanged";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.event.virtualTimeBudgetExpired/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct VirtualTimeBudgetExpiredParams {}
impl<'de> serde::Deserialize<'de> for VirtualTimeBudgetExpiredParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = VirtualTimeBudgetExpiredParams;
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
                Ok(VirtualTimeBudgetExpiredParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.event.virtualTimeBudgetExpired。
///
///协议原文：
///
///<pre>Notification sent after the virtual time budget for the current VirtualTimePolicy has run out.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct VirtualTimeBudgetExpired;
impl ydp_core::Event for VirtualTimeBudgetExpired {
    type Params = VirtualTimeBudgetExpiredParams;
    const METHOD: &'static str = "Emulation.virtualTimeBudgetExpired";
}
