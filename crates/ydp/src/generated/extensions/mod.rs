///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：Extensions。
///
///协议原文：
///
///<pre>Defines commands and events for browser extensions.</pre>
///
///此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Extensions.command.clearStorageItems。
    ///
    ///协议原文：
    ///
    ///<pre>Clears extension storage in the given `storageArea`.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::extensions::commands::ClearStorageItemsParams) -> ydp_core::Result<()> {
    ///let response = session.extensions().clear_storage_items(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn clear_storage_items(
        &self,
        params: commands::ClearStorageItemsParams,
    ) -> ydp_core::Request<commands::ClearStorageItems> {
        self.session.send::<commands::ClearStorageItems>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Extensions.command.getExtensions。
    ///
    ///协议原文：
    ///
    ///<pre>Gets a list of all unpacked extensions.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::extensions::commands::GetExtensionsParams) -> ydp_core::Result<()> {
    ///let response = session.extensions().get_extensions(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_extensions(
        &self,
        params: commands::GetExtensionsParams,
    ) -> ydp_core::Request<commands::GetExtensions> {
        self.session.send::<commands::GetExtensions>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Extensions.command.getStorageItems。
    ///
    ///协议原文：
    ///
    ///<pre>Gets data from extension storage in the given `storageArea`. If `keys` is
    ///specified, these are used to filter the result.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::extensions::commands::GetStorageItemsParams) -> ydp_core::Result<()> {
    ///let response = session.extensions().get_storage_items(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_storage_items(
        &self,
        params: commands::GetStorageItemsParams,
    ) -> ydp_core::Request<commands::GetStorageItems> {
        self.session.send::<commands::GetStorageItems>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Extensions.command.loadUnpacked。
    ///
    ///协议原文：
    ///
    ///<pre>Installs an unpacked extension from the filesystem similar to
    ///--load-extension CLI flags. Returns extension ID once the extension
    ///has been installed.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::extensions::commands::LoadUnpackedParams) -> ydp_core::Result<()> {
    ///let response = session.extensions().load_unpacked(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn load_unpacked(
        &self,
        params: commands::LoadUnpackedParams,
    ) -> ydp_core::Request<commands::LoadUnpacked> {
        self.session.send::<commands::LoadUnpacked>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Extensions.command.removeStorageItems。
    ///
    ///协议原文：
    ///
    ///<pre>Removes `keys` from extension storage in the given `storageArea`.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::extensions::commands::RemoveStorageItemsParams) -> ydp_core::Result<()> {
    ///let response = session.extensions().remove_storage_items(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn remove_storage_items(
        &self,
        params: commands::RemoveStorageItemsParams,
    ) -> ydp_core::Request<commands::RemoveStorageItems> {
        self.session.send::<commands::RemoveStorageItems>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Extensions.command.setStorageItems。
    ///
    ///协议原文：
    ///
    ///<pre>Sets `values` in extension storage in the given `storageArea`. The provided `values`
    ///will be merged with existing values in the storage area.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::extensions::commands::SetStorageItemsParams) -> ydp_core::Result<()> {
    ///let response = session.extensions().set_storage_items(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_storage_items(
        &self,
        params: commands::SetStorageItemsParams,
    ) -> ydp_core::Request<commands::SetStorageItems> {
        self.session.send::<commands::SetStorageItems>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Extensions.command.triggerAction。
    ///
    ///协议原文：
    ///
    ///<pre>Runs an extension default action.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::extensions::commands::TriggerActionParams) -> ydp_core::Result<()> {
    ///let response = session.extensions().trigger_action(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn trigger_action(
        &self,
        params: commands::TriggerActionParams,
    ) -> ydp_core::Request<commands::TriggerAction> {
        self.session.send::<commands::TriggerAction>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Extensions.command.uninstall。
    ///
    ///协议原文：
    ///
    ///<pre>Uninstalls an unpacked extension (others not supported) from the profile.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::extensions::commands::UninstallParams) -> ydp_core::Result<()> {
    ///let response = session.extensions().uninstall(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn uninstall(
        &self,
        params: commands::UninstallParams,
    ) -> ydp_core::Request<commands::Uninstall> {
        self.session.send::<commands::Uninstall>(params)
    }
}
