// This file is @generated by prost-build.
/// This shared configuration for Envoy key value stores.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValueStoreConfig {
    /// \[\#extension-category: envoy.common.key_value\]
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<
        super::super::super::core::v3::TypedExtensionConfig,
    >,
}
