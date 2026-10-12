///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：LayerTree。此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：LayerTree.command.compositingReasons。
    ///
    ///协议原文：
    ///
    ///<pre>Provides the reasons why the given layer was composited.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::layer_tree::commands::CompositingReasonsParams) -> ydp_core::Result<()> {
    ///let response = session.layer_tree().compositing_reasons(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn compositing_reasons(
        &self,
        params: commands::CompositingReasonsParams,
    ) -> ydp_core::Request<commands::CompositingReasons> {
        self.session.send::<commands::CompositingReasons>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：LayerTree.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Disables compositing tree inspection.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::layer_tree::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.layer_tree().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：LayerTree.command.enable。
    ///
    ///协议原文：
    ///
    ///<pre>Enables compositing tree inspection.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::layer_tree::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.layer_tree().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：LayerTree.command.loadSnapshot。
    ///
    ///协议原文：
    ///
    ///<pre>Returns the snapshot identifier.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::layer_tree::commands::LoadSnapshotParams) -> ydp_core::Result<()> {
    ///let response = session.layer_tree().load_snapshot(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn load_snapshot(
        &self,
        params: commands::LoadSnapshotParams,
    ) -> ydp_core::Request<commands::LoadSnapshot> {
        self.session.send::<commands::LoadSnapshot>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：LayerTree.command.makeSnapshot。
    ///
    ///协议原文：
    ///
    ///<pre>Returns the layer snapshot identifier.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::layer_tree::commands::MakeSnapshotParams) -> ydp_core::Result<()> {
    ///let response = session.layer_tree().make_snapshot(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn make_snapshot(
        &self,
        params: commands::MakeSnapshotParams,
    ) -> ydp_core::Request<commands::MakeSnapshot> {
        self.session.send::<commands::MakeSnapshot>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：LayerTree.command.profileSnapshot。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::layer_tree::commands::ProfileSnapshotParams) -> ydp_core::Result<()> {
    ///let response = session.layer_tree().profile_snapshot(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn profile_snapshot(
        &self,
        params: commands::ProfileSnapshotParams,
    ) -> ydp_core::Request<commands::ProfileSnapshot> {
        self.session.send::<commands::ProfileSnapshot>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：LayerTree.command.releaseSnapshot。
    ///
    ///协议原文：
    ///
    ///<pre>Releases layer snapshot captured by the back-end.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::layer_tree::commands::ReleaseSnapshotParams) -> ydp_core::Result<()> {
    ///let response = session.layer_tree().release_snapshot(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn release_snapshot(
        &self,
        params: commands::ReleaseSnapshotParams,
    ) -> ydp_core::Request<commands::ReleaseSnapshot> {
        self.session.send::<commands::ReleaseSnapshot>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：LayerTree.command.replaySnapshot。
    ///
    ///协议原文：
    ///
    ///<pre>Replays the layer snapshot and returns the resulting bitmap.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::layer_tree::commands::ReplaySnapshotParams) -> ydp_core::Result<()> {
    ///let response = session.layer_tree().replay_snapshot(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn replay_snapshot(
        &self,
        params: commands::ReplaySnapshotParams,
    ) -> ydp_core::Request<commands::ReplaySnapshot> {
        self.session.send::<commands::ReplaySnapshot>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：LayerTree.command.snapshotCommandLog。
    ///
    ///协议原文：
    ///
    ///<pre>Replays the layer snapshot and returns canvas log.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::layer_tree::commands::SnapshotCommandLogParams) -> ydp_core::Result<()> {
    ///let response = session.layer_tree().snapshot_command_log(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn snapshot_command_log(
        &self,
        params: commands::SnapshotCommandLogParams,
    ) -> ydp_core::Request<commands::SnapshotCommandLog> {
        self.session.send::<commands::SnapshotCommandLog>(params)
    }
}
