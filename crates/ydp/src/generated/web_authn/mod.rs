///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：WebAuthn。
///
///协议原文：
///
///<pre>This domain allows configuring virtual authenticators to test the WebAuthn
///API.</pre>
///
///此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：WebAuthn.command.addCredential。
    ///
    ///协议原文：
    ///
    ///<pre>Adds the credential to the specified authenticator.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::web_authn::commands::AddCredentialParams) -> ydp_core::Result<()> {
    ///let response = session.web_authn().add_credential(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn add_credential(
        &self,
        params: commands::AddCredentialParams,
    ) -> ydp_core::Request<commands::AddCredential> {
        self.session.send::<commands::AddCredential>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：WebAuthn.command.addVirtualAuthenticator。
    ///
    ///协议原文：
    ///
    ///<pre>Creates and adds a virtual authenticator.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::web_authn::commands::AddVirtualAuthenticatorParams) -> ydp_core::Result<()> {
    ///let response = session.web_authn().add_virtual_authenticator(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn add_virtual_authenticator(
        &self,
        params: commands::AddVirtualAuthenticatorParams,
    ) -> ydp_core::Request<commands::AddVirtualAuthenticator> {
        self.session
            .send::<commands::AddVirtualAuthenticator>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：WebAuthn.command.clearCredentials。
    ///
    ///协议原文：
    ///
    ///<pre>Clears all the credentials from the specified device.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::web_authn::commands::ClearCredentialsParams) -> ydp_core::Result<()> {
    ///let response = session.web_authn().clear_credentials(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn clear_credentials(
        &self,
        params: commands::ClearCredentialsParams,
    ) -> ydp_core::Request<commands::ClearCredentials> {
        self.session.send::<commands::ClearCredentials>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：WebAuthn.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Disable the WebAuthn domain.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::web_authn::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.web_authn().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：WebAuthn.command.enable。
    ///
    ///协议原文：
    ///
    ///<pre>Enable the WebAuthn domain and start intercepting credential storage and
    ///retrieval with a virtual authenticator.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::web_authn::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.web_authn().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：WebAuthn.command.getCredential。
    ///
    ///协议原文：
    ///
    ///<pre>Returns a single credential stored in the given virtual authenticator that
    ///matches the credential ID.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::web_authn::commands::GetCredentialParams) -> ydp_core::Result<()> {
    ///let response = session.web_authn().get_credential(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_credential(
        &self,
        params: commands::GetCredentialParams,
    ) -> ydp_core::Request<commands::GetCredential> {
        self.session.send::<commands::GetCredential>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：WebAuthn.command.getCredentials。
    ///
    ///协议原文：
    ///
    ///<pre>Returns all the credentials stored in the given virtual authenticator.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::web_authn::commands::GetCredentialsParams) -> ydp_core::Result<()> {
    ///let response = session.web_authn().get_credentials(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_credentials(
        &self,
        params: commands::GetCredentialsParams,
    ) -> ydp_core::Request<commands::GetCredentials> {
        self.session.send::<commands::GetCredentials>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：WebAuthn.command.removeCredential。
    ///
    ///协议原文：
    ///
    ///<pre>Removes a credential from the authenticator.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::web_authn::commands::RemoveCredentialParams) -> ydp_core::Result<()> {
    ///let response = session.web_authn().remove_credential(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn remove_credential(
        &self,
        params: commands::RemoveCredentialParams,
    ) -> ydp_core::Request<commands::RemoveCredential> {
        self.session.send::<commands::RemoveCredential>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：WebAuthn.command.removeVirtualAuthenticator。
    ///
    ///协议原文：
    ///
    ///<pre>Removes the given authenticator.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::web_authn::commands::RemoveVirtualAuthenticatorParams) -> ydp_core::Result<()> {
    ///let response = session.web_authn().remove_virtual_authenticator(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn remove_virtual_authenticator(
        &self,
        params: commands::RemoveVirtualAuthenticatorParams,
    ) -> ydp_core::Request<commands::RemoveVirtualAuthenticator> {
        self.session
            .send::<commands::RemoveVirtualAuthenticator>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：WebAuthn.command.setAutomaticPresenceSimulation。
    ///
    ///协议原文：
    ///
    ///<pre>Sets whether tests of user presence will succeed immediately (if true) or fail to resolve (if false) for an authenticator.
    ///The default is true.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::web_authn::commands::SetAutomaticPresenceSimulationParams) -> ydp_core::Result<()> {
    ///let response = session.web_authn().set_automatic_presence_simulation(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_automatic_presence_simulation(
        &self,
        params: commands::SetAutomaticPresenceSimulationParams,
    ) -> ydp_core::Request<commands::SetAutomaticPresenceSimulation> {
        self.session
            .send::<commands::SetAutomaticPresenceSimulation>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：WebAuthn.command.setCredentialProperties。
    ///
    ///协议原文：
    ///
    ///<pre>Allows setting credential properties.
    ///https://w3c.github.io/webauthn/#sctn-automation-set-credential-properties</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::web_authn::commands::SetCredentialPropertiesParams) -> ydp_core::Result<()> {
    ///let response = session.web_authn().set_credential_properties(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_credential_properties(
        &self,
        params: commands::SetCredentialPropertiesParams,
    ) -> ydp_core::Request<commands::SetCredentialProperties> {
        self.session
            .send::<commands::SetCredentialProperties>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：WebAuthn.command.setResponseOverrideBits。
    ///
    ///协议原文：
    ///
    ///<pre>Resets parameters isBogusSignature, isBadUV, isBadUP to false if they are not present.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::web_authn::commands::SetResponseOverrideBitsParams) -> ydp_core::Result<()> {
    ///let response = session.web_authn().set_response_override_bits(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_response_override_bits(
        &self,
        params: commands::SetResponseOverrideBitsParams,
    ) -> ydp_core::Request<commands::SetResponseOverrideBits> {
        self.session
            .send::<commands::SetResponseOverrideBits>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：WebAuthn.command.setUserVerified。
    ///
    ///协议原文：
    ///
    ///<pre>Sets whether User Verification succeeds or fails for an authenticator.
    ///The default is true.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::web_authn::commands::SetUserVerifiedParams) -> ydp_core::Result<()> {
    ///let response = session.web_authn().set_user_verified(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_user_verified(
        &self,
        params: commands::SetUserVerifiedParams,
    ) -> ydp_core::Request<commands::SetUserVerified> {
        self.session.send::<commands::SetUserVerified>(params)
    }
}
