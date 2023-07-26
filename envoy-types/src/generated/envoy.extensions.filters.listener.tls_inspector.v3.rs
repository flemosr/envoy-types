#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TlsInspector {
    /// Populate ``JA3`` fingerprint hash using data from the TLS Client Hello packet. Default is false.
    #[prost(message, optional, tag = "1")]
    pub enable_ja3_fingerprinting: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::BoolValue,
    >,
}
