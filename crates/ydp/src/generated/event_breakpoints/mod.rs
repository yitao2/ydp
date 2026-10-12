///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：EventBreakpoints。
///
///协议原文：
///
///<pre>EventBreakpoints permits setting JavaScript breakpoints on operations and events
///occurring in native code invoked from JavaScript. Once breakpoint is hit, it is
///reported through Debugger domain, similarly to regular breakpoints being hit.</pre>
///
///此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：EventBreakpoints.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Removes all breakpoints</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::event_breakpoints::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.event_breakpoints().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：EventBreakpoints.command.removeInstrumentationBreakpoint。
    ///
    ///协议原文：
    ///
    ///<pre>Removes breakpoint on particular native event.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::event_breakpoints::commands::RemoveInstrumentationBreakpointParams) -> ydp_core::Result<()> {
    ///let response = session.event_breakpoints().remove_instrumentation_breakpoint(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn remove_instrumentation_breakpoint(
        &self,
        params: commands::RemoveInstrumentationBreakpointParams,
    ) -> ydp_core::Request<commands::RemoveInstrumentationBreakpoint> {
        self.session
            .send::<commands::RemoveInstrumentationBreakpoint>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：EventBreakpoints.command.setInstrumentationBreakpoint。
    ///
    ///协议原文：
    ///
    ///<pre>Sets breakpoint on particular native event.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::event_breakpoints::commands::SetInstrumentationBreakpointParams) -> ydp_core::Result<()> {
    ///let response = session.event_breakpoints().set_instrumentation_breakpoint(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_instrumentation_breakpoint(
        &self,
        params: commands::SetInstrumentationBreakpointParams,
    ) -> ydp_core::Request<commands::SetInstrumentationBreakpoint> {
        self.session
            .send::<commands::SetInstrumentationBreakpoint>(params)
    }
}
