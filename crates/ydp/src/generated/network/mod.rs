///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：Network。
///
///协议原文：
///
///<pre>Network domain allows tracking network activities of the page. It exposes information about http,
///file, data and other requests and responses, their headers, bodies, timing, etc.</pre>
///
///
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.canClearBrowserCache。
    ///
    ///协议原文：
    ///
    ///<pre>Tells whether clearing browser cache is supported.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::CanClearBrowserCacheParams) -> ydp_core::Result<()> {
    ///let response = session.network().can_clear_browser_cache(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn can_clear_browser_cache(
        &self,
        params: commands::CanClearBrowserCacheParams,
    ) -> ydp_core::Request<commands::CanClearBrowserCache> {
        self.session.send::<commands::CanClearBrowserCache>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.canClearBrowserCookies。
    ///
    ///协议原文：
    ///
    ///<pre>Tells whether clearing browser cookies is supported.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::CanClearBrowserCookiesParams) -> ydp_core::Result<()> {
    ///let response = session.network().can_clear_browser_cookies(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn can_clear_browser_cookies(
        &self,
        params: commands::CanClearBrowserCookiesParams,
    ) -> ydp_core::Request<commands::CanClearBrowserCookies> {
        self.session
            .send::<commands::CanClearBrowserCookies>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.canEmulateNetworkConditions。
    ///
    ///协议原文：
    ///
    ///<pre>Tells whether emulation of network conditions is supported.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::CanEmulateNetworkConditionsParams) -> ydp_core::Result<()> {
    ///let response = session.network().can_emulate_network_conditions(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn can_emulate_network_conditions(
        &self,
        params: commands::CanEmulateNetworkConditionsParams,
    ) -> ydp_core::Request<commands::CanEmulateNetworkConditions> {
        self.session
            .send::<commands::CanEmulateNetworkConditions>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.clearBrowserCache。
    ///
    ///协议原文：
    ///
    ///<pre>Clears browser cache.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::ClearBrowserCacheParams) -> ydp_core::Result<()> {
    ///let response = session.network().clear_browser_cache(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn clear_browser_cache(
        &self,
        params: commands::ClearBrowserCacheParams,
    ) -> ydp_core::Request<commands::ClearBrowserCache> {
        self.session.send::<commands::ClearBrowserCache>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.clearBrowserCookies。
    ///
    ///协议原文：
    ///
    ///<pre>Clears browser cookies.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::ClearBrowserCookiesParams) -> ydp_core::Result<()> {
    ///let response = session.network().clear_browser_cookies(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn clear_browser_cookies(
        &self,
        params: commands::ClearBrowserCookiesParams,
    ) -> ydp_core::Request<commands::ClearBrowserCookies> {
        self.session.send::<commands::ClearBrowserCookies>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.configureDurableMessages。
    ///
    ///协议原文：
    ///
    ///<pre>Configures storing response bodies outside of renderer, so that these survive
    ///a cross-process navigation.
    ///If maxTotalBufferSize is not set, durable messages are disabled.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::ConfigureDurableMessagesParams) -> ydp_core::Result<()> {
    ///let response = session.network().configure_durable_messages(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn configure_durable_messages(
        &self,
        params: commands::ConfigureDurableMessagesParams,
    ) -> ydp_core::Request<commands::ConfigureDurableMessages> {
        self.session
            .send::<commands::ConfigureDurableMessages>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.deleteCookies。
    ///
    ///协议原文：
    ///
    ///<pre>Deletes browser cookies with matching name and url or domain/path/partitionKey pair.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::DeleteCookiesParams) -> ydp_core::Result<()> {
    ///let response = session.network().delete_cookies(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn delete_cookies(
        &self,
        params: commands::DeleteCookiesParams,
    ) -> ydp_core::Request<commands::DeleteCookies> {
        self.session.send::<commands::DeleteCookies>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.deleteDeviceBoundSession。
    ///
    ///协议原文：
    ///
    ///<pre>Deletes a device bound session.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::DeleteDeviceBoundSessionParams) -> ydp_core::Result<()> {
    ///let response = session.network().delete_device_bound_session(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn delete_device_bound_session(
        &self,
        params: commands::DeleteDeviceBoundSessionParams,
    ) -> ydp_core::Request<commands::DeleteDeviceBoundSession> {
        self.session
            .send::<commands::DeleteDeviceBoundSession>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Disables network tracking, prevents network events from being sent to the client.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.network().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.emulateNetworkConditions。
    ///
    ///协议原文：
    ///
    ///<pre>Activates emulation of network conditions. This command is deprecated in favor of the emulateNetworkConditionsByRule
    ///and overrideNetworkState commands, which can be used together to the same effect.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::EmulateNetworkConditionsParams) -> ydp_core::Result<()> {
    ///let response = session.network().emulate_network_conditions(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn emulate_network_conditions(
        &self,
        params: commands::EmulateNetworkConditionsParams,
    ) -> ydp_core::Request<commands::EmulateNetworkConditions> {
        self.session
            .send::<commands::EmulateNetworkConditions>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.emulateNetworkConditionsByRule。
    ///
    ///协议原文：
    ///
    ///<pre>Activates emulation of network conditions for individual requests using URL match patterns. Unlike the deprecated
    ///Network.emulateNetworkConditions this method does not affect `navigator` state. Use Network.overrideNetworkState to
    ///explicitly modify `navigator` behavior.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::EmulateNetworkConditionsByRuleParams) -> ydp_core::Result<()> {
    ///let response = session.network().emulate_network_conditions_by_rule(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn emulate_network_conditions_by_rule(
        &self,
        params: commands::EmulateNetworkConditionsByRuleParams,
    ) -> ydp_core::Request<commands::EmulateNetworkConditionsByRule> {
        self.session
            .send::<commands::EmulateNetworkConditionsByRule>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.enable。
    ///
    ///协议原文：
    ///
    ///<pre>Enables network tracking, network events will now be delivered to the client.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.network().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.enableDeviceBoundSessions。
    ///
    ///协议原文：
    ///
    ///<pre>Sets up tracking device bound sessions and fetching of initial set of sessions.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::EnableDeviceBoundSessionsParams) -> ydp_core::Result<()> {
    ///let response = session.network().enable_device_bound_sessions(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable_device_bound_sessions(
        &self,
        params: commands::EnableDeviceBoundSessionsParams,
    ) -> ydp_core::Request<commands::EnableDeviceBoundSessions> {
        self.session
            .send::<commands::EnableDeviceBoundSessions>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.enableReportingApi。
    ///
    ///协议原文：
    ///
    ///<pre>Enables tracking for the Reporting API, events generated by the Reporting API will now be delivered to the client.
    ///Enabling triggers 'reportingApiReportAdded' for all existing reports.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::EnableReportingApiParams) -> ydp_core::Result<()> {
    ///let response = session.network().enable_reporting_api(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable_reporting_api(
        &self,
        params: commands::EnableReportingApiParams,
    ) -> ydp_core::Request<commands::EnableReportingApi> {
        self.session.send::<commands::EnableReportingApi>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.fetchSchemefulSite。
    ///
    ///协议原文：
    ///
    ///<pre>Fetches the schemeful site for a specific origin.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::FetchSchemefulSiteParams) -> ydp_core::Result<()> {
    ///let response = session.network().fetch_schemeful_site(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn fetch_schemeful_site(
        &self,
        params: commands::FetchSchemefulSiteParams,
    ) -> ydp_core::Request<commands::FetchSchemefulSite> {
        self.session.send::<commands::FetchSchemefulSite>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.getAllCookies。
    ///
    ///协议原文：
    ///
    ///<pre>Returns all browser cookies. Depending on the backend support, will return detailed cookie
    ///information in the `cookies` field.
    ///Deprecated. Use Storage.getCookies instead.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::GetAllCookiesParams) -> ydp_core::Result<()> {
    ///let response = session.network().get_all_cookies(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_all_cookies(
        &self,
        params: commands::GetAllCookiesParams,
    ) -> ydp_core::Request<commands::GetAllCookies> {
        self.session.send::<commands::GetAllCookies>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.getCertificate。
    ///
    ///协议原文：
    ///
    ///<pre>Returns the DER-encoded certificate.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::GetCertificateParams) -> ydp_core::Result<()> {
    ///let response = session.network().get_certificate(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_certificate(
        &self,
        params: commands::GetCertificateParams,
    ) -> ydp_core::Request<commands::GetCertificate> {
        self.session.send::<commands::GetCertificate>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.getCookies。
    ///
    ///协议原文：
    ///
    ///<pre>Returns all browser cookies for the current URL. Depending on the backend support, will return
    ///detailed cookie information in the `cookies` field.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::GetCookiesParams) -> ydp_core::Result<()> {
    ///let response = session.network().get_cookies(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_cookies(
        &self,
        params: commands::GetCookiesParams,
    ) -> ydp_core::Request<commands::GetCookies> {
        self.session.send::<commands::GetCookies>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.getRequestPostData。
    ///
    ///协议原文：
    ///
    ///<pre>Returns post data sent with the request. Returns an error when no data was sent with the request.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::GetRequestPostDataParams) -> ydp_core::Result<()> {
    ///let response = session.network().get_request_post_data(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_request_post_data(
        &self,
        params: commands::GetRequestPostDataParams,
    ) -> ydp_core::Request<commands::GetRequestPostData> {
        self.session.send::<commands::GetRequestPostData>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.getResponseBody。
    ///
    ///协议原文：
    ///
    ///<pre>Returns content served for the given request.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::GetResponseBodyParams) -> ydp_core::Result<()> {
    ///let response = session.network().get_response_body(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_response_body(
        &self,
        params: commands::GetResponseBodyParams,
    ) -> ydp_core::Request<commands::GetResponseBody> {
        self.session.send::<commands::GetResponseBody>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.getSecurityIsolationStatus。
    ///
    ///协议原文：
    ///
    ///<pre>Returns information about the COEP/COOP isolation status.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::GetSecurityIsolationStatusParams) -> ydp_core::Result<()> {
    ///let response = session.network().get_security_isolation_status(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_security_isolation_status(
        &self,
        params: commands::GetSecurityIsolationStatusParams,
    ) -> ydp_core::Request<commands::GetSecurityIsolationStatus> {
        self.session
            .send::<commands::GetSecurityIsolationStatus>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.loadNetworkResource。
    ///
    ///协议原文：
    ///
    ///<pre>Fetches the resource and returns the content.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::LoadNetworkResourceParams) -> ydp_core::Result<()> {
    ///let response = session.network().load_network_resource(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn load_network_resource(
        &self,
        params: commands::LoadNetworkResourceParams,
    ) -> ydp_core::Request<commands::LoadNetworkResource> {
        self.session.send::<commands::LoadNetworkResource>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.overrideNetworkState。
    ///
    ///协议原文：
    ///
    ///<pre>Override the state of navigator.onLine and navigator.connection.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::OverrideNetworkStateParams) -> ydp_core::Result<()> {
    ///let response = session.network().override_network_state(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn override_network_state(
        &self,
        params: commands::OverrideNetworkStateParams,
    ) -> ydp_core::Request<commands::OverrideNetworkState> {
        self.session.send::<commands::OverrideNetworkState>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.replayXHR。
    ///
    ///协议原文：
    ///
    ///<pre>This method sends a new XMLHttpRequest which is identical to the original one. The following
    ///parameters should be identical: method, url, async, request body, extra headers, withCredentials
    ///attribute, user, password.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::ReplayXhrParams) -> ydp_core::Result<()> {
    ///let response = session.network().replay_xhr(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn replay_xhr(
        &self,
        params: commands::ReplayXhrParams,
    ) -> ydp_core::Request<commands::ReplayXhr> {
        self.session.send::<commands::ReplayXhr>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.searchInResponseBody。
    ///
    ///协议原文：
    ///
    ///<pre>Searches for given string in response content.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::SearchInResponseBodyParams) -> ydp_core::Result<()> {
    ///let response = session.network().search_in_response_body(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn search_in_response_body(
        &self,
        params: commands::SearchInResponseBodyParams,
    ) -> ydp_core::Request<commands::SearchInResponseBody> {
        self.session.send::<commands::SearchInResponseBody>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.setAttachDebugStack。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies whether to attach a page script stack id in requests</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::SetAttachDebugStackParams) -> ydp_core::Result<()> {
    ///let response = session.network().set_attach_debug_stack(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_attach_debug_stack(
        &self,
        params: commands::SetAttachDebugStackParams,
    ) -> ydp_core::Request<commands::SetAttachDebugStack> {
        self.session.send::<commands::SetAttachDebugStack>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.setBlockedURLs。
    ///
    ///协议原文：
    ///
    ///<pre>Blocks URLs from loading.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::SetBlockedUrLsParams) -> ydp_core::Result<()> {
    ///let response = session.network().set_blocked_ur_ls(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_blocked_ur_ls(
        &self,
        params: commands::SetBlockedUrLsParams,
    ) -> ydp_core::Request<commands::SetBlockedUrLs> {
        self.session.send::<commands::SetBlockedUrLs>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.setBypassServiceWorker。
    ///
    ///协议原文：
    ///
    ///<pre>Toggles ignoring of service worker for each request.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::SetBypassServiceWorkerParams) -> ydp_core::Result<()> {
    ///let response = session.network().set_bypass_service_worker(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_bypass_service_worker(
        &self,
        params: commands::SetBypassServiceWorkerParams,
    ) -> ydp_core::Request<commands::SetBypassServiceWorker> {
        self.session
            .send::<commands::SetBypassServiceWorker>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.setCacheDisabled。
    ///
    ///协议原文：
    ///
    ///<pre>Toggles ignoring cache for each request. If `true`, cache will not be used.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::SetCacheDisabledParams) -> ydp_core::Result<()> {
    ///let response = session.network().set_cache_disabled(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_cache_disabled(
        &self,
        params: commands::SetCacheDisabledParams,
    ) -> ydp_core::Request<commands::SetCacheDisabled> {
        self.session.send::<commands::SetCacheDisabled>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.setCookie。
    ///
    ///协议原文：
    ///
    ///<pre>Sets a cookie with the given cookie data; may overwrite equivalent cookies if they exist.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::SetCookieParams) -> ydp_core::Result<()> {
    ///let response = session.network().set_cookie(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_cookie(
        &self,
        params: commands::SetCookieParams,
    ) -> ydp_core::Request<commands::SetCookie> {
        self.session.send::<commands::SetCookie>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.setCookieControls。
    ///
    ///协议原文：
    ///
    ///<pre>Sets Controls for third-party cookie access
    ///Page reload is required before the new cookie behavior will be observed</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::SetCookieControlsParams) -> ydp_core::Result<()> {
    ///let response = session.network().set_cookie_controls(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_cookie_controls(
        &self,
        params: commands::SetCookieControlsParams,
    ) -> ydp_core::Request<commands::SetCookieControls> {
        self.session.send::<commands::SetCookieControls>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.setCookies。
    ///
    ///协议原文：
    ///
    ///<pre>Sets given cookies.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::SetCookiesParams) -> ydp_core::Result<()> {
    ///let response = session.network().set_cookies(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_cookies(
        &self,
        params: commands::SetCookiesParams,
    ) -> ydp_core::Request<commands::SetCookies> {
        self.session.send::<commands::SetCookies>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.setExtraHTTPHeaders。
    ///
    ///协议原文：
    ///
    ///<pre>Specifies whether to always send extra HTTP headers with the requests from this page.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::SetExtraHttpHeadersParams) -> ydp_core::Result<()> {
    ///let response = session.network().set_extra_http_headers(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_extra_http_headers(
        &self,
        params: commands::SetExtraHttpHeadersParams,
    ) -> ydp_core::Request<commands::SetExtraHttpHeaders> {
        self.session.send::<commands::SetExtraHttpHeaders>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.setUserAgentOverride。
    ///
    ///协议原文：
    ///
    ///<pre>Allows overriding user agent with the given string.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::SetUserAgentOverrideParams) -> ydp_core::Result<()> {
    ///let response = session.network().set_user_agent_override(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_user_agent_override(
        &self,
        params: commands::SetUserAgentOverrideParams,
    ) -> ydp_core::Request<commands::SetUserAgentOverride> {
        self.session.send::<commands::SetUserAgentOverride>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Network.command.streamResourceContent。
    ///
    ///协议原文：
    ///
    ///<pre>Enables streaming of the response for the given requestId.
    ///If enabled, the dataReceived event contains the data that was received during streaming.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::network::commands::StreamResourceContentParams) -> ydp_core::Result<()> {
    ///let response = session.network().stream_resource_content(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn stream_resource_content(
        &self,
        params: commands::StreamResourceContentParams,
    ) -> ydp_core::Request<commands::StreamResourceContent> {
        self.session.send::<commands::StreamResourceContent>(params)
    }
}
