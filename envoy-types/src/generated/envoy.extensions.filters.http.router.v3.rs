/// \[\#next-free-field: 10\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Router {
    /// Whether the router generates dynamic cluster statistics. Defaults to
    /// true. Can be disabled in high performance scenarios.
    #[prost(message, optional, tag = "1")]
    pub dynamic_stats: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::BoolValue,
    >,
    /// Whether to start a child span for egress routed calls. This can be
    /// useful in scenarios where other filters (auth, ratelimit, etc.) make
    /// outbound calls and have child spans rooted at the same ingress
    /// parent. Defaults to false.
    #[prost(bool, tag = "2")]
    pub start_child_span: bool,
    /// Configuration for HTTP upstream logs emitted by the router. Upstream logs
    /// are configured in the same way as access logs, but each log entry represents
    /// an upstream request. Presuming retries are configured, multiple upstream
    /// requests may be made for each downstream (inbound) request.
    #[prost(message, repeated, tag = "3")]
    pub upstream_log: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::accesslog::v3::AccessLog,
    >,
    /// Additional upstream access log options.
    #[prost(message, optional, tag = "9")]
    pub upstream_log_options: ::core::option::Option<router::UpstreamAccessLogOptions>,
    /// Do not add any additional `x-envoy-` headers to requests or responses. This
    /// only affects the :ref:`router filter generated x-envoy- headers <config_http_filters_router_headers_set>`, other Envoy filters and the HTTP
    /// connection manager may continue to set `x-envoy-` headers.
    #[prost(bool, tag = "4")]
    pub suppress_envoy_headers: bool,
    /// Specifies a list of HTTP headers to strictly validate. Envoy will reject a
    /// request and respond with HTTP status 400 if the request contains an invalid
    /// value for any of the headers listed in this field. Strict header checking
    /// is only supported for the following headers:
    ///
    /// Value must be a ','-delimited list (i.e. no spaces) of supported retry
    /// policy values:
    ///
    /// * :ref:`config_http_filters_router_x-envoy-retry-grpc-on`
    /// * :ref:`config_http_filters_router_x-envoy-retry-on`
    ///
    /// Value must be an integer:
    ///
    /// * :ref:`config_http_filters_router_x-envoy-max-retries`
    /// * :ref:`config_http_filters_router_x-envoy-upstream-rq-timeout-ms`
    /// * :ref:`config_http_filters_router_x-envoy-upstream-rq-per-try-timeout-ms`
    #[prost(string, repeated, tag = "5")]
    pub strict_check_headers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If not set, ingress Envoy will ignore
    /// :ref:`config_http_filters_router_x-envoy-expected-rq-timeout-ms` header, populated by egress
    /// Envoy, when deriving timeout for upstream cluster.
    #[prost(bool, tag = "6")]
    pub respect_expected_rq_timeout: bool,
    /// If set, Envoy will avoid incrementing HTTP failure code stats
    /// on gRPC requests. This includes the individual status code value
    /// (e.g. upstream_rq_504) and group stats (e.g. upstream_rq_5xx).
    /// This field is useful if interested in relying only on the gRPC
    /// stats filter to define success and failure metrics for gRPC requests
    /// as not all failed gRPC requests charge HTTP status code metrics. See
    /// :ref:`gRPC stats filter<config_http_filters_grpc_stats>` documentation
    /// for more details.
    #[prost(bool, tag = "7")]
    pub suppress_grpc_request_failure_code_stats: bool,
    /// .. note::
    /// Upstream HTTP filters are currently in alpha.
    ///
    /// Optional HTTP filters for the upstream filter chain.
    ///
    /// These filters will be applied for all requests that pass through the router.
    /// They will also be applied to shadowed requests.
    /// Upstream filters cannot change route or cluster.
    /// Upstream filters specified on the cluster will override these filters.
    ///
    /// If using upstream filters, please be aware that local errors sent by
    /// upstream filters will not trigger retries, and local errors sent by
    /// upstream filters will count as a final response if hedging is configured.
    /// \[\#extension-category: envoy.filters.http.upstream\]
    #[prost(message, repeated, tag = "8")]
    pub upstream_http_filters: ::prost::alloc::vec::Vec<
        super::super::super::network::http_connection_manager::v3::HttpFilter,
    >,
}
/// Nested message and enum types in `Router`.
pub mod router {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpstreamAccessLogOptions {
        /// If set to true, an upstream access log will be recorded when an upstream stream is
        /// associated to an http request. Note: Each HTTP request received for an already established
        /// connection will result in an upstream access log record. This includes, for example,
        /// consecutive HTTP requests over the same connection or a request that is retried.
        /// In case a retry is applied, an upstream access log will be recorded for each retry.
        #[prost(bool, tag = "1")]
        pub flush_upstream_log_on_upstream_stream: bool,
        /// The interval to flush the upstream access logs. By default, the router will flush an upstream
        /// access log on stream close, when the HTTP request is complete. If this field is set, the router
        /// will flush access logs periodically at the specified interval. This is especially useful in the
        /// case of long-lived requests, such as CONNECT and Websockets.
        /// The interval must be at least 1 millisecond.
        #[prost(message, optional, tag = "2")]
        pub upstream_log_flush_interval: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::Duration,
        >,
    }
}
