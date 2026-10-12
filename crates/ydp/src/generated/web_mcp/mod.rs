///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：WebMCP。此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：WebMCP.command.cancelInvocation。
    ///
    ///协议原文：
    ///
    ///<pre>Cancels a pending tool invocation.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::web_mcp::commands::CancelInvocationParams) -> ydp_core::Result<()> {
    ///let response = session.web_mcp().cancel_invocation(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn cancel_invocation(
        &self,
        params: commands::CancelInvocationParams,
    ) -> ydp_core::Request<commands::CancelInvocation> {
        self.session.send::<commands::CancelInvocation>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：WebMCP.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Disables the WebMCP domain.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::web_mcp::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.web_mcp().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：WebMCP.command.enable。
    ///
    ///协议原文：
    ///
    ///<pre>Enables the WebMCP domain, allowing events to be sent. Enabling the domain will trigger a toolsAdded event for
    ///all currently registered tools.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::web_mcp::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.web_mcp().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：WebMCP.command.invokeTool。
    ///
    ///协议原文：
    ///
    ///<pre>Invokes a registered tool.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::web_mcp::commands::InvokeToolParams) -> ydp_core::Result<()> {
    ///let response = session.web_mcp().invoke_tool(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn invoke_tool(
        &self,
        params: commands::InvokeToolParams,
    ) -> ydp_core::Request<commands::InvokeTool> {
        self.session.send::<commands::InvokeTool>(params)
    }
}
