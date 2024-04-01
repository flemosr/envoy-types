/// Configuration for the *envoy.access_loggers.fluentd* :ref:`AccessLog <envoy_v3_api_msg_config.accesslog.v3.AccessLog>`.
/// This access log extension will send the emitted access logs over a TCP connection to an upstream that is accepting
/// the Fluentd Forward Protocol as described in: `Fluentd Forward Protocol Specification <<https://github.com/fluent/fluentd/wiki/Forward-Protocol-Specification-v1>`\_.>
/// \[\#extension: envoy.access_loggers.fluentd\]
/// \[\#next-free-field: 9\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FluentdAccessLogConfig {
    /// The upstream cluster to connect to for streaming the Fluentd messages.
    #[prost(string, tag = "1")]
    pub cluster: ::prost::alloc::string::String,
    /// A tag is a string separated with '.' (e.g. log.type) to categorize events.
    /// See: <https://github.com/fluent/fluentd/wiki/Forward-Protocol-Specification-v1#message-modes>
    #[prost(string, tag = "2")]
    pub tag: ::prost::alloc::string::String,
    /// The prefix to use when emitting :ref:`statistics <config_access_log_stats>`.
    #[prost(string, tag = "3")]
    pub stat_prefix: ::prost::alloc::string::String,
    /// Interval for flushing access logs to the TCP stream. Logger will flush requests every time
    /// this interval is elapsed, or when batch size limit is hit, whichever comes first. Defaults to
    /// 1 second.
    #[prost(message, optional, tag = "4")]
    pub buffer_flush_interval: ::core::option::Option<
        super::super::super::super::super::google::protobuf::Duration,
    >,
    /// Soft size limit in bytes for access log entries buffer. The logger will buffer requests until
    /// this limit it hit, or every time flush interval is elapsed, whichever comes first. When the buffer
    /// limit is hit, the logger will immediately flush the buffer contents. Setting it to zero effectively
    /// disables the batching. Defaults to 16384.
    #[prost(message, optional, tag = "5")]
    pub buffer_size_bytes: ::core::option::Option<
        super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// A struct that represents the record that is sent for each log entry.
    /// <https://github.com/fluent/fluentd/wiki/Forward-Protocol-Specification-v1#entry>
    /// Values are rendered as strings, numbers, or boolean values as appropriate.
    /// Nested JSON objects may be produced by some command operators (e.g. FILTER_STATE or DYNAMIC_METADATA).
    /// See :ref:`format string<config_access_log_format_strings>` documentation for a specific command operator details.
    ///
    /// .. validated-code-block:: yaml
    /// :type-name: envoy.extensions.access_loggers.fluentd.v3.FluentdAccessLogConfig
    ///
    /// record:
    /// status: "%RESPONSE_CODE%"
    /// message: "%LOCAL_REPLY_BODY%"
    ///
    /// The following msgpack record would be created:
    ///
    /// .. code-block:: json
    ///
    /// {
    /// "status": 500,
    /// "message": "My error message"
    /// }
    #[prost(message, optional, tag = "6")]
    pub record: ::core::option::Option<
        super::super::super::super::super::google::protobuf::Struct,
    >,
    /// Optional retry, in case upstream connection has failed. If this field is not set, the default values will be applied,
    /// as specified in the :ref:`RetryOptions <envoy_v3_api_msg_extensions.access_loggers.fluentd.v3.FluentdAccessLogConfig.RetryOptions>`
    /// configuration.
    #[prost(message, optional, tag = "7")]
    pub retry_options: ::core::option::Option<fluentd_access_log_config::RetryOptions>,
    /// Specifies a collection of Formatter plugins that can be called from the access log configuration.
    /// See the formatters extensions documentation for details.
    /// \[\#extension-category: envoy.formatter\]
    #[prost(message, repeated, tag = "8")]
    pub formatters: ::prost::alloc::vec::Vec<
        super::super::super::super::config::core::v3::TypedExtensionConfig,
    >,
}
/// Nested message and enum types in `FluentdAccessLogConfig`.
pub mod fluentd_access_log_config {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RetryOptions {
        /// The number of times the logger will attempt to connect to the upstream during reconnects.
        /// By default, there is no limit. The logger will attempt to reconnect to the upstream each time
        /// connecting to the upstream failed or the upstream connection had been closed for any reason.
        #[prost(message, optional, tag = "1")]
        pub max_connect_attempts: ::core::option::Option<
            super::super::super::super::super::super::google::protobuf::UInt32Value,
        >,
        /// Sets the backoff strategy. If this value is not set, the default base backoff interval is 500
        /// milliseconds and the default max backoff interval is 5 seconds (10 times the base interval).
        #[prost(message, optional, tag = "2")]
        pub backoff_options: ::core::option::Option<
            super::super::super::super::super::config::core::v3::BackoffStrategy,
        >,
    }
}
