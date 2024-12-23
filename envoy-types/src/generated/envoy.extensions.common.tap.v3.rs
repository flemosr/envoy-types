// This file is @generated by prost-build.
/// Common configuration for all tap extensions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonExtensionConfig {
    #[prost(oneof = "common_extension_config::ConfigType", tags = "1, 2")]
    pub config_type: ::core::option::Option<common_extension_config::ConfigType>,
}
/// Nested message and enum types in `CommonExtensionConfig`.
pub mod common_extension_config {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConfigType {
        /// If specified, the tap filter will be configured via an admin handler.
        #[prost(message, tag = "1")]
        AdminConfig(super::AdminConfig),
        /// If specified, the tap filter will be configured via a static configuration that cannot be
        /// changed.
        #[prost(message, tag = "2")]
        StaticConfig(super::super::super::super::super::config::tap::v3::TapConfig),
    }
}
/// Configuration for the admin handler. See :ref:`here <config_http_filters_tap_admin_handler>` for
/// more information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminConfig {
    /// Opaque configuration ID. When requests are made to the admin handler, the passed opaque ID is
    /// matched to the configured filter opaque ID to determine which filter to configure.
    #[prost(string, tag = "1")]
    pub config_id: ::prost::alloc::string::String,
}
