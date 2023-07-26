/// Configuration for the alternate protocols cache HTTP filter.
/// [#extension: envoy.filters.http.alternate_protocols_cache]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterConfig {
    /// If set, causes the use of the alternate protocols cache, which is responsible for
    /// parsing and caching HTTP Alt-Svc headers. This enables the use of HTTP/3 for upstream
    /// servers that advertise supporting it.
    /// TODO(RyanTheOptimist): Make this field required when HTTP/3 is enabled via auto_http.
    #[prost(message, optional, tag = "1")]
    pub alternate_protocols_cache_options: ::core::option::Option<
        super::super::super::super::super::config::core::v3::AlternateProtocolsCacheOptions,
    >,
}
