/// Custom response policy to serve a locally stored response to the
/// downstream.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalResponsePolicy {
    /// Optional new local reply body text. It will be used
    /// in the `%LOCAL_REPLY_BODY%` command operator in the `body_format`.
    #[prost(message, optional, tag = "1")]
    pub body: ::core::option::Option<
        super::super::super::super::super::config::core::v3::DataSource,
    >,
    /// Optional body format to be used for this response. If `body_format` is  not
    /// provided, and `body` is, the contents of `body` will be used to populate
    /// the body of the local reply without formatting.
    #[prost(message, optional, tag = "2")]
    pub body_format: ::core::option::Option<
        super::super::super::super::super::config::core::v3::SubstitutionFormatString,
    >,
    /// The new response status code if specified.
    #[prost(message, optional, tag = "3")]
    pub status_code: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// HTTP headers to add to the response. This allows the
    /// response policy to append, to add or to override headers of
    /// the original response for local body, or the custom response from the
    /// remote body, before it is sent to a downstream client.
    #[prost(message, repeated, tag = "4")]
    pub response_headers_to_add: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::core::v3::HeaderValueOption,
    >,
}
