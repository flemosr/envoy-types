/// Monitor which counts consecutive errors.
/// If number of consecutive errors exceeds the threshold, monitor will report that the host
/// is unhealthy.
/// \[\#not-implemented-hide:\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsecutiveErrors {
    /// Monitor name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The number of consecutive errors before ejection occurs.
    #[prost(message, optional, tag = "2")]
    pub threshold: ::core::option::Option<
        super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// The % chance that a host is actually ejected. Defaults to 100.
    #[prost(message, optional, tag = "3")]
    pub enforcing: ::core::option::Option<
        super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// Error buckets.
    #[prost(message, optional, tag = "4")]
    pub error_buckets: ::core::option::Option<super::super::common::v3::ErrorBuckets>,
}
