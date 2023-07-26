/// \[\#next-free-field: 6\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RateLimit {
    /// The rate limit domain to use in the rate limit service request.
    #[prost(string, tag = "1")]
    pub domain: ::prost::alloc::string::String,
    /// Specifies the rate limit configuration stage. Each configured rate limit filter performs a
    /// rate limit check using descriptors configured in the
    /// :ref:`envoy_v3_api_msg_extensions.filters.network.thrift_proxy.v3.RouteAction` for the request.
    /// Only those entries with a matching stage number are used for a given filter. If not set, the
    /// default stage number is 0.
    ///
    /// .. note::
    ///
    /// The filter supports a range of 0 - 10 inclusively for stage numbers.
    #[prost(uint32, tag = "2")]
    pub stage: u32,
    /// The timeout in milliseconds for the rate limit service RPC. If not
    /// set, this defaults to 20ms.
    #[prost(message, optional, tag = "3")]
    pub timeout: ::core::option::Option<
        super::super::super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// The filter's behaviour in case the rate limiting service does
    /// not respond back. When it is set to true, Envoy will not allow traffic in case of
    /// communication failure between rate limiting service and the proxy.
    /// Defaults to false.
    #[prost(bool, tag = "4")]
    pub failure_mode_deny: bool,
    /// Configuration for an external rate limit service provider. If not
    /// specified, any calls to the rate limit service will immediately return
    /// success.
    #[prost(message, optional, tag = "5")]
    pub rate_limit_service: ::core::option::Option<
        super::super::super::super::super::super::super::config::ratelimit::v3::RateLimitServiceConfig,
    >,
}
