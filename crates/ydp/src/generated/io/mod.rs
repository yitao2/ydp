///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：IO。
///
///协议原文：
///
///<pre>Input/Output operations for streams produced by DevTools.</pre>
///
///
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：IO.command.close。
    ///
    ///协议原文：
    ///
    ///<pre>Close the stream, discard any temporary backing storage.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::io::commands::CloseParams) -> ydp_core::Result<()> {
    ///let response = session.io().close(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn close(&self, params: commands::CloseParams) -> ydp_core::Request<commands::Close> {
        self.session.send::<commands::Close>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：IO.command.read。
    ///
    ///协议原文：
    ///
    ///<pre>Read a chunk of the stream</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::io::commands::ReadParams) -> ydp_core::Result<()> {
    ///let response = session.io().read(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn read(&self, params: commands::ReadParams) -> ydp_core::Request<commands::Read> {
        self.session.send::<commands::Read>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：IO.command.resolveBlob。
    ///
    ///协议原文：
    ///
    ///<pre>Return UUID of Blob object specified by a remote object id.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::io::commands::ResolveBlobParams) -> ydp_core::Result<()> {
    ///let response = session.io().resolve_blob(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn resolve_blob(
        &self,
        params: commands::ResolveBlobParams,
    ) -> ydp_core::Request<commands::ResolveBlob> {
        self.session.send::<commands::ResolveBlob>(params)
    }
}
