///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：Emulation。
///
///协议原文：
///
///<pre>This domain emulates different environments for the page.</pre>
///
///
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.addScreen。
    ///
    ///协议原文：
    ///
    ///<pre>Add a new screen to the device. Only supported in headless mode.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::AddScreenParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().add_screen(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn add_screen(
        &self,
        params: commands::AddScreenParams,
    ) -> ydp_core::Request<commands::AddScreen> {
        self.session.send::<commands::AddScreen>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.canEmulate。
    ///
    ///协议原文：
    ///
    ///<pre>Tells whether emulation is supported.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::CanEmulateParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().can_emulate(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn can_emulate(
        &self,
        params: commands::CanEmulateParams,
    ) -> ydp_core::Request<commands::CanEmulate> {
        self.session.send::<commands::CanEmulate>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.clearDeviceMetricsOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Clears the overridden device metrics.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::ClearDeviceMetricsOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().clear_device_metrics_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn clear_device_metrics_override(
        &self,
        params: commands::ClearDeviceMetricsOverrideParams,
    ) -> ydp_core::Request<commands::ClearDeviceMetricsOverride> {
        self.session
            .send::<commands::ClearDeviceMetricsOverride>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.clearDevicePostureOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Clears a device posture override set with either setDeviceMetricsOverride()
    ///or setDevicePostureOverride() and starts using posture information from the
    ///platform again.
    ///Does nothing if no override is set.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::ClearDevicePostureOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().clear_device_posture_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn clear_device_posture_override(
        &self,
        params: commands::ClearDevicePostureOverrideParams,
    ) -> ydp_core::Request<commands::ClearDevicePostureOverride> {
        self.session
            .send::<commands::ClearDevicePostureOverride>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.clearDisplayFeaturesOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Clears the display features override set with either setDeviceMetricsOverride()
    ///or setDisplayFeaturesOverride() and starts using display features from the
    ///platform again.
    ///Does nothing if no override is set.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::ClearDisplayFeaturesOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().clear_display_features_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn clear_display_features_override(
        &self,
        params: commands::ClearDisplayFeaturesOverrideParams,
    ) -> ydp_core::Request<commands::ClearDisplayFeaturesOverride> {
        self.session
            .send::<commands::ClearDisplayFeaturesOverride>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.clearGeolocationOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Clears the overridden Geolocation Position and Error.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::ClearGeolocationOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().clear_geolocation_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn clear_geolocation_override(
        &self,
        params: commands::ClearGeolocationOverrideParams,
    ) -> ydp_core::Request<commands::ClearGeolocationOverride> {
        self.session
            .send::<commands::ClearGeolocationOverride>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.clearIdleOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Clears Idle state overrides.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::ClearIdleOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().clear_idle_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn clear_idle_override(
        &self,
        params: commands::ClearIdleOverrideParams,
    ) -> ydp_core::Request<commands::ClearIdleOverride> {
        self.session.send::<commands::ClearIdleOverride>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.getOverriddenSensorInformation。此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::GetOverriddenSensorInformationParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().get_overridden_sensor_information(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_overridden_sensor_information(
        &self,
        params: commands::GetOverriddenSensorInformationParams,
    ) -> ydp_core::Request<commands::GetOverriddenSensorInformation> {
        self.session
            .send::<commands::GetOverriddenSensorInformation>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.getScreenInfos。
    ///
    ///协议原文：
    ///
    ///<pre>Returns device's screen configuration. In headful mode, the physical screens configuration is returned,
    ///whereas in headless mode, a virtual headless screen configuration is provided instead.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::GetScreenInfosParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().get_screen_infos(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_screen_infos(
        &self,
        params: commands::GetScreenInfosParams,
    ) -> ydp_core::Request<commands::GetScreenInfos> {
        self.session.send::<commands::GetScreenInfos>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.removeScreen。
    ///
    ///协议原文：
    ///
    ///<pre>Remove screen from the device. Only supported in headless mode.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::RemoveScreenParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().remove_screen(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn remove_screen(
        &self,
        params: commands::RemoveScreenParams,
    ) -> ydp_core::Request<commands::RemoveScreen> {
        self.session.send::<commands::RemoveScreen>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.resetPageScaleFactor。
    ///
    ///协议原文：
    ///
    ///<pre>Requests that page scale factor is reset to initial values.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::ResetPageScaleFactorParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().reset_page_scale_factor(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn reset_page_scale_factor(
        &self,
        params: commands::ResetPageScaleFactorParams,
    ) -> ydp_core::Request<commands::ResetPageScaleFactor> {
        self.session.send::<commands::ResetPageScaleFactor>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setAutoDarkModeOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Automatically render all web contents using a dark theme.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetAutoDarkModeOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_auto_dark_mode_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_auto_dark_mode_override(
        &self,
        params: commands::SetAutoDarkModeOverrideParams,
    ) -> ydp_core::Request<commands::SetAutoDarkModeOverride> {
        self.session
            .send::<commands::SetAutoDarkModeOverride>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setAutomationOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Allows overriding the automation flag.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetAutomationOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_automation_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_automation_override(
        &self,
        params: commands::SetAutomationOverrideParams,
    ) -> ydp_core::Request<commands::SetAutomationOverride> {
        self.session.send::<commands::SetAutomationOverride>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setCPUPerformanceOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Overrides the value of navigator.cpuPerformance</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetCpuPerformanceOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_cpu_performance_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_cpu_performance_override(
        &self,
        params: commands::SetCpuPerformanceOverrideParams,
    ) -> ydp_core::Request<commands::SetCpuPerformanceOverride> {
        self.session
            .send::<commands::SetCpuPerformanceOverride>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setCPUThrottlingRate。
    ///
    ///协议原文：
    ///
    ///<pre>Enables CPU throttling to emulate slow CPUs.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetCpuThrottlingRateParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_cpu_throttling_rate(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_cpu_throttling_rate(
        &self,
        params: commands::SetCpuThrottlingRateParams,
    ) -> ydp_core::Request<commands::SetCpuThrottlingRate> {
        self.session.send::<commands::SetCpuThrottlingRate>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setDataSaverOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Override the value of navigator.connection.saveData</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetDataSaverOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_data_saver_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_data_saver_override(
        &self,
        params: commands::SetDataSaverOverrideParams,
    ) -> ydp_core::Request<commands::SetDataSaverOverride> {
        self.session.send::<commands::SetDataSaverOverride>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setDefaultBackgroundColorOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Sets or clears an override of the default background color of the frame. This override is used
    ///if the content does not specify one.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetDefaultBackgroundColorOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_default_background_color_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_default_background_color_override(
        &self,
        params: commands::SetDefaultBackgroundColorOverrideParams,
    ) -> ydp_core::Request<commands::SetDefaultBackgroundColorOverride> {
        self.session
            .send::<commands::SetDefaultBackgroundColorOverride>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setDeviceMetricsOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Overrides the values of device screen dimensions (window.screen.width, window.screen.height,
    ///window.innerWidth, window.innerHeight, and "device-width"/"device-height"-related CSS media
    ///query results).</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetDeviceMetricsOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_device_metrics_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_device_metrics_override(
        &self,
        params: commands::SetDeviceMetricsOverrideParams,
    ) -> ydp_core::Request<commands::SetDeviceMetricsOverride> {
        self.session
            .send::<commands::SetDeviceMetricsOverride>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setDevicePostureOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Start reporting the given posture value to the Device Posture API.
    ///This override can also be set in setDeviceMetricsOverride().</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetDevicePostureOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_device_posture_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_device_posture_override(
        &self,
        params: commands::SetDevicePostureOverrideParams,
    ) -> ydp_core::Request<commands::SetDevicePostureOverride> {
        self.session
            .send::<commands::SetDevicePostureOverride>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setDisabledImageTypes。此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetDisabledImageTypesParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_disabled_image_types(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_disabled_image_types(
        &self,
        params: commands::SetDisabledImageTypesParams,
    ) -> ydp_core::Request<commands::SetDisabledImageTypes> {
        self.session.send::<commands::SetDisabledImageTypes>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setDisplayFeaturesOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Start using the given display features to pupulate the Viewport Segments API.
    ///This override can also be set in setDeviceMetricsOverride().</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetDisplayFeaturesOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_display_features_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_display_features_override(
        &self,
        params: commands::SetDisplayFeaturesOverrideParams,
    ) -> ydp_core::Request<commands::SetDisplayFeaturesOverride> {
        self.session
            .send::<commands::SetDisplayFeaturesOverride>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setDocumentCookieDisabled。此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetDocumentCookieDisabledParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_document_cookie_disabled(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_document_cookie_disabled(
        &self,
        params: commands::SetDocumentCookieDisabledParams,
    ) -> ydp_core::Request<commands::SetDocumentCookieDisabled> {
        self.session
            .send::<commands::SetDocumentCookieDisabled>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setEmitTouchEventsForMouse。此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetEmitTouchEventsForMouseParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_emit_touch_events_for_mouse(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_emit_touch_events_for_mouse(
        &self,
        params: commands::SetEmitTouchEventsForMouseParams,
    ) -> ydp_core::Request<commands::SetEmitTouchEventsForMouse> {
        self.session
            .send::<commands::SetEmitTouchEventsForMouse>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setEmulatedMedia。
    ///
    ///协议原文：
    ///
    ///<pre>Emulates the given media type or media feature for CSS media queries.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetEmulatedMediaParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_emulated_media(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_emulated_media(
        &self,
        params: commands::SetEmulatedMediaParams,
    ) -> ydp_core::Request<commands::SetEmulatedMedia> {
        self.session.send::<commands::SetEmulatedMedia>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setEmulatedOSTextScale。
    ///
    ///协议原文：
    ///
    ///<pre>Emulates the given OS text scale.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetEmulatedOsTextScaleParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_emulated_os_text_scale(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_emulated_os_text_scale(
        &self,
        params: commands::SetEmulatedOsTextScaleParams,
    ) -> ydp_core::Request<commands::SetEmulatedOsTextScale> {
        self.session
            .send::<commands::SetEmulatedOsTextScale>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setEmulatedVisionDeficiency。
    ///
    ///协议原文：
    ///
    ///<pre>Emulates the given vision deficiency.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetEmulatedVisionDeficiencyParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_emulated_vision_deficiency(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_emulated_vision_deficiency(
        &self,
        params: commands::SetEmulatedVisionDeficiencyParams,
    ) -> ydp_core::Request<commands::SetEmulatedVisionDeficiency> {
        self.session
            .send::<commands::SetEmulatedVisionDeficiency>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setFocusEmulationEnabled。
    ///
    ///协议原文：
    ///
    ///<pre>Enables or disables simulating a focused and active page.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetFocusEmulationEnabledParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_focus_emulation_enabled(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_focus_emulation_enabled(
        &self,
        params: commands::SetFocusEmulationEnabledParams,
    ) -> ydp_core::Request<commands::SetFocusEmulationEnabled> {
        self.session
            .send::<commands::SetFocusEmulationEnabled>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setGeolocationOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Overrides the Geolocation Position or Error. Omitting latitude, longitude or
    ///accuracy emulates position unavailable.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetGeolocationOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_geolocation_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_geolocation_override(
        &self,
        params: commands::SetGeolocationOverrideParams,
    ) -> ydp_core::Request<commands::SetGeolocationOverride> {
        self.session
            .send::<commands::SetGeolocationOverride>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setHardwareConcurrencyOverride。此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetHardwareConcurrencyOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_hardware_concurrency_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_hardware_concurrency_override(
        &self,
        params: commands::SetHardwareConcurrencyOverrideParams,
    ) -> ydp_core::Request<commands::SetHardwareConcurrencyOverride> {
        self.session
            .send::<commands::SetHardwareConcurrencyOverride>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setIdleOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Overrides the Idle state.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetIdleOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_idle_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_idle_override(
        &self,
        params: commands::SetIdleOverrideParams,
    ) -> ydp_core::Request<commands::SetIdleOverride> {
        self.session.send::<commands::SetIdleOverride>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setLocaleOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Overrides default host system locale with the specified one.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetLocaleOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_locale_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_locale_override(
        &self,
        params: commands::SetLocaleOverrideParams,
    ) -> ydp_core::Request<commands::SetLocaleOverride> {
        self.session.send::<commands::SetLocaleOverride>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setNavigatorOverrides。
    ///
    ///协议原文：
    ///
    ///<pre>Overrides value returned by the javascript navigator object.</pre>
    ///
    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetNavigatorOverridesParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_navigator_overrides(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_navigator_overrides(
        &self,
        params: commands::SetNavigatorOverridesParams,
    ) -> ydp_core::Request<commands::SetNavigatorOverrides> {
        self.session.send::<commands::SetNavigatorOverrides>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setPageScaleFactor。
    ///
    ///协议原文：
    ///
    ///<pre>Sets a specified page scale factor.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetPageScaleFactorParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_page_scale_factor(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_page_scale_factor(
        &self,
        params: commands::SetPageScaleFactorParams,
    ) -> ydp_core::Request<commands::SetPageScaleFactor> {
        self.session.send::<commands::SetPageScaleFactor>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setPressureSourceOverrideEnabled。
    ///
    ///协议原文：
    ///
    ///<pre>Overrides a pressure source of a given type, as used by the Compute
    ///Pressure API, so that updates to PressureObserver.observe() are provided
    ///via setPressureStateOverride instead of being retrieved from
    ///platform-provided telemetry data.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetPressureSourceOverrideEnabledParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_pressure_source_override_enabled(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_pressure_source_override_enabled(
        &self,
        params: commands::SetPressureSourceOverrideEnabledParams,
    ) -> ydp_core::Request<commands::SetPressureSourceOverrideEnabled> {
        self.session
            .send::<commands::SetPressureSourceOverrideEnabled>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setPressureStateOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Provides a given pressure state that will be processed and eventually be
    ///delivered to PressureObserver users. |source| must have been previously
    ///overridden by setPressureSourceOverrideEnabled.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetPressureStateOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_pressure_state_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_pressure_state_override(
        &self,
        params: commands::SetPressureStateOverrideParams,
    ) -> ydp_core::Request<commands::SetPressureStateOverride> {
        self.session
            .send::<commands::SetPressureStateOverride>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setPrimaryScreen。
    ///
    ///协议原文：
    ///
    ///<pre>Set primary screen. Only supported in headless mode.
    ///Note that this changes the coordinate system origin to the top-left
    ///of the new primary screen, updating the bounds and work areas
    ///of all existing screens accordingly.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetPrimaryScreenParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_primary_screen(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_primary_screen(
        &self,
        params: commands::SetPrimaryScreenParams,
    ) -> ydp_core::Request<commands::SetPrimaryScreen> {
        self.session.send::<commands::SetPrimaryScreen>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setSafeAreaInsetsOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Overrides the values for env(safe-area-inset-*) and env(safe-area-max-inset-*). Unset values will cause the
    ///respective variables to be undefined, even if previously overridden.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetSafeAreaInsetsOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_safe_area_insets_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_safe_area_insets_override(
        &self,
        params: commands::SetSafeAreaInsetsOverrideParams,
    ) -> ydp_core::Request<commands::SetSafeAreaInsetsOverride> {
        self.session
            .send::<commands::SetSafeAreaInsetsOverride>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setScriptExecutionDisabled。
    ///
    ///协议原文：
    ///
    ///<pre>Switches script execution in the page.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetScriptExecutionDisabledParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_script_execution_disabled(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_script_execution_disabled(
        &self,
        params: commands::SetScriptExecutionDisabledParams,
    ) -> ydp_core::Request<commands::SetScriptExecutionDisabled> {
        self.session
            .send::<commands::SetScriptExecutionDisabled>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setScrollbarsHidden。此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetScrollbarsHiddenParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_scrollbars_hidden(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_scrollbars_hidden(
        &self,
        params: commands::SetScrollbarsHiddenParams,
    ) -> ydp_core::Request<commands::SetScrollbarsHidden> {
        self.session.send::<commands::SetScrollbarsHidden>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setSensorOverrideEnabled。
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
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetSensorOverrideEnabledParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_sensor_override_enabled(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_sensor_override_enabled(
        &self,
        params: commands::SetSensorOverrideEnabledParams,
    ) -> ydp_core::Request<commands::SetSensorOverrideEnabled> {
        self.session
            .send::<commands::SetSensorOverrideEnabled>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setSensorOverrideReadings。
    ///
    ///协议原文：
    ///
    ///<pre>Updates the sensor readings reported by a sensor type previously overridden
    ///by setSensorOverrideEnabled.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetSensorOverrideReadingsParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_sensor_override_readings(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_sensor_override_readings(
        &self,
        params: commands::SetSensorOverrideReadingsParams,
    ) -> ydp_core::Request<commands::SetSensorOverrideReadings> {
        self.session
            .send::<commands::SetSensorOverrideReadings>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setSmallViewportHeightDifferenceOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Allows overriding the difference between the small and large viewport sizes, which determine the
    ///value of the `svh` and `lvh` unit, respectively. Only supported for top-level frames.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetSmallViewportHeightDifferenceOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_small_viewport_height_difference_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_small_viewport_height_difference_override(
        &self,
        params: commands::SetSmallViewportHeightDifferenceOverrideParams,
    ) -> ydp_core::Request<commands::SetSmallViewportHeightDifferenceOverride> {
        self.session
            .send::<commands::SetSmallViewportHeightDifferenceOverride>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setTimezoneOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Overrides default host system timezone with the specified one.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetTimezoneOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_timezone_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_timezone_override(
        &self,
        params: commands::SetTimezoneOverrideParams,
    ) -> ydp_core::Request<commands::SetTimezoneOverride> {
        self.session.send::<commands::SetTimezoneOverride>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setTouchEmulationEnabled。
    ///
    ///协议原文：
    ///
    ///<pre>Enables touch on platforms which do not support them.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetTouchEmulationEnabledParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_touch_emulation_enabled(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_touch_emulation_enabled(
        &self,
        params: commands::SetTouchEmulationEnabledParams,
    ) -> ydp_core::Request<commands::SetTouchEmulationEnabled> {
        self.session
            .send::<commands::SetTouchEmulationEnabled>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setUserAgentOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Allows overriding user agent with the given string.
    ///`userAgentMetadata` must be set for Client Hint headers to be sent.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetUserAgentOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_user_agent_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_user_agent_override(
        &self,
        params: commands::SetUserAgentOverrideParams,
    ) -> ydp_core::Request<commands::SetUserAgentOverride> {
        self.session.send::<commands::SetUserAgentOverride>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setVirtualKeyboardGeometryOverride。
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
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetVirtualKeyboardGeometryOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_virtual_keyboard_geometry_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_virtual_keyboard_geometry_override(
        &self,
        params: commands::SetVirtualKeyboardGeometryOverrideParams,
    ) -> ydp_core::Request<commands::SetVirtualKeyboardGeometryOverride> {
        self.session
            .send::<commands::SetVirtualKeyboardGeometryOverride>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setVirtualTimePolicy。
    ///
    ///协议原文：
    ///
    ///<pre>Turns on virtual time for all frames (replacing real-time with a synthetic time source) and sets
    ///the current virtual time policy.  Note this supersedes any previous time budget.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetVirtualTimePolicyParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_virtual_time_policy(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_virtual_time_policy(
        &self,
        params: commands::SetVirtualTimePolicyParams,
    ) -> ydp_core::Request<commands::SetVirtualTimePolicy> {
        self.session.send::<commands::SetVirtualTimePolicy>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.setVisibleSize。
    ///
    ///协议原文：
    ///
    ///<pre>Resizes the frame/viewport of the page. Note that this does not affect the frame's container
    ///(e.g. browser window). Can be used to produce screenshots of the specified size. Not supported
    ///on Android.</pre>
    ///
    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::SetVisibleSizeParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().set_visible_size(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_visible_size(
        &self,
        params: commands::SetVisibleSizeParams,
    ) -> ydp_core::Request<commands::SetVisibleSize> {
        self.session.send::<commands::SetVisibleSize>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Emulation.command.updateScreen。
    ///
    ///协议原文：
    ///
    ///<pre>Updates specified screen parameters. Only supported in headless mode.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::emulation::commands::UpdateScreenParams) -> ydp_core::Result<()> {
    ///let response = session.emulation().update_screen(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn update_screen(
        &self,
        params: commands::UpdateScreenParams,
    ) -> ydp_core::Request<commands::UpdateScreen> {
        self.session.send::<commands::UpdateScreen>(params)
    }
}
