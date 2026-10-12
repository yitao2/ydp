///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：Cast。
///
///协议原文：
///
///<pre>A domain for interacting with Cast, Presentation API, and Remote Playback API
///functionalities.</pre>
///
///此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Cast.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Stops observing for sinks and issues.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::cast::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.cast().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Cast.command.enable。
    ///
    ///协议原文：
    ///
    ///<pre>Starts observing for sinks that can be used for tab mirroring, and if set,
    ///sinks compatible with |presentationUrl| as well. When sinks are found, a
    ///|sinksUpdated| event is fired.
    ///Also starts observing for issue messages. When an issue is added or removed,
    ///an |issueUpdated| event is fired.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::cast::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.cast().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Cast.command.setSinkToUse。
    ///
    ///协议原文：
    ///
    ///<pre>Sets a sink to be used when the web page requests the browser to choose a
    ///sink via Presentation API, Remote Playback API, or Cast SDK.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::cast::commands::SetSinkToUseParams) -> ydp_core::Result<()> {
    ///let response = session.cast().set_sink_to_use(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_sink_to_use(
        &self,
        params: commands::SetSinkToUseParams,
    ) -> ydp_core::Request<commands::SetSinkToUse> {
        self.session.send::<commands::SetSinkToUse>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Cast.command.startDesktopMirroring。
    ///
    ///协议原文：
    ///
    ///<pre>Starts mirroring the desktop to the sink.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::cast::commands::StartDesktopMirroringParams) -> ydp_core::Result<()> {
    ///let response = session.cast().start_desktop_mirroring(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn start_desktop_mirroring(
        &self,
        params: commands::StartDesktopMirroringParams,
    ) -> ydp_core::Request<commands::StartDesktopMirroring> {
        self.session.send::<commands::StartDesktopMirroring>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Cast.command.startTabMirroring。
    ///
    ///协议原文：
    ///
    ///<pre>Starts mirroring the tab to the sink.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::cast::commands::StartTabMirroringParams) -> ydp_core::Result<()> {
    ///let response = session.cast().start_tab_mirroring(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn start_tab_mirroring(
        &self,
        params: commands::StartTabMirroringParams,
    ) -> ydp_core::Request<commands::StartTabMirroring> {
        self.session.send::<commands::StartTabMirroring>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Cast.command.stopCasting。
    ///
    ///协议原文：
    ///
    ///<pre>Stops the active Cast session on the sink.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::cast::commands::StopCastingParams) -> ydp_core::Result<()> {
    ///let response = session.cast().stop_casting(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn stop_casting(
        &self,
        params: commands::StopCastingParams,
    ) -> ydp_core::Request<commands::StopCasting> {
        self.session.send::<commands::StopCasting>(params)
    }
}
