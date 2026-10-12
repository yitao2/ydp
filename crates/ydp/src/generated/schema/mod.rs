///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：Schema。
///
///协议原文：
///
///<pre>This domain is deprecated.</pre>
///
///此项已 deprecated，保留用于旧端点兼容。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Schema.command.getDomains。
    ///
    ///协议原文：
    ///
    ///<pre>Returns supported domains.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::schema::commands::GetDomainsParams) -> ydp_core::Result<()> {
    ///let response = session.schema().get_domains(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_domains(
        &self,
        params: commands::GetDomainsParams,
    ) -> ydp_core::Request<commands::GetDomains> {
        self.session.send::<commands::GetDomains>(params)
    }
}
