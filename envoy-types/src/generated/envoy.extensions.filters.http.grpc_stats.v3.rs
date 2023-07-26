/// gRPC statistics filter configuration
/// [#next-free-field: 6]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterConfig {
    /// If true, the filter maintains a filter state object with the request and response message
    /// counts.
    #[prost(bool, tag = "1")]
    pub emit_filter_state: bool,
    /// If true, the filter will gather a histogram for the request time of the upstream.
    /// It works with :ref:`stats_for_all_methods
    /// <envoy_v3_api_field_extensions.filters.http.grpc_stats.v3.FilterConfig.stats_for_all_methods>`
    /// and :ref:`individual_method_stats_allowlist
    /// <envoy_v3_api_field_extensions.filters.http.grpc_stats.v3.FilterConfig.individual_method_stats_allowlist>` the same way
    /// request_message_count and response_message_count works.
    #[prost(bool, tag = "4")]
    pub enable_upstream_stats: bool,
    /// If true, the filter will replace dots in the grpc_service_name with underscores before emitting
    /// the metrics. Only works when :ref:`stats_for_all_methods
    /// <envoy_v3_api_field_extensions.filters.http.grpc_stats.v3.FilterConfig.stats_for_all_methods>`
    /// is set to true. It could cause metrics to be merged if the edited service name conflicts with
    /// an existing service. For example there are both service "foo.bar" & "foo_bar" running.
    /// This config can fix incorrect gRPC metrics with dots because the existing stats tag extractor
    /// assumes no dots in the gRPC service name. By default this is set as false.
    #[prost(bool, tag = "5")]
    pub replace_dots_in_grpc_service_name: bool,
    #[prost(oneof = "filter_config::PerMethodStatSpecifier", tags = "2, 3")]
    pub per_method_stat_specifier: ::core::option::Option<
        filter_config::PerMethodStatSpecifier,
    >,
}
/// Nested message and enum types in `FilterConfig`.
pub mod filter_config {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PerMethodStatSpecifier {
        /// If set, specifies an allowlist of service/methods that will have individual stats
        /// emitted for them. Any call that does not match the allowlist will be counted
        /// in a stat with no method specifier: ``cluster.<name>.grpc.*``.
        #[prost(message, tag = "2")]
        IndividualMethodStatsAllowlist(
            super::super::super::super::super::super::config::core::v3::GrpcMethodList,
        ),
        /// If set to true, emit stats for all service/method names.
        ///
        /// If set to false, emit stats for all service/message types to the same stats without including
        /// the service/method in the name, with prefix ``cluster.<name>.grpc``. This can be useful if
        /// service/method granularity is not needed, or if each cluster only receives a single method.
        ///
        /// .. attention::
        ///    This option is only safe if all clients are trusted. If this option is enabled
        ///    with untrusted clients, the clients could cause unbounded growth in the number of stats in
        ///    Envoy, using unbounded memory and potentially slowing down stats pipelines.
        ///
        /// .. attention::
        ///    If neither ``individual_method_stats_allowlist`` nor ``stats_for_all_methods`` is set, the
        ///    behavior will default to ``stats_for_all_methods=false``. This default value is changed due
        ///    to the previous value being deprecated. This behavior can be changed with runtime override
        ///    ``envoy.deprecated_features.grpc_stats_filter_enable_stats_for_all_methods_by_default``.
        #[prost(message, tag = "3")]
        StatsForAllMethods(
            super::super::super::super::super::super::super::google::protobuf::BoolValue,
        ),
    }
}
/// gRPC statistics filter state object in protobuf form.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterObject {
    /// Count of request messages in the request stream.
    #[prost(uint64, tag = "1")]
    pub request_message_count: u64,
    /// Count of response messages in the response stream.
    #[prost(uint64, tag = "2")]
    pub response_message_count: u64,
}
