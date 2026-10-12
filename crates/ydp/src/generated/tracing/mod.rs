///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：Tracing。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Tracing.command.end。
    ///
    ///协议原文：
    ///
    ///<pre>Stop trace events collection.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::tracing::commands::EndParams) -> ydp_core::Result<()> {
    ///let response = session.tracing().end(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn end(&self, params: commands::EndParams) -> ydp_core::Request<commands::End> {
        self.session.send::<commands::End>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Tracing.command.getCategories。
    ///
    ///协议原文：
    ///
    ///<pre>Gets supported tracing categories.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::tracing::commands::GetCategoriesParams) -> ydp_core::Result<()> {
    ///let response = session.tracing().get_categories(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_categories(
        &self,
        params: commands::GetCategoriesParams,
    ) -> ydp_core::Request<commands::GetCategories> {
        self.session.send::<commands::GetCategories>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Tracing.command.getTrackEventDescriptor。
    ///
    ///协议原文：
    ///
    ///<pre>Return a descriptor for all available tracing categories.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::tracing::commands::GetTrackEventDescriptorParams) -> ydp_core::Result<()> {
    ///let response = session.tracing().get_track_event_descriptor(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_track_event_descriptor(
        &self,
        params: commands::GetTrackEventDescriptorParams,
    ) -> ydp_core::Request<commands::GetTrackEventDescriptor> {
        self.session
            .send::<commands::GetTrackEventDescriptor>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Tracing.command.recordClockSyncMarker。
    ///
    ///协议原文：
    ///
    ///<pre>Record a clock sync marker in the trace.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::tracing::commands::RecordClockSyncMarkerParams) -> ydp_core::Result<()> {
    ///let response = session.tracing().record_clock_sync_marker(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn record_clock_sync_marker(
        &self,
        params: commands::RecordClockSyncMarkerParams,
    ) -> ydp_core::Request<commands::RecordClockSyncMarker> {
        self.session.send::<commands::RecordClockSyncMarker>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Tracing.command.requestMemoryDump。
    ///
    ///协议原文：
    ///
    ///<pre>Request a global memory dump.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::tracing::commands::RequestMemoryDumpParams) -> ydp_core::Result<()> {
    ///let response = session.tracing().request_memory_dump(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn request_memory_dump(
        &self,
        params: commands::RequestMemoryDumpParams,
    ) -> ydp_core::Request<commands::RequestMemoryDump> {
        self.session.send::<commands::RequestMemoryDump>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Tracing.command.start。
    ///
    ///协议原文：
    ///
    ///<pre>Start trace events collection.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::tracing::commands::StartParams) -> ydp_core::Result<()> {
    ///let response = session.tracing().start(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn start(&self, params: commands::StartParams) -> ydp_core::Request<commands::Start> {
        self.session.send::<commands::Start>(params)
    }
}
