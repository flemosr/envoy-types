/// Custom response policy to internally redirect the original response to a different
/// upstream.
/// [#next-free-field: 7]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedirectPolicy {
    /// The new response status code if specified. This is used to override the
    /// status code of the response from the new upstream if it is not an error status.
    #[prost(message, optional, tag = "3")]
    pub status_code: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// HTTP headers to add to the response. This allows the
    /// response policy to append, to add or to override headers of
    /// the original response for local body, or the custom response from the
    /// remote body, before it is sent to a downstream client.
    /// Note that these are not applied if the redirected response is an error
    /// response.
    #[prost(message, repeated, tag = "4")]
    pub response_headers_to_add: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::core::v3::HeaderValueOption,
    >,
    /// HTTP headers to add to the request before it is internally redirected.
    #[prost(message, repeated, tag = "5")]
    pub request_headers_to_add: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::core::v3::HeaderValueOption,
    >,
    /// Custom action to modify request headers before selection of the
    /// redirected route.
    /// [#comment: TODO(pradeepcrao) add an extension category.]
    #[prost(message, optional, tag = "6")]
    pub modify_request_headers_action: ::core::option::Option<
        super::super::super::super::super::config::core::v3::TypedExtensionConfig,
    >,
    #[prost(oneof = "redirect_policy::RedirectActionSpecifier", tags = "1, 2")]
    pub redirect_action_specifier: ::core::option::Option<
        redirect_policy::RedirectActionSpecifier,
    >,
}
/// Nested message and enum types in `RedirectPolicy`.
pub mod redirect_policy {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RedirectActionSpecifier {
        /// The Http URI to redirect the original request to, to get the custom
        /// response.
        /// It should be a full FQDN with protocol, host and path.
        ///
        /// Example:
        ///
        /// .. code-block:: yaml
        ///
        ///     uri: <https://www.mydomain.com/path/to/404.txt>
        ///
        #[prost(string, tag = "1")]
        Uri(::prost::alloc::string::String),
        /// Specify elements of the redirect url individually.
        /// Note: Do not specify the `response_code` field in `redirect_action`, use
        /// `status_code` instead.
        /// The following fields in `redirect_action` are currently not supported,
        /// and specifying them will cause the config to be rejected:
        /// - `prefix_rewrite`
        /// - `regex_rewrite`
        #[prost(message, tag = "2")]
        RedirectAction(
            super::super::super::super::super::super::config::route::v3::RedirectAction,
        ),
    }
}
