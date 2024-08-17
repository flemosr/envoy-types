/// Filter configuration.
/// \[\#next-free-field: 7\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcpAuthnFilterConfig {
    /// The HTTP URI to fetch tokens from GCE Metadata Server(<https://cloud.google.com/compute/docs/metadata/overview>).
    /// The URL format is "<http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/identity?audience=\[AUDIENCE\]">
    ///
    /// This field is deprecated because it does not match the API surface provided by the google auth libraries.
    /// Control planes should not attempt to override the metadata server URI.
    /// The cluster and timeout can be configured using the `cluster` and `timeout` fields instead.
    /// For backward compatibility, the cluster and timeout configured in this field will be used
    /// if the new `cluster` and `timeout` fields are not set.
    #[deprecated]
    #[prost(message, optional, tag = "1")]
    pub http_uri: ::core::option::Option<
        super::super::super::super::super::config::core::v3::HttpUri,
    >,
    /// Retry policy for fetching tokens.
    /// Not supported by all data planes.
    #[prost(message, optional, tag = "2")]
    pub retry_policy: ::core::option::Option<
        super::super::super::super::super::config::core::v3::RetryPolicy,
    >,
    /// Token cache configuration. This field is optional.
    #[prost(message, optional, tag = "3")]
    pub cache_config: ::core::option::Option<TokenCacheConfig>,
    /// Request header location to extract the token. By default (i.e. if this field is not specified), the token
    /// is extracted to the Authorization HTTP header, in the format "Authorization: Bearer <token>".
    /// Not supported by all data planes.
    #[prost(message, optional, tag = "4")]
    pub token_header: ::core::option::Option<TokenHeader>,
    /// Cluster to send traffic to the GCE metadata server. Not supported
    /// by all data planes; a data plane may instead have its own mechanism
    /// for contacting the metadata server.
    #[prost(string, tag = "5")]
    pub cluster: ::prost::alloc::string::String,
    /// Timeout for fetching the tokens from the GCE metadata server.
    /// Not supported by all data planes.
    #[prost(message, optional, tag = "6")]
    pub timeout: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
}
/// Audience is the URL of the receiving service that performs token authentication.
/// It will be provided to the filter through cluster's typed_filter_metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Audience {
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
}
/// Token Cache configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenCacheConfig {
    /// The number of cache entries. The maximum number of entries is INT64_MAX as it is constrained by underlying cache implementation.
    /// Default value 0 (i.e., proto3 defaults) disables the cache by default. Other default values will enable the cache.
    #[prost(message, optional, tag = "1")]
    pub cache_size: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt64Value,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenHeader {
    /// The HTTP header's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The header's prefix. The format is "value_prefix<token>"
    /// For example, for "Authorization: Bearer <token>", value_prefix="Bearer " with a space at the
    /// end.
    #[prost(string, tag = "2")]
    pub value_prefix: ::prost::alloc::string::String,
}
