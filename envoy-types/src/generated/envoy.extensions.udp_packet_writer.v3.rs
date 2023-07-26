/// Configuration for the default UDP packet writer factory which simply
/// uses the kernel's sendmsg() to send UDP packets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UdpDefaultWriterFactory {}
/// Configuration for the UDP GSO batch packet writer factory.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UdpGsoBatchWriterFactory {}
