///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：Fetch。
///
///协议原文：
///
///<pre>A domain for letting clients substitute browser's network layer with client code.</pre>
///
///
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Fetch.command.continueRequest。
    ///
    ///协议原文：
    ///
    ///<pre>Continues the request, optionally modifying some of its parameters.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::fetch::commands::ContinueRequestParams) -> ydp_core::Result<()> {
    ///let response = session.fetch().continue_request(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn continue_request(
        &self,
        params: commands::ContinueRequestParams,
    ) -> ydp_core::Request<commands::ContinueRequest> {
        self.session.send::<commands::ContinueRequest>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Fetch.command.continueResponse。
    ///
    ///协议原文：
    ///
    ///<pre>Continues loading of the paused response, optionally modifying the
    ///response headers. If either responseCode or headers are modified, all of them
    ///must be present.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::fetch::commands::ContinueResponseParams) -> ydp_core::Result<()> {
    ///let response = session.fetch().continue_response(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn continue_response(
        &self,
        params: commands::ContinueResponseParams,
    ) -> ydp_core::Request<commands::ContinueResponse> {
        self.session.send::<commands::ContinueResponse>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Fetch.command.continueWithAuth。
    ///
    ///协议原文：
    ///
    ///<pre>Continues a request supplying authChallengeResponse following authRequired event.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::fetch::commands::ContinueWithAuthParams) -> ydp_core::Result<()> {
    ///let response = session.fetch().continue_with_auth(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn continue_with_auth(
        &self,
        params: commands::ContinueWithAuthParams,
    ) -> ydp_core::Request<commands::ContinueWithAuth> {
        self.session.send::<commands::ContinueWithAuth>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Fetch.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Disables the fetch domain.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::fetch::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.fetch().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Fetch.command.enable。
    ///
    ///协议原文：
    ///
    ///<pre>Enables issuing of requestPaused events. A request will be paused until client
    ///calls one of failRequest, fulfillRequest or continueRequest/continueWithAuth.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::fetch::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.fetch().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Fetch.command.failRequest。
    ///
    ///协议原文：
    ///
    ///<pre>Causes the request to fail with specified reason.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::fetch::commands::FailRequestParams) -> ydp_core::Result<()> {
    ///let response = session.fetch().fail_request(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn fail_request(
        &self,
        params: commands::FailRequestParams,
    ) -> ydp_core::Request<commands::FailRequest> {
        self.session.send::<commands::FailRequest>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Fetch.command.fulfillRequest。
    ///
    ///协议原文：
    ///
    ///<pre>Provides response to the request.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::fetch::commands::FulfillRequestParams) -> ydp_core::Result<()> {
    ///let response = session.fetch().fulfill_request(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn fulfill_request(
        &self,
        params: commands::FulfillRequestParams,
    ) -> ydp_core::Request<commands::FulfillRequest> {
        self.session.send::<commands::FulfillRequest>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Fetch.command.getResponseBody。
    ///
    ///协议原文：
    ///
    ///<pre>Causes the body of the response to be received from the server and
    ///returned as a single string. May only be issued for a request that
    ///is paused in the Response stage and is mutually exclusive with
    ///takeResponseBodyForInterceptionAsStream. Calling other methods that
    ///affect the request or disabling fetch domain before body is received
    ///results in an undefined behavior.
    ///Note that the response body is not available for redirects. Requests
    ///paused in the _redirect received_ state may be differentiated by
    ///`responseCode` and presence of `location` response header, see
    ///comments to `requestPaused` for details.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::fetch::commands::GetResponseBodyParams) -> ydp_core::Result<()> {
    ///let response = session.fetch().get_response_body(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_response_body(
        &self,
        params: commands::GetResponseBodyParams,
    ) -> ydp_core::Request<commands::GetResponseBody> {
        self.session.send::<commands::GetResponseBody>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Fetch.command.takeResponseBodyAsStream。
    ///
    ///协议原文：
    ///
    ///<pre>Returns a handle to the stream representing the response body.
    ///The request must be paused in the HeadersReceived stage.
    ///Note that after this command the request can't be continued
    ///as is -- client either needs to cancel it or to provide the
    ///response body.
    ///The stream only supports sequential read, IO.read will fail if the position
    ///is specified.
    ///This method is mutually exclusive with getResponseBody.
    ///Calling other methods that affect the request or disabling fetch
    ///domain before body is received results in an undefined behavior.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::fetch::commands::TakeResponseBodyAsStreamParams) -> ydp_core::Result<()> {
    ///let response = session.fetch().take_response_body_as_stream(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn take_response_body_as_stream(
        &self,
        params: commands::TakeResponseBodyAsStreamParams,
    ) -> ydp_core::Request<commands::TakeResponseBodyAsStream> {
        self.session
            .send::<commands::TakeResponseBodyAsStream>(params)
    }
}
