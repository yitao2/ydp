///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：Browser。
///
///协议原文：
///
///<pre>The Browser domain defines methods and events for browser managing.</pre>
///
///
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Browser.command.addMockCamera。
    ///
    ///协议原文：
    ///
    ///<pre>Adds or updates a mock camera in the shared video capture device list for
    ///test automation. The mock camera is not scoped to a particular page or
    ///frame and is removed when the DevTools session that created it disconnects.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::browser::commands::AddMockCameraParams) -> ydp_core::Result<()> {
    ///let response = session.browser().add_mock_camera(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn add_mock_camera(
        &self,
        params: commands::AddMockCameraParams,
    ) -> ydp_core::Request<commands::AddMockCamera> {
        self.session.send::<commands::AddMockCamera>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Browser.command.addPrivacySandboxEnrollmentOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Allows a site to use privacy sandbox features that require enrollment
    ///without the site actually being enrolled. Only supported on page targets.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::browser::commands::AddPrivacySandboxEnrollmentOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.browser().add_privacy_sandbox_enrollment_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn add_privacy_sandbox_enrollment_override(
        &self,
        params: commands::AddPrivacySandboxEnrollmentOverrideParams,
    ) -> ydp_core::Request<commands::AddPrivacySandboxEnrollmentOverride> {
        self.session
            .send::<commands::AddPrivacySandboxEnrollmentOverride>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Browser.command.cancelDownload。
    ///
    ///协议原文：
    ///
    ///<pre>Cancel a download if in progress</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::browser::commands::CancelDownloadParams) -> ydp_core::Result<()> {
    ///let response = session.browser().cancel_download(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn cancel_download(
        &self,
        params: commands::CancelDownloadParams,
    ) -> ydp_core::Request<commands::CancelDownload> {
        self.session.send::<commands::CancelDownload>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Browser.command.close。
    ///
    ///协议原文：
    ///
    ///<pre>Close browser gracefully.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::browser::commands::CloseParams) -> ydp_core::Result<()> {
    ///let response = session.browser().close(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn close(&self, params: commands::CloseParams) -> ydp_core::Request<commands::Close> {
        self.session.send::<commands::Close>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Browser.command.crash。
    ///
    ///协议原文：
    ///
    ///<pre>Crashes browser on the main thread.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::browser::commands::CrashParams) -> ydp_core::Result<()> {
    ///let response = session.browser().crash(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn crash(&self, params: commands::CrashParams) -> ydp_core::Request<commands::Crash> {
        self.session.send::<commands::Crash>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Browser.command.crashGpuProcess。
    ///
    ///协议原文：
    ///
    ///<pre>Crashes GPU process.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::browser::commands::CrashGpuProcessParams) -> ydp_core::Result<()> {
    ///let response = session.browser().crash_gpu_process(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn crash_gpu_process(
        &self,
        params: commands::CrashGpuProcessParams,
    ) -> ydp_core::Request<commands::CrashGpuProcess> {
        self.session.send::<commands::CrashGpuProcess>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Browser.command.executeBrowserCommand。
    ///
    ///协议原文：
    ///
    ///<pre>Invoke custom browser commands used by telemetry.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::browser::commands::ExecuteBrowserCommandParams) -> ydp_core::Result<()> {
    ///let response = session.browser().execute_browser_command(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn execute_browser_command(
        &self,
        params: commands::ExecuteBrowserCommandParams,
    ) -> ydp_core::Request<commands::ExecuteBrowserCommand> {
        self.session.send::<commands::ExecuteBrowserCommand>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Browser.command.getBrowserCommandLine。
    ///
    ///协议原文：
    ///
    ///<pre>Returns the command line switches for the browser process if, and only if
    ///--enable-automation is on the commandline.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::browser::commands::GetBrowserCommandLineParams) -> ydp_core::Result<()> {
    ///let response = session.browser().get_browser_command_line(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_browser_command_line(
        &self,
        params: commands::GetBrowserCommandLineParams,
    ) -> ydp_core::Request<commands::GetBrowserCommandLine> {
        self.session.send::<commands::GetBrowserCommandLine>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Browser.command.getGlobalPrivacyControl。
    ///
    ///协议原文：
    ///
    ///<pre>Gets the current globally-applied privacy control status
    ///See https://www.w3.org/TR/gpc/#get-global-privacy-control</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::browser::commands::GetGlobalPrivacyControlParams) -> ydp_core::Result<()> {
    ///let response = session.browser().get_global_privacy_control(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_global_privacy_control(
        &self,
        params: commands::GetGlobalPrivacyControlParams,
    ) -> ydp_core::Request<commands::GetGlobalPrivacyControl> {
        self.session
            .send::<commands::GetGlobalPrivacyControl>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Browser.command.getHistogram。
    ///
    ///协议原文：
    ///
    ///<pre>Get a Chrome histogram by name.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::browser::commands::GetHistogramParams) -> ydp_core::Result<()> {
    ///let response = session.browser().get_histogram(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_histogram(
        &self,
        params: commands::GetHistogramParams,
    ) -> ydp_core::Request<commands::GetHistogram> {
        self.session.send::<commands::GetHistogram>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Browser.command.getHistograms。
    ///
    ///协议原文：
    ///
    ///<pre>Get Chrome histograms.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::browser::commands::GetHistogramsParams) -> ydp_core::Result<()> {
    ///let response = session.browser().get_histograms(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_histograms(
        &self,
        params: commands::GetHistogramsParams,
    ) -> ydp_core::Request<commands::GetHistograms> {
        self.session.send::<commands::GetHistograms>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Browser.command.getVersion。
    ///
    ///协议原文：
    ///
    ///<pre>Returns version information.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::browser::commands::GetVersionParams) -> ydp_core::Result<()> {
    ///let response = session.browser().get_version(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_version(
        &self,
        params: commands::GetVersionParams,
    ) -> ydp_core::Request<commands::GetVersion> {
        self.session.send::<commands::GetVersion>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Browser.command.getWindowBounds。
    ///
    ///协议原文：
    ///
    ///<pre>Get position and size of the browser window.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::browser::commands::GetWindowBoundsParams) -> ydp_core::Result<()> {
    ///let response = session.browser().get_window_bounds(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_window_bounds(
        &self,
        params: commands::GetWindowBoundsParams,
    ) -> ydp_core::Request<commands::GetWindowBounds> {
        self.session.send::<commands::GetWindowBounds>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Browser.command.getWindowForTarget。
    ///
    ///协议原文：
    ///
    ///<pre>Get the browser window that contains the devtools target.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::browser::commands::GetWindowForTargetParams) -> ydp_core::Result<()> {
    ///let response = session.browser().get_window_for_target(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_window_for_target(
        &self,
        params: commands::GetWindowForTargetParams,
    ) -> ydp_core::Request<commands::GetWindowForTarget> {
        self.session.send::<commands::GetWindowForTarget>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Browser.command.grantPermissions。
    ///
    ///协议原文：
    ///
    ///<pre>Grant specific permissions to the given origin and reject all others. Deprecated. Use
    ///setPermission instead.</pre>
    ///
    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::browser::commands::GrantPermissionsParams) -> ydp_core::Result<()> {
    ///let response = session.browser().grant_permissions(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn grant_permissions(
        &self,
        params: commands::GrantPermissionsParams,
    ) -> ydp_core::Request<commands::GrantPermissions> {
        self.session.send::<commands::GrantPermissions>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Browser.command.resetPermissions。
    ///
    ///协议原文：
    ///
    ///<pre>Reset all permission management for all origins.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::browser::commands::ResetPermissionsParams) -> ydp_core::Result<()> {
    ///let response = session.browser().reset_permissions(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn reset_permissions(
        &self,
        params: commands::ResetPermissionsParams,
    ) -> ydp_core::Request<commands::ResetPermissions> {
        self.session.send::<commands::ResetPermissions>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Browser.command.setContentsSize。
    ///
    ///协议原文：
    ///
    ///<pre>Set size of the browser contents resizing browser window as necessary.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::browser::commands::SetContentsSizeParams) -> ydp_core::Result<()> {
    ///let response = session.browser().set_contents_size(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_contents_size(
        &self,
        params: commands::SetContentsSizeParams,
    ) -> ydp_core::Request<commands::SetContentsSize> {
        self.session.send::<commands::SetContentsSize>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Browser.command.setDockTile。
    ///
    ///协议原文：
    ///
    ///<pre>Set dock tile details, platform-specific.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::browser::commands::SetDockTileParams) -> ydp_core::Result<()> {
    ///let response = session.browser().set_dock_tile(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_dock_tile(
        &self,
        params: commands::SetDockTileParams,
    ) -> ydp_core::Request<commands::SetDockTile> {
        self.session.send::<commands::SetDockTile>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Browser.command.setDownloadBehavior。
    ///
    ///协议原文：
    ///
    ///<pre>Set the behavior when downloading a file.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::browser::commands::SetDownloadBehaviorParams) -> ydp_core::Result<()> {
    ///let response = session.browser().set_download_behavior(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_download_behavior(
        &self,
        params: commands::SetDownloadBehaviorParams,
    ) -> ydp_core::Request<commands::SetDownloadBehavior> {
        self.session.send::<commands::SetDownloadBehavior>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Browser.command.setGlobalPrivacyControl。
    ///
    ///协议原文：
    ///
    ///<pre>Sets and then gets the current globally-applied privacy control status
    ///See https://www.w3.org/TR/gpc/#set-global-privacy-control</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::browser::commands::SetGlobalPrivacyControlParams) -> ydp_core::Result<()> {
    ///let response = session.browser().set_global_privacy_control(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_global_privacy_control(
        &self,
        params: commands::SetGlobalPrivacyControlParams,
    ) -> ydp_core::Request<commands::SetGlobalPrivacyControl> {
        self.session
            .send::<commands::SetGlobalPrivacyControl>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Browser.command.setPermission。
    ///
    ///协议原文：
    ///
    ///<pre>Set permission settings for given embedding and embedded origins.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::browser::commands::SetPermissionParams) -> ydp_core::Result<()> {
    ///let response = session.browser().set_permission(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_permission(
        &self,
        params: commands::SetPermissionParams,
    ) -> ydp_core::Request<commands::SetPermission> {
        self.session.send::<commands::SetPermission>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Browser.command.setWindowBounds。
    ///
    ///协议原文：
    ///
    ///<pre>Set position and/or size of the browser window.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::browser::commands::SetWindowBoundsParams) -> ydp_core::Result<()> {
    ///let response = session.browser().set_window_bounds(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_window_bounds(
        &self,
        params: commands::SetWindowBoundsParams,
    ) -> ydp_core::Request<commands::SetWindowBounds> {
        self.session.send::<commands::SetWindowBounds>(params)
    }
}
