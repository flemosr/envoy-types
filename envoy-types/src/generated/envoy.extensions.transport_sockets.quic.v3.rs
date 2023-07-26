/// Configuration for Downstream QUIC transport socket. This provides Google's implementation of Google QUIC and IETF QUIC to Envoy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuicDownstreamTransport {
    #[prost(message, optional, tag = "1")]
    pub downstream_tls_context: ::core::option::Option<
        super::super::tls::v3::DownstreamTlsContext,
    >,
    /// If false, QUIC will tell TLS to reject any early data and to stop issuing 0-RTT credentials with resumption session tickets. This will prevent clients from sending 0-RTT requests.
    /// Default to true.
    #[prost(message, optional, tag = "2")]
    pub enable_early_data: ::core::option::Option<
        super::super::super::super::super::google::protobuf::BoolValue,
    >,
}
/// Configuration for Upstream QUIC transport socket. This provides Google's implementation of Google QUIC and IETF QUIC to Envoy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuicUpstreamTransport {
    #[prost(message, optional, tag = "1")]
    pub upstream_tls_context: ::core::option::Option<
        super::super::tls::v3::UpstreamTlsContext,
    >,
}
