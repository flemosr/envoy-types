/// [#next-free-field: 12]
#[allow(clippy::derive_partial_eq_without_eq)]
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
    ///   The filter supports a range of 0 - 10 inclusively for stage numbers.
    #[prost(uint32, tag = "2")]
    pub stage: u32,
    /// The type of requests the filter should apply to. The supported
    /// types are ``internal``, ``external`` or ``both``. A request is considered internal if
    /// :ref:`x-envoy-internal<config_http_conn_man_headers_x-envoy-internal>` is set to true. If
    /// :ref:`x-envoy-internal<config_http_conn_man_headers_x-envoy-internal>` is not set or false, a
    /// request is considered external. The filter defaults to ``both``, and it will apply to all request
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
    /// Specifies whether a ``RESOURCE_EXHAUSTED`` gRPC code must be returned instead
    /// of the default ``UNAVAILABLE`` gRPC code for a rate limited gRPC call. The
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
    /// * ``X-RateLimit-Limit`` - indicates the request-quota associated to the
    ///    client in the current time-window followed by the description of the
    ///    quota policy. The values are returned by the rate limiting service in
    ///    :ref:`current_limit<envoy_v3_api_field_service.ratelimit.v3.RateLimitResponse.DescriptorStatus.current_limit>`
    ///    field. Example: ``10, 10;w=1;name="per-ip", 1000;w=3600``.
    /// * ``X-RateLimit-Remaining`` - indicates the remaining requests in the
    ///    current time-window. The values are returned by the rate limiting service
    ///    in :ref:`limit_remaining<envoy_v3_api_field_service.ratelimit.v3.RateLimitResponse.DescriptorStatus.limit_remaining>`
    ///    field.
    /// * ``X-RateLimit-Reset`` - indicates the number of seconds until reset of
    ///    the current time-window. The values are returned by the rate limiting service
    ///    in :ref:`duration_until_reset<envoy_v3_api_field_service.ratelimit.v3.RateLimitResponse.DescriptorStatus.duration_until_reset>`
    ///    field.
    ///
    /// In case rate limiting policy specifies more then one time window, the values
    /// above represent the window that is closest to reaching its limit.
    ///
    /// For more information about the headers specification see selected version of
    /// the `draft RFC <<https://tools.ietf.org/id/draft-polli-ratelimit-headers-03.html>`_.>
    ///
    /// Disabled by default.
    ///
    /// [#next-major-version: unify with local ratelimit, should use common.ratelimit.v3.XRateLimitHeadersRFCVersion instead.]
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
    ///    If this is set to < 400, 429 will be used instead.
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
}
/// Nested message and enum types in `RateLimit`.
pub mod rate_limit {
    /// Defines the version of the standard to use for X-RateLimit headers.
    ///
    /// [#next-major-version: unify with local ratelimit, should use common.ratelimit.v3.XRateLimitHeadersRFCVersion instead.]
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
        /// Use `draft RFC Version 03 <<https://tools.ietf.org/id/draft-polli-ratelimit-headers-03.html>`_.>
        DraftVersion03 = 1,
    }
    impl XRateLimitHeadersRfcVersion {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                XRateLimitHeadersRfcVersion::Off => "OFF",
                XRateLimitHeadersRfcVersion::DraftVersion03 => "DRAFT_VERSION_03",
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
/// Global rate limiting :ref:`architecture overview <arch_overview_global_rate_limit>`.
/// Also applies to Local rate limiting :ref:`using descriptors <config_http_filters_local_rate_limit_descriptors>`.
/// \[#not-implemented-hide:\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RateLimitConfig {
    /// Refers to the stage set in the filter. The rate limit configuration only
    /// applies to filters with the same stage number. The default stage number is
    /// 0.
    ///
    /// .. note::
    ///
    ///    The filter supports a range of 0 - 10 inclusively for stage numbers.
    #[prost(uint32, tag = "1")]
    pub stage: u32,
    /// The key to be set in runtime to disable this rate limit configuration.
    #[prost(string, tag = "2")]
    pub disable_key: ::prost::alloc::string::String,
    /// A list of actions that are to be applied for this rate limit configuration.
    /// Order matters as the actions are processed sequentially and the descriptor
    /// is composed by appending descriptor entries in that sequence. If an action
    /// cannot append a descriptor entry, no descriptor is generated for the
    /// configuration. See :ref:`composing actions
    /// <config_http_filters_rate_limit_composing_actions>` for additional documentation.
    #[prost(message, repeated, tag = "3")]
    pub actions: ::prost::alloc::vec::Vec<rate_limit_config::Action>,
    /// An optional limit override to be appended to the descriptor produced by this
    /// rate limit configuration. If the override value is invalid or cannot be resolved
    /// from metadata, no override is provided. See :ref:`rate limit override
    /// <config_http_filters_rate_limit_rate_limit_override>` for more information.
    #[prost(message, optional, tag = "4")]
    pub limit: ::core::option::Option<rate_limit_config::Override>,
}
/// Nested message and enum types in `RateLimitConfig`.
pub mod rate_limit_config {
    /// [#next-free-field: 10]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Action {
        #[prost(oneof = "action::ActionSpecifier", tags = "1, 2, 3, 4, 5, 6, 8, 9")]
        pub action_specifier: ::core::option::Option<action::ActionSpecifier>,
    }
    /// Nested message and enum types in `Action`.
    pub mod action {
        /// The following descriptor entry is appended to the descriptor:
        ///
        /// .. code-block:: cpp
        ///
        ///    ("source_cluster", "<local service cluster>")
        ///
        /// <local service cluster> is derived from the :option:`--service-cluster` option.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SourceCluster {}
        /// The following descriptor entry is appended to the descriptor:
        ///
        /// .. code-block:: cpp
        ///
        ///    ("destination_cluster", "<routed target cluster>")
        ///
        /// Once a request matches against a route table rule, a routed cluster is determined by one of
        /// the following :ref:`route table configuration <envoy_v3_api_msg_config.route.v3.RouteConfiguration>`
        /// settings:
        ///
        /// * :ref:`cluster <envoy_v3_api_field_config.route.v3.RouteAction.cluster>` indicates the upstream cluster
        ///    to route to.
        /// * :ref:`weighted_clusters <envoy_v3_api_field_config.route.v3.RouteAction.weighted_clusters>`
        ///    chooses a cluster randomly from a set of clusters with attributed weight.
        /// * :ref:`cluster_header <envoy_v3_api_field_config.route.v3.RouteAction.cluster_header>` indicates which
        ///    header in the request contains the target cluster.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DestinationCluster {}
        /// The following descriptor entry is appended when a header contains a key that matches the
        /// ``header_name``:
        ///
        /// .. code-block:: cpp
        ///
        ///    ("<descriptor_key>", "<header_value_queried_from_header>")
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct RequestHeaders {
            /// The header name to be queried from the request headers. The header’s
            /// value is used to populate the value of the descriptor entry for the
            /// descriptor_key.
            #[prost(string, tag = "1")]
            pub header_name: ::prost::alloc::string::String,
            /// The key to use in the descriptor entry.
            #[prost(string, tag = "2")]
            pub descriptor_key: ::prost::alloc::string::String,
            /// If set to true, Envoy skips the descriptor while calling rate limiting service
            /// when header is not present in the request. By default it skips calling the
            /// rate limiting service if this header is not present in the request.
            #[prost(bool, tag = "3")]
            pub skip_if_absent: bool,
        }
        /// The following descriptor entry is appended to the descriptor and is populated using the
        /// trusted address from :ref:`x-forwarded-for <config_http_conn_man_headers_x-forwarded-for>`:
        ///
        /// .. code-block:: cpp
        ///
        ///    ("remote_address", "<trusted address from x-forwarded-for>")
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct RemoteAddress {}
        /// The following descriptor entry is appended to the descriptor:
        ///
        /// .. code-block:: cpp
        ///
        ///    ("generic_key", "<descriptor_value>")
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct GenericKey {
            /// The value to use in the descriptor entry.
            #[prost(string, tag = "1")]
            pub descriptor_value: ::prost::alloc::string::String,
            /// An optional key to use in the descriptor entry. If not set it defaults
            /// to 'generic_key' as the descriptor key.
            #[prost(string, tag = "2")]
            pub descriptor_key: ::prost::alloc::string::String,
        }
        /// The following descriptor entry is appended to the descriptor:
        ///
        /// .. code-block:: cpp
        ///
        ///    ("header_match", "<descriptor_value>")
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct HeaderValueMatch {
            /// The value to use in the descriptor entry.
            #[prost(string, tag = "1")]
            pub descriptor_value: ::prost::alloc::string::String,
            /// If set to true, the action will append a descriptor entry when the
            /// request matches the headers. If set to false, the action will append a
            /// descriptor entry when the request does not match the headers. The
            /// default value is true.
            #[prost(bool, tag = "2")]
            pub expect_match: bool,
            /// Specifies a set of headers that the rate limit action should match
            /// on. The action will check the request’s headers against all the
            /// specified headers in the config. A match will happen if all the
            /// headers in the config are present in the request with the same values
            /// (or based on presence if the value field is not in the config).
            #[prost(message, repeated, tag = "3")]
            pub headers: ::prost::alloc::vec::Vec<
                super::super::super::super::super::super::super::config::route::v3::HeaderMatcher,
            >,
        }
        /// The following descriptor entry is appended when the metadata contains a key value:
        ///
        /// .. code-block:: cpp
        ///
        ///    ("<descriptor_key>", "<value_queried_from_metadata>")
        /// [#next-free-field: 6]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MetaData {
            /// The key to use in the descriptor entry.
            #[prost(string, tag = "1")]
            pub descriptor_key: ::prost::alloc::string::String,
            /// Metadata struct that defines the key and path to retrieve the string value. A match will
            /// only happen if the value in the metadata is of type string.
            #[prost(message, optional, tag = "2")]
            pub metadata_key: ::core::option::Option<
                super::super::super::super::super::super::super::r#type::metadata::v3::MetadataKey,
            >,
            /// An optional value to use if ``metadata_key`` is empty. If not set and
            /// no value is present under the metadata_key then ``skip_if_absent`` is followed to
            /// skip calling the rate limiting service or skip the descriptor.
            #[prost(string, tag = "3")]
            pub default_value: ::prost::alloc::string::String,
            /// Source of metadata
            #[prost(enumeration = "meta_data::Source", tag = "4")]
            pub source: i32,
            /// If set to true, Envoy skips the descriptor while calling rate limiting service
            /// when ``metadata_key`` is empty and ``default_value`` is not set. By default it skips calling the
            /// rate limiting service in that case.
            #[prost(bool, tag = "5")]
            pub skip_if_absent: bool,
        }
        /// Nested message and enum types in `MetaData`.
        pub mod meta_data {
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
            pub enum Source {
                /// Query :ref:`dynamic metadata <well_known_dynamic_metadata>`
                Dynamic = 0,
                /// Query :ref:`route entry metadata <envoy_v3_api_field_config.route.v3.Route.metadata>`
                RouteEntry = 1,
            }
            impl Source {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        Source::Dynamic => "DYNAMIC",
                        Source::RouteEntry => "ROUTE_ENTRY",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "DYNAMIC" => Some(Self::Dynamic),
                        "ROUTE_ENTRY" => Some(Self::RouteEntry),
                        _ => None,
                    }
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ActionSpecifier {
            /// Rate limit on source cluster.
            #[prost(message, tag = "1")]
            SourceCluster(SourceCluster),
            /// Rate limit on destination cluster.
            #[prost(message, tag = "2")]
            DestinationCluster(DestinationCluster),
            /// Rate limit on request headers.
            #[prost(message, tag = "3")]
            RequestHeaders(RequestHeaders),
            /// Rate limit on remote address.
            #[prost(message, tag = "4")]
            RemoteAddress(RemoteAddress),
            /// Rate limit on a generic key.
            #[prost(message, tag = "5")]
            GenericKey(GenericKey),
            /// Rate limit on the existence of request headers.
            #[prost(message, tag = "6")]
            HeaderValueMatch(HeaderValueMatch),
            /// Rate limit on metadata.
            #[prost(message, tag = "8")]
            Metadata(MetaData),
            /// Rate limit descriptor extension. See the rate limit descriptor extensions documentation.
            /// [#extension-category: envoy.rate_limit_descriptors]
            #[prost(message, tag = "9")]
            Extension(
                super::super::super::super::super::super::super::config::core::v3::TypedExtensionConfig,
            ),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Override {
        #[prost(oneof = "r#override::OverrideSpecifier", tags = "1")]
        pub override_specifier: ::core::option::Option<r#override::OverrideSpecifier>,
    }
    /// Nested message and enum types in `Override`.
    pub mod r#override {
        /// Fetches the override from the dynamic metadata.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DynamicMetadata {
            /// Metadata struct that defines the key and path to retrieve the struct value.
            /// The value must be a struct containing an integer "requests_per_unit" property
            /// and a "unit" property with a value parseable to :ref:`RateLimitUnit
            /// enum <envoy_v3_api_enum_type.v3.RateLimitUnit>`
            #[prost(message, optional, tag = "1")]
            pub metadata_key: ::core::option::Option<
                super::super::super::super::super::super::super::r#type::metadata::v3::MetadataKey,
            >,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum OverrideSpecifier {
            /// Limit override from dynamic metadata.
            #[prost(message, tag = "1")]
            DynamicMetadata(DynamicMetadata),
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RateLimitPerRoute {
    /// Specifies if the rate limit filter should include the virtual host rate limits.
    /// [#next-major-version: unify with local ratelimit, should use common.ratelimit.v3.VhRateLimitsOptions instead.]
    #[prost(enumeration = "rate_limit_per_route::VhRateLimitsOptions", tag = "1")]
    pub vh_rate_limits: i32,
    /// Specifies if the rate limit filter should include the lower levels (route level, virtual host level or cluster weight level) rate limits override options.
    /// \[#not-implemented-hide:\]
    #[prost(enumeration = "rate_limit_per_route::OverrideOptions", tag = "2")]
    pub override_option: i32,
    /// Rate limit configuration. If not set, uses the
    /// :ref:`VirtualHost.rate_limits<envoy_v3_api_field_config.route.v3.VirtualHost.rate_limits>` or
    /// :ref:`RouteAction.rate_limits<envoy_v3_api_field_config.route.v3.RouteAction.rate_limits>` fields instead.
    /// \[#not-implemented-hide:\]
    #[prost(message, repeated, tag = "3")]
    pub rate_limits: ::prost::alloc::vec::Vec<RateLimitConfig>,
    /// Overrides the domain. If not set, uses the filter-level domain instead.
    #[prost(string, tag = "4")]
    pub domain: ::prost::alloc::string::String,
}
/// Nested message and enum types in `RateLimitPerRoute`.
pub mod rate_limit_per_route {
    /// [#next-major-version: unify with local ratelimit, should use common.ratelimit.v3.VhRateLimitsOptions instead.]
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
                VhRateLimitsOptions::Override => "OVERRIDE",
                VhRateLimitsOptions::Include => "INCLUDE",
                VhRateLimitsOptions::Ignore => "IGNORE",
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
    /// \[#not-implemented-hide:\]
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
                OverrideOptions::Default => "DEFAULT",
                OverrideOptions::OverridePolicy => "OVERRIDE_POLICY",
                OverrideOptions::IncludePolicy => "INCLUDE_POLICY",
                OverrideOptions::IgnorePolicy => "IGNORE_POLICY",
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
