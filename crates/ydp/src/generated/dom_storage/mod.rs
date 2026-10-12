///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：DOMStorage。
///
///协议原文：
///
///<pre>Query and modify DOM storage.</pre>
///
///此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOMStorage.command.clear。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom_storage::commands::ClearParams) -> ydp_core::Result<()> {
    ///let response = session.dom_storage().clear(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn clear(&self, params: commands::ClearParams) -> ydp_core::Request<commands::Clear> {
        self.session.send::<commands::Clear>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOMStorage.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Disables storage tracking, prevents storage events from being sent to the client.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom_storage::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.dom_storage().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOMStorage.command.enable。
    ///
    ///协议原文：
    ///
    ///<pre>Enables storage tracking, storage events will now be delivered to the client.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom_storage::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.dom_storage().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOMStorage.command.getDOMStorageItems。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom_storage::commands::GetDomStorageItemsParams) -> ydp_core::Result<()> {
    ///let response = session.dom_storage().get_dom_storage_items(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_dom_storage_items(
        &self,
        params: commands::GetDomStorageItemsParams,
    ) -> ydp_core::Request<commands::GetDomStorageItems> {
        self.session.send::<commands::GetDomStorageItems>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOMStorage.command.removeDOMStorageItem。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom_storage::commands::RemoveDomStorageItemParams) -> ydp_core::Result<()> {
    ///let response = session.dom_storage().remove_dom_storage_item(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn remove_dom_storage_item(
        &self,
        params: commands::RemoveDomStorageItemParams,
    ) -> ydp_core::Request<commands::RemoveDomStorageItem> {
        self.session.send::<commands::RemoveDomStorageItem>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：DOMStorage.command.setDOMStorageItem。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::dom_storage::commands::SetDomStorageItemParams) -> ydp_core::Result<()> {
    ///let response = session.dom_storage().set_dom_storage_item(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_dom_storage_item(
        &self,
        params: commands::SetDomStorageItemParams,
    ) -> ydp_core::Request<commands::SetDomStorageItem> {
        self.session.send::<commands::SetDomStorageItem>(params)
    }
}
