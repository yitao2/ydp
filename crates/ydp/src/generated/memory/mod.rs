///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：Memory。此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Memory.command.forciblyPurgeJavaScriptMemory。
    ///
    ///协议原文：
    ///
    ///<pre>Simulate OomIntervention by purging V8 memory.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::memory::commands::ForciblyPurgeJavaScriptMemoryParams) -> ydp_core::Result<()> {
    ///let response = session.memory().forcibly_purge_java_script_memory(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn forcibly_purge_java_script_memory(
        &self,
        params: commands::ForciblyPurgeJavaScriptMemoryParams,
    ) -> ydp_core::Request<commands::ForciblyPurgeJavaScriptMemory> {
        self.session
            .send::<commands::ForciblyPurgeJavaScriptMemory>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Memory.command.getAllTimeSamplingProfile。
    ///
    ///协议原文：
    ///
    ///<pre>Retrieve native memory allocations profile
    ///collected since renderer process startup.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::memory::commands::GetAllTimeSamplingProfileParams) -> ydp_core::Result<()> {
    ///let response = session.memory().get_all_time_sampling_profile(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_all_time_sampling_profile(
        &self,
        params: commands::GetAllTimeSamplingProfileParams,
    ) -> ydp_core::Request<commands::GetAllTimeSamplingProfile> {
        self.session
            .send::<commands::GetAllTimeSamplingProfile>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Memory.command.getBrowserSamplingProfile。
    ///
    ///协议原文：
    ///
    ///<pre>Retrieve native memory allocations profile
    ///collected since browser process startup.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::memory::commands::GetBrowserSamplingProfileParams) -> ydp_core::Result<()> {
    ///let response = session.memory().get_browser_sampling_profile(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_browser_sampling_profile(
        &self,
        params: commands::GetBrowserSamplingProfileParams,
    ) -> ydp_core::Request<commands::GetBrowserSamplingProfile> {
        self.session
            .send::<commands::GetBrowserSamplingProfile>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Memory.command.getDOMCounters。
    ///
    ///协议原文：
    ///
    ///<pre>Retruns current DOM object counters.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::memory::commands::GetDomCountersParams) -> ydp_core::Result<()> {
    ///let response = session.memory().get_dom_counters(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_dom_counters(
        &self,
        params: commands::GetDomCountersParams,
    ) -> ydp_core::Request<commands::GetDomCounters> {
        self.session.send::<commands::GetDomCounters>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Memory.command.getDOMCountersForLeakDetection。
    ///
    ///协议原文：
    ///
    ///<pre>Retruns DOM object counters after preparing renderer for leak detection.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::memory::commands::GetDomCountersForLeakDetectionParams) -> ydp_core::Result<()> {
    ///let response = session.memory().get_dom_counters_for_leak_detection(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_dom_counters_for_leak_detection(
        &self,
        params: commands::GetDomCountersForLeakDetectionParams,
    ) -> ydp_core::Request<commands::GetDomCountersForLeakDetection> {
        self.session
            .send::<commands::GetDomCountersForLeakDetection>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Memory.command.getSamplingProfile。
    ///
    ///协议原文：
    ///
    ///<pre>Retrieve native memory allocations profile collected since last
    ///`startSampling` call.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::memory::commands::GetSamplingProfileParams) -> ydp_core::Result<()> {
    ///let response = session.memory().get_sampling_profile(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_sampling_profile(
        &self,
        params: commands::GetSamplingProfileParams,
    ) -> ydp_core::Request<commands::GetSamplingProfile> {
        self.session.send::<commands::GetSamplingProfile>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Memory.command.prepareForLeakDetection。
    ///
    ///协议原文：
    ///
    ///<pre>Prepares for leak detection by terminating workers, stopping spellcheckers,
    ///dropping non-essential internal caches, running garbage collections, etc.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::memory::commands::PrepareForLeakDetectionParams) -> ydp_core::Result<()> {
    ///let response = session.memory().prepare_for_leak_detection(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn prepare_for_leak_detection(
        &self,
        params: commands::PrepareForLeakDetectionParams,
    ) -> ydp_core::Request<commands::PrepareForLeakDetection> {
        self.session
            .send::<commands::PrepareForLeakDetection>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Memory.command.setPressureNotificationsSuppressed。
    ///
    ///协议原文：
    ///
    ///<pre>Enable/disable suppressing memory pressure notifications in all processes.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::memory::commands::SetPressureNotificationsSuppressedParams) -> ydp_core::Result<()> {
    ///let response = session.memory().set_pressure_notifications_suppressed(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_pressure_notifications_suppressed(
        &self,
        params: commands::SetPressureNotificationsSuppressedParams,
    ) -> ydp_core::Request<commands::SetPressureNotificationsSuppressed> {
        self.session
            .send::<commands::SetPressureNotificationsSuppressed>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Memory.command.simulatePressureNotification。
    ///
    ///协议原文：
    ///
    ///<pre>Simulate a memory pressure notification in all processes.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::memory::commands::SimulatePressureNotificationParams) -> ydp_core::Result<()> {
    ///let response = session.memory().simulate_pressure_notification(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn simulate_pressure_notification(
        &self,
        params: commands::SimulatePressureNotificationParams,
    ) -> ydp_core::Request<commands::SimulatePressureNotification> {
        self.session
            .send::<commands::SimulatePressureNotification>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Memory.command.startSampling。
    ///
    ///协议原文：
    ///
    ///<pre>Start collecting native memory profile.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::memory::commands::StartSamplingParams) -> ydp_core::Result<()> {
    ///let response = session.memory().start_sampling(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn start_sampling(
        &self,
        params: commands::StartSamplingParams,
    ) -> ydp_core::Request<commands::StartSampling> {
        self.session.send::<commands::StartSampling>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Memory.command.stopSampling。
    ///
    ///协议原文：
    ///
    ///<pre>Stop collecting native memory profile.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::memory::commands::StopSamplingParams) -> ydp_core::Result<()> {
    ///let response = session.memory().stop_sampling(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn stop_sampling(
        &self,
        params: commands::StopSamplingParams,
    ) -> ydp_core::Request<commands::StopSampling> {
        self.session.send::<commands::StopSampling>(params)
    }
}
