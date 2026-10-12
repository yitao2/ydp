///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：Performance。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Performance.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Disable collecting and reporting metrics.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::performance::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.performance().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Performance.command.enable。
    ///
    ///协议原文：
    ///
    ///<pre>Enable collecting and reporting metrics.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::performance::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.performance().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Performance.command.getMetrics。
    ///
    ///协议原文：
    ///
    ///<pre>Retrieve current values of run-time metrics.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::performance::commands::GetMetricsParams) -> ydp_core::Result<()> {
    ///let response = session.performance().get_metrics(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_metrics(
        &self,
        params: commands::GetMetricsParams,
    ) -> ydp_core::Request<commands::GetMetrics> {
        self.session.send::<commands::GetMetrics>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Performance.command.setTimeDomain。
    ///
    ///协议原文：
    ///
    ///<pre>Sets time domain to use for collecting and reporting duration metrics.
    ///Note that this must be called before enabling metrics collection. Calling
    ///this method while metrics collection is enabled returns an error.</pre>
    ///
    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::performance::commands::SetTimeDomainParams) -> ydp_core::Result<()> {
    ///let response = session.performance().set_time_domain(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_time_domain(
        &self,
        params: commands::SetTimeDomainParams,
    ) -> ydp_core::Request<commands::SetTimeDomain> {
        self.session.send::<commands::SetTimeDomain>(params)
    }
}
