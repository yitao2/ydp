///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：HeapProfiler。此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：HeapProfiler.command.addInspectedHeapObject。
    ///
    ///协议原文：
    ///
    ///<pre>Enables console to refer to the node with given id via $x (see Command Line API for more details
    ///$x functions).</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::heap_profiler::commands::AddInspectedHeapObjectParams) -> ydp_core::Result<()> {
    ///let response = session.heap_profiler().add_inspected_heap_object(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn add_inspected_heap_object(
        &self,
        params: commands::AddInspectedHeapObjectParams,
    ) -> ydp_core::Request<commands::AddInspectedHeapObject> {
        self.session
            .send::<commands::AddInspectedHeapObject>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：HeapProfiler.command.collectGarbage。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::heap_profiler::commands::CollectGarbageParams) -> ydp_core::Result<()> {
    ///let response = session.heap_profiler().collect_garbage(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn collect_garbage(
        &self,
        params: commands::CollectGarbageParams,
    ) -> ydp_core::Request<commands::CollectGarbage> {
        self.session.send::<commands::CollectGarbage>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：HeapProfiler.command.disable。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::heap_profiler::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.heap_profiler().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：HeapProfiler.command.enable。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::heap_profiler::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.heap_profiler().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：HeapProfiler.command.getHeapObjectId。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::heap_profiler::commands::GetHeapObjectIdParams) -> ydp_core::Result<()> {
    ///let response = session.heap_profiler().get_heap_object_id(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_heap_object_id(
        &self,
        params: commands::GetHeapObjectIdParams,
    ) -> ydp_core::Request<commands::GetHeapObjectId> {
        self.session.send::<commands::GetHeapObjectId>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：HeapProfiler.command.getObjectByHeapObjectId。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::heap_profiler::commands::GetObjectByHeapObjectIdParams) -> ydp_core::Result<()> {
    ///let response = session.heap_profiler().get_object_by_heap_object_id(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_object_by_heap_object_id(
        &self,
        params: commands::GetObjectByHeapObjectIdParams,
    ) -> ydp_core::Request<commands::GetObjectByHeapObjectId> {
        self.session
            .send::<commands::GetObjectByHeapObjectId>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：HeapProfiler.command.getSamplingProfile。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::heap_profiler::commands::GetSamplingProfileParams) -> ydp_core::Result<()> {
    ///let response = session.heap_profiler().get_sampling_profile(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_sampling_profile(
        &self,
        params: commands::GetSamplingProfileParams,
    ) -> ydp_core::Request<commands::GetSamplingProfile> {
        self.session.send::<commands::GetSamplingProfile>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：HeapProfiler.command.startSampling。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::heap_profiler::commands::StartSamplingParams) -> ydp_core::Result<()> {
    ///let response = session.heap_profiler().start_sampling(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn start_sampling(
        &self,
        params: commands::StartSamplingParams,
    ) -> ydp_core::Request<commands::StartSampling> {
        self.session.send::<commands::StartSampling>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：HeapProfiler.command.startTrackingHeapObjects。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::heap_profiler::commands::StartTrackingHeapObjectsParams) -> ydp_core::Result<()> {
    ///let response = session.heap_profiler().start_tracking_heap_objects(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn start_tracking_heap_objects(
        &self,
        params: commands::StartTrackingHeapObjectsParams,
    ) -> ydp_core::Request<commands::StartTrackingHeapObjects> {
        self.session
            .send::<commands::StartTrackingHeapObjects>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：HeapProfiler.command.stopSampling。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::heap_profiler::commands::StopSamplingParams) -> ydp_core::Result<()> {
    ///let response = session.heap_profiler().stop_sampling(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn stop_sampling(
        &self,
        params: commands::StopSamplingParams,
    ) -> ydp_core::Request<commands::StopSampling> {
        self.session.send::<commands::StopSampling>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：HeapProfiler.command.stopTrackingHeapObjects。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::heap_profiler::commands::StopTrackingHeapObjectsParams) -> ydp_core::Result<()> {
    ///let response = session.heap_profiler().stop_tracking_heap_objects(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn stop_tracking_heap_objects(
        &self,
        params: commands::StopTrackingHeapObjectsParams,
    ) -> ydp_core::Request<commands::StopTrackingHeapObjects> {
        self.session
            .send::<commands::StopTrackingHeapObjects>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：HeapProfiler.command.takeHeapSnapshot。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::heap_profiler::commands::TakeHeapSnapshotParams) -> ydp_core::Result<()> {
    ///let response = session.heap_profiler().take_heap_snapshot(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn take_heap_snapshot(
        &self,
        params: commands::TakeHeapSnapshotParams,
    ) -> ydp_core::Request<commands::TakeHeapSnapshot> {
        self.session.send::<commands::TakeHeapSnapshot>(params)
    }
}
