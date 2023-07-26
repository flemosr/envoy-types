/// Configuration for the built-in ``envoy.access_loggers.open_telemetry``
/// :ref:`AccessLog <envoy_v3_api_msg_config.accesslog.v3.AccessLog>`. This configuration will
/// populate `opentelemetry.proto.collector.v1.logs.ExportLogsServiceRequest.resource_logs <<https://github.com/open-telemetry/opentelemetry-proto/blob/main/opentelemetry/proto/collector/logs/v1/logs_service.proto>`_.>
/// In addition, the request start time is set in the dedicated field.
/// [#extension: envoy.access_loggers.open_telemetry]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenTelemetryAccessLogConfig {
    /// [#comment:TODO(itamarkam): add 'filter_state_objects_to_log' to logs.]
    #[prost(message, optional, tag = "1")]
    pub common_config: ::core::option::Option<
        super::super::grpc::v3::CommonGrpcAccessLogConfig,
    >,
    /// OpenTelemetry `Resource <<https://github.com/open-telemetry/opentelemetry-proto/blob/main/opentelemetry/proto/logs/v1/logs.proto#L51>`_>
    /// attributes are filled with Envoy node info.
    /// Example: ``resource_attributes { values { key: "region" value { string_value: "cn-north-7" } } }``.
    #[prost(message, optional, tag = "4")]
    pub resource_attributes: ::core::option::Option<
        super::super::super::super::super::opentelemetry::proto::common::v1::KeyValueList,
    >,
    /// OpenTelemetry `LogResource <<https://github.com/open-telemetry/opentelemetry-proto/blob/main/opentelemetry/proto/logs/v1/logs.proto>`_>
    /// fields, following `Envoy access logging formatting <<https://www.envoyproxy.io/docs/envoy/latest/configuration/observability/access_log/usage>`_.>
    ///
    /// See 'body' in the LogResource proto for more details.
    /// Example: ``body { string_value: "%PROTOCOL%" }``.
    #[prost(message, optional, tag = "2")]
    pub body: ::core::option::Option<
        super::super::super::super::super::opentelemetry::proto::common::v1::AnyValue,
    >,
    /// See 'attributes' in the LogResource proto for more details.
    /// Example: ``attributes { values { key: "user_agent" value { string_value: "%REQ(USER-AGENT)%" } } }``.
    #[prost(message, optional, tag = "3")]
    pub attributes: ::core::option::Option<
        super::super::super::super::super::opentelemetry::proto::common::v1::KeyValueList,
    >,
}
