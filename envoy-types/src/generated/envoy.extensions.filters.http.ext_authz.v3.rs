/// [#next-free-field: 19]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtAuthz {
    /// API version for ext_authz transport protocol. This describes the ext_authz gRPC endpoint and
    /// version of messages used on the wire.
    #[prost(
        enumeration = "super::super::super::super::super::config::core::v3::ApiVersion",
        tag = "12"
    )]
    pub transport_api_version: i32,
    ///   Changes filter's behaviour on errors:
    ///
    ///   1. When set to true, the filter will ``accept`` client request even if the communication with
    ///   the authorization service has failed, or if the authorization service has returned a HTTP 5xx
    ///   error.
    ///
    ///   2. When set to false, ext-authz will ``reject`` client requests and return a ``Forbidden``
    ///   response if the communication with the authorization service has failed, or if the
    ///   authorization service has returned a HTTP 5xx error.
    ///
    /// Note that errors can be ``always`` tracked in the :ref:`stats
    /// <config_http_filters_ext_authz_stats>`.
    #[prost(bool, tag = "2")]
    pub failure_mode_allow: bool,
    /// Enables filter to buffer the client request body and send it within the authorization request.
    /// A ``x-envoy-auth-partial-body: false|true`` metadata header will be added to the authorization
    /// request message indicating if the body data is partial.
    #[prost(message, optional, tag = "5")]
    pub with_request_body: ::core::option::Option<BufferSettings>,
    /// Clears route cache in order to allow the external authorization service to correctly affect
    /// routing decisions. Filter clears all cached routes when:
    ///
    /// 1. The field is set to ``true``.
    ///
    /// 2. The status returned from the authorization service is a HTTP 200 or gRPC 0.
    ///
    /// 3. At least one ``authorization response header`` is added to the client request, or is used for
    /// altering another client request header.
    ///
    #[prost(bool, tag = "6")]
    pub clear_route_cache: bool,
    /// Sets the HTTP status that is returned to the client when the authorization server returns an error
    /// or cannot be reached. The default status is HTTP 403 Forbidden.
    #[prost(message, optional, tag = "7")]
    pub status_on_error: ::core::option::Option<
        super::super::super::super::super::r#type::v3::HttpStatus,
    >,
    /// Specifies a list of metadata namespaces whose values, if present, will be passed to the
    /// ext_authz service. :ref:`filter_metadata <envoy_v3_api_field_config.core.v3.Metadata.filter_metadata>` is passed as an opaque ``protobuf::Struct``.
    ///
    /// For example, if the ``jwt_authn`` filter is used and :ref:`payload_in_metadata
    /// <envoy_v3_api_field_extensions.filters.http.jwt_authn.v3.JwtProvider.payload_in_metadata>` is set,
    /// then the following will pass the jwt payload to the authorization server.
    ///
    /// .. code-block:: yaml
    ///
    ///     metadata_context_namespaces:
    ///     - envoy.filters.http.jwt_authn
    ///
    #[prost(string, repeated, tag = "8")]
    pub metadata_context_namespaces: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Specifies a list of metadata namespaces whose values, if present, will be passed to the
    /// ext_authz service. :ref:`typed_filter_metadata <envoy_v3_api_field_config.core.v3.Metadata.typed_filter_metadata>` is passed as an ``protobuf::Any``.
    ///
    /// It works in a way similar to ``metadata_context_namespaces`` but allows envoy and external authz server to share the protobuf message definition
    /// in order to do a safe parsing.
    ///
    #[prost(string, repeated, tag = "16")]
    pub typed_metadata_context_namespaces: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Specifies if the filter is enabled.
    ///
    /// If :ref:`runtime_key <envoy_v3_api_field_config.core.v3.RuntimeFractionalPercent.runtime_key>` is specified,
    /// Envoy will lookup the runtime key to get the percentage of requests to filter.
    ///
    /// If this field is not specified, the filter will be enabled for all requests.
    #[prost(message, optional, tag = "9")]
    pub filter_enabled: ::core::option::Option<
        super::super::super::super::super::config::core::v3::RuntimeFractionalPercent,
    >,
    /// Specifies if the filter is enabled with metadata matcher.
    /// If this field is not specified, the filter will be enabled for all requests.
    #[prost(message, optional, tag = "14")]
    pub filter_enabled_metadata: ::core::option::Option<
        super::super::super::super::super::r#type::matcher::v3::MetadataMatcher,
    >,
    /// Specifies whether to deny the requests, when the filter is disabled.
    /// If :ref:`runtime_key <envoy_v3_api_field_config.core.v3.RuntimeFeatureFlag.runtime_key>` is specified,
    /// Envoy will lookup the runtime key to determine whether to deny request for
    /// filter protected path at filter disabling. If filter is disabled in
    /// typed_per_filter_config for the path, requests will not be denied.
    ///
    /// If this field is not specified, all requests will be allowed when disabled.
    ///
    /// If a request is denied due to this setting, the response code in :ref:`status_on_error
    /// <envoy_v3_api_field_extensions.filters.http.ext_authz.v3.ExtAuthz.status_on_error>` will
    /// be returned.
    #[prost(message, optional, tag = "11")]
    pub deny_at_disable: ::core::option::Option<
        super::super::super::super::super::config::core::v3::RuntimeFeatureFlag,
    >,
    /// Specifies if the peer certificate is sent to the external service.
    ///
    /// When this field is true, Envoy will include the peer X.509 certificate, if available, in the
    /// :ref:`certificate<envoy_v3_api_field_service.auth.v3.AttributeContext.Peer.certificate>`.
    #[prost(bool, tag = "10")]
    pub include_peer_certificate: bool,
    /// Optional additional prefix to use when emitting statistics. This allows to distinguish
    /// emitted statistics between configured ``ext_authz`` filters in an HTTP filter chain. For example:
    ///
    /// .. code-block:: yaml
    ///
    ///    http_filters:
    ///      - name: envoy.filters.http.ext_authz
    ///        typed_config:
    ///          "@type": type.googleapis.com/envoy.extensions.filters.http.ext_authz.v3.ExtAuthz
    ///          stat_prefix: waf # This emits ext_authz.waf.ok, ext_authz.waf.denied, etc.
    ///      - name: envoy.filters.http.ext_authz
    ///        typed_config:
    ///          "@type": type.googleapis.com/envoy.extensions.filters.http.ext_authz.v3.ExtAuthz
    ///          stat_prefix: blocker # This emits ext_authz.blocker.ok, ext_authz.blocker.denied, etc.
    ///
    #[prost(string, tag = "13")]
    pub stat_prefix: ::prost::alloc::string::String,
    /// Optional labels that will be passed to :ref:`labels<envoy_v3_api_field_service.auth.v3.AttributeContext.Peer.labels>` in
    /// :ref:`destination<envoy_v3_api_field_service.auth.v3.AttributeContext.destination>`.
    /// The labels will be read from :ref:`metadata<envoy_v3_api_msg_config.core.v3.Node>` with the specified key.
    #[prost(string, tag = "15")]
    pub bootstrap_metadata_labels_key: ::prost::alloc::string::String,
    /// Check request to authorization server will include the client request headers that have a correspondent match
    /// in the :ref:`list <envoy_v3_api_msg_type.matcher.v3.ListStringMatcher>`. If this option isn't specified, then
    /// all client request headers are included in the check request to a gRPC authorization server, whereas no client request headers
    /// (besides the ones allowed by default - see note below) are included in the check request to an HTTP authorization server.
    /// This inconsistency between gRPC and HTTP servers is to maintain backwards compatibility with legacy behavior.
    ///
    /// .. note::
    ///
    ///   1. For requests to an HTTP authorization server: in addition to the the user's supplied matchers, ``Host``, ``Method``, ``Path``,
    ///      ``Content-Length``, and ``Authorization`` are **additionally included** in the list.
    ///
    /// .. note::
    ///
    ///   2. For requests to an HTTP authorization server: *Content-Length* will be set to 0 and the request to the
    ///   authorization server will not have a message body. However, the check request can include the buffered
    ///   client request body (controlled by :ref:`with_request_body
    ///   <envoy_v3_api_field_extensions.filters.http.ext_authz.v3.ExtAuthz.with_request_body>` setting),
    ///   consequently the value of *Content-Length* of the authorization request reflects the size of
    ///   its payload size.
    #[prost(message, optional, tag = "17")]
    pub allowed_headers: ::core::option::Option<
        super::super::super::super::super::r#type::matcher::v3::ListStringMatcher,
    >,
    /// Specifies if the TLS session level details like SNI are sent to the external service.
    ///
    /// When this field is true, Envoy will include the SNI name used for TLSClientHello, if available, in the
    /// :ref:`tls_session<envoy_v3_api_field_service.auth.v3.AttributeContext.tls_session>`.
    #[prost(bool, tag = "18")]
    pub include_tls_session: bool,
    /// External authorization service configuration.
    #[prost(oneof = "ext_authz::Services", tags = "1, 3")]
    pub services: ::core::option::Option<ext_authz::Services>,
}
/// Nested message and enum types in `ExtAuthz`.
pub mod ext_authz {
    /// External authorization service configuration.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Services {
        /// gRPC service configuration (default timeout: 200ms).
        #[prost(message, tag = "1")]
        GrpcService(
            super::super::super::super::super::super::config::core::v3::GrpcService,
        ),
        /// HTTP service configuration (default timeout: 200ms).
        #[prost(message, tag = "3")]
        HttpService(super::HttpService),
    }
}
/// Configuration for buffering the request data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BufferSettings {
    /// Sets the maximum size of a message body that the filter will hold in memory. Envoy will return
    /// ``HTTP 413`` and will *not* initiate the authorization process when buffer reaches the number
    /// set in this field. Note that this setting will have precedence over :ref:`failure_mode_allow
    /// <envoy_v3_api_field_extensions.filters.http.ext_authz.v3.ExtAuthz.failure_mode_allow>`.
    #[prost(uint32, tag = "1")]
    pub max_request_bytes: u32,
    /// When this field is true, Envoy will buffer the message until ``max_request_bytes`` is reached.
    /// The authorization request will be dispatched and no 413 HTTP error will be returned by the
    /// filter.
    #[prost(bool, tag = "2")]
    pub allow_partial_message: bool,
    /// If true, the body sent to the external authorization service is set with raw bytes, it sets
    /// the :ref:`raw_body<envoy_v3_api_field_service.auth.v3.AttributeContext.HttpRequest.raw_body>`
    /// field of HTTP request attribute context. Otherwise, :ref:`body
    /// <envoy_v3_api_field_service.auth.v3.AttributeContext.HttpRequest.body>` will be filled
    /// with UTF-8 string request body.
    #[prost(bool, tag = "3")]
    pub pack_as_bytes: bool,
}
/// HttpService is used for raw HTTP communication between the filter and the authorization service.
/// When configured, the filter will parse the client request and use these attributes to call the
/// authorization server. Depending on the response, the filter may reject or accept the client
/// request. Note that in any of these events, metadata can be added, removed or overridden by the
/// filter:
///
/// *On authorization request*, a list of allowed request headers may be supplied. See
/// :ref:`allowed_headers
/// <envoy_v3_api_field_extensions.filters.http.ext_authz.v3.AuthorizationRequest.allowed_headers>`
/// for details. Additional headers metadata may be added to the authorization request. See
/// :ref:`headers_to_add
/// <envoy_v3_api_field_extensions.filters.http.ext_authz.v3.AuthorizationRequest.headers_to_add>` for
/// details.
///
/// On authorization response status HTTP 200 OK, the filter will allow traffic to the upstream and
/// additional headers metadata may be added to the original client request. See
/// :ref:`allowed_upstream_headers
/// <envoy_v3_api_field_extensions.filters.http.ext_authz.v3.AuthorizationResponse.allowed_upstream_headers>`
/// for details. Additionally, the filter may add additional headers to the client's response. See
/// :ref:`allowed_client_headers_on_success
/// <envoy_v3_api_field_extensions.filters.http.ext_authz.v3.AuthorizationResponse.allowed_client_headers_on_success>`
/// for details.
///
/// On other authorization response statuses, the filter will not allow traffic. Additional headers
/// metadata as well as body may be added to the client's response. See :ref:`allowed_client_headers
/// <envoy_v3_api_field_extensions.filters.http.ext_authz.v3.AuthorizationResponse.allowed_client_headers>`
/// for details.
/// [#next-free-field: 9]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpService {
    /// Sets the HTTP server URI which the authorization requests must be sent to.
    #[prost(message, optional, tag = "1")]
    pub server_uri: ::core::option::Option<
        super::super::super::super::super::config::core::v3::HttpUri,
    >,
    /// Sets a prefix to the value of authorization request header ``Path``.
    #[prost(string, tag = "2")]
    pub path_prefix: ::prost::alloc::string::String,
    /// Settings used for controlling authorization request metadata.
    #[prost(message, optional, tag = "7")]
    pub authorization_request: ::core::option::Option<AuthorizationRequest>,
    /// Settings used for controlling authorization response metadata.
    #[prost(message, optional, tag = "8")]
    pub authorization_response: ::core::option::Option<AuthorizationResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizationRequest {
    /// Authorization request includes the client request headers that have a correspondent match
    /// in the :ref:`list <envoy_v3_api_msg_type.matcher.v3.ListStringMatcher>`.
    /// This field has been deprecated in favor of :ref:`allowed_headers
    /// <envoy_v3_api_field_extensions.filters.http.ext_authz.v3.ExtAuthz.allowed_headers>`.
    ///
    /// .. note::
    ///
    ///    In addition to the the user's supplied matchers, ``Host``, ``Method``, ``Path``,
    ///    ``Content-Length``, and ``Authorization`` are **automatically included** to the list.
    ///
    /// .. note::
    ///
    ///    By default, ``Content-Length`` header is set to ``0`` and the request to the authorization
    ///    service has no message body. However, the authorization request *may* include the buffered
    ///    client request body (controlled by :ref:`with_request_body
    ///    <envoy_v3_api_field_extensions.filters.http.ext_authz.v3.ExtAuthz.with_request_body>`
    ///    setting) hence the value of its ``Content-Length`` reflects the size of its payload size.
    ///
    #[deprecated]
    #[prost(message, optional, tag = "1")]
    pub allowed_headers: ::core::option::Option<
        super::super::super::super::super::r#type::matcher::v3::ListStringMatcher,
    >,
    /// Sets a list of headers that will be included to the request to authorization service. Note that
    /// client request of the same key will be overridden.
    #[prost(message, repeated, tag = "2")]
    pub headers_to_add: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::core::v3::HeaderValue,
    >,
}
/// [#next-free-field: 6]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizationResponse {
    /// When this :ref:`list <envoy_v3_api_msg_type.matcher.v3.ListStringMatcher>` is set, authorization
    /// response headers that have a correspondent match will be added to the original client request.
    /// Note that coexistent headers will be overridden.
    #[prost(message, optional, tag = "1")]
    pub allowed_upstream_headers: ::core::option::Option<
        super::super::super::super::super::r#type::matcher::v3::ListStringMatcher,
    >,
    /// When this :ref:`list <envoy_v3_api_msg_type.matcher.v3.ListStringMatcher>` is set, authorization
    /// response headers that have a correspondent match will be added to the client's response. Note
    /// that coexistent headers will be appended.
    #[prost(message, optional, tag = "3")]
    pub allowed_upstream_headers_to_append: ::core::option::Option<
        super::super::super::super::super::r#type::matcher::v3::ListStringMatcher,
    >,
    /// When this :ref:`list <envoy_v3_api_msg_type.matcher.v3.ListStringMatcher>` is set, authorization
    /// response headers that have a correspondent match will be added to the client's response. Note
    /// that when this list is *not* set, all the authorization response headers, except ``Authority
    /// (Host)`` will be in the response to the client. When a header is included in this list, ``Path``,
    /// ``Status``, ``Content-Length``, ``WWWAuthenticate`` and ``Location`` are automatically added.
    #[prost(message, optional, tag = "2")]
    pub allowed_client_headers: ::core::option::Option<
        super::super::super::super::super::r#type::matcher::v3::ListStringMatcher,
    >,
    /// When this :ref:`list <envoy_v3_api_msg_type.matcher.v3.ListStringMatcher>` is set, authorization
    /// response headers that have a correspondent match will be added to the client's response when
    /// the authorization response itself is successful, i.e. not failed or denied. When this list is
    /// *not* set, no additional headers will be added to the client's response on success.
    #[prost(message, optional, tag = "4")]
    pub allowed_client_headers_on_success: ::core::option::Option<
        super::super::super::super::super::r#type::matcher::v3::ListStringMatcher,
    >,
    /// When this :ref:`list <envoy_v3_api_msg_type.matcher.v3.ListStringMatcher>` is set, authorization
    /// response headers that have a correspondent match will be emitted as dynamic metadata to be consumed
    /// by the next filter. This metadata lives in a namespace specified by the canonical name of extension filter
    /// that requires it:
    ///
    /// - :ref:`envoy.filters.http.ext_authz <config_http_filters_ext_authz_dynamic_metadata>` for HTTP filter.
    /// - :ref:`envoy.filters.network.ext_authz <config_network_filters_ext_authz_dynamic_metadata>` for network filter.
    #[prost(message, optional, tag = "5")]
    pub dynamic_metadata_from_headers: ::core::option::Option<
        super::super::super::super::super::r#type::matcher::v3::ListStringMatcher,
    >,
}
/// Extra settings on a per virtualhost/route/weighted-cluster level.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtAuthzPerRoute {
    #[prost(oneof = "ext_authz_per_route::Override", tags = "1, 2")]
    pub r#override: ::core::option::Option<ext_authz_per_route::Override>,
}
/// Nested message and enum types in `ExtAuthzPerRoute`.
pub mod ext_authz_per_route {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Override {
        /// Disable the ext auth filter for this particular vhost or route.
        /// If disabled is specified in multiple per-filter-configs, the most specific one will be used.
        #[prost(bool, tag = "1")]
        Disabled(bool),
        /// Check request settings for this route.
        #[prost(message, tag = "2")]
        CheckSettings(super::CheckSettings),
    }
}
/// Extra settings for the check request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckSettings {
    /// Context extensions to set on the CheckRequest's
    /// :ref:`AttributeContext.context_extensions<envoy_v3_api_field_service.auth.v3.AttributeContext.context_extensions>`
    ///
    /// You can use this to provide extra context for the external authorization server on specific
    /// virtual hosts/routes. For example, adding a context extension on the virtual host level can
    /// give the ext-authz server information on what virtual host is used without needing to parse the
    /// host header. If CheckSettings is specified in multiple per-filter-configs, they will be merged
    /// in order, and the result will be used.
    ///
    /// Merge semantics for this field are such that keys from more specific configs override.
    ///
    /// .. note::
    ///
    ///    These settings are only applied to a filter configured with a
    ///    :ref:`grpc_service<envoy_v3_api_field_extensions.filters.http.ext_authz.v3.ExtAuthz.grpc_service>`.
    #[prost(map = "string, string", tag = "1")]
    pub context_extensions: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// When set to true, disable the configured :ref:`with_request_body
    /// <envoy_v3_api_field_extensions.filters.http.ext_authz.v3.ExtAuthz.with_request_body>` for a route.
    #[prost(bool, tag = "2")]
    pub disable_request_body_buffering: bool,
}
