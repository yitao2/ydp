///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：Log。
///
///协议原文：
///
///<pre>Provides access to log entries.</pre>
///
///
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Log.command.clear。
    ///
    ///协议原文：
    ///
    ///<pre>Clears the log.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::log::commands::ClearParams) -> ydp_core::Result<()> {
    ///let response = session.log().clear(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn clear(&self, params: commands::ClearParams) -> ydp_core::Request<commands::Clear> {
        self.session.send::<commands::Clear>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Log.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Disables log domain, prevents further log entries from being reported to the client.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::log::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.log().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Log.command.enable。
    ///
    ///协议原文：
    ///
    ///<pre>Enables log domain, sends the entries collected so far to the client by means of the
    ///`entryAdded` notification.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::log::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.log().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Log.command.startViolationsReport。
    ///
    ///协议原文：
    ///
    ///<pre>start violation reporting.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::log::commands::StartViolationsReportParams) -> ydp_core::Result<()> {
    ///let response = session.log().start_violations_report(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn start_violations_report(
        &self,
        params: commands::StartViolationsReportParams,
    ) -> ydp_core::Request<commands::StartViolationsReport> {
        self.session.send::<commands::StartViolationsReport>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Log.command.stopViolationsReport。
    ///
    ///协议原文：
    ///
    ///<pre>Stop violation reporting.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::log::commands::StopViolationsReportParams) -> ydp_core::Result<()> {
    ///let response = session.log().stop_violations_report(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn stop_violations_report(
        &self,
        params: commands::StopViolationsReportParams,
    ) -> ydp_core::Request<commands::StopViolationsReport> {
        self.session.send::<commands::StopViolationsReport>(params)
    }
}
