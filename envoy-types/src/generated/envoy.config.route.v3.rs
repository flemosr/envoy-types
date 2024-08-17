/// The top level element in the routing configuration is a virtual host. Each virtual host has
/// a logical name as well as a set of domains that get routed to it based on the incoming request's
/// host header. This allows a single listener to service multiple top level domain path trees. Once
/// a virtual host is selected based on the domain, the routes are processed in order to see which
/// upstream cluster to route to or whether to perform a redirect.
/// \[\#next-free-field: 25\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualHost {
    /// The logical name of the virtual host. This is used when emitting certain
    /// statistics but is not relevant for routing.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A list of domains (host/authority header) that will be matched to this
    /// virtual host. Wildcard hosts are supported in the suffix or prefix form.
    ///
    /// Domain search order:
    ///
    /// 1. Exact domain names: `www.foo.com`.
    /// 1. Suffix domain wildcards: `*.foo.com` or `*-bar.foo.com`.
    /// 1. Prefix domain wildcards: `foo.*` or `foo-*`.
    /// 1. Special wildcard `*` matching any domain.
    ///
    /// .. note::
    ///
    /// The wildcard will not match the empty string.
    /// e.g. `*-bar.foo.com` will match `baz-bar.foo.com` but not `-bar.foo.com`.
    /// The longest wildcards match first.
    /// Only a single virtual host in the entire route configuration can match on `*`. A domain
    /// must be unique across all virtual hosts or the config will fail to load.
    ///
    /// Domains cannot contain control characters. This is validated by the well_known_regex HTTP_HEADER_VALUE.
    #[prost(string, repeated, tag = "2")]
    pub domains: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The list of routes that will be matched, in order, for incoming requests.
    /// The first route that matches will be used.
    /// Only one of this and `matcher` can be specified.
    #[prost(message, repeated, tag = "3")]
    pub routes: ::prost::alloc::vec::Vec<Route>,
    /// \[\#next-major-version: This should be included in a oneof with routes wrapped in a message.\]
    /// The match tree to use when resolving route actions for incoming requests. Only one of this and `routes`
    /// can be specified.
    #[prost(message, optional, tag = "21")]
    pub matcher: ::core::option::Option<
        super::super::super::super::xds::r#type::matcher::v3::Matcher,
    >,
    /// Specifies the type of TLS enforcement the virtual host expects. If this option is not
    /// specified, there is no TLS requirement for the virtual host.
    #[prost(enumeration = "virtual_host::TlsRequirementType", tag = "4")]
    pub require_tls: i32,
    /// A list of virtual clusters defined for this virtual host. Virtual clusters
    /// are used for additional statistics gathering.
    #[prost(message, repeated, tag = "5")]
    pub virtual_clusters: ::prost::alloc::vec::Vec<VirtualCluster>,
    /// Specifies a set of rate limit configurations that will be applied to the
    /// virtual host.
    #[prost(message, repeated, tag = "6")]
    pub rate_limits: ::prost::alloc::vec::Vec<RateLimit>,
    /// Specifies a list of HTTP headers that should be added to each request
    /// handled by this virtual host. Headers specified at this level are applied
    /// after headers from enclosed :ref:`envoy_v3_api_msg_config.route.v3.Route` and before headers from the
    /// enclosing :ref:`envoy_v3_api_msg_config.route.v3.RouteConfiguration`. For more information, including
    /// details on header value syntax, see the documentation on :ref:`custom request headers <config_http_conn_man_headers_custom_request_headers>`.
    #[prost(message, repeated, tag = "7")]
    pub request_headers_to_add: ::prost::alloc::vec::Vec<
        super::super::core::v3::HeaderValueOption,
    >,
    /// Specifies a list of HTTP headers that should be removed from each request
    /// handled by this virtual host.
    #[prost(string, repeated, tag = "13")]
    pub request_headers_to_remove: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Specifies a list of HTTP headers that should be added to each response
    /// handled by this virtual host. Headers specified at this level are applied
    /// after headers from enclosed :ref:`envoy_v3_api_msg_config.route.v3.Route` and before headers from the
    /// enclosing :ref:`envoy_v3_api_msg_config.route.v3.RouteConfiguration`. For more information, including
    /// details on header value syntax, see the documentation on :ref:`custom request headers <config_http_conn_man_headers_custom_request_headers>`.
    #[prost(message, repeated, tag = "10")]
    pub response_headers_to_add: ::prost::alloc::vec::Vec<
        super::super::core::v3::HeaderValueOption,
    >,
    /// Specifies a list of HTTP headers that should be removed from each response
    /// handled by this virtual host.
    #[prost(string, repeated, tag = "11")]
    pub response_headers_to_remove: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Indicates that the virtual host has a CORS policy. This field is ignored if related cors policy is
    /// found in the
    /// :ref:`VirtualHost.typed_per_filter_config<envoy_v3_api_field_config.route.v3.VirtualHost.typed_per_filter_config>`.
    ///
    /// .. attention::
    ///
    /// This option has been deprecated. Please use
    /// :ref:`VirtualHost.typed_per_filter_config<envoy_v3_api_field_config.route.v3.VirtualHost.typed_per_filter_config>`
    /// to configure the CORS HTTP filter.
    #[deprecated]
    #[prost(message, optional, tag = "8")]
    pub cors: ::core::option::Option<CorsPolicy>,
    /// This field can be used to provide virtual host level per filter config. The key should match the
    /// :ref:`filter config name <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpFilter.name>`.
    /// See :ref:`Http filter route specific config <arch_overview_http_filters_per_filter_config>`
    /// for details.
    /// \[\#comment: An entry's value may be wrapped in a
    /// :ref:`FilterConfig<envoy_v3_api_msg_config.route.v3.FilterConfig>`
    /// message to specify additional options.\]
    #[prost(map = "string, message", tag = "15")]
    pub typed_per_filter_config: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::super::super::google::protobuf::Any,
    >,
    /// Decides whether the :ref:`x-envoy-attempt-count <config_http_filters_router_x-envoy-attempt-count>` header should be included
    /// in the upstream request. Setting this option will cause it to override any existing header
    /// value, so in the case of two Envoys on the request path with this option enabled, the upstream
    /// will see the attempt count as perceived by the second Envoy. Defaults to false.
    /// This header is unaffected by the
    /// :ref:`suppress_envoy_headers <envoy_v3_api_field_extensions.filters.http.router.v3.Router.suppress_envoy_headers>` flag.
    ///
    /// \[\#next-major-version: rename to include_attempt_count_in_request.\]
    #[prost(bool, tag = "14")]
    pub include_request_attempt_count: bool,
    /// Decides whether the :ref:`x-envoy-attempt-count <config_http_filters_router_x-envoy-attempt-count>` header should be included
    /// in the downstream response. Setting this option will cause the router to override any existing header
    /// value, so in the case of two Envoys on the request path with this option enabled, the downstream
    /// will see the attempt count as perceived by the Envoy closest upstream from itself. Defaults to false.
    /// This header is unaffected by the
    /// :ref:`suppress_envoy_headers <envoy_v3_api_field_extensions.filters.http.router.v3.Router.suppress_envoy_headers>` flag.
    #[prost(bool, tag = "19")]
    pub include_attempt_count_in_response: bool,
    /// Indicates the retry policy for all routes in this virtual host. Note that setting a
    /// route level entry will take precedence over this config and it'll be treated
    /// independently (e.g.: values are not inherited).
    #[prost(message, optional, tag = "16")]
    pub retry_policy: ::core::option::Option<RetryPolicy>,
    /// \[\#not-implemented-hide:\]
    /// Specifies the configuration for retry policy extension. Note that setting a route level entry
    /// will take precedence over this config and it'll be treated independently (e.g.: values are not
    /// inherited). :ref:`Retry policy <envoy_v3_api_field_config.route.v3.VirtualHost.retry_policy>` should not be
    /// set if this field is used.
    #[prost(message, optional, tag = "20")]
    pub retry_policy_typed_config: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
    /// Indicates the hedge policy for all routes in this virtual host. Note that setting a
    /// route level entry will take precedence over this config and it'll be treated
    /// independently (e.g.: values are not inherited).
    #[prost(message, optional, tag = "17")]
    pub hedge_policy: ::core::option::Option<HedgePolicy>,
    /// Decides whether to include the :ref:`x-envoy-is-timeout-retry <config_http_filters_router_x-envoy-is-timeout-retry>`
    /// request header in retries initiated by per try timeouts.
    #[prost(bool, tag = "23")]
    pub include_is_timeout_retry_header: bool,
    /// The maximum bytes which will be buffered for retries and shadowing.
    /// If set and a route-specific limit is not set, the bytes actually buffered will be the minimum
    /// value of this and the listener per_connection_buffer_limit_bytes.
    #[prost(message, optional, tag = "18")]
    pub per_request_buffer_limit_bytes: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// Specify a set of default request mirroring policies for every route under this virtual host.
    /// It takes precedence over the route config mirror policy entirely.
    /// That is, policies are not merged, the most specific non-empty one becomes the mirror policies.
    #[prost(message, repeated, tag = "22")]
    pub request_mirror_policies: ::prost::alloc::vec::Vec<
        route_action::RequestMirrorPolicy,
    >,
    /// The metadata field can be used to provide additional information
    /// about the virtual host. It can be used for configuration, stats, and logging.
    /// The metadata should go under the filter namespace that will need it.
    /// For instance, if the metadata is intended for the Router filter,
    /// the filter name should be specified as `envoy.filters.http.router`.
    #[prost(message, optional, tag = "24")]
    pub metadata: ::core::option::Option<super::super::core::v3::Metadata>,
}
/// Nested message and enum types in `VirtualHost`.
pub mod virtual_host {
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
    pub enum TlsRequirementType {
        /// No TLS requirement for the virtual host.
        None = 0,
        /// External requests must use TLS. If a request is external and it is not
        /// using TLS, a 301 redirect will be sent telling the client to use HTTPS.
        ExternalOnly = 1,
        /// All requests must use TLS. If a request is not using TLS, a 301 redirect
        /// will be sent telling the client to use HTTPS.
        All = 2,
    }
    impl TlsRequirementType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TlsRequirementType::None => "NONE",
                TlsRequirementType::ExternalOnly => "EXTERNAL_ONLY",
                TlsRequirementType::All => "ALL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NONE" => Some(Self::None),
                "EXTERNAL_ONLY" => Some(Self::ExternalOnly),
                "ALL" => Some(Self::All),
                _ => None,
            }
        }
    }
}
/// A filter-defined action type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterAction {
    #[prost(message, optional, tag = "1")]
    pub action: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
}
/// This can be used in route matcher :ref:`VirtualHost.matcher <envoy_v3_api_field_config.route.v3.VirtualHost.matcher>`.
/// When the matcher matches, routes will be matched and run.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteList {
    /// The list of routes that will be matched and run, in order. The first route that matches will be used.
    #[prost(message, repeated, tag = "1")]
    pub routes: ::prost::alloc::vec::Vec<Route>,
}
/// A route is both a specification of how to match a request as well as an indication of what to do
/// next (e.g., redirect, forward, rewrite, etc.).
///
/// .. attention::
///
/// Envoy supports routing on HTTP method via :ref:`header matching <envoy_v3_api_msg_config.route.v3.HeaderMatcher>`.
/// \[\#next-free-field: 20\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Route {
    /// Name for the route.
    #[prost(string, tag = "14")]
    pub name: ::prost::alloc::string::String,
    /// Route matching parameters.
    #[prost(message, optional, tag = "1")]
    pub r#match: ::core::option::Option<RouteMatch>,
    /// The Metadata field can be used to provide additional information
    /// about the route. It can be used for configuration, stats, and logging.
    /// The metadata should go under the filter namespace that will need it.
    /// For instance, if the metadata is intended for the Router filter,
    /// the filter name should be specified as `envoy.filters.http.router`.
    #[prost(message, optional, tag = "4")]
    pub metadata: ::core::option::Option<super::super::core::v3::Metadata>,
    /// Decorator for the matched route.
    #[prost(message, optional, tag = "5")]
    pub decorator: ::core::option::Option<Decorator>,
    /// This field can be used to provide route specific per filter config. The key should match the
    /// :ref:`filter config name <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpFilter.name>`.
    /// See :ref:`Http filter route specific config <arch_overview_http_filters_per_filter_config>`
    /// for details.
    /// \[\#comment: An entry's value may be wrapped in a
    /// :ref:`FilterConfig<envoy_v3_api_msg_config.route.v3.FilterConfig>`
    /// message to specify additional options.\]
    #[prost(map = "string, message", tag = "13")]
    pub typed_per_filter_config: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::super::super::google::protobuf::Any,
    >,
    /// Specifies a set of headers that will be added to requests matching this
    /// route. Headers specified at this level are applied before headers from the
    /// enclosing :ref:`envoy_v3_api_msg_config.route.v3.VirtualHost` and
    /// :ref:`envoy_v3_api_msg_config.route.v3.RouteConfiguration`. For more information, including details on
    /// header value syntax, see the documentation on :ref:`custom request headers <config_http_conn_man_headers_custom_request_headers>`.
    #[prost(message, repeated, tag = "9")]
    pub request_headers_to_add: ::prost::alloc::vec::Vec<
        super::super::core::v3::HeaderValueOption,
    >,
    /// Specifies a list of HTTP headers that should be removed from each request
    /// matching this route.
    #[prost(string, repeated, tag = "12")]
    pub request_headers_to_remove: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Specifies a set of headers that will be added to responses to requests
    /// matching this route. Headers specified at this level are applied before
    /// headers from the enclosing :ref:`envoy_v3_api_msg_config.route.v3.VirtualHost` and
    /// :ref:`envoy_v3_api_msg_config.route.v3.RouteConfiguration`. For more information, including
    /// details on header value syntax, see the documentation on
    /// :ref:`custom request headers <config_http_conn_man_headers_custom_request_headers>`.
    #[prost(message, repeated, tag = "10")]
    pub response_headers_to_add: ::prost::alloc::vec::Vec<
        super::super::core::v3::HeaderValueOption,
    >,
    /// Specifies a list of HTTP headers that should be removed from each response
    /// to requests matching this route.
    #[prost(string, repeated, tag = "11")]
    pub response_headers_to_remove: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Presence of the object defines whether the connection manager's tracing configuration
    /// is overridden by this route specific instance.
    #[prost(message, optional, tag = "15")]
    pub tracing: ::core::option::Option<Tracing>,
    /// The maximum bytes which will be buffered for retries and shadowing.
    /// If set, the bytes actually buffered will be the minimum value of this and the
    /// listener per_connection_buffer_limit_bytes.
    #[prost(message, optional, tag = "16")]
    pub per_request_buffer_limit_bytes: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// The human readable prefix to use when emitting statistics for this endpoint.
    /// The statistics are rooted at vhost.<virtual host name>.route.\<stat_prefix>.
    /// This should be set for highly critical
    /// endpoints that one wishes to get “per-route” statistics on.
    /// If not set, endpoint statistics are not generated.
    ///
    /// The emitted statistics are the same as those documented for :ref:`virtual clusters <config_http_filters_router_vcluster_stats>`.
    ///
    /// .. warning::
    ///
    /// ```text
    /// We do not recommend setting up a stat prefix for
    /// every application endpoint. This is both not easily maintainable and
    /// statistics use a non-trivial amount of memory(approximately 1KiB per route).
    /// ```
    #[prost(string, tag = "19")]
    pub stat_prefix: ::prost::alloc::string::String,
    #[prost(oneof = "route::Action", tags = "2, 3, 7, 17, 18")]
    pub action: ::core::option::Option<route::Action>,
}
/// Nested message and enum types in `Route`.
pub mod route {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        /// Route request to some upstream cluster.
        #[prost(message, tag = "2")]
        Route(super::RouteAction),
        /// Return a redirect.
        #[prost(message, tag = "3")]
        Redirect(super::RedirectAction),
        /// Return an arbitrary HTTP response directly, without proxying.
        #[prost(message, tag = "7")]
        DirectResponse(super::DirectResponseAction),
        /// \[\#not-implemented-hide:\]
        /// A filter-defined action (e.g., it could dynamically generate the RouteAction).
        /// \[\#comment: TODO(samflattery): Remove cleanup in route_fuzz_test.cc when
        /// implemented\]
        #[prost(message, tag = "17")]
        FilterAction(super::FilterAction),
        /// \[\#not-implemented-hide:\]
        /// An action used when the route will generate a response directly,
        /// without forwarding to an upstream host. This will be used in non-proxy
        /// xDS clients like the gRPC server. It could also be used in the future
        /// in Envoy for a filter that directly generates responses for requests.
        #[prost(message, tag = "18")]
        NonForwardingAction(super::NonForwardingAction),
    }
}
/// Compared to the :ref:`cluster <envoy_v3_api_field_config.route.v3.RouteAction.cluster>` field that specifies a
/// single upstream cluster as the target of a request, the :ref:`weighted_clusters <envoy_v3_api_field_config.route.v3.RouteAction.weighted_clusters>` option allows for specification of
/// multiple upstream clusters along with weights that indicate the percentage of
/// traffic to be forwarded to each cluster. The router selects an upstream cluster based on the
/// weights.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WeightedCluster {
    /// Specifies one or more upstream clusters associated with the route.
    #[prost(message, repeated, tag = "1")]
    pub clusters: ::prost::alloc::vec::Vec<weighted_cluster::ClusterWeight>,
    /// Specifies the total weight across all clusters. The sum of all cluster weights must equal this
    /// value, if this is greater than 0.
    /// This field is now deprecated, and the client will use the sum of all
    /// cluster weights. It is up to the management server to supply the correct weights.
    #[deprecated]
    #[prost(message, optional, tag = "3")]
    pub total_weight: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// Specifies the runtime key prefix that should be used to construct the
    /// runtime keys associated with each cluster. When the `runtime_key_prefix` is
    /// specified, the router will look for weights associated with each upstream
    /// cluster under the key `runtime_key_prefix` + `.` + `cluster\[i\].name` where
    /// `cluster\[i\]` denotes an entry in the clusters array field. If the runtime
    /// key for the cluster does not exist, the value specified in the
    /// configuration file will be used as the default weight. See the :ref:`runtime documentation <operations_runtime>` for how key names map to the underlying implementation.
    #[prost(string, tag = "2")]
    pub runtime_key_prefix: ::prost::alloc::string::String,
    #[prost(oneof = "weighted_cluster::RandomValueSpecifier", tags = "4")]
    pub random_value_specifier: ::core::option::Option<
        weighted_cluster::RandomValueSpecifier,
    >,
}
/// Nested message and enum types in `WeightedCluster`.
pub mod weighted_cluster {
    /// \[\#next-free-field: 13\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ClusterWeight {
        /// Only one of `name` and `cluster_header` may be specified.
        /// \[\#next-major-version: Need to add back the validation rule: (validate.rules).string = {min_len: 1}\]
        /// Name of the upstream cluster. The cluster must exist in the
        /// :ref:`cluster manager configuration <config_cluster_manager>`.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// Only one of `name` and `cluster_header` may be specified.
        /// \[\#next-major-version: Need to add back the validation rule: (validate.rules).string = {min_len: 1 }\]
        /// Envoy will determine the cluster to route to by reading the value of the
        /// HTTP header named by cluster_header from the request headers. If the
        /// header is not found or the referenced cluster does not exist, Envoy will
        /// return a 404 response.
        ///
        /// .. attention::
        ///
        /// Internally, Envoy always uses the HTTP/2 `:authority` header to represent the HTTP/1
        /// `Host` header. Thus, if attempting to match on `Host`, match on `:authority` instead.
        ///
        /// .. note::
        ///
        /// If the header appears multiple times only the first value is used.
        #[prost(string, tag = "12")]
        pub cluster_header: ::prost::alloc::string::String,
        /// The weight of the cluster. This value is relative to the other clusters'
        /// weights. When a request matches the route, the choice of an upstream cluster
        /// is determined by its weight. The sum of weights across all
        /// entries in the clusters array must be greater than 0, and must not exceed
        /// uint32_t maximal value (4294967295).
        #[prost(message, optional, tag = "2")]
        pub weight: ::core::option::Option<
            super::super::super::super::super::google::protobuf::UInt32Value,
        >,
        /// Optional endpoint metadata match criteria used by the subset load balancer. Only endpoints in
        /// the upstream cluster with metadata matching what is set in this field will be considered for
        /// load balancing. Note that this will be merged with what's provided in
        /// :ref:`RouteAction.metadata_match <envoy_v3_api_field_config.route.v3.RouteAction.metadata_match>`, with
        /// values here taking precedence. The filter name should be specified as `envoy.lb`.
        #[prost(message, optional, tag = "3")]
        pub metadata_match: ::core::option::Option<
            super::super::super::core::v3::Metadata,
        >,
        /// Specifies a list of headers to be added to requests when this cluster is selected
        /// through the enclosing :ref:`envoy_v3_api_msg_config.route.v3.RouteAction`.
        /// Headers specified at this level are applied before headers from the enclosing
        /// :ref:`envoy_v3_api_msg_config.route.v3.Route`, :ref:`envoy_v3_api_msg_config.route.v3.VirtualHost`, and
        /// :ref:`envoy_v3_api_msg_config.route.v3.RouteConfiguration`. For more information, including details on
        /// header value syntax, see the documentation on :ref:`custom request headers <config_http_conn_man_headers_custom_request_headers>`.
        #[prost(message, repeated, tag = "4")]
        pub request_headers_to_add: ::prost::alloc::vec::Vec<
            super::super::super::core::v3::HeaderValueOption,
        >,
        /// Specifies a list of HTTP headers that should be removed from each request when
        /// this cluster is selected through the enclosing :ref:`envoy_v3_api_msg_config.route.v3.RouteAction`.
        #[prost(string, repeated, tag = "9")]
        pub request_headers_to_remove: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        /// Specifies a list of headers to be added to responses when this cluster is selected
        /// through the enclosing :ref:`envoy_v3_api_msg_config.route.v3.RouteAction`.
        /// Headers specified at this level are applied before headers from the enclosing
        /// :ref:`envoy_v3_api_msg_config.route.v3.Route`, :ref:`envoy_v3_api_msg_config.route.v3.VirtualHost`, and
        /// :ref:`envoy_v3_api_msg_config.route.v3.RouteConfiguration`. For more information, including details on
        /// header value syntax, see the documentation on :ref:`custom request headers <config_http_conn_man_headers_custom_request_headers>`.
        #[prost(message, repeated, tag = "5")]
        pub response_headers_to_add: ::prost::alloc::vec::Vec<
            super::super::super::core::v3::HeaderValueOption,
        >,
        /// Specifies a list of headers to be removed from responses when this cluster is selected
        /// through the enclosing :ref:`envoy_v3_api_msg_config.route.v3.RouteAction`.
        #[prost(string, repeated, tag = "6")]
        pub response_headers_to_remove: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        /// This field can be used to provide weighted cluster specific per filter config. The key should match the
        /// :ref:`filter config name <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpFilter.name>`.
        /// See :ref:`Http filter route specific config <arch_overview_http_filters_per_filter_config>`
        /// for details.
        /// \[\#comment: An entry's value may be wrapped in a
        /// :ref:`FilterConfig<envoy_v3_api_msg_config.route.v3.FilterConfig>`
        /// message to specify additional options.\]
        #[prost(map = "string, message", tag = "10")]
        pub typed_per_filter_config: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            super::super::super::super::super::google::protobuf::Any,
        >,
        #[prost(oneof = "cluster_weight::HostRewriteSpecifier", tags = "11")]
        pub host_rewrite_specifier: ::core::option::Option<
            cluster_weight::HostRewriteSpecifier,
        >,
    }
    /// Nested message and enum types in `ClusterWeight`.
    pub mod cluster_weight {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum HostRewriteSpecifier {
            /// Indicates that during forwarding, the host header will be swapped with
            /// this value.
            #[prost(string, tag = "11")]
            HostRewriteLiteral(::prost::alloc::string::String),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RandomValueSpecifier {
        /// Specifies the header name that is used to look up the random value passed in the request header.
        /// This is used to ensure consistent cluster picking across multiple proxy levels for weighted traffic.
        /// If header is not present or invalid, Envoy will fall back to use the internally generated random value.
        /// This header is expected to be single-valued header as we only want to have one selected value throughout
        /// the process for the consistency. And the value is a unsigned number between 0 and UINT64_MAX.
        #[prost(string, tag = "4")]
        HeaderName(::prost::alloc::string::String),
    }
}
/// Configuration for a cluster specifier plugin.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterSpecifierPlugin {
    /// The name of the plugin and its opaque configuration.
    #[prost(message, optional, tag = "1")]
    pub extension: ::core::option::Option<super::super::core::v3::TypedExtensionConfig>,
    /// If is_optional is not set or is set to false and the plugin defined by this message is not a
    /// supported type, the containing resource is NACKed. If is_optional is set to true, the resource
    /// would not be NACKed for this reason. In this case, routes referencing this plugin's name would
    /// not be treated as an illegal configuration, but would result in a failure if the route is
    /// selected.
    #[prost(bool, tag = "2")]
    pub is_optional: bool,
}
/// \[\#next-free-field: 16\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteMatch {
    /// Indicates that prefix/path matching should be case sensitive. The default
    /// is true. Ignored for safe_regex matching.
    #[prost(message, optional, tag = "4")]
    pub case_sensitive: ::core::option::Option<
        super::super::super::super::google::protobuf::BoolValue,
    >,
    /// Indicates that the route should additionally match on a runtime key. Every time the route
    /// is considered for a match, it must also fall under the percentage of matches indicated by
    /// this field. For some fraction N/D, a random number in the range \[0,D) is selected. If the
    /// number is \<= the value of the numerator N, or if the key is not present, the default
    /// value, the router continues to evaluate the remaining match criteria. A runtime_fraction
    /// route configuration can be used to roll out route changes in a gradual manner without full
    /// code/config deploys. Refer to the :ref:`traffic shifting <config_http_conn_man_route_table_traffic_splitting_shift>` docs for additional documentation.
    ///
    /// .. note::
    ///
    /// ```text
    /// Parsing this field is implemented such that the runtime key's data may be represented
    /// as a FractionalPercent proto represented as JSON/YAML and may also be represented as an
    /// integer with the assumption that the value is an integral percentage out of 100. For
    /// instance, a runtime key lookup returning the value "42" would parse as a FractionalPercent
    /// whose numerator is 42 and denominator is HUNDRED. This preserves legacy semantics.
    /// ```
    #[prost(message, optional, tag = "9")]
    pub runtime_fraction: ::core::option::Option<
        super::super::core::v3::RuntimeFractionalPercent,
    >,
    /// Specifies a set of headers that the route should match on. The router will
    /// check the request’s headers against all the specified headers in the route
    /// config. A match will happen if all the headers in the route are present in
    /// the request with the same values (or based on presence if the value field
    /// is not in the config).
    #[prost(message, repeated, tag = "6")]
    pub headers: ::prost::alloc::vec::Vec<HeaderMatcher>,
    /// Specifies a set of URL query parameters on which the route should
    /// match. The router will check the query string from the `path` header
    /// against all the specified query parameters. If the number of specified
    /// query parameters is nonzero, they all must match the `path` header's
    /// query string for a match to occur. In the event query parameters are
    /// repeated, only the first value for each key will be considered.
    ///
    /// .. note::
    ///
    /// ```text
    /// If query parameters are used to pass request message fields when
    /// `grpc_json_transcoder <<https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/grpc_json_transcoder_filter>`_>
    /// is used, the transcoded message fields maybe different. The query parameters are
    /// url encoded, but the message fields are not. For example, if a query
    /// parameter is "foo%20bar", the message field will be "foo bar".
    /// ```
    #[prost(message, repeated, tag = "7")]
    pub query_parameters: ::prost::alloc::vec::Vec<QueryParameterMatcher>,
    /// If specified, only gRPC requests will be matched. The router will check
    /// that the content-type header has a application/grpc or one of the various
    /// application/grpc+ values.
    #[prost(message, optional, tag = "8")]
    pub grpc: ::core::option::Option<route_match::GrpcRouteMatchOptions>,
    /// If specified, the client tls context will be matched against the defined
    /// match options.
    ///
    /// \[\#next-major-version: unify with RBAC\]
    #[prost(message, optional, tag = "11")]
    pub tls_context: ::core::option::Option<route_match::TlsContextMatchOptions>,
    /// Specifies a set of dynamic metadata matchers on which the route should match.
    /// The router will check the dynamic metadata against all the specified dynamic metadata matchers.
    /// If the number of specified dynamic metadata matchers is nonzero, they all must match the
    /// dynamic metadata for a match to occur.
    #[prost(message, repeated, tag = "13")]
    pub dynamic_metadata: ::prost::alloc::vec::Vec<
        super::super::super::r#type::matcher::v3::MetadataMatcher,
    >,
    #[prost(oneof = "route_match::PathSpecifier", tags = "1, 2, 10, 12, 14, 15")]
    pub path_specifier: ::core::option::Option<route_match::PathSpecifier>,
}
/// Nested message and enum types in `RouteMatch`.
pub mod route_match {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GrpcRouteMatchOptions {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TlsContextMatchOptions {
        /// If specified, the route will match against whether or not a certificate is presented.
        /// If not specified, certificate presentation status (true or false) will not be considered when route matching.
        #[prost(message, optional, tag = "1")]
        pub presented: ::core::option::Option<
            super::super::super::super::super::google::protobuf::BoolValue,
        >,
        /// If specified, the route will match against whether or not a certificate is validated.
        /// If not specified, certificate validation status (true or false) will not be considered when route matching.
        ///
        /// .. warning::
        ///
        /// ```text
        /// Client certificate validation is not currently performed upon TLS session resumption. For
        /// a resumed TLS session the route will match only when ``validated`` is false, regardless of
        /// whether the client TLS certificate is valid.
        ///
        /// The only known workaround for this issue is to disable TLS session resumption entirely, by
        /// setting both :ref:`disable_stateless_session_resumption <envoy_v3_api_field_extensions.transport_sockets.tls.v3.DownstreamTlsContext.disable_stateless_session_resumption>`
        /// and :ref:`disable_stateful_session_resumption <envoy_v3_api_field_extensions.transport_sockets.tls.v3.DownstreamTlsContext.disable_stateful_session_resumption>` on the DownstreamTlsContext.
        /// ```
        #[prost(message, optional, tag = "2")]
        pub validated: ::core::option::Option<
            super::super::super::super::super::google::protobuf::BoolValue,
        >,
    }
    /// An extensible message for matching CONNECT or CONNECT-UDP requests.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConnectMatcher {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PathSpecifier {
        /// If specified, the route is a prefix rule meaning that the prefix must
        /// match the beginning of the `:path` header.
        #[prost(string, tag = "1")]
        Prefix(::prost::alloc::string::String),
        /// If specified, the route is an exact path rule meaning that the path must
        /// exactly match the `:path` header once the query string is removed.
        #[prost(string, tag = "2")]
        Path(::prost::alloc::string::String),
        /// If specified, the route is a regular expression rule meaning that the
        /// regex must match the `:path` header once the query string is removed. The entire path
        /// (without the query string) must match the regex. The rule will not match if only a
        /// subsequence of the `:path` header matches the regex.
        ///
        /// \[\#next-major-version: In the v3 API we should redo how path specification works such
        /// that we utilize StringMatcher, and additionally have consistent options around whether we
        /// strip query strings, do a case sensitive match, etc. In the interim it will be too disruptive
        /// to deprecate the existing options. We should even consider whether we want to do away with
        /// path_specifier entirely and just rely on a set of header matchers which can already match
        /// on :path, etc. The issue with that is it is unclear how to generically deal with query string
        /// stripping. This needs more thought.\]
        #[prost(message, tag = "10")]
        SafeRegex(super::super::super::super::r#type::matcher::v3::RegexMatcher),
        /// If this is used as the matcher, the matcher will only match CONNECT or CONNECT-UDP requests.
        /// Note that this will not match other Extended CONNECT requests (WebSocket and the like) as
        /// they are normalized in Envoy as HTTP/1.1 style upgrades.
        /// This is the only way to match CONNECT requests for HTTP/1.1. For HTTP/2 and HTTP/3,
        /// where Extended CONNECT requests may have a path, the path matchers will work if
        /// there is a path present.
        /// Note that CONNECT support is currently considered alpha in Envoy.
        /// \[\#comment: TODO(htuch): Replace the above comment with an alpha tag.\]
        #[prost(message, tag = "12")]
        ConnectMatcher(ConnectMatcher),
        /// If specified, the route is a path-separated prefix rule meaning that the
        /// `:path` header (without the query string) must either exactly match the
        /// `path_separated_prefix` or have it as a prefix, followed by `/`
        ///
        /// For example, `/api/dev` would match
        /// `/api/dev`, `/api/dev/`, `/api/dev/v1`, and `/api/dev?param=true`
        /// but would not match `/api/developer`
        ///
        /// Expect the value to not contain `?` or `#` and not to end in `/`
        #[prost(string, tag = "14")]
        PathSeparatedPrefix(::prost::alloc::string::String),
        /// \[\#extension-category: envoy.path.match\]
        #[prost(message, tag = "15")]
        PathMatchPolicy(super::super::super::core::v3::TypedExtensionConfig),
    }
}
/// Cors policy configuration.
///
/// .. attention::
///
/// This message has been deprecated. Please use
/// :ref:`CorsPolicy in filter extension <envoy_v3_api_msg_extensions.filters.http.cors.v3.CorsPolicy>`
/// as as alternative.
///
/// \[\#next-free-field: 14\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CorsPolicy {
    /// Specifies string patterns that match allowed origins. An origin is allowed if any of the
    /// string matchers match.
    #[prost(message, repeated, tag = "11")]
    pub allow_origin_string_match: ::prost::alloc::vec::Vec<
        super::super::super::r#type::matcher::v3::StringMatcher,
    >,
    /// Specifies the content for the `access-control-allow-methods` header.
    #[prost(string, tag = "2")]
    pub allow_methods: ::prost::alloc::string::String,
    /// Specifies the content for the `access-control-allow-headers` header.
    #[prost(string, tag = "3")]
    pub allow_headers: ::prost::alloc::string::String,
    /// Specifies the content for the `access-control-expose-headers` header.
    #[prost(string, tag = "4")]
    pub expose_headers: ::prost::alloc::string::String,
    /// Specifies the content for the `access-control-max-age` header.
    #[prost(string, tag = "5")]
    pub max_age: ::prost::alloc::string::String,
    /// Specifies whether the resource allows credentials.
    #[prost(message, optional, tag = "6")]
    pub allow_credentials: ::core::option::Option<
        super::super::super::super::google::protobuf::BoolValue,
    >,
    /// Specifies the % of requests for which the CORS policies will be evaluated and tracked, but not
    /// enforced.
    ///
    /// This field is intended to be used when `filter_enabled` and `enabled` are off. One of those
    /// fields have to explicitly disable the filter in order for this setting to take effect.
    ///
    /// If :ref:`runtime_key <envoy_v3_api_field_config.core.v3.RuntimeFractionalPercent.runtime_key>` is specified,
    /// Envoy will lookup the runtime key to get the percentage of requests for which it will evaluate
    /// and track the request's `Origin` to determine if it's valid but will not enforce any policies.
    #[prost(message, optional, tag = "10")]
    pub shadow_enabled: ::core::option::Option<
        super::super::core::v3::RuntimeFractionalPercent,
    >,
    /// Specify whether allow requests whose target server's IP address is more private than that from
    /// which the request initiator was fetched.
    ///
    /// More details refer to <https://developer.chrome.com/blog/private-network-access-preflight.>
    #[prost(message, optional, tag = "12")]
    pub allow_private_network_access: ::core::option::Option<
        super::super::super::super::google::protobuf::BoolValue,
    >,
    /// Specifies if preflight requests not matching the configured allowed origin should be forwarded
    /// to the upstream. Default is true.
    #[prost(message, optional, tag = "13")]
    pub forward_not_matching_preflights: ::core::option::Option<
        super::super::super::super::google::protobuf::BoolValue,
    >,
    #[prost(oneof = "cors_policy::EnabledSpecifier", tags = "9")]
    pub enabled_specifier: ::core::option::Option<cors_policy::EnabledSpecifier>,
}
/// Nested message and enum types in `CorsPolicy`.
pub mod cors_policy {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EnabledSpecifier {
        /// Specifies the % of requests for which the CORS filter is enabled.
        ///
        /// If neither `enabled`, `filter_enabled`, nor `shadow_enabled` are specified, the CORS
        /// filter will be enabled for 100% of the requests.
        ///
        /// If :ref:`runtime_key <envoy_v3_api_field_config.core.v3.RuntimeFractionalPercent.runtime_key>` is
        /// specified, Envoy will lookup the runtime key to get the percentage of requests to filter.
        #[prost(message, tag = "9")]
        FilterEnabled(super::super::super::core::v3::RuntimeFractionalPercent),
    }
}
/// \[\#next-free-field: 42\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteAction {
    /// The HTTP status code to use when configured cluster is not found.
    /// The default response code is 503 Service Unavailable.
    #[prost(enumeration = "route_action::ClusterNotFoundResponseCode", tag = "20")]
    pub cluster_not_found_response_code: i32,
    /// Optional endpoint metadata match criteria used by the subset load balancer. Only endpoints
    /// in the upstream cluster with metadata matching what's set in this field will be considered
    /// for load balancing. If using :ref:`weighted_clusters <envoy_v3_api_field_config.route.v3.RouteAction.weighted_clusters>`, metadata will be merged, with values
    /// provided there taking precedence. The filter name should be specified as `envoy.lb`.
    #[prost(message, optional, tag = "4")]
    pub metadata_match: ::core::option::Option<super::super::core::v3::Metadata>,
    /// Indicates that during forwarding, the matched prefix (or path) should be
    /// swapped with this value. This option allows application URLs to be rooted
    /// at a different path from those exposed at the reverse proxy layer. The router filter will
    /// place the original path before rewrite into the :ref:`x-envoy-original-path <config_http_filters_router_x-envoy-original-path>` header.
    ///
    /// Only one of :ref:`regex_rewrite <envoy_v3_api_field_config.route.v3.RouteAction.regex_rewrite>`
    /// :ref:`path_rewrite_policy <envoy_v3_api_field_config.route.v3.RouteAction.path_rewrite_policy>`,
    /// or :ref:`prefix_rewrite <envoy_v3_api_field_config.route.v3.RouteAction.prefix_rewrite>` may be specified.
    ///
    /// .. attention::
    ///
    /// Pay careful attention to the use of trailing slashes in the
    /// :ref:`route's match <envoy_v3_api_field_config.route.v3.Route.match>` prefix value.
    /// Stripping a prefix from a path requires multiple Routes to handle all cases. For example,
    /// rewriting `/prefix` to `/` and `/prefix/etc` to `/etc` cannot be done in a single
    /// :ref:`Route <envoy_v3_api_msg_config.route.v3.Route>`, as shown by the below config entries:
    ///
    /// .. code-block:: yaml
    ///
    /// ```text
    /// - match:
    ///      prefix: "/prefix/"
    ///    route:
    ///      prefix_rewrite: "/"
    /// - match:
    ///      prefix: "/prefix"
    ///    route:
    ///      prefix_rewrite: "/"
    /// ```
    ///
    /// Having above entries in the config, requests to `/prefix` will be stripped to `/`, while
    /// requests to `/prefix/etc` will be stripped to `/etc`.
    #[prost(string, tag = "5")]
    pub prefix_rewrite: ::prost::alloc::string::String,
    /// Indicates that during forwarding, portions of the path that match the
    /// pattern should be rewritten, even allowing the substitution of capture
    /// groups from the pattern into the new path as specified by the rewrite
    /// substitution string. This is useful to allow application paths to be
    /// rewritten in a way that is aware of segments with variable content like
    /// identifiers. The router filter will place the original path as it was
    /// before the rewrite into the :ref:`x-envoy-original-path <config_http_filters_router_x-envoy-original-path>` header.
    ///
    /// Only one of :ref:`regex_rewrite <envoy_v3_api_field_config.route.v3.RouteAction.regex_rewrite>`,
    /// :ref:`prefix_rewrite <envoy_v3_api_field_config.route.v3.RouteAction.prefix_rewrite>`, or
    /// :ref:`path_rewrite_policy <envoy_v3_api_field_config.route.v3.RouteAction.path_rewrite_policy>`\]
    /// may be specified.
    ///
    /// Examples using Google's `RE2 <<https://github.com/google/re2>`\_> engine:
    ///
    /// * The path pattern `^/service/(\[^/\]+)(/.*)$` paired with a substitution
    ///   string of `\2/instance/\1` would transform `/service/foo/v1/api`
    ///   into `/v1/api/instance/foo`.
    ///
    /// * The pattern `one` paired with a substitution string of `two` would
    ///   transform `/xxx/one/yyy/one/zzz` into `/xxx/two/yyy/two/zzz`.
    ///
    /// * The pattern `^(.*?)one(.*)$` paired with a substitution string of
    ///   `\1two\2` would replace only the first occurrence of `one`,
    ///   transforming path `/xxx/one/yyy/one/zzz` into `/xxx/two/yyy/one/zzz`.
    ///
    /// * The pattern `(?i)/xxx/` paired with a substitution string of `/yyy/`
    ///   would do a case-insensitive match and transform path `/aaa/XxX/bbb` to
    ///   `/aaa/yyy/bbb`.
    #[prost(message, optional, tag = "32")]
    pub regex_rewrite: ::core::option::Option<
        super::super::super::r#type::matcher::v3::RegexMatchAndSubstitute,
    >,
    /// \[\#extension-category: envoy.path.rewrite\]
    #[prost(message, optional, tag = "41")]
    pub path_rewrite_policy: ::core::option::Option<
        super::super::core::v3::TypedExtensionConfig,
    >,
    /// If set, then a host rewrite action (one of
    /// :ref:`host_rewrite_literal <envoy_v3_api_field_config.route.v3.RouteAction.host_rewrite_literal>`,
    /// :ref:`auto_host_rewrite <envoy_v3_api_field_config.route.v3.RouteAction.auto_host_rewrite>`,
    /// :ref:`host_rewrite_header <envoy_v3_api_field_config.route.v3.RouteAction.host_rewrite_header>`, or
    /// :ref:`host_rewrite_path_regex <envoy_v3_api_field_config.route.v3.RouteAction.host_rewrite_path_regex>`)
    /// causes the original value of the host header, if any, to be appended to the
    /// :ref:`config_http_conn_man_headers_x-forwarded-host` HTTP header if it is different to the last value appended.
    #[prost(bool, tag = "38")]
    pub append_x_forwarded_host: bool,
    /// Specifies the upstream timeout for the route. If not specified, the default is 15s. This
    /// spans between the point at which the entire downstream request (i.e. end-of-stream) has been
    /// processed and when the upstream response has been completely processed. A value of 0 will
    /// disable the route's timeout.
    ///
    /// .. note::
    ///
    /// This timeout includes all retries. See also
    /// :ref:`config_http_filters_router_x-envoy-upstream-rq-timeout-ms`,
    /// :ref:`config_http_filters_router_x-envoy-upstream-rq-per-try-timeout-ms`, and the
    /// :ref:`retry overview <arch_overview_http_routing_retry>`.
    #[prost(message, optional, tag = "8")]
    pub timeout: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    /// Specifies the idle timeout for the route. If not specified, there is no per-route idle timeout,
    /// although the connection manager wide :ref:`stream_idle_timeout <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.stream_idle_timeout>`
    /// will still apply. A value of 0 will completely disable the route's idle timeout, even if a
    /// connection manager stream idle timeout is configured.
    ///
    /// The idle timeout is distinct to :ref:`timeout <envoy_v3_api_field_config.route.v3.RouteAction.timeout>`, which provides an upper bound
    /// on the upstream response time; :ref:`idle_timeout <envoy_v3_api_field_config.route.v3.RouteAction.idle_timeout>` instead bounds the amount
    /// of time the request's stream may be idle.
    ///
    /// After header decoding, the idle timeout will apply on downstream and
    /// upstream request events. Each time an encode/decode event for headers or
    /// data is processed for the stream, the timer will be reset. If the timeout
    /// fires, the stream is terminated with a 408 Request Timeout error code if no
    /// upstream response header has been received, otherwise a stream reset
    /// occurs.
    ///
    /// If the :ref:`overload action <config_overload_manager_overload_actions>` "envoy.overload_actions.reduce_timeouts"
    /// is configured, this timeout is scaled according to the value for
    /// :ref:`HTTP_DOWNSTREAM_STREAM_IDLE <envoy_v3_api_enum_value_config.overload.v3.ScaleTimersOverloadActionConfig.TimerType.HTTP_DOWNSTREAM_STREAM_IDLE>`.
    #[prost(message, optional, tag = "24")]
    pub idle_timeout: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    /// Specifies how to send request over TLS early data.
    /// If absent, allows `safe HTTP requests <<https://www.rfc-editor.org/rfc/rfc7231#section-4.2.1>`\_> to be sent on early data.
    /// \[\#extension-category: envoy.route.early_data_policy\]
    #[prost(message, optional, tag = "40")]
    pub early_data_policy: ::core::option::Option<
        super::super::core::v3::TypedExtensionConfig,
    >,
    /// Indicates that the route has a retry policy. Note that if this is set,
    /// it'll take precedence over the virtual host level retry policy entirely
    /// (e.g.: policies are not merged, most internal one becomes the enforced policy).
    #[prost(message, optional, tag = "9")]
    pub retry_policy: ::core::option::Option<RetryPolicy>,
    /// \[\#not-implemented-hide:\]
    /// Specifies the configuration for retry policy extension. Note that if this is set, it'll take
    /// precedence over the virtual host level retry policy entirely (e.g.: policies are not merged,
    /// most internal one becomes the enforced policy). :ref:`Retry policy <envoy_v3_api_field_config.route.v3.VirtualHost.retry_policy>`
    /// should not be set if this field is used.
    #[prost(message, optional, tag = "33")]
    pub retry_policy_typed_config: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
    /// Specify a set of route request mirroring policies.
    /// It takes precedence over the virtual host and route config mirror policy entirely.
    /// That is, policies are not merged, the most specific non-empty one becomes the mirror policies.
    #[prost(message, repeated, tag = "30")]
    pub request_mirror_policies: ::prost::alloc::vec::Vec<
        route_action::RequestMirrorPolicy,
    >,
    /// Optionally specifies the :ref:`routing priority <arch_overview_http_routing_priority>`.
    #[prost(enumeration = "super::super::core::v3::RoutingPriority", tag = "11")]
    pub priority: i32,
    /// Specifies a set of rate limit configurations that could be applied to the
    /// route.
    #[prost(message, repeated, tag = "13")]
    pub rate_limits: ::prost::alloc::vec::Vec<RateLimit>,
    /// Specifies if the rate limit filter should include the virtual host rate
    /// limits. By default, if the route configured rate limits, the virtual host
    /// :ref:`rate_limits <envoy_v3_api_field_config.route.v3.VirtualHost.rate_limits>` are not applied to the
    /// request.
    ///
    /// This field is deprecated. Please use :ref:`vh_rate_limits <envoy_v3_api_field_extensions.filters.http.ratelimit.v3.RateLimitPerRoute.vh_rate_limits>`
    #[deprecated]
    #[prost(message, optional, tag = "14")]
    pub include_vh_rate_limits: ::core::option::Option<
        super::super::super::super::google::protobuf::BoolValue,
    >,
    /// Specifies a list of hash policies to use for ring hash load balancing. Each
    /// hash policy is evaluated individually and the combined result is used to
    /// route the request. The method of combination is deterministic such that
    /// identical lists of hash policies will produce the same hash. Since a hash
    /// policy examines specific parts of a request, it can fail to produce a hash
    /// (i.e. if the hashed header is not present). If (and only if) all configured
    /// hash policies fail to generate a hash, no hash will be produced for
    /// the route. In this case, the behavior is the same as if no hash policies
    /// were specified (i.e. the ring hash load balancer will choose a random
    /// backend). If a hash policy has the "terminal" attribute set to true, and
    /// there is already a hash generated, the hash is returned immediately,
    /// ignoring the rest of the hash policy list.
    #[prost(message, repeated, tag = "15")]
    pub hash_policy: ::prost::alloc::vec::Vec<route_action::HashPolicy>,
    /// Indicates that the route has a CORS policy. This field is ignored if related cors policy is
    /// found in the :ref:`Route.typed_per_filter_config<envoy_v3_api_field_config.route.v3.Route.typed_per_filter_config>` or
    /// :ref:`WeightedCluster.ClusterWeight.typed_per_filter_config<envoy_v3_api_field_config.route.v3.WeightedCluster.ClusterWeight.typed_per_filter_config>`.
    ///
    /// .. attention::
    ///
    /// This option has been deprecated. Please use
    /// :ref:`Route.typed_per_filter_config<envoy_v3_api_field_config.route.v3.Route.typed_per_filter_config>` or
    /// :ref:`WeightedCluster.ClusterWeight.typed_per_filter_config<envoy_v3_api_field_config.route.v3.WeightedCluster.ClusterWeight.typed_per_filter_config>`
    /// to configure the CORS HTTP filter.
    #[deprecated]
    #[prost(message, optional, tag = "17")]
    pub cors: ::core::option::Option<CorsPolicy>,
    /// Deprecated by :ref:`grpc_timeout_header_max <envoy_v3_api_field_config.route.v3.RouteAction.MaxStreamDuration.grpc_timeout_header_max>`
    /// If present, and the request is a gRPC request, use the
    /// `grpc-timeout header <<https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-HTTP2.md>`\_,>
    /// or its default value (infinity) instead of
    /// :ref:`timeout <envoy_v3_api_field_config.route.v3.RouteAction.timeout>`, but limit the applied timeout
    /// to the maximum value specified here. If configured as 0, the maximum allowed timeout for
    /// gRPC requests is infinity. If not configured at all, the `grpc-timeout` header is not used
    /// and gRPC requests time out like any other requests using
    /// :ref:`timeout <envoy_v3_api_field_config.route.v3.RouteAction.timeout>` or its default.
    /// This can be used to prevent unexpected upstream request timeouts due to potentially long
    /// time gaps between gRPC request and response in gRPC streaming mode.
    ///
    /// .. note::
    ///
    /// ```text
    /// If a timeout is specified using :ref:`config_http_filters_router_x-envoy-upstream-rq-timeout-ms`, it takes
    /// precedence over `grpc-timeout header <<https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-HTTP2.md>`_,> when
    /// both are present. See also
    /// :ref:`config_http_filters_router_x-envoy-upstream-rq-timeout-ms`,
    /// :ref:`config_http_filters_router_x-envoy-upstream-rq-per-try-timeout-ms`, and the
    /// :ref:`retry overview <arch_overview_http_routing_retry>`.
    /// ```
    #[deprecated]
    #[prost(message, optional, tag = "23")]
    pub max_grpc_timeout: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    /// Deprecated by :ref:`grpc_timeout_header_offset <envoy_v3_api_field_config.route.v3.RouteAction.MaxStreamDuration.grpc_timeout_header_offset>`.
    /// If present, Envoy will adjust the timeout provided by the `grpc-timeout` header by subtracting
    /// the provided duration from the header. This is useful in allowing Envoy to set its global
    /// timeout to be less than that of the deadline imposed by the calling client, which makes it more
    /// likely that Envoy will handle the timeout instead of having the call canceled by the client.
    /// The offset will only be applied if the provided grpc_timeout is greater than the offset. This
    /// ensures that the offset will only ever decrease the timeout and never set it to 0 (meaning
    /// infinity).
    #[deprecated]
    #[prost(message, optional, tag = "28")]
    pub grpc_timeout_offset: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    #[prost(message, repeated, tag = "25")]
    pub upgrade_configs: ::prost::alloc::vec::Vec<route_action::UpgradeConfig>,
    /// If present, Envoy will try to follow an upstream redirect response instead of proxying the
    /// response back to the downstream. An upstream redirect response is defined
    /// by :ref:`redirect_response_codes <envoy_v3_api_field_config.route.v3.InternalRedirectPolicy.redirect_response_codes>`.
    #[prost(message, optional, tag = "34")]
    pub internal_redirect_policy: ::core::option::Option<InternalRedirectPolicy>,
    #[deprecated]
    #[prost(enumeration = "route_action::InternalRedirectAction", tag = "26")]
    pub internal_redirect_action: i32,
    /// An internal redirect is handled, iff the number of previous internal redirects that a
    /// downstream request has encountered is lower than this value, and
    /// :ref:`internal_redirect_action <envoy_v3_api_field_config.route.v3.RouteAction.internal_redirect_action>`
    /// is set to :ref:`HANDLE_INTERNAL_REDIRECT <envoy_v3_api_enum_value_config.route.v3.RouteAction.InternalRedirectAction.HANDLE_INTERNAL_REDIRECT>`
    /// In the case where a downstream request is bounced among multiple routes by internal redirect,
    /// the first route that hits this threshold, or has
    /// :ref:`internal_redirect_action <envoy_v3_api_field_config.route.v3.RouteAction.internal_redirect_action>`
    /// set to
    /// :ref:`PASS_THROUGH_INTERNAL_REDIRECT <envoy_v3_api_enum_value_config.route.v3.RouteAction.InternalRedirectAction.PASS_THROUGH_INTERNAL_REDIRECT>`
    /// will pass the redirect back to downstream.
    ///
    /// If not specified, at most one redirect will be followed.
    #[deprecated]
    #[prost(message, optional, tag = "31")]
    pub max_internal_redirects: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// Indicates that the route has a hedge policy. Note that if this is set,
    /// it'll take precedence over the virtual host level hedge policy entirely
    /// (e.g.: policies are not merged, most internal one becomes the enforced policy).
    #[prost(message, optional, tag = "27")]
    pub hedge_policy: ::core::option::Option<HedgePolicy>,
    /// Specifies the maximum stream duration for this route.
    #[prost(message, optional, tag = "36")]
    pub max_stream_duration: ::core::option::Option<route_action::MaxStreamDuration>,
    #[prost(oneof = "route_action::ClusterSpecifier", tags = "1, 2, 3, 37, 39")]
    pub cluster_specifier: ::core::option::Option<route_action::ClusterSpecifier>,
    #[prost(oneof = "route_action::HostRewriteSpecifier", tags = "6, 7, 29, 35")]
    pub host_rewrite_specifier: ::core::option::Option<
        route_action::HostRewriteSpecifier,
    >,
}
/// Nested message and enum types in `RouteAction`.
pub mod route_action {
    /// The router is capable of shadowing traffic from one cluster to another. The current
    /// implementation is "fire and forget," meaning Envoy will not wait for the shadow cluster to
    /// respond before returning the response from the primary cluster. All normal statistics are
    /// collected for the shadow cluster making this feature useful for testing.
    ///
    /// During shadowing, the host/authority header is altered such that `-shadow` is appended. This is
    /// useful for logging. For example, `cluster1` becomes `cluster1-shadow`. This behavior can be
    /// disabled by setting `disable_shadow_host_suffix_append` to `true`.
    ///
    /// .. note::
    ///
    /// Shadowing will not be triggered if the primary cluster does not exist.
    ///
    /// .. note::
    ///
    /// Shadowing doesn't support Http CONNECT and upgrades.
    /// \[\#next-free-field: 7\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RequestMirrorPolicy {
        /// Only one of `cluster` and `cluster_header` can be specified.
        /// \[\#next-major-version: Need to add back the validation rule: (validate.rules).string = {min_len: 1}\]
        /// Specifies the cluster that requests will be mirrored to. The cluster must
        /// exist in the cluster manager configuration.
        #[prost(string, tag = "1")]
        pub cluster: ::prost::alloc::string::String,
        /// Only one of `cluster` and `cluster_header` can be specified.
        /// Envoy will determine the cluster to route to by reading the value of the
        /// HTTP header named by cluster_header from the request headers. Only the first value in header is used,
        /// and no shadow request will happen if the value is not found in headers. Envoy will not wait for
        /// the shadow cluster to respond before returning the response from the primary cluster.
        ///
        /// .. attention::
        ///
        /// Internally, Envoy always uses the HTTP/2 `:authority` header to represent the HTTP/1
        /// `Host` header. Thus, if attempting to match on `Host`, match on `:authority` instead.
        ///
        /// .. note::
        ///
        /// If the header appears multiple times only the first value is used.
        #[prost(string, tag = "5")]
        pub cluster_header: ::prost::alloc::string::String,
        /// If not specified, all requests to the target cluster will be mirrored.
        ///
        /// If specified, this field takes precedence over the `runtime_key` field and requests must also
        /// fall under the percentage of matches indicated by this field.
        ///
        /// For some fraction N/D, a random number in the range \[0,D) is selected. If the
        /// number is \<= the value of the numerator N, or if the key is not present, the default
        /// value, the request will be mirrored.
        #[prost(message, optional, tag = "3")]
        pub runtime_fraction: ::core::option::Option<
            super::super::super::core::v3::RuntimeFractionalPercent,
        >,
        /// Determines if the trace span should be sampled. Defaults to true.
        #[prost(message, optional, tag = "4")]
        pub trace_sampled: ::core::option::Option<
            super::super::super::super::super::google::protobuf::BoolValue,
        >,
        /// Disables appending the `-shadow` suffix to the shadowed `Host` header. Defaults to `false`.
        #[prost(bool, tag = "6")]
        pub disable_shadow_host_suffix_append: bool,
    }
    /// Specifies the route's hashing policy if the upstream cluster uses a hashing :ref:`load balancer <arch_overview_load_balancing_types>`.
    /// \[\#next-free-field: 7\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HashPolicy {
        /// The flag that short-circuits the hash computing. This field provides a
        /// 'fallback' style of configuration: "if a terminal policy doesn't work,
        /// fallback to rest of the policy list", it saves time when the terminal
        /// policy works.
        ///
        /// If true, and there is already a hash computed, ignore rest of the
        /// list of hash polices.
        /// For example, if the following hash methods are configured:
        ///
        /// ========= ========
        /// specifier terminal
        /// ========= ========
        /// Header A  true
        /// Header B  false
        /// Header C  false
        /// ========= ========
        ///
        /// The generateHash process ends if policy "header A" generates a hash, as
        /// it's a terminal policy.
        #[prost(bool, tag = "4")]
        pub terminal: bool,
        #[prost(oneof = "hash_policy::PolicySpecifier", tags = "1, 2, 3, 5, 6")]
        pub policy_specifier: ::core::option::Option<hash_policy::PolicySpecifier>,
    }
    /// Nested message and enum types in `HashPolicy`.
    pub mod hash_policy {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Header {
            /// The name of the request header that will be used to obtain the hash
            /// key. If the request header is not present, no hash will be produced.
            #[prost(string, tag = "1")]
            pub header_name: ::prost::alloc::string::String,
            /// If specified, the request header value will be rewritten and used
            /// to produce the hash key.
            #[prost(message, optional, tag = "2")]
            pub regex_rewrite: ::core::option::Option<
                super::super::super::super::super::r#type::matcher::v3::RegexMatchAndSubstitute,
            >,
        }
        /// CookieAttribute defines an API for adding additional attributes for a HTTP cookie.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CookieAttribute {
            /// The name of the cookie attribute.
            #[prost(string, tag = "1")]
            pub name: ::prost::alloc::string::String,
            /// The optional value of the cookie attribute.
            #[prost(string, tag = "2")]
            pub value: ::prost::alloc::string::String,
        }
        /// Envoy supports two types of cookie affinity:
        ///
        /// 1. Passive. Envoy takes a cookie that's present in the cookies header and
        ///    hashes on its value.
        ///
        /// 1. Generated. Envoy generates and sets a cookie with an expiration (TTL)
        ///    on the first request from the client in its response to the client,
        ///    based on the endpoint the request gets sent to. The client then
        ///    presents this on the next and all subsequent requests. The hash of
        ///    this is sufficient to ensure these requests get sent to the same
        ///    endpoint. The cookie is generated by hashing the source and
        ///    destination ports and addresses so that multiple independent HTTP2
        ///    streams on the same connection will independently receive the same
        ///    cookie, even if they arrive at the Envoy simultaneously.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Cookie {
            /// The name of the cookie that will be used to obtain the hash key. If the
            /// cookie is not present and ttl below is not set, no hash will be
            /// produced.
            #[prost(string, tag = "1")]
            pub name: ::prost::alloc::string::String,
            /// If specified, a cookie with the TTL will be generated if the cookie is
            /// not present. If the TTL is present and zero, the generated cookie will
            /// be a session cookie.
            #[prost(message, optional, tag = "2")]
            pub ttl: ::core::option::Option<
                super::super::super::super::super::super::google::protobuf::Duration,
            >,
            /// The name of the path for the cookie. If no path is specified here, no path
            /// will be set for the cookie.
            #[prost(string, tag = "3")]
            pub path: ::prost::alloc::string::String,
            /// Additional attributes for the cookie. They will be used when generating a new cookie.
            #[prost(message, repeated, tag = "4")]
            pub attributes: ::prost::alloc::vec::Vec<CookieAttribute>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ConnectionProperties {
            /// Hash on source IP address.
            #[prost(bool, tag = "1")]
            pub source_ip: bool,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct QueryParameter {
            /// The name of the URL query parameter that will be used to obtain the hash
            /// key. If the parameter is not present, no hash will be produced. Query
            /// parameter names are case-sensitive. If query parameters are repeated, only
            /// the first value will be considered.
            #[prost(string, tag = "1")]
            pub name: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FilterState {
            /// The name of the Object in the per-request filterState, which is an
            /// Envoy::Hashable object. If there is no data associated with the key,
            /// or the stored object is not Envoy::Hashable, no hash will be produced.
            #[prost(string, tag = "1")]
            pub key: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum PolicySpecifier {
            /// Header hash policy.
            #[prost(message, tag = "1")]
            Header(Header),
            /// Cookie hash policy.
            #[prost(message, tag = "2")]
            Cookie(Cookie),
            /// Connection properties hash policy.
            #[prost(message, tag = "3")]
            ConnectionProperties(ConnectionProperties),
            /// Query parameter hash policy.
            #[prost(message, tag = "5")]
            QueryParameter(QueryParameter),
            /// Filter state hash policy.
            #[prost(message, tag = "6")]
            FilterState(FilterState),
        }
    }
    /// Allows enabling and disabling upgrades on a per-route basis.
    /// This overrides any enabled/disabled upgrade filter chain specified in the
    /// HttpConnectionManager
    /// :ref:`upgrade_configs <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.upgrade_configs>`
    /// but does not affect any custom filter chain specified there.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpgradeConfig {
        /// The case-insensitive name of this upgrade, e.g. "websocket".
        /// For each upgrade type present in upgrade_configs, requests with
        /// Upgrade: \[upgrade_type\] will be proxied upstream.
        #[prost(string, tag = "1")]
        pub upgrade_type: ::prost::alloc::string::String,
        /// Determines if upgrades are available on this route. Defaults to true.
        #[prost(message, optional, tag = "2")]
        pub enabled: ::core::option::Option<
            super::super::super::super::super::google::protobuf::BoolValue,
        >,
        /// Configuration for sending data upstream as a raw data payload. This is used for
        /// CONNECT requests, when forwarding CONNECT payload as raw TCP.
        /// Note that CONNECT support is currently considered alpha in Envoy.
        /// \[\#comment: TODO(htuch): Replace the above comment with an alpha tag.\]
        #[prost(message, optional, tag = "3")]
        pub connect_config: ::core::option::Option<upgrade_config::ConnectConfig>,
    }
    /// Nested message and enum types in `UpgradeConfig`.
    pub mod upgrade_config {
        /// Configuration for sending data upstream as a raw data payload. This is used for
        /// CONNECT or POST requests, when forwarding request payload as raw TCP.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ConnectConfig {
            /// If present, the proxy protocol header will be prepended to the CONNECT payload sent upstream.
            #[prost(message, optional, tag = "1")]
            pub proxy_protocol_config: ::core::option::Option<
                super::super::super::super::core::v3::ProxyProtocolConfig,
            >,
            /// If set, the route will also allow forwarding POST payload as raw TCP.
            #[prost(bool, tag = "2")]
            pub allow_post: bool,
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MaxStreamDuration {
        /// Specifies the maximum duration allowed for streams on the route. If not specified, the value
        /// from the :ref:`max_stream_duration <envoy_v3_api_field_config.core.v3.HttpProtocolOptions.max_stream_duration>` field in
        /// :ref:`HttpConnectionManager.common_http_protocol_options <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.common_http_protocol_options>`
        /// is used. If this field is set explicitly to zero, any
        /// HttpConnectionManager max_stream_duration timeout will be disabled for
        /// this route.
        #[prost(message, optional, tag = "1")]
        pub max_stream_duration: ::core::option::Option<
            super::super::super::super::super::google::protobuf::Duration,
        >,
        /// If present, and the request contains a `grpc-timeout header <<https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-HTTP2.md>`\_,> use that value as the
        /// `max_stream_duration`, but limit the applied timeout to the maximum value specified here.
        /// If set to 0, the `grpc-timeout` header is used without modification.
        #[prost(message, optional, tag = "2")]
        pub grpc_timeout_header_max: ::core::option::Option<
            super::super::super::super::super::google::protobuf::Duration,
        >,
        /// If present, Envoy will adjust the timeout provided by the `grpc-timeout` header by
        /// subtracting the provided duration from the header. This is useful for allowing Envoy to set
        /// its global timeout to be less than that of the deadline imposed by the calling client, which
        /// makes it more likely that Envoy will handle the timeout instead of having the call canceled
        /// by the client. If, after applying the offset, the resulting timeout is zero or negative,
        /// the stream will timeout immediately.
        #[prost(message, optional, tag = "3")]
        pub grpc_timeout_header_offset: ::core::option::Option<
            super::super::super::super::super::google::protobuf::Duration,
        >,
    }
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
    pub enum ClusterNotFoundResponseCode {
        /// HTTP status code - 503 Service Unavailable.
        ServiceUnavailable = 0,
        /// HTTP status code - 404 Not Found.
        NotFound = 1,
        /// HTTP status code - 500 Internal Server Error.
        InternalServerError = 2,
    }
    impl ClusterNotFoundResponseCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ClusterNotFoundResponseCode::ServiceUnavailable => "SERVICE_UNAVAILABLE",
                ClusterNotFoundResponseCode::NotFound => "NOT_FOUND",
                ClusterNotFoundResponseCode::InternalServerError => {
                    "INTERNAL_SERVER_ERROR"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SERVICE_UNAVAILABLE" => Some(Self::ServiceUnavailable),
                "NOT_FOUND" => Some(Self::NotFound),
                "INTERNAL_SERVER_ERROR" => Some(Self::InternalServerError),
                _ => None,
            }
        }
    }
    /// Configures :ref:`internal redirect <arch_overview_internal_redirects>` behavior.
    /// \[\#next-major-version: remove this definition - it's defined in the InternalRedirectPolicy message.\]
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
    pub enum InternalRedirectAction {
        PassThroughInternalRedirect = 0,
        HandleInternalRedirect = 1,
    }
    impl InternalRedirectAction {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InternalRedirectAction::PassThroughInternalRedirect => {
                    "PASS_THROUGH_INTERNAL_REDIRECT"
                }
                InternalRedirectAction::HandleInternalRedirect => {
                    "HANDLE_INTERNAL_REDIRECT"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PASS_THROUGH_INTERNAL_REDIRECT" => {
                    Some(Self::PassThroughInternalRedirect)
                }
                "HANDLE_INTERNAL_REDIRECT" => Some(Self::HandleInternalRedirect),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ClusterSpecifier {
        /// Indicates the upstream cluster to which the request should be routed
        /// to.
        #[prost(string, tag = "1")]
        Cluster(::prost::alloc::string::String),
        /// Envoy will determine the cluster to route to by reading the value of the
        /// HTTP header named by cluster_header from the request headers. If the
        /// header is not found or the referenced cluster does not exist, Envoy will
        /// return a 404 response.
        ///
        /// .. attention::
        ///
        /// Internally, Envoy always uses the HTTP/2 `:authority` header to represent the HTTP/1
        /// `Host` header. Thus, if attempting to match on `Host`, match on `:authority` instead.
        ///
        /// .. note::
        ///
        /// If the header appears multiple times only the first value is used.
        #[prost(string, tag = "2")]
        ClusterHeader(::prost::alloc::string::String),
        /// Multiple upstream clusters can be specified for a given route. The
        /// request is routed to one of the upstream clusters based on weights
        /// assigned to each cluster. See
        /// :ref:`traffic splitting <config_http_conn_man_route_table_traffic_splitting_split>`
        /// for additional documentation.
        #[prost(message, tag = "3")]
        WeightedClusters(super::WeightedCluster),
        /// Name of the cluster specifier plugin to use to determine the cluster for requests on this route.
        /// The cluster specifier plugin name must be defined in the associated
        /// :ref:`cluster specifier plugins <envoy_v3_api_field_config.route.v3.RouteConfiguration.cluster_specifier_plugins>`
        /// in the :ref:`name <envoy_v3_api_field_config.core.v3.TypedExtensionConfig.name>` field.
        #[prost(string, tag = "37")]
        ClusterSpecifierPlugin(::prost::alloc::string::String),
        /// Custom cluster specifier plugin configuration to use to determine the cluster for requests
        /// on this route.
        #[prost(message, tag = "39")]
        InlineClusterSpecifierPlugin(super::ClusterSpecifierPlugin),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum HostRewriteSpecifier {
        /// Indicates that during forwarding, the host header will be swapped with
        /// this value. Using this option will append the
        /// :ref:`config_http_conn_man_headers_x-forwarded-host` header if
        /// :ref:`append_x_forwarded_host <envoy_v3_api_field_config.route.v3.RouteAction.append_x_forwarded_host>`
        /// is set.
        #[prost(string, tag = "6")]
        HostRewriteLiteral(::prost::alloc::string::String),
        /// Indicates that during forwarding, the host header will be swapped with
        /// the hostname of the upstream host chosen by the cluster manager. This
        /// option is applicable only when the destination cluster for a route is of
        /// type `strict_dns` or `logical_dns`,
        /// or when :ref:`hostname <envoy_v3_api_field_config.endpoint.v3.Endpoint.hostname>`
        /// field is not empty. Setting this to true with other cluster types
        /// has no effect. Using this option will append the
        /// :ref:`config_http_conn_man_headers_x-forwarded-host` header if
        /// :ref:`append_x_forwarded_host <envoy_v3_api_field_config.route.v3.RouteAction.append_x_forwarded_host>`
        /// is set.
        #[prost(message, tag = "7")]
        AutoHostRewrite(super::super::super::super::super::google::protobuf::BoolValue),
        /// Indicates that during forwarding, the host header will be swapped with the content of given
        /// downstream or :ref:`custom <config_http_conn_man_headers_custom_request_headers>` header.
        /// If header value is empty, host header is left intact. Using this option will append the
        /// :ref:`config_http_conn_man_headers_x-forwarded-host` header if
        /// :ref:`append_x_forwarded_host <envoy_v3_api_field_config.route.v3.RouteAction.append_x_forwarded_host>`
        /// is set.
        ///
        /// .. attention::
        ///
        /// Pay attention to the potential security implications of using this option. Provided header
        /// must come from trusted source.
        ///
        /// .. note::
        ///
        /// If the header appears multiple times only the first value is used.
        #[prost(string, tag = "29")]
        HostRewriteHeader(::prost::alloc::string::String),
        /// Indicates that during forwarding, the host header will be swapped with
        /// the result of the regex substitution executed on path value with query and fragment removed.
        /// This is useful for transitioning variable content between path segment and subdomain.
        /// Using this option will append the
        /// :ref:`config_http_conn_man_headers_x-forwarded-host` header if
        /// :ref:`append_x_forwarded_host <envoy_v3_api_field_config.route.v3.RouteAction.append_x_forwarded_host>`
        /// is set.
        ///
        /// For example with the following config:
        ///
        /// .. code-block:: yaml
        ///
        /// ```text
        /// host_rewrite_path_regex:
        ///    pattern:
        ///      google_re2: {}
        ///      regex: "^/(.+)/.+$"
        ///    substitution: \1
        /// ```
        ///
        /// Would rewrite the host header to `envoyproxy.io` given the path `/envoyproxy.io/some/path`.
        #[prost(message, tag = "35")]
        HostRewritePathRegex(
            super::super::super::super::r#type::matcher::v3::RegexMatchAndSubstitute,
        ),
    }
}
/// HTTP retry :ref:`architecture overview <arch_overview_http_routing_retry>`.
/// \[\#next-free-field: 14\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetryPolicy {
    /// Specifies the conditions under which retry takes place. These are the same
    /// conditions documented for :ref:`config_http_filters_router_x-envoy-retry-on` and
    /// :ref:`config_http_filters_router_x-envoy-retry-grpc-on`.
    #[prost(string, tag = "1")]
    pub retry_on: ::prost::alloc::string::String,
    /// Specifies the allowed number of retries. This parameter is optional and
    /// defaults to 1. These are the same conditions documented for
    /// :ref:`config_http_filters_router_x-envoy-max-retries`.
    #[prost(message, optional, tag = "2")]
    pub num_retries: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// Specifies a non-zero upstream timeout per retry attempt (including the initial attempt). This
    /// parameter is optional. The same conditions documented for
    /// :ref:`config_http_filters_router_x-envoy-upstream-rq-per-try-timeout-ms` apply.
    ///
    /// .. note::
    ///
    /// If left unspecified, Envoy will use the global
    /// :ref:`route timeout <envoy_v3_api_field_config.route.v3.RouteAction.timeout>` for the request.
    /// Consequently, when using a :ref:`5xx <config_http_filters_router_x-envoy-retry-on>` based
    /// retry policy, a request that times out will not be retried as the total timeout budget
    /// would have been exhausted.
    #[prost(message, optional, tag = "3")]
    pub per_try_timeout: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    /// Specifies an upstream idle timeout per retry attempt (including the initial attempt). This
    /// parameter is optional and if absent there is no per try idle timeout. The semantics of the per
    /// try idle timeout are similar to the
    /// :ref:`route idle timeout <envoy_v3_api_field_config.route.v3.RouteAction.timeout>` and
    /// :ref:`stream idle timeout <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.stream_idle_timeout>`
    /// both enforced by the HTTP connection manager. The difference is that this idle timeout
    /// is enforced by the router for each individual attempt and thus after all previous filters have
    /// run, as opposed to *before* all previous filters run for the other idle timeouts. This timeout
    /// is useful in cases in which total request timeout is bounded by a number of retries and a
    /// :ref:`per_try_timeout <envoy_v3_api_field_config.route.v3.RetryPolicy.per_try_timeout>`, but
    /// there is a desire to ensure each try is making incremental progress. Note also that similar
    /// to :ref:`per_try_timeout <envoy_v3_api_field_config.route.v3.RetryPolicy.per_try_timeout>`,
    /// this idle timeout does not start until after both the entire request has been received by the
    /// router *and* a connection pool connection has been obtained. Unlike
    /// :ref:`per_try_timeout <envoy_v3_api_field_config.route.v3.RetryPolicy.per_try_timeout>`,
    /// the idle timer continues once the response starts streaming back to the downstream client.
    /// This ensures that response data continues to make progress without using one of the HTTP
    /// connection manager idle timeouts.
    #[prost(message, optional, tag = "13")]
    pub per_try_idle_timeout: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    /// Specifies an implementation of a RetryPriority which is used to determine the
    /// distribution of load across priorities used for retries. Refer to
    /// :ref:`retry plugin configuration <arch_overview_http_retry_plugins>` for more details.
    #[prost(message, optional, tag = "4")]
    pub retry_priority: ::core::option::Option<retry_policy::RetryPriority>,
    /// Specifies a collection of RetryHostPredicates that will be consulted when selecting a host
    /// for retries. If any of the predicates reject the host, host selection will be reattempted.
    /// Refer to :ref:`retry plugin configuration <arch_overview_http_retry_plugins>` for more
    /// details.
    #[prost(message, repeated, tag = "5")]
    pub retry_host_predicate: ::prost::alloc::vec::Vec<retry_policy::RetryHostPredicate>,
    /// Retry options predicates that will be applied prior to retrying a request. These predicates
    /// allow customizing request behavior between retries.
    /// \[\#comment: add \[\#extension-category: envoy.retry_options_predicates\] when there are built-in extensions\]
    #[prost(message, repeated, tag = "12")]
    pub retry_options_predicates: ::prost::alloc::vec::Vec<
        super::super::core::v3::TypedExtensionConfig,
    >,
    /// The maximum number of times host selection will be reattempted before giving up, at which
    /// point the host that was last selected will be routed to. If unspecified, this will default to
    /// retrying once.
    #[prost(int64, tag = "6")]
    pub host_selection_retry_max_attempts: i64,
    /// HTTP status codes that should trigger a retry in addition to those specified by retry_on.
    #[prost(uint32, repeated, tag = "7")]
    pub retriable_status_codes: ::prost::alloc::vec::Vec<u32>,
    /// Specifies parameters that control exponential retry back off. This parameter is optional, in which case the
    /// default base interval is 25 milliseconds or, if set, the current value of the
    /// `upstream.base_retry_backoff_ms` runtime parameter. The default maximum interval is 10 times
    /// the base interval. The documentation for :ref:`config_http_filters_router_x-envoy-max-retries`
    /// describes Envoy's back-off algorithm.
    #[prost(message, optional, tag = "8")]
    pub retry_back_off: ::core::option::Option<retry_policy::RetryBackOff>,
    /// Specifies parameters that control a retry back-off strategy that is used
    /// when the request is rate limited by the upstream server. The server may
    /// return a response header like `Retry-After` or `X-RateLimit-Reset` to
    /// provide feedback to the client on how long to wait before retrying. If
    /// configured, this back-off strategy will be used instead of the
    /// default exponential back off strategy (configured using `retry_back_off`)
    /// whenever a response includes the matching headers.
    #[prost(message, optional, tag = "11")]
    pub rate_limited_retry_back_off: ::core::option::Option<
        retry_policy::RateLimitedRetryBackOff,
    >,
    /// HTTP response headers that trigger a retry if present in the response. A retry will be
    /// triggered if any of the header matches match the upstream response headers.
    /// The field is only consulted if 'retriable-headers' retry policy is active.
    #[prost(message, repeated, tag = "9")]
    pub retriable_headers: ::prost::alloc::vec::Vec<HeaderMatcher>,
    /// HTTP headers which must be present in the request for retries to be attempted.
    #[prost(message, repeated, tag = "10")]
    pub retriable_request_headers: ::prost::alloc::vec::Vec<HeaderMatcher>,
}
/// Nested message and enum types in `RetryPolicy`.
pub mod retry_policy {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RetryPriority {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// \[\#extension-category: envoy.retry_priorities\]
        #[prost(oneof = "retry_priority::ConfigType", tags = "3")]
        pub config_type: ::core::option::Option<retry_priority::ConfigType>,
    }
    /// Nested message and enum types in `RetryPriority`.
    pub mod retry_priority {
        /// \[\#extension-category: envoy.retry_priorities\]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ConfigType {
            #[prost(message, tag = "3")]
            TypedConfig(super::super::super::super::super::super::google::protobuf::Any),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RetryHostPredicate {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// \[\#extension-category: envoy.retry_host_predicates\]
        #[prost(oneof = "retry_host_predicate::ConfigType", tags = "3")]
        pub config_type: ::core::option::Option<retry_host_predicate::ConfigType>,
    }
    /// Nested message and enum types in `RetryHostPredicate`.
    pub mod retry_host_predicate {
        /// \[\#extension-category: envoy.retry_host_predicates\]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ConfigType {
            #[prost(message, tag = "3")]
            TypedConfig(super::super::super::super::super::super::google::protobuf::Any),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RetryBackOff {
        /// Specifies the base interval between retries. This parameter is required and must be greater
        /// than zero. Values less than 1 ms are rounded up to 1 ms.
        /// See :ref:`config_http_filters_router_x-envoy-max-retries` for a discussion of Envoy's
        /// back-off algorithm.
        #[prost(message, optional, tag = "1")]
        pub base_interval: ::core::option::Option<
            super::super::super::super::super::google::protobuf::Duration,
        >,
        /// Specifies the maximum interval between retries. This parameter is optional, but must be
        /// greater than or equal to the `base_interval` if set. The default is 10 times the
        /// `base_interval`. See :ref:`config_http_filters_router_x-envoy-max-retries` for a discussion
        /// of Envoy's back-off algorithm.
        #[prost(message, optional, tag = "2")]
        pub max_interval: ::core::option::Option<
            super::super::super::super::super::google::protobuf::Duration,
        >,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResetHeader {
        /// The name of the reset header.
        ///
        /// .. note::
        ///
        /// If the header appears multiple times only the first value is used.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// The format of the reset header.
        #[prost(enumeration = "ResetHeaderFormat", tag = "2")]
        pub format: i32,
    }
    /// A retry back-off strategy that applies when the upstream server rate limits
    /// the request.
    ///
    /// Given this configuration:
    ///
    /// .. code-block:: yaml
    ///
    /// rate_limited_retry_back_off:
    /// reset_headers:
    /// - name: Retry-After
    /// format: SECONDS
    /// - name: X-RateLimit-Reset
    /// format: UNIX_TIMESTAMP
    /// max_interval: "300s"
    ///
    /// The following algorithm will apply:
    ///
    /// 1. If the response contains the header `Retry-After` its value must be on
    ///    the form `120` (an integer that represents the number of seconds to
    ///    wait before retrying). If so, this value is used as the back-off interval.
    /// 1. Otherwise, if the response contains the header `X-RateLimit-Reset` its
    ///    value must be on the form `1595320702` (an integer that represents the
    ///    point in time at which to retry, as a Unix timestamp in seconds). If so,
    ///    the current time is subtracted from this value and the result is used as
    ///    the back-off interval.
    /// 1. Otherwise, Envoy will use the default
    ///    :ref:`exponential back-off <envoy_v3_api_field_config.route.v3.RetryPolicy.retry_back_off>`
    ///    strategy.
    ///
    /// No matter which format is used, if the resulting back-off interval exceeds
    /// `max_interval` it is discarded and the next header in `reset_headers`
    /// is tried. If a request timeout is configured for the route it will further
    /// limit how long the request will be allowed to run.
    ///
    /// To prevent many clients retrying at the same point in time jitter is added
    /// to the back-off interval, so the resulting interval is decided by taking:
    /// `random(interval, interval * 1.5)`.
    ///
    /// .. attention::
    ///
    /// Configuring `rate_limited_retry_back_off` will not by itself cause a request
    /// to be retried. You will still need to configure the right retry policy to match
    /// the responses from the upstream server.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RateLimitedRetryBackOff {
        /// Specifies the reset headers (like `Retry-After` or `X-RateLimit-Reset`)
        /// to match against the response. Headers are tried in order, and matched case
        /// insensitive. The first header to be parsed successfully is used. If no headers
        /// match the default exponential back-off is used instead.
        #[prost(message, repeated, tag = "1")]
        pub reset_headers: ::prost::alloc::vec::Vec<ResetHeader>,
        /// Specifies the maximum back off interval that Envoy will allow. If a reset
        /// header contains an interval longer than this then it will be discarded and
        /// the next header will be tried. Defaults to 300 seconds.
        #[prost(message, optional, tag = "2")]
        pub max_interval: ::core::option::Option<
            super::super::super::super::super::google::protobuf::Duration,
        >,
    }
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
    pub enum ResetHeaderFormat {
        Seconds = 0,
        UnixTimestamp = 1,
    }
    impl ResetHeaderFormat {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ResetHeaderFormat::Seconds => "SECONDS",
                ResetHeaderFormat::UnixTimestamp => "UNIX_TIMESTAMP",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SECONDS" => Some(Self::Seconds),
                "UNIX_TIMESTAMP" => Some(Self::UnixTimestamp),
                _ => None,
            }
        }
    }
}
/// HTTP request hedging :ref:`architecture overview <arch_overview_http_routing_hedging>`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HedgePolicy {
    /// Specifies the number of initial requests that should be sent upstream.
    /// Must be at least 1.
    /// Defaults to 1.
    /// \[\#not-implemented-hide:\]
    #[prost(message, optional, tag = "1")]
    pub initial_requests: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// Specifies a probability that an additional upstream request should be sent
    /// on top of what is specified by initial_requests.
    /// Defaults to 0.
    /// \[\#not-implemented-hide:\]
    #[prost(message, optional, tag = "2")]
    pub additional_request_chance: ::core::option::Option<
        super::super::super::r#type::v3::FractionalPercent,
    >,
    /// Indicates that a hedged request should be sent when the per-try timeout is hit.
    /// This means that a retry will be issued without resetting the original request, leaving multiple upstream requests in flight.
    /// The first request to complete successfully will be the one returned to the caller.
    ///
    /// * At any time, a successful response (i.e. not triggering any of the retry-on conditions) would be returned to the client.
    /// * Before per-try timeout, an error response (per retry-on conditions) would be retried immediately or returned ot the client
    ///   if there are no more retries left.
    /// * After per-try timeout, an error response would be discarded, as a retry in the form of a hedged request is already in progress.
    ///
    /// Note: For this to have effect, you must have a :ref:`RetryPolicy <envoy_v3_api_msg_config.route.v3.RetryPolicy>` that retries at least
    /// one error code and specifies a maximum number of retries.
    ///
    /// Defaults to false.
    #[prost(bool, tag = "3")]
    pub hedge_on_per_try_timeout: bool,
}
/// \[\#next-free-field: 10\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedirectAction {
    /// The host portion of the URL will be swapped with this value.
    #[prost(string, tag = "1")]
    pub host_redirect: ::prost::alloc::string::String,
    /// The port value of the URL will be swapped with this value.
    #[prost(uint32, tag = "8")]
    pub port_redirect: u32,
    /// The HTTP status code to use in the redirect response. The default response
    /// code is MOVED_PERMANENTLY (301).
    #[prost(enumeration = "redirect_action::RedirectResponseCode", tag = "3")]
    pub response_code: i32,
    /// Indicates that during redirection, the query portion of the URL will
    /// be removed. Default value is false.
    #[prost(bool, tag = "6")]
    pub strip_query: bool,
    /// When the scheme redirection take place, the following rules apply:
    ///
    /// 1. If the source URI scheme is `http` and the port is explicitly
    ///    set to `:80`, the port will be removed after the redirection
    /// 1. If the source URI scheme is `https` and the port is explicitly
    ///    set to `:443`, the port will be removed after the redirection
    #[prost(oneof = "redirect_action::SchemeRewriteSpecifier", tags = "4, 7")]
    pub scheme_rewrite_specifier: ::core::option::Option<
        redirect_action::SchemeRewriteSpecifier,
    >,
    #[prost(oneof = "redirect_action::PathRewriteSpecifier", tags = "2, 5, 9")]
    pub path_rewrite_specifier: ::core::option::Option<
        redirect_action::PathRewriteSpecifier,
    >,
}
/// Nested message and enum types in `RedirectAction`.
pub mod redirect_action {
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
    pub enum RedirectResponseCode {
        /// Moved Permanently HTTP Status Code - 301.
        MovedPermanently = 0,
        /// Found HTTP Status Code - 302.
        Found = 1,
        /// See Other HTTP Status Code - 303.
        SeeOther = 2,
        /// Temporary Redirect HTTP Status Code - 307.
        TemporaryRedirect = 3,
        /// Permanent Redirect HTTP Status Code - 308.
        PermanentRedirect = 4,
    }
    impl RedirectResponseCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RedirectResponseCode::MovedPermanently => "MOVED_PERMANENTLY",
                RedirectResponseCode::Found => "FOUND",
                RedirectResponseCode::SeeOther => "SEE_OTHER",
                RedirectResponseCode::TemporaryRedirect => "TEMPORARY_REDIRECT",
                RedirectResponseCode::PermanentRedirect => "PERMANENT_REDIRECT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MOVED_PERMANENTLY" => Some(Self::MovedPermanently),
                "FOUND" => Some(Self::Found),
                "SEE_OTHER" => Some(Self::SeeOther),
                "TEMPORARY_REDIRECT" => Some(Self::TemporaryRedirect),
                "PERMANENT_REDIRECT" => Some(Self::PermanentRedirect),
                _ => None,
            }
        }
    }
    /// When the scheme redirection take place, the following rules apply:
    ///
    /// 1. If the source URI scheme is `http` and the port is explicitly
    ///    set to `:80`, the port will be removed after the redirection
    /// 1. If the source URI scheme is `https` and the port is explicitly
    ///    set to `:443`, the port will be removed after the redirection
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SchemeRewriteSpecifier {
        /// The scheme portion of the URL will be swapped with "https".
        #[prost(bool, tag = "4")]
        HttpsRedirect(bool),
        /// The scheme portion of the URL will be swapped with this value.
        #[prost(string, tag = "7")]
        SchemeRedirect(::prost::alloc::string::String),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PathRewriteSpecifier {
        /// The path portion of the URL will be swapped with this value.
        /// Please note that query string in path_redirect will override the
        /// request's query string and will not be stripped.
        ///
        /// For example, let's say we have the following routes:
        ///
        /// * match: { path: "/old-path-1" }
        ///   redirect: { path_redirect: "/new-path-1" }
        /// * match: { path: "/old-path-2" }
        ///   redirect: { path_redirect: "/new-path-2", strip-query: "true" }
        /// * match: { path: "/old-path-3" }
        ///   redirect: { path_redirect: "/new-path-3?foo=1", strip_query: "true" }
        ///
        /// 1. if request uri is "/old-path-1?bar=1", users will be redirected to "/new-path-1?bar=1"
        /// 1. if request uri is "/old-path-2?bar=1", users will be redirected to "/new-path-2"
        /// 1. if request uri is "/old-path-3?bar=1", users will be redirected to "/new-path-3?foo=1"
        #[prost(string, tag = "2")]
        PathRedirect(::prost::alloc::string::String),
        /// Indicates that during redirection, the matched prefix (or path)
        /// should be swapped with this value. This option allows redirect URLs be dynamically created
        /// based on the request.
        ///
        /// .. attention::
        ///
        /// Pay attention to the use of trailing slashes as mentioned in
        /// :ref:`RouteAction's prefix_rewrite <envoy_v3_api_field_config.route.v3.RouteAction.prefix_rewrite>`.
        #[prost(string, tag = "5")]
        PrefixRewrite(::prost::alloc::string::String),
        /// Indicates that during redirect, portions of the path that match the
        /// pattern should be rewritten, even allowing the substitution of capture
        /// groups from the pattern into the new path as specified by the rewrite
        /// substitution string. This is useful to allow application paths to be
        /// rewritten in a way that is aware of segments with variable content like
        /// identifiers.
        ///
        /// Examples using Google's `RE2 <<https://github.com/google/re2>`\_> engine:
        ///
        /// * The path pattern `^/service/(\[^/\]+)(/.*)$` paired with a substitution
        ///   string of `\2/instance/\1` would transform `/service/foo/v1/api`
        ///   into `/v1/api/instance/foo`.
        ///
        /// * The pattern `one` paired with a substitution string of `two` would
        ///   transform `/xxx/one/yyy/one/zzz` into `/xxx/two/yyy/two/zzz`.
        ///
        /// * The pattern `^(.*?)one(.*)$` paired with a substitution string of
        ///   `\1two\2` would replace only the first occurrence of `one`,
        ///   transforming path `/xxx/one/yyy/one/zzz` into `/xxx/two/yyy/one/zzz`.
        ///
        /// * The pattern `(?i)/xxx/` paired with a substitution string of `/yyy/`
        ///   would do a case-insensitive match and transform path `/aaa/XxX/bbb` to
        ///   `/aaa/yyy/bbb`.
        #[prost(message, tag = "9")]
        RegexRewrite(
            super::super::super::super::r#type::matcher::v3::RegexMatchAndSubstitute,
        ),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectResponseAction {
    /// Specifies the HTTP response status to be returned.
    #[prost(uint32, tag = "1")]
    pub status: u32,
    /// Specifies the content of the response body. If this setting is omitted,
    /// no body is included in the generated response.
    ///
    /// .. note::
    ///
    /// Headers can be specified using `response_headers_to_add` in the enclosing
    /// :ref:`envoy_v3_api_msg_config.route.v3.Route`, :ref:`envoy_v3_api_msg_config.route.v3.RouteConfiguration` or
    /// :ref:`envoy_v3_api_msg_config.route.v3.VirtualHost`.
    #[prost(message, optional, tag = "2")]
    pub body: ::core::option::Option<super::super::core::v3::DataSource>,
}
/// \[\#not-implemented-hide:\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NonForwardingAction {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Decorator {
    /// The operation name associated with the request matched to this route. If tracing is
    /// enabled, this information will be used as the span name reported for this request.
    ///
    /// .. note::
    ///
    /// For ingress (inbound) requests, or egress (outbound) responses, this value may be overridden
    /// by the :ref:`x-envoy-decorator-operation <config_http_filters_router_x-envoy-decorator-operation>` header.
    #[prost(string, tag = "1")]
    pub operation: ::prost::alloc::string::String,
    /// Whether the decorated details should be propagated to the other party. The default is true.
    #[prost(message, optional, tag = "2")]
    pub propagate: ::core::option::Option<
        super::super::super::super::google::protobuf::BoolValue,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tracing {
    /// Target percentage of requests managed by this HTTP connection manager that will be force
    /// traced if the :ref:`x-client-trace-id <config_http_conn_man_headers_x-client-trace-id>`
    /// header is set. This field is a direct analog for the runtime variable
    /// 'tracing.client_enabled' in the :ref:`HTTP Connection Manager <config_http_conn_man_runtime>`.
    /// Default: 100%
    #[prost(message, optional, tag = "1")]
    pub client_sampling: ::core::option::Option<
        super::super::super::r#type::v3::FractionalPercent,
    >,
    /// Target percentage of requests managed by this HTTP connection manager that will be randomly
    /// selected for trace generation, if not requested by the client or not forced. This field is
    /// a direct analog for the runtime variable 'tracing.random_sampling' in the
    /// :ref:`HTTP Connection Manager <config_http_conn_man_runtime>`.
    /// Default: 100%
    #[prost(message, optional, tag = "2")]
    pub random_sampling: ::core::option::Option<
        super::super::super::r#type::v3::FractionalPercent,
    >,
    /// Target percentage of requests managed by this HTTP connection manager that will be traced
    /// after all other sampling checks have been applied (client-directed, force tracing, random
    /// sampling). This field functions as an upper limit on the total configured sampling rate. For
    /// instance, setting client_sampling to 100% but overall_sampling to 1% will result in only 1%
    /// of client requests with the appropriate headers to be force traced. This field is a direct
    /// analog for the runtime variable 'tracing.global_enabled' in the
    /// :ref:`HTTP Connection Manager <config_http_conn_man_runtime>`.
    /// Default: 100%
    #[prost(message, optional, tag = "3")]
    pub overall_sampling: ::core::option::Option<
        super::super::super::r#type::v3::FractionalPercent,
    >,
    /// A list of custom tags with unique tag name to create tags for the active span.
    /// It will take effect after merging with the :ref:`corresponding configuration <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.Tracing.custom_tags>`
    /// configured in the HTTP connection manager. If two tags with the same name are configured
    /// each in the HTTP connection manager and the route level, the one configured here takes
    /// priority.
    #[prost(message, repeated, tag = "4")]
    pub custom_tags: ::prost::alloc::vec::Vec<
        super::super::super::r#type::tracing::v3::CustomTag,
    >,
}
/// A virtual cluster is a way of specifying a regex matching rule against
/// certain important endpoints such that statistics are generated explicitly for
/// the matched requests. The reason this is useful is that when doing
/// prefix/path matching Envoy does not always know what the application
/// considers to be an endpoint. Thus, it’s impossible for Envoy to generically
/// emit per endpoint statistics. However, often systems have highly critical
/// endpoints that they wish to get “perfect” statistics on. Virtual cluster
/// statistics are perfect in the sense that they are emitted on the downstream
/// side such that they include network level failures.
///
/// Documentation for :ref:`virtual cluster statistics <config_http_filters_router_vcluster_stats>`.
///
/// .. note::
///
/// ```text
/// Virtual clusters are a useful tool, but we do not recommend setting up a virtual cluster for
/// every application endpoint. This is both not easily maintainable and as well the matching and
/// statistics output are not free.
/// ```
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualCluster {
    /// Specifies a list of header matchers to use for matching requests. Each specified header must
    /// match. The pseudo-headers `:path` and `:method` can be used to match the request path and
    /// method, respectively.
    #[prost(message, repeated, tag = "4")]
    pub headers: ::prost::alloc::vec::Vec<HeaderMatcher>,
    /// Specifies the name of the virtual cluster. The virtual cluster name as well
    /// as the virtual host name are used when emitting statistics. The statistics are emitted by the
    /// router filter and are documented :ref:`here <config_http_filters_router_stats>`.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
/// Global rate limiting :ref:`architecture overview <arch_overview_global_rate_limit>`.
/// Also applies to Local rate limiting :ref:`using descriptors <config_http_filters_local_rate_limit_descriptors>`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RateLimit {
    /// Refers to the stage set in the filter. The rate limit configuration only
    /// applies to filters with the same stage number. The default stage number is
    /// 0.
    ///
    /// .. note::
    ///
    /// The filter supports a range of 0 - 10 inclusively for stage numbers.
    #[prost(message, optional, tag = "1")]
    pub stage: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// The key to be set in runtime to disable this rate limit configuration.
    #[prost(string, tag = "2")]
    pub disable_key: ::prost::alloc::string::String,
    /// A list of actions that are to be applied for this rate limit configuration.
    /// Order matters as the actions are processed sequentially and the descriptor
    /// is composed by appending descriptor entries in that sequence. If an action
    /// cannot append a descriptor entry, no descriptor is generated for the
    /// configuration. See :ref:`composing actions <config_http_filters_rate_limit_composing_actions>` for additional documentation.
    #[prost(message, repeated, tag = "3")]
    pub actions: ::prost::alloc::vec::Vec<rate_limit::Action>,
    /// An optional limit override to be appended to the descriptor produced by this
    /// rate limit configuration. If the override value is invalid or cannot be resolved
    /// from metadata, no override is provided. See :ref:`rate limit override <config_http_filters_rate_limit_rate_limit_override>` for more information.
    #[prost(message, optional, tag = "4")]
    pub limit: ::core::option::Option<rate_limit::Override>,
}
/// Nested message and enum types in `RateLimit`.
pub mod rate_limit {
    /// \[\#next-free-field: 12\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Action {
        #[prost(
            oneof = "action::ActionSpecifier",
            tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11"
        )]
        pub action_specifier: ::core::option::Option<action::ActionSpecifier>,
    }
    /// Nested message and enum types in `Action`.
    pub mod action {
        /// The following descriptor entry is appended to the descriptor:
        ///
        /// .. code-block:: cpp
        ///
        /// ("source_cluster", "<local service cluster>")
        ///
        /// <local service cluster> is derived from the :option:`--service-cluster` option.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SourceCluster {}
        /// The following descriptor entry is appended to the descriptor:
        ///
        /// .. code-block:: cpp
        ///
        /// ("destination_cluster", "<routed target cluster>")
        ///
        /// Once a request matches against a route table rule, a routed cluster is determined by one of
        /// the following :ref:`route table configuration <envoy_v3_api_msg_config.route.v3.RouteConfiguration>`
        /// settings:
        ///
        /// * :ref:`cluster <envoy_v3_api_field_config.route.v3.RouteAction.cluster>` indicates the upstream cluster
        ///   to route to.
        /// * :ref:`weighted_clusters <envoy_v3_api_field_config.route.v3.RouteAction.weighted_clusters>`
        ///   chooses a cluster randomly from a set of clusters with attributed weight.
        /// * :ref:`cluster_header <envoy_v3_api_field_config.route.v3.RouteAction.cluster_header>` indicates which
        ///   header in the request contains the target cluster.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DestinationCluster {}
        /// The following descriptor entry is appended when a header contains a key that matches the
        /// `header_name`:
        ///
        /// .. code-block:: cpp
        ///
        /// ("\<descriptor_key>", "\<header_value_queried_from_header>")
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
        /// ("remote_address", "<trusted address from x-forwarded-for>")
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct RemoteAddress {}
        /// The following descriptor entry is appended to the descriptor and is populated using the
        /// masked address from :ref:`x-forwarded-for <config_http_conn_man_headers_x-forwarded-for>`:
        ///
        /// .. code-block:: cpp
        ///
        /// ("masked_remote_address", "<masked address from x-forwarded-for>")
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MaskedRemoteAddress {
            /// Length of prefix mask len for IPv4 (e.g. 0, 32).
            /// Defaults to 32 when unset.
            /// For example, trusted address from x-forwarded-for is `192.168.1.1`,
            /// the descriptor entry is ("masked_remote_address", "192.168.1.1/32");
            /// if mask len is 24, the descriptor entry is ("masked_remote_address", "192.168.1.0/24").
            #[prost(message, optional, tag = "1")]
            pub v4_prefix_mask_len: ::core::option::Option<
                super::super::super::super::super::super::google::protobuf::UInt32Value,
            >,
            /// Length of prefix mask len for IPv6 (e.g. 0, 128).
            /// Defaults to 128 when unset.
            /// For example, trusted address from x-forwarded-for is `2001:abcd:ef01:2345:6789:abcd:ef01:234`,
            /// the descriptor entry is ("masked_remote_address", "2001:abcd:ef01:2345:6789:abcd:ef01:234/128");
            /// if mask len is 64, the descriptor entry is ("masked_remote_address", "2001:abcd:ef01:2345::/64").
            #[prost(message, optional, tag = "2")]
            pub v6_prefix_mask_len: ::core::option::Option<
                super::super::super::super::super::super::google::protobuf::UInt32Value,
            >,
        }
        /// The following descriptor entry is appended to the descriptor:
        ///
        /// .. code-block:: cpp
        ///
        /// ("generic_key", "\<descriptor_value>")
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
        /// ("header_match", "\<descriptor_value>")
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct HeaderValueMatch {
            /// The key to use in the descriptor entry. Defaults to `header_match`.
            #[prost(string, tag = "4")]
            pub descriptor_key: ::prost::alloc::string::String,
            /// The value to use in the descriptor entry.
            #[prost(string, tag = "1")]
            pub descriptor_value: ::prost::alloc::string::String,
            /// If set to true, the action will append a descriptor entry when the
            /// request matches the headers. If set to false, the action will append a
            /// descriptor entry when the request does not match the headers. The
            /// default value is true.
            #[prost(message, optional, tag = "2")]
            pub expect_match: ::core::option::Option<
                super::super::super::super::super::super::google::protobuf::BoolValue,
            >,
            /// Specifies a set of headers that the rate limit action should match
            /// on. The action will check the request’s headers against all the
            /// specified headers in the config. A match will happen if all the
            /// headers in the config are present in the request with the same values
            /// (or based on presence if the value field is not in the config).
            #[prost(message, repeated, tag = "3")]
            pub headers: ::prost::alloc::vec::Vec<super::super::HeaderMatcher>,
        }
        /// The following descriptor entry is appended when the
        /// :ref:`dynamic metadata <well_known_dynamic_metadata>` contains a key value:
        ///
        /// .. code-block:: cpp
        ///
        /// ("\<descriptor_key>", "\<value_queried_from_dynamic_metadata>")
        ///
        /// .. attention::
        /// This action has been deprecated in favor of the :ref:`metadata <envoy_v3_api_msg_config.route.v3.RateLimit.Action.MetaData>` action
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DynamicMetaData {
            /// The key to use in the descriptor entry.
            #[prost(string, tag = "1")]
            pub descriptor_key: ::prost::alloc::string::String,
            /// Metadata struct that defines the key and path to retrieve the string value. A match will
            /// only happen if the value in the dynamic metadata is of type string.
            #[prost(message, optional, tag = "2")]
            pub metadata_key: ::core::option::Option<
                super::super::super::super::super::r#type::metadata::v3::MetadataKey,
            >,
            /// An optional value to use if `metadata_key` is empty. If not set and
            /// no value is present under the metadata_key then no descriptor is generated.
            #[prost(string, tag = "3")]
            pub default_value: ::prost::alloc::string::String,
        }
        /// The following descriptor entry is appended when the metadata contains a key value:
        ///
        /// .. code-block:: cpp
        ///
        /// ("\<descriptor_key>", "\<value_queried_from_metadata>")
        /// \[\#next-free-field: 6\]
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
                super::super::super::super::super::r#type::metadata::v3::MetadataKey,
            >,
            /// An optional value to use if `metadata_key` is empty. If not set and
            /// no value is present under the metadata_key then `skip_if_absent` is followed to
            /// skip calling the rate limiting service or skip the descriptor.
            #[prost(string, tag = "3")]
            pub default_value: ::prost::alloc::string::String,
            /// Source of metadata
            #[prost(enumeration = "meta_data::Source", tag = "4")]
            pub source: i32,
            /// If set to true, Envoy skips the descriptor while calling rate limiting service
            /// when `metadata_key` is empty and `default_value` is not set. By default it skips calling the
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
        /// The following descriptor entry is appended to the descriptor:
        ///
        /// .. code-block:: cpp
        ///
        /// ("query_match", "\<descriptor_value>")
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct QueryParameterValueMatch {
            /// The key to use in the descriptor entry. Defaults to `query_match`.
            #[prost(string, tag = "4")]
            pub descriptor_key: ::prost::alloc::string::String,
            /// The value to use in the descriptor entry.
            #[prost(string, tag = "1")]
            pub descriptor_value: ::prost::alloc::string::String,
            /// If set to true, the action will append a descriptor entry when the
            /// request matches the headers. If set to false, the action will append a
            /// descriptor entry when the request does not match the headers. The
            /// default value is true.
            #[prost(message, optional, tag = "2")]
            pub expect_match: ::core::option::Option<
                super::super::super::super::super::super::google::protobuf::BoolValue,
            >,
            /// Specifies a set of query parameters that the rate limit action should match
            /// on. The action will check the request’s query parameters against all the
            /// specified query parameters in the config. A match will happen if all the
            /// query parameters in the config are present in the request with the same values
            /// (or based on presence if the value field is not in the config).
            #[prost(message, repeated, tag = "3")]
            pub query_parameters: ::prost::alloc::vec::Vec<
                super::super::QueryParameterMatcher,
            >,
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
            /// Rate limit on dynamic metadata.
            ///
            /// .. attention::
            /// This field has been deprecated in favor of the :ref:`metadata <envoy_v3_api_field_config.route.v3.RateLimit.Action.metadata>` field
            #[prost(message, tag = "7")]
            DynamicMetadata(DynamicMetaData),
            /// Rate limit on metadata.
            #[prost(message, tag = "8")]
            Metadata(MetaData),
            /// Rate limit descriptor extension. See the rate limit descriptor extensions documentation.
            ///
            /// :ref:`HTTP matching input functions <arch_overview_matching_api>` are
            /// permitted as descriptor extensions. The input functions are only
            /// looked up if there is no rate limit descriptor extension matching
            /// the type URL.
            ///
            /// \[\#extension-category: envoy.rate_limit_descriptors\]
            #[prost(message, tag = "9")]
            Extension(super::super::super::super::core::v3::TypedExtensionConfig),
            /// Rate limit on masked remote address.
            #[prost(message, tag = "10")]
            MaskedRemoteAddress(MaskedRemoteAddress),
            /// Rate limit on the existence of query parameters.
            #[prost(message, tag = "11")]
            QueryParameterValueMatch(QueryParameterValueMatch),
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
            /// and a "unit" property with a value parseable to :ref:`RateLimitUnit enum <envoy_v3_api_enum_type.v3.RateLimitUnit>`
            #[prost(message, optional, tag = "1")]
            pub metadata_key: ::core::option::Option<
                super::super::super::super::super::r#type::metadata::v3::MetadataKey,
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
/// .. attention::
///
/// Internally, Envoy always uses the HTTP/2 `:authority` header to represent the HTTP/1 `Host`
/// header. Thus, if attempting to match on `Host`, match on `:authority` instead.
///
/// .. attention::
///
/// To route on HTTP method, use the special HTTP/2 `:method` header. This works for both
/// HTTP/1 and HTTP/2 as Envoy normalizes headers. E.g.,
///
/// .. code-block:: json
///
/// ```text
/// {
///    "name": ":method",
///    "string_match": {
///      "exact": "POST"
///    }
/// }
/// ```
///
/// .. attention::
/// In the absence of any header match specifier, match will default to :ref:`present_match <envoy_v3_api_field_config.route.v3.HeaderMatcher.present_match>`. i.e, a request that has the :ref:`name <envoy_v3_api_field_config.route.v3.HeaderMatcher.name>` header will match, regardless of the header's
/// value.
///
/// \[\#next-major-version: HeaderMatcher should be refactored to use StringMatcher.\]
/// \[\#next-free-field: 15\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderMatcher {
    /// Specifies the name of the header in the request.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If specified, the match result will be inverted before checking. Defaults to false.
    ///
    /// Examples:
    ///
    /// * The regex `\d{3}` does not match the value `1234`, so it will match when inverted.
    /// * The range \[-10,0) will match the value -1, so it will not match when inverted.
    #[prost(bool, tag = "8")]
    pub invert_match: bool,
    /// If specified, for any header match rule, if the header match rule specified header
    /// does not exist, this header value will be treated as empty. Defaults to false.
    ///
    /// Examples:
    ///
    /// * The header match rule specified header "header1" to range match of \[0, 10\],
    ///   :ref:`invert_match <envoy_v3_api_field_config.route.v3.HeaderMatcher.invert_match>`
    ///   is set to true and :ref:`treat_missing_header_as_empty <envoy_v3_api_field_config.route.v3.HeaderMatcher.treat_missing_header_as_empty>`
    ///   is set to true; The "header1" header is not present. The match rule will
    ///   treat the "header1" as an empty header. The empty header does not match the range,
    ///   so it will match when inverted.
    /// * The header match rule specified header "header2" to range match of \[0, 10\],
    ///   :ref:`invert_match <envoy_v3_api_field_config.route.v3.HeaderMatcher.invert_match>`
    ///   is set to true and :ref:`treat_missing_header_as_empty <envoy_v3_api_field_config.route.v3.HeaderMatcher.treat_missing_header_as_empty>`
    ///   is set to false; The "header2" header is not present and the header
    ///   matcher rule for "header2" will be ignored so it will not match.
    /// * The header match rule specified header "header3" to a string regex match
    ///   `^$` which means an empty string, and
    ///   :ref:`treat_missing_header_as_empty <envoy_v3_api_field_config.route.v3.HeaderMatcher.treat_missing_header_as_empty>`
    ///   is set to true; The "header3" header is not present.
    ///   The match rule will treat the "header3" header as an empty header so it will match.
    /// * The header match rule specified header "header4" to a string regex match
    ///   `^$` which means an empty string, and
    ///   :ref:`treat_missing_header_as_empty <envoy_v3_api_field_config.route.v3.HeaderMatcher.treat_missing_header_as_empty>`
    ///   is set to false; The "header4" header is not present.
    ///   The match rule for "header4" will be ignored so it will not match.
    #[prost(bool, tag = "14")]
    pub treat_missing_header_as_empty: bool,
    /// Specifies how the header match will be performed to route the request.
    #[prost(
        oneof = "header_matcher::HeaderMatchSpecifier",
        tags = "4, 11, 6, 7, 9, 10, 12, 13"
    )]
    pub header_match_specifier: ::core::option::Option<
        header_matcher::HeaderMatchSpecifier,
    >,
}
/// Nested message and enum types in `HeaderMatcher`.
pub mod header_matcher {
    /// Specifies how the header match will be performed to route the request.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum HeaderMatchSpecifier {
        /// If specified, header match will be performed based on the value of the header.
        /// This field is deprecated. Please use :ref:`string_match <envoy_v3_api_field_config.route.v3.HeaderMatcher.string_match>`.
        #[prost(string, tag = "4")]
        ExactMatch(::prost::alloc::string::String),
        /// If specified, this regex string is a regular expression rule which implies the entire request
        /// header value must match the regex. The rule will not match if only a subsequence of the
        /// request header value matches the regex.
        /// This field is deprecated. Please use :ref:`string_match <envoy_v3_api_field_config.route.v3.HeaderMatcher.string_match>`.
        #[prost(message, tag = "11")]
        SafeRegexMatch(super::super::super::super::r#type::matcher::v3::RegexMatcher),
        /// If specified, header match will be performed based on range.
        /// The rule will match if the request header value is within this range.
        /// The entire request header value must represent an integer in base 10 notation: consisting of
        /// an optional plus or minus sign followed by a sequence of digits. The rule will not match if
        /// the header value does not represent an integer. Match will fail for empty values, floating
        /// point numbers or if only a subsequence of the header value is an integer.
        ///
        /// Examples:
        ///
        /// * For range \[-10,0), route will match for header value -1, but not for 0, `somestring`, 10.9,
        ///   `-1somestring`
        #[prost(message, tag = "6")]
        RangeMatch(super::super::super::super::r#type::v3::Int64Range),
        /// If specified as true, header match will be performed based on whether the header is in the
        /// request. If specified as false, header match will be performed based on whether the header is absent.
        #[prost(bool, tag = "7")]
        PresentMatch(bool),
        /// If specified, header match will be performed based on the prefix of the header value.
        /// Note: empty prefix is not allowed, please use present_match instead.
        /// This field is deprecated. Please use :ref:`string_match <envoy_v3_api_field_config.route.v3.HeaderMatcher.string_match>`.
        ///
        /// Examples:
        ///
        /// * The prefix `abcd` matches the value `abcdxyz`, but not for `abcxyz`.
        #[prost(string, tag = "9")]
        PrefixMatch(::prost::alloc::string::String),
        /// If specified, header match will be performed based on the suffix of the header value.
        /// Note: empty suffix is not allowed, please use present_match instead.
        /// This field is deprecated. Please use :ref:`string_match <envoy_v3_api_field_config.route.v3.HeaderMatcher.string_match>`.
        ///
        /// Examples:
        ///
        /// * The suffix `abcd` matches the value `xyzabcd`, but not for `xyzbcd`.
        #[prost(string, tag = "10")]
        SuffixMatch(::prost::alloc::string::String),
        /// If specified, header match will be performed based on whether the header value contains
        /// the given value or not.
        /// Note: empty contains match is not allowed, please use present_match instead.
        /// This field is deprecated. Please use :ref:`string_match <envoy_v3_api_field_config.route.v3.HeaderMatcher.string_match>`.
        ///
        /// Examples:
        ///
        /// * The value `abcd` matches the value `xyzabcdpqr`, but not for `xyzbcdpqr`.
        #[prost(string, tag = "12")]
        ContainsMatch(::prost::alloc::string::String),
        /// If specified, header match will be performed based on the string match of the header value.
        #[prost(message, tag = "13")]
        StringMatch(super::super::super::super::r#type::matcher::v3::StringMatcher),
    }
}
/// Query parameter matching treats the query string of a request's :path header
/// as an ampersand-separated list of keys and/or key=value elements.
/// \[\#next-free-field: 7\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParameterMatcher {
    /// Specifies the name of a key that must be present in the requested
    /// `path`'s query string.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(
        oneof = "query_parameter_matcher::QueryParameterMatchSpecifier",
        tags = "5, 6"
    )]
    pub query_parameter_match_specifier: ::core::option::Option<
        query_parameter_matcher::QueryParameterMatchSpecifier,
    >,
}
/// Nested message and enum types in `QueryParameterMatcher`.
pub mod query_parameter_matcher {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum QueryParameterMatchSpecifier {
        /// Specifies whether a query parameter value should match against a string.
        #[prost(message, tag = "5")]
        StringMatch(super::super::super::super::r#type::matcher::v3::StringMatcher),
        /// Specifies whether a query parameter should be present.
        #[prost(bool, tag = "6")]
        PresentMatch(bool),
    }
}
/// HTTP Internal Redirect :ref:`architecture overview <arch_overview_internal_redirects>`.
/// \[\#next-free-field: 6\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InternalRedirectPolicy {
    /// An internal redirect is not handled, unless the number of previous internal redirects that a
    /// downstream request has encountered is lower than this value.
    /// In the case where a downstream request is bounced among multiple routes by internal redirect,
    /// the first route that hits this threshold, or does not set :ref:`internal_redirect_policy <envoy_v3_api_field_config.route.v3.RouteAction.internal_redirect_policy>`
    /// will pass the redirect back to downstream.
    ///
    /// If not specified, at most one redirect will be followed.
    #[prost(message, optional, tag = "1")]
    pub max_internal_redirects: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// Defines what upstream response codes are allowed to trigger internal redirect. If unspecified,
    /// only 302 will be treated as internal redirect.
    /// Only 301, 302, 303, 307 and 308 are valid values. Any other codes will be ignored.
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub redirect_response_codes: ::prost::alloc::vec::Vec<u32>,
    /// Specifies a list of predicates that are queried when an upstream response is deemed
    /// to trigger an internal redirect by all other criteria. Any predicate in the list can reject
    /// the redirect, causing the response to be proxied to downstream.
    /// \[\#extension-category: envoy.internal_redirect_predicates\]
    #[prost(message, repeated, tag = "3")]
    pub predicates: ::prost::alloc::vec::Vec<
        super::super::core::v3::TypedExtensionConfig,
    >,
    /// Allow internal redirect to follow a target URI with a different scheme than the value of
    /// x-forwarded-proto. The default is false.
    #[prost(bool, tag = "4")]
    pub allow_cross_scheme_redirect: bool,
    /// Specifies a list of headers, by name, to copy from the internal redirect into the subsequent
    /// request. If a header is specified here but not present in the redirect, it will be cleared in
    /// the subsequent request.
    #[prost(string, repeated, tag = "5")]
    pub response_headers_to_copy: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
}
/// A simple wrapper for an HTTP filter config. This is intended to be used as a wrapper for the
/// map value in
/// :ref:`VirtualHost.typed_per_filter_config<envoy_v3_api_field_config.route.v3.VirtualHost.typed_per_filter_config>`,
/// :ref:`Route.typed_per_filter_config<envoy_v3_api_field_config.route.v3.Route.typed_per_filter_config>`,
/// or :ref:`WeightedCluster.ClusterWeight.typed_per_filter_config<envoy_v3_api_field_config.route.v3.WeightedCluster.ClusterWeight.typed_per_filter_config>`
/// to add additional flags to the filter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterConfig {
    /// The filter config.
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
    /// If true, the filter is optional, meaning that if the client does
    /// not support the specified filter, it may ignore the map entry rather
    /// than rejecting the config.
    #[prost(bool, tag = "2")]
    pub is_optional: bool,
    /// If true, the filter is disabled in the route or virtual host and the `config` field is ignored.
    /// See :ref:`route based filter chain <arch_overview_http_filters_route_based_filter_chain>`
    /// for more details.
    ///
    /// .. note::
    ///
    /// This field will take effect when the request arrive and filter chain is created for the request.
    /// If initial route is selected for the request and a filter is disabled in the initial route, then
    /// the filter will not be added to the filter chain.
    /// And if the request is mutated later and re-match to another route, the disabled filter by the
    /// initial route will not be added back to the filter chain because the filter chain is already
    /// created and it is too late to change the chain.
    ///
    /// This field only make sense for the downstream HTTP filters for now.
    #[prost(bool, tag = "3")]
    pub disabled: bool,
}
/// \[\#next-free-field: 18\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteConfiguration {
    /// The name of the route configuration. For example, it might match
    /// :ref:`route_config_name <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.Rds.route_config_name>` in
    /// :ref:`envoy_v3_api_msg_extensions.filters.network.http_connection_manager.v3.Rds`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// An array of virtual hosts that make up the route table.
    #[prost(message, repeated, tag = "2")]
    pub virtual_hosts: ::prost::alloc::vec::Vec<VirtualHost>,
    /// An array of virtual hosts will be dynamically loaded via the VHDS API.
    /// Both `virtual_hosts` and `vhds` fields will be used when present. `virtual_hosts` can be used
    /// for a base routing table or for infrequently changing virtual hosts. `vhds` is used for
    /// on-demand discovery of virtual hosts. The contents of these two fields will be merged to
    /// generate a routing table for a given RouteConfiguration, with `vhds` derived configuration
    /// taking precedence.
    #[prost(message, optional, tag = "9")]
    pub vhds: ::core::option::Option<Vhds>,
    /// Optionally specifies a list of HTTP headers that the connection manager
    /// will consider to be internal only. If they are found on external requests they will be cleaned
    /// prior to filter invocation. See :ref:`config_http_conn_man_headers_x-envoy-internal` for more
    /// information.
    #[prost(string, repeated, tag = "3")]
    pub internal_only_headers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Specifies a list of HTTP headers that should be added to each response that
    /// the connection manager encodes. Headers specified at this level are applied
    /// after headers from any enclosed :ref:`envoy_v3_api_msg_config.route.v3.VirtualHost` or
    /// :ref:`envoy_v3_api_msg_config.route.v3.RouteAction`. For more information, including details on
    /// header value syntax, see the documentation on :ref:`custom request headers <config_http_conn_man_headers_custom_request_headers>`.
    #[prost(message, repeated, tag = "4")]
    pub response_headers_to_add: ::prost::alloc::vec::Vec<
        super::super::core::v3::HeaderValueOption,
    >,
    /// Specifies a list of HTTP headers that should be removed from each response
    /// that the connection manager encodes.
    #[prost(string, repeated, tag = "5")]
    pub response_headers_to_remove: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Specifies a list of HTTP headers that should be added to each request
    /// routed by the HTTP connection manager. Headers specified at this level are
    /// applied after headers from any enclosed :ref:`envoy_v3_api_msg_config.route.v3.VirtualHost` or
    /// :ref:`envoy_v3_api_msg_config.route.v3.RouteAction`. For more information, including details on
    /// header value syntax, see the documentation on :ref:`custom request headers <config_http_conn_man_headers_custom_request_headers>`.
    #[prost(message, repeated, tag = "6")]
    pub request_headers_to_add: ::prost::alloc::vec::Vec<
        super::super::core::v3::HeaderValueOption,
    >,
    /// Specifies a list of HTTP headers that should be removed from each request
    /// routed by the HTTP connection manager.
    #[prost(string, repeated, tag = "8")]
    pub request_headers_to_remove: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Headers mutations at all levels are evaluated, if specified. By default, the order is from most
    /// specific (i.e. route entry level) to least specific (i.e. route configuration level). Later header
    /// mutations may override earlier mutations.
    /// This order can be reversed by setting this field to true. In other words, most specific level mutation
    /// is evaluated last.
    #[prost(bool, tag = "10")]
    pub most_specific_header_mutations_wins: bool,
    /// An optional boolean that specifies whether the clusters that the route
    /// table refers to will be validated by the cluster manager. If set to true
    /// and a route refers to a non-existent cluster, the route table will not
    /// load. If set to false and a route refers to a non-existent cluster, the
    /// route table will load and the router filter will return a 404 if the route
    /// is selected at runtime. This setting defaults to true if the route table
    /// is statically defined via the :ref:`route_config <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.route_config>`
    /// option. This setting default to false if the route table is loaded dynamically via the
    /// :ref:`rds <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.rds>`
    /// option. Users may wish to override the default behavior in certain cases (for example when
    /// using CDS with a static route table).
    #[prost(message, optional, tag = "7")]
    pub validate_clusters: ::core::option::Option<
        super::super::super::super::google::protobuf::BoolValue,
    >,
    /// The maximum bytes of the response :ref:`direct response body <envoy_v3_api_field_config.route.v3.DirectResponseAction.body>` size. If not specified the default
    /// is 4096.
    ///
    /// .. warning::
    ///
    /// Envoy currently holds the content of :ref:`direct response body <envoy_v3_api_field_config.route.v3.DirectResponseAction.body>` in memory. Be careful setting
    /// this to be larger than the default 4KB, since the allocated memory for direct response body
    /// is not subject to data plane buffering controls.
    #[prost(message, optional, tag = "11")]
    pub max_direct_response_body_size_bytes: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// A list of plugins and their configurations which may be used by a
    /// :ref:`cluster specifier plugin name <envoy_v3_api_field_config.route.v3.RouteAction.cluster_specifier_plugin>`
    /// within the route. All `extension.name` fields in this list must be unique.
    #[prost(message, repeated, tag = "12")]
    pub cluster_specifier_plugins: ::prost::alloc::vec::Vec<ClusterSpecifierPlugin>,
    /// Specify a set of default request mirroring policies which apply to all routes under its virtual hosts.
    /// Note that policies are not merged, the most specific non-empty one becomes the mirror policies.
    #[prost(message, repeated, tag = "13")]
    pub request_mirror_policies: ::prost::alloc::vec::Vec<
        route_action::RequestMirrorPolicy,
    >,
    /// By default, port in :authority header (if any) is used in host matching.
    /// With this option enabled, Envoy will ignore the port number in the :authority header (if any) when picking VirtualHost.
    /// NOTE: this option will not strip the port number (if any) contained in route config
    /// :ref:`envoy_v3_api_msg_config.route.v3.VirtualHost`.domains field.
    #[prost(bool, tag = "14")]
    pub ignore_port_in_host_matching: bool,
    /// Ignore path-parameters in path-matching.
    /// Before RFC3986, URI were like(RFC1808): <scheme>://\<net_loc>/<path>;<params>?<query>\#<fragment>
    /// Envoy by default takes ":path" as "<path>;<params>".
    /// For users who want to only match path on the "<path>" portion, this option should be true.
    #[prost(bool, tag = "15")]
    pub ignore_path_parameters_in_path_matching: bool,
    /// This field can be used to provide RouteConfiguration level per filter config. The key should match the
    /// :ref:`filter config name <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpFilter.name>`.
    /// See :ref:`Http filter route specific config <arch_overview_http_filters_per_filter_config>`
    /// for details.
    /// \[\#comment: An entry's value may be wrapped in a
    /// :ref:`FilterConfig<envoy_v3_api_msg_config.route.v3.FilterConfig>`
    /// message to specify additional options.\]
    #[prost(map = "string, message", tag = "16")]
    pub typed_per_filter_config: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::super::super::google::protobuf::Any,
    >,
    /// The metadata field can be used to provide additional information
    /// about the route configuration. It can be used for configuration, stats, and logging.
    /// The metadata should go under the filter namespace that will need it.
    /// For instance, if the metadata is intended for the Router filter,
    /// the filter name should be specified as `envoy.filters.http.router`.
    #[prost(message, optional, tag = "17")]
    pub metadata: ::core::option::Option<super::super::core::v3::Metadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vhds {
    /// Configuration source specifier for VHDS.
    #[prost(message, optional, tag = "1")]
    pub config_source: ::core::option::Option<super::super::core::v3::ConfigSource>,
}
/// Specifies a routing scope, which associates a
/// :ref:`Key<envoy_v3_api_msg_config.route.v3.ScopedRouteConfiguration.Key>` to a
/// :ref:`envoy_v3_api_msg_config.route.v3.RouteConfiguration`.
/// The :ref:`envoy_v3_api_msg_config.route.v3.RouteConfiguration` can be obtained dynamically
/// via RDS (:ref:`route_configuration_name<envoy_v3_api_field_config.route.v3.ScopedRouteConfiguration.route_configuration_name>`)
/// or specified inline (:ref:`route_configuration<envoy_v3_api_field_config.route.v3.ScopedRouteConfiguration.route_configuration>`).
///
/// The HTTP connection manager builds up a table consisting of these Key to
/// RouteConfiguration mappings, and looks up the RouteConfiguration to use per
/// request according to the algorithm specified in the
/// :ref:`scope_key_builder<envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.ScopedRoutes.scope_key_builder>`
/// assigned to the HttpConnectionManager.
///
/// For example, with the following configurations (in YAML):
///
/// HttpConnectionManager config:
///
/// .. code::
///
/// ...
/// scoped_routes:
/// name: foo-scoped-routes
/// scope_key_builder:
/// fragments:
/// - header_value_extractor:
/// name: X-Route-Selector
/// element_separator: ","
/// element:
/// separator: =
/// key: vip
///
/// ScopedRouteConfiguration resources (specified statically via
/// :ref:`scoped_route_configurations_list<envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.ScopedRoutes.scoped_route_configurations_list>`
/// or obtained dynamically via SRDS):
///
/// .. code::
///
/// (1)
/// name: route-scope1
/// route_configuration_name: route-config1
/// key:
/// fragments:
/// - string_key: 172.10.10.20
///
/// (2)
/// name: route-scope2
/// route_configuration_name: route-config2
/// key:
/// fragments:
/// - string_key: 172.20.20.30
///
/// A request from a client such as:
///
/// .. code::
///
/// ```text
/// GET / HTTP/1.1
/// Host: foo.com
/// X-Route-Selector: vip=172.10.10.20
/// ```
///
/// would result in the routing table defined by the `route-config1`
/// RouteConfiguration being assigned to the HTTP request/stream.
///
/// \[\#next-free-field: 6\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScopedRouteConfiguration {
    /// Whether the RouteConfiguration should be loaded on demand.
    #[prost(bool, tag = "4")]
    pub on_demand: bool,
    /// The name assigned to the routing scope.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The resource name to use for a :ref:`envoy_v3_api_msg_service.discovery.v3.DiscoveryRequest` to an
    /// RDS server to fetch the :ref:`envoy_v3_api_msg_config.route.v3.RouteConfiguration` associated
    /// with this scope.
    #[prost(string, tag = "2")]
    pub route_configuration_name: ::prost::alloc::string::String,
    /// The :ref:`envoy_v3_api_msg_config.route.v3.RouteConfiguration` associated with the scope.
    #[prost(message, optional, tag = "5")]
    pub route_configuration: ::core::option::Option<RouteConfiguration>,
    /// The key to match against.
    #[prost(message, optional, tag = "3")]
    pub key: ::core::option::Option<scoped_route_configuration::Key>,
}
/// Nested message and enum types in `ScopedRouteConfiguration`.
pub mod scoped_route_configuration {
    /// Specifies a key which is matched against the output of the
    /// :ref:`scope_key_builder<envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.ScopedRoutes.scope_key_builder>`
    /// specified in the HttpConnectionManager. The matching is done per HTTP
    /// request and is dependent on the order of the fragments contained in the
    /// Key.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Key {
        /// The ordered set of fragments to match against. The order must match the
        /// fragments in the corresponding
        /// :ref:`scope_key_builder<envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.ScopedRoutes.scope_key_builder>`.
        #[prost(message, repeated, tag = "1")]
        pub fragments: ::prost::alloc::vec::Vec<key::Fragment>,
    }
    /// Nested message and enum types in `Key`.
    pub mod key {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Fragment {
            #[prost(oneof = "fragment::Type", tags = "1")]
            pub r#type: ::core::option::Option<fragment::Type>,
        }
        /// Nested message and enum types in `Fragment`.
        pub mod fragment {
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Type {
                /// A string to match against.
                #[prost(string, tag = "1")]
                StringKey(::prost::alloc::string::String),
            }
        }
    }
}
