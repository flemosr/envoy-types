#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Buffer {
    /// The maximum request size that the filter will buffer before the connection
    /// manager will stop buffering and return a 413 response.
    #[prost(message, optional, tag = "1")]
    pub max_request_bytes: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BufferPerRoute {
    #[prost(oneof = "buffer_per_route::Override", tags = "1, 2")]
    pub r#override: ::core::option::Option<buffer_per_route::Override>,
}
/// Nested message and enum types in `BufferPerRoute`.
pub mod buffer_per_route {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Override {
        /// Disable the buffer filter for this particular vhost or route.
        #[prost(bool, tag = "1")]
        Disabled(bool),
        /// Override the global configuration of the filter with this new config.
        #[prost(message, tag = "2")]
        Buffer(super::Buffer),
    }
}
