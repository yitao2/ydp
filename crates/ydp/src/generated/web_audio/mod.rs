///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：WebAudio。
///
///协议原文：
///
///<pre>This domain allows inspection of Web Audio API.
///https://webaudio.github.io/web-audio-api/</pre>
///
///此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：WebAudio.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Disables the WebAudio domain.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::web_audio::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.web_audio().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：WebAudio.command.enable。
    ///
    ///协议原文：
    ///
    ///<pre>Enables the WebAudio domain and starts sending context lifetime events.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::web_audio::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.web_audio().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：WebAudio.command.getRealtimeData。
    ///
    ///协议原文：
    ///
    ///<pre>Fetch the realtime data from the registered contexts.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::web_audio::commands::GetRealtimeDataParams) -> ydp_core::Result<()> {
    ///let response = session.web_audio().get_realtime_data(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_realtime_data(
        &self,
        params: commands::GetRealtimeDataParams,
    ) -> ydp_core::Request<commands::GetRealtimeData> {
        self.session.send::<commands::GetRealtimeData>(params)
    }
}
