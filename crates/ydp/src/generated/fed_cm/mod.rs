///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：FedCm。
///
///协议原文：
///
///<pre>This domain allows interacting with the FedCM dialog.</pre>
///
///此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：FedCm.command.clickDialogButton。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::fed_cm::commands::ClickDialogButtonParams) -> ydp_core::Result<()> {
    ///let response = session.fed_cm().click_dialog_button(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn click_dialog_button(
        &self,
        params: commands::ClickDialogButtonParams,
    ) -> ydp_core::Request<commands::ClickDialogButton> {
        self.session.send::<commands::ClickDialogButton>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：FedCm.command.disable。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::fed_cm::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.fed_cm().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：FedCm.command.dismissDialog。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::fed_cm::commands::DismissDialogParams) -> ydp_core::Result<()> {
    ///let response = session.fed_cm().dismiss_dialog(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn dismiss_dialog(
        &self,
        params: commands::DismissDialogParams,
    ) -> ydp_core::Request<commands::DismissDialog> {
        self.session.send::<commands::DismissDialog>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：FedCm.command.enable。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::fed_cm::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.fed_cm().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：FedCm.command.openUrl。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::fed_cm::commands::OpenUrlParams) -> ydp_core::Result<()> {
    ///let response = session.fed_cm().open_url(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn open_url(
        &self,
        params: commands::OpenUrlParams,
    ) -> ydp_core::Request<commands::OpenUrl> {
        self.session.send::<commands::OpenUrl>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：FedCm.command.resetCooldown。
    ///
    ///协议原文：
    ///
    ///<pre>Resets the cooldown time, if any, to allow the next FedCM call to show
    ///a dialog even if one was recently dismissed by the user.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::fed_cm::commands::ResetCooldownParams) -> ydp_core::Result<()> {
    ///let response = session.fed_cm().reset_cooldown(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn reset_cooldown(
        &self,
        params: commands::ResetCooldownParams,
    ) -> ydp_core::Request<commands::ResetCooldown> {
        self.session.send::<commands::ResetCooldown>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：FedCm.command.selectAccount。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::fed_cm::commands::SelectAccountParams) -> ydp_core::Result<()> {
    ///let response = session.fed_cm().select_account(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn select_account(
        &self,
        params: commands::SelectAccountParams,
    ) -> ydp_core::Request<commands::SelectAccount> {
        self.session.send::<commands::SelectAccount>(params)
    }
}
