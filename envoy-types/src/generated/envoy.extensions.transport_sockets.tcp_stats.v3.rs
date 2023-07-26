/// Configuration for the TCP Stats transport socket wrapper, which wraps another transport socket for
/// all communication, but emits stats about the underlying TCP connection.
///
/// The stats are documented :ref:`here <config_listener_stats_tcp>` for listeners and
/// :ref:`here <config_cluster_manager_cluster_stats_tcp>` for clusters.
///
/// This transport socket is currently only supported on Linux.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    /// The underlying transport socket being wrapped.
    #[prost(message, optional, tag = "1")]
    pub transport_socket: ::core::option::Option<
        super::super::super::super::config::core::v3::TransportSocket,
    >,
    /// Period to update stats while the connection is open. If unset, updates only happen when the
    /// connection is closed. Stats are always updated one final time when the connection is closed.
    #[prost(message, optional, tag = "2")]
    pub update_period: ::core::option::Option<
        super::super::super::super::super::google::protobuf::Duration,
    >,
}
