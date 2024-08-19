// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LuaConfig {
    /// The lua code that Envoy will execute to select cluster.
    #[prost(message, optional, tag = "1")]
    pub source_code: ::core::option::Option<
        super::super::super::super::super::config::core::v3::DataSource,
    >,
    /// Default cluster. It will be used when the lua code execute failure.
    #[prost(string, tag = "2")]
    pub default_cluster: ::prost::alloc::string::String,
}
