/// \[\#protodoc-title: Outlier detection error buckets\]
/// Error bucket for HTTP codes.
/// \[\#not-implemented-hide:\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpErrors {
    #[prost(message, optional, tag = "1")]
    pub range: ::core::option::Option<
        super::super::super::super::r#type::v3::Int32Range,
    >,
}
/// Error bucket for locally originated errors.
/// \[\#not-implemented-hide:\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalOriginErrors {}
/// Error bucket for database errors.
/// Sub-parameters may be added later, like malformed response, error on write, etc.
/// \[\#not-implemented-hide:\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseErrors {}
/// Union of possible error buckets.
/// \[\#not-implemented-hide:\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorBuckets {
    /// List of buckets "catching" HTTP codes.
    #[prost(message, repeated, tag = "1")]
    pub http_errors: ::prost::alloc::vec::Vec<HttpErrors>,
    /// List of buckets "catching" locally originated errors.
    #[prost(message, repeated, tag = "2")]
    pub local_origin_errors: ::prost::alloc::vec::Vec<LocalOriginErrors>,
    /// List of buckets "catching" database errors.
    #[prost(message, repeated, tag = "3")]
    pub database_errors: ::prost::alloc::vec::Vec<DatabaseErrors>,
}
