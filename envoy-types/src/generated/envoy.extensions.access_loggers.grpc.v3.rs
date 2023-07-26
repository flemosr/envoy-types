/// Configuration for the built-in ``envoy.access_loggers.http_grpc``
/// :ref:`AccessLog <envoy_v3_api_msg_config.accesslog.v3.AccessLog>`. This configuration will
/// populate :ref:`StreamAccessLogsMessage.http_logs
/// <envoy_v3_api_field_service.accesslog.v3.StreamAccessLogsMessage.http_logs>`.
/// [#extension: envoy.access_loggers.http_grpc]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpGrpcAccessLogConfig {
    #[prost(message, optional, tag = "1")]
    pub common_config: ::core::option::Option<CommonGrpcAccessLogConfig>,
    /// Additional request headers to log in :ref:`HTTPRequestProperties.request_headers
    /// <envoy_v3_api_field_data.accesslog.v3.HTTPRequestProperties.request_headers>`.
    #[prost(string, repeated, tag = "2")]
    pub additional_request_headers_to_log: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Additional response headers to log in :ref:`HTTPResponseProperties.response_headers
    /// <envoy_v3_api_field_data.accesslog.v3.HTTPResponseProperties.response_headers>`.
    #[prost(string, repeated, tag = "3")]
    pub additional_response_headers_to_log: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Additional response trailers to log in :ref:`HTTPResponseProperties.response_trailers
    /// <envoy_v3_api_field_data.accesslog.v3.HTTPResponseProperties.response_trailers>`.
    #[prost(string, repeated, tag = "4")]
    pub additional_response_trailers_to_log: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
}
/// Configuration for the built-in ``envoy.access_loggers.tcp_grpc`` type. This configuration will
/// populate ``StreamAccessLogsMessage.tcp_logs``.
/// [#extension: envoy.access_loggers.tcp_grpc]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TcpGrpcAccessLogConfig {
    #[prost(message, optional, tag = "1")]
    pub common_config: ::core::option::Option<CommonGrpcAccessLogConfig>,
}
/// Common configuration for gRPC access logs.
/// [#next-free-field: 9]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonGrpcAccessLogConfig {
    /// The friendly name of the access log to be returned in :ref:`StreamAccessLogsMessage.Identifier
    /// <envoy_v3_api_msg_service.accesslog.v3.StreamAccessLogsMessage.Identifier>`. This allows the
    /// access log server to differentiate between different access logs coming from the same Envoy.
    #[prost(string, tag = "1")]
    pub log_name: ::prost::alloc::string::String,
    /// The gRPC service for the access log service.
    #[prost(message, optional, tag = "2")]
    pub grpc_service: ::core::option::Option<
        super::super::super::super::config::core::v3::GrpcService,
    >,
    /// API version for access logs service transport protocol. This describes the access logs service
    /// gRPC endpoint and version of messages used on the wire.
    #[prost(
        enumeration = "super::super::super::super::config::core::v3::ApiVersion",
        tag = "6"
    )]
    pub transport_api_version: i32,
    /// Interval for flushing access logs to the gRPC stream. Logger will flush requests every time
    /// this interval is elapsed, or when batch size limit is hit, whichever comes first. Defaults to
    /// 1 second.
    #[prost(message, optional, tag = "3")]
    pub buffer_flush_interval: ::core::option::Option<
        super::super::super::super::super::google::protobuf::Duration,
    >,
    /// Soft size limit in bytes for access log entries buffer. Logger will buffer requests until
    /// this limit it hit, or every time flush interval is elapsed, whichever comes first. Setting it
    /// to zero effectively disables the batching. Defaults to 16384.
    #[prost(message, optional, tag = "4")]
    pub buffer_size_bytes: ::core::option::Option<
        super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// Additional filter state objects to log in :ref:`filter_state_objects
    /// <envoy_v3_api_field_data.accesslog.v3.AccessLogCommon.filter_state_objects>`.
    /// Logger will call ``FilterState::Object::serializeAsProto`` to serialize the filter state object.
    #[prost(string, repeated, tag = "5")]
    pub filter_state_objects_to_log: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Sets the retry policy when the establishment of a gRPC stream fails.
    /// If the stream succeeds at least once in establishing itself,
    /// no retry will be performed no matter what gRPC status is received.
    /// Note that only :ref:`num_retries <envoy_v3_api_field_config.core.v3.RetryPolicy.num_retries>`
    /// will be used in this configuration. This feature is used only when you are using
    /// :ref:`Envoy gRPC client <envoy_v3_api_field_config.core.v3.GrpcService.envoy_grpc>`.
    #[prost(message, optional, tag = "7")]
    pub grpc_stream_retry_policy: ::core::option::Option<
        super::super::super::super::config::core::v3::RetryPolicy,
    >,
    /// A list of custom tags with unique tag name to create tags for the logs.
    #[prost(message, repeated, tag = "8")]
    pub custom_tags: ::prost::alloc::vec::Vec<
        super::super::super::super::r#type::tracing::v3::CustomTag,
    >,
}
