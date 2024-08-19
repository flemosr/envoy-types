// This file is @generated by prost-build.
/// \[\#next-free-field: 18\]
/// GrpcJsonTranscoder filter configuration.
/// The filter itself can be used per route / per virtual host or on the general level. The most
/// specific one is being used for a given route. If the list of services is empty - filter
/// is considered to be disabled.
/// Note that if specifying the filter per route, first the route is matched, and then transcoding
/// filter is applied. It matters when specifying the route configuration and paths to match the
/// request - for per-route grpc transcoder configs, the original path should be matched, while
/// in other cases, the grpc-like path is expected (the one AFTER the filter is applied).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrpcJsonTranscoder {
    /// A list of strings that
    /// supplies the fully qualified service names (i.e. "package_name.service_name") that
    /// the transcoder will translate. If the service name doesn't exist in `proto_descriptor`,
    /// Envoy will fail at startup. The `proto_descriptor` may contain more services than
    /// the service names specified here, but they won't be translated.
    ///
    /// By default, the filter will pass through requests that do not map to any specified services.
    /// If the list of services is empty, filter is considered disabled.
    /// However, this behavior changes if
    /// :ref:`reject_unknown_method <envoy_v3_api_field_extensions.filters.http.grpc_json_transcoder.v3.GrpcJsonTranscoder.RequestValidationOptions.reject_unknown_method>`
    /// is enabled.
    #[prost(string, repeated, tag = "2")]
    pub services: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Control options for response JSON. These options are passed directly to
    /// `JsonPrintOptions <<https://developers.google.com/protocol-buffers/docs/reference/cpp/> google.protobuf.util.json_util#JsonPrintOptions>`\_.
    #[prost(message, optional, tag = "3")]
    pub print_options: ::core::option::Option<grpc_json_transcoder::PrintOptions>,
    /// Whether to keep the incoming request route after the outgoing headers have been transformed to
    /// the match the upstream gRPC service. Note: This means that routes for gRPC services that are
    /// not transcoded cannot be used in combination with `match_incoming_request_route`.
    #[prost(bool, tag = "5")]
    pub match_incoming_request_route: bool,
    /// A list of query parameters to be ignored for transcoding method mapping.
    /// By default, the transcoder filter will not transcode a request if there are any
    /// unknown/invalid query parameters.
    ///
    /// Example :
    ///
    /// .. code-block:: proto
    ///
    /// ```text
    /// service Bookstore {
    ///    rpc GetShelf(GetShelfRequest) returns (Shelf) {
    ///      option (google.api.http) = {
    ///        get: "/shelves/{shelf}"
    ///      };
    ///    }
    /// }
    ///
    /// message GetShelfRequest {
    ///    int64 shelf = 1;
    /// }
    ///
    /// message Shelf {}
    /// ```
    ///
    /// The request `/shelves/100?foo=bar` will not be mapped to `` GetShelf``` because variable binding for  ``foo`is not defined. Adding`foo`to`ignored_query_parameters`will allow the same request to be mapped to`GetShelf\``.
    #[prost(string, repeated, tag = "6")]
    pub ignored_query_parameters: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Whether to route methods without the `google.api.http` option.
    ///
    /// Example :
    ///
    /// .. code-block:: proto
    ///
    /// ```text
    /// package bookstore;
    ///
    /// service Bookstore {
    ///    rpc GetShelf(GetShelfRequest) returns (Shelf) {}
    /// }
    ///
    /// message GetShelfRequest {
    ///    int64 shelf = 1;
    /// }
    ///
    /// message Shelf {}
    /// ```
    ///
    /// The client could `post` a json body `{"shelf": 1234}` with the path of
    /// `/bookstore.Bookstore/GetShelfRequest` to call `GetShelfRequest`.
    #[prost(bool, tag = "7")]
    pub auto_mapping: bool,
    /// Whether to ignore query parameters that cannot be mapped to a corresponding
    /// protobuf field. Use this if you cannot control the query parameters and do
    /// not know them beforehand. Otherwise use `ignored_query_parameters`.
    /// Defaults to false.
    #[prost(bool, tag = "8")]
    pub ignore_unknown_query_parameters: bool,
    /// Whether to convert gRPC status headers to JSON.
    /// When trailer indicates a gRPC error and there was no HTTP body, take `google.rpc.Status`
    /// from the `grpc-status-details-bin` header and use it as JSON body.
    /// If there was no such header, make `google.rpc.Status` out of the `grpc-status` and
    /// `grpc-message` headers.
    /// The error details types must be present in the `proto_descriptor`.
    ///
    /// For example, if an upstream server replies with headers:
    ///
    /// .. code-block:: none
    ///
    /// ```text
    /// grpc-status: 5
    /// grpc-status-details-bin:
    ///      CAUaMwoqdHlwZS5nb29nbGVhcGlzLmNvbS9nb29nbGUucnBjLlJlcXVlc3RJbmZvEgUKA3ItMQ
    /// ```
    ///
    /// The `grpc-status-details-bin` header contains a base64-encoded protobuf message
    /// `google.rpc.Status`. It will be transcoded into:
    ///
    /// .. code-block:: none
    ///
    /// ```text
    /// HTTP/1.1 404 Not Found
    /// content-type: application/json
    ///
    /// {"code":5,"details":\[{"@type":"type.googleapis.com/google.rpc.RequestInfo","requestId":"r-1"}\]}
    /// ```
    ///
    /// In order to transcode the message, the `google.rpc.RequestInfo` type from
    /// the `google/rpc/error_details.proto` should be included in the configured
    /// :ref:`proto descriptor set <config_grpc_json_generate_proto_descriptor_set>`.
    #[prost(bool, tag = "9")]
    pub convert_grpc_status: bool,
    /// URL unescaping policy.
    /// This spec is only applied when extracting variable with multiple segments in the URL path.
    /// For example, in case of `/foo/{x=*}/bar/{y=prefix/*}/{z=**}` `x` variable is single segment and `y` and `z` are multiple segments.
    /// For a path with `/foo/first/bar/prefix/second/third/fourth`, `x=first`, `y=prefix/second`, `z=third/fourth`.
    /// If this setting is not specified, the value defaults to :ref:`ALL_CHARACTERS_EXCEPT_RESERVED<envoy_v3_api_enum_value_extensions.filters.http.grpc_json_transcoder.v3.GrpcJsonTranscoder.UrlUnescapeSpec.ALL_CHARACTERS_EXCEPT_RESERVED>`.
    #[prost(enumeration = "grpc_json_transcoder::UrlUnescapeSpec", tag = "10")]
    pub url_unescape_spec: i32,
    /// If true, unescape '+' to space when extracting variables in query parameters.
    /// This is to support `HTML 2.0 <<https://tools.ietf.org/html/rfc1866#section-8.2.1>`\_>
    #[prost(bool, tag = "12")]
    pub query_param_unescape_plus: bool,
    /// If true, try to match the custom verb even if it is unregistered. By
    /// default, only match when it is registered.
    ///
    /// According to the http template `syntax <<https://github.com/googleapis/googleapis/blob/master/google/api/http.proto#L226-L231>`\_,>
    /// the custom verb is **":" LITERAL** at the end of http template.
    ///
    /// For a request with `/foo/bar:baz` and `:baz` is not registered in any url_template, here is the behavior change
    ///
    /// * if the field is not set, `:baz` will not be treated as custom verb, so it will match `/foo/{x=*}`.
    /// * if the field is set, `:baz` is treated as custom verb,  so it will NOT match `/foo/{x=*}` since the template doesn't use any custom verb.
    #[prost(bool, tag = "13")]
    pub match_unregistered_custom_verb: bool,
    /// Configure the behavior when handling requests that cannot be transcoded.
    ///
    /// By default, the transcoder will silently pass through HTTP requests that are malformed.
    /// This includes requests with unknown query parameters, unregister paths, etc.
    ///
    /// Set these options to enable strict HTTP request validation, resulting in the transcoder rejecting
    /// such requests with a `HTTP 4xx`. See each individual option for more details on the validation.
    /// gRPC requests will still silently pass through without transcoding.
    ///
    /// The benefit is a proper error message to the downstream.
    /// If the upstream is a gRPC server, it cannot handle the passed-through HTTP requests and will reset
    /// the TCP connection. The downstream will then
    /// receive a `HTTP 503 Service Unavailable` due to the upstream connection reset.
    /// This incorrect error message may conflict with other Envoy components, such as retry policies.
    #[prost(message, optional, tag = "11")]
    pub request_validation_options: ::core::option::Option<
        grpc_json_transcoder::RequestValidationOptions,
    >,
    /// Proto enum values are supposed to be in upper cases when used in JSON.
    /// Set this to true if your JSON request uses non uppercase enum values.
    #[prost(bool, tag = "14")]
    pub case_insensitive_enum_parsing: bool,
    /// The maximum size of a request body to be transcoded, in bytes. A body exceeding this size will
    /// provoke a `HTTP 413 Request Entity Too Large` response.
    ///
    /// Large values may cause envoy to use a lot of memory if there are many concurrent requests.
    ///
    /// If unset, the current stream buffer size is used.
    #[prost(message, optional, tag = "15")]
    pub max_request_body_size: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// The maximum size of a response body to be transcoded, in bytes. A body exceeding this size will
    /// provoke a `HTTP 500 Internal Server Error` response.
    ///
    /// Large values may cause envoy to use a lot of memory if there are many concurrent requests.
    ///
    /// If unset, the current stream buffer size is used.
    #[prost(message, optional, tag = "16")]
    pub max_response_body_size: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// If true, query parameters that cannot be mapped to a corresponding
    /// protobuf field are captured in an HttpBody extension of UnknownQueryParams.
    #[prost(bool, tag = "17")]
    pub capture_unknown_query_parameters: bool,
    #[prost(oneof = "grpc_json_transcoder::DescriptorSet", tags = "1, 4")]
    pub descriptor_set: ::core::option::Option<grpc_json_transcoder::DescriptorSet>,
}
/// Nested message and enum types in `GrpcJsonTranscoder`.
pub mod grpc_json_transcoder {
    /// \[\#next-free-field: 6\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct PrintOptions {
        /// Whether to add spaces, line breaks and indentation to make the JSON
        /// output easy to read. Defaults to false.
        #[prost(bool, tag = "1")]
        pub add_whitespace: bool,
        /// Whether to always print primitive fields. By default primitive
        /// fields with default values will be omitted in JSON output. For
        /// example, an int32 field set to 0 will be omitted. Setting this flag to
        /// true will override the default behavior and print primitive fields
        /// regardless of their values. Defaults to false.
        #[prost(bool, tag = "2")]
        pub always_print_primitive_fields: bool,
        /// Whether to always print enums as ints. By default they are rendered
        /// as strings. Defaults to false.
        #[prost(bool, tag = "3")]
        pub always_print_enums_as_ints: bool,
        /// Whether to preserve proto field names. By default protobuf will
        /// generate JSON field names using the `json_name` option, or lower camel case,
        /// in that order. Setting this flag will preserve the original field names. Defaults to false.
        #[prost(bool, tag = "4")]
        pub preserve_proto_field_names: bool,
        /// If true, return all streams as newline-delimited JSON messages instead of as a comma-separated array
        #[prost(bool, tag = "5")]
        pub stream_newline_delimited: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct RequestValidationOptions {
        /// By default, a request that cannot be mapped to any specified gRPC
        /// :ref:`services <envoy_v3_api_field_extensions.filters.http.grpc_json_transcoder.v3.GrpcJsonTranscoder.services>`
        /// will pass-through this filter.
        /// When set to true, the request will be rejected with a `HTTP 404 Not Found`.
        #[prost(bool, tag = "1")]
        pub reject_unknown_method: bool,
        /// By default, a request with query parameters that cannot be mapped to the gRPC request message
        /// will pass-through this filter.
        /// When set to true, the request will be rejected with a `HTTP 400 Bad Request`.
        ///
        /// The fields
        /// :ref:`ignore_unknown_query_parameters <envoy_v3_api_field_extensions.filters.http.grpc_json_transcoder.v3.GrpcJsonTranscoder.ignore_unknown_query_parameters>`,
        /// :ref:`capture_unknown_query_parameters <envoy_v3_api_field_extensions.filters.http.grpc_json_transcoder.v3.GrpcJsonTranscoder.capture_unknown_query_parameters>`,
        /// and
        /// :ref:`ignored_query_parameters <envoy_v3_api_field_extensions.filters.http.grpc_json_transcoder.v3.GrpcJsonTranscoder.ignored_query_parameters>`
        /// have priority over this strict validation behavior.
        #[prost(bool, tag = "2")]
        pub reject_unknown_query_parameters: bool,
        /// "id: 456" in the body will override "id=123" in the binding.
        ///
        /// If this field is set to true, the request will be rejected if the binding
        /// value is different from the body value.
        #[prost(bool, tag = "3")]
        pub reject_binding_body_field_collisions: bool,
    }
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
    pub enum UrlUnescapeSpec {
        /// URL path parameters will not decode RFC 6570 reserved characters.
        /// For example, segment `%2f%23/%20%2523` is unescaped to `%2f%23/ %23`.
        AllCharactersExceptReserved = 0,
        /// URL path parameters will be fully URI-decoded except in
        /// cases of single segment matches in reserved expansion, where `%2F` will be
        /// left encoded.
        /// For example, segment `%2f%23/%20%2523` is unescaped to `%2f#/ %23`.
        AllCharactersExceptSlash = 1,
        /// URL path parameters will be fully URI-decoded.
        /// For example, segment `%2f%23/%20%2523` is unescaped to `/#/ %23`.
        AllCharacters = 2,
    }
    impl UrlUnescapeSpec {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                UrlUnescapeSpec::AllCharactersExceptReserved => {
                    "ALL_CHARACTERS_EXCEPT_RESERVED"
                }
                UrlUnescapeSpec::AllCharactersExceptSlash => {
                    "ALL_CHARACTERS_EXCEPT_SLASH"
                }
                UrlUnescapeSpec::AllCharacters => "ALL_CHARACTERS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ALL_CHARACTERS_EXCEPT_RESERVED" => {
                    Some(Self::AllCharactersExceptReserved)
                }
                "ALL_CHARACTERS_EXCEPT_SLASH" => Some(Self::AllCharactersExceptSlash),
                "ALL_CHARACTERS" => Some(Self::AllCharacters),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DescriptorSet {
        /// Supplies the filename of
        /// :ref:`the proto descriptor set <config_grpc_json_generate_proto_descriptor_set>` for the gRPC
        /// services.
        #[prost(string, tag = "1")]
        ProtoDescriptor(::prost::alloc::string::String),
        /// Supplies the binary content of
        /// :ref:`the proto descriptor set <config_grpc_json_generate_proto_descriptor_set>` for the gRPC
        /// services.
        #[prost(bytes, tag = "4")]
        ProtoDescriptorBin(::prost::alloc::vec::Vec<u8>),
    }
}
/// `UnknownQueryParams` is added as an extension field in `HttpBody` if
/// `GrpcJsonTranscoder::capture_unknown_query_parameters` is true and unknown query
/// parameters were present in the request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnknownQueryParams {
    /// A map from unrecognized query parameter keys, to the values associated with those keys.
    #[prost(map = "string, message", tag = "1")]
    pub key: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        unknown_query_params::Values,
    >,
}
/// Nested message and enum types in `UnknownQueryParams`.
pub mod unknown_query_params {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Values {
        #[prost(string, repeated, tag = "1")]
        pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
