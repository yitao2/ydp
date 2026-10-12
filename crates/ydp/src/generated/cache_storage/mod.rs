///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：CacheStorage。此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CacheStorage.command.deleteCache。
    ///
    ///协议原文：
    ///
    ///<pre>Deletes a cache.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::cache_storage::commands::DeleteCacheParams) -> ydp_core::Result<()> {
    ///let response = session.cache_storage().delete_cache(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn delete_cache(
        &self,
        params: commands::DeleteCacheParams,
    ) -> ydp_core::Request<commands::DeleteCache> {
        self.session.send::<commands::DeleteCache>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CacheStorage.command.deleteEntry。
    ///
    ///协议原文：
    ///
    ///<pre>Deletes a cache entry.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::cache_storage::commands::DeleteEntryParams) -> ydp_core::Result<()> {
    ///let response = session.cache_storage().delete_entry(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn delete_entry(
        &self,
        params: commands::DeleteEntryParams,
    ) -> ydp_core::Request<commands::DeleteEntry> {
        self.session.send::<commands::DeleteEntry>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CacheStorage.command.requestCacheNames。
    ///
    ///协议原文：
    ///
    ///<pre>Requests cache names.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::cache_storage::commands::RequestCacheNamesParams) -> ydp_core::Result<()> {
    ///let response = session.cache_storage().request_cache_names(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn request_cache_names(
        &self,
        params: commands::RequestCacheNamesParams,
    ) -> ydp_core::Request<commands::RequestCacheNames> {
        self.session.send::<commands::RequestCacheNames>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CacheStorage.command.requestCachedResponse。
    ///
    ///协议原文：
    ///
    ///<pre>Fetches cache entry.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::cache_storage::commands::RequestCachedResponseParams) -> ydp_core::Result<()> {
    ///let response = session.cache_storage().request_cached_response(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn request_cached_response(
        &self,
        params: commands::RequestCachedResponseParams,
    ) -> ydp_core::Request<commands::RequestCachedResponse> {
        self.session.send::<commands::RequestCachedResponse>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：CacheStorage.command.requestEntries。
    ///
    ///协议原文：
    ///
    ///<pre>Requests data from cache.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::cache_storage::commands::RequestEntriesParams) -> ydp_core::Result<()> {
    ///let response = session.cache_storage().request_entries(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn request_entries(
        &self,
        params: commands::RequestEntriesParams,
    ) -> ydp_core::Request<commands::RequestEntries> {
        self.session.send::<commands::RequestEntries>(params)
    }
}
