///命令 marker、完整参数与成功响应。
pub mod commands;
///事件 marker 和拥有型事件参数。
pub mod events;
///命名协议类型；不包含连接状态。
pub mod types;
///薄 Domain 视图，仅借用 Session；创建不发送数据。协议来源：Storage。此项为 experimental，端点可能不支持。
pub struct Domain<'a> {
    ///借用的通信路由，生命周期不超过 Session。
    pub(super) session: &'a ydp_core::Session,
}
impl<'a> Domain<'a> {
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Storage.command.clearCookies。
    ///
    ///协议原文：
    ///
    ///<pre>Clears cookies.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::storage::commands::ClearCookiesParams) -> ydp_core::Result<()> {
    ///let response = session.storage().clear_cookies(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn clear_cookies(
        &self,
        params: commands::ClearCookiesParams,
    ) -> ydp_core::Request<commands::ClearCookies> {
        self.session.send::<commands::ClearCookies>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Storage.command.clearDataForOrigin。
    ///
    ///协议原文：
    ///
    ///<pre>Clears storage for origin.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::storage::commands::ClearDataForOriginParams) -> ydp_core::Result<()> {
    ///let response = session.storage().clear_data_for_origin(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn clear_data_for_origin(
        &self,
        params: commands::ClearDataForOriginParams,
    ) -> ydp_core::Request<commands::ClearDataForOrigin> {
        self.session.send::<commands::ClearDataForOrigin>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Storage.command.clearDataForStorageKey。
    ///
    ///协议原文：
    ///
    ///<pre>Clears storage for storage key.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::storage::commands::ClearDataForStorageKeyParams) -> ydp_core::Result<()> {
    ///let response = session.storage().clear_data_for_storage_key(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn clear_data_for_storage_key(
        &self,
        params: commands::ClearDataForStorageKeyParams,
    ) -> ydp_core::Request<commands::ClearDataForStorageKey> {
        self.session
            .send::<commands::ClearDataForStorageKey>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Storage.command.clearTrustTokens。
    ///
    ///协议原文：
    ///
    ///<pre>Removes all Trust Tokens issued by the provided issuerOrigin.
    ///Leaves other stored data, including the issuer's Redemption Records, intact.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::storage::commands::ClearTrustTokensParams) -> ydp_core::Result<()> {
    ///let response = session.storage().clear_trust_tokens(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn clear_trust_tokens(
        &self,
        params: commands::ClearTrustTokensParams,
    ) -> ydp_core::Request<commands::ClearTrustTokens> {
        self.session.send::<commands::ClearTrustTokens>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Storage.command.deleteStorageBucket。
    ///
    ///协议原文：
    ///
    ///<pre>Deletes the Storage Bucket with the given storage key and bucket name.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::storage::commands::DeleteStorageBucketParams) -> ydp_core::Result<()> {
    ///let response = session.storage().delete_storage_bucket(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn delete_storage_bucket(
        &self,
        params: commands::DeleteStorageBucketParams,
    ) -> ydp_core::Request<commands::DeleteStorageBucket> {
        self.session.send::<commands::DeleteStorageBucket>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Storage.command.getCookies。
    ///
    ///协议原文：
    ///
    ///<pre>Returns all browser cookies.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::storage::commands::GetCookiesParams) -> ydp_core::Result<()> {
    ///let response = session.storage().get_cookies(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_cookies(
        &self,
        params: commands::GetCookiesParams,
    ) -> ydp_core::Request<commands::GetCookies> {
        self.session.send::<commands::GetCookies>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Storage.command.getRelatedWebsiteSets。
    ///
    ///协议原文：
    ///
    ///<pre>Returns the effective Related Website Sets in use by this profile for the browser
    ///session. The effective Related Website Sets will not change during a browser session.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::storage::commands::GetRelatedWebsiteSetsParams) -> ydp_core::Result<()> {
    ///let response = session.storage().get_related_website_sets(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_related_website_sets(
        &self,
        params: commands::GetRelatedWebsiteSetsParams,
    ) -> ydp_core::Request<commands::GetRelatedWebsiteSets> {
        self.session.send::<commands::GetRelatedWebsiteSets>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Storage.command.getStorageKey。
    ///
    ///协议原文：
    ///
    ///<pre>Returns storage key for the given frame. If no frame ID is provided,
    ///the storage key of the target executing this command is returned.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::storage::commands::GetStorageKeyParams) -> ydp_core::Result<()> {
    ///let response = session.storage().get_storage_key(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_storage_key(
        &self,
        params: commands::GetStorageKeyParams,
    ) -> ydp_core::Request<commands::GetStorageKey> {
        self.session.send::<commands::GetStorageKey>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Storage.command.getStorageKeyForFrame。
    ///
    ///协议原文：
    ///
    ///<pre>Returns a storage key given a frame id.
    ///Deprecated. Please use Storage.getStorageKey instead.</pre>
    ///
    ///此项已 deprecated，保留用于旧端点兼容。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::storage::commands::GetStorageKeyForFrameParams) -> ydp_core::Result<()> {
    ///let response = session.storage().get_storage_key_for_frame(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_storage_key_for_frame(
        &self,
        params: commands::GetStorageKeyForFrameParams,
    ) -> ydp_core::Request<commands::GetStorageKeyForFrame> {
        self.session.send::<commands::GetStorageKeyForFrame>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Storage.command.getTrustTokens。
    ///
    ///协议原文：
    ///
    ///<pre>Returns the number of stored Trust Tokens per issuer for the
    ///current browsing context.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::storage::commands::GetTrustTokensParams) -> ydp_core::Result<()> {
    ///let response = session.storage().get_trust_tokens(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_trust_tokens(
        &self,
        params: commands::GetTrustTokensParams,
    ) -> ydp_core::Request<commands::GetTrustTokens> {
        self.session.send::<commands::GetTrustTokens>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Storage.command.getUsageAndQuota。
    ///
    ///协议原文：
    ///
    ///<pre>Returns usage and quota in bytes.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::storage::commands::GetUsageAndQuotaParams) -> ydp_core::Result<()> {
    ///let response = session.storage().get_usage_and_quota(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn get_usage_and_quota(
        &self,
        params: commands::GetUsageAndQuotaParams,
    ) -> ydp_core::Request<commands::GetUsageAndQuota> {
        self.session.send::<commands::GetUsageAndQuota>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Storage.command.overrideQuotaForOrigin。
    ///
    ///协议原文：
    ///
    ///<pre>Override quota for the specified origin</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::storage::commands::OverrideQuotaForOriginParams) -> ydp_core::Result<()> {
    ///let response = session.storage().override_quota_for_origin(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn override_quota_for_origin(
        &self,
        params: commands::OverrideQuotaForOriginParams,
    ) -> ydp_core::Request<commands::OverrideQuotaForOrigin> {
        self.session
            .send::<commands::OverrideQuotaForOrigin>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Storage.command.runBounceTrackingMitigations。
    ///
    ///协议原文：
    ///
    ///<pre>Deletes state for sites identified as potential bounce trackers, immediately.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::storage::commands::RunBounceTrackingMitigationsParams) -> ydp_core::Result<()> {
    ///let response = session.storage().run_bounce_tracking_mitigations(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn run_bounce_tracking_mitigations(
        &self,
        params: commands::RunBounceTrackingMitigationsParams,
    ) -> ydp_core::Request<commands::RunBounceTrackingMitigations> {
        self.session
            .send::<commands::RunBounceTrackingMitigations>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Storage.command.setCookies。
    ///
    ///协议原文：
    ///
    ///<pre>Sets given cookies.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::storage::commands::SetCookiesParams) -> ydp_core::Result<()> {
    ///let response = session.storage().set_cookies(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_cookies(
        &self,
        params: commands::SetCookiesParams,
    ) -> ydp_core::Request<commands::SetCookies> {
        self.session.send::<commands::SetCookies>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Storage.command.setStorageBucketTracking。
    ///
    ///协议原文：
    ///
    ///<pre>Set tracking for a storage key's buckets.</pre>
    ///
    ///此项为 experimental，端点可能不支持。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::storage::commands::SetStorageBucketTrackingParams) -> ydp_core::Result<()> {
    ///let response = session.storage().set_storage_bucket_tracking(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn set_storage_bucket_tracking(
        &self,
        params: commands::SetStorageBucketTrackingParams,
    ) -> ydp_core::Request<commands::SetStorageBucketTracking> {
        self.session
            .send::<commands::SetStorageBucketTracking>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Storage.command.trackCacheStorageForOrigin。
    ///
    ///协议原文：
    ///
    ///<pre>Registers origin to be notified when an update occurs to its cache storage list.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::storage::commands::TrackCacheStorageForOriginParams) -> ydp_core::Result<()> {
    ///let response = session.storage().track_cache_storage_for_origin(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn track_cache_storage_for_origin(
        &self,
        params: commands::TrackCacheStorageForOriginParams,
    ) -> ydp_core::Request<commands::TrackCacheStorageForOrigin> {
        self.session
            .send::<commands::TrackCacheStorageForOrigin>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Storage.command.trackCacheStorageForStorageKey。
    ///
    ///协议原文：
    ///
    ///<pre>Registers storage key to be notified when an update occurs to its cache storage list.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::storage::commands::TrackCacheStorageForStorageKeyParams) -> ydp_core::Result<()> {
    ///let response = session.storage().track_cache_storage_for_storage_key(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn track_cache_storage_for_storage_key(
        &self,
        params: commands::TrackCacheStorageForStorageKeyParams,
    ) -> ydp_core::Request<commands::TrackCacheStorageForStorageKey> {
        self.session
            .send::<commands::TrackCacheStorageForStorageKey>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Storage.command.trackIndexedDBForOrigin。
    ///
    ///协议原文：
    ///
    ///<pre>Registers origin to be notified when an update occurs to its IndexedDB.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::storage::commands::TrackIndexedDbForOriginParams) -> ydp_core::Result<()> {
    ///let response = session.storage().track_indexed_db_for_origin(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn track_indexed_db_for_origin(
        &self,
        params: commands::TrackIndexedDbForOriginParams,
    ) -> ydp_core::Request<commands::TrackIndexedDbForOrigin> {
        self.session
            .send::<commands::TrackIndexedDbForOrigin>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Storage.command.trackIndexedDBForStorageKey。
    ///
    ///协议原文：
    ///
    ///<pre>Registers storage key to be notified when an update occurs to its IndexedDB.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::storage::commands::TrackIndexedDbForStorageKeyParams) -> ydp_core::Result<()> {
    ///let response = session.storage().track_indexed_db_for_storage_key(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn track_indexed_db_for_storage_key(
        &self,
        params: commands::TrackIndexedDbForStorageKeyParams,
    ) -> ydp_core::Request<commands::TrackIndexedDbForStorageKey> {
        self.session
            .send::<commands::TrackIndexedDbForStorageKey>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Storage.command.untrackCacheStorageForOrigin。
    ///
    ///协议原文：
    ///
    ///<pre>Unregisters origin from receiving notifications for cache storage.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::storage::commands::UntrackCacheStorageForOriginParams) -> ydp_core::Result<()> {
    ///let response = session.storage().untrack_cache_storage_for_origin(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn untrack_cache_storage_for_origin(
        &self,
        params: commands::UntrackCacheStorageForOriginParams,
    ) -> ydp_core::Request<commands::UntrackCacheStorageForOrigin> {
        self.session
            .send::<commands::UntrackCacheStorageForOrigin>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Storage.command.untrackCacheStorageForStorageKey。
    ///
    ///协议原文：
    ///
    ///<pre>Unregisters storage key from receiving notifications for cache storage.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::storage::commands::UntrackCacheStorageForStorageKeyParams) -> ydp_core::Result<()> {
    ///let response = session.storage().untrack_cache_storage_for_storage_key(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn untrack_cache_storage_for_storage_key(
        &self,
        params: commands::UntrackCacheStorageForStorageKeyParams,
    ) -> ydp_core::Request<commands::UntrackCacheStorageForStorageKey> {
        self.session
            .send::<commands::UntrackCacheStorageForStorageKey>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Storage.command.untrackIndexedDBForOrigin。
    ///
    ///协议原文：
    ///
    ///<pre>Unregisters origin from receiving notifications for IndexedDB.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::storage::commands::UntrackIndexedDbForOriginParams) -> ydp_core::Result<()> {
    ///let response = session.storage().untrack_indexed_db_for_origin(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn untrack_indexed_db_for_origin(
        &self,
        params: commands::UntrackIndexedDbForOriginParams,
    ) -> ydp_core::Request<commands::UntrackIndexedDbForOrigin> {
        self.session
            .send::<commands::UntrackIndexedDbForOrigin>(params)
    }
    ///拥有参数并返回冷 Request；首次 poll 才提交，Drop 仅结束本地等待，发布后远端可能继续执行。协议来源：Storage.command.untrackIndexedDBForStorageKey。
    ///
    ///协议原文：
    ///
    ///<pre>Unregisters storage key from receiving notifications for IndexedDB.</pre>
    ///
    ///
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# async fn call(session: &ydp_core::Session, params: ydp::storage::commands::UntrackIndexedDbForStorageKeyParams) -> ydp_core::Result<()> {
    ///let response = session.storage().untrack_indexed_db_for_storage_key(params).await?;
    ///# Ok(())
    ///# }
    ///```
    pub fn untrack_indexed_db_for_storage_key(
        &self,
        params: commands::UntrackIndexedDbForStorageKeyParams,
    ) -> ydp_core::Request<commands::UntrackIndexedDbForStorageKey> {
        self.session
            .send::<commands::UntrackIndexedDbForStorageKey>(params)
    }
}
