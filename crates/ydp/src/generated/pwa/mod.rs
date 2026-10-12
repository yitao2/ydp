///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：PWA。
///
///协议原文：
///
///<pre>This domain allows interacting with the browser to control PWAs.</pre>
///
///此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：PWA.command.changeAppUserSettings。
    ///
    ///协议原文：
    ///
    ///<pre>Changes user settings of the web app identified by its manifestId. If the
    ///app was not installed, this command returns an error. Unset parameters will
    ///be ignored; unrecognized values will cause an error.
    ///
    ///Unlike the ones defined in the manifest files of the web apps, these
    ///settings are provided by the browser and controlled by the users, they
    ///impact the way the browser handling the web apps.
    ///
    ///See the comment of each parameter.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::pwa::commands::ChangeAppUserSettingsParams) -> ydp_core::Result<()> {
    ///let response = session.pwa().change_app_user_settings(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn change_app_user_settings(
        &self,
        params: commands::ChangeAppUserSettingsParams,
    ) -> ydp_core::Request<commands::ChangeAppUserSettings> {
        self.session.send::<commands::ChangeAppUserSettings>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：PWA.command.getOsAppState。
    ///
    ///协议原文：
    ///
    ///<pre>Returns the following OS state for the given manifest id.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::pwa::commands::GetOsAppStateParams) -> ydp_core::Result<()> {
    ///let response = session.pwa().get_os_app_state(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_os_app_state(
        &self,
        params: commands::GetOsAppStateParams,
    ) -> ydp_core::Request<commands::GetOsAppState> {
        self.session.send::<commands::GetOsAppState>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：PWA.command.install。
    ///
    ///协议原文：
    ///
    ///<pre>Installs the given manifest identity, optionally using the given installUrlOrBundleUrl
    ///
    ///IWA-specific install description:
    ///manifestId corresponds to isolated-app:// + web_package::SignedWebBundleId
    ///
    ///File installation mode:
    ///The installUrlOrBundleUrl can be either file:// or http(s):// pointing
    ///to a signed web bundle (.swbn). In this case SignedWebBundleId must correspond to
    ///The .swbn file's signing key.
    ///
    ///Dev proxy installation mode:
    ///installUrlOrBundleUrl must be http(s):// that serves dev mode IWA.
    ///web_package::SignedWebBundleId must be of type dev proxy.
    ///
    ///The advantage of dev proxy mode is that all changes to IWA
    ///automatically will be reflected in the running app without
    ///reinstallation.
    ///
    ///To generate bundle id for proxy mode:
    ///1. Generate 32 random bytes.
    ///2. Add a specific suffix at the end following the documentation
    ///   https://github.com/WICG/isolated-web-apps/blob/main/Scheme.md#suffix
    ///3. Encode the entire sequence using Base32 without padding.
    ///
    ///If Chrome is not in IWA dev
    ///mode, the installation will fail, regardless of the state of the allowlist.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::pwa::commands::InstallParams) -> ydp_core::Result<()> {
    ///let response = session.pwa().install(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn install(&self, params: commands::InstallParams) -> ydp_core::Request<commands::Install> {
        self.session.send::<commands::Install>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：PWA.command.launch。
    ///
    ///协议原文：
    ///
    ///<pre>Launches the installed web app, or an url in the same web app instead of the
    ///default start url if it is provided. Returns a page Target.TargetID which
    ///can be used to attach to via Target.attachToTarget or similar APIs.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::pwa::commands::LaunchParams) -> ydp_core::Result<()> {
    ///let response = session.pwa().launch(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn launch(&self, params: commands::LaunchParams) -> ydp_core::Request<commands::Launch> {
        self.session.send::<commands::Launch>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：PWA.command.launchFilesInApp。
    ///
    ///协议原文：
    ///
    ///<pre>Opens one or more local files from an installed web app identified by its
    ///manifestId. The web app needs to have file handlers registered to process
    ///the files. The API returns one or more page Target.TargetIDs which can be
    ///used to attach to via Target.attachToTarget or similar APIs.
    ///If some files in the parameters cannot be handled by the web app, they will
    ///be ignored. If none of the files can be handled, this API returns an error.
    ///If no files are provided as the parameter, this API also returns an error.
    ///
    ///According to the definition of the file handlers in the manifest file, one
    ///Target.TargetID may represent a page handling one or more files. The order
    ///of the returned Target.TargetIDs is not guaranteed.
    ///
    ///TODO(crbug.com/339454034): Check the existences of the input files.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::pwa::commands::LaunchFilesInAppParams) -> ydp_core::Result<()> {
    ///let response = session.pwa().launch_files_in_app(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn launch_files_in_app(
        &self,
        params: commands::LaunchFilesInAppParams,
    ) -> ydp_core::Request<commands::LaunchFilesInApp> {
        self.session.send::<commands::LaunchFilesInApp>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：PWA.command.openCurrentPageInApp。
    ///
    ///协议原文：
    ///
    ///<pre>Opens the current page in its web app identified by the manifest id, needs
    ///to be called on a page target. This function returns immediately without
    ///waiting for the app to finish loading.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::pwa::commands::OpenCurrentPageInAppParams) -> ydp_core::Result<()> {
    ///let response = session.pwa().open_current_page_in_app(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn open_current_page_in_app(
        &self,
        params: commands::OpenCurrentPageInAppParams,
    ) -> ydp_core::Request<commands::OpenCurrentPageInApp> {
        self.session.send::<commands::OpenCurrentPageInApp>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：PWA.command.uninstall。
    ///
    ///协议原文：
    ///
    ///<pre>Uninstalls the given manifest_id and closes any opened app windows.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::pwa::commands::UninstallParams) -> ydp_core::Result<()> {
    ///let response = session.pwa().uninstall(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn uninstall(
        &self,
        params: commands::UninstallParams,
    ) -> ydp_core::Request<commands::Uninstall> {
        self.session.send::<commands::Uninstall>(params)
    }
}
