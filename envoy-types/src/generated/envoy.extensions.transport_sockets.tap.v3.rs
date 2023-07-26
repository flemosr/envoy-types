/// Configuration for tap transport socket. This wraps another transport socket, providing the
/// ability to interpose and record in plain text any traffic that is surfaced to Envoy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tap {
    /// Common configuration for the tap transport socket.
    #[prost(message, optional, tag = "1")]
    pub common_config: ::core::option::Option<
        super::super::super::common::tap::v3::CommonExtensionConfig,
    >,
    /// The underlying transport socket being wrapped.
    #[prost(message, optional, tag = "2")]
    pub transport_socket: ::core::option::Option<
        super::super::super::super::config::core::v3::TransportSocket,
    >,
}
