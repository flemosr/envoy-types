/// Configuration for the internal upstream address. An internal address defines
/// a loopback user space socket residing in the same proxy instance. This
/// extension allows passing additional structured state across the user space
/// socket in addition to the regular byte stream. The purpose is to facilitate
/// communication between filters on the downstream and the upstream internal
/// connections. All filter state objects that are shared with the upstream
/// connection are also shared with the downstream internal connection using
/// this transport socket.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InternalUpstreamTransport {
    /// Specifies the metadata namespaces and values to insert into the downstream
    /// internal connection dynamic metadata when an internal address is used as a
    /// host. If the destination name is repeated across two metadata source
    /// locations, and both locations contain the metadata with the given name,
    /// then the latter in the list overrides the former.
    #[prost(message, repeated, tag = "1")]
    pub passthrough_metadata: ::prost::alloc::vec::Vec<
        internal_upstream_transport::MetadataValueSource,
    >,
    /// The underlying transport socket being wrapped.
    #[prost(message, optional, tag = "3")]
    pub transport_socket: ::core::option::Option<
        super::super::super::super::config::core::v3::TransportSocket,
    >,
}
/// Nested message and enum types in `InternalUpstreamTransport`.
pub mod internal_upstream_transport {
    /// Describes the location of the imported metadata value.
    /// If the metadata with the given name is not present at the source location,
    /// then no metadata is passed through for this particular instance.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MetadataValueSource {
        /// Specifies what kind of metadata.
        #[prost(message, optional, tag = "1")]
        pub kind: ::core::option::Option<
            super::super::super::super::super::r#type::metadata::v3::MetadataKind,
        >,
        /// Name is the filter namespace used in the dynamic metadata.
        #[prost(string, tag = "2")]
        pub name: ::prost::alloc::string::String,
    }
}
