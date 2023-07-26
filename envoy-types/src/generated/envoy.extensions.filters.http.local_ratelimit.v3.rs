/// \[\#next-free-field: 14\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalRateLimit {
    /// The human readable prefix to use when emitting stats.
    #[prost(string, tag = "1")]
    pub stat_prefix: ::prost::alloc::string::String,
    /// This field allows for a custom HTTP response status code to the downstream client when
    /// the request has been rate limited.
    /// Defaults to 429 (TooManyRequests).
    ///
    /// .. note::
    /// If this is set to \< 400, 429 will be used instead.
    #[prost(message, optional, tag = "2")]
    pub status: ::core::option::Option<
        super::super::super::super::super::r#type::v3::HttpStatus,
    >,
    /// The token bucket configuration to use for rate limiting requests that are processed by this
    /// filter. Each request processed by the filter consumes a single token. If the token is available,
    /// the request will be allowed. If no tokens are available, the request will receive the configured
    /// rate limit status.
    ///
    /// .. note::
    /// It's fine for the token bucket to be unset for the global configuration since the rate limit
    /// can be applied at a the virtual host or route level. Thus, the token bucket must be set
    /// for the per route configuration otherwise the config will be rejected.
    ///
    /// .. note::
    /// When using per route configuration, the bucket becomes unique to that route.
    ///
    /// .. note::
    /// In the current implementation the token bucket's :ref:`fill_interval <envoy_v3_api_field_type.v3.TokenBucket.fill_interval>` must be >= 50ms to avoid too aggressive
    /// refills.
    #[prost(message, optional, tag = "3")]
    pub token_bucket: ::core::option::Option<
        super::super::super::super::super::r#type::v3::TokenBucket,
    >,
    /// If set, this will enable -- but not necessarily enforce -- the rate limit for the given
    /// fraction of requests.
    /// Defaults to 0% of requests for safety.
    #[prost(message, optional, tag = "4")]
    pub filter_enabled: ::core::option::Option<
        super::super::super::super::super::config::core::v3::RuntimeFractionalPercent,
    >,
    /// If set, this will enforce the rate limit decisions for the given fraction of requests.
    ///
    /// Note: this only applies to the fraction of enabled requests.
    ///
    /// Defaults to 0% of requests for safety.
    #[prost(message, optional, tag = "5")]
    pub filter_enforced: ::core::option::Option<
        super::super::super::super::super::config::core::v3::RuntimeFractionalPercent,
    >,
    /// Specifies a list of HTTP headers that should be added to each request that
    /// has been rate limited and is also forwarded upstream. This can only occur when the
    /// filter is enabled but not enforced.
    #[prost(message, repeated, tag = "10")]
    pub request_headers_to_add_when_not_enforced: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::core::v3::HeaderValueOption,
    >,
    /// Specifies a list of HTTP headers that should be added to each response for requests that
    /// have been rate limited. This occurs when the filter is either enabled or fully enforced.
    #[prost(message, repeated, tag = "6")]
    pub response_headers_to_add: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::core::v3::HeaderValueOption,
    >,
    /// The rate limit descriptor list to use in the local rate limit to override
    /// on. The rate limit descriptor is selected by the first full match from the
    /// request descriptors.
    ///
    /// Example on how to use :ref:`this <config_http_filters_local_rate_limit_descriptors>`.
    ///
    /// .. note::
    ///
    /// In the current implementation the descriptor's token bucket :ref:`fill_interval <envoy_v3_api_field_type.v3.TokenBucket.fill_interval>` must be a multiple
    /// global :ref:`token bucket's<envoy_v3_api_field_extensions.filters.http.local_ratelimit.v3.LocalRateLimit.token_bucket>` fill interval.
    ///
    /// The descriptors must match verbatim for rate limiting to apply. There is no partial
    /// match by a subset of descriptor entries in the current implementation.
    #[prost(message, repeated, tag = "8")]
    pub descriptors: ::prost::alloc::vec::Vec<
        super::super::super::super::common::ratelimit::v3::LocalRateLimitDescriptor,
    >,
    /// Specifies the rate limit configurations to be applied with the same
    /// stage number. If not set, the default stage number is 0.
    ///
    /// .. note::
    ///
    /// The filter supports a range of 0 - 10 inclusively for stage numbers.
    #[prost(uint32, tag = "9")]
    pub stage: u32,
    /// Specifies the scope of the rate limiter's token bucket.
    /// If set to false, the token bucket is shared across all worker threads,
    /// thus the rate limits are applied per Envoy process.
    /// If set to true, a token bucket is allocated for each connection.
    /// Thus the rate limits are applied per connection thereby allowing
    /// one to rate limit requests on a per connection basis.
    /// If unspecified, the default value is false.
    #[prost(bool, tag = "11")]
    pub local_rate_limit_per_downstream_connection: bool,
    /// Defines the standard version to use for X-RateLimit headers emitted by the filter.
    ///
    /// Disabled by default.
    #[prost(
        enumeration = "super::super::super::super::common::ratelimit::v3::XRateLimitHeadersRfcVersion",
        tag = "12"
    )]
    pub enable_x_ratelimit_headers: i32,
    /// Specifies if the local rate limit filter should include the virtual host rate limits.
    #[prost(
        enumeration = "super::super::super::super::common::ratelimit::v3::VhRateLimitsOptions",
        tag = "13"
    )]
    pub vh_rate_limits: i32,
}
