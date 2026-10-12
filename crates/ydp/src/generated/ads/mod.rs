///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：Ads。
///
///协议原文：
///
///<pre>A domain for ad-related metrics and data.</pre>
///
///此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Ads.command.getAdMetrics。
    ///
    ///协议原文：
    ///
    ///<pre>Retrieves ad metrics for the current page.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::ads::commands::GetAdMetricsParams) -> ydp_core::Result<()> {
    ///let response = session.ads().get_ad_metrics(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_ad_metrics(
        &self,
        params: commands::GetAdMetricsParams,
    ) -> ydp_core::Request<commands::GetAdMetrics> {
        self.session.send::<commands::GetAdMetrics>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Ads.command.getAdScripts。
    ///
    ///协议原文：
    ///
    ///<pre>Retrieves ad scripts for the current page. To minimize payload size, this
    ///only returns the newly tracked ad scripts since the last call to
    ///getAdScripts (i.e., the delta).</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::ads::commands::GetAdScriptsParams) -> ydp_core::Result<()> {
    ///let response = session.ads().get_ad_scripts(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_ad_scripts(
        &self,
        params: commands::GetAdScriptsParams,
    ) -> ydp_core::Request<commands::GetAdScripts> {
        self.session.send::<commands::GetAdScripts>(params)
    }
}
