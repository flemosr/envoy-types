/// This extension validates that HTTP request and response headers are well formed according to respective RFCs.
///
/// \#. HTTP/1 header map validity according to `RFC 7230 section 3.2 <<https://datatracker.ietf.org/doc/html/rfc7230#section-3.2>`\_>
/// \#. Syntax of HTTP/1 request target URI and response status
/// \#. HTTP/2 header map validity according to `RFC 7540 section 8.1.2 <<https://datatracker.ietf.org/doc/html/rfc7540#section-8.1.2>`\_>
/// \#. Syntax of HTTP/2 pseudo headers
/// \#. HTTP/3 header map validity according to `RFC 9114 section 4.3  <<https://www.rfc-editor.org/rfc/rfc9114.html>`\_>
/// \#. Syntax of HTTP/3 pseudo headers
/// \#. Syntax of Content-Length and Transfer-Encoding
/// \#. Validation of HTTP/1 requests with both `Content-Length` and `Transfer-Encoding` headers
/// \#. Normalization of the URI path according to `Normalization and Comparison <<https://datatracker.ietf.org/doc/html/rfc3986#section-6>`\_>
/// without `case normalization <<https://datatracker.ietf.org/doc/html/rfc3986#section-6.2.2.1>`\_>
///
/// \[\#comment:TODO(yanavlasov): Put #extension: envoy.http.header_validators.envoy_default after it is not hidden any more\]
/// \[\#next-free-field: 6\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderValidatorConfig {
    #[prost(message, optional, tag = "1")]
    pub http1_protocol_options: ::core::option::Option<
        header_validator_config::Http1ProtocolOptions,
    >,
    /// The URI path normalization options.
    /// By default Envoy normalizes URI path using the default values of the :ref:`UriPathNormalizationOptions <envoy_v3_api_msg_extensions.http.header_validators.envoy_default.v3.HeaderValidatorConfig.UriPathNormalizationOptions>`.
    /// URI path transformations specified by the `uri_path_normalization_options` configuration can be applied to a portion
    /// of requests by setting the `envoy_default_header_validator.uri_path_transformations` runtime value.
    /// Caution: disabling path normalization may lead to path confusion vulnerabilities in access control or incorrect service
    /// selection.
    #[prost(message, optional, tag = "2")]
    pub uri_path_normalization_options: ::core::option::Option<
        header_validator_config::UriPathNormalizationOptions,
    >,
    /// Restrict HTTP methods to these defined in the `RFC 7231 section 4.1 <<https://datatracker.ietf.org/doc/html/rfc7231#section-4.1>`\_>
    /// Envoy will respond with 400 to requests with disallowed methods.
    /// By default methods with arbitrary names are accepted.
    #[prost(bool, tag = "3")]
    pub restrict_http_methods: bool,
    /// Action to take when a client request with a header name containing underscore characters is received.
    /// If this setting is not specified, the value defaults to ALLOW.
    #[prost(
        enumeration = "header_validator_config::HeadersWithUnderscoresAction",
        tag = "4"
    )]
    pub headers_with_underscores_action: i32,
    /// Allow requests with fragment in URL path and strip the fragment before request processing.
    /// By default Envoy rejects requests with fragment in URL path.
    #[prost(bool, tag = "5")]
    pub strip_fragment_from_path: bool,
}
/// Nested message and enum types in `HeaderValidatorConfig`.
pub mod header_validator_config {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UriPathNormalizationOptions {
        /// Should paths be normalized according to RFC 3986?
        /// This operation overwrites the original request URI path and the new path is used for processing of
        /// the request by HTTP filters and proxied to the upstream service.
        /// Envoy will respond with 400 to requests with malformed paths that fail path normalization.
        /// The default behavior is to normalize the path.
        /// This value may be overridden by the runtime variable
        /// :ref:`http_connection_manager.normalize_path<config_http_conn_man_runtime_normalize_path>`.
        /// See `Normalization and Comparison <<https://datatracker.ietf.org/doc/html/rfc3986#section-6>`\_>
        /// for details of normalization.
        /// Note that Envoy does not perform
        /// `case normalization <<https://datatracker.ietf.org/doc/html/rfc3986#section-6.2.2.1>`\_>
        /// URI path normalization can be applied to a portion of requests by setting the
        /// `envoy_default_header_validator.path_normalization` runtime value.
        #[prost(bool, tag = "1")]
        pub skip_path_normalization: bool,
        /// Determines if adjacent slashes in the path are merged into one.
        /// This operation overwrites the original request URI path and the new path is used for processing of
        /// the request by HTTP filters and proxied to the upstream service.
        /// Setting this option to true will cause incoming requests with path `//dir///file` to not match against
        /// route with `prefix` match set to `/dir`. Defaults to `false`. Note that slash merging is not part of
        /// `HTTP spec <<https://datatracker.ietf.org/doc/html/rfc3986>`\_> and is provided for convenience.
        /// Merging of slashes in URI path can be applied to a portion of requests by setting the
        /// `envoy_default_header_validator.merge_slashes` runtime value.
        #[prost(bool, tag = "2")]
        pub skip_merging_slashes: bool,
        /// The action to take when request URL path contains escaped slash sequences (`%2F`, `%2f`, `%5C` and `%5c`).
        /// This operation may overwrite the original request URI path and the new path is used for processing of
        /// the request by HTTP filters and proxied to the upstream service.
        #[prost(
            enumeration = "uri_path_normalization_options::PathWithEscapedSlashesAction",
            tag = "3"
        )]
        pub path_with_escaped_slashes_action: i32,
    }
    /// Nested message and enum types in `UriPathNormalizationOptions`.
    pub mod uri_path_normalization_options {
        /// Determines the action for requests that contain `%2F`, `%2f`, `%5C` or `%5c` sequences in the URI path.
        /// This operation occurs before URL normalization and the merge slashes transformations if they were enabled.
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum PathWithEscapedSlashesAction {
            /// Default behavior specific to implementation (i.e. Envoy) of this configuration option.
            /// Envoy, by default, takes the `KEEP_UNCHANGED` action.
            /// NOTE: the implementation may change the default behavior at-will.
            ImplementationSpecificDefault = 0,
            /// Keep escaped slashes.
            KeepUnchanged = 1,
            /// Reject client request with the 400 status. gRPC requests will be rejected with the `INTERNAL` (13) error code.
            /// The `http#.downstream_rq_failed_path_normalization` counter is incremented for each rejected request.
            RejectRequest = 2,
            /// Unescape `%2F` and `%5C` sequences and redirect the request to the new path if these sequences were present.
            /// The redirect occurs after path normalization and merge slashes transformations if they were configured.
            /// NOTE: gRPC requests will be rejected with the `INTERNAL` (13) error code.
            /// This option minimizes possibility of path confusion exploits by forcing request with unescaped slashes to
            /// traverse all parties: downstream client, intermediate proxies, Envoy and upstream server.
            /// The `http#.downstream_rq_redirected_with_normalized_path` counter is incremented for each
            /// redirected request.
            UnescapeAndRedirect = 3,
            /// Unescape `%2F` and `%5C` sequences.
            /// Note: this option should not be enabled if intermediaries perform path based access control as
            /// it may lead to path confusion vulnerabilities.
            UnescapeAndForward = 4,
        }
        impl PathWithEscapedSlashesAction {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    PathWithEscapedSlashesAction::ImplementationSpecificDefault => {
                        "IMPLEMENTATION_SPECIFIC_DEFAULT"
                    }
                    PathWithEscapedSlashesAction::KeepUnchanged => "KEEP_UNCHANGED",
                    PathWithEscapedSlashesAction::RejectRequest => "REJECT_REQUEST",
                    PathWithEscapedSlashesAction::UnescapeAndRedirect => {
                        "UNESCAPE_AND_REDIRECT"
                    }
                    PathWithEscapedSlashesAction::UnescapeAndForward => {
                        "UNESCAPE_AND_FORWARD"
                    }
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "IMPLEMENTATION_SPECIFIC_DEFAULT" => {
                        Some(Self::ImplementationSpecificDefault)
                    }
                    "KEEP_UNCHANGED" => Some(Self::KeepUnchanged),
                    "REJECT_REQUEST" => Some(Self::RejectRequest),
                    "UNESCAPE_AND_REDIRECT" => Some(Self::UnescapeAndRedirect),
                    "UNESCAPE_AND_FORWARD" => Some(Self::UnescapeAndForward),
                    _ => None,
                }
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Http1ProtocolOptions {
        /// Allows Envoy to process HTTP/1 requests/responses with both `Content-Length` and `Transfer-Encoding`
        /// headers set. By default such messages are rejected, but if option is enabled - Envoy will
        /// remove the `Content-Length` header and process the message.
        /// See `RFC7230, sec. 3.3.3 <<https://datatracker.ietf.org/doc/html/rfc7230#section-3.3.3>`\_> for details.
        ///
        /// .. attention::
        /// Enabling this option might lead to request smuggling vulnerabilities, especially if traffic
        /// is proxied via multiple layers of proxies.
        #[prost(bool, tag = "1")]
        pub allow_chunked_length: bool,
    }
    /// Action to take when Envoy receives client request with header names containing underscore
    /// characters.
    /// Underscore character is allowed in header names by the RFC-7230 and this behavior is implemented
    /// as a security measure due to systems that treat '\_' and '-' as interchangeable. Envoy by default allows client request headers with underscore
    /// characters.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum HeadersWithUnderscoresAction {
        /// Allow headers with underscores. This is the default behavior.
        Allow = 0,
        /// Reject client request. HTTP/1 requests are rejected with the 400 status. HTTP/2 requests
        /// end with the stream reset. The
        /// :ref:`httpN.requests_rejected_with_underscores_in_headers <config_http_conn_man_stats_per_codec>` counter
        /// is incremented for each rejected request.
        RejectRequest = 1,
        /// Drop the client header with name containing underscores. The header is dropped before the filter chain is
        /// invoked and as such filters will not see dropped headers. The
        /// :ref:`httpN.dropped_headers_with_underscores <config_http_conn_man_stats_per_codec>` is incremented for
        /// each dropped header.
        DropHeader = 2,
    }
    impl HeadersWithUnderscoresAction {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                HeadersWithUnderscoresAction::Allow => "ALLOW",
                HeadersWithUnderscoresAction::RejectRequest => "REJECT_REQUEST",
                HeadersWithUnderscoresAction::DropHeader => "DROP_HEADER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ALLOW" => Some(Self::Allow),
                "REJECT_REQUEST" => Some(Self::RejectRequest),
                "DROP_HEADER" => Some(Self::DropHeader),
                _ => None,
            }
        }
    }
}
