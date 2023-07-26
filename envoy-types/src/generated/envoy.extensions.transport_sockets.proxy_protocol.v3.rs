/// Configuration for PROXY protocol socket
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyProtocolUpstreamTransport {
    /// The PROXY protocol settings
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<
        super::super::super::super::config::core::v3::ProxyProtocolConfig,
    >,
    /// The underlying transport socket being wrapped.
    #[prost(message, optional, tag = "2")]
    pub transport_socket: ::core::option::Option<
        super::super::super::super::config::core::v3::TransportSocket,
    >,
}
