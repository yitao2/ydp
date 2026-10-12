///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：SmartCardEmulation。此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：SmartCardEmulation.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Disables the |SmartCardEmulation| domain.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::smart_card_emulation::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.smart_card_emulation().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：SmartCardEmulation.command.enable。
    ///
    ///协议原文：
    ///
    ///<pre>Enables the |SmartCardEmulation| domain.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::smart_card_emulation::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.smart_card_emulation().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：SmartCardEmulation.command.reportBeginTransactionResult。
    ///
    ///协议原文：
    ///
    ///<pre>Reports the result of a |SCardBeginTransaction| call.
    ///On success, this creates a new transaction object.
    ///
    ///This maps to:
    ///PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gaddb835dce01a0da1d6ca02d33ee7d861
    ///Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardbegintransaction</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::smart_card_emulation::commands::ReportBeginTransactionResultParams) -> ydp_core::Result<()> {
    ///let response = session.smart_card_emulation().report_begin_transaction_result(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn report_begin_transaction_result(
        &self,
        params: commands::ReportBeginTransactionResultParams,
    ) -> ydp_core::Request<commands::ReportBeginTransactionResult> {
        self.session
            .send::<commands::ReportBeginTransactionResult>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：SmartCardEmulation.command.reportConnectResult。
    ///
    ///协议原文：
    ///
    ///<pre>Reports the successful result of a |SCardConnect| call.
    ///
    ///This maps to:
    ///PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga4e515829752e0a8dbc4d630696a8d6a5
    ///Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardconnecta</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::smart_card_emulation::commands::ReportConnectResultParams) -> ydp_core::Result<()> {
    ///let response = session.smart_card_emulation().report_connect_result(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn report_connect_result(
        &self,
        params: commands::ReportConnectResultParams,
    ) -> ydp_core::Request<commands::ReportConnectResult> {
        self.session.send::<commands::ReportConnectResult>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：SmartCardEmulation.command.reportDataResult。
    ///
    ///协议原文：
    ///
    ///<pre>Reports the successful result of a call that sends back data on success.
    ///Used for |SCardTransmit|, |SCardControl|, and |SCardGetAttrib|.
    ///
    ///This maps to:
    ///1. SCardTransmit
    ///   PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga9a2d77242a271310269065e64633ab99
    ///   Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardtransmit
    ///
    ///2. SCardControl
    ///   PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gac3454d4657110fd7f753b2d3d8f4e32f
    ///   Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardcontrol
    ///
    ///3. SCardGetAttrib
    ///   PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gaacfec51917255b7a25b94c5104961602
    ///   Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardgetattrib</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::smart_card_emulation::commands::ReportDataResultParams) -> ydp_core::Result<()> {
    ///let response = session.smart_card_emulation().report_data_result(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn report_data_result(
        &self,
        params: commands::ReportDataResultParams,
    ) -> ydp_core::Request<commands::ReportDataResult> {
        self.session.send::<commands::ReportDataResult>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：SmartCardEmulation.command.reportError。
    ///
    ///协议原文：
    ///
    ///<pre>Reports an error result for the given request.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::smart_card_emulation::commands::ReportErrorParams) -> ydp_core::Result<()> {
    ///let response = session.smart_card_emulation().report_error(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn report_error(
        &self,
        params: commands::ReportErrorParams,
    ) -> ydp_core::Request<commands::ReportError> {
        self.session.send::<commands::ReportError>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：SmartCardEmulation.command.reportEstablishContextResult。
    ///
    ///协议原文：
    ///
    ///<pre>Reports the successful result of a |SCardEstablishContext| call.
    ///
    ///This maps to:
    ///PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gaa1b8970169fd4883a6dc4a8f43f19b67
    ///Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardestablishcontext</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::smart_card_emulation::commands::ReportEstablishContextResultParams) -> ydp_core::Result<()> {
    ///let response = session.smart_card_emulation().report_establish_context_result(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn report_establish_context_result(
        &self,
        params: commands::ReportEstablishContextResultParams,
    ) -> ydp_core::Request<commands::ReportEstablishContextResult> {
        self.session
            .send::<commands::ReportEstablishContextResult>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：SmartCardEmulation.command.reportGetStatusChangeResult。
    ///
    ///协议原文：
    ///
    ///<pre>Reports the successful result of a |SCardGetStatusChange| call.
    ///
    ///This maps to:
    ///PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga33247d5d1257d59e55647c3bb717db24
    ///Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardgetstatuschangea</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::smart_card_emulation::commands::ReportGetStatusChangeResultParams) -> ydp_core::Result<()> {
    ///let response = session.smart_card_emulation().report_get_status_change_result(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn report_get_status_change_result(
        &self,
        params: commands::ReportGetStatusChangeResultParams,
    ) -> ydp_core::Request<commands::ReportGetStatusChangeResult> {
        self.session
            .send::<commands::ReportGetStatusChangeResult>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：SmartCardEmulation.command.reportListReadersResult。
    ///
    ///协议原文：
    ///
    ///<pre>Reports the successful result of a |SCardListReaders| call.
    ///
    ///This maps to:
    ///PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga93b07815789b3cf2629d439ecf20f0d9
    ///Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardlistreadersa</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::smart_card_emulation::commands::ReportListReadersResultParams) -> ydp_core::Result<()> {
    ///let response = session.smart_card_emulation().report_list_readers_result(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn report_list_readers_result(
        &self,
        params: commands::ReportListReadersResultParams,
    ) -> ydp_core::Request<commands::ReportListReadersResult> {
        self.session
            .send::<commands::ReportListReadersResult>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：SmartCardEmulation.command.reportPlainResult。
    ///
    ///协议原文：
    ///
    ///<pre>Reports the successful result of a call that returns only a result code.
    ///Used for: |SCardCancel|, |SCardDisconnect|, |SCardSetAttrib|, |SCardEndTransaction|.
    ///
    ///This maps to:
    ///1. SCardCancel
    ///   PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gaacbbc0c6d6c0cbbeb4f4debf6fbeeee6
    ///   Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardcancel
    ///
    ///2. SCardDisconnect
    ///   PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga4be198045c73ec0deb79e66c0ca1738a
    ///   Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scarddisconnect
    ///
    ///3. SCardSetAttrib
    ///   PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga060f0038a4ddfd5dd2b8fadf3c3a2e4f
    ///   Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardsetattrib
    ///
    ///4. SCardEndTransaction
    ///   PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gae8742473b404363e5c587f570d7e2f3b
    ///   Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardendtransaction</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::smart_card_emulation::commands::ReportPlainResultParams) -> ydp_core::Result<()> {
    ///let response = session.smart_card_emulation().report_plain_result(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn report_plain_result(
        &self,
        params: commands::ReportPlainResultParams,
    ) -> ydp_core::Request<commands::ReportPlainResult> {
        self.session.send::<commands::ReportPlainResult>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：SmartCardEmulation.command.reportReleaseContextResult。
    ///
    ///协议原文：
    ///
    ///<pre>Reports the successful result of a |SCardReleaseContext| call.
    ///
    ///This maps to:
    ///PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga6aabcba7744c5c9419fdd6404f73a934
    ///Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardreleasecontext</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::smart_card_emulation::commands::ReportReleaseContextResultParams) -> ydp_core::Result<()> {
    ///let response = session.smart_card_emulation().report_release_context_result(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn report_release_context_result(
        &self,
        params: commands::ReportReleaseContextResultParams,
    ) -> ydp_core::Request<commands::ReportReleaseContextResult> {
        self.session
            .send::<commands::ReportReleaseContextResult>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：SmartCardEmulation.command.reportStatusResult。
    ///
    ///协议原文：
    ///
    ///<pre>Reports the successful result of a |SCardStatus| call.
    ///
    ///This maps to:
    ///PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gae49c3c894ad7ac12a5b896bde70d0382
    ///Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardstatusa</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::smart_card_emulation::commands::ReportStatusResultParams) -> ydp_core::Result<()> {
    ///let response = session.smart_card_emulation().report_status_result(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn report_status_result(
        &self,
        params: commands::ReportStatusResultParams,
    ) -> ydp_core::Request<commands::ReportStatusResult> {
        self.session.send::<commands::ReportStatusResult>(params)
    }
}
