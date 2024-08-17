/// The HTTP/1.1 proxy transport socket opens an upstream connection to a specified proxy address
/// rather than the target host's address. If this transport socket is configured and proxy
/// information is configured, then:
///
/// * Upstream connections to the proxy address will have a raw HTTP/1.1 CONNECT header prefaced to
///   the payload, and 200 response stripped (if less than 200 bytes).
/// * Plaintext HTTP/1.1 connections will be sent with a fully qualified URL.
///
/// There are two primary ways to configure proxy information:
///
/// * An intermediate filter adds the stream info necessary for proxying to the stream info (as the
///   test filter does :repo:`here <test/integration/filters/header_to_proxy_filter.cc>`).
/// * Setting the "typed_filter_metadata" in :ref:`LbEndpoint.Metadata <envoy_v3_api_field_config.endpoint.v3.lbendpoint.metadata>`
///   or :ref:`LocalityLbEndpoints.Metadata <envoy_v3_api_field_config.endpoint.v3.LocalityLbEndpoints.metadata>` using the key
///   "envoy.http11_proxy_transport_socket.proxy_address" and the proxy address in
///   config::core::v3::Address format.
///
/// Some important notes regarding this transport socket:
///
/// * Configuration via stream info (as opposed to endpoint/locality metadata) will only proxy TLS
///   connections to the proxy address on port 443. This is to maintain the original behavior of the
///   transport socket when using this method of configuration.
/// * The transport socket is not compatible with HTTP/3 or plaintext HTTP/2.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Http11ProxyUpstreamTransport {
    /// The underlying transport socket being wrapped.
    #[prost(message, optional, tag = "1")]
    pub transport_socket: ::core::option::Option<
        super::super::super::super::config::core::v3::TransportSocket,
    >,
}
