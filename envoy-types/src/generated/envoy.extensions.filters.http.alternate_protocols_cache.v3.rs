/// Configuration for the alternate protocols cache HTTP filter.
/// \[\#extension: envoy.filters.http.alternate_protocols_cache\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterConfig {
    /// This field is ignored: the alternate protocols cache filter will use the
    /// cache for the cluster the request is routed to.
    #[deprecated]
    #[prost(message, optional, tag = "1")]
    pub alternate_protocols_cache_options: ::core::option::Option<
        super::super::super::super::super::config::core::v3::AlternateProtocolsCacheOptions,
    >,
}
