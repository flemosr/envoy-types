/// Specifies that matching should be performed by the destination IP address.
/// [#extension: envoy.matching.inputs.destination_ip]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestinationIpInput {}
/// Specifies that matching should be performed by the destination port.
/// [#extension: envoy.matching.inputs.destination_port]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestinationPortInput {}
/// Specifies that matching should be performed by the source IP address.
/// [#extension: envoy.matching.inputs.source_ip]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceIpInput {}
/// Specifies that matching should be performed by the source port.
/// [#extension: envoy.matching.inputs.source_port]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourcePortInput {}
/// Input that matches by the directly connected source IP address (this
/// will only be different from the source IP address when using a listener
/// filter that overrides the source address, such as the :ref:`Proxy Protocol
/// listener filter <config_listener_filters_proxy_protocol>`).
/// [#extension: envoy.matching.inputs.direct_source_ip]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectSourceIpInput {}
/// Input that matches by the source IP type.
/// Specifies the source IP match type. The values include:
///
/// * ``local`` - matches a connection originating from the same host,
/// [#extension: envoy.matching.inputs.source_type]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceTypeInput {}
/// Input that matches by the requested server name (e.g. SNI in TLS).
///
/// :ref:`TLS Inspector <config_listener_filters_tls_inspector>` provides the requested server name based on SNI,
/// when TLS protocol is detected.
/// [#extension: envoy.matching.inputs.server_name]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerNameInput {}
/// Input that matches by the transport protocol.
///
/// Suggested values include:
///
/// * ``raw_buffer`` - default, used when no transport protocol is detected,
/// * ``tls`` - set by :ref:`envoy.filters.listener.tls_inspector <config_listener_filters_tls_inspector>`
///    when TLS protocol is detected.
/// [#extension: envoy.matching.inputs.transport_protocol]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransportProtocolInput {}
/// List of quoted and comma-separated requested application protocols. The list consists of a
/// single negotiated application protocol once the network stream is established.
///
/// Examples:
///
/// * ``'h2','http/1.1'``
/// * ``'h2c'``
///
/// Suggested values in the list include:
///
/// * ``http/1.1`` - set by :ref:`envoy.filters.listener.tls_inspector
///    <config_listener_filters_tls_inspector>` and :ref:`envoy.filters.listener.http_inspector
///    <config_listener_filters_http_inspector>`,
/// * ``h2`` - set by :ref:`envoy.filters.listener.tls_inspector <config_listener_filters_tls_inspector>`
/// * ``h2c`` - set by :ref:`envoy.filters.listener.http_inspector <config_listener_filters_http_inspector>`
///
/// .. attention::
///
///    Currently, :ref:`TLS Inspector <config_listener_filters_tls_inspector>` provides
///    application protocol detection based on the requested
///    `ALPN <<https://en.wikipedia.org/wiki/Application-Layer_Protocol_Negotiation>`_> values.
///
///    However, the use of ALPN is pretty much limited to the HTTP/2 traffic on the Internet,
///    and matching on values other than ``h2`` is going to lead to a lot of false negatives,
///    unless all connecting clients are known to use ALPN.
/// [#extension: envoy.matching.inputs.application_protocol]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplicationProtocolInput {}
/// Input that matches by a specific filter state key.
/// The value of the provided filter state key will be the raw string representation of the filter state object
/// [#extension: envoy.matching.inputs.filter_state]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterStateInput {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
}
