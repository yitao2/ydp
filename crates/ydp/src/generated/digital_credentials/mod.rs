///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：DigitalCredentials。
///
///协议原文：
///
///<pre>This domain allows interacting with the Digital Credentials API for automation.</pre>
///
///此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DigitalCredentials.command.setVirtualWalletBehavior。
    ///
    ///协议原文：
    ///
    ///<pre>Sets the behavior of the virtual wallet for digital credential requests
    ///issued from this frame.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::digital_credentials::commands::SetVirtualWalletBehaviorParams) -> ydp_core::Result<()> {
    ///let response = session.digital_credentials().set_virtual_wallet_behavior(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_virtual_wallet_behavior(
        &self,
        params: commands::SetVirtualWalletBehaviorParams,
    ) -> ydp_core::Request<commands::SetVirtualWalletBehavior> {
        self.session
            .send::<commands::SetVirtualWalletBehavior>(params)
    }
}
