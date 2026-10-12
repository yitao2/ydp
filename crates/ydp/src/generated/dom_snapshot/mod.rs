///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：DOMSnapshot。
///
///协议原文：
///
///<pre>This domain facilitates obtaining document snapshots with DOM, layout, and style information.</pre>
///
///此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOMSnapshot.command.captureSnapshot。
    ///
    ///协议原文：
    ///
    ///<pre>Returns a document snapshot, including the full DOM tree of the root node (including iframes,
    ///template contents, and imported documents) in a flattened array, as well as layout and
    ///white-listed computed style information for the nodes. Shadow DOM in the returned DOM tree is
    ///flattened.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom_snapshot::commands::CaptureSnapshotParams) -> ydp_core::Result<()> {
    ///let response = session.dom_snapshot().capture_snapshot(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn capture_snapshot(
        &self,
        params: commands::CaptureSnapshotParams,
    ) -> ydp_core::Request<commands::CaptureSnapshot> {
        self.session.send::<commands::CaptureSnapshot>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOMSnapshot.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Disables DOM snapshot agent for the given page.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom_snapshot::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.dom_snapshot().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOMSnapshot.command.enable。
    ///
    ///协议原文：
    ///
    ///<pre>Enables DOM snapshot agent for the given page.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom_snapshot::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.dom_snapshot().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOMSnapshot.command.getSnapshot。
    ///
    ///协议原文：
    ///
    ///<pre>Returns a document snapshot, including the full DOM tree of the root node (including iframes,
    ///template contents, and imported documents) in a flattened array, as well as layout and
    ///white-listed computed style information for the nodes. Shadow DOM in the returned DOM tree is
    ///flattened.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom_snapshot::commands::GetSnapshotParams) -> ydp_core::Result<()> {
    ///let response = session.dom_snapshot().get_snapshot(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_snapshot(
        &self,
        params: commands::GetSnapshotParams,
    ) -> ydp_core::Request<commands::GetSnapshot> {
        self.session.send::<commands::GetSnapshot>(params)
    }
}
