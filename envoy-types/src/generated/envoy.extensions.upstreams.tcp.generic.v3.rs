/// A connection pool which forwards downstream TCP as TCP or HTTP to upstream,
/// based on CONNECT configuration.
/// \[\#extension: envoy.upstreams.tcp.generic\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericConnectionPoolProto {}
