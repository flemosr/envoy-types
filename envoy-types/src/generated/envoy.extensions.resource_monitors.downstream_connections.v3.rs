/// The downstream connections resource monitor tracks the global number of open downstream connections.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownstreamConnectionsConfig {
    /// Maximum threshold for global open downstream connections, defaults to 0.
    /// If monitor is enabled in Overload manager api, this field should be explicitly configured with value greater than 0.
    #[prost(int64, tag = "1")]
    pub max_active_downstream_connections: i64,
}
