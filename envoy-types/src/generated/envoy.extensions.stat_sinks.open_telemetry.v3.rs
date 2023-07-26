/// [#next-free-field: 6]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SinkConfig {
    /// If set to true, counters will be emitted as deltas, and the OTLP message will have
    /// ``AGGREGATION_TEMPORALITY_DELTA`` set as AggregationTemporality.
    #[prost(bool, tag = "2")]
    pub report_counters_as_deltas: bool,
    /// If set to true, histograms will be emitted as deltas, and the OTLP message will have
    /// ``AGGREGATION_TEMPORALITY_DELTA`` set as AggregationTemporality.
    #[prost(bool, tag = "3")]
    pub report_histograms_as_deltas: bool,
    /// If set to true, metrics will have their tags emitted as OTLP attributes, which may
    /// contain values used by the tag extractor or additional tags added during stats creation.
    /// Otherwise, no attributes will be associated with the export message. Default value is true.
    #[prost(message, optional, tag = "4")]
    pub emit_tags_as_attributes: ::core::option::Option<
        super::super::super::super::super::google::protobuf::BoolValue,
    >,
    /// If set to true, metric names will be represented as the tag extracted name instead
    /// of the full metric name. Default value is true.
    #[prost(message, optional, tag = "5")]
    pub use_tag_extracted_name: ::core::option::Option<
        super::super::super::super::super::google::protobuf::BoolValue,
    >,
    #[prost(oneof = "sink_config::ProtocolSpecifier", tags = "1")]
    pub protocol_specifier: ::core::option::Option<sink_config::ProtocolSpecifier>,
}
/// Nested message and enum types in `SinkConfig`.
pub mod sink_config {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ProtocolSpecifier {
        /// The upstream gRPC cluster that implements the OTLP/gRPC collector.
        #[prost(message, tag = "1")]
        GrpcService(super::super::super::super::super::config::core::v3::GrpcService),
    }
}
