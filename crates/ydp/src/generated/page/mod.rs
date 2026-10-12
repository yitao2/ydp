///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：Page。
///
///协议原文：
///
///<pre>Actions and events related to the inspected page belong to the page domain.</pre>
///
///
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.addCompilationCache。
    ///
    ///协议原文：
    ///
    ///<pre>Seeds compilation cache for given url. Compilation cache does not survive
    ///cross-process navigation.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::AddCompilationCacheParams) -> ydp_core::Result<()> {
    ///let response = session.page().add_compilation_cache(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn add_compilation_cache(
        &self,
        params: commands::AddCompilationCacheParams,
    ) -> ydp_core::Request<commands::AddCompilationCache> {
        self.session.send::<commands::AddCompilationCache>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.addScriptToEvaluateOnLoad。
    ///
    ///协议原文：
    ///
    ///<pre>Deprecated, please use addScriptToEvaluateOnNewDocument instead.</pre>
    ///
    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::AddScriptToEvaluateOnLoadParams) -> ydp_core::Result<()> {
    ///let response = session.page().add_script_to_evaluate_on_load(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn add_script_to_evaluate_on_load(
        &self,
        params: commands::AddScriptToEvaluateOnLoadParams,
    ) -> ydp_core::Request<commands::AddScriptToEvaluateOnLoad> {
        self.session
            .send::<commands::AddScriptToEvaluateOnLoad>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.addScriptToEvaluateOnNewDocument。
    ///
    ///协议原文：
    ///
    ///<pre>Evaluates given script in every frame upon creation (before loading frame's scripts).</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::AddScriptToEvaluateOnNewDocumentParams) -> ydp_core::Result<()> {
    ///let response = session.page().add_script_to_evaluate_on_new_document(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn add_script_to_evaluate_on_new_document(
        &self,
        params: commands::AddScriptToEvaluateOnNewDocumentParams,
    ) -> ydp_core::Request<commands::AddScriptToEvaluateOnNewDocument> {
        self.session
            .send::<commands::AddScriptToEvaluateOnNewDocument>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.bringToFront。
    ///
    ///协议原文：
    ///
    ///<pre>Brings page to front (activates tab).</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::BringToFrontParams) -> ydp_core::Result<()> {
    ///let response = session.page().bring_to_front(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn bring_to_front(
        &self,
        params: commands::BringToFrontParams,
    ) -> ydp_core::Request<commands::BringToFront> {
        self.session.send::<commands::BringToFront>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.captureScreenshot。
    ///
    ///协议原文：
    ///
    ///<pre>Capture page screenshot.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::CaptureScreenshotParams) -> ydp_core::Result<()> {
    ///let response = session.page().capture_screenshot(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn capture_screenshot(
        &self,
        params: commands::CaptureScreenshotParams,
    ) -> ydp_core::Request<commands::CaptureScreenshot> {
        self.session.send::<commands::CaptureScreenshot>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.captureSnapshot。
    ///
    ///协议原文：
    ///
    ///<pre>Returns a snapshot of the page as a string. For MHTML format, the serialization includes
    ///iframes, shadow DOM, external resources, and element-inline styles.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::CaptureSnapshotParams) -> ydp_core::Result<()> {
    ///let response = session.page().capture_snapshot(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn capture_snapshot(
        &self,
        params: commands::CaptureSnapshotParams,
    ) -> ydp_core::Request<commands::CaptureSnapshot> {
        self.session.send::<commands::CaptureSnapshot>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.clearCompilationCache。
    ///
    ///协议原文：
    ///
    ///<pre>Clears seeded compilation cache.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::ClearCompilationCacheParams) -> ydp_core::Result<()> {
    ///let response = session.page().clear_compilation_cache(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn clear_compilation_cache(
        &self,
        params: commands::ClearCompilationCacheParams,
    ) -> ydp_core::Request<commands::ClearCompilationCache> {
        self.session.send::<commands::ClearCompilationCache>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.clearDeviceMetricsOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Clears the overridden device metrics.</pre>
    ///
    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::ClearDeviceMetricsOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.page().clear_device_metrics_override(params).await?;
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
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.clearDeviceOrientationOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Clears the overridden Device Orientation.</pre>
    ///
    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::ClearDeviceOrientationOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.page().clear_device_orientation_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn clear_device_orientation_override(
        &self,
        params: commands::ClearDeviceOrientationOverrideParams,
    ) -> ydp_core::Request<commands::ClearDeviceOrientationOverride> {
        self.session
            .send::<commands::ClearDeviceOrientationOverride>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.clearGeolocationOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Clears the overridden Geolocation Position and Error.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::ClearGeolocationOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.page().clear_geolocation_override(params).await?;
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
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.close。
    ///
    ///协议原文：
    ///
    ///<pre>Tries to close page, running its beforeunload hooks, if any.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::CloseParams) -> ydp_core::Result<()> {
    ///let response = session.page().close(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn close(&self, params: commands::CloseParams) -> ydp_core::Request<commands::Close> {
        self.session.send::<commands::Close>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.crash。
    ///
    ///协议原文：
    ///
    ///<pre>Crashes renderer on the IO thread, generates minidumps.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::CrashParams) -> ydp_core::Result<()> {
    ///let response = session.page().crash(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn crash(&self, params: commands::CrashParams) -> ydp_core::Request<commands::Crash> {
        self.session.send::<commands::Crash>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.createIsolatedWorld。
    ///
    ///协议原文：
    ///
    ///<pre>Creates an isolated world for the given frame.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::CreateIsolatedWorldParams) -> ydp_core::Result<()> {
    ///let response = session.page().create_isolated_world(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn create_isolated_world(
        &self,
        params: commands::CreateIsolatedWorldParams,
    ) -> ydp_core::Request<commands::CreateIsolatedWorld> {
        self.session.send::<commands::CreateIsolatedWorld>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.deleteCookie。
    ///
    ///协议原文：
    ///
    ///<pre>Deletes browser cookie with given name, domain and path.</pre>
    ///
    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::DeleteCookieParams) -> ydp_core::Result<()> {
    ///let response = session.page().delete_cookie(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn delete_cookie(
        &self,
        params: commands::DeleteCookieParams,
    ) -> ydp_core::Request<commands::DeleteCookie> {
        self.session.send::<commands::DeleteCookie>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Disables page domain notifications.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.page().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.enable。
    ///
    ///协议原文：
    ///
    ///<pre>Enables page domain notifications.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.page().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.generateTestReport。
    ///
    ///协议原文：
    ///
    ///<pre>Generates a report for testing.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::GenerateTestReportParams) -> ydp_core::Result<()> {
    ///let response = session.page().generate_test_report(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn generate_test_report(
        &self,
        params: commands::GenerateTestReportParams,
    ) -> ydp_core::Request<commands::GenerateTestReport> {
        self.session.send::<commands::GenerateTestReport>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.getAdScriptAncestry。此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::GetAdScriptAncestryParams) -> ydp_core::Result<()> {
    ///let response = session.page().get_ad_script_ancestry(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_ad_script_ancestry(
        &self,
        params: commands::GetAdScriptAncestryParams,
    ) -> ydp_core::Request<commands::GetAdScriptAncestry> {
        self.session.send::<commands::GetAdScriptAncestry>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.getAnnotatedPageContent。
    ///
    ///协议原文：
    ///
    ///<pre>Get the annotated page content for the main frame.
    ///This is an experimental command that is subject to change.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::GetAnnotatedPageContentParams) -> ydp_core::Result<()> {
    ///let response = session.page().get_annotated_page_content(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_annotated_page_content(
        &self,
        params: commands::GetAnnotatedPageContentParams,
    ) -> ydp_core::Request<commands::GetAnnotatedPageContent> {
        self.session
            .send::<commands::GetAnnotatedPageContent>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.getAppId。
    ///
    ///协议原文：
    ///
    ///<pre>Returns the unique (PWA) app id.
    ///Only returns values if the feature flag 'WebAppEnableManifestId' is enabled</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::GetAppIdParams) -> ydp_core::Result<()> {
    ///let response = session.page().get_app_id(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_app_id(
        &self,
        params: commands::GetAppIdParams,
    ) -> ydp_core::Request<commands::GetAppId> {
        self.session.send::<commands::GetAppId>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.getAppManifest。
    ///
    ///协议原文：
    ///
    ///<pre>Gets the processed manifest for this current document.
    ///  This API always waits for the manifest to be loaded.
    ///  If manifestId is provided, and it does not match the manifest of the
    ///    current document, this API errors out.
    ///  If there is not a loaded page, this API errors out immediately.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::GetAppManifestParams) -> ydp_core::Result<()> {
    ///let response = session.page().get_app_manifest(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_app_manifest(
        &self,
        params: commands::GetAppManifestParams,
    ) -> ydp_core::Request<commands::GetAppManifest> {
        self.session.send::<commands::GetAppManifest>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.getFrameTree。
    ///
    ///协议原文：
    ///
    ///<pre>Returns present frame tree structure.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::GetFrameTreeParams) -> ydp_core::Result<()> {
    ///let response = session.page().get_frame_tree(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_frame_tree(
        &self,
        params: commands::GetFrameTreeParams,
    ) -> ydp_core::Request<commands::GetFrameTree> {
        self.session.send::<commands::GetFrameTree>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.getInstallabilityErrors。此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::GetInstallabilityErrorsParams) -> ydp_core::Result<()> {
    ///let response = session.page().get_installability_errors(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_installability_errors(
        &self,
        params: commands::GetInstallabilityErrorsParams,
    ) -> ydp_core::Request<commands::GetInstallabilityErrors> {
        self.session
            .send::<commands::GetInstallabilityErrors>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.getLayoutMetrics。
    ///
    ///协议原文：
    ///
    ///<pre>Returns metrics relating to the layouting of the page, such as viewport bounds/scale.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::GetLayoutMetricsParams) -> ydp_core::Result<()> {
    ///let response = session.page().get_layout_metrics(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_layout_metrics(
        &self,
        params: commands::GetLayoutMetricsParams,
    ) -> ydp_core::Request<commands::GetLayoutMetrics> {
        self.session.send::<commands::GetLayoutMetrics>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.getManifestIcons。
    ///
    ///协议原文：
    ///
    ///<pre>Deprecated because it's not guaranteed that the returned icon is in fact the one used for PWA installation.</pre>
    ///
    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::GetManifestIconsParams) -> ydp_core::Result<()> {
    ///let response = session.page().get_manifest_icons(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_manifest_icons(
        &self,
        params: commands::GetManifestIconsParams,
    ) -> ydp_core::Request<commands::GetManifestIcons> {
        self.session.send::<commands::GetManifestIcons>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.getNavigationHistory。
    ///
    ///协议原文：
    ///
    ///<pre>Returns navigation history for the current page.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::GetNavigationHistoryParams) -> ydp_core::Result<()> {
    ///let response = session.page().get_navigation_history(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_navigation_history(
        &self,
        params: commands::GetNavigationHistoryParams,
    ) -> ydp_core::Request<commands::GetNavigationHistory> {
        self.session.send::<commands::GetNavigationHistory>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.getOriginTrials。
    ///
    ///协议原文：
    ///
    ///<pre>Get Origin Trials on given frame.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::GetOriginTrialsParams) -> ydp_core::Result<()> {
    ///let response = session.page().get_origin_trials(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_origin_trials(
        &self,
        params: commands::GetOriginTrialsParams,
    ) -> ydp_core::Request<commands::GetOriginTrials> {
        self.session.send::<commands::GetOriginTrials>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.getPermissionsPolicyState。
    ///
    ///协议原文：
    ///
    ///<pre>Get Permissions Policy state on given frame.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::GetPermissionsPolicyStateParams) -> ydp_core::Result<()> {
    ///let response = session.page().get_permissions_policy_state(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_permissions_policy_state(
        &self,
        params: commands::GetPermissionsPolicyStateParams,
    ) -> ydp_core::Request<commands::GetPermissionsPolicyState> {
        self.session
            .send::<commands::GetPermissionsPolicyState>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.getResourceContent。
    ///
    ///协议原文：
    ///
    ///<pre>Returns content of the given resource.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::GetResourceContentParams) -> ydp_core::Result<()> {
    ///let response = session.page().get_resource_content(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_resource_content(
        &self,
        params: commands::GetResourceContentParams,
    ) -> ydp_core::Request<commands::GetResourceContent> {
        self.session.send::<commands::GetResourceContent>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.getResourceTree。
    ///
    ///协议原文：
    ///
    ///<pre>Returns present frame / resource tree structure.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::GetResourceTreeParams) -> ydp_core::Result<()> {
    ///let response = session.page().get_resource_tree(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_resource_tree(
        &self,
        params: commands::GetResourceTreeParams,
    ) -> ydp_core::Request<commands::GetResourceTree> {
        self.session.send::<commands::GetResourceTree>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.handleJavaScriptDialog。
    ///
    ///协议原文：
    ///
    ///<pre>Accepts or dismisses a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload).</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::HandleJavaScriptDialogParams) -> ydp_core::Result<()> {
    ///let response = session.page().handle_java_script_dialog(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn handle_java_script_dialog(
        &self,
        params: commands::HandleJavaScriptDialogParams,
    ) -> ydp_core::Request<commands::HandleJavaScriptDialog> {
        self.session
            .send::<commands::HandleJavaScriptDialog>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.navigate。
    ///
    ///协议原文：
    ///
    ///<pre>Navigates current page to the given URL.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::NavigateParams) -> ydp_core::Result<()> {
    ///let response = session.page().navigate(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn navigate(
        &self,
        params: commands::NavigateParams,
    ) -> ydp_core::Request<commands::Navigate> {
        self.session.send::<commands::Navigate>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.navigateToHistoryEntry。
    ///
    ///协议原文：
    ///
    ///<pre>Navigates current page to the given history entry.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::NavigateToHistoryEntryParams) -> ydp_core::Result<()> {
    ///let response = session.page().navigate_to_history_entry(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn navigate_to_history_entry(
        &self,
        params: commands::NavigateToHistoryEntryParams,
    ) -> ydp_core::Request<commands::NavigateToHistoryEntry> {
        self.session
            .send::<commands::NavigateToHistoryEntry>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.printToPDF。
    ///
    ///协议原文：
    ///
    ///<pre>Print page as PDF.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::PrintToPdfParams) -> ydp_core::Result<()> {
    ///let response = session.page().print_to_pdf(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn print_to_pdf(
        &self,
        params: commands::PrintToPdfParams,
    ) -> ydp_core::Request<commands::PrintToPdf> {
        self.session.send::<commands::PrintToPdf>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.produceCompilationCache。
    ///
    ///协议原文：
    ///
    ///<pre>Requests backend to produce compilation cache for the specified scripts.
    ///`scripts` are appended to the list of scripts for which the cache
    ///would be produced. The list may be reset during page navigation.
    ///When script with a matching URL is encountered, the cache is optionally
    ///produced upon backend discretion, based on internal heuristics.
    ///See also: `Page.compilationCacheProduced`.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::ProduceCompilationCacheParams) -> ydp_core::Result<()> {
    ///let response = session.page().produce_compilation_cache(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn produce_compilation_cache(
        &self,
        params: commands::ProduceCompilationCacheParams,
    ) -> ydp_core::Request<commands::ProduceCompilationCache> {
        self.session
            .send::<commands::ProduceCompilationCache>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.reload。
    ///
    ///协议原文：
    ///
    ///<pre>Reloads given page optionally ignoring the cache.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::ReloadParams) -> ydp_core::Result<()> {
    ///let response = session.page().reload(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn reload(&self, params: commands::ReloadParams) -> ydp_core::Request<commands::Reload> {
        self.session.send::<commands::Reload>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.removeScriptToEvaluateOnLoad。
    ///
    ///协议原文：
    ///
    ///<pre>Deprecated, please use removeScriptToEvaluateOnNewDocument instead.</pre>
    ///
    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::RemoveScriptToEvaluateOnLoadParams) -> ydp_core::Result<()> {
    ///let response = session.page().remove_script_to_evaluate_on_load(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn remove_script_to_evaluate_on_load(
        &self,
        params: commands::RemoveScriptToEvaluateOnLoadParams,
    ) -> ydp_core::Request<commands::RemoveScriptToEvaluateOnLoad> {
        self.session
            .send::<commands::RemoveScriptToEvaluateOnLoad>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.removeScriptToEvaluateOnNewDocument。
    ///
    ///协议原文：
    ///
    ///<pre>Removes given script from the list.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::RemoveScriptToEvaluateOnNewDocumentParams) -> ydp_core::Result<()> {
    ///let response = session.page().remove_script_to_evaluate_on_new_document(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn remove_script_to_evaluate_on_new_document(
        &self,
        params: commands::RemoveScriptToEvaluateOnNewDocumentParams,
    ) -> ydp_core::Request<commands::RemoveScriptToEvaluateOnNewDocument> {
        self.session
            .send::<commands::RemoveScriptToEvaluateOnNewDocument>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.resetNavigationHistory。
    ///
    ///协议原文：
    ///
    ///<pre>Resets navigation history for the current page.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::ResetNavigationHistoryParams) -> ydp_core::Result<()> {
    ///let response = session.page().reset_navigation_history(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn reset_navigation_history(
        &self,
        params: commands::ResetNavigationHistoryParams,
    ) -> ydp_core::Request<commands::ResetNavigationHistory> {
        self.session
            .send::<commands::ResetNavigationHistory>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.screencastFrameAck。
    ///
    ///协议原文：
    ///
    ///<pre>Acknowledges that a screencast frame has been received by the frontend.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::ScreencastFrameAckParams) -> ydp_core::Result<()> {
    ///let response = session.page().screencast_frame_ack(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn screencast_frame_ack(
        &self,
        params: commands::ScreencastFrameAckParams,
    ) -> ydp_core::Request<commands::ScreencastFrameAck> {
        self.session.send::<commands::ScreencastFrameAck>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.searchInResource。
    ///
    ///协议原文：
    ///
    ///<pre>Searches for given string in resource content.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::SearchInResourceParams) -> ydp_core::Result<()> {
    ///let response = session.page().search_in_resource(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn search_in_resource(
        &self,
        params: commands::SearchInResourceParams,
    ) -> ydp_core::Request<commands::SearchInResource> {
        self.session.send::<commands::SearchInResource>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.setAdBlockingEnabled。
    ///
    ///协议原文：
    ///
    ///<pre>Enable Chrome's experimental ad filter on all sites.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::SetAdBlockingEnabledParams) -> ydp_core::Result<()> {
    ///let response = session.page().set_ad_blocking_enabled(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_ad_blocking_enabled(
        &self,
        params: commands::SetAdBlockingEnabledParams,
    ) -> ydp_core::Request<commands::SetAdBlockingEnabled> {
        self.session.send::<commands::SetAdBlockingEnabled>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.setBypassCSP。
    ///
    ///协议原文：
    ///
    ///<pre>Enable page Content Security Policy by-passing.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::SetBypassCspParams) -> ydp_core::Result<()> {
    ///let response = session.page().set_bypass_csp(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_bypass_csp(
        &self,
        params: commands::SetBypassCspParams,
    ) -> ydp_core::Request<commands::SetBypassCsp> {
        self.session.send::<commands::SetBypassCsp>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.setDeviceMetricsOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Overrides the values of device screen dimensions (window.screen.width, window.screen.height,
    ///window.innerWidth, window.innerHeight, and "device-width"/"device-height"-related CSS media
    ///query results).</pre>
    ///
    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::SetDeviceMetricsOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.page().set_device_metrics_override(params).await?;
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
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.setDeviceOrientationOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Overrides the Device Orientation.</pre>
    ///
    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::SetDeviceOrientationOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.page().set_device_orientation_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_device_orientation_override(
        &self,
        params: commands::SetDeviceOrientationOverrideParams,
    ) -> ydp_core::Request<commands::SetDeviceOrientationOverride> {
        self.session
            .send::<commands::SetDeviceOrientationOverride>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.setDocumentContent。
    ///
    ///协议原文：
    ///
    ///<pre>Sets given markup as the document's HTML.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::SetDocumentContentParams) -> ydp_core::Result<()> {
    ///let response = session.page().set_document_content(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_document_content(
        &self,
        params: commands::SetDocumentContentParams,
    ) -> ydp_core::Request<commands::SetDocumentContent> {
        self.session.send::<commands::SetDocumentContent>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.setDownloadBehavior。
    ///
    ///协议原文：
    ///
    ///<pre>Set the behavior when downloading a file.</pre>
    ///
    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::SetDownloadBehaviorParams) -> ydp_core::Result<()> {
    ///let response = session.page().set_download_behavior(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_download_behavior(
        &self,
        params: commands::SetDownloadBehaviorParams,
    ) -> ydp_core::Request<commands::SetDownloadBehavior> {
        self.session.send::<commands::SetDownloadBehavior>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.setFontFamilies。
    ///
    ///协议原文：
    ///
    ///<pre>Set generic font families.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::SetFontFamiliesParams) -> ydp_core::Result<()> {
    ///let response = session.page().set_font_families(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_font_families(
        &self,
        params: commands::SetFontFamiliesParams,
    ) -> ydp_core::Request<commands::SetFontFamilies> {
        self.session.send::<commands::SetFontFamilies>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.setFontSizes。
    ///
    ///协议原文：
    ///
    ///<pre>Set default font sizes.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::SetFontSizesParams) -> ydp_core::Result<()> {
    ///let response = session.page().set_font_sizes(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_font_sizes(
        &self,
        params: commands::SetFontSizesParams,
    ) -> ydp_core::Request<commands::SetFontSizes> {
        self.session.send::<commands::SetFontSizes>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.setGeolocationOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Overrides the Geolocation Position or Error. Omitting any of the parameters emulates position
    ///unavailable.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::SetGeolocationOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.page().set_geolocation_override(params).await?;
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
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.setInterceptFileChooserDialog。
    ///
    ///协议原文：
    ///
    ///<pre>Intercept file chooser requests and transfer control to protocol clients.
    ///When file chooser interception is enabled, native file chooser dialog is not shown.
    ///Instead, a protocol event `Page.fileChooserOpened` is emitted.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::SetInterceptFileChooserDialogParams) -> ydp_core::Result<()> {
    ///let response = session.page().set_intercept_file_chooser_dialog(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_intercept_file_chooser_dialog(
        &self,
        params: commands::SetInterceptFileChooserDialogParams,
    ) -> ydp_core::Request<commands::SetInterceptFileChooserDialog> {
        self.session
            .send::<commands::SetInterceptFileChooserDialog>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.setLifecycleEventsEnabled。
    ///
    ///协议原文：
    ///
    ///<pre>Controls whether page will emit lifecycle events.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::SetLifecycleEventsEnabledParams) -> ydp_core::Result<()> {
    ///let response = session.page().set_lifecycle_events_enabled(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_lifecycle_events_enabled(
        &self,
        params: commands::SetLifecycleEventsEnabledParams,
    ) -> ydp_core::Request<commands::SetLifecycleEventsEnabled> {
        self.session
            .send::<commands::SetLifecycleEventsEnabled>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.setPrerenderingAllowed。
    ///
    ///协议原文：
    ///
    ///<pre>Enable/disable prerendering manually.
    ///
    ///This command is a short-term solution for https://crbug.com/1440085.
    ///See https://docs.google.com/document/d/12HVmFxYj5Jc-eJr5OmWsa2bqTJsbgGLKI6ZIyx0_wpA
    ///for more details.
    ///
    ///TODO(https://crbug.com/1440085): Remove this once Puppeteer supports tab targets.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::SetPrerenderingAllowedParams) -> ydp_core::Result<()> {
    ///let response = session.page().set_prerendering_allowed(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_prerendering_allowed(
        &self,
        params: commands::SetPrerenderingAllowedParams,
    ) -> ydp_core::Request<commands::SetPrerenderingAllowed> {
        self.session
            .send::<commands::SetPrerenderingAllowed>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.setRPHRegistrationMode。
    ///
    ///协议原文：
    ///
    ///<pre>Extensions for Custom Handlers API:
    ///https://html.spec.whatwg.org/multipage/system-state.html#rph-automation</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::SetRphRegistrationModeParams) -> ydp_core::Result<()> {
    ///let response = session.page().set_rph_registration_mode(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_rph_registration_mode(
        &self,
        params: commands::SetRphRegistrationModeParams,
    ) -> ydp_core::Request<commands::SetRphRegistrationMode> {
        self.session
            .send::<commands::SetRphRegistrationMode>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.setSPCTransactionMode。
    ///
    ///协议原文：
    ///
    ///<pre>Sets the Secure Payment Confirmation transaction mode.
    ///https://w3c.github.io/secure-payment-confirmation/#sctn-automation-set-spc-transaction-mode</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::SetSpcTransactionModeParams) -> ydp_core::Result<()> {
    ///let response = session.page().set_spc_transaction_mode(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_spc_transaction_mode(
        &self,
        params: commands::SetSpcTransactionModeParams,
    ) -> ydp_core::Request<commands::SetSpcTransactionMode> {
        self.session.send::<commands::SetSpcTransactionMode>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.setTouchEmulationEnabled。
    ///
    ///协议原文：
    ///
    ///<pre>Toggles mouse event-based touch event emulation.</pre>
    ///
    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::SetTouchEmulationEnabledParams) -> ydp_core::Result<()> {
    ///let response = session.page().set_touch_emulation_enabled(params).await?;
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
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.setWebLifecycleState。
    ///
    ///协议原文：
    ///
    ///<pre>Tries to update the web lifecycle state of the page.
    ///It will transition the page to the given state according to:
    ///https://github.com/WICG/web-lifecycle/</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::SetWebLifecycleStateParams) -> ydp_core::Result<()> {
    ///let response = session.page().set_web_lifecycle_state(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_web_lifecycle_state(
        &self,
        params: commands::SetWebLifecycleStateParams,
    ) -> ydp_core::Request<commands::SetWebLifecycleState> {
        self.session.send::<commands::SetWebLifecycleState>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.startScreenRecording。
    ///
    ///协议原文：
    ///
    ///<pre>Starts screencast video recording.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::StartScreenRecordingParams) -> ydp_core::Result<()> {
    ///let response = session.page().start_screen_recording(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn start_screen_recording(
        &self,
        params: commands::StartScreenRecordingParams,
    ) -> ydp_core::Request<commands::StartScreenRecording> {
        self.session.send::<commands::StartScreenRecording>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.startScreencast。
    ///
    ///协议原文：
    ///
    ///<pre>Starts sending each frame using the `screencastFrame` event.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::StartScreencastParams) -> ydp_core::Result<()> {
    ///let response = session.page().start_screencast(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn start_screencast(
        &self,
        params: commands::StartScreencastParams,
    ) -> ydp_core::Request<commands::StartScreencast> {
        self.session.send::<commands::StartScreencast>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.stopLoading。
    ///
    ///协议原文：
    ///
    ///<pre>Force the page stop all navigations and pending resource fetches.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::StopLoadingParams) -> ydp_core::Result<()> {
    ///let response = session.page().stop_loading(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn stop_loading(
        &self,
        params: commands::StopLoadingParams,
    ) -> ydp_core::Request<commands::StopLoading> {
        self.session.send::<commands::StopLoading>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.stopScreenRecording。
    ///
    ///协议原文：
    ///
    ///<pre>Stops screencast video recording.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::StopScreenRecordingParams) -> ydp_core::Result<()> {
    ///let response = session.page().stop_screen_recording(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn stop_screen_recording(
        &self,
        params: commands::StopScreenRecordingParams,
    ) -> ydp_core::Request<commands::StopScreenRecording> {
        self.session.send::<commands::StopScreenRecording>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.stopScreencast。
    ///
    ///协议原文：
    ///
    ///<pre>Stops sending each frame in the `screencastFrame`.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::StopScreencastParams) -> ydp_core::Result<()> {
    ///let response = session.page().stop_screencast(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn stop_screencast(
        &self,
        params: commands::StopScreencastParams,
    ) -> ydp_core::Request<commands::StopScreencast> {
        self.session.send::<commands::StopScreencast>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Page.command.waitForDebugger。
    ///
    ///协议原文：
    ///
    ///<pre>Pauses page execution. Can be resumed using generic Runtime.runIfWaitingForDebugger.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::page::commands::WaitForDebuggerParams) -> ydp_core::Result<()> {
    ///let response = session.page().wait_for_debugger(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn wait_for_debugger(
        &self,
        params: commands::WaitForDebuggerParams,
    ) -> ydp_core::Request<commands::WaitForDebugger> {
        self.session.send::<commands::WaitForDebugger>(params)
    }
}
