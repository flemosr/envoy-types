/// Custom configuration for an :ref:`AccessLog <envoy_v3_api_msg_config.accesslog.v3.AccessLog>`
/// that writes log entries directly to the operating system's standard output.
/// [#extension: envoy.access_loggers.stdout]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StdoutAccessLog {
    #[prost(oneof = "stdout_access_log::AccessLogFormat", tags = "1")]
    pub access_log_format: ::core::option::Option<stdout_access_log::AccessLogFormat>,
}
/// Nested message and enum types in `StdoutAccessLog`.
pub mod stdout_access_log {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AccessLogFormat {
        /// Configuration to form access log data and format.
        /// If not specified, use :ref:`default format <config_access_log_default_format>`.
        #[prost(message, tag = "1")]
        LogFormat(
            super::super::super::super::super::config::core::v3::SubstitutionFormatString,
        ),
    }
}
/// Custom configuration for an :ref:`AccessLog <envoy_v3_api_msg_config.accesslog.v3.AccessLog>`
/// that writes log entries directly to the operating system's standard error.
/// [#extension: envoy.access_loggers.stderr]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StderrAccessLog {
    #[prost(oneof = "stderr_access_log::AccessLogFormat", tags = "1")]
    pub access_log_format: ::core::option::Option<stderr_access_log::AccessLogFormat>,
}
/// Nested message and enum types in `StderrAccessLog`.
pub mod stderr_access_log {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AccessLogFormat {
        /// Configuration to form access log data and format.
        /// If not specified, use :ref:`default format <config_access_log_default_format>`.
        #[prost(message, tag = "1")]
        LogFormat(
            super::super::super::super::super::config::core::v3::SubstitutionFormatString,
        ),
    }
}
