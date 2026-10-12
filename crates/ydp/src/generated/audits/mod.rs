///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：Audits。
///
///协议原文：
///
///<pre>Audits domain allows investigation of page violations and possible improvements.</pre>
///
///此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Audits.command.checkFormsIssues。
    ///
    ///协议原文：
    ///
    ///<pre>Runs the form issues check for the target page. Found issues are reported
    ///using Audits.issueAdded event.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::audits::commands::CheckFormsIssuesParams) -> ydp_core::Result<()> {
    ///let response = session.audits().check_forms_issues(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn check_forms_issues(
        &self,
        params: commands::CheckFormsIssuesParams,
    ) -> ydp_core::Request<commands::CheckFormsIssues> {
        self.session.send::<commands::CheckFormsIssues>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Audits.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Disables issues domain, prevents further issues from being reported to the client.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::audits::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.audits().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Audits.command.enable。
    ///
    ///协议原文：
    ///
    ///<pre>Enables issues domain, sends the issues collected so far to the client by means of the
    ///`issueAdded` event.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::audits::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.audits().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Audits.command.getEncodedResponse。
    ///
    ///协议原文：
    ///
    ///<pre>Returns the response body and size if it were re-encoded with the specified settings. Only
    ///applies to images.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::audits::commands::GetEncodedResponseParams) -> ydp_core::Result<()> {
    ///let response = session.audits().get_encoded_response(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_encoded_response(
        &self,
        params: commands::GetEncodedResponseParams,
    ) -> ydp_core::Request<commands::GetEncodedResponse> {
        self.session.send::<commands::GetEncodedResponse>(params)
    }
}
