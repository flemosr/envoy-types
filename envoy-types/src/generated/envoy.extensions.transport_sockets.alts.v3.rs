/// Configuration for ALTS transport socket. This provides Google's ALTS protocol to Envoy.
/// Store the peer identity in dynamic metadata, namespace is "envoy.transport_socket.peer_information", key is "peer_identity".
/// <https://cloud.google.com/security/encryption-in-transit/application-layer-transport-security/>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Alts {
    /// The location of a handshaker service, this is usually 169.254.169.254:8080
    /// on GCE.
    #[prost(string, tag = "1")]
    pub handshaker_service: ::prost::alloc::string::String,
    /// The acceptable service accounts from peer, peers not in the list will be rejected in the
    /// handshake validation step. If empty, no validation will be performed.
    #[prost(string, repeated, tag = "2")]
    pub peer_service_accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
