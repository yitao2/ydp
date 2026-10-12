///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：Target。
///
///协议原文：
///
///<pre>Supports additional targets discovery and allows to attach to them.</pre>
///
///
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Target.command.activateTarget。
    ///
    ///协议原文：
    ///
    ///<pre>Activates (focuses) the target.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::target::commands::ActivateTargetParams) -> ydp_core::Result<()> {
    ///let response = session.target().activate_target(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn activate_target(
        &self,
        params: commands::ActivateTargetParams,
    ) -> ydp_core::Request<commands::ActivateTarget> {
        self.session.send::<commands::ActivateTarget>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Target.command.attachToBrowserTarget。
    ///
    ///协议原文：
    ///
    ///<pre>Attaches to the browser target, only uses flat sessionId mode.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::target::commands::AttachToBrowserTargetParams) -> ydp_core::Result<()> {
    ///let response = session.target().attach_to_browser_target(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn attach_to_browser_target(
        &self,
        params: commands::AttachToBrowserTargetParams,
    ) -> ydp_core::Request<commands::AttachToBrowserTarget> {
        self.session.send::<commands::AttachToBrowserTarget>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Target.command.attachToTarget。
    ///
    ///协议原文：
    ///
    ///<pre>Attaches to the target with given id.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::target::commands::AttachToTargetParams) -> ydp_core::Result<()> {
    ///let response = session.target().attach_to_target(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn attach_to_target(
        &self,
        params: commands::AttachToTargetParams,
    ) -> ydp_core::Request<commands::AttachToTarget> {
        self.session.send::<commands::AttachToTarget>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Target.command.autoAttachRelated。
    ///
    ///协议原文：
    ///
    ///<pre>Adds the specified target to the list of targets that will be monitored for any related target
    ///creation (such as child frames, child workers and new versions of service worker) and reported
    ///through `attachedToTarget`. The specified target is also auto-attached.
    ///This cancels the effect of any previous `setAutoAttach` and is also cancelled by subsequent
    ///`setAutoAttach`. Only available at the Browser target.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::target::commands::AutoAttachRelatedParams) -> ydp_core::Result<()> {
    ///let response = session.target().auto_attach_related(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn auto_attach_related(
        &self,
        params: commands::AutoAttachRelatedParams,
    ) -> ydp_core::Request<commands::AutoAttachRelated> {
        self.session.send::<commands::AutoAttachRelated>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Target.command.closeTarget。
    ///
    ///协议原文：
    ///
    ///<pre>Closes the target. If the target is a page that gets closed too.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::target::commands::CloseTargetParams) -> ydp_core::Result<()> {
    ///let response = session.target().close_target(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn close_target(
        &self,
        params: commands::CloseTargetParams,
    ) -> ydp_core::Request<commands::CloseTarget> {
        self.session.send::<commands::CloseTarget>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Target.command.createBrowserContext。
    ///
    ///协议原文：
    ///
    ///<pre>Creates a new empty BrowserContext. Similar to an incognito profile but you can have more than
    ///one.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::target::commands::CreateBrowserContextParams) -> ydp_core::Result<()> {
    ///let response = session.target().create_browser_context(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn create_browser_context(
        &self,
        params: commands::CreateBrowserContextParams,
    ) -> ydp_core::Request<commands::CreateBrowserContext> {
        self.session.send::<commands::CreateBrowserContext>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Target.command.createTarget。
    ///
    ///协议原文：
    ///
    ///<pre>Creates a new page.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::target::commands::CreateTargetParams) -> ydp_core::Result<()> {
    ///let response = session.target().create_target(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn create_target(
        &self,
        params: commands::CreateTargetParams,
    ) -> ydp_core::Request<commands::CreateTarget> {
        self.session.send::<commands::CreateTarget>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Target.command.detachFromTarget。
    ///
    ///协议原文：
    ///
    ///<pre>Detaches session with given id.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::target::commands::DetachFromTargetParams) -> ydp_core::Result<()> {
    ///let response = session.target().detach_from_target(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn detach_from_target(
        &self,
        params: commands::DetachFromTargetParams,
    ) -> ydp_core::Request<commands::DetachFromTarget> {
        self.session.send::<commands::DetachFromTarget>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Target.command.disposeBrowserContext。
    ///
    ///协议原文：
    ///
    ///<pre>Deletes a BrowserContext. All the belonging pages will be closed without calling their
    ///beforeunload hooks.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::target::commands::DisposeBrowserContextParams) -> ydp_core::Result<()> {
    ///let response = session.target().dispose_browser_context(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn dispose_browser_context(
        &self,
        params: commands::DisposeBrowserContextParams,
    ) -> ydp_core::Request<commands::DisposeBrowserContext> {
        self.session.send::<commands::DisposeBrowserContext>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Target.command.exposeDevToolsProtocol。
    ///
    ///协议原文：
    ///
    ///<pre>Inject object to the target's main frame that provides a communication
    ///channel with browser target.
    ///
    ///Injected object will be available as `window[bindingName]`.
    ///
    ///The object has the following API:
    ///- `binding.send(json)` - a method to send messages over the remote debugging protocol
    ///- `binding.onmessage = json =&gt; handleMessage(json)` - a callback that will be called for the protocol notifications and command responses.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::target::commands::ExposeDevToolsProtocolParams) -> ydp_core::Result<()> {
    ///let response = session.target().expose_dev_tools_protocol(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn expose_dev_tools_protocol(
        &self,
        params: commands::ExposeDevToolsProtocolParams,
    ) -> ydp_core::Request<commands::ExposeDevToolsProtocol> {
        self.session
            .send::<commands::ExposeDevToolsProtocol>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Target.command.getBrowserContexts。
    ///
    ///协议原文：
    ///
    ///<pre>Returns all browser contexts created with `Target.createBrowserContext` method.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::target::commands::GetBrowserContextsParams) -> ydp_core::Result<()> {
    ///let response = session.target().get_browser_contexts(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_browser_contexts(
        &self,
        params: commands::GetBrowserContextsParams,
    ) -> ydp_core::Request<commands::GetBrowserContexts> {
        self.session.send::<commands::GetBrowserContexts>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Target.command.getDevToolsTarget。
    ///
    ///协议原文：
    ///
    ///<pre>Gets the targetId of the DevTools page target opened for the given target
    ///(if any).</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::target::commands::GetDevToolsTargetParams) -> ydp_core::Result<()> {
    ///let response = session.target().get_dev_tools_target(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_dev_tools_target(
        &self,
        params: commands::GetDevToolsTargetParams,
    ) -> ydp_core::Request<commands::GetDevToolsTarget> {
        self.session.send::<commands::GetDevToolsTarget>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Target.command.getTargetInfo。
    ///
    ///协议原文：
    ///
    ///<pre>Returns information about a target.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::target::commands::GetTargetInfoParams) -> ydp_core::Result<()> {
    ///let response = session.target().get_target_info(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_target_info(
        &self,
        params: commands::GetTargetInfoParams,
    ) -> ydp_core::Request<commands::GetTargetInfo> {
        self.session.send::<commands::GetTargetInfo>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Target.command.getTargets。
    ///
    ///协议原文：
    ///
    ///<pre>Retrieves a list of available targets.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::target::commands::GetTargetsParams) -> ydp_core::Result<()> {
    ///let response = session.target().get_targets(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_targets(
        &self,
        params: commands::GetTargetsParams,
    ) -> ydp_core::Request<commands::GetTargets> {
        self.session.send::<commands::GetTargets>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Target.command.openDevTools。
    ///
    ///协议原文：
    ///
    ///<pre>Opens a DevTools window for the target.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::target::commands::OpenDevToolsParams) -> ydp_core::Result<()> {
    ///let response = session.target().open_dev_tools(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn open_dev_tools(
        &self,
        params: commands::OpenDevToolsParams,
    ) -> ydp_core::Request<commands::OpenDevTools> {
        self.session.send::<commands::OpenDevTools>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Target.command.sendMessageToTarget。
    ///
    ///协议原文：
    ///
    ///<pre>Sends protocol message over session with given id.
    ///Consider using flat mode instead; see commands attachToTarget, setAutoAttach,
    ///and crbug.com/991325.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::target::commands::SendMessageToTargetParams) -> ydp_core::Result<()> {
    ///let response = session.target().send_message_to_target(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn send_message_to_target(
        &self,
        params: commands::SendMessageToTargetParams,
    ) -> ydp_core::Request<commands::SendMessageToTarget> {
        self.session.send::<commands::SendMessageToTarget>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Target.command.setAutoAttach。
    ///
    ///协议原文：
    ///
    ///<pre>Controls whether to automatically attach to new targets which are considered
    ///to be directly related to this one (for example, iframes or workers).
    ///When turned on, attaches to all existing related targets as well. When turned off,
    ///automatically detaches from all currently attached targets.
    ///This also clears all targets added by `autoAttachRelated` from the list of targets to watch
    ///for creation of related targets.
    ///You might want to call this recursively for auto-attached targets to attach
    ///to all available targets.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::target::commands::SetAutoAttachParams) -> ydp_core::Result<()> {
    ///let response = session.target().set_auto_attach(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_auto_attach(
        &self,
        params: commands::SetAutoAttachParams,
    ) -> ydp_core::Request<commands::SetAutoAttach> {
        self.session.send::<commands::SetAutoAttach>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Target.command.setDiscoverTargets。
    ///
    ///协议原文：
    ///
    ///<pre>Controls whether to discover available targets and notify via
    ///`targetCreated/targetInfoChanged/targetDestroyed` events.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::target::commands::SetDiscoverTargetsParams) -> ydp_core::Result<()> {
    ///let response = session.target().set_discover_targets(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_discover_targets(
        &self,
        params: commands::SetDiscoverTargetsParams,
    ) -> ydp_core::Request<commands::SetDiscoverTargets> {
        self.session.send::<commands::SetDiscoverTargets>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Target.command.setRemoteLocations。
    ///
    ///协议原文：
    ///
    ///<pre>Enables target discovery for the specified locations, when `setDiscoverTargets` was set to
    ///`true`.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::target::commands::SetRemoteLocationsParams) -> ydp_core::Result<()> {
    ///let response = session.target().set_remote_locations(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_remote_locations(
        &self,
        params: commands::SetRemoteLocationsParams,
    ) -> ydp_core::Request<commands::SetRemoteLocations> {
        self.session.send::<commands::SetRemoteLocations>(params)
    }
}
