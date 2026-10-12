///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：HeadlessExperimental。
///
///协议原文：
///
///<pre>This domain provides experimental commands only supported in headless mode.</pre>
///
///此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：HeadlessExperimental.command.beginFrame。
    ///
    ///协议原文：
    ///
    ///<pre>Sends a BeginFrame to the target and returns when the frame was completed. Optionally captures a
    ///screenshot from the resulting frame. Requires that the target was created with enabled
    ///BeginFrameControl. Designed for use with --run-all-compositor-stages-before-draw, see also
    ///https://goo.gle/chrome-headless-rendering for more background.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::headless_experimental::commands::BeginFrameParams) -> ydp_core::Result<()> {
    ///let response = session.headless_experimental().begin_frame(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn begin_frame(
        &self,
        params: commands::BeginFrameParams,
    ) -> ydp_core::Request<commands::BeginFrame> {
        self.session.send::<commands::BeginFrame>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：HeadlessExperimental.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Disables headless events for the target.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::headless_experimental::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.headless_experimental().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：HeadlessExperimental.command.enable。
    ///
    ///协议原文：
    ///
    ///<pre>Enables headless events for the target.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::headless_experimental::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.headless_experimental().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
}
