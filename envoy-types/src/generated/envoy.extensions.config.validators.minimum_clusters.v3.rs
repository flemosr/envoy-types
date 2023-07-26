/// Validates a CDS config, and ensures that the number of clusters is above the
/// set threshold.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MinimumClustersValidator {
    /// The minimal clusters threshold. Any CDS config update leading to less than
    /// this number will be rejected.
    /// Default value is 0.
    #[prost(uint32, tag = "1")]
    pub min_clusters_num: u32,
}
