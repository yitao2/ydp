///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：Profiler。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Profiler.command.disable。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::profiler::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.profiler().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Profiler.command.enable。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::profiler::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.profiler().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Profiler.command.getBestEffortCoverage。
    ///
    ///协议原文：
    ///
    ///<pre>Collect coverage data for the current isolate. The coverage data may be incomplete due to
    ///garbage collection.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::profiler::commands::GetBestEffortCoverageParams) -> ydp_core::Result<()> {
    ///let response = session.profiler().get_best_effort_coverage(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_best_effort_coverage(
        &self,
        params: commands::GetBestEffortCoverageParams,
    ) -> ydp_core::Request<commands::GetBestEffortCoverage> {
        self.session.send::<commands::GetBestEffortCoverage>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Profiler.command.setSamplingInterval。
    ///
    ///协议原文：
    ///
    ///<pre>Changes CPU profiler sampling interval. Must be called before CPU profiles recording started.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::profiler::commands::SetSamplingIntervalParams) -> ydp_core::Result<()> {
    ///let response = session.profiler().set_sampling_interval(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_sampling_interval(
        &self,
        params: commands::SetSamplingIntervalParams,
    ) -> ydp_core::Request<commands::SetSamplingInterval> {
        self.session.send::<commands::SetSamplingInterval>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Profiler.command.start。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::profiler::commands::StartParams) -> ydp_core::Result<()> {
    ///let response = session.profiler().start(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn start(&self, params: commands::StartParams) -> ydp_core::Request<commands::Start> {
        self.session.send::<commands::Start>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Profiler.command.startPreciseCoverage。
    ///
    ///协议原文：
    ///
    ///<pre>Enable precise code coverage. Coverage data for JavaScript executed before enabling precise code
    ///coverage may be incomplete. Enabling prevents running optimized code and resets execution
    ///counters.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::profiler::commands::StartPreciseCoverageParams) -> ydp_core::Result<()> {
    ///let response = session.profiler().start_precise_coverage(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn start_precise_coverage(
        &self,
        params: commands::StartPreciseCoverageParams,
    ) -> ydp_core::Request<commands::StartPreciseCoverage> {
        self.session.send::<commands::StartPreciseCoverage>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Profiler.command.stop。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::profiler::commands::StopParams) -> ydp_core::Result<()> {
    ///let response = session.profiler().stop(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn stop(&self, params: commands::StopParams) -> ydp_core::Request<commands::Stop> {
        self.session.send::<commands::Stop>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Profiler.command.stopPreciseCoverage。
    ///
    ///协议原文：
    ///
    ///<pre>Disable precise code coverage. Disabling releases unnecessary execution count records and allows
    ///executing optimized code.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::profiler::commands::StopPreciseCoverageParams) -> ydp_core::Result<()> {
    ///let response = session.profiler().stop_precise_coverage(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn stop_precise_coverage(
        &self,
        params: commands::StopPreciseCoverageParams,
    ) -> ydp_core::Request<commands::StopPreciseCoverage> {
        self.session.send::<commands::StopPreciseCoverage>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Profiler.command.takePreciseCoverage。
    ///
    ///协议原文：
    ///
    ///<pre>Collect coverage data for the current isolate, and resets execution counters. Precise code
    ///coverage needs to have started.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::profiler::commands::TakePreciseCoverageParams) -> ydp_core::Result<()> {
    ///let response = session.profiler().take_precise_coverage(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn take_precise_coverage(
        &self,
        params: commands::TakePreciseCoverageParams,
    ) -> ydp_core::Request<commands::TakePreciseCoverage> {
        self.session.send::<commands::TakePreciseCoverage>(params)
    }
}
