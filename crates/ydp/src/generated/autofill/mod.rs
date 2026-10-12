///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：Autofill。
///
///协议原文：
///
///<pre>Defines commands and events for Autofill.</pre>
///
///此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Autofill.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Disables autofill domain notifications.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::autofill::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.autofill().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Autofill.command.enable。
    ///
    ///协议原文：
    ///
    ///<pre>Enables autofill domain notifications.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::autofill::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.autofill().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Autofill.command.setAddresses。
    ///
    ///协议原文：
    ///
    ///<pre>Set addresses so that developers can verify their forms implementation.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::autofill::commands::SetAddressesParams) -> ydp_core::Result<()> {
    ///let response = session.autofill().set_addresses(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_addresses(
        &self,
        params: commands::SetAddressesParams,
    ) -> ydp_core::Request<commands::SetAddresses> {
        self.session.send::<commands::SetAddresses>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Autofill.command.trigger。
    ///
    ///协议原文：
    ///
    ///<pre>Trigger autofill on a form identified by the fieldId.
    ///If the field and related form cannot be autofilled, returns an error.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::autofill::commands::TriggerParams) -> ydp_core::Result<()> {
    ///let response = session.autofill().trigger(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn trigger(&self, params: commands::TriggerParams) -> ydp_core::Request<commands::Trigger> {
        self.session.send::<commands::Trigger>(params)
    }
}
