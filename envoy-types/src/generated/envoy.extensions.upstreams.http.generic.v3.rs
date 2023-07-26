/// A connection pool which forwards downstream HTTP as TCP or HTTP to upstream,
/// based on CONNECT configuration.
/// [#extension: envoy.upstreams.http.generic]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericConnectionPoolProto {}
