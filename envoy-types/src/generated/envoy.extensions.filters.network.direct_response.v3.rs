#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    /// Response data as a data source.
    #[prost(message, optional, tag = "1")]
    pub response: ::core::option::Option<
        super::super::super::super::super::config::core::v3::DataSource,
    >,
}
