///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：Console。
///
///协议原文：
///
///<pre>This domain is deprecated - use Runtime or Log instead.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Console.command.clearMessages。
    ///
    ///协议原文：
    ///
    ///<pre>Does nothing.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::console::commands::ClearMessagesParams) -> ydp_core::Result<()> {
    ///let response = session.console().clear_messages(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn clear_messages(
        &self,
        params: commands::ClearMessagesParams,
    ) -> ydp_core::Request<commands::ClearMessages> {
        self.session.send::<commands::ClearMessages>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Console.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Disables console domain, prevents further console messages from being reported to the client.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::console::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.console().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Console.command.enable。
    ///
    ///协议原文：
    ///
    ///<pre>Enables console domain, sends the messages collected so far to the client by means of the
    ///`messageAdded` notification.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::console::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.console().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
}
