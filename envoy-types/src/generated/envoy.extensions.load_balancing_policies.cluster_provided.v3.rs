/// The cluster provided load balancing policy allows cluster to specify its own load balancing.
/// If this extension is configured, the target cluster must provide load balancer when the cluster
/// is created.
///
/// .. note::
/// Cluster provided load balancing policy could not be used as sub-policy of other hierarchical
/// load balancing policies, such as subset load balancing policy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterProvided {}
