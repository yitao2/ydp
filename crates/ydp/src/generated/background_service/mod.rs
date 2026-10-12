///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：BackgroundService。
///
///协议原文：
///
///<pre>Defines events for background web platform features.</pre>
///
///此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：BackgroundService.command.clearEvents。
    ///
    ///协议原文：
    ///
    ///<pre>Clears all stored data for the service.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::background_service::commands::ClearEventsParams) -> ydp_core::Result<()> {
    ///let response = session.background_service().clear_events(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn clear_events(
        &self,
        params: commands::ClearEventsParams,
    ) -> ydp_core::Request<commands::ClearEvents> {
        self.session.send::<commands::ClearEvents>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：BackgroundService.command.setRecording。
    ///
    ///协议原文：
    ///
    ///<pre>Set the recording state for the service.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::background_service::commands::SetRecordingParams) -> ydp_core::Result<()> {
    ///let response = session.background_service().set_recording(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_recording(
        &self,
        params: commands::SetRecordingParams,
    ) -> ydp_core::Request<commands::SetRecording> {
        self.session.send::<commands::SetRecording>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：BackgroundService.command.startObserving。
    ///
    ///协议原文：
    ///
    ///<pre>Enables event updates for the service.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::background_service::commands::StartObservingParams) -> ydp_core::Result<()> {
    ///let response = session.background_service().start_observing(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn start_observing(
        &self,
        params: commands::StartObservingParams,
    ) -> ydp_core::Request<commands::StartObserving> {
        self.session.send::<commands::StartObserving>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：BackgroundService.command.stopObserving。
    ///
    ///协议原文：
    ///
    ///<pre>Disables event updates for the service.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::background_service::commands::StopObservingParams) -> ydp_core::Result<()> {
    ///let response = session.background_service().stop_observing(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn stop_observing(
        &self,
        params: commands::StopObservingParams,
    ) -> ydp_core::Request<commands::StopObserving> {
        self.session.send::<commands::StopObserving>(params)
    }
}
