/// Custom configuration for an :ref:`AccessLog <envoy_v3_api_msg_config.accesslog.v3.AccessLog>`
/// that writes log entries directly to a file. Configures the built-in ``envoy.access_loggers.file``
/// AccessLog.
/// [#next-free-field: 6]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileAccessLog {
    /// A path to a local file to which to write the access log entries.
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    #[prost(oneof = "file_access_log::AccessLogFormat", tags = "2, 3, 4, 5")]
    pub access_log_format: ::core::option::Option<file_access_log::AccessLogFormat>,
}
/// Nested message and enum types in `FileAccessLog`.
pub mod file_access_log {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AccessLogFormat {
        /// Access log :ref:`format string<config_access_log_format_strings>`.
        /// Envoy supports :ref:`custom access log formats <config_access_log_format>` as well as a
        /// :ref:`default format <config_access_log_default_format>`.
        /// This field is deprecated.
        /// Please use :ref:`log_format <envoy_v3_api_field_extensions.access_loggers.file.v3.FileAccessLog.log_format>`.
        #[prost(string, tag = "2")]
        Format(::prost::alloc::string::String),
        /// Access log :ref:`format dictionary<config_access_log_format_dictionaries>`. All values
        /// are rendered as strings.
        /// This field is deprecated.
        /// Please use :ref:`log_format <envoy_v3_api_field_extensions.access_loggers.file.v3.FileAccessLog.log_format>`.
        #[prost(message, tag = "3")]
        JsonFormat(super::super::super::super::super::super::google::protobuf::Struct),
        /// Access log :ref:`format dictionary<config_access_log_format_dictionaries>`. Values are
        /// rendered as strings, numbers, or boolean values as appropriate. Nested JSON objects may
        /// be produced by some command operators (e.g.FILTER_STATE or DYNAMIC_METADATA). See the
        /// documentation for a specific command operator for details.
        /// This field is deprecated.
        /// Please use :ref:`log_format <envoy_v3_api_field_extensions.access_loggers.file.v3.FileAccessLog.log_format>`.
        #[prost(message, tag = "4")]
        TypedJsonFormat(
            super::super::super::super::super::super::google::protobuf::Struct,
        ),
        /// Configuration to form access log data and format.
        /// If not specified, use :ref:`default format <config_access_log_default_format>`.
        #[prost(message, tag = "5")]
        LogFormat(
            super::super::super::super::super::config::core::v3::SubstitutionFormatString,
        ),
    }
}
