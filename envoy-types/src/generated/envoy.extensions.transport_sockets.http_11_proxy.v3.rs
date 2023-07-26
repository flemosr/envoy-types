/// Configuration for HTTP/1.1 proxy transport sockets.
/// This is intended for use in Envoy Mobile, though may eventually be extended
/// for upstream Envoy use.
/// If this transport socket is configured, and an intermediate filter adds the
/// stream info necessary for proxying to the stream info (as the test filter
/// does :repo:`here <test/integration/filters/header_to_proxy_filter.cc>`) then
///
/// * Upstream connections will be directed to the specified proxy address rather
///    than the host's address
/// * Upstream TLS connections will have a raw HTTP/1.1 CONNECT header prefaced
///    to the payload, and 200 response stripped (if less than 200 bytes)
/// * Plaintext HTTP/1.1 connections will be sent with a fully qualified URL.
///
/// This transport socket is not compatible with HTTP/3, plaintext HTTP/2, or raw TCP.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Http11ProxyUpstreamTransport {
    /// The underlying transport socket being wrapped.
    #[prost(message, optional, tag = "1")]
    pub transport_socket: ::core::option::Option<
        super::super::super::super::config::core::v3::TransportSocket,
    >,
}
