///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeadlessExperimental.command.beginFrame/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct BeginFrameParams {
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeadlessExperimental.command.beginFrame/parameters/frameTimeTicks。
    ///
    ///协议原文：
    ///
    ///<pre>Timestamp of this BeginFrame in Renderer TimeTicks (milliseconds of uptime). If not set,
    ///the current time will be used.</pre>
    ///
    ///
    #[serde(rename = "frameTimeTicks")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub frame_time_ticks: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeadlessExperimental.command.beginFrame/parameters/interval。
    ///
    ///协议原文：
    ///
    ///<pre>The interval between BeginFrames that is reported to the compositor, in milliseconds.
    ///Defaults to a 60 frames/second interval, i.e. about 16.666 milliseconds.</pre>
    ///
    ///
    #[serde(rename = "interval")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub interval: Option<ydp_core::FiniteF64>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeadlessExperimental.command.beginFrame/parameters/noDisplayUpdates。
    ///
    ///协议原文：
    ///
    ///<pre>Whether updates should not be committed and drawn onto the display. False by default. If
    ///true, only side effects of the BeginFrame will be run, such as layout and animations, but
    ///any visual updates may not be visible on the display or in screenshots.</pre>
    ///
    ///
    #[serde(rename = "noDisplayUpdates")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub no_display_updates: Option<bool>,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeadlessExperimental.command.beginFrame/parameters/screenshot。
    ///
    ///协议原文：
    ///
    ///<pre>If set, a screenshot of the frame will be captured and returned in the response. Otherwise,
    ///no screenshot will be captured. Note that capturing a screenshot can fail, for example,
    ///during renderer initialization. In such a case, no screenshot data will be returned.</pre>
    ///
    ///
    #[serde(rename = "screenshot")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub screenshot: Option<crate::headless_experimental::types::ScreenshotParams>,
}
impl<'de> serde::Deserialize<'de> for BeginFrameParams {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BeginFrameParams;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeadlessExperimental.command.beginFrame/parameters/frameTimeTicks。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Timestamp of this BeginFrame in Renderer TimeTicks (milliseconds of uptime). If not set,
                    ///the current time will be used.</pre>
                    ///
                    ///
                    #[serde(rename = "frameTimeTicks")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub frame_time_ticks: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeadlessExperimental.command.beginFrame/parameters/interval。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>The interval between BeginFrames that is reported to the compositor, in milliseconds.
                    ///Defaults to a 60 frames/second interval, i.e. about 16.666 milliseconds.</pre>
                    ///
                    ///
                    #[serde(rename = "interval")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub interval: Option<ydp_core::FiniteF64>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeadlessExperimental.command.beginFrame/parameters/noDisplayUpdates。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether updates should not be committed and drawn onto the display. False by default. If
                    ///true, only side effects of the BeginFrame will be run, such as layout and animations, but
                    ///any visual updates may not be visible on the display or in screenshots.</pre>
                    ///
                    ///
                    #[serde(rename = "noDisplayUpdates")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub no_display_updates: Option<bool>,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeadlessExperimental.command.beginFrame/parameters/screenshot。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>If set, a screenshot of the frame will be captured and returned in the response. Otherwise,
                    ///no screenshot will be captured. Note that capturing a screenshot can fail, for example,
                    ///during renderer initialization. In such a case, no screenshot data will be returned.</pre>
                    ///
                    ///
                    #[serde(rename = "screenshot")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub screenshot: Option<crate::headless_experimental::types::ScreenshotParams>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(BeginFrameParams {
                    frame_time_ticks: fields.frame_time_ticks,
                    interval: fields.interval,
                    no_display_updates: fields.no_display_updates,
                    screenshot: fields.screenshot,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：HeadlessExperimental.command.beginFrame。
///
///协议原文：
///
///<pre>Sends a BeginFrame to the target and returns when the frame was completed. Optionally captures a
///screenshot from the resulting frame. Requires that the target was created with enabled
///BeginFrameControl. Designed for use with --run-all-compositor-stages-before-draw, see also
///https://goo.gle/chrome-headless-rendering for more background.</pre>
///
///
#[derive(Debug, Clone, Copy)]
pub struct BeginFrame;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeadlessExperimental.command.beginFrame/returns。
#[derive(Debug, Clone, serde::Serialize)]
pub struct BeginFrameResponse {
    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeadlessExperimental.command.beginFrame/returns/hasDamage。
    ///
    ///协议原文：
    ///
    ///<pre>Whether the BeginFrame resulted in damage and, thus, a new frame was committed to the
    ///display. Reported for diagnostic uses, may be removed in the future.</pre>
    ///
    ///
    #[serde(rename = "hasDamage")]
    #[serde(deserialize_with = "ydp_core::binding_support::required")]
    pub has_damage: bool,
    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeadlessExperimental.command.beginFrame/returns/screenshotData。
    ///
    ///协议原文：
    ///
    ///<pre>Base64-encoded image data of the screenshot, if one was requested and successfully taken. (Encoded as a base64 string when passed over JSON)</pre>
    ///
    ///
    #[serde(rename = "screenshotData")]
    #[serde(
        default,
        deserialize_with = "ydp_core::binding_support::presence",
        skip_serializing_if = "Option::is_none"
    )]
    pub screenshot_data: Option<String>,
}
impl<'de> serde::Deserialize<'de> for BeginFrameResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BeginFrameResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("协议 object")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                a: A,
            ) -> Result<Self::Value, A::Error> {
                #[derive(serde::Deserialize)]
                struct Fields {
                    ///必需字段；缺失或类型错误导致解码失败。协议来源：HeadlessExperimental.command.beginFrame/returns/hasDamage。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Whether the BeginFrame resulted in damage and, thus, a new frame was committed to the
                    ///display. Reported for diagnostic uses, may be removed in the future.</pre>
                    ///
                    ///
                    #[serde(rename = "hasDamage")]
                    #[serde(deserialize_with = "ydp_core::binding_support::required")]
                    pub has_damage: bool,
                    ///可缺失字段；None 编码时省略，存在时按声明类型校验。协议来源：HeadlessExperimental.command.beginFrame/returns/screenshotData。
                    ///
                    ///协议原文：
                    ///
                    ///<pre>Base64-encoded image data of the screenshot, if one was requested and successfully taken. (Encoded as a base64 string when passed over JSON)</pre>
                    ///
                    ///
                    #[serde(rename = "screenshotData")]
                    #[serde(
                        default,
                        deserialize_with = "ydp_core::binding_support::presence",
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub screenshot_data: Option<String>,
                }
                let fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(BeginFrameResponse {
                    has_damage: fields.has_damage,
                    screenshot_data: fields.screenshot_data,
                })
            }
        }
        d.deserialize_map(Visitor)
    }
}
impl ydp_core::Command for BeginFrame {
    type Params = BeginFrameParams;
    type Response = BeginFrameResponse;
    const METHOD: &'static str = "HeadlessExperimental.beginFrame";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeadlessExperimental.command.disable/parameters。
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
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：HeadlessExperimental.command.disable。
///
///协议原文：
///
///<pre>Disables headless events for the target.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct Disable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeadlessExperimental.command.disable/returns。
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
    const METHOD: &'static str = "HeadlessExperimental.disable";
}
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeadlessExperimental.command.enable/parameters。
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct EnableParams {}
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
                struct Fields {}
                let _fields = <Fields as serde::Deserialize>::deserialize(
                    serde::de::value::MapAccessDeserializer::new(a),
                )?;
                Ok(EnableParams {})
            }
        }
        d.deserialize_map(Visitor)
    }
}
///无状态协议 marker；METHOD 保持 wire 名，不启动自动化行为。协议来源：HeadlessExperimental.command.enable。
///
///协议原文：
///
///<pre>Enables headless events for the target.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。
#[derive(Debug, Clone, Copy)]
pub struct Enable;
///拥有型协议数据；Clone 复制字段，未知 object 字段容忍但不保留。协议来源：HeadlessExperimental.command.enable/returns。
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
    const METHOD: &'static str = "HeadlessExperimental.enable";
}
