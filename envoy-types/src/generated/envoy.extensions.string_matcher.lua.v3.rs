#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Lua {
    /// The Lua code that Envoy will execute
    #[prost(message, optional, tag = "1")]
    pub source_code: ::core::option::Option<
        super::super::super::super::config::core::v3::DataSource,
    >,
}
