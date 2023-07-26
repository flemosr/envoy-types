/// Configuration for a cache implementation that caches in the local file system.
///
/// By default this cache uses a least-recently-used eviction strategy.
///
/// For implementation details, see `DESIGN.md <<https://github.com/envoyproxy/envoy/blob/main/source/extensions/http/cache/file_system_http_cache/DESIGN.md>`\_.>
/// \[\#next-free-field: 11\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileSystemHttpCacheConfig {
    /// Configuration of a manager for how the file system is used asynchronously.
    #[prost(message, optional, tag = "1")]
    pub manager_config: ::core::option::Option<
        super::super::super::super::common::async_files::v3::AsyncFileManagerConfig,
    >,
    /// Path at which the cache files will be stored.
    ///
    /// This also doubles as the unique identifier for a cache, so a cache can be shared
    /// between different routes, or separate paths can be used to specify separate caches.
    ///
    /// If the same `cache_path` is used in more than one `CacheConfig`, the rest of the
    /// `FileSystemHttpCacheConfig` must also match, and will refer to the same cache
    /// instance.
    #[prost(string, tag = "2")]
    pub cache_path: ::prost::alloc::string::String,
    /// The maximum size of the cache in bytes - when reached, cache eviction is triggered.
    ///
    /// This is measured as the sum of file sizes, such that it includes headers, trailers,
    /// and metadata, but does not include e.g. file system overhead and block size padding.
    ///
    /// If unset there is no limit except file system failure.
    #[prost(message, optional, tag = "3")]
    pub max_cache_size_bytes: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt64Value,
    >,
    /// The maximum size of a cache entry in bytes - larger responses will not be cached.
    ///
    /// This is measured as the file size for the cache entry, such that it includes
    /// headers, trailers, and metadata.
    ///
    /// If unset there is no limit.
    ///
    /// \\[\#not-implemented-hide:\\]
    #[prost(message, optional, tag = "4")]
    pub max_individual_cache_entry_size_bytes: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt64Value,
    >,
    /// The maximum number of cache entries - when reached, cache eviction is triggered.
    ///
    /// If unset there is no limit.
    #[prost(message, optional, tag = "5")]
    pub max_cache_entry_count: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt64Value,
    >,
    /// A number of folders into which to subdivide the cache.
    ///
    /// Setting this can help with performance in file systems where a large number of inodes
    /// in a single branch degrades performance. The optimal value in that case would be
    /// `sqrt(expected_cache_entry_count)`.
    ///
    /// On file systems that perform well with many inodes, the default value of 1 should be used.
    ///
    /// \\[\#not-implemented-hide:\\]
    #[prost(uint32, tag = "6")]
    pub cache_subdivisions: u32,
    /// The amount of the maximum cache size or count to evict when cache eviction is
    /// triggered. For example, if `max_cache_size_bytes` is 10000000 and `evict_fraction`
    /// is 0.2, then when the cache exceeds 10MB, entries will be evicted until the cache size is
    /// less than or equal to 8MB.
    ///
    /// The default value of 0 means when the cache exceeds 10MB, entries will be evicted only
    /// until the cache is less than or equal to 10MB.
    ///
    /// Evicting a larger fraction will mean the eviction thread will run less often (sparing
    /// CPU load) at the cost of more cache misses due to the extra evicted entries.
    ///
    /// \\[\#not-implemented-hide:\\]
    #[prost(float, tag = "7")]
    pub evict_fraction: f32,
    /// The longest amount of time to wait before running a cache eviction pass. An eviction
    /// pass may not necessarily remove any files, but it will update the cache state to match
    /// the on-disk state. This can be important if multiple instances are accessing the same
    /// cache in parallel. (e.g. if two instances each independently added non-overlapping 10MB
    /// of content to a cache with a 15MB limit, neither instance would be aware that the limit
    /// was exceeded without this synchronizing pass.)
    ///
    /// If an eviction pass has not happened within this duration, the eviction thread will
    /// be awoken and perform an eviction pass.
    ///
    /// If unset, there will be no eviction passes except those triggered by cache limits.
    ///
    /// \\[\#not-implemented-hide:\\]
    #[prost(message, optional, tag = "8")]
    pub max_eviction_period: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// The shortest amount of time between cache eviction passes. This can be used to reduce
    /// eviction churn, if your cache max size can be flexible. If a cache eviction pass already
    /// occurred more recently than this period when another would be triggered, that new
    /// pass is cancelled.
    ///
    /// This means the cache can potentially grow beyond `max_cache_size_bytes` by as much as
    /// can be written within the duration specified.
    ///
    /// Generally you would use *either* `min_eviction_period` *or* `evict_fraction` to
    /// reduce churn. Both together will work but since they're both aiming for the same goal,
    /// it's simpler not to.
    ///
    /// \\[\#not-implemented-hide:\\]
    #[prost(message, optional, tag = "9")]
    pub min_eviction_period: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// If true, and the cache path does not exist, attempt to create the cache path, including
    /// any missing directories leading up to it. On failure, the config is rejected.
    ///
    /// If false, and the cache path does not exist, the config is rejected.
    ///
    /// \\[\#not-implemented-hide:\\]
    #[prost(bool, tag = "10")]
    pub create_cache_path: bool,
}
