#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GraphiteStatsdSink {
    /// Optional custom metric name prefix. See :ref:`StatsdSink's prefix field <envoy_v3_api_field_config.metrics.v3.StatsdSink.prefix>` for more details.
    #[prost(string, tag = "3")]
    pub prefix: ::prost::alloc::string::String,
    /// Optional max datagram size to use when sending UDP messages. By default Envoy
    /// will emit one metric per datagram. By specifying a max-size larger than a single
    /// metric, Envoy will emit multiple, new-line separated metrics. The max datagram
    /// size should not exceed your network's MTU.
    ///
    /// Note that this value may not be respected if smaller than a single metric.
    #[prost(message, optional, tag = "4")]
    pub max_bytes_per_datagram: ::core::option::Option<
        super::super::super::super::super::google::protobuf::UInt64Value,
    >,
    #[prost(oneof = "graphite_statsd_sink::StatsdSpecifier", tags = "1")]
    pub statsd_specifier: ::core::option::Option<graphite_statsd_sink::StatsdSpecifier>,
}
/// Nested message and enum types in `GraphiteStatsdSink`.
pub mod graphite_statsd_sink {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StatsdSpecifier {
        /// The UDP address of a running Graphite-compliant listener. If specified,
        /// statistics will be flushed to this address.
        #[prost(message, tag = "1")]
        Address(super::super::super::super::super::config::core::v3::Address),
    }
}
