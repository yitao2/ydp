///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：ServiceWorker。此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：ServiceWorker.command.deliverPushMessage。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::service_worker::commands::DeliverPushMessageParams) -> ydp_core::Result<()> {
    ///let response = session.service_worker().deliver_push_message(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn deliver_push_message(
        &self,
        params: commands::DeliverPushMessageParams,
    ) -> ydp_core::Request<commands::DeliverPushMessage> {
        self.session.send::<commands::DeliverPushMessage>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：ServiceWorker.command.disable。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::service_worker::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.service_worker().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：ServiceWorker.command.dispatchPeriodicSyncEvent。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::service_worker::commands::DispatchPeriodicSyncEventParams) -> ydp_core::Result<()> {
    ///let response = session.service_worker().dispatch_periodic_sync_event(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn dispatch_periodic_sync_event(
        &self,
        params: commands::DispatchPeriodicSyncEventParams,
    ) -> ydp_core::Request<commands::DispatchPeriodicSyncEvent> {
        self.session
            .send::<commands::DispatchPeriodicSyncEvent>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：ServiceWorker.command.dispatchSyncEvent。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::service_worker::commands::DispatchSyncEventParams) -> ydp_core::Result<()> {
    ///let response = session.service_worker().dispatch_sync_event(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn dispatch_sync_event(
        &self,
        params: commands::DispatchSyncEventParams,
    ) -> ydp_core::Request<commands::DispatchSyncEvent> {
        self.session.send::<commands::DispatchSyncEvent>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：ServiceWorker.command.enable。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::service_worker::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.service_worker().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：ServiceWorker.command.setForceUpdateOnPageLoad。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::service_worker::commands::SetForceUpdateOnPageLoadParams) -> ydp_core::Result<()> {
    ///let response = session.service_worker().set_force_update_on_page_load(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_force_update_on_page_load(
        &self,
        params: commands::SetForceUpdateOnPageLoadParams,
    ) -> ydp_core::Request<commands::SetForceUpdateOnPageLoad> {
        self.session
            .send::<commands::SetForceUpdateOnPageLoad>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：ServiceWorker.command.skipWaiting。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::service_worker::commands::SkipWaitingParams) -> ydp_core::Result<()> {
    ///let response = session.service_worker().skip_waiting(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn skip_waiting(
        &self,
        params: commands::SkipWaitingParams,
    ) -> ydp_core::Request<commands::SkipWaiting> {
        self.session.send::<commands::SkipWaiting>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：ServiceWorker.command.startWorker。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::service_worker::commands::StartWorkerParams) -> ydp_core::Result<()> {
    ///let response = session.service_worker().start_worker(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn start_worker(
        &self,
        params: commands::StartWorkerParams,
    ) -> ydp_core::Request<commands::StartWorker> {
        self.session.send::<commands::StartWorker>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：ServiceWorker.command.stopAllWorkers。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::service_worker::commands::StopAllWorkersParams) -> ydp_core::Result<()> {
    ///let response = session.service_worker().stop_all_workers(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn stop_all_workers(
        &self,
        params: commands::StopAllWorkersParams,
    ) -> ydp_core::Request<commands::StopAllWorkers> {
        self.session.send::<commands::StopAllWorkers>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：ServiceWorker.command.stopWorker。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::service_worker::commands::StopWorkerParams) -> ydp_core::Result<()> {
    ///let response = session.service_worker().stop_worker(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn stop_worker(
        &self,
        params: commands::StopWorkerParams,
    ) -> ydp_core::Request<commands::StopWorker> {
        self.session.send::<commands::StopWorker>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：ServiceWorker.command.unregister。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::service_worker::commands::UnregisterParams) -> ydp_core::Result<()> {
    ///let response = session.service_worker().unregister(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn unregister(
        &self,
        params: commands::UnregisterParams,
    ) -> ydp_core::Request<commands::Unregister> {
        self.session.send::<commands::Unregister>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：ServiceWorker.command.updateRegistration。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::service_worker::commands::UpdateRegistrationParams) -> ydp_core::Result<()> {
    ///let response = session.service_worker().update_registration(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn update_registration(
        &self,
        params: commands::UpdateRegistrationParams,
    ) -> ydp_core::Request<commands::UpdateRegistration> {
        self.session.send::<commands::UpdateRegistration>(params)
    }
}
