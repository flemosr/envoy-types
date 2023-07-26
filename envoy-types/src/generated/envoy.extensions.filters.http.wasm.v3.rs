#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Wasm {
    /// General Plugin configuration.
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<
        super::super::super::super::wasm::v3::PluginConfig,
    >,
}
