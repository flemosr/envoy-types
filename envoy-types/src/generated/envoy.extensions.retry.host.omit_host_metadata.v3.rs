/// A retry host predicate that can be used to reject a host based on
/// predefined metadata match criteria.
/// \[\#extension: envoy.retry_host_predicates.omit_host_metadata\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OmitHostMetadataConfig {
    /// Retry host predicate metadata match criteria. The hosts in
    /// the upstream cluster with matching metadata will be omitted while
    /// attempting a retry of a failed request. The metadata should be specified
    /// under the `envoy.lb` key.
    #[prost(message, optional, tag = "1")]
    pub metadata_match: ::core::option::Option<
        super::super::super::super::super::config::core::v3::Metadata,
    >,
}
