// This file is @generated by prost-build.
/// Configuration for a QUIC debug visitor which emits stats from the underlying QUIC transport.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Config {
    /// Period to update stats while the connection is open. If unset, updates only happen when the
    /// connection is closed. Stats are always updated one final time when the connection is closed.
    #[prost(message, optional, tag = "2")]
    pub update_period: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
}