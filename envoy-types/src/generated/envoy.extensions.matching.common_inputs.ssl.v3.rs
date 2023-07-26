/// List of comma-delimited URIs in the SAN field of the peer certificate for a downstream.
/// [#extension: envoy.matching.inputs.uri_san]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UriSanInput {}
/// List of comma-delimited DNS entries in the SAN field of the peer certificate for a downstream.
/// [#extension: envoy.matching.inputs.dns_san]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DnsSanInput {}
/// Input that matches the subject field of the peer certificate in RFC 2253 format for a
/// downstream.
/// [#extension: envoy.matching.inputs.subject]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubjectInput {}
