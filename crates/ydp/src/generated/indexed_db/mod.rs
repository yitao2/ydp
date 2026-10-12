///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：IndexedDB。此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：IndexedDB.command.clearObjectStore。
    ///
    ///协议原文：
    ///
    ///<pre>Clears all entries from an object store.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::indexed_db::commands::ClearObjectStoreParams) -> ydp_core::Result<()> {
    ///let response = session.indexed_db().clear_object_store(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn clear_object_store(
        &self,
        params: commands::ClearObjectStoreParams,
    ) -> ydp_core::Request<commands::ClearObjectStore> {
        self.session.send::<commands::ClearObjectStore>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：IndexedDB.command.deleteDatabase。
    ///
    ///协议原文：
    ///
    ///<pre>Deletes a database.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::indexed_db::commands::DeleteDatabaseParams) -> ydp_core::Result<()> {
    ///let response = session.indexed_db().delete_database(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn delete_database(
        &self,
        params: commands::DeleteDatabaseParams,
    ) -> ydp_core::Request<commands::DeleteDatabase> {
        self.session.send::<commands::DeleteDatabase>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：IndexedDB.command.deleteObjectStoreEntries。
    ///
    ///协议原文：
    ///
    ///<pre>Delete a range of entries from an object store</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::indexed_db::commands::DeleteObjectStoreEntriesParams) -> ydp_core::Result<()> {
    ///let response = session.indexed_db().delete_object_store_entries(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn delete_object_store_entries(
        &self,
        params: commands::DeleteObjectStoreEntriesParams,
    ) -> ydp_core::Request<commands::DeleteObjectStoreEntries> {
        self.session
            .send::<commands::DeleteObjectStoreEntries>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：IndexedDB.command.disable。
    ///
    ///协议原文：
    ///
    ///<pre>Disables events from backend.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::indexed_db::commands::DisableParams) -> ydp_core::Result<()> {
    ///let response = session.indexed_db().disable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn disable(&self, params: commands::DisableParams) -> ydp_core::Request<commands::Disable> {
        self.session.send::<commands::Disable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：IndexedDB.command.enable。
    ///
    ///协议原文：
    ///
    ///<pre>Enables events from backend.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::indexed_db::commands::EnableParams) -> ydp_core::Result<()> {
    ///let response = session.indexed_db().enable(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn enable(&self, params: commands::EnableParams) -> ydp_core::Request<commands::Enable> {
        self.session.send::<commands::Enable>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：IndexedDB.command.getMetadata。
    ///
    ///协议原文：
    ///
    ///<pre>Gets metadata of an object store.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::indexed_db::commands::GetMetadataParams) -> ydp_core::Result<()> {
    ///let response = session.indexed_db().get_metadata(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_metadata(
        &self,
        params: commands::GetMetadataParams,
    ) -> ydp_core::Request<commands::GetMetadata> {
        self.session.send::<commands::GetMetadata>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：IndexedDB.command.requestData。
    ///
    ///协议原文：
    ///
    ///<pre>Requests data from object store or index.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::indexed_db::commands::RequestDataParams) -> ydp_core::Result<()> {
    ///let response = session.indexed_db().request_data(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn request_data(
        &self,
        params: commands::RequestDataParams,
    ) -> ydp_core::Request<commands::RequestData> {
        self.session.send::<commands::RequestData>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：IndexedDB.command.requestDatabase。
    ///
    ///协议原文：
    ///
    ///<pre>Requests database with given name in given frame.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::indexed_db::commands::RequestDatabaseParams) -> ydp_core::Result<()> {
    ///let response = session.indexed_db().request_database(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn request_database(
        &self,
        params: commands::RequestDatabaseParams,
    ) -> ydp_core::Request<commands::RequestDatabase> {
        self.session.send::<commands::RequestDatabase>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：IndexedDB.command.requestDatabaseNames。
    ///
    ///协议原文：
    ///
    ///<pre>Requests database names for given security origin.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::indexed_db::commands::RequestDatabaseNamesParams) -> ydp_core::Result<()> {
    ///let response = session.indexed_db().request_database_names(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn request_database_names(
        &self,
        params: commands::RequestDatabaseNamesParams,
    ) -> ydp_core::Request<commands::RequestDatabaseNames> {
        self.session.send::<commands::RequestDatabaseNames>(params)
    }
}
