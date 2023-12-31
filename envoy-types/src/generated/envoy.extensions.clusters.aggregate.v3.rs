/// Configuration for the aggregate cluster. See the :ref:`architecture overview <arch_overview_aggregate_cluster>` for more information.
/// \[\#extension: envoy.clusters.aggregate\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterConfig {
    /// Load balancing clusters in aggregate cluster. Clusters are prioritized based on the order they
    /// appear in this list.
    #[prost(string, repeated, tag = "1")]
    pub clusters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
