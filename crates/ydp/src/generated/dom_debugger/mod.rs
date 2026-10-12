///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：DOMDebugger。
///
///协议原文：
///
///<pre>DOM debugging allows setting breakpoints on particular DOM operations and events. JavaScript
///execution will stop on these operations as if there was a regular breakpoint set.</pre>
///
///
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOMDebugger.command.getEventListeners。
    ///
    ///协议原文：
    ///
    ///<pre>Returns event listeners of the given object.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom_debugger::commands::GetEventListenersParams) -> ydp_core::Result<()> {
    ///let response = session.dom_debugger().get_event_listeners(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_event_listeners(
        &self,
        params: commands::GetEventListenersParams,
    ) -> ydp_core::Request<commands::GetEventListeners> {
        self.session.send::<commands::GetEventListeners>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOMDebugger.command.removeDOMBreakpoint。
    ///
    ///协议原文：
    ///
    ///<pre>Removes DOM breakpoint that was set using `setDOMBreakpoint`.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom_debugger::commands::RemoveDomBreakpointParams) -> ydp_core::Result<()> {
    ///let response = session.dom_debugger().remove_dom_breakpoint(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn remove_dom_breakpoint(
        &self,
        params: commands::RemoveDomBreakpointParams,
    ) -> ydp_core::Request<commands::RemoveDomBreakpoint> {
        self.session.send::<commands::RemoveDomBreakpoint>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOMDebugger.command.removeEventListenerBreakpoint。
    ///
    ///协议原文：
    ///
    ///<pre>Removes breakpoint on particular DOM event.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom_debugger::commands::RemoveEventListenerBreakpointParams) -> ydp_core::Result<()> {
    ///let response = session.dom_debugger().remove_event_listener_breakpoint(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn remove_event_listener_breakpoint(
        &self,
        params: commands::RemoveEventListenerBreakpointParams,
    ) -> ydp_core::Request<commands::RemoveEventListenerBreakpoint> {
        self.session
            .send::<commands::RemoveEventListenerBreakpoint>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOMDebugger.command.removeInstrumentationBreakpoint。
    ///
    ///协议原文：
    ///
    ///<pre>Removes breakpoint on particular native event.</pre>
    ///
    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。此命令由协议重定向到 EventBreakpoints 域处理。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom_debugger::commands::RemoveInstrumentationBreakpointParams) -> ydp_core::Result<()> {
    ///let response = session.dom_debugger().remove_instrumentation_breakpoint(params).await?;
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
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOMDebugger.command.removeXHRBreakpoint。
    ///
    ///协议原文：
    ///
    ///<pre>Removes breakpoint from XMLHttpRequest.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom_debugger::commands::RemoveXhrBreakpointParams) -> ydp_core::Result<()> {
    ///let response = session.dom_debugger().remove_xhr_breakpoint(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn remove_xhr_breakpoint(
        &self,
        params: commands::RemoveXhrBreakpointParams,
    ) -> ydp_core::Request<commands::RemoveXhrBreakpoint> {
        self.session.send::<commands::RemoveXhrBreakpoint>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOMDebugger.command.setBreakOnCSPViolation。
    ///
    ///协议原文：
    ///
    ///<pre>Sets breakpoint on particular CSP violations.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom_debugger::commands::SetBreakOnCspViolationParams) -> ydp_core::Result<()> {
    ///let response = session.dom_debugger().set_break_on_csp_violation(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_break_on_csp_violation(
        &self,
        params: commands::SetBreakOnCspViolationParams,
    ) -> ydp_core::Request<commands::SetBreakOnCspViolation> {
        self.session
            .send::<commands::SetBreakOnCspViolation>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOMDebugger.command.setDOMBreakpoint。
    ///
    ///协议原文：
    ///
    ///<pre>Sets breakpoint on particular operation with DOM.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom_debugger::commands::SetDomBreakpointParams) -> ydp_core::Result<()> {
    ///let response = session.dom_debugger().set_dom_breakpoint(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_dom_breakpoint(
        &self,
        params: commands::SetDomBreakpointParams,
    ) -> ydp_core::Request<commands::SetDomBreakpoint> {
        self.session.send::<commands::SetDomBreakpoint>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOMDebugger.command.setEventListenerBreakpoint。
    ///
    ///协议原文：
    ///
    ///<pre>Sets breakpoint on particular DOM event.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom_debugger::commands::SetEventListenerBreakpointParams) -> ydp_core::Result<()> {
    ///let response = session.dom_debugger().set_event_listener_breakpoint(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_event_listener_breakpoint(
        &self,
        params: commands::SetEventListenerBreakpointParams,
    ) -> ydp_core::Request<commands::SetEventListenerBreakpoint> {
        self.session
            .send::<commands::SetEventListenerBreakpoint>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOMDebugger.command.setInstrumentationBreakpoint。
    ///
    ///协议原文：
    ///
    ///<pre>Sets breakpoint on particular native event.</pre>
    ///
    ///此项为 experimental，端点可能不支持。此项已 deprecated，保留用于旧端点兼容。此命令由协议重定向到 EventBreakpoints 域处理。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom_debugger::commands::SetInstrumentationBreakpointParams) -> ydp_core::Result<()> {
    ///let response = session.dom_debugger().set_instrumentation_breakpoint(params).await?;
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
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOMDebugger.command.setXHRBreakpoint。
    ///
    ///协议原文：
    ///
    ///<pre>Sets breakpoint on XMLHttpRequest.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom_debugger::commands::SetXhrBreakpointParams) -> ydp_core::Result<()> {
    ///let response = session.dom_debugger().set_xhr_breakpoint(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_xhr_breakpoint(
        &self,
        params: commands::SetXhrBreakpointParams,
    ) -> ydp_core::Request<commands::SetXhrBreakpoint> {
        self.session.send::<commands::SetXhrBreakpoint>(params)
    }
}
