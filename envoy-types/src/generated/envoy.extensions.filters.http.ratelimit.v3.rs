// This file is @generated by prost-build.
/// \[\#next-free-field: 14\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RateLimit {
    /// The rate limit domain to use when calling the rate limit service.
    #[prost(string, tag = "1")]
    pub domain: ::prost::alloc::string::String,
    /// Specifies the rate limit configurations to be applied with the same
    /// stage number. If not set, the default stage number is 0.
    ///
    /// .. note::
    ///
    /// The filter supports a range of 0 - 10 inclusively for stage numbers.
    #[prost(uint32, tag = "2")]
    pub stage: u32,
    ///
    /// The type of requests the filter should apply to. The supported
    /// types are `internal`, `external` or `both`. A request is considered internal if
    /// : ref:`x-envoy-internal<config_http_conn_man_headers_x-envoy-internal>` is set to true. If
    /// : ref:`x-envoy-internal<config_http_conn_man_headers_x-envoy-internal>` is not set or false, a
    /// request is considered external. The filter defaults to `both`, and it will apply to all request
    /// types.
    #[prost(string, tag = "3")]
    pub request_type: ::prost::alloc::string::String,
    /// The timeout in milliseconds for the rate limit service RPC. If not
    /// set, this defaults to 20ms.
    #[prost(message, optional, tag = "4")]
    pub timeout: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// The filter's behaviour in case the rate limiting service does
    /// not respond back. When it is set to true, Envoy will not allow traffic in case of
    /// communication failure between rate limiting service and the proxy.
    #[prost(bool, tag = "5")]
    pub failure_mode_deny: bool,
    /// Specifies whether a `RESOURCE_EXHAUSTED` gRPC code must be returned instead
    /// of the default `UNAVAILABLE` gRPC code for a rate limited gRPC call. The
    /// HTTP code will be 200 for a gRPC response.
    #[prost(bool, tag = "6")]
    pub rate_limited_as_resource_exhausted: bool,
    /// Configuration for an external rate limit service provider. If not
    /// specified, any calls to the rate limit service will immediately return
    /// success.
    #[prost(message, optional, tag = "7")]
    pub rate_limit_service: ::core::option::Option<
        super::super::super::super::super::config::ratelimit::v3::RateLimitServiceConfig,
    >,
    /// Defines the standard version to use for X-RateLimit headers emitted by the filter:
    ///
    /// *
    /// `X-RateLimit-Limit` - indicates the request-quota associated to the
    ///   client in the current time-window followed by the description of the
    ///   quota policy. The values are returned by the rate limiting service in
    /// : ref:`current_limit<envoy_v3_api_field_service.ratelimit.v3.RateLimitResponse.DescriptorStatus.current_limit>`
    ///   field. Example: `10, 10;w=1;name="per-ip", 1000;w=3600`.
    ///
    ///
    /// * `X-RateLimit-Remaining` - indicates the remaining requests in the
    ///   current time-window. The values are returned by the rate limiting service
    ///   in :ref:`limit_remaining<envoy_v3_api_field_service.ratelimit.v3.RateLimitResponse.DescriptorStatus.limit_remaining>`
    ///   field.
    /// * `X-RateLimit-Reset` - indicates the number of seconds until reset of
    ///   the current time-window. The values are returned by the rate limiting service
    ///   in :ref:`duration_until_reset<envoy_v3_api_field_service.ratelimit.v3.RateLimitResponse.DescriptorStatus.duration_until_reset>`
    ///   field.
    ///
    /// In case rate limiting policy specifies more then one time window, the values
    /// above represent the window that is closest to reaching its limit.
    ///
    /// For more information about the headers specification see selected version of
    /// the `draft RFC <<https://tools.ietf.org/id/draft-polli-ratelimit-headers-03.html>`\_.>
    ///
    /// Disabled by default.
    ///
    /// \[\#next-major-version: unify with local ratelimit, should use common.ratelimit.v3.XRateLimitHeadersRFCVersion instead.\]
    #[prost(enumeration = "rate_limit::XRateLimitHeadersRfcVersion", tag = "8")]
    pub enable_x_ratelimit_headers: i32,
    /// Disables emitting the :ref:`x-envoy-ratelimited<config_http_filters_router_x-envoy-ratelimited>` header
    /// in case of rate limiting (i.e. 429 responses).
    /// Having this header not present potentially makes the request retriable.
    #[prost(bool, tag = "9")]
    pub disable_x_envoy_ratelimited_header: bool,
    /// This field allows for a custom HTTP response status code to the downstream client when
    /// the request has been rate limited.
    /// Defaults to 429 (TooManyRequests).
    ///
    /// .. note::
    /// If this is set to \< 400, 429 will be used instead.
    #[prost(message, optional, tag = "10")]
    pub rate_limited_status: ::core::option::Option<
        super::super::super::super::super::r#type::v3::HttpStatus,
    >,
    /// Specifies a list of HTTP headers that should be added to each response for requests that
    /// have been rate limited.
    #[prost(message, repeated, tag = "11")]
    pub response_headers_to_add: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::core::v3::HeaderValueOption,
    >,
    /// Sets the HTTP status that is returned to the client when the ratelimit server returns an error
    /// or cannot be reached. The default status is 500.
    #[prost(message, optional, tag = "12")]
    pub status_on_error: ::core::option::Option<
        super::super::super::super::super::r#type::v3::HttpStatus,
    >,
    /// Optional additional prefix to use when emitting statistics. This allows to distinguish
    /// emitted statistics between configured `ratelimit` filters in an HTTP filter chain.
    #[prost(string, tag = "13")]
    pub stat_prefix: ::prost::alloc::string::String,
}
/// Nested message and enum types in `RateLimit`.
pub mod rate_limit {
    /// Defines the version of the standard to use for X-RateLimit headers.
    ///
    /// \[\#next-major-version: unify with local ratelimit, should use common.ratelimit.v3.XRateLimitHeadersRFCVersion instead.\]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum XRateLimitHeadersRfcVersion {
        /// X-RateLimit headers disabled.
        Off = 0,
        /// Use `draft RFC Version 03 <<https://tools.ietf.org/id/draft-polli-ratelimit-headers-03.html>`\_.>
        DraftVersion03 = 1,
    }
    impl XRateLimitHeadersRfcVersion {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Off => "OFF",
                Self::DraftVersion03 => "DRAFT_VERSION_03",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OFF" => Some(Self::Off),
                "DRAFT_VERSION_03" => Some(Self::DraftVersion03),
                _ => None,
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RateLimitPerRoute {
    /// Specifies if the rate limit filter should include the virtual host rate limits.
    /// \[\#next-major-version: unify with local ratelimit, should use common.ratelimit.v3.VhRateLimitsOptions instead.\]
    #[prost(enumeration = "rate_limit_per_route::VhRateLimitsOptions", tag = "1")]
    pub vh_rate_limits: i32,
    /// Specifies if the rate limit filter should include the lower levels (route level, virtual host level or cluster weight level) rate limits override options.
    /// \[\#not-implemented-hide:\]
    #[prost(enumeration = "rate_limit_per_route::OverrideOptions", tag = "2")]
    pub override_option: i32,
    ///
    /// Rate limit configuration that is used to generate a list of descriptor entries based on
    /// the request context. The generated entries will be used to find one or multiple matched rate
    /// limit rule from the `descriptors`.
    /// If this is set, then
    /// : ref:`VirtualHost.rate_limits<envoy_v3_api_field_config.route.v3.VirtualHost.rate_limits>` or
    /// : ref:`RouteAction.rate_limits<envoy_v3_api_field_config.route.v3.RouteAction.rate_limits>` fields
    /// will be ignored.
    ///
    /// .. note::
    /// Not all configuration fields of
    /// : ref:`rate limit config <envoy_v3_api_msg_config.route.v3.RateLimit>` is supported at here.
    /// Following fields are not supported:
    /// 1. :ref:`rate limit stage <envoy_v3_api_field_config.route.v3.RateLimit.stage>`.
    /// 1. :ref:`dynamic metadata <envoy_v3_api_field_config.route.v3.RateLimit.Action.dynamic_metadata>`.
    /// 1. :ref:`disable_key <envoy_v3_api_field_config.route.v3.RateLimit.disable_key>`.
    /// 1. :ref:`override limit <envoy_v3_api_field_config.route.v3.RateLimit.limit>`.
    #[prost(message, repeated, tag = "3")]
    pub rate_limits: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::route::v3::RateLimit,
    >,
    /// Overrides the domain. If not set, uses the filter-level domain instead.
    #[prost(string, tag = "4")]
    pub domain: ::prost::alloc::string::String,
}
/// Nested message and enum types in `RateLimitPerRoute`.
pub mod rate_limit_per_route {
    /// \[\#next-major-version: unify with local ratelimit, should use common.ratelimit.v3.VhRateLimitsOptions instead.\]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum VhRateLimitsOptions {
        /// Use the virtual host rate limits unless the route has a rate limit policy.
        Override = 0,
        /// Use the virtual host rate limits even if the route has a rate limit policy.
        Include = 1,
        /// Ignore the virtual host rate limits even if the route does not have a rate limit policy.
        Ignore = 2,
    }
    impl VhRateLimitsOptions {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Override => "OVERRIDE",
                Self::Include => "INCLUDE",
                Self::Ignore => "IGNORE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OVERRIDE" => Some(Self::Override),
                "INCLUDE" => Some(Self::Include),
                "IGNORE" => Some(Self::Ignore),
                _ => None,
            }
        }
    }
    /// The override option determines how the filter handles the cases where there is an override config at a more specific level than this one (from least to most specific: virtual host, route, cluster weight).
    /// \[\#not-implemented-hide:\]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum OverrideOptions {
        /// Client-defined default, typically OVERRIDE_POLICY. If VhRateLimitsOptions is set, that will be used instead.
        Default = 0,
        /// If there is an override config at a more specific level, use that instead of this one.
        OverridePolicy = 1,
        /// If there is an override config at a more specific level, use data from both.
        IncludePolicy = 2,
        /// If there is an override config at a more specific level, ignore it and use only this one.
        IgnorePolicy = 3,
    }
    impl OverrideOptions {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Default => "DEFAULT",
                Self::OverridePolicy => "OVERRIDE_POLICY",
                Self::IncludePolicy => "INCLUDE_POLICY",
                Self::IgnorePolicy => "IGNORE_POLICY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEFAULT" => Some(Self::Default),
                "OVERRIDE_POLICY" => Some(Self::OverridePolicy),
                "INCLUDE_POLICY" => Some(Self::IncludePolicy),
                "IGNORE_POLICY" => Some(Self::IgnorePolicy),
                _ => None,
            }
        }
    }
}
