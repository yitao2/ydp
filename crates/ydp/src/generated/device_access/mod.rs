///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：DeviceAccess。此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DeviceAccess.command.cancelPrompt。
    ///
    ///协议原文：
    ///
    ///<pre>Cancel a prompt in response to a DeviceAccess.deviceRequestPrompted event.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::device_access::commands::CancelPromptParams) -> ydp_core::Result<()> {
    ///let response = session.device_access().cancel_prompt(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn cancel_prompt(
        &self,
        params: commands::CancelPromptParams,
    ) -> ydp_core::Request<commands::CancelPrompt> {
        self.session.send::<commands::CancelPrompt>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DeviceAccess.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Disable events in this domain.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::device_access::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.device_access().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DeviceAccess.command.enable。
    ///
    ///协议原文：
    ///
    ///<pre>Enable events in this domain.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::device_access::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.device_access().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DeviceAccess.command.selectPrompt。
    ///
    ///协议原文：
    ///
    ///<pre>Select a device in response to a DeviceAccess.deviceRequestPrompted event.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::device_access::commands::SelectPromptParams) -> ydp_core::Result<()> {
    ///let response = session.device_access().select_prompt(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn select_prompt(
        &self,
        params: commands::SelectPromptParams,
    ) -> ydp_core::Request<commands::SelectPrompt> {
        self.session.send::<commands::SelectPrompt>(params)
    }
}
