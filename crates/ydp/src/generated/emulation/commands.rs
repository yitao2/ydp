///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.addScreen/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddScreenParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.addScreen/parameters/left。
    ///
    ///协议原文：
    ///
    ///<pre>Offset of the left edge of the screen in pixels.</pre>
    ///
    ///
    #[serde(rename = "left")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub left: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.addScreen/parameters/top。
    ///
    ///协议原文：
    ///
    ///<pre>Offset of the top edge of the screen in pixels.</pre>
    ///
    ///
    #[serde(rename = "top")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub top: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.addScreen/parameters/width。
    ///
    ///协议原文：
    ///
    ///<pre>The width of the screen in pixels.</pre>
    ///
    ///
    #[serde(rename = "width")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub width: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.addScreen/parameters/height。
    ///
    ///协议原文：
    ///
    ///<pre>The height of the screen in pixels.</pre>
    ///
    ///
    #[serde(rename = "height")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub height: i64,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.addScreen/parameters/workAreaInsets。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies the screen's work area. Default is entire screen.</pre>
    ///
    ///
    #[serde(rename = "workAreaInsets")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub work_area_insets: Option<crate::emulation::types::WorkAreaInsets>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.addScreen/parameters/devicePixelRatio。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies the screen's device pixel ratio. Default is 1.</pre>
    ///
    ///
    #[serde(rename = "devicePixelRatio")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub device_pixel_ratio: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.addScreen/parameters/rotation。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies the screen's rotation angle. Available values are 0, 90, 180 and 270. Default is 0.</pre>
    ///
    ///
    #[serde(rename = "rotation")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub rotation: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.addScreen/parameters/colorDepth。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies the screen's color depth in bits. Default is 24.</pre>
    ///
    ///
    #[serde(rename = "colorDepth")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub color_depth: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.addScreen/parameters/label。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies the descriptive label for the screen. Default is none.</pre>
    ///
    ///
    #[serde(rename = "label")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub label: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.addScreen/parameters/isInternal。
    ///
    ///协议原文：
    ///
    ///<pre>Indicates whether the screen is internal to the device or external, attached to the device. Default is false.</pre>
    ///
    ///
    #[serde(rename = "isInternal")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_internal: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for AddScreenParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddScreenParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.addScreen/parameters/left。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Offset of the left edge of the screen in pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "left")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub left: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.addScreen/parameters/top。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Offset of the top edge of the screen in pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "top")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub top: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.addScreen/parameters/width。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The width of the screen in pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "width")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub width: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.addScreen/parameters/height。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The height of the screen in pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "height")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub height: i64,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.addScreen/parameters/workAreaInsets。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies the screen's work area. Default is entire screen.</pre>
                    ///
                    ///
                    #[serde(rename = "workAreaInsets")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub work_area_insets: Option<crate::emulation::types::WorkAreaInsets>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.addScreen/parameters/devicePixelRatio。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies the screen's device pixel ratio. Default is 1.</pre>
                    ///
                    ///
                    #[serde(rename = "devicePixelRatio")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub device_pixel_ratio: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.addScreen/parameters/rotation。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies the screen's rotation angle. Available values are 0, 90, 180 and 270. Default is 0.</pre>
                    ///
                    ///
                    #[serde(rename = "rotation")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub rotation: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.addScreen/parameters/colorDepth。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies the screen's color depth in bits. Default is 24.</pre>
                    ///
                    ///
                    #[serde(rename = "colorDepth")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub color_depth: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.addScreen/parameters/label。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies the descriptive label for the screen. Default is none.</pre>
                    ///
                    ///
                    #[serde(rename = "label")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub label: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.addScreen/parameters/isInternal。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Indicates whether the screen is internal to the device or external, attached to the device. Default is false.</pre>
                    ///
                    ///
                    #[serde(rename = "isInternal")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub is_internal: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AddScreenParams {
                    left: fields.left,
                    top: fields.top,
                    width: fields.width,
                    height: fields.height,
                    work_area_insets: fields.work_area_insets,
                    device_pixel_ratio: fields.device_pixel_ratio,
                    rotation: fields.rotation,
                    color_depth: fields.color_depth,
                    label: fields.label,
                    is_internal: fields.is_internal,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.addScreen。
///
///协议原文：
///
///<pre>Add a new screen to the device. Only supported in headless mode.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct AddScreen;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.addScreen/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct AddScreenResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.addScreen/returns/screenInfo。
    #[serde(rename = "screenInfo")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub screen_info: crate::emulation::types::ScreenInfo,
}
impl<'de> serde::Deserialize<'de> for AddScreenResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AddScreenResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.addScreen/returns/screenInfo。
                    #[serde(rename = "screenInfo")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub screen_info: crate::emulation::types::ScreenInfo,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(AddScreenResponse {
                    screen_info: fields.screen_info,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for AddScreen {
    type Params = AddScreenParams;
    type Response = AddScreenResponse;
    const METHOD: &'static str = "Emulation.addScreen";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.canEmulate/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct CanEmulateParams {}
impl<'de> serde::Deserialize<'de> for CanEmulateParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CanEmulateParams;
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
                Ok(CanEmulateParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.canEmulate。
///
///协议原文：
///
///<pre>Tells whether emulation is supported.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct CanEmulate;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.canEmulate/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct CanEmulateResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.canEmulate/returns/result。
    ///
    ///协议原文：
    ///
    ///<pre>True if emulation is supported.</pre>
    ///
    ///
    #[serde(rename = "result")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub result: bool,
}
impl<'de> serde::Deserialize<'de> for CanEmulateResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CanEmulateResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.canEmulate/returns/result。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>True if emulation is supported.</pre>
                    ///
                    ///
                    #[serde(rename = "result")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub result: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(CanEmulateResponse {
                    result: fields.result,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for CanEmulate {
    type Params = CanEmulateParams;
    type Response = CanEmulateResponse;
    const METHOD: &'static str = "Emulation.canEmulate";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.clearDeviceMetricsOverride/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ClearDeviceMetricsOverrideParams {}
impl<'de> serde::Deserialize<'de> for ClearDeviceMetricsOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearDeviceMetricsOverrideParams;
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
                Ok(ClearDeviceMetricsOverrideParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.clearDeviceMetricsOverride。
///
///协议原文：
///
///<pre>Clears the overridden device metrics.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ClearDeviceMetricsOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.clearDeviceMetricsOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearDeviceMetricsOverrideResponse {}
impl<'de> serde::Deserialize<'de> for ClearDeviceMetricsOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearDeviceMetricsOverrideResponse;
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
                Ok(ClearDeviceMetricsOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ClearDeviceMetricsOverride {
    type Params = ClearDeviceMetricsOverrideParams;
    type Response = ClearDeviceMetricsOverrideResponse;
    const METHOD: &'static str = "Emulation.clearDeviceMetricsOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.clearDevicePostureOverride/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ClearDevicePostureOverrideParams {}
impl<'de> serde::Deserialize<'de> for ClearDevicePostureOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearDevicePostureOverrideParams;
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
                Ok(ClearDevicePostureOverrideParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.clearDevicePostureOverride。
///
///协议原文：
///
///<pre>Clears a device posture override set with either setDeviceMetricsOverride()
///or setDevicePostureOverride() and starts using posture information from the
///platform again.
///Does nothing if no override is set.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ClearDevicePostureOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.clearDevicePostureOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearDevicePostureOverrideResponse {}
impl<'de> serde::Deserialize<'de> for ClearDevicePostureOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearDevicePostureOverrideResponse;
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
                Ok(ClearDevicePostureOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ClearDevicePostureOverride {
    type Params = ClearDevicePostureOverrideParams;
    type Response = ClearDevicePostureOverrideResponse;
    const METHOD: &'static str = "Emulation.clearDevicePostureOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.clearDisplayFeaturesOverride/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ClearDisplayFeaturesOverrideParams {}
impl<'de> serde::Deserialize<'de> for ClearDisplayFeaturesOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearDisplayFeaturesOverrideParams;
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
                Ok(ClearDisplayFeaturesOverrideParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.clearDisplayFeaturesOverride。
///
///协议原文：
///
///<pre>Clears the display features override set with either setDeviceMetricsOverride()
///or setDisplayFeaturesOverride() and starts using display features from the
///platform again.
///Does nothing if no override is set.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ClearDisplayFeaturesOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.clearDisplayFeaturesOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearDisplayFeaturesOverrideResponse {}
impl<'de> serde::Deserialize<'de> for ClearDisplayFeaturesOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearDisplayFeaturesOverrideResponse;
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
                Ok(ClearDisplayFeaturesOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ClearDisplayFeaturesOverride {
    type Params = ClearDisplayFeaturesOverrideParams;
    type Response = ClearDisplayFeaturesOverrideResponse;
    const METHOD: &'static str = "Emulation.clearDisplayFeaturesOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.clearGeolocationOverride/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ClearGeolocationOverrideParams {}
impl<'de> serde::Deserialize<'de> for ClearGeolocationOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearGeolocationOverrideParams;
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
                Ok(ClearGeolocationOverrideParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.clearGeolocationOverride。
///
///协议原文：
///
///<pre>Clears the overridden Geolocation Position and Error.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ClearGeolocationOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.clearGeolocationOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearGeolocationOverrideResponse {}
impl<'de> serde::Deserialize<'de> for ClearGeolocationOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearGeolocationOverrideResponse;
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
                Ok(ClearGeolocationOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ClearGeolocationOverride {
    type Params = ClearGeolocationOverrideParams;
    type Response = ClearGeolocationOverrideResponse;
    const METHOD: &'static str = "Emulation.clearGeolocationOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.clearIdleOverride/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ClearIdleOverrideParams {}
impl<'de> serde::Deserialize<'de> for ClearIdleOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearIdleOverrideParams;
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
                Ok(ClearIdleOverrideParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.clearIdleOverride。
///
///协议原文：
///
///<pre>Clears Idle state overrides.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct ClearIdleOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.clearIdleOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClearIdleOverrideResponse {}
impl<'de> serde::Deserialize<'de> for ClearIdleOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClearIdleOverrideResponse;
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
                Ok(ClearIdleOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ClearIdleOverride {
    type Params = ClearIdleOverrideParams;
    type Response = ClearIdleOverrideResponse;
    const METHOD: &'static str = "Emulation.clearIdleOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.getOverriddenSensorInformation/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetOverriddenSensorInformationParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.getOverriddenSensorInformation/parameters/type。
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: crate::emulation::types::SensorType,
}
impl<'de> serde::Deserialize<'de> for GetOverriddenSensorInformationParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetOverriddenSensorInformationParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.getOverriddenSensorInformation/parameters/type。
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: crate::emulation::types::SensorType,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetOverriddenSensorInformationParams {
                    r#type: fields.r#type,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.getOverriddenSensorInformation。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetOverriddenSensorInformation;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.getOverriddenSensorInformation/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetOverriddenSensorInformationResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.getOverriddenSensorInformation/returns/requestedSamplingFrequency。
    #[serde(rename = "requestedSamplingFrequency")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub requested_sampling_frequency: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for GetOverriddenSensorInformationResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetOverriddenSensorInformationResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.getOverriddenSensorInformation/returns/requestedSamplingFrequency。
                    #[serde(rename = "requestedSamplingFrequency")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub requested_sampling_frequency: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetOverriddenSensorInformationResponse {
                    requested_sampling_frequency: fields.requested_sampling_frequency,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetOverriddenSensorInformation {
    type Params = GetOverriddenSensorInformationParams;
    type Response = GetOverriddenSensorInformationResponse;
    const METHOD: &'static str = "Emulation.getOverriddenSensorInformation";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.getScreenInfos/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct GetScreenInfosParams {}
impl<'de> serde::Deserialize<'de> for GetScreenInfosParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetScreenInfosParams;
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
                Ok(GetScreenInfosParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.getScreenInfos。
///
///协议原文：
///
///<pre>Returns device's screen configuration. In headful mode, the physical screens configuration is returned,
///whereas in headless mode, a virtual headless screen configuration is provided instead.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct GetScreenInfos;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.getScreenInfos/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct GetScreenInfosResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.getScreenInfos/returns/screenInfos。
    #[serde(rename = "screenInfos")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub screen_infos: Vec<crate::emulation::types::ScreenInfo>,
}
impl<'de> serde::Deserialize<'de> for GetScreenInfosResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GetScreenInfosResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.getScreenInfos/returns/screenInfos。
                    #[serde(rename = "screenInfos")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub screen_infos: Vec<crate::emulation::types::ScreenInfo>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(GetScreenInfosResponse {
                    screen_infos: fields.screen_infos,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for GetScreenInfos {
    type Params = GetScreenInfosParams;
    type Response = GetScreenInfosResponse;
    const METHOD: &'static str = "Emulation.getScreenInfos";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.removeScreen/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveScreenParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.removeScreen/parameters/screenId。
    #[serde(rename = "screenId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub screen_id: crate::emulation::types::ScreenId,
}
impl<'de> serde::Deserialize<'de> for RemoveScreenParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveScreenParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.removeScreen/parameters/screenId。
                    #[serde(rename = "screenId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub screen_id: crate::emulation::types::ScreenId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(RemoveScreenParams {
                    screen_id: fields.screen_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.removeScreen。
///
///协议原文：
///
///<pre>Remove screen from the device. Only supported in headless mode.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct RemoveScreen;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.removeScreen/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct RemoveScreenResponse {}
impl<'de> serde::Deserialize<'de> for RemoveScreenResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RemoveScreenResponse;
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
                Ok(RemoveScreenResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for RemoveScreen {
    type Params = RemoveScreenParams;
    type Response = RemoveScreenResponse;
    const METHOD: &'static str = "Emulation.removeScreen";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.resetPageScaleFactor/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct ResetPageScaleFactorParams {}
impl<'de> serde::Deserialize<'de> for ResetPageScaleFactorParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResetPageScaleFactorParams;
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
                Ok(ResetPageScaleFactorParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.resetPageScaleFactor。
///
///协议原文：
///
///<pre>Requests that page scale factor is reset to initial values.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct ResetPageScaleFactor;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.resetPageScaleFactor/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct ResetPageScaleFactorResponse {}
impl<'de> serde::Deserialize<'de> for ResetPageScaleFactorResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ResetPageScaleFactorResponse;
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
                Ok(ResetPageScaleFactorResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for ResetPageScaleFactor {
    type Params = ResetPageScaleFactorParams;
    type Response = ResetPageScaleFactorResponse;
    const METHOD: &'static str = "Emulation.resetPageScaleFactor";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setAutoDarkModeOverride/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct SetAutoDarkModeOverrideParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setAutoDarkModeOverride/parameters/enabled。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to enable or disable automatic dark mode.
    ///If not specified, any existing override will be cleared.</pre>
    ///
    ///
    #[serde(rename = "enabled")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub enabled: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for SetAutoDarkModeOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetAutoDarkModeOverrideParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setAutoDarkModeOverride/parameters/enabled。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to enable or disable automatic dark mode.
                    ///If not specified, any existing override will be cleared.</pre>
                    ///
                    ///
                    #[serde(rename = "enabled")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub enabled: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetAutoDarkModeOverrideParams {
                    enabled: fields.enabled,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setAutoDarkModeOverride。
///
///协议原文：
///
///<pre>Automatically render all web contents using a dark theme.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetAutoDarkModeOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setAutoDarkModeOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetAutoDarkModeOverrideResponse {}
impl<'de> serde::Deserialize<'de> for SetAutoDarkModeOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetAutoDarkModeOverrideResponse;
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
                Ok(SetAutoDarkModeOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetAutoDarkModeOverride {
    type Params = SetAutoDarkModeOverrideParams;
    type Response = SetAutoDarkModeOverrideResponse;
    const METHOD: &'static str = "Emulation.setAutoDarkModeOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setAutomationOverride/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetAutomationOverrideParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setAutomationOverride/parameters/enabled。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the override should be enabled.</pre>
    ///
    ///
    #[serde(rename = "enabled")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub enabled: bool,
}
impl<'de> serde::Deserialize<'de> for SetAutomationOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetAutomationOverrideParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setAutomationOverride/parameters/enabled。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the override should be enabled.</pre>
                    ///
                    ///
                    #[serde(rename = "enabled")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub enabled: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetAutomationOverrideParams {
                    enabled: fields.enabled,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setAutomationOverride。
///
///协议原文：
///
///<pre>Allows overriding the automation flag.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetAutomationOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setAutomationOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetAutomationOverrideResponse {}
impl<'de> serde::Deserialize<'de> for SetAutomationOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetAutomationOverrideResponse;
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
                Ok(SetAutomationOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetAutomationOverride {
    type Params = SetAutomationOverrideParams;
    type Response = SetAutomationOverrideResponse;
    const METHOD: &'static str = "Emulation.setAutomationOverride";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Emulation.command.setCPUPerformanceOverride/parameters/performanceTier。
///
///协议原文：
///
///<pre>Override value. Omitting the parameter disables the override.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SetCpuPerformanceOverrideParamsPerformanceTier {
    ///协议值 "unknown"。
    KnownUnknown,
    ///协议值 "low"。
    Low,
    ///协议值 "mid"。
    Mid,
    ///协议值 "high"。
    High,
    ///协议值 "ultra"。
    Ultra,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl SetCpuPerformanceOverrideParamsPerformanceTier {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::emulation::commands::SetCpuPerformanceOverrideParamsPerformanceTier::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::KnownUnknown => "unknown",
            Self::Low => "low",
            Self::Mid => "mid",
            Self::High => "high",
            Self::Ultra => "ultra",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for SetCpuPerformanceOverrideParamsPerformanceTier {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for SetCpuPerformanceOverrideParamsPerformanceTier {
    fn from(value: String) -> Self {
        match value.as_str() {
            "unknown" => SetCpuPerformanceOverrideParamsPerformanceTier::KnownUnknown,
            "low" => SetCpuPerformanceOverrideParamsPerformanceTier::Low,
            "mid" => SetCpuPerformanceOverrideParamsPerformanceTier::Mid,
            "high" => SetCpuPerformanceOverrideParamsPerformanceTier::High,
            "ultra" => SetCpuPerformanceOverrideParamsPerformanceTier::Ultra,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for SetCpuPerformanceOverrideParamsPerformanceTier {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetCpuPerformanceOverrideParamsPerformanceTier {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetCpuPerformanceOverrideParamsPerformanceTier;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "unknown" => SetCpuPerformanceOverrideParamsPerformanceTier::KnownUnknown,
                    "low" => SetCpuPerformanceOverrideParamsPerformanceTier::Low,
                    "mid" => SetCpuPerformanceOverrideParamsPerformanceTier::Mid,
                    "high" => SetCpuPerformanceOverrideParamsPerformanceTier::High,
                    "ultra" => SetCpuPerformanceOverrideParamsPerformanceTier::Ultra,
                    _ => SetCpuPerformanceOverrideParamsPerformanceTier::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(SetCpuPerformanceOverrideParamsPerformanceTier::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setCPUPerformanceOverride/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct SetCpuPerformanceOverrideParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setCPUPerformanceOverride/parameters/performanceTier。
    ///
    ///协议原文：
    ///
    ///<pre>Override value. Omitting the parameter disables the override.</pre>
    ///
    ///
    #[serde(rename = "performanceTier")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub performance_tier: Option<SetCpuPerformanceOverrideParamsPerformanceTier>,
}
impl<'de> serde::Deserialize<'de> for SetCpuPerformanceOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetCpuPerformanceOverrideParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setCPUPerformanceOverride/parameters/performanceTier。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Override value. Omitting the parameter disables the override.</pre>
                    ///
                    ///
                    #[serde(rename = "performanceTier")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub performance_tier: Option<SetCpuPerformanceOverrideParamsPerformanceTier>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetCpuPerformanceOverrideParams {
                    performance_tier: fields.performance_tier,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setCPUPerformanceOverride。
///
///协议原文：
///
///<pre>Overrides the value of navigator.cpuPerformance</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetCpuPerformanceOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setCPUPerformanceOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetCpuPerformanceOverrideResponse {}
impl<'de> serde::Deserialize<'de> for SetCpuPerformanceOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetCpuPerformanceOverrideResponse;
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
                Ok(SetCpuPerformanceOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetCpuPerformanceOverride {
    type Params = SetCpuPerformanceOverrideParams;
    type Response = SetCpuPerformanceOverrideResponse;
    const METHOD: &'static str = "Emulation.setCPUPerformanceOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setCPUThrottlingRate/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetCpuThrottlingRateParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setCPUThrottlingRate/parameters/rate。
    ///
    ///协议原文：
    ///
    ///<pre>Throttling rate as a slowdown factor (1 is no throttle, 2 is 2x slowdown, etc).</pre>
    ///
    ///
    #[serde(rename = "rate")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub rate: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for SetCpuThrottlingRateParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetCpuThrottlingRateParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setCPUThrottlingRate/parameters/rate。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Throttling rate as a slowdown factor (1 is no throttle, 2 is 2x slowdown, etc).</pre>
                    ///
                    ///
                    #[serde(rename = "rate")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub rate: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetCpuThrottlingRateParams { rate: fields.rate })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setCPUThrottlingRate。
///
///协议原文：
///
///<pre>Enables CPU throttling to emulate slow CPUs.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetCpuThrottlingRate;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setCPUThrottlingRate/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetCpuThrottlingRateResponse {}
impl<'de> serde::Deserialize<'de> for SetCpuThrottlingRateResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetCpuThrottlingRateResponse;
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
                Ok(SetCpuThrottlingRateResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetCpuThrottlingRate {
    type Params = SetCpuThrottlingRateParams;
    type Response = SetCpuThrottlingRateResponse;
    const METHOD: &'static str = "Emulation.setCPUThrottlingRate";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setDataSaverOverride/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct SetDataSaverOverrideParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDataSaverOverride/parameters/dataSaverEnabled。
    ///
    ///协议原文：
    ///
    ///<pre>Override value. Omitting the parameter disables the override.</pre>
    ///
    ///
    #[serde(rename = "dataSaverEnabled")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub data_saver_enabled: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for SetDataSaverOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDataSaverOverrideParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDataSaverOverride/parameters/dataSaverEnabled。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Override value. Omitting the parameter disables the override.</pre>
                    ///
                    ///
                    #[serde(rename = "dataSaverEnabled")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub data_saver_enabled: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetDataSaverOverrideParams {
                    data_saver_enabled: fields.data_saver_enabled,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setDataSaverOverride。
///
///协议原文：
///
///<pre>Override the value of navigator.connection.saveData</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetDataSaverOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setDataSaverOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDataSaverOverrideResponse {}
impl<'de> serde::Deserialize<'de> for SetDataSaverOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDataSaverOverrideResponse;
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
                Ok(SetDataSaverOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetDataSaverOverride {
    type Params = SetDataSaverOverrideParams;
    type Response = SetDataSaverOverrideResponse;
    const METHOD: &'static str = "Emulation.setDataSaverOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setDefaultBackgroundColorOverride/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct SetDefaultBackgroundColorOverrideParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDefaultBackgroundColorOverride/parameters/color。
    ///
    ///协议原文：
    ///
    ///<pre>RGBA of the default background color. If not specified, any existing override will be
    ///cleared.</pre>
    ///
    ///
    #[serde(rename = "color")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub color: Option<crate::dom::types::Rgba>,
}
impl<'de> serde::Deserialize<'de> for SetDefaultBackgroundColorOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDefaultBackgroundColorOverrideParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDefaultBackgroundColorOverride/parameters/color。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>RGBA of the default background color. If not specified, any existing override will be
                    ///cleared.</pre>
                    ///
                    ///
                    #[serde(rename = "color")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub color: Option<crate::dom::types::Rgba>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetDefaultBackgroundColorOverrideParams {
                    color: fields.color,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setDefaultBackgroundColorOverride。
///
///协议原文：
///
///<pre>Sets or clears an override of the default background color of the frame. This override is used
///if the content does not specify one.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetDefaultBackgroundColorOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setDefaultBackgroundColorOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDefaultBackgroundColorOverrideResponse {}
impl<'de> serde::Deserialize<'de> for SetDefaultBackgroundColorOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDefaultBackgroundColorOverrideResponse;
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
                Ok(SetDefaultBackgroundColorOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetDefaultBackgroundColorOverride {
    type Params = SetDefaultBackgroundColorOverrideParams;
    type Response = SetDefaultBackgroundColorOverrideResponse;
    const METHOD: &'static str = "Emulation.setDefaultBackgroundColorOverride";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/scrollbarType。
///
///协议原文：
///
///<pre>Scrollbar type. Default: `default`.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SetDeviceMetricsOverrideParamsScrollbarType {
    ///协议值 "overlay"。
    Overlay,
    ///协议值 "default"。
    Default,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl SetDeviceMetricsOverrideParamsScrollbarType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::emulation::commands::SetDeviceMetricsOverrideParamsScrollbarType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Overlay => "overlay",
            Self::Default => "default",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for SetDeviceMetricsOverrideParamsScrollbarType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for SetDeviceMetricsOverrideParamsScrollbarType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "overlay" => SetDeviceMetricsOverrideParamsScrollbarType::Overlay,
            "default" => SetDeviceMetricsOverrideParamsScrollbarType::Default,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for SetDeviceMetricsOverrideParamsScrollbarType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetDeviceMetricsOverrideParamsScrollbarType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDeviceMetricsOverrideParamsScrollbarType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "overlay" => SetDeviceMetricsOverrideParamsScrollbarType::Overlay,
                    "default" => SetDeviceMetricsOverrideParamsScrollbarType::Default,
                    _ => SetDeviceMetricsOverrideParamsScrollbarType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(SetDeviceMetricsOverrideParamsScrollbarType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/viewportMeta。
///
///协议原文：
///
///<pre>Viewport meta tag behavior. Default: `default`. Note: if `mobile` is `true`,
///the viewport meta tag is always enabled.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SetDeviceMetricsOverrideParamsViewportMeta {
    ///协议值 "enable"。
    Enable,
    ///协议值 "default"。
    Default,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl SetDeviceMetricsOverrideParamsViewportMeta {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::emulation::commands::SetDeviceMetricsOverrideParamsViewportMeta::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Enable => "enable",
            Self::Default => "default",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for SetDeviceMetricsOverrideParamsViewportMeta {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for SetDeviceMetricsOverrideParamsViewportMeta {
    fn from(value: String) -> Self {
        match value.as_str() {
            "enable" => SetDeviceMetricsOverrideParamsViewportMeta::Enable,
            "default" => SetDeviceMetricsOverrideParamsViewportMeta::Default,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for SetDeviceMetricsOverrideParamsViewportMeta {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetDeviceMetricsOverrideParamsViewportMeta {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDeviceMetricsOverrideParamsViewportMeta;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "enable" => SetDeviceMetricsOverrideParamsViewportMeta::Enable,
                    "default" => SetDeviceMetricsOverrideParamsViewportMeta::Default,
                    _ => SetDeviceMetricsOverrideParamsViewportMeta::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(SetDeviceMetricsOverrideParamsViewportMeta::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setDeviceMetricsOverride/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDeviceMetricsOverrideParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/width。
    ///
    ///协议原文：
    ///
    ///<pre>Overriding width value in pixels (minimum 0, maximum 10000000). 0 disables the override.</pre>
    ///
    ///
    #[serde(rename = "width")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub width: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/height。
    ///
    ///协议原文：
    ///
    ///<pre>Overriding height value in pixels (minimum 0, maximum 10000000). 0 disables the override.</pre>
    ///
    ///
    #[serde(rename = "height")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub height: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/deviceScaleFactor。
    ///
    ///协议原文：
    ///
    ///<pre>Overriding device scale factor value. 0 disables the override.</pre>
    ///
    ///
    #[serde(rename = "deviceScaleFactor")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub device_scale_factor: ydp_core::FiniteF64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/mobile。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to emulate mobile device. This includes viewport meta tag, overlay scrollbars, text
    ///autosizing and more.</pre>
    ///
    ///
    #[serde(rename = "mobile")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub mobile: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/scale。
    ///
    ///协议原文：
    ///
    ///<pre>Scale to apply to resulting view image.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "scale")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub scale: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/screenWidth。
    ///
    ///协议原文：
    ///
    ///<pre>Overriding screen width value in pixels (minimum 0, maximum 10000000).</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "screenWidth")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub screen_width: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/screenHeight。
    ///
    ///协议原文：
    ///
    ///<pre>Overriding screen height value in pixels (minimum 0, maximum 10000000).</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "screenHeight")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub screen_height: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/positionX。
    ///
    ///协议原文：
    ///
    ///<pre>Overriding view X position on screen in pixels (minimum 0, maximum 10000000).</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "positionX")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub position_x: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/positionY。
    ///
    ///协议原文：
    ///
    ///<pre>Overriding view Y position on screen in pixels (minimum 0, maximum 10000000).</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "positionY")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub position_y: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/dontSetVisibleSize。
    ///
    ///协议原文：
    ///
    ///<pre>Do not set visible view size, rely upon explicit setVisibleSize call.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "dontSetVisibleSize")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub dont_set_visible_size: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/screenOrientation。
    ///
    ///协议原文：
    ///
    ///<pre>Screen orientation override.</pre>
    ///
    ///
    #[serde(rename = "screenOrientation")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub screen_orientation: Option<crate::emulation::types::ScreenOrientation>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/viewport。
    ///
    ///协议原文：
    ///
    ///<pre>If set, the visible area of the page will be overridden to this viewport. This viewport
    ///change is not observed by the page, e.g. viewport-relative elements do not change positions.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "viewport")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub viewport: Option<crate::page::types::Viewport>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/displayFeature。
    ///
    ///协议原文：
    ///
    ///<pre>If set, the display feature of a multi-segment screen. If not set, multi-segment support
    ///is turned-off.
    ///Deprecated, use Emulation.setDisplayFeaturesOverride.</pre>
    ///
    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "displayFeature")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_feature: Option<crate::emulation::types::DisplayFeature>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/devicePosture。
    ///
    ///协议原文：
    ///
    ///<pre>If set, the posture of a foldable device. If not set the posture is set
    ///to continuous.
    ///Deprecated, use Emulation.setDevicePostureOverride.</pre>
    ///
    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
    #[serde(rename = "devicePosture")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub device_posture: Option<crate::emulation::types::DevicePosture>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/scrollbarType。
    ///
    ///协议原文：
    ///
    ///<pre>Scrollbar type. Default: `default`.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "scrollbarType")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub scrollbar_type: Option<SetDeviceMetricsOverrideParamsScrollbarType>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/screenOrientationLockEmulation。
    ///
    ///协议原文：
    ///
    ///<pre>If set to true, enables screen orientation lock emulation, which
    ///intercepts screen.orientation.lock() calls from the page and reports
    ///orientation changes via screenOrientationLockChanged events. This is
    ///useful for emulating mobile device orientation lock behavior in
    ///responsive design mode.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "screenOrientationLockEmulation")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub screen_orientation_lock_emulation: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/viewportMeta。
    ///
    ///协议原文：
    ///
    ///<pre>Viewport meta tag behavior. Default: `default`. Note: if `mobile` is `true`,
    ///the viewport meta tag is always enabled.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "viewportMeta")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub viewport_meta: Option<SetDeviceMetricsOverrideParamsViewportMeta>,
}
impl<'de> serde::Deserialize<'de> for SetDeviceMetricsOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDeviceMetricsOverrideParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/width。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Overriding width value in pixels (minimum 0, maximum 10000000). 0 disables the override.</pre>
                    ///
                    ///
                    #[serde(rename = "width")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub width: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/height。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Overriding height value in pixels (minimum 0, maximum 10000000). 0 disables the override.</pre>
                    ///
                    ///
                    #[serde(rename = "height")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub height: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/deviceScaleFactor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Overriding device scale factor value. 0 disables the override.</pre>
                    ///
                    ///
                    #[serde(rename = "deviceScaleFactor")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub device_scale_factor: ydp_core::FiniteF64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/mobile。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to emulate mobile device. This includes viewport meta tag, overlay scrollbars, text
                    ///autosizing and more.</pre>
                    ///
                    ///
                    #[serde(rename = "mobile")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub mobile: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/scale。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Scale to apply to resulting view image.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "scale")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub scale: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/screenWidth。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Overriding screen width value in pixels (minimum 0, maximum 10000000).</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "screenWidth")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub screen_width: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/screenHeight。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Overriding screen height value in pixels (minimum 0, maximum 10000000).</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "screenHeight")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub screen_height: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/positionX。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Overriding view X position on screen in pixels (minimum 0, maximum 10000000).</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "positionX")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub position_x: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/positionY。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Overriding view Y position on screen in pixels (minimum 0, maximum 10000000).</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "positionY")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub position_y: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/dontSetVisibleSize。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Do not set visible view size, rely upon explicit setVisibleSize call.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "dontSetVisibleSize")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub dont_set_visible_size: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/screenOrientation。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Screen orientation override.</pre>
                    ///
                    ///
                    #[serde(rename = "screenOrientation")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub screen_orientation: Option<crate::emulation::types::ScreenOrientation>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/viewport。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If set, the visible area of the page will be overridden to this viewport. This viewport
                    ///change is not observed by the page, e.g. viewport-relative elements do not change positions.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "viewport")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub viewport: Option<crate::page::types::Viewport>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/displayFeature。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If set, the display feature of a multi-segment screen. If not set, multi-segment support
                    ///is turned-off.
                    ///Deprecated, use Emulation.setDisplayFeaturesOverride.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "displayFeature")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub display_feature: Option<crate::emulation::types::DisplayFeature>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/devicePosture。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If set, the posture of a foldable device. If not set the posture is set
                    ///to continuous.
                    ///Deprecated, use Emulation.setDevicePostureOverride.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
                    #[serde(rename = "devicePosture")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub device_posture: Option<crate::emulation::types::DevicePosture>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/scrollbarType。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Scrollbar type. Default: `default`.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "scrollbarType")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub scrollbar_type: Option<SetDeviceMetricsOverrideParamsScrollbarType>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/screenOrientationLockEmulation。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If set to true, enables screen orientation lock emulation, which
                    ///intercepts screen.orientation.lock() calls from the page and reports
                    ///orientation changes via screenOrientationLockChanged events. This is
                    ///useful for emulating mobile device orientation lock behavior in
                    ///responsive design mode.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "screenOrientationLockEmulation")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub screen_orientation_lock_emulation: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setDeviceMetricsOverride/parameters/viewportMeta。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Viewport meta tag behavior. Default: `default`. Note: if `mobile` is `true`,
                    ///the viewport meta tag is always enabled.</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "viewportMeta")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub viewport_meta: Option<SetDeviceMetricsOverrideParamsViewportMeta>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetDeviceMetricsOverrideParams {
                    width: fields.width,
                    height: fields.height,
                    device_scale_factor: fields.device_scale_factor,
                    mobile: fields.mobile,
                    scale: fields.scale,
                    screen_width: fields.screen_width,
                    screen_height: fields.screen_height,
                    position_x: fields.position_x,
                    position_y: fields.position_y,
                    dont_set_visible_size: fields.dont_set_visible_size,
                    screen_orientation: fields.screen_orientation,
                    viewport: fields.viewport,
                    display_feature: fields.display_feature,
                    device_posture: fields.device_posture,
                    scrollbar_type: fields.scrollbar_type,
                    screen_orientation_lock_emulation: fields.screen_orientation_lock_emulation,
                    viewport_meta: fields.viewport_meta,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setDeviceMetricsOverride。
///
///协议原文：
///
///<pre>Overrides the values of device screen dimensions (window.screen.width, window.screen.height,
///window.innerWidth, window.innerHeight, and "device-width"/"device-height"-related CSS media
///query results).</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetDeviceMetricsOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setDeviceMetricsOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDeviceMetricsOverrideResponse {}
impl<'de> serde::Deserialize<'de> for SetDeviceMetricsOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDeviceMetricsOverrideResponse;
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
                Ok(SetDeviceMetricsOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetDeviceMetricsOverride {
    type Params = SetDeviceMetricsOverrideParams;
    type Response = SetDeviceMetricsOverrideResponse;
    const METHOD: &'static str = "Emulation.setDeviceMetricsOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setDevicePostureOverride/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDevicePostureOverrideParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setDevicePostureOverride/parameters/posture。
    #[serde(rename = "posture")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub posture: crate::emulation::types::DevicePosture,
}
impl<'de> serde::Deserialize<'de> for SetDevicePostureOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDevicePostureOverrideParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setDevicePostureOverride/parameters/posture。
                    #[serde(rename = "posture")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub posture: crate::emulation::types::DevicePosture,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetDevicePostureOverrideParams {
                    posture: fields.posture,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setDevicePostureOverride。
///
///协议原文：
///
///<pre>Start reporting the given posture value to the Device Posture API.
///This override can also be set in setDeviceMetricsOverride().</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetDevicePostureOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setDevicePostureOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDevicePostureOverrideResponse {}
impl<'de> serde::Deserialize<'de> for SetDevicePostureOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDevicePostureOverrideResponse;
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
                Ok(SetDevicePostureOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetDevicePostureOverride {
    type Params = SetDevicePostureOverrideParams;
    type Response = SetDevicePostureOverrideResponse;
    const METHOD: &'static str = "Emulation.setDevicePostureOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setDisabledImageTypes/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDisabledImageTypesParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setDisabledImageTypes/parameters/imageTypes。
    ///
    ///协议原文：
    ///
    ///<pre>Image types to disable.</pre>
    ///
    ///
    #[serde(rename = "imageTypes")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub image_types: Vec<crate::emulation::types::DisabledImageType>,
}
impl<'de> serde::Deserialize<'de> for SetDisabledImageTypesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDisabledImageTypesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setDisabledImageTypes/parameters/imageTypes。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Image types to disable.</pre>
                    ///
                    ///
                    #[serde(rename = "imageTypes")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub image_types: Vec<crate::emulation::types::DisabledImageType>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetDisabledImageTypesParams {
                    image_types: fields.image_types,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setDisabledImageTypes。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetDisabledImageTypes;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setDisabledImageTypes/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDisabledImageTypesResponse {}
impl<'de> serde::Deserialize<'de> for SetDisabledImageTypesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDisabledImageTypesResponse;
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
                Ok(SetDisabledImageTypesResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetDisabledImageTypes {
    type Params = SetDisabledImageTypesParams;
    type Response = SetDisabledImageTypesResponse;
    const METHOD: &'static str = "Emulation.setDisabledImageTypes";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setDisplayFeaturesOverride/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDisplayFeaturesOverrideParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setDisplayFeaturesOverride/parameters/features。
    #[serde(rename = "features")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub features: Vec<crate::emulation::types::DisplayFeature>,
}
impl<'de> serde::Deserialize<'de> for SetDisplayFeaturesOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDisplayFeaturesOverrideParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setDisplayFeaturesOverride/parameters/features。
                    #[serde(rename = "features")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub features: Vec<crate::emulation::types::DisplayFeature>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetDisplayFeaturesOverrideParams {
                    features: fields.features,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setDisplayFeaturesOverride。
///
///协议原文：
///
///<pre>Start using the given display features to pupulate the Viewport Segments API.
///This override can also be set in setDeviceMetricsOverride().</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetDisplayFeaturesOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setDisplayFeaturesOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDisplayFeaturesOverrideResponse {}
impl<'de> serde::Deserialize<'de> for SetDisplayFeaturesOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDisplayFeaturesOverrideResponse;
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
                Ok(SetDisplayFeaturesOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetDisplayFeaturesOverride {
    type Params = SetDisplayFeaturesOverrideParams;
    type Response = SetDisplayFeaturesOverrideResponse;
    const METHOD: &'static str = "Emulation.setDisplayFeaturesOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setDocumentCookieDisabled/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDocumentCookieDisabledParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setDocumentCookieDisabled/parameters/disabled。
    ///
    ///协议原文：
    ///
    ///<pre>Whether document.coookie API should be disabled.</pre>
    ///
    ///
    #[serde(rename = "disabled")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub disabled: bool,
}
impl<'de> serde::Deserialize<'de> for SetDocumentCookieDisabledParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDocumentCookieDisabledParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setDocumentCookieDisabled/parameters/disabled。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether document.coookie API should be disabled.</pre>
                    ///
                    ///
                    #[serde(rename = "disabled")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub disabled: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetDocumentCookieDisabledParams {
                    disabled: fields.disabled,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setDocumentCookieDisabled。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetDocumentCookieDisabled;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setDocumentCookieDisabled/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetDocumentCookieDisabledResponse {}
impl<'de> serde::Deserialize<'de> for SetDocumentCookieDisabledResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetDocumentCookieDisabledResponse;
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
                Ok(SetDocumentCookieDisabledResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetDocumentCookieDisabled {
    type Params = SetDocumentCookieDisabledParams;
    type Response = SetDocumentCookieDisabledResponse;
    const METHOD: &'static str = "Emulation.setDocumentCookieDisabled";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Emulation.command.setEmitTouchEventsForMouse/parameters/configuration。
///
///协议原文：
///
///<pre>Touch/gesture events configuration. Default: current platform.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SetEmitTouchEventsForMouseParamsConfiguration {
    ///协议值 "mobile"。
    Mobile,
    ///协议值 "desktop"。
    Desktop,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl SetEmitTouchEventsForMouseParamsConfiguration {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::emulation::commands::SetEmitTouchEventsForMouseParamsConfiguration::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Mobile => "mobile",
            Self::Desktop => "desktop",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for SetEmitTouchEventsForMouseParamsConfiguration {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for SetEmitTouchEventsForMouseParamsConfiguration {
    fn from(value: String) -> Self {
        match value.as_str() {
            "mobile" => SetEmitTouchEventsForMouseParamsConfiguration::Mobile,
            "desktop" => SetEmitTouchEventsForMouseParamsConfiguration::Desktop,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for SetEmitTouchEventsForMouseParamsConfiguration {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetEmitTouchEventsForMouseParamsConfiguration {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetEmitTouchEventsForMouseParamsConfiguration;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "mobile" => SetEmitTouchEventsForMouseParamsConfiguration::Mobile,
                    "desktop" => SetEmitTouchEventsForMouseParamsConfiguration::Desktop,
                    _ => SetEmitTouchEventsForMouseParamsConfiguration::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(SetEmitTouchEventsForMouseParamsConfiguration::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setEmitTouchEventsForMouse/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetEmitTouchEventsForMouseParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setEmitTouchEventsForMouse/parameters/enabled。
    ///
    ///协议原文：
    ///
    ///<pre>Whether touch emulation based on mouse input should be enabled.</pre>
    ///
    ///
    #[serde(rename = "enabled")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub enabled: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setEmitTouchEventsForMouse/parameters/configuration。
    ///
    ///协议原文：
    ///
    ///<pre>Touch/gesture events configuration. Default: current platform.</pre>
    ///
    ///
    #[serde(rename = "configuration")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub configuration: Option<SetEmitTouchEventsForMouseParamsConfiguration>,
}
impl<'de> serde::Deserialize<'de> for SetEmitTouchEventsForMouseParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetEmitTouchEventsForMouseParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setEmitTouchEventsForMouse/parameters/enabled。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether touch emulation based on mouse input should be enabled.</pre>
                    ///
                    ///
                    #[serde(rename = "enabled")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub enabled: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setEmitTouchEventsForMouse/parameters/configuration。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Touch/gesture events configuration. Default: current platform.</pre>
                    ///
                    ///
                    #[serde(rename = "configuration")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub configuration: Option<SetEmitTouchEventsForMouseParamsConfiguration>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetEmitTouchEventsForMouseParams {
                    enabled: fields.enabled,
                    configuration: fields.configuration,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setEmitTouchEventsForMouse。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetEmitTouchEventsForMouse;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setEmitTouchEventsForMouse/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetEmitTouchEventsForMouseResponse {}
impl<'de> serde::Deserialize<'de> for SetEmitTouchEventsForMouseResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetEmitTouchEventsForMouseResponse;
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
                Ok(SetEmitTouchEventsForMouseResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetEmitTouchEventsForMouse {
    type Params = SetEmitTouchEventsForMouseParams;
    type Response = SetEmitTouchEventsForMouseResponse;
    const METHOD: &'static str = "Emulation.setEmitTouchEventsForMouse";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setEmulatedMedia/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct SetEmulatedMediaParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setEmulatedMedia/parameters/media。
    ///
    ///协议原文：
    ///
    ///<pre>Media type to emulate. Empty string disables the override.</pre>
    ///
    ///
    #[serde(rename = "media")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub media: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setEmulatedMedia/parameters/features。
    ///
    ///协议原文：
    ///
    ///<pre>Media features to emulate.</pre>
    ///
    ///
    #[serde(rename = "features")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub features: Option<Vec<crate::emulation::types::MediaFeature>>,
}
impl<'de> serde::Deserialize<'de> for SetEmulatedMediaParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetEmulatedMediaParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setEmulatedMedia/parameters/media。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Media type to emulate. Empty string disables the override.</pre>
                    ///
                    ///
                    #[serde(rename = "media")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub media: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setEmulatedMedia/parameters/features。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Media features to emulate.</pre>
                    ///
                    ///
                    #[serde(rename = "features")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub features: Option<Vec<crate::emulation::types::MediaFeature>>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetEmulatedMediaParams {
                    media: fields.media,
                    features: fields.features,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setEmulatedMedia。
///
///协议原文：
///
///<pre>Emulates the given media type or media feature for CSS media queries.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetEmulatedMedia;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setEmulatedMedia/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetEmulatedMediaResponse {}
impl<'de> serde::Deserialize<'de> for SetEmulatedMediaResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetEmulatedMediaResponse;
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
                Ok(SetEmulatedMediaResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetEmulatedMedia {
    type Params = SetEmulatedMediaParams;
    type Response = SetEmulatedMediaResponse;
    const METHOD: &'static str = "Emulation.setEmulatedMedia";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setEmulatedOSTextScale/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct SetEmulatedOsTextScaleParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setEmulatedOSTextScale/parameters/scale。
    #[serde(rename = "scale")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub scale: Option<ydp_core::FiniteF64>,
}
impl<'de> serde::Deserialize<'de> for SetEmulatedOsTextScaleParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetEmulatedOsTextScaleParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setEmulatedOSTextScale/parameters/scale。
                    #[serde(rename = "scale")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub scale: Option<ydp_core::FiniteF64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetEmulatedOsTextScaleParams {
                    scale: fields.scale,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setEmulatedOSTextScale。
///
///协议原文：
///
///<pre>Emulates the given OS text scale.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetEmulatedOsTextScale;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setEmulatedOSTextScale/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetEmulatedOsTextScaleResponse {}
impl<'de> serde::Deserialize<'de> for SetEmulatedOsTextScaleResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetEmulatedOsTextScaleResponse;
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
                Ok(SetEmulatedOsTextScaleResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetEmulatedOsTextScale {
    type Params = SetEmulatedOsTextScaleParams;
    type Response = SetEmulatedOsTextScaleResponse;
    const METHOD: &'static str = "Emulation.setEmulatedOSTextScale";
}
///开放字符串枚举；已知值不分配 String，未知值拥有原始文本。协议来源：Emulation.command.setEmulatedVisionDeficiency/parameters/type。
///
///协议原文：
///
///<pre>Vision deficiency to emulate. Order: best-effort emulations come first, followed by any
///physiologically accurate emulations for medically recognized color vision deficiencies.</pre>
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SetEmulatedVisionDeficiencyParamsType {
    ///协议值 "none"。
    None,
    ///协议值 "blurredVision"。
    BlurredVision,
    ///协议值 "reducedContrast"。
    ReducedContrast,
    ///协议值 "achromatopsia"。
    Achromatopsia,
    ///协议值 "deuteranopia"。
    Deuteranopia,
    ///协议值 "protanopia"。
    Protanopia,
    ///协议值 "tritanopia"。
    Tritanopia,
    ///未知协议文本；Clone 复制字符串，编码保持原值。
    Unknown(String),
}
impl SetEmulatedVisionDeficiencyParamsType {
    ///借用原始协议字符串，生命周期绑定自身。
    ///
    ///```
    ///let value = ydp::emulation::commands::SetEmulatedVisionDeficiencyParamsType::from(String::from("future-value"));
    ///assert_eq!(value.as_str(), "future-value");
    ///```
    pub fn as_str(&self) -> &str {
        match self {
            Self::None => "none",
            Self::BlurredVision => "blurredVision",
            Self::ReducedContrast => "reducedContrast",
            Self::Achromatopsia => "achromatopsia",
            Self::Deuteranopia => "deuteranopia",
            Self::Protanopia => "protanopia",
            Self::Tritanopia => "tritanopia",
            Self::Unknown(value) => value,
        }
    }
}
impl AsRef<str> for SetEmulatedVisionDeficiencyParamsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<String> for SetEmulatedVisionDeficiencyParamsType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "none" => SetEmulatedVisionDeficiencyParamsType::None,
            "blurredVision" => SetEmulatedVisionDeficiencyParamsType::BlurredVision,
            "reducedContrast" => SetEmulatedVisionDeficiencyParamsType::ReducedContrast,
            "achromatopsia" => SetEmulatedVisionDeficiencyParamsType::Achromatopsia,
            "deuteranopia" => SetEmulatedVisionDeficiencyParamsType::Deuteranopia,
            "protanopia" => SetEmulatedVisionDeficiencyParamsType::Protanopia,
            "tritanopia" => SetEmulatedVisionDeficiencyParamsType::Tritanopia,
            _ => Self::Unknown(value),
        }
    }
}
impl serde::Serialize for SetEmulatedVisionDeficiencyParamsType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SetEmulatedVisionDeficiencyParamsType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetEmulatedVisionDeficiencyParamsType;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议字符串枚举")
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(match v {
                    "none" => SetEmulatedVisionDeficiencyParamsType::None,
                    "blurredVision" => SetEmulatedVisionDeficiencyParamsType::BlurredVision,
                    "reducedContrast" => SetEmulatedVisionDeficiencyParamsType::ReducedContrast,
                    "achromatopsia" => SetEmulatedVisionDeficiencyParamsType::Achromatopsia,
                    "deuteranopia" => SetEmulatedVisionDeficiencyParamsType::Deuteranopia,
                    "protanopia" => SetEmulatedVisionDeficiencyParamsType::Protanopia,
                    "tritanopia" => SetEmulatedVisionDeficiencyParamsType::Tritanopia,
                    _ => SetEmulatedVisionDeficiencyParamsType::Unknown(v.to_owned()),
                })
            }
            fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(SetEmulatedVisionDeficiencyParamsType::from(v))
            }
        }
        d.deserialize_str(Visitor)
    }
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setEmulatedVisionDeficiency/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetEmulatedVisionDeficiencyParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setEmulatedVisionDeficiency/parameters/type。
    ///
    ///协议原文：
    ///
    ///<pre>Vision deficiency to emulate. Order: best-effort emulations come first, followed by any
    ///physiologically accurate emulations for medically recognized color vision deficiencies.</pre>
    ///
    ///
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: SetEmulatedVisionDeficiencyParamsType,
}
impl<'de> serde::Deserialize<'de> for SetEmulatedVisionDeficiencyParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetEmulatedVisionDeficiencyParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setEmulatedVisionDeficiency/parameters/type。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Vision deficiency to emulate. Order: best-effort emulations come first, followed by any
                    ///physiologically accurate emulations for medically recognized color vision deficiencies.</pre>
                    ///
                    ///
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: SetEmulatedVisionDeficiencyParamsType,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetEmulatedVisionDeficiencyParams {
                    r#type: fields.r#type,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setEmulatedVisionDeficiency。
///
///协议原文：
///
///<pre>Emulates the given vision deficiency.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetEmulatedVisionDeficiency;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setEmulatedVisionDeficiency/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetEmulatedVisionDeficiencyResponse {}
impl<'de> serde::Deserialize<'de> for SetEmulatedVisionDeficiencyResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetEmulatedVisionDeficiencyResponse;
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
                Ok(SetEmulatedVisionDeficiencyResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetEmulatedVisionDeficiency {
    type Params = SetEmulatedVisionDeficiencyParams;
    type Response = SetEmulatedVisionDeficiencyResponse;
    const METHOD: &'static str = "Emulation.setEmulatedVisionDeficiency";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setFocusEmulationEnabled/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetFocusEmulationEnabledParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setFocusEmulationEnabled/parameters/enabled。
    ///
    ///协议原文：
    ///
    ///<pre>Whether to enable to disable focus emulation.</pre>
    ///
    ///
    #[serde(rename = "enabled")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub enabled: bool,
}
impl<'de> serde::Deserialize<'de> for SetFocusEmulationEnabledParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetFocusEmulationEnabledParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setFocusEmulationEnabled/parameters/enabled。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether to enable to disable focus emulation.</pre>
                    ///
                    ///
                    #[serde(rename = "enabled")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub enabled: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetFocusEmulationEnabledParams {
                    enabled: fields.enabled,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setFocusEmulationEnabled。
///
///协议原文：
///
///<pre>Enables or disables simulating a focused and active page.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetFocusEmulationEnabled;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setFocusEmulationEnabled/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetFocusEmulationEnabledResponse {}
impl<'de> serde::Deserialize<'de> for SetFocusEmulationEnabledResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetFocusEmulationEnabledResponse;
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
                Ok(SetFocusEmulationEnabledResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetFocusEmulationEnabled {
    type Params = SetFocusEmulationEnabledParams;
    type Response = SetFocusEmulationEnabledResponse;
    const METHOD: &'static str = "Emulation.setFocusEmulationEnabled";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setGeolocationOverride/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct SetGeolocationOverrideParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setGeolocationOverride/parameters/latitude。
    ///
    ///协议原文：
    ///
    ///<pre>Mock latitude</pre>
    ///
    ///
    #[serde(rename = "latitude")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub latitude: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setGeolocationOverride/parameters/longitude。
    ///
    ///协议原文：
    ///
    ///<pre>Mock longitude</pre>
    ///
    ///
    #[serde(rename = "longitude")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub longitude: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setGeolocationOverride/parameters/accuracy。
    ///
    ///协议原文：
    ///
    ///<pre>Mock accuracy</pre>
    ///
    ///
    #[serde(rename = "accuracy")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub accuracy: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setGeolocationOverride/parameters/altitude。
    ///
    ///协议原文：
    ///
    ///<pre>Mock altitude</pre>
    ///
    ///
    #[serde(rename = "altitude")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub altitude: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setGeolocationOverride/parameters/altitudeAccuracy。
    ///
    ///协议原文：
    ///
    ///<pre>Mock altitudeAccuracy</pre>
    ///
    ///
    #[serde(rename = "altitudeAccuracy")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub altitude_accuracy: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setGeolocationOverride/parameters/heading。
    ///
    ///协议原文：
    ///
    ///<pre>Mock heading</pre>
    ///
    ///
    #[serde(rename = "heading")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub heading: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setGeolocationOverride/parameters/speed。
    ///
    ///协议原文：
    ///
    ///<pre>Mock speed</pre>
    ///
    ///
    #[serde(rename = "speed")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub speed: Option<ydp_core::FiniteF64>,
}
impl<'de> serde::Deserialize<'de> for SetGeolocationOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetGeolocationOverrideParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setGeolocationOverride/parameters/latitude。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Mock latitude</pre>
                    ///
                    ///
                    #[serde(rename = "latitude")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub latitude: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setGeolocationOverride/parameters/longitude。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Mock longitude</pre>
                    ///
                    ///
                    #[serde(rename = "longitude")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub longitude: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setGeolocationOverride/parameters/accuracy。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Mock accuracy</pre>
                    ///
                    ///
                    #[serde(rename = "accuracy")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub accuracy: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setGeolocationOverride/parameters/altitude。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Mock altitude</pre>
                    ///
                    ///
                    #[serde(rename = "altitude")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub altitude: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setGeolocationOverride/parameters/altitudeAccuracy。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Mock altitudeAccuracy</pre>
                    ///
                    ///
                    #[serde(rename = "altitudeAccuracy")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub altitude_accuracy: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setGeolocationOverride/parameters/heading。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Mock heading</pre>
                    ///
                    ///
                    #[serde(rename = "heading")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub heading: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setGeolocationOverride/parameters/speed。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Mock speed</pre>
                    ///
                    ///
                    #[serde(rename = "speed")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub speed: Option<ydp_core::FiniteF64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetGeolocationOverrideParams {
                    latitude: fields.latitude,
                    longitude: fields.longitude,
                    accuracy: fields.accuracy,
                    altitude: fields.altitude,
                    altitude_accuracy: fields.altitude_accuracy,
                    heading: fields.heading,
                    speed: fields.speed,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setGeolocationOverride。
///
///协议原文：
///
///<pre>Overrides the Geolocation Position or Error. Omitting latitude, longitude or
///accuracy emulates position unavailable.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetGeolocationOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setGeolocationOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetGeolocationOverrideResponse {}
impl<'de> serde::Deserialize<'de> for SetGeolocationOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetGeolocationOverrideResponse;
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
                Ok(SetGeolocationOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetGeolocationOverride {
    type Params = SetGeolocationOverrideParams;
    type Response = SetGeolocationOverrideResponse;
    const METHOD: &'static str = "Emulation.setGeolocationOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setHardwareConcurrencyOverride/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetHardwareConcurrencyOverrideParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setHardwareConcurrencyOverride/parameters/hardwareConcurrency。
    ///
    ///协议原文：
    ///
    ///<pre>Hardware concurrency to report</pre>
    ///
    ///
    #[serde(rename = "hardwareConcurrency")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub hardware_concurrency: i64,
}
impl<'de> serde::Deserialize<'de> for SetHardwareConcurrencyOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetHardwareConcurrencyOverrideParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setHardwareConcurrencyOverride/parameters/hardwareConcurrency。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Hardware concurrency to report</pre>
                    ///
                    ///
                    #[serde(rename = "hardwareConcurrency")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub hardware_concurrency: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetHardwareConcurrencyOverrideParams {
                    hardware_concurrency: fields.hardware_concurrency,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setHardwareConcurrencyOverride。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetHardwareConcurrencyOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setHardwareConcurrencyOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetHardwareConcurrencyOverrideResponse {}
impl<'de> serde::Deserialize<'de> for SetHardwareConcurrencyOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetHardwareConcurrencyOverrideResponse;
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
                Ok(SetHardwareConcurrencyOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetHardwareConcurrencyOverride {
    type Params = SetHardwareConcurrencyOverrideParams;
    type Response = SetHardwareConcurrencyOverrideResponse;
    const METHOD: &'static str = "Emulation.setHardwareConcurrencyOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setIdleOverride/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetIdleOverrideParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setIdleOverride/parameters/isUserActive。
    ///
    ///协议原文：
    ///
    ///<pre>Mock isUserActive</pre>
    ///
    ///
    #[serde(rename = "isUserActive")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub is_user_active: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setIdleOverride/parameters/isScreenUnlocked。
    ///
    ///协议原文：
    ///
    ///<pre>Mock isScreenUnlocked</pre>
    ///
    ///
    #[serde(rename = "isScreenUnlocked")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub is_screen_unlocked: bool,
}
impl<'de> serde::Deserialize<'de> for SetIdleOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetIdleOverrideParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setIdleOverride/parameters/isUserActive。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Mock isUserActive</pre>
                    ///
                    ///
                    #[serde(rename = "isUserActive")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub is_user_active: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setIdleOverride/parameters/isScreenUnlocked。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Mock isScreenUnlocked</pre>
                    ///
                    ///
                    #[serde(rename = "isScreenUnlocked")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub is_screen_unlocked: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetIdleOverrideParams {
                    is_user_active: fields.is_user_active,
                    is_screen_unlocked: fields.is_screen_unlocked,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setIdleOverride。
///
///协议原文：
///
///<pre>Overrides the Idle state.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetIdleOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setIdleOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetIdleOverrideResponse {}
impl<'de> serde::Deserialize<'de> for SetIdleOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetIdleOverrideResponse;
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
                Ok(SetIdleOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetIdleOverride {
    type Params = SetIdleOverrideParams;
    type Response = SetIdleOverrideResponse;
    const METHOD: &'static str = "Emulation.setIdleOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setLocaleOverride/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct SetLocaleOverrideParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setLocaleOverride/parameters/locale。
    ///
    ///协议原文：
    ///
    ///<pre>ICU style C locale (e.g. "en_US"). If not specified or empty, disables the override and
    ///restores default host system locale.</pre>
    ///
    ///
    #[serde(rename = "locale")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub locale: Option<String>,
}
impl<'de> serde::Deserialize<'de> for SetLocaleOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetLocaleOverrideParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setLocaleOverride/parameters/locale。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>ICU style C locale (e.g. "en_US"). If not specified or empty, disables the override and
                    ///restores default host system locale.</pre>
                    ///
                    ///
                    #[serde(rename = "locale")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub locale: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetLocaleOverrideParams {
                    locale: fields.locale,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setLocaleOverride。
///
///协议原文：
///
///<pre>Overrides default host system locale with the specified one.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetLocaleOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setLocaleOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetLocaleOverrideResponse {}
impl<'de> serde::Deserialize<'de> for SetLocaleOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetLocaleOverrideResponse;
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
                Ok(SetLocaleOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetLocaleOverride {
    type Params = SetLocaleOverrideParams;
    type Response = SetLocaleOverrideResponse;
    const METHOD: &'static str = "Emulation.setLocaleOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setNavigatorOverrides/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetNavigatorOverridesParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setNavigatorOverrides/parameters/platform。
    ///
    ///协议原文：
    ///
    ///<pre>The platform navigator.platform should return.</pre>
    ///
    ///
    #[serde(rename = "platform")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub platform: String,
}
impl<'de> serde::Deserialize<'de> for SetNavigatorOverridesParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetNavigatorOverridesParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setNavigatorOverrides/parameters/platform。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The platform navigator.platform should return.</pre>
                    ///
                    ///
                    #[serde(rename = "platform")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub platform: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetNavigatorOverridesParams {
                    platform: fields.platform,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setNavigatorOverrides。
///
///协议原文：
///
///<pre>Overrides value returned by the javascript navigator object.</pre>
///
///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct SetNavigatorOverrides;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setNavigatorOverrides/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetNavigatorOverridesResponse {}
impl<'de> serde::Deserialize<'de> for SetNavigatorOverridesResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetNavigatorOverridesResponse;
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
                Ok(SetNavigatorOverridesResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetNavigatorOverrides {
    type Params = SetNavigatorOverridesParams;
    type Response = SetNavigatorOverridesResponse;
    const METHOD: &'static str = "Emulation.setNavigatorOverrides";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setPageScaleFactor/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetPageScaleFactorParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setPageScaleFactor/parameters/pageScaleFactor。
    ///
    ///协议原文：
    ///
    ///<pre>Page scale factor.</pre>
    ///
    ///
    #[serde(rename = "pageScaleFactor")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub page_scale_factor: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for SetPageScaleFactorParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetPageScaleFactorParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setPageScaleFactor/parameters/pageScaleFactor。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Page scale factor.</pre>
                    ///
                    ///
                    #[serde(rename = "pageScaleFactor")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub page_scale_factor: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetPageScaleFactorParams {
                    page_scale_factor: fields.page_scale_factor,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setPageScaleFactor。
///
///协议原文：
///
///<pre>Sets a specified page scale factor.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetPageScaleFactor;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setPageScaleFactor/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetPageScaleFactorResponse {}
impl<'de> serde::Deserialize<'de> for SetPageScaleFactorResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetPageScaleFactorResponse;
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
                Ok(SetPageScaleFactorResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetPageScaleFactor {
    type Params = SetPageScaleFactorParams;
    type Response = SetPageScaleFactorResponse;
    const METHOD: &'static str = "Emulation.setPageScaleFactor";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setPressureSourceOverrideEnabled/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetPressureSourceOverrideEnabledParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setPressureSourceOverrideEnabled/parameters/enabled。
    #[serde(rename = "enabled")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub enabled: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setPressureSourceOverrideEnabled/parameters/source。
    #[serde(rename = "source")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub source: crate::emulation::types::PressureSource,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setPressureSourceOverrideEnabled/parameters/metadata。
    #[serde(rename = "metadata")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata: Option<crate::emulation::types::PressureMetadata>,
}
impl<'de> serde::Deserialize<'de> for SetPressureSourceOverrideEnabledParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetPressureSourceOverrideEnabledParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setPressureSourceOverrideEnabled/parameters/enabled。
                    #[serde(rename = "enabled")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub enabled: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setPressureSourceOverrideEnabled/parameters/source。
                    #[serde(rename = "source")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub source: crate::emulation::types::PressureSource,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setPressureSourceOverrideEnabled/parameters/metadata。
                    #[serde(rename = "metadata")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub metadata: Option<crate::emulation::types::PressureMetadata>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetPressureSourceOverrideEnabledParams {
                    enabled: fields.enabled,
                    source: fields.source,
                    metadata: fields.metadata,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setPressureSourceOverrideEnabled。
///
///协议原文：
///
///<pre>Overrides a pressure source of a given type, as used by the Compute
///Pressure API, so that updates to PressureObserver.observe() are provided
///via setPressureStateOverride instead of being retrieved from
///platform-provided telemetry data.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetPressureSourceOverrideEnabled;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setPressureSourceOverrideEnabled/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetPressureSourceOverrideEnabledResponse {}
impl<'de> serde::Deserialize<'de> for SetPressureSourceOverrideEnabledResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetPressureSourceOverrideEnabledResponse;
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
                Ok(SetPressureSourceOverrideEnabledResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetPressureSourceOverrideEnabled {
    type Params = SetPressureSourceOverrideEnabledParams;
    type Response = SetPressureSourceOverrideEnabledResponse;
    const METHOD: &'static str = "Emulation.setPressureSourceOverrideEnabled";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setPressureStateOverride/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetPressureStateOverrideParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setPressureStateOverride/parameters/source。
    #[serde(rename = "source")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub source: crate::emulation::types::PressureSource,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setPressureStateOverride/parameters/state。
    #[serde(rename = "state")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub state: crate::emulation::types::PressureState,
}
impl<'de> serde::Deserialize<'de> for SetPressureStateOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetPressureStateOverrideParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setPressureStateOverride/parameters/source。
                    #[serde(rename = "source")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub source: crate::emulation::types::PressureSource,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setPressureStateOverride/parameters/state。
                    #[serde(rename = "state")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub state: crate::emulation::types::PressureState,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetPressureStateOverrideParams {
                    source: fields.source,
                    state: fields.state,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setPressureStateOverride。
///
///协议原文：
///
///<pre>Provides a given pressure state that will be processed and eventually be
///delivered to PressureObserver users. |source| must have been previously
///overridden by setPressureSourceOverrideEnabled.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetPressureStateOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setPressureStateOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetPressureStateOverrideResponse {}
impl<'de> serde::Deserialize<'de> for SetPressureStateOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetPressureStateOverrideResponse;
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
                Ok(SetPressureStateOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetPressureStateOverride {
    type Params = SetPressureStateOverrideParams;
    type Response = SetPressureStateOverrideResponse;
    const METHOD: &'static str = "Emulation.setPressureStateOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setPrimaryScreen/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetPrimaryScreenParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setPrimaryScreen/parameters/screenId。
    #[serde(rename = "screenId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub screen_id: crate::emulation::types::ScreenId,
}
impl<'de> serde::Deserialize<'de> for SetPrimaryScreenParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetPrimaryScreenParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setPrimaryScreen/parameters/screenId。
                    #[serde(rename = "screenId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub screen_id: crate::emulation::types::ScreenId,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetPrimaryScreenParams {
                    screen_id: fields.screen_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setPrimaryScreen。
///
///协议原文：
///
///<pre>Set primary screen. Only supported in headless mode.
///Note that this changes the coordinate system origin to the top-left
///of the new primary screen, updating the bounds and work areas
///of all existing screens accordingly.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetPrimaryScreen;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setPrimaryScreen/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetPrimaryScreenResponse {}
impl<'de> serde::Deserialize<'de> for SetPrimaryScreenResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetPrimaryScreenResponse;
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
                Ok(SetPrimaryScreenResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetPrimaryScreen {
    type Params = SetPrimaryScreenParams;
    type Response = SetPrimaryScreenResponse;
    const METHOD: &'static str = "Emulation.setPrimaryScreen";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setSafeAreaInsetsOverride/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetSafeAreaInsetsOverrideParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setSafeAreaInsetsOverride/parameters/insets。
    #[serde(rename = "insets")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub insets: crate::emulation::types::SafeAreaInsets,
}
impl<'de> serde::Deserialize<'de> for SetSafeAreaInsetsOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetSafeAreaInsetsOverrideParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setSafeAreaInsetsOverride/parameters/insets。
                    #[serde(rename = "insets")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub insets: crate::emulation::types::SafeAreaInsets,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetSafeAreaInsetsOverrideParams {
                    insets: fields.insets,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setSafeAreaInsetsOverride。
///
///协议原文：
///
///<pre>Overrides the values for env(safe-area-inset-*) and env(safe-area-max-inset-*). Unset values will cause the
///respective variables to be undefined, even if previously overridden.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetSafeAreaInsetsOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setSafeAreaInsetsOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetSafeAreaInsetsOverrideResponse {}
impl<'de> serde::Deserialize<'de> for SetSafeAreaInsetsOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetSafeAreaInsetsOverrideResponse;
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
                Ok(SetSafeAreaInsetsOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetSafeAreaInsetsOverride {
    type Params = SetSafeAreaInsetsOverrideParams;
    type Response = SetSafeAreaInsetsOverrideResponse;
    const METHOD: &'static str = "Emulation.setSafeAreaInsetsOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setScriptExecutionDisabled/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetScriptExecutionDisabledParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setScriptExecutionDisabled/parameters/value。
    ///
    ///协议原文：
    ///
    ///<pre>Whether script execution should be disabled in the page.</pre>
    ///
    ///
    #[serde(rename = "value")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub value: bool,
}
impl<'de> serde::Deserialize<'de> for SetScriptExecutionDisabledParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetScriptExecutionDisabledParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setScriptExecutionDisabled/parameters/value。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether script execution should be disabled in the page.</pre>
                    ///
                    ///
                    #[serde(rename = "value")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub value: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetScriptExecutionDisabledParams {
                    value: fields.value,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setScriptExecutionDisabled。
///
///协议原文：
///
///<pre>Switches script execution in the page.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetScriptExecutionDisabled;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setScriptExecutionDisabled/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetScriptExecutionDisabledResponse {}
impl<'de> serde::Deserialize<'de> for SetScriptExecutionDisabledResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetScriptExecutionDisabledResponse;
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
                Ok(SetScriptExecutionDisabledResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetScriptExecutionDisabled {
    type Params = SetScriptExecutionDisabledParams;
    type Response = SetScriptExecutionDisabledResponse;
    const METHOD: &'static str = "Emulation.setScriptExecutionDisabled";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setScrollbarsHidden/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetScrollbarsHiddenParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setScrollbarsHidden/parameters/hidden。
    ///
    ///协议原文：
    ///
    ///<pre>Whether scrollbars should be always hidden.</pre>
    ///
    ///
    #[serde(rename = "hidden")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub hidden: bool,
}
impl<'de> serde::Deserialize<'de> for SetScrollbarsHiddenParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetScrollbarsHiddenParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setScrollbarsHidden/parameters/hidden。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether scrollbars should be always hidden.</pre>
                    ///
                    ///
                    #[serde(rename = "hidden")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub hidden: bool,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetScrollbarsHiddenParams {
                    hidden: fields.hidden,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setScrollbarsHidden。此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetScrollbarsHidden;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setScrollbarsHidden/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetScrollbarsHiddenResponse {}
impl<'de> serde::Deserialize<'de> for SetScrollbarsHiddenResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetScrollbarsHiddenResponse;
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
                Ok(SetScrollbarsHiddenResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetScrollbarsHidden {
    type Params = SetScrollbarsHiddenParams;
    type Response = SetScrollbarsHiddenResponse;
    const METHOD: &'static str = "Emulation.setScrollbarsHidden";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setSensorOverrideEnabled/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetSensorOverrideEnabledParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setSensorOverrideEnabled/parameters/enabled。
    #[serde(rename = "enabled")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub enabled: bool,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setSensorOverrideEnabled/parameters/type。
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: crate::emulation::types::SensorType,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setSensorOverrideEnabled/parameters/metadata。
    #[serde(rename = "metadata")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata: Option<crate::emulation::types::SensorMetadata>,
}
impl<'de> serde::Deserialize<'de> for SetSensorOverrideEnabledParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetSensorOverrideEnabledParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setSensorOverrideEnabled/parameters/enabled。
                    #[serde(rename = "enabled")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub enabled: bool,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setSensorOverrideEnabled/parameters/type。
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: crate::emulation::types::SensorType,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setSensorOverrideEnabled/parameters/metadata。
                    #[serde(rename = "metadata")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub metadata: Option<crate::emulation::types::SensorMetadata>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetSensorOverrideEnabledParams {
                    enabled: fields.enabled,
                    r#type: fields.r#type,
                    metadata: fields.metadata,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setSensorOverrideEnabled。
///
///协议原文：
///
///<pre>Overrides a platform sensor of a given type. If |enabled| is true, calls to
///Sensor.start() will use a virtual sensor as backend rather than fetching
///data from a real hardware sensor. Otherwise, existing virtual
///sensor-backend Sensor objects will fire an error event and new calls to
///Sensor.start() will attempt to use a real sensor instead.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetSensorOverrideEnabled;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setSensorOverrideEnabled/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetSensorOverrideEnabledResponse {}
impl<'de> serde::Deserialize<'de> for SetSensorOverrideEnabledResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetSensorOverrideEnabledResponse;
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
                Ok(SetSensorOverrideEnabledResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetSensorOverrideEnabled {
    type Params = SetSensorOverrideEnabledParams;
    type Response = SetSensorOverrideEnabledResponse;
    const METHOD: &'static str = "Emulation.setSensorOverrideEnabled";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setSensorOverrideReadings/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetSensorOverrideReadingsParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setSensorOverrideReadings/parameters/type。
    #[serde(rename = "type")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub r#type: crate::emulation::types::SensorType,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setSensorOverrideReadings/parameters/reading。
    #[serde(rename = "reading")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub reading: crate::emulation::types::SensorReading,
}
impl<'de> serde::Deserialize<'de> for SetSensorOverrideReadingsParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetSensorOverrideReadingsParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setSensorOverrideReadings/parameters/type。
                    #[serde(rename = "type")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub r#type: crate::emulation::types::SensorType,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setSensorOverrideReadings/parameters/reading。
                    #[serde(rename = "reading")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub reading: crate::emulation::types::SensorReading,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetSensorOverrideReadingsParams {
                    r#type: fields.r#type,
                    reading: fields.reading,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setSensorOverrideReadings。
///
///协议原文：
///
///<pre>Updates the sensor readings reported by a sensor type previously overridden
///by setSensorOverrideEnabled.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetSensorOverrideReadings;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setSensorOverrideReadings/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetSensorOverrideReadingsResponse {}
impl<'de> serde::Deserialize<'de> for SetSensorOverrideReadingsResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetSensorOverrideReadingsResponse;
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
                Ok(SetSensorOverrideReadingsResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetSensorOverrideReadings {
    type Params = SetSensorOverrideReadingsParams;
    type Response = SetSensorOverrideReadingsResponse;
    const METHOD: &'static str = "Emulation.setSensorOverrideReadings";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setSmallViewportHeightDifferenceOverride/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetSmallViewportHeightDifferenceOverrideParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setSmallViewportHeightDifferenceOverride/parameters/difference。
    ///
    ///协议原文：
    ///
    ///<pre>This will cause an element of size 100svh to be `difference` pixels smaller than an element
    ///of size 100lvh.</pre>
    ///
    ///
    #[serde(rename = "difference")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub difference: i64,
}
impl<'de> serde::Deserialize<'de> for SetSmallViewportHeightDifferenceOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetSmallViewportHeightDifferenceOverrideParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setSmallViewportHeightDifferenceOverride/parameters/difference。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>This will cause an element of size 100svh to be `difference` pixels smaller than an element
                    ///of size 100lvh.</pre>
                    ///
                    ///
                    #[serde(rename = "difference")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub difference: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetSmallViewportHeightDifferenceOverrideParams {
                    difference: fields.difference,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setSmallViewportHeightDifferenceOverride。
///
///协议原文：
///
///<pre>Allows overriding the difference between the small and large viewport sizes, which determine the
///value of the `svh` and `lvh` unit, respectively. Only supported for top-level frames.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetSmallViewportHeightDifferenceOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setSmallViewportHeightDifferenceOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetSmallViewportHeightDifferenceOverrideResponse {}
impl<'de> serde::Deserialize<'de> for SetSmallViewportHeightDifferenceOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetSmallViewportHeightDifferenceOverrideResponse;
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
                Ok(SetSmallViewportHeightDifferenceOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetSmallViewportHeightDifferenceOverride {
    type Params = SetSmallViewportHeightDifferenceOverrideParams;
    type Response = SetSmallViewportHeightDifferenceOverrideResponse;
    const METHOD: &'static str = "Emulation.setSmallViewportHeightDifferenceOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setTimezoneOverride/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetTimezoneOverrideParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setTimezoneOverride/parameters/timezoneId。
    ///
    ///协议原文：
    ///
    ///<pre>The timezone identifier. List of supported timezones:
    ///https://source.chromium.org/chromium/chromium/deps/icu.git/+/faee8bc70570192d82d2978a71e2a615788597d1:source/data/misc/metaZones.txt
    ///If empty, disables the override and restores default host system timezone.</pre>
    ///
    ///
    #[serde(rename = "timezoneId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub timezone_id: String,
}
impl<'de> serde::Deserialize<'de> for SetTimezoneOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetTimezoneOverrideParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setTimezoneOverride/parameters/timezoneId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The timezone identifier. List of supported timezones:
                    ///https://source.chromium.org/chromium/chromium/deps/icu.git/+/faee8bc70570192d82d2978a71e2a615788597d1:source/data/misc/metaZones.txt
                    ///If empty, disables the override and restores default host system timezone.</pre>
                    ///
                    ///
                    #[serde(rename = "timezoneId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub timezone_id: String,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetTimezoneOverrideParams {
                    timezone_id: fields.timezone_id,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setTimezoneOverride。
///
///协议原文：
///
///<pre>Overrides default host system timezone with the specified one.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetTimezoneOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setTimezoneOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetTimezoneOverrideResponse {}
impl<'de> serde::Deserialize<'de> for SetTimezoneOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetTimezoneOverrideResponse;
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
                Ok(SetTimezoneOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetTimezoneOverride {
    type Params = SetTimezoneOverrideParams;
    type Response = SetTimezoneOverrideResponse;
    const METHOD: &'static str = "Emulation.setTimezoneOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setTouchEmulationEnabled/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetTouchEmulationEnabledParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setTouchEmulationEnabled/parameters/enabled。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the touch event emulation should be enabled.</pre>
    ///
    ///
    #[serde(rename = "enabled")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub enabled: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setTouchEmulationEnabled/parameters/maxTouchPoints。
    ///
    ///协议原文：
    ///
    ///<pre>Maximum touch points supported. Defaults to one.</pre>
    ///
    ///
    #[serde(rename = "maxTouchPoints")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_touch_points: Option<i64>,
}
impl<'de> serde::Deserialize<'de> for SetTouchEmulationEnabledParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetTouchEmulationEnabledParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setTouchEmulationEnabled/parameters/enabled。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the touch event emulation should be enabled.</pre>
                    ///
                    ///
                    #[serde(rename = "enabled")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub enabled: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setTouchEmulationEnabled/parameters/maxTouchPoints。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Maximum touch points supported. Defaults to one.</pre>
                    ///
                    ///
                    #[serde(rename = "maxTouchPoints")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub max_touch_points: Option<i64>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetTouchEmulationEnabledParams {
                    enabled: fields.enabled,
                    max_touch_points: fields.max_touch_points,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setTouchEmulationEnabled。
///
///协议原文：
///
///<pre>Enables touch on platforms which do not support them.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetTouchEmulationEnabled;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setTouchEmulationEnabled/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetTouchEmulationEnabledResponse {}
impl<'de> serde::Deserialize<'de> for SetTouchEmulationEnabledResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetTouchEmulationEnabledResponse;
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
                Ok(SetTouchEmulationEnabledResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetTouchEmulationEnabled {
    type Params = SetTouchEmulationEnabledParams;
    type Response = SetTouchEmulationEnabledResponse;
    const METHOD: &'static str = "Emulation.setTouchEmulationEnabled";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setUserAgentOverride/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetUserAgentOverrideParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setUserAgentOverride/parameters/userAgent。
    ///
    ///协议原文：
    ///
    ///<pre>User agent to use.</pre>
    ///
    ///
    #[serde(rename = "userAgent")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub user_agent: String,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setUserAgentOverride/parameters/acceptLanguage。
    ///
    ///协议原文：
    ///
    ///<pre>Browser language to emulate.</pre>
    ///
    ///
    #[serde(rename = "acceptLanguage")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub accept_language: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setUserAgentOverride/parameters/platform。
    ///
    ///协议原文：
    ///
    ///<pre>The platform navigator.platform should return.</pre>
    ///
    ///
    #[serde(rename = "platform")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub platform: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setUserAgentOverride/parameters/userAgentMetadata。
    ///
    ///协议原文：
    ///
    ///<pre>To be sent in Sec-CH-UA-* headers and returned in navigator.userAgentData</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    #[serde(rename = "userAgentMetadata")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_agent_metadata: Option<crate::emulation::types::UserAgentMetadata>,
}
impl<'de> serde::Deserialize<'de> for SetUserAgentOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetUserAgentOverrideParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setUserAgentOverride/parameters/userAgent。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>User agent to use.</pre>
                    ///
                    ///
                    #[serde(rename = "userAgent")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub user_agent: String,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setUserAgentOverride/parameters/acceptLanguage。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Browser language to emulate.</pre>
                    ///
                    ///
                    #[serde(rename = "acceptLanguage")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub accept_language: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setUserAgentOverride/parameters/platform。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The platform navigator.platform should return.</pre>
                    ///
                    ///
                    #[serde(rename = "platform")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub platform: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setUserAgentOverride/parameters/userAgentMetadata。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>To be sent in Sec-CH-UA-* headers and returned in navigator.userAgentData</pre>
                    ///
                    ///此项为 experimental，端点可能不支持。
                    #[serde(rename = "userAgentMetadata")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub user_agent_metadata: Option<crate::emulation::types::UserAgentMetadata>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetUserAgentOverrideParams {
                    user_agent: fields.user_agent,
                    accept_language: fields.accept_language,
                    platform: fields.platform,
                    user_agent_metadata: fields.user_agent_metadata,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setUserAgentOverride。
///
///协议原文：
///
///<pre>Allows overriding user agent with the given string.
///`userAgentMetadata` must be set for Client Hint headers to be sent.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct SetUserAgentOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setUserAgentOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetUserAgentOverrideResponse {}
impl<'de> serde::Deserialize<'de> for SetUserAgentOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetUserAgentOverrideResponse;
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
                Ok(SetUserAgentOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetUserAgentOverride {
    type Params = SetUserAgentOverrideParams;
    type Response = SetUserAgentOverrideResponse;
    const METHOD: &'static str = "Emulation.setUserAgentOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setVirtualKeyboardGeometryOverride/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct SetVirtualKeyboardGeometryOverrideParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setVirtualKeyboardGeometryOverride/parameters/keyboardRect。
    #[serde(rename = "keyboardRect")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub keyboard_rect: Option<crate::dom::types::Rect>,
}
impl<'de> serde::Deserialize<'de> for SetVirtualKeyboardGeometryOverrideParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetVirtualKeyboardGeometryOverrideParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setVirtualKeyboardGeometryOverride/parameters/keyboardRect。
                    #[serde(rename = "keyboardRect")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub keyboard_rect: Option<crate::dom::types::Rect>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetVirtualKeyboardGeometryOverrideParams {
                    keyboard_rect: fields.keyboard_rect,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setVirtualKeyboardGeometryOverride。
///
///协议原文：
///
///<pre>Overrides virtual keyboard geometry in CSS pixels, relative to the top-level viewport. The
///provided rect is used for navigator.virtualKeyboard.boundingRect, geometrychange events, and
///env(keyboard-inset-*) values on the inspected frame. The override applies independently of
///navigator.virtualKeyboard.overlaysContent so clients can preview overlay geometry without
///mutating page state. Values are rounded to the nearest CSS pixel. Omitting the rect clears the
///override.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetVirtualKeyboardGeometryOverride;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setVirtualKeyboardGeometryOverride/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetVirtualKeyboardGeometryOverrideResponse {}
impl<'de> serde::Deserialize<'de> for SetVirtualKeyboardGeometryOverrideResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetVirtualKeyboardGeometryOverrideResponse;
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
                Ok(SetVirtualKeyboardGeometryOverrideResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetVirtualKeyboardGeometryOverride {
    type Params = SetVirtualKeyboardGeometryOverrideParams;
    type Response = SetVirtualKeyboardGeometryOverrideResponse;
    const METHOD: &'static str = "Emulation.setVirtualKeyboardGeometryOverride";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setVirtualTimePolicy/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetVirtualTimePolicyParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setVirtualTimePolicy/parameters/policy。
    #[serde(rename = "policy")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub policy: crate::emulation::types::VirtualTimePolicy,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setVirtualTimePolicy/parameters/budget。
    ///
    ///协议原文：
    ///
    ///<pre>If set, after this many virtual milliseconds have elapsed virtual time will be paused and a
    ///virtualTimeBudgetExpired event is sent.</pre>
    ///
    ///
    #[serde(rename = "budget")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub budget: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setVirtualTimePolicy/parameters/maxVirtualTimeTaskStarvationCount。
    ///
    ///协议原文：
    ///
    ///<pre>If set this specifies the maximum number of tasks that can be run before virtual is forced
    ///forwards to prevent deadlock.</pre>
    ///
    ///
    #[serde(rename = "maxVirtualTimeTaskStarvationCount")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_virtual_time_task_starvation_count: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setVirtualTimePolicy/parameters/initialVirtualTime。
    ///
    ///协议原文：
    ///
    ///<pre>If set, base::Time::Now will be overridden to initially return this value.</pre>
    ///
    ///
    #[serde(rename = "initialVirtualTime")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_virtual_time: Option<crate::network::types::TimeSinceEpoch>,
}
impl<'de> serde::Deserialize<'de> for SetVirtualTimePolicyParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetVirtualTimePolicyParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setVirtualTimePolicy/parameters/policy。
                    #[serde(rename = "policy")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub policy: crate::emulation::types::VirtualTimePolicy,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setVirtualTimePolicy/parameters/budget。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If set, after this many virtual milliseconds have elapsed virtual time will be paused and a
                    ///virtualTimeBudgetExpired event is sent.</pre>
                    ///
                    ///
                    #[serde(rename = "budget")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub budget: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setVirtualTimePolicy/parameters/maxVirtualTimeTaskStarvationCount。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If set this specifies the maximum number of tasks that can be run before virtual is forced
                    ///forwards to prevent deadlock.</pre>
                    ///
                    ///
                    #[serde(rename = "maxVirtualTimeTaskStarvationCount")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub max_virtual_time_task_starvation_count: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.setVirtualTimePolicy/parameters/initialVirtualTime。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If set, base::Time::Now will be overridden to initially return this value.</pre>
                    ///
                    ///
                    #[serde(rename = "initialVirtualTime")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub initial_virtual_time: Option<crate::network::types::TimeSinceEpoch>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetVirtualTimePolicyParams {
                    policy: fields.policy,
                    budget: fields.budget,
                    max_virtual_time_task_starvation_count: fields
                        .max_virtual_time_task_starvation_count,
                    initial_virtual_time: fields.initial_virtual_time,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setVirtualTimePolicy。
///
///协议原文：
///
///<pre>Turns on virtual time for all frames (replacing real-time with a synthetic time source) and sets
///the current virtual time policy.  Note this supersedes any previous time budget.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct SetVirtualTimePolicy;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setVirtualTimePolicy/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetVirtualTimePolicyResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setVirtualTimePolicy/returns/virtualTimeTicksBase。
    ///
    ///协议原文：
    ///
    ///<pre>Absolute timestamp at which virtual time was first enabled (up time in milliseconds).</pre>
    ///
    ///
    #[serde(rename = "virtualTimeTicksBase")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub virtual_time_ticks_base: ydp_core::FiniteF64,
}
impl<'de> serde::Deserialize<'de> for SetVirtualTimePolicyResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetVirtualTimePolicyResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setVirtualTimePolicy/returns/virtualTimeTicksBase。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Absolute timestamp at which virtual time was first enabled (up time in milliseconds).</pre>
                    ///
                    ///
                    #[serde(rename = "virtualTimeTicksBase")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub virtual_time_ticks_base: ydp_core::FiniteF64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetVirtualTimePolicyResponse {
                    virtual_time_ticks_base: fields.virtual_time_ticks_base,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetVirtualTimePolicy {
    type Params = SetVirtualTimePolicyParams;
    type Response = SetVirtualTimePolicyResponse;
    const METHOD: &'static str = "Emulation.setVirtualTimePolicy";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setVisibleSize/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetVisibleSizeParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setVisibleSize/parameters/width。
    ///
    ///协议原文：
    ///
    ///<pre>Frame width (DIP).</pre>
    ///
    ///
    #[serde(rename = "width")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub width: i64,
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setVisibleSize/parameters/height。
    ///
    ///协议原文：
    ///
    ///<pre>Frame height (DIP).</pre>
    ///
    ///
    #[serde(rename = "height")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub height: i64,
}
impl<'de> serde::Deserialize<'de> for SetVisibleSizeParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetVisibleSizeParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setVisibleSize/parameters/width。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame width (DIP).</pre>
                    ///
                    ///
                    #[serde(rename = "width")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub width: i64,
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.setVisibleSize/parameters/height。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Frame height (DIP).</pre>
                    ///
                    ///
                    #[serde(rename = "height")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub height: i64,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(SetVisibleSizeParams {
                    width: fields.width,
                    height: fields.height,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.setVisibleSize。
///
///协议原文：
///
///<pre>Resizes the frame/viewport of the page. Note that this does not affect the frame's container
///(e.g. browser window). Can be used to produce screenshots of the specified size. Not supported
///on Android.</pre>
///
///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct SetVisibleSize;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.setVisibleSize/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct SetVisibleSizeResponse {}
impl<'de> serde::Deserialize<'de> for SetVisibleSizeResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SetVisibleSizeResponse;
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
                Ok(SetVisibleSizeResponse {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for SetVisibleSize {
    type Params = SetVisibleSizeParams;
    type Response = SetVisibleSizeResponse;
    const METHOD: &'static str = "Emulation.setVisibleSize";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.updateScreen/parameters。
#[derive(Debug, Clone, serde::Serialize)]
pub struct UpdateScreenParams {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.updateScreen/parameters/screenId。
    ///
    ///协议原文：
    ///
    ///<pre>Target screen identifier.</pre>
    ///
    ///
    #[serde(rename = "screenId")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub screen_id: crate::emulation::types::ScreenId,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.updateScreen/parameters/left。
    ///
    ///协议原文：
    ///
    ///<pre>Offset of the left edge of the screen in pixels.</pre>
    ///
    ///
    #[serde(rename = "left")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub left: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.updateScreen/parameters/top。
    ///
    ///协议原文：
    ///
    ///<pre>Offset of the top edge of the screen in pixels.</pre>
    ///
    ///
    #[serde(rename = "top")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub top: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.updateScreen/parameters/width。
    ///
    ///协议原文：
    ///
    ///<pre>The width of the screen in pixels.</pre>
    ///
    ///
    #[serde(rename = "width")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub width: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.updateScreen/parameters/height。
    ///
    ///协议原文：
    ///
    ///<pre>The height of the screen in pixels.</pre>
    ///
    ///
    #[serde(rename = "height")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub height: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.updateScreen/parameters/workAreaInsets。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies the screen's work area.</pre>
    ///
    ///
    #[serde(rename = "workAreaInsets")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub work_area_insets: Option<crate::emulation::types::WorkAreaInsets>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.updateScreen/parameters/devicePixelRatio。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies the screen's device pixel ratio.</pre>
    ///
    ///
    #[serde(rename = "devicePixelRatio")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub device_pixel_ratio: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.updateScreen/parameters/rotation。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies the screen's rotation angle. Available values are 0, 90, 180 and 270.</pre>
    ///
    ///
    #[serde(rename = "rotation")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub rotation: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.updateScreen/parameters/colorDepth。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies the screen's color depth in bits.</pre>
    ///
    ///
    #[serde(rename = "colorDepth")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub color_depth: Option<i64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.updateScreen/parameters/label。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies the descriptive label for the screen.</pre>
    ///
    ///
    #[serde(rename = "label")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub label: Option<String>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.updateScreen/parameters/isInternal。
    ///
    ///协议原文：
    ///
    ///<pre>Indicates whether the screen is internal to the device or external, attached to the device. Default is false.</pre>
    ///
    ///
    #[serde(rename = "isInternal")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_internal: Option<bool>,
}
impl<'de> serde::Deserialize<'de> for UpdateScreenParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UpdateScreenParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.updateScreen/parameters/screenId。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Target screen identifier.</pre>
                    ///
                    ///
                    #[serde(rename = "screenId")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub screen_id: crate::emulation::types::ScreenId,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.updateScreen/parameters/left。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Offset of the left edge of the screen in pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "left")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub left: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.updateScreen/parameters/top。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Offset of the top edge of the screen in pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "top")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub top: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.updateScreen/parameters/width。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The width of the screen in pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "width")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub width: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.updateScreen/parameters/height。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The height of the screen in pixels.</pre>
                    ///
                    ///
                    #[serde(rename = "height")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub height: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.updateScreen/parameters/workAreaInsets。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies the screen's work area.</pre>
                    ///
                    ///
                    #[serde(rename = "workAreaInsets")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub work_area_insets: Option<crate::emulation::types::WorkAreaInsets>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.updateScreen/parameters/devicePixelRatio。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies the screen's device pixel ratio.</pre>
                    ///
                    ///
                    #[serde(rename = "devicePixelRatio")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub device_pixel_ratio: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.updateScreen/parameters/rotation。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies the screen's rotation angle. Available values are 0, 90, 180 and 270.</pre>
                    ///
                    ///
                    #[serde(rename = "rotation")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub rotation: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.updateScreen/parameters/colorDepth。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies the screen's color depth in bits.</pre>
                    ///
                    ///
                    #[serde(rename = "colorDepth")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub color_depth: Option<i64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.updateScreen/parameters/label。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Specifies the descriptive label for the screen.</pre>
                    ///
                    ///
                    #[serde(rename = "label")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub label: Option<String>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：Emulation.command.updateScreen/parameters/isInternal。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Indicates whether the screen is internal to the device or external, attached to the device. Default is false.</pre>
                    ///
                    ///
                    #[serde(rename = "isInternal")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub is_internal: Option<bool>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(UpdateScreenParams {
                    screen_id: fields.screen_id,
                    left: fields.left,
                    top: fields.top,
                    width: fields.width,
                    height: fields.height,
                    work_area_insets: fields.work_area_insets,
                    device_pixel_ratio: fields.device_pixel_ratio,
                    rotation: fields.rotation,
                    color_depth: fields.color_depth,
                    label: fields.label,
                    is_internal: fields.is_internal,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：Emulation.command.updateScreen。
///
///协议原文：
///
///<pre>Updates specified screen parameters. Only supported in headless mode.</pre>
///
///此项为 experimental，端点可能不支持。
#[derive(Debug, Clone, Copy)]
pub struct UpdateScreen;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：Emulation.command.updateScreen/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct UpdateScreenResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.updateScreen/returns/screenInfo。
    #[serde(rename = "screenInfo")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub screen_info: crate::emulation::types::ScreenInfo,
}
impl<'de> serde::Deserialize<'de> for UpdateScreenResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UpdateScreenResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：Emulation.command.updateScreen/returns/screenInfo。
                    #[serde(rename = "screenInfo")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub screen_info: crate::emulation::types::ScreenInfo,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(UpdateScreenResponse {
                    screen_info: fields.screen_info,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for UpdateScreen {
    type Params = UpdateScreenParams;
    type Response = UpdateScreenResponse;
    const METHOD: &'static str = "Emulation.updateScreen";
}
