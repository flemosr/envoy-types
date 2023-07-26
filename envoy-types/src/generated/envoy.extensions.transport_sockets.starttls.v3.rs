/// Configuration for a downstream StartTls transport socket.
/// StartTls transport socket wraps two sockets:
///
/// * raw_buffer socket which is used at the beginning of the session
/// * TLS socket used when a protocol negotiates a switch to encrypted traffic.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartTlsConfig {
    /// (optional) Configuration for clear-text socket used at the beginning of the session.
    #[prost(message, optional, tag = "1")]
    pub cleartext_socket_config: ::core::option::Option<
        super::super::raw_buffer::v3::RawBuffer,
    >,
    /// Configuration for a downstream TLS socket.
    #[prost(message, optional, tag = "2")]
    pub tls_socket_config: ::core::option::Option<
        super::super::tls::v3::DownstreamTlsContext,
    >,
}
/// Configuration for an upstream StartTls transport socket.
/// StartTls transport socket wraps two sockets:
///
/// * raw_buffer socket which is used at the beginning of the session
/// * TLS socket used when a protocol negotiates a switch to encrypted traffic.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpstreamStartTlsConfig {
    /// (optional) Configuration for clear-text socket used at the beginning of the session.
    #[prost(message, optional, tag = "1")]
    pub cleartext_socket_config: ::core::option::Option<
        super::super::raw_buffer::v3::RawBuffer,
    >,
    /// Configuration for an upstream TLS socket.
    #[prost(message, optional, tag = "2")]
    pub tls_socket_config: ::core::option::Option<
        super::super::tls::v3::UpstreamTlsContext,
    >,
}
