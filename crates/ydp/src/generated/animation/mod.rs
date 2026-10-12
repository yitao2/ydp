///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：Animation。此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Animation.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Disables animation domain notifications.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::animation::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.animation().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Animation.command.enable。
    ///
    ///协议原文：
    ///
    ///<pre>Enables animation domain notifications.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::animation::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.animation().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Animation.command.getCurrentTime。
    ///
    ///协议原文：
    ///
    ///<pre>Returns the current time of the an animation.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::animation::commands::GetCurrentTimeParams) -> ydp_core::Result<()> {
    ///let response = session.animation().get_current_time(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_current_time(
        &self,
        params: commands::GetCurrentTimeParams,
    ) -> ydp_core::Request<commands::GetCurrentTime> {
        self.session.send::<commands::GetCurrentTime>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Animation.command.getPlaybackRate。
    ///
    ///协议原文：
    ///
    ///<pre>Gets the playback rate of the document timeline.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::animation::commands::GetPlaybackRateParams) -> ydp_core::Result<()> {
    ///let response = session.animation().get_playback_rate(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_playback_rate(
        &self,
        params: commands::GetPlaybackRateParams,
    ) -> ydp_core::Request<commands::GetPlaybackRate> {
        self.session.send::<commands::GetPlaybackRate>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Animation.command.releaseAnimations。
    ///
    ///协议原文：
    ///
    ///<pre>Releases a set of animations to no longer be manipulated.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::animation::commands::ReleaseAnimationsParams) -> ydp_core::Result<()> {
    ///let response = session.animation().release_animations(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn release_animations(
        &self,
        params: commands::ReleaseAnimationsParams,
    ) -> ydp_core::Request<commands::ReleaseAnimations> {
        self.session.send::<commands::ReleaseAnimations>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Animation.command.resolveAnimation。
    ///
    ///协议原文：
    ///
    ///<pre>Gets the remote object of the Animation.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::animation::commands::ResolveAnimationParams) -> ydp_core::Result<()> {
    ///let response = session.animation().resolve_animation(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn resolve_animation(
        &self,
        params: commands::ResolveAnimationParams,
    ) -> ydp_core::Request<commands::ResolveAnimation> {
        self.session.send::<commands::ResolveAnimation>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Animation.command.seekAnimations。
    ///
    ///协议原文：
    ///
    ///<pre>Seek a set of animations to a particular time within each animation.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::animation::commands::SeekAnimationsParams) -> ydp_core::Result<()> {
    ///let response = session.animation().seek_animations(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn seek_animations(
        &self,
        params: commands::SeekAnimationsParams,
    ) -> ydp_core::Request<commands::SeekAnimations> {
        self.session.send::<commands::SeekAnimations>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Animation.command.setPaused。
    ///
    ///协议原文：
    ///
    ///<pre>Sets the paused state of a set of animations.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::animation::commands::SetPausedParams) -> ydp_core::Result<()> {
    ///let response = session.animation().set_paused(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_paused(
        &self,
        params: commands::SetPausedParams,
    ) -> ydp_core::Request<commands::SetPaused> {
        self.session.send::<commands::SetPaused>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Animation.command.setPlaybackRate。
    ///
    ///协议原文：
    ///
    ///<pre>Sets the playback rate of the document timeline.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::animation::commands::SetPlaybackRateParams) -> ydp_core::Result<()> {
    ///let response = session.animation().set_playback_rate(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_playback_rate(
        &self,
        params: commands::SetPlaybackRateParams,
    ) -> ydp_core::Request<commands::SetPlaybackRate> {
        self.session.send::<commands::SetPlaybackRate>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Animation.command.setTiming。
    ///
    ///协议原文：
    ///
    ///<pre>Sets the timing of an animation node.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::animation::commands::SetTimingParams) -> ydp_core::Result<()> {
    ///let response = session.animation().set_timing(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_timing(
        &self,
        params: commands::SetTimingParams,
    ) -> ydp_core::Request<commands::SetTiming> {
        self.session.send::<commands::SetTiming>(params)
    }
}
