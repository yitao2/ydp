///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：Security。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Security.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Disables tracking security state changes.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::security::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.security().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Security.command.enable。
    ///
    ///协议原文：
    ///
    ///<pre>Enables tracking security state changes.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::security::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.security().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Security.command.handleCertificateError。
    ///
    ///协议原文：
    ///
    ///<pre>Handles a certificate error that fired a certificateError event.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::security::commands::HandleCertificateErrorParams) -> ydp_core::Result<()> {
    ///let response = session.security().handle_certificate_error(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn handle_certificate_error(
        &self,
        params: commands::HandleCertificateErrorParams,
    ) -> ydp_core::Request<commands::HandleCertificateError> {
        self.session
            .send::<commands::HandleCertificateError>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Security.command.setIgnoreCertificateErrors。
    ///
    ///协议原文：
    ///
    ///<pre>Enable/disable whether all certificate errors should be ignored.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::security::commands::SetIgnoreCertificateErrorsParams) -> ydp_core::Result<()> {
    ///let response = session.security().set_ignore_certificate_errors(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_ignore_certificate_errors(
        &self,
        params: commands::SetIgnoreCertificateErrorsParams,
    ) -> ydp_core::Request<commands::SetIgnoreCertificateErrors> {
        self.session
            .send::<commands::SetIgnoreCertificateErrors>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Security.command.setOverrideCertificateErrors。
    ///
    ///协议原文：
    ///
    ///<pre>Enable/disable overriding certificate errors. If enabled, all certificate error events need to
    ///be handled by the DevTools client and should be answered with `handleCertificateError` commands.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::security::commands::SetOverrideCertificateErrorsParams) -> ydp_core::Result<()> {
    ///let response = session.security().set_override_certificate_errors(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_override_certificate_errors(
        &self,
        params: commands::SetOverrideCertificateErrorsParams,
    ) -> ydp_core::Request<commands::SetOverrideCertificateErrors> {
        self.session
            .send::<commands::SetOverrideCertificateErrors>(params)
    }
}
