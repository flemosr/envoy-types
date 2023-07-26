/// [#next-free-field: 6]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FaultAbort {
    /// The percentage of requests/operations/connections that will be aborted with the error code
    /// provided.
    #[prost(message, optional, tag = "3")]
    pub percentage: ::core::option::Option<
        super::super::super::super::super::r#type::v3::FractionalPercent,
    >,
    #[prost(oneof = "fault_abort::ErrorType", tags = "2, 5, 4")]
    pub error_type: ::core::option::Option<fault_abort::ErrorType>,
}
/// Nested message and enum types in `FaultAbort`.
pub mod fault_abort {
    /// Fault aborts are controlled via an HTTP header (if applicable). See the
    /// :ref:`HTTP fault filter <config_http_filters_fault_injection_http_header>` documentation for
    /// more information.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HeaderAbort {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ErrorType {
        /// HTTP status code to use to abort the HTTP request.
        #[prost(uint32, tag = "2")]
        HttpStatus(u32),
        /// gRPC status code to use to abort the gRPC request.
        #[prost(uint32, tag = "5")]
        GrpcStatus(u32),
        /// Fault aborts are controlled via an HTTP header (if applicable).
        #[prost(message, tag = "4")]
        HeaderAbort(HeaderAbort),
    }
}
/// [#next-free-field: 17]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpFault {
    /// If specified, the filter will inject delays based on the values in the
    /// object.
    #[prost(message, optional, tag = "1")]
    pub delay: ::core::option::Option<
        super::super::super::common::fault::v3::FaultDelay,
    >,
    /// If specified, the filter will abort requests based on the values in
    /// the object. At least ``abort`` or ``delay`` must be specified.
    #[prost(message, optional, tag = "2")]
    pub abort: ::core::option::Option<FaultAbort>,
    /// Specifies the name of the (destination) upstream cluster that the
    /// filter should match on. Fault injection will be restricted to requests
    /// bound to the specific upstream cluster.
    #[prost(string, tag = "3")]
    pub upstream_cluster: ::prost::alloc::string::String,
    /// Specifies a set of headers that the filter should match on. The fault
    /// injection filter can be applied selectively to requests that match a set of
    /// headers specified in the fault filter config. The chances of actual fault
    /// injection further depend on the value of the :ref:`percentage
    /// <envoy_v3_api_field_extensions.filters.http.fault.v3.FaultAbort.percentage>` field.
    /// The filter will check the request's headers against all the specified
    /// headers in the filter config. A match will happen if all the headers in the
    /// config are present in the request with the same values (or based on
    /// presence if the ``value`` field is not in the config).
    #[prost(message, repeated, tag = "4")]
    pub headers: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::route::v3::HeaderMatcher,
    >,
    /// Faults are injected for the specified list of downstream hosts. If this
    /// setting is not set, faults are injected for all downstream nodes.
    /// Downstream node name is taken from :ref:`the HTTP
    /// x-envoy-downstream-service-node
    /// <config_http_conn_man_headers_downstream-service-node>` header and compared
    /// against downstream_nodes list.
    #[prost(string, repeated, tag = "5")]
    pub downstream_nodes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The maximum number of faults that can be active at a single time via the configured fault
    /// filter. Note that because this setting can be overridden at the route level, it's possible
    /// for the number of active faults to be greater than this value (if injected via a different
    /// route). If not specified, defaults to unlimited. This setting can be overridden via
    /// ``runtime <config_http_filters_fault_injection_runtime>`` and any faults that are not injected
    /// due to overflow will be indicated via the ``faults_overflow
    /// <config_http_filters_fault_injection_stats>`` stat.
    ///
    /// .. attention::
    ///    Like other :ref:`circuit breakers <arch_overview_circuit_break>` in Envoy, this is a fuzzy
    ///    limit. It's possible for the number of active faults to rise slightly above the configured
    ///    amount due to the implementation details.
    #[prost(message, optional, tag = "6")]
    pub max_active_faults: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// The response rate limit to be applied to the response body of the stream. When configured,
    /// the percentage can be overridden by the :ref:`fault.http.rate_limit.response_percent
    /// <config_http_filters_fault_injection_runtime>` runtime key.
    ///
    /// .. attention::
    ///   This is a per-stream limit versus a connection level limit. This means that concurrent streams
    ///   will each get an independent limit.
    #[prost(message, optional, tag = "7")]
    pub response_rate_limit: ::core::option::Option<
        super::super::super::common::fault::v3::FaultRateLimit,
    >,
    /// The runtime key to override the :ref:`default <config_http_filters_fault_injection_runtime>`
    /// runtime. The default is: fault.http.delay.fixed_delay_percent
    #[prost(string, tag = "8")]
    pub delay_percent_runtime: ::prost::alloc::string::String,
    /// The runtime key to override the :ref:`default <config_http_filters_fault_injection_runtime>`
    /// runtime. The default is: fault.http.abort.abort_percent
    #[prost(string, tag = "9")]
    pub abort_percent_runtime: ::prost::alloc::string::String,
    /// The runtime key to override the :ref:`default <config_http_filters_fault_injection_runtime>`
    /// runtime. The default is: fault.http.delay.fixed_duration_ms
    #[prost(string, tag = "10")]
    pub delay_duration_runtime: ::prost::alloc::string::String,
    /// The runtime key to override the :ref:`default <config_http_filters_fault_injection_runtime>`
    /// runtime. The default is: fault.http.abort.http_status
    #[prost(string, tag = "11")]
    pub abort_http_status_runtime: ::prost::alloc::string::String,
    /// The runtime key to override the :ref:`default <config_http_filters_fault_injection_runtime>`
    /// runtime. The default is: fault.http.max_active_faults
    #[prost(string, tag = "12")]
    pub max_active_faults_runtime: ::prost::alloc::string::String,
    /// The runtime key to override the :ref:`default <config_http_filters_fault_injection_runtime>`
    /// runtime. The default is: fault.http.rate_limit.response_percent
    #[prost(string, tag = "13")]
    pub response_rate_limit_percent_runtime: ::prost::alloc::string::String,
    /// The runtime key to override the :ref:`default <config_http_filters_fault_injection_runtime>`
    /// runtime. The default is: fault.http.abort.grpc_status
    #[prost(string, tag = "14")]
    pub abort_grpc_status_runtime: ::prost::alloc::string::String,
    /// To control whether stats storage is allocated dynamically for each downstream server.
    /// If set to true, "x-envoy-downstream-service-cluster" field of header will be ignored by this filter.
    /// If set to false, dynamic stats storage will be allocated for the downstream cluster name.
    /// Default value is false.
    #[prost(bool, tag = "15")]
    pub disable_downstream_cluster_stats: bool,
    /// When an abort or delay fault is executed, the metadata struct provided here will be added to the
    /// request's dynamic metadata under the namespace corresponding to the name of the fault filter.
    /// This data can be logged as part of Access Logs using the :ref:`command operator
    /// <config_access_log_command_operators>` %DYNAMIC_METADATA(NAMESPACE)%, where NAMESPACE is the name of
    /// the fault filter.
    #[prost(message, optional, tag = "16")]
    pub filter_metadata: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Struct,
    >,
}
