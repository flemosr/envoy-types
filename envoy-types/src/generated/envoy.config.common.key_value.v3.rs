/// This shared configuration for Envoy key value stores.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValueStoreConfig {
    /// [#extension-category: envoy.common.key_value]
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<
        super::super::super::core::v3::TypedExtensionConfig,
    >,
}
