/// \[\#next-free-field: 7\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RateLimit {
    /// The prefix to use when emitting :ref:`statistics <config_network_filters_rate_limit_stats>`.
    #[prost(string, tag = "1")]
    pub stat_prefix: ::prost::alloc::string::String,
    /// The rate limit domain to use in the rate limit service request.
    #[prost(string, tag = "2")]
    pub domain: ::prost::alloc::string::String,
    /// The rate limit descriptor list to use in the rate limit service request.
    #[prost(message, repeated, tag = "3")]
    pub descriptors: ::prost::alloc::vec::Vec<
        super::super::super::super::common::ratelimit::v3::RateLimitDescriptor,
    >,
    /// The timeout in milliseconds for the rate limit service RPC. If not
    /// set, this defaults to 20ms.
    #[prost(message, optional, tag = "4")]
    pub timeout: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// The filter's behaviour in case the rate limiting service does
    /// not respond back. When it is set to true, Envoy will not allow traffic in case of
    /// communication failure between rate limiting service and the proxy.
    /// Defaults to false.
    #[prost(bool, tag = "5")]
    pub failure_mode_deny: bool,
    /// Configuration for an external rate limit service provider. If not
    /// specified, any calls to the rate limit service will immediately return
    /// success.
    #[prost(message, optional, tag = "6")]
    pub rate_limit_service: ::core::option::Option<
        super::super::super::super::super::config::ratelimit::v3::RateLimitServiceConfig,
    >,
}
