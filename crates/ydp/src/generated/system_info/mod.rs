///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：SystemInfo。
///
///协议原文：
///
///<pre>The SystemInfo domain defines methods and events for querying low-level system information.</pre>
///
///此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：SystemInfo.command.getFeatureState。
    ///
    ///协议原文：
    ///
    ///<pre>Returns information about the feature state.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::system_info::commands::GetFeatureStateParams) -> ydp_core::Result<()> {
    ///let response = session.system_info().get_feature_state(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_feature_state(
        &self,
        params: commands::GetFeatureStateParams,
    ) -> ydp_core::Request<commands::GetFeatureState> {
        self.session.send::<commands::GetFeatureState>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：SystemInfo.command.getInfo。
    ///
    ///协议原文：
    ///
    ///<pre>Returns information about the system.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::system_info::commands::GetInfoParams) -> ydp_core::Result<()> {
    ///let response = session.system_info().get_info(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_info(
        &self,
        params: commands::GetInfoParams,
    ) -> ydp_core::Request<commands::GetInfo> {
        self.session.send::<commands::GetInfo>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：SystemInfo.command.getProcessInfo。
    ///
    ///协议原文：
    ///
    ///<pre>Returns information about all running processes.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::system_info::commands::GetProcessInfoParams) -> ydp_core::Result<()> {
    ///let response = session.system_info().get_process_info(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_process_info(
        &self,
        params: commands::GetProcessInfoParams,
    ) -> ydp_core::Request<commands::GetProcessInfo> {
        self.session.send::<commands::GetProcessInfo>(params)
    }
}
